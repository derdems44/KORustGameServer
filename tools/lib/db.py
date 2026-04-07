"""Shared DB access for KO packet analysis tools.

Wraps psql queries via ``docker exec`` against the sniffer's
``pkt.*`` schema and provides session listing, packet fetching,
and full-session decrypt using :mod:`.ko_crypto`.
"""

from __future__ import annotations

import subprocess
from dataclasses import dataclass
from typing import List, Optional

from .ko_crypto import decrypt_wire_packet
from .opcodes import get_name


# ---------------------------------------------------------------------------
# Low-level DB helper
# ---------------------------------------------------------------------------

def _psql(query: str) -> str:
    """Run a psql query via ``docker exec`` and return raw output.

    Uses the ``ko-postgres`` container with ``-t -A`` flags for
    unaligned, tuple-only output.
    """
    result = subprocess.run(
        [
            "docker", "exec", "ko-postgres",
            "psql", "-U", "koserver", "-d", "ko_server",
            "-t", "-A", "-c", query,
        ],
        capture_output=True,
        text=True,
        timeout=30,
    )
    if result.returncode != 0:
        raise RuntimeError(f"psql error: {result.stderr.strip()}")
    return result.stdout.strip()


# ---------------------------------------------------------------------------
# DecodedPacket dataclass
# ---------------------------------------------------------------------------

@dataclass
class DecodedPacket:
    """A fully decoded packet ready for analysis."""

    seq: int
    direction: str
    opcode: int
    opcode_name: str
    plaintext: bytes
    source: str  # 'sniffer', 'redecrypt', 'plaintext'


# ---------------------------------------------------------------------------
# Session queries
# ---------------------------------------------------------------------------

def get_sessions(limit: int = 20) -> List[dict]:
    """List recent sessions from ``pkt.sessions``.

    Returns:
        List of dicts with keys: id, client_ip, server_ip,
        server_port, server_type, aes_key, created_at.
    """
    raw = _psql(
        f"SELECT id, client_ip, server_ip, server_port, server_type, "
        f"aes_key_ascii, created_at "
        f"FROM pkt.sessions ORDER BY id DESC LIMIT {int(limit)};"
    )
    if not raw:
        return []

    sessions: List[dict] = []
    for line in raw.split("\n"):
        if not line:
            continue
        parts = line.split("|")
        if len(parts) < 7:
            continue
        sessions.append({
            "id": int(parts[0]),
            "client_ip": parts[1],
            "server_ip": parts[2],
            "server_port": int(parts[3]) if parts[3] else 0,
            "server_type": parts[4],
            "aes_key": parts[5],
            "created_at": parts[6],
        })
    return sessions


def get_session_packets(session_id: int) -> List[dict]:
    """Fetch all packets for a session from ``pkt.packets``.

    Returns:
        List of dicts with keys: id, seq, direction, encrypted, opcode,
        opcode_name, raw_wire_hex, plaintext_hex, payload_len.

    Ordered by ``id`` (auto-increment) which gives true temporal
    ordering.  C2S and S2C have independent ``seq`` counters, so
    ``ORDER BY seq`` would interleave them incorrectly.
    """
    raw = _psql(
        f"SELECT id, seq, direction, encrypted::text, opcode, opcode_name, "
        f"encode(raw_wire, 'hex'), encode(plaintext, 'hex'), payload_len "
        f"FROM pkt.packets WHERE session_id = {int(session_id)} "
        f"ORDER BY id;"
    )
    if not raw:
        return []

    packets: List[dict] = []
    for line in raw.split("\n"):
        if not line:
            continue
        parts = line.split("|")
        if len(parts) < 9:
            continue
        packets.append({
            "id": int(parts[0]),
            "seq": int(parts[1]),
            "direction": parts[2],
            "encrypted": parts[3] in ("t", "true"),
            "opcode": int(parts[4]) if parts[4] else 0,
            "opcode_name": parts[5],
            "raw_wire_hex": parts[6],
            "plaintext_hex": parts[7],
            "payload_len": int(parts[8]) if parts[8] else 0,
        })
    return packets


# ---------------------------------------------------------------------------
# Full session decrypt
# ---------------------------------------------------------------------------

def decrypt_session(session_id: int, aes_key: str) -> List[DecodedPacket]:
    """Fetch and decrypt ALL packets for a session.

    Three cases per packet:

    1. Has ``plaintext_hex`` in DB -- use DB opcode directly (source='sniffer').
    2. Encrypted but no plaintext -- re-decrypt with :func:`decrypt_wire_packet`
       (source='redecrypt').
    3. Non-encrypted -- use :func:`decrypt_wire_packet` which handles
       flag != 0x01 as plaintext (source='plaintext').

    Args:
        session_id: The sniffer session ID.
        aes_key: AES key string (ASCII).

    Returns:
        List of :class:`DecodedPacket` in sequence order.
    """
    rows = get_session_packets(session_id)
    key_bytes = aes_key.encode("ascii")
    result: List[DecodedPacket] = []

    for row in rows:
        pkt = _decode_row(row, key_bytes)
        if pkt is not None:
            result.append(pkt)

    return result


def _decode_row(row: dict, key_bytes: bytes) -> Optional[DecodedPacket]:
    """Decode a single packet row into a :class:`DecodedPacket`."""
    seq = row["seq"]
    direction = row["direction"]
    encrypted = row["encrypted"]
    db_opcode = row["opcode"]
    plaintext_hex = row["plaintext_hex"]
    raw_wire_hex = row["raw_wire_hex"]

    # Case 1: DB already has plaintext
    if plaintext_hex:
        pt = bytes.fromhex(plaintext_hex)
        return DecodedPacket(
            seq=seq,
            direction=direction,
            opcode=db_opcode,
            opcode_name=get_name(db_opcode),
            plaintext=pt,
            source="sniffer",
        )

    # Case 2 & 3: Need to decrypt from raw wire
    if raw_wire_hex:
        raw_wire = bytes.fromhex(raw_wire_hex)
        decoded = decrypt_wire_packet(raw_wire, key_bytes, direction)
        if decoded is not None:
            opcode, pt = decoded
            source = "redecrypt" if encrypted else "plaintext"
            return DecodedPacket(
                seq=seq,
                direction=direction,
                opcode=opcode,
                opcode_name=get_name(opcode),
                plaintext=pt,
                source=source,
            )

    return None
