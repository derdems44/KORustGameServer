//! Knight Online encryption/decryption — byte-for-byte port of JvCryption.cpp.
//!
//! C++ Reference: `KOOriginalGameServer/shared/JvCryption.cpp`
//!
//! The algorithm is a symmetric XOR stream cipher:
//! - 3-layer XOR per byte: rotation key, session key (8-byte cycle), length key
//! - Rotation key evolves via `rkey *= 2171` (32-bit wrapping)
//! - CRC32 appended for integrity verification
//!
//! Encryption == Decryption (XOR is self-inverse).

use std::sync::atomic::{AtomicU32, Ordering};

/// Hardcoded private key — must match the client.
/// C++ Reference: `#define g_private_key 0x1207500120128966`
const PRIVATE_KEY: u64 = 0x1207_5001_2012_8966;

/// Initial rotation key constant.
const INITIAL_RKEY: u32 = 2157;

/// Rotation key multiplier.
const RKEY_MULTIPLIER: u32 = 2171;

/// Length key multiplier (masked to u8).
const LKEY_MULTIPLIER: u8 = 157;

/// CRC32 polynomial (reversed).
const CRC32_POLY: u32 = 0xEDB8_8320;

/// Pre-computed CRC32 lookup table (256 entries).
/// Generated from polynomial 0xEDB88320.
const CRC32_TABLE: [u32; 256] = {
    let mut table = [0u32; 256];
    let mut i = 0u32;
    while i < 256 {
        let mut crc = i;
        let mut j = 0;
        while j < 8 {
            if crc & 1 != 0 {
                crc = (crc >> 1) ^ CRC32_POLY;
            } else {
                crc >>= 1;
            }
            j += 1;
        }
        table[i as usize] = crc;
        i += 1;
    }
    table
};

/// Knight Online session encryption state.
///
/// Each TCP connection gets its own `JvCryption` instance.
/// After key exchange, all packets are encrypted/decrypted with this.
pub struct JvCryption {
    /// Public key — random, exchanged during handshake.
    public_key: u64,
    /// Working key = `public_key ^ PRIVATE_KEY`.
    tkey: u64,
    /// Whether encryption is active.
    enabled: bool,
    /// Inbound packet sequence counter.
    sequence: AtomicU32,
}

impl JvCryption {
    /// Create a new (disabled) encryption state.
    pub fn new() -> Self {
        Self {
            public_key: 0,
            tkey: 0,
            enabled: false,
            sequence: AtomicU32::new(0),
        }
    }

    /// Generate a random non-zero public key for key exchange.
    pub fn generate_key(&mut self) -> u64 {
        loop {
            let key = rand_u64();
            if key != 0 {
                self.public_key = key;
                return key;
            }
        }
    }

    /// Set the public key (received from the other side) and derive the working key.
    pub fn set_public_key(&mut self, key: u64) {
        self.public_key = key;
    }

    /// Initialize encryption: derive the working key and enable crypto.
    /// Must be called after `generate_key()` or `set_public_key()`.
    pub fn init(&mut self) {
        self.tkey = self.public_key ^ PRIVATE_KEY;
        self.enabled = true;
    }

    /// Whether encryption is currently active.
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    /// Get the current public key.
    pub fn public_key(&self) -> u64 {
        self.public_key
    }

    /// Encrypt data in-place.
    /// C++ Reference: `CJvCryption::JvEncryptionFast`
    ///
    /// Algorithm per byte `i`:
    /// ```text
    /// rsk = (rkey >> 8) & 0xFF
    /// out[i] = ((in[i] ^ rsk) ^ tkey_bytes[i % 8]) ^ lkey
    /// rkey = rkey.wrapping_mul(2171)
    /// ```
    pub fn encrypt(&self, data: &mut [u8]) {
        self.transform(data);
    }

    /// Decrypt data in-place (same as encrypt — XOR is symmetric).
    pub fn decrypt(&self, data: &mut [u8]) {
        self.transform(data);
    }

    /// Core transform — used for both encrypt and decrypt.
    fn transform(&self, data: &mut [u8]) {
        let len = data.len();
        let pkey = self.tkey.to_le_bytes();
        let lkey = (len as u8).wrapping_mul(LKEY_MULTIPLIER);
        let mut rkey: u32 = INITIAL_RKEY;

        for (i, byte) in data.iter_mut().enumerate() {
            let rsk = ((rkey >> 8) & 0xFF) as u8;
            *byte = ((*byte ^ rsk) ^ pkey[i % 8]) ^ lkey;
            rkey = rkey.wrapping_mul(RKEY_MULTIPLIER);
        }
    }

    /// Decrypt data and verify CRC32 checksum.
    /// Returns the payload length (without CRC) on success, or `None` if CRC fails.
    ///
    /// C++ Reference: `CJvCryption::JvDecryptionWithCRC32`
    pub fn decrypt_with_crc32(&self, data: &mut [u8]) -> Option<usize> {
        if data.len() < 4 {
            return None;
        }
        self.decrypt(data);

        let payload_len = data.len() - 4;
        let computed = crc32(&data[..payload_len]);
        let stored = u32::from_le_bytes([
            data[payload_len],
            data[payload_len + 1],
            data[payload_len + 2],
            data[payload_len + 3],
        ]);

        if computed == stored {
            Some(payload_len)
        } else {
            None
        }
    }

    /// Encrypt data and append CRC32 checksum.
    /// The output buffer must have 4 extra bytes for the CRC.
    pub fn encrypt_with_crc32(&self, data: &mut Vec<u8>) {
        let crc = crc32(data);
        data.extend_from_slice(&crc.to_le_bytes());
        self.encrypt(data);
    }

    /// Get and increment the inbound sequence counter.
    pub fn next_sequence(&self) -> u32 {
        self.sequence.fetch_add(1, Ordering::Relaxed) + 1
    }

    /// Get the current sequence value (without incrementing).
    pub fn current_sequence(&self) -> u32 {
        self.sequence.load(Ordering::Relaxed)
    }
}

impl Default for JvCryption {
    fn default() -> Self {
        Self::new()
    }
}

/// Compute CRC32 checksum with a custom start value (same as C++ `crc32()` function).
///
/// C++ signature: `crc32(const unsigned char *s, unsigned int len, unsigned int startVal)`
/// Uses polynomial 0xEDB88320.
pub fn crc32_with_start(data: &[u8], start_val: u32) -> u32 {
    let mut crc: u32 = start_val;
    for &byte in data {
        crc = CRC32_TABLE[((crc ^ u32::from(byte)) & 0xFF) as usize] ^ (crc >> 8);
    }
    crc
}

/// Compute CRC32 checksum with initial value 0xFFFFFFFF.
///
/// Used by JvCryption encrypt/decrypt (C++ calls `crc32(data, len, -1)`).
pub fn crc32(data: &[u8]) -> u32 {
    crc32_with_start(data, 0xFFFF_FFFF)
}

/// Generate a pseudo-random u64.
fn rand_u64() -> u64 {
    // Simple xorshift64* — sufficient for key generation.
    use std::time::SystemTime;
    let seed = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos() as u64;
    let mut x = seed;
    x ^= x >> 12;
    x ^= x << 25;
    x ^= x >> 27;
    x.wrapping_mul(0x2545_F491_4F6C_DD1D)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt_decrypt_roundtrip() {
        let mut crypto = JvCryption::new();
        crypto.public_key = 0xDEAD_BEEF_CAFE_BABE;
        crypto.init();

        let original = b"Hello, Knight Online!".to_vec();
        let mut data = original.clone();

        crypto.encrypt(&mut data);
        assert_ne!(data, original, "encrypted data should differ from original");

        crypto.decrypt(&mut data);
        assert_eq!(data, original, "decrypted data should match original");
    }

    #[test]
    fn test_encrypt_decrypt_with_crc32() {
        let mut crypto = JvCryption::new();
        crypto.public_key = 0x1234_5678_9ABC_DEF0;
        crypto.init();

        let original = b"Test CRC32 payload".to_vec();
        let mut data = original.clone();

        crypto.encrypt_with_crc32(&mut data);
        assert_eq!(data.len(), original.len() + 4, "CRC adds 4 bytes");

        let payload_len = crypto.decrypt_with_crc32(&mut data);
        assert_eq!(payload_len, Some(original.len()));
        assert_eq!(&data[..original.len()], original.as_slice());
    }

    #[test]
    fn test_crc32_corrupt_detection() {
        let mut crypto = JvCryption::new();
        crypto.public_key = 0xAAAA_BBBB_CCCC_DDDD;
        crypto.init();

        let mut data = b"integrity check".to_vec();
        crypto.encrypt_with_crc32(&mut data);

        // Corrupt one byte
        data[0] ^= 0xFF;

        let result = crypto.decrypt_with_crc32(&mut data);
        assert_eq!(result, None, "corrupted data should fail CRC check");
    }

    #[test]
    fn test_private_key_constant() {
        assert_eq!(PRIVATE_KEY, 0x1207_5001_2012_8966);
    }

    #[test]
    fn test_key_derivation() {
        let mut crypto = JvCryption::new();
        crypto.public_key = 0;
        crypto.init();
        assert_eq!(crypto.tkey, PRIVATE_KEY, "0 XOR key = key");

        crypto.public_key = PRIVATE_KEY;
        crypto.init();
        assert_eq!(crypto.tkey, 0, "key XOR key = 0");
    }

    #[test]
    fn test_crc32_known_values() {
        // Empty data — no bytes processed, returns initial value
        assert_eq!(crc32(b""), 0xFFFF_FFFF);

        // C++ KO crc32 does NOT apply final XOR (unlike standard CRC32).
        // Standard CRC32("123456789") = 0xCBF43926 (with final ^= 0xFFFFFFFF)
        // KO crc32("123456789") = 0x340BC6D9 (no final XOR)
        let crc = crc32(b"123456789");
        assert_eq!(crc, 0x340B_C6D9);
    }

    #[test]
    fn test_crc32_with_start_zero() {
        // C++ compressed packets use startVal=0: crc32(buffer, inLength, 0)
        // Empty data with startVal=0 returns 0
        assert_eq!(crc32_with_start(b"", 0), 0);

        // crc32("123456789", 0) — different from crc32("123456789", 0xFFFFFFFF)
        let crc_zero = crc32_with_start(b"123456789", 0);
        let crc_ff = crc32(b"123456789");
        assert_ne!(
            crc_zero, crc_ff,
            "different startVal must produce different CRC"
        );

        // crc32_with_start(data, 0xFFFFFFFF) == crc32(data)
        assert_eq!(crc32_with_start(b"123456789", 0xFFFF_FFFF), crc_ff);
    }

    #[test]
    fn test_transform_deterministic() {
        let mut crypto = JvCryption::new();
        crypto.public_key = 0x1111_2222_3333_4444;
        crypto.init();

        let mut data1 = vec![0x00; 16];
        let mut data2 = vec![0x00; 16];

        crypto.encrypt(&mut data1);
        crypto.encrypt(&mut data2);

        assert_eq!(data1, data2, "same key + same input = same output");
    }

    #[test]
    fn test_empty_data() {
        let mut crypto = JvCryption::new();
        crypto.public_key = 0xFFFF_FFFF_FFFF_FFFF;
        crypto.init();

        let mut data: Vec<u8> = vec![];
        crypto.encrypt(&mut data);
        assert!(data.is_empty());
    }

    #[test]
    fn test_sequence_counter() {
        let crypto = JvCryption::new();
        assert_eq!(crypto.next_sequence(), 1);
        assert_eq!(crypto.next_sequence(), 2);
        assert_eq!(crypto.next_sequence(), 3);
        assert_eq!(crypto.current_sequence(), 3);
    }
}
