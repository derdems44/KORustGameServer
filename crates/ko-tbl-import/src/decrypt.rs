//! KO .tbl file decryption — ChaosExpansion (v1886+ / USKO) encryption.
//!
//! Ported from PentegramTBL C# project:
//!   KOEncryption_ChaosExpansion.cs — double encryption (DES-like block + XOR stream)
//!   KOEncryption_Standart.cs — simple XOR stream (v1098-1534)

/// 768-bit key used by ChaosExpansion layer-1 DES-like cipher.
/// Auto-generated from PentegramTBL C# source — DO NOT EDIT MANUALLY.
const KEY: [u8; 768] = [
    0, 0, 1, 1, 1, 0, 0, 0, 1, 1, 0, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 1, 0, 1, 0, 1, 1, 1,
    1, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 0, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0,
    1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 1, 1, 1, 1, 0,
    0, 0, 0, 1, 1, 1, 1, 0, 1, 0, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 0, 0, 1, 0, 1, 1, 1, 0, 1,
    0, 0, 1, 1, 0, 1, 0, 0, 1, 1, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 1, 1, 1, 0, 1, 0, 0,
    0, 0, 0, 0, 1, 1, 1, 1, 0, 1, 1, 0, 1, 1, 1, 0, 1, 1, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 1, 0, 0, 1,
    0, 1, 1, 0, 1, 0, 1, 1, 1, 0, 0, 0, 0, 1, 1, 1, 0, 1, 1, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0,
    1, 1, 1, 1, 1, 1, 0, 1, 0, 1, 0, 0, 1, 1, 1, 1, 1, 1, 0, 0, 1, 0, 0, 0, 1, 1, 0, 1, 1, 0, 0, 0,
    1, 0, 1, 1, 1, 1, 1, 1, 1, 0, 1, 0, 1, 1, 1, 0, 1, 0, 0, 1, 0, 1, 0, 1, 1, 0, 1, 1, 0, 0, 0, 0,
    1, 0, 1, 1, 0, 1, 0, 1, 1, 0, 1, 0, 1, 0, 1, 1, 0, 1, 1, 1, 1, 0, 1, 0, 1, 1, 0, 0, 1, 0, 0, 1,
    0, 1, 0, 0, 1, 1, 1, 1, 0, 1, 1, 0, 0, 0, 1, 1, 1, 0, 1, 0, 0, 1, 1, 0, 0, 1, 1, 1, 1, 1, 1, 0,
    1, 0, 1, 0, 0, 0, 1, 1, 0, 1, 0, 1, 1, 1, 1, 0, 1, 1, 0, 0, 1, 0, 1, 0, 0, 0, 0, 1, 1, 0, 0, 0,
    0, 0, 1, 1, 1, 1, 0, 0, 1, 0, 1, 1, 0, 0, 1, 1, 0, 1, 0, 0, 0, 1, 1, 1, 1, 0, 0, 1, 0, 0, 0, 0,
    1, 0, 0, 0, 1, 1, 0, 0, 0, 1, 1, 1, 1, 1, 1, 0, 0, 0, 1, 0, 0, 0, 1, 1, 0, 1, 0, 1, 1, 1, 0, 0,
    0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 1, 1, 0, 1, 1, 0, 0, 1, 1, 1, 1, 0, 1, 0, 0, 1, 0, 1, 0, 0,
    1, 1, 1, 0, 1, 1, 0, 1, 0, 1, 1, 0, 0, 0, 0, 1, 1, 1, 1, 1, 0, 1, 0, 0, 0, 0, 0, 1, 1, 0, 0, 1,
    0, 1, 1, 0, 0, 1, 1, 0, 1, 1, 1, 1, 1, 0, 0, 1, 1, 0, 0, 1, 0, 1, 1, 0, 1, 1, 0, 0, 1, 1, 1, 1,
    1, 0, 1, 1, 1, 0, 0, 1, 0, 0, 1, 1, 1, 0, 1, 1, 1, 1, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1,
    1, 1, 0, 1, 1, 1, 1, 1, 0, 0, 1, 1, 0, 0, 1, 1, 0, 0, 1, 1, 0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 1, 0,
    0, 1, 1, 0, 0, 1, 0, 1, 1, 0, 1, 1, 0, 1, 1, 0, 1, 0, 1, 0, 1, 1, 1, 1, 1, 0, 0, 1, 1, 1, 1, 0,
    1, 1, 0, 0, 1, 1, 0, 1, 1, 0, 1, 0, 1, 1, 0, 1, 0, 0, 1, 0, 1, 0, 1, 1, 1, 0, 0, 0, 0, 1, 0, 1,
    0, 1, 0, 1, 1, 0, 1, 1, 0, 1, 1, 1, 0, 0, 1, 0, 1, 1, 0, 0, 1, 1, 1, 0, 1, 1, 1, 1, 0, 0, 1, 0,
    0, 1, 0, 0, 0, 0, 1, 0, 1, 1, 0, 1, 0, 0, 1, 1, 0, 1, 1, 1, 0, 1, 0, 1, 0, 0, 0, 0, 1, 1, 1, 0,
    1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 0, 0, 1, 1, 1, 0, 1, 0, 1, 0, 0, 1, 0, 1, 0, 0, 0, 1, 1, 0,
];

const EXPANSION_MATRIX: [u8; 48] = [
    32, 1, 2, 3, 4, 5, 4, 5, 6, 7, 8, 9, 8, 9, 10, 11, 12, 13, 12, 13, 14, 15, 16, 17, 16, 17, 18,
    19, 20, 21, 20, 21, 22, 23, 24, 25, 24, 25, 26, 27, 28, 29, 28, 29, 30, 31, 32, 1,
];

const PERMUTATION: [u8; 32] = [
    16, 7, 20, 21, 29, 12, 28, 17, 1, 15, 23, 26, 5, 18, 31, 10, 2, 8, 24, 14, 32, 27, 3, 9, 19,
    13, 30, 6, 22, 11, 4, 25,
];

// S-box lookup tables (from C# source)
const SBOX_0: [u32; 64] = [
    0x10101, 0x100, 0x1000101, 0x1000000, 0x10000, 0x1010101, 0x1010001, 1, 0x1010000, 0x10001,
    0x10100, 0x101, 0x1000100, 0x1000001, 0, 0x1010100, 0, 0x1010101, 0x1010100, 0x100, 0x10101,
    0x10000, 0x1000101, 0x1000000, 0x10001, 0x10100, 0x101, 0x1010001, 0x1000001, 0x1000100,
    0x1010000, 1, 0x100, 0x1000000, 0x10101, 1, 0x1000101, 0x10100, 0x10000, 0x1010001, 0x1010101,
    0x101, 0x1000001, 0x1010100, 0x1010000, 0x10001, 0x1000100, 0, 0x1010101, 0x101, 1, 0x10000,
    0x100, 0x1000001, 0x1000000, 0x1010100, 0x1000100, 0x1010001, 0x1010000, 0x10101, 0x10001, 0,
    0x10100, 0x1000101,
];
const SBOX_1: [u32; 64] = [
    0x1010101, 0x1000000, 1, 0x10101, 0x10100, 0x1010001, 0x1010000, 0x100, 0x1000001, 0x1010100,
    0x10000, 0x1000101, 0x101, 0, 0x1000100, 0x10001, 0x1010000, 0x1000101, 0x100, 0x1010100,
    0x1010101, 0x10000, 1, 0x10101, 0x101, 0, 0x1000000, 0x10001, 0x10100, 0x1000001, 0x1010001,
    0x1000100, 0, 0x10101, 0x1010100, 0x1010001, 0x10001, 0x100, 0x1000101, 0x1000000, 0x1000100,
    1, 0x101, 0x10100, 0x1000001, 0x1010000, 0x10000, 0x1010101, 0x1000101, 1, 0x10001, 0x1000000,
    0x1010000, 0x1010101, 0x100, 0x10000, 0x1010001, 0x10100, 0x1010100, 0x101, 0, 0x1000100,
    0x10101, 0x1000001,
];
const SBOX_2: [u32; 64] = [
    0x10001, 0, 0x1000001, 0x10101, 0x10100, 0x1010000, 0x1010101, 0x1000100, 0x1000000, 0x1000101,
    0x101, 0x1010100, 0x1010001, 0x100, 0x10000, 1, 0x1000101, 0x1010100, 0, 0x1000001, 0x1010000,
    0x100, 0x10100, 0x10001, 0x10000, 1, 0x1000100, 0x10101, 0x101, 0x1010001, 0x1010101,
    0x1000000, 0x1000101, 0x10100, 0x100, 0x1000001, 1, 0x1010101, 0x1010000, 0, 0x1010001,
    0x1000000, 0x10000, 0x101, 0x1000100, 0x10001, 0x10101, 0x1010100, 0x1000000, 0x10001,
    0x1000101, 0, 0x10100, 0x1000001, 1, 0x1010100, 0x100, 0x1010101, 0x10101, 0x1010000,
    0x1010001, 0x1000100, 0x10000, 0x101,
];
const SBOX_3: [u32; 64] = [
    0x1010100, 0x1000101, 0x10101, 0x1010000, 0, 0x10100, 0x1000001, 0x10001, 0x1000000, 0x10000,
    1, 0x1000100, 0x1010001, 0x101, 0x100, 0x1010101, 0x1000101, 1, 0x1010001, 0x1000100, 0x10100,
    0x1010101, 0, 0x1010000, 0x100, 0x1010100, 0x10000, 0x101, 0x1000000, 0x10001, 0x10101,
    0x1000001, 0x10001, 0x10100, 0x1000001, 0, 0x101, 0x1010001, 0x1010100, 0x1000101, 0x1010101,
    0x1000000, 0x1010000, 0x10101, 0x1000100, 0x10000, 1, 0x100, 0x1010000, 0x1010101, 0, 0x10100,
    0x10001, 0x1000000, 0x1000101, 1, 0x1000001, 0x100, 0x1000100, 0x1010001, 0x101, 0x1010100,
    0x10000, 0x10101,
];
const SBOX_4: [u32; 64] = [
    0x10000, 0x101, 0x100, 0x1000000, 0x1010100, 0x10001, 0x1010001, 0x10100, 1, 0x1000100,
    0x1010000, 0x1010101, 0x1000101, 0, 0x10101, 0x1000001, 0x10101, 0x1010001, 0x10000, 0x101,
    0x100, 0x1010100, 0x1000101, 0x1000000, 0x1000100, 0, 0x1010101, 0x10001, 0x1010000, 0x1000001,
    1, 0x10100, 0x100, 0x10000, 0x1000000, 0x1010001, 0x10001, 0x1000101, 0x1010100, 1, 0x1010101,
    0x1000001, 0x101, 0x1000100, 0x10100, 0x1010000, 0, 0x10101, 0x1010001, 1, 0x101, 0x1010100,
    0x1000000, 0x10101, 0x10000, 0x1000101, 0x10100, 0x1010101, 0, 0x1000001, 0x10001, 0x100,
    0x1000100, 0x1010000,
];
const SBOX_5: [u32; 64] = [
    0x101, 0x1000000, 0x10001, 0x1010101, 0x1000001, 0x10000, 0x10100, 1, 0, 0x1000101, 0x1010000,
    0x100, 0x10101, 0x1010100, 0x1000100, 0x1010001, 0x10001, 0x1010101, 0x100, 0x10000, 0x1010100,
    0x101, 0x1000001, 0x1000100, 0x10100, 0x1000000, 0x1000101, 0x10101, 0, 0x1010001, 0x1010000,
    1, 0x1000001, 0x10101, 0x1010101, 0x1000100, 0x10000, 1, 0x101, 0x1010000, 0x1010100, 0, 0x100,
    0x10001, 0x1000000, 0x1000101, 0x1010001, 0x10100, 0x100, 0x1010000, 0x10000, 0x101, 0x1000001,
    0x1000100, 0x1010101, 0x10001, 0x1010001, 0x10101, 0x1000000, 0x1010100, 0x10100, 0, 1,
    0x1000101,
];
const SBOX_6: [u32; 64] = [
    0x100, 0x1010001, 0x10000, 0x10101, 0x1010101, 0, 1, 0x1000101, 0x1010000, 0x101, 0x1000001,
    0x1010100, 0x1000100, 0x10001, 0x10100, 0x1000000, 0x1000101, 0, 0x1010001, 0x1010100, 0x100,
    0x1000001, 0x1000000, 0x10001, 0x10101, 0x1010000, 0x1000100, 0x101, 0x10000, 0x1010101, 1,
    0x10100, 0x1000000, 0x100, 0x1010001, 0x1000101, 0x101, 0x1010000, 0x1010100, 0x10101, 0x10001,
    0x1010101, 0x10100, 1, 0, 0x1000100, 0x1000001, 0x10000, 0x10100, 0x1010001, 0x1000101, 1,
    0x1000000, 0x100, 0x10001, 0x1010100, 0x1000001, 0x1000100, 0, 0x1010101, 0x10101, 0x10000,
    0x1010000, 0x101,
];
const SBOX_7: [u32; 64] = [
    0x1000101, 0x10000, 1, 0x100, 0x10100, 0x1010101, 0x1010001, 0x1000000, 0x10001, 0x1000001,
    0x1010000, 0x10101, 0x1000100, 0, 0x101, 0x1010100, 0x1000000, 0x1010101, 0x1000101, 1,
    0x10001, 0x1010000, 0x1010100, 0x100, 0x101, 0x1000100, 0x10100, 0x1010001, 0, 0x10101,
    0x1000001, 0x10000, 0x1010100, 0x1010001, 0x100, 0x1000000, 0x1000001, 0x101, 0x10101, 0x10000,
    0, 0x10100, 0x10001, 0x1000101, 0x1010101, 0x1010000, 0x1000100, 1, 0x10000, 0x1000000,
    0x10101, 0x1010100, 0x100, 0x10001, 1, 0x1000101, 0x1010101, 0x101, 0x1000001, 0, 0x1010000,
    0x1000100, 0x10100, 0x1010001,
];

const SBOXES: [&[u32; 64]; 8] = [
    &SBOX_0, &SBOX_1, &SBOX_2, &SBOX_3, &SBOX_4, &SBOX_5, &SBOX_6, &SBOX_7,
];

/// DES-like Feistel round (C# InitialDecode_Sub).
fn initial_decode_sub(p0: &mut [u8], p1: &mut [u8; 64], p2: i32) {
    let mut num: i32 = 0;
    let mut num2: i32 = 15;
    let mut buffer = [0u8; 48];

    loop {
        // Key expansion
        let mut index = 0usize;
        loop {
            let num4 = if p2 == 0 { num2 as usize } else { num as usize };
            let num5 = p0[48 * num4 + index] ^ p1[(EXPANSION_MATRIX[index] as usize) + 31];
            buffer[index] = num5;
            index += 1;
            if index >= 48 {
                break;
            }
        }

        // S-box lookups
        let mut num_array = [0u32; 8];
        for s in 0..8 {
            let base = s * 6;
            let idx = buffer[base + 4] as usize
                | (2 * (buffer[base + 3] as usize
                    | (2 * (buffer[base + 2] as usize
                        | (2 * (buffer[base + 1] as usize
                            | (2 * (buffer[base + 5] as usize | (2 * buffer[base] as usize)))))))));
            num_array[s] = SBOXES[s][idx];
        }

        // Build destination array
        let mut dest = [0u8; 32];
        for i in 0..8 {
            let bytes = num_array[i].to_le_bytes();
            dest[i * 4] = bytes[0];
            dest[i * 4 + 1] = bytes[1];
            dest[i * 4 + 2] = bytes[2];
            dest[i * 4 + 3] = bytes[3];
        }

        // Permutation step
        let mut num7 = 0usize;
        if num2 <= 0 {
            let mut remaining = 32;
            while remaining > 0 {
                p1[num7] ^= dest[(PERMUTATION[num7] as usize) - 1];
                num7 += 1;
                remaining -= 1;
            }
        } else {
            let mut remaining = 32;
            while remaining > 0 {
                let num10 = p1[num7 + 32];
                let num11 = p1[num7] ^ dest[(PERMUTATION[num7] as usize) - 1];
                num7 += 1;
                remaining -= 1;
                p1[num7 + 31] = num11;
                p1[num7 - 1] = num10;
            }
        }
        num2 -= 1;
        num += 1;

        if num2 <= -1 {
            break;
        }
    }
}

/// Read big-endian i32 from 4 bytes.
fn get_real_length(buf: &[u8], offset: usize) -> i32 {
    ((buf[offset] as i32) << 24)
        + ((buf[offset + 1] as i32) << 16)
        + ((buf[offset + 2] as i32) << 8)
        + (buf[offset + 3] as i32)
}

/// ChaosExpansion Layer 1 — DES-like block cipher decrypt.
fn decode_layer1(key: &[u8; 768], input: &[u8], buf_len: usize, output: &mut Vec<u8>) {
    let length = buf_len - 20;
    let mut main_counter = (length + 7) >> 3;
    let mut num3 = 0usize;
    let mut output_index = 0usize;

    // Skip first 20 bytes (header)
    output.resize(length, 0);
    output[..length].copy_from_slice(&input[20..20 + length]);

    loop {
        let mut plain_byte_block = [0u8; 64];
        let mut index = 0usize;
        let mut counter = 8;

        // Separate bytes into bits
        loop {
            let num8 = output[num3];
            num3 += 1;
            plain_byte_block[index] = (num8 >> 7) & 1;
            let mut num9 = index + 1;
            plain_byte_block[num9] = (num8 >> 6) & 1;
            num9 += 1;
            plain_byte_block[num9] = (num8 >> 5) & 1;
            num9 += 1;
            plain_byte_block[num9] = (num8 >> 4) & 1;
            num9 += 1;
            plain_byte_block[num9] = (num8 >> 3) & 1;
            num9 += 1;
            plain_byte_block[num9] = (num8 >> 2) & 1;
            num9 += 1;
            plain_byte_block[num9] = (num8 >> 1) & 1;
            num9 += 1;
            plain_byte_block[num9] = num8 & 1;
            index = num9 + 1;
            counter -= 1;
            if counter <= 0 {
                break;
            }
        }

        // Process byte block (decode direction: p2=0)
        let mut key_copy = key.to_vec();
        initial_decode_sub(&mut key_copy, &mut plain_byte_block, 0);

        // Merge bits back to bytes
        let mut counter2 = 0usize;
        loop {
            let b = plain_byte_block[counter2 + 7]
                | (2 * (plain_byte_block[counter2 + 6]
                    | (2 * (plain_byte_block[counter2 + 5]
                        | (2 * (plain_byte_block[counter2 + 4]
                            | (2 * (plain_byte_block[counter2 + 3]
                                | (2 * (plain_byte_block[counter2 + 2]
                                    | (2 * (plain_byte_block[counter2 + 1]
                                        | (2 * plain_byte_block[counter2])))))))))))));
            output[output_index] = b;
            output_index += 1;
            counter2 += 8;
            if counter2 >= 64 {
                break;
            }
        }

        main_counter -= 1;
        if main_counter == 0 {
            break;
        }
    }
}

/// ChaosExpansion Layer 2 — XOR stream cipher.
fn decode_layer2(data: &mut [u8]) {
    let mut volatile_key: u16 = 0x0418;
    for byte in data.iter_mut() {
        let raw_byte = *byte;
        let temporary_key = ((volatile_key & 0xff00) >> 8) as u8;
        let decrypted = temporary_key ^ raw_byte;
        volatile_key = ((raw_byte as u16).wrapping_add(volatile_key))
            .wrapping_mul(0x8041)
            .wrapping_add(0x1804);
        *byte = decrypted;
    }
}

/// Standart encryption — XOR stream cipher (v1098-1534).
fn decode_standart(data: &mut [u8]) {
    let mut volatile_key: u16 = 0x0816;
    for byte in data.iter_mut() {
        let raw_byte = *byte;
        let temporary_key = ((volatile_key & 0xff00) >> 8) as u8;
        let decrypted = temporary_key ^ raw_byte;
        volatile_key = ((raw_byte as u16).wrapping_add(volatile_key))
            .wrapping_mul(0x6081)
            .wrapping_add(0x1608);
        *byte = decrypted;
    }
}

/// ChaosExpansion file header magic bytes.
const CHAOS_HEADER: [u8; 16] = [
    0x4C, 0x26, 0x43, 0x7F, 0x80, 0xF1, 0x57, 0x98, 0x79, 0xFC, 0xAF, 0x26, 0x86, 0xD6, 0x20, 0x8E,
];

/// Encryption type detected from file.
#[derive(Debug, Clone, Copy)]
pub enum EncryptionType {
    ChaosExpansion,
    Standart,
    None,
}

/// Detect encryption type from file bytes.
pub fn detect_encryption(data: &[u8]) -> EncryptionType {
    if data.len() >= 20 && data[..16] == CHAOS_HEADER {
        EncryptionType::ChaosExpansion
    } else {
        // Try standart first — if decoded data looks valid (column_count > 0, < 1000), use it
        let mut test = data.to_vec();
        decode_standart(&mut test);
        if test.len() >= 4 {
            let col_count = i32::from_le_bytes([test[0], test[1], test[2], test[3]]);
            if col_count > 0 && col_count < 500 {
                return EncryptionType::Standart;
            }
        }
        // Try no encryption
        if data.len() >= 4 {
            let col_count = i32::from_le_bytes([data[0], data[1], data[2], data[3]]);
            if col_count > 0 && col_count < 500 {
                return EncryptionType::None;
            }
        }
        // Default to standart
        EncryptionType::Standart
    }
}

/// Decrypt a .tbl file and return the plaintext table bytes.
///
/// Returns `(decrypted_bytes, is_new_structure)`.
pub fn decrypt_tbl(data: &[u8]) -> anyhow::Result<(Vec<u8>, bool)> {
    let enc = detect_encryption(data);
    match enc {
        EncryptionType::ChaosExpansion => {
            let real_len = get_real_length(data, 16) as usize;
            let mut output = Vec::new();
            decode_layer1(&KEY, data, data.len(), &mut output);
            decode_layer2(&mut output);
            if real_len <= output.len() {
                output.truncate(real_len);
            }
            Ok((output, true))
        }
        EncryptionType::Standart => {
            let mut output = data.to_vec();
            decode_standart(&mut output);
            Ok((output, false))
        }
        EncryptionType::None => Ok((data.to_vec(), false)),
    }
}

/// ChaosExpansion Layer 2 — XOR stream cipher ENCRYPT (reverse of decode).
fn encode_layer2(data: &mut [u8]) {
    let mut volatile_key: u16 = 0x0418;
    for byte in data.iter_mut() {
        let plain_byte = *byte;
        let temporary_key = ((volatile_key & 0xff00) >> 8) as u8;
        let encrypted = temporary_key ^ plain_byte;
        volatile_key = ((encrypted as u16).wrapping_add(volatile_key))
            .wrapping_mul(0x8041)
            .wrapping_add(0x1804);
        *byte = encrypted;
    }
}

/// ChaosExpansion Layer 1 — DES-like block cipher ENCRYPT.
fn encode_layer1(key: &[u8; 768], input: &[u8]) -> Vec<u8> {
    let length = input.len();
    let padded_len = length.div_ceil(8) * 8;
    let mut data = vec![0u8; padded_len];
    data[..length].copy_from_slice(input);

    let block_count = padded_len / 8;
    let mut output = vec![0u8; padded_len];

    for block in 0..block_count {
        let offset = block * 8;
        let mut plain_byte_block = [0u8; 64];

        // Separate bytes into bits
        for i in 0..8 {
            let b = data[offset + i];
            let base = i * 8;
            plain_byte_block[base] = (b >> 7) & 1;
            plain_byte_block[base + 1] = (b >> 6) & 1;
            plain_byte_block[base + 2] = (b >> 5) & 1;
            plain_byte_block[base + 3] = (b >> 4) & 1;
            plain_byte_block[base + 4] = (b >> 3) & 1;
            plain_byte_block[base + 5] = (b >> 2) & 1;
            plain_byte_block[base + 6] = (b >> 1) & 1;
            plain_byte_block[base + 7] = b & 1;
        }

        // Encrypt (p2=1 for encode direction)
        let mut key_copy = key.to_vec();
        initial_decode_sub(&mut key_copy, &mut plain_byte_block, 1);

        // Merge bits back to bytes
        for i in 0..8 {
            let base = i * 8;
            output[offset + i] = plain_byte_block[base + 7]
                | (2 * (plain_byte_block[base + 6]
                    | (2 * (plain_byte_block[base + 5]
                        | (2 * (plain_byte_block[base + 4]
                            | (2 * (plain_byte_block[base + 3]
                                | (2 * (plain_byte_block[base + 2]
                                    | (2 * (plain_byte_block[base + 1]
                                        | (2 * plain_byte_block[base])))))))))))));
        }
    }

    output
}

/// Write big-endian i32 to 4 bytes.
fn put_real_length(buf: &mut [u8], offset: usize, value: i32) {
    buf[offset] = ((value >> 24) & 0xff) as u8;
    buf[offset + 1] = ((value >> 16) & 0xff) as u8;
    buf[offset + 2] = ((value >> 8) & 0xff) as u8;
    buf[offset + 3] = (value & 0xff) as u8;
}

/// Encrypt plaintext table bytes into ChaosExpansion .tbl format.
pub fn encrypt_tbl(plaintext: &[u8]) -> Vec<u8> {
    let real_len = plaintext.len();

    // Layer 2: XOR stream encrypt
    let mut layer2_data = plaintext.to_vec();
    encode_layer2(&mut layer2_data);

    // Layer 1: DES-like block encrypt
    let encrypted_body = encode_layer1(&KEY, &layer2_data);

    // Build final output: 16-byte header + 4-byte real_length + encrypted body
    let mut output = Vec::with_capacity(20 + encrypted_body.len());
    output.extend_from_slice(&CHAOS_HEADER);
    let mut len_buf = [0u8; 4];
    put_real_length(&mut len_buf, 0, real_len as i32);
    output.extend_from_slice(&len_buf);
    output.extend_from_slice(&encrypted_body);

    output
}
