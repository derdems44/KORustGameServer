"""Shared KO protocol crypto helpers.

AES-128-CBC decryption with the static IV used by all Knight Online
game/login packets, plus wire-frame parsing for the ``[AA55]...[55AA]``
envelope.
"""

from __future__ import annotations

import struct
from typing import Optional, Tuple

from Crypto.Cipher import AES

# ---- Constants ----

AES_IV: bytes = bytes([
    0x32, 0x4E, 0xAA, 0x58, 0xBC, 0xB3, 0xAE, 0xE3,
    0x6B, 0xC7, 0x4C, 0x56, 0x36, 0x47, 0x34, 0xF2,
])

AES_FLAG: int = 0x01


# ---- Helpers ----

def pkcs7_unpad(data: bytes) -> Optional[bytes]:
    """Remove PKCS7 padding. Return *None* if padding is invalid."""
    if not data:
        return None
    pad = data[-1]
    if pad < 1 or pad > 16:
        return None
    if any(b != pad for b in data[-pad:]):
        return None
    return data[:-pad]


def aes_decrypt(key: bytes, ciphertext: bytes) -> Optional[bytes]:
    """AES-128-CBC decrypt *ciphertext* with the static KO IV.

    Returns the unpadded plaintext, or the raw decrypted block if
    PKCS7 padding is invalid (some packets are not padded cleanly).
    Returns *None* on total failure (empty input, wrong block size).
    """
    if len(ciphertext) == 0 or len(ciphertext) % 16 != 0:
        return None
    try:
        cipher = AES.new(key, AES.MODE_CBC, AES_IV)
        raw = cipher.decrypt(ciphertext)
        unpadded = pkcs7_unpad(raw)
        return unpadded if unpadded else raw
    except Exception:
        return None


def decrypt_wire_packet(
    raw_wire: bytes,
    aes_key: bytes,
    direction: str,
) -> Optional[Tuple[int, bytes]]:
    """Decrypt a full KO wire packet and return ``(opcode, plaintext)``.

    Wire format::

        [AA 55] [payload_len:u16le] [flag] [payload ...] [55 AA]

    *direction* must be ``'C2S'`` or ``'S2C'``.

    For encrypted packets (flag == 0x01):
      - C2S: plaintext[0] = xor_seq counter, plaintext[1] = opcode
      - S2C: plaintext[0] = opcode

    For non-encrypted packets (flag != 0x01), the flag byte is part of
    the plaintext (first byte after the length field), and the opcode is
    determined using the same C2S/S2C rule on that plaintext.

    Returns *None* when the frame is malformed or decryption fails.
    """
    # Minimum frame: AA55 + len(2) + flag(1) + 55AA = 7 bytes
    if len(raw_wire) < 7:
        return None
    if raw_wire[:2] != b'\xaa\x55' or raw_wire[-2:] != b'\x55\xaa':
        return None

    # payload_len = struct.unpack_from('<H', raw_wire, 2)[0]
    flag = raw_wire[4]
    payload = raw_wire[5:-2]

    if flag == AES_FLAG:
        # Encrypted packet
        pt = aes_decrypt(aes_key, payload)
        if pt is None or len(pt) == 0:
            return None
        if direction == 'C2S' and len(pt) >= 2:
            opcode = pt[1]
        else:
            opcode = pt[0]
        return (opcode, pt)
    else:
        # Non-encrypted: flag byte is first byte of plaintext
        pt = bytes([flag]) + payload
        if len(pt) == 0:
            return None
        if direction == 'C2S' and len(pt) >= 2:
            opcode = pt[1]
        else:
            opcode = pt[0]
        return (opcode, pt)
