//! A bit-by-bit reference implementation of field placement, used to check
//! the generated packing code.

#![allow(dead_code)]

/// Write a single bit at the MSB0 position `pos`.
pub fn set_bit_msb0(buf: &mut [u8], pos: usize, bit: bool) {
    let mask = 1u8 << (7 - (pos % 8));
    if bit {
        buf[pos / 8] |= mask;
    } else {
        buf[pos / 8] &= !mask;
    }
}

/// The sequence of bits a `width` wide integer occupies, in MSB0 stream order.
///
/// MSB integers are simply the value, most significant bit first. LSB integers
/// are their little endian bytes, with the unused top bits of the last byte
/// dropped. Integers of 8 bits or less are always MSB.
pub fn integer_bits(raw: u64, width: usize, lsb: bool) -> Vec<bool> {
    if !lsb || width <= 8 {
        return (0..width).map(|k| (raw >> (width - 1 - k)) & 1 == 1).collect();
    }

    let n = width.div_ceil(8);
    let pad = n * 8 - width;
    let mut bits = vec![];
    for j in 0..n {
        let byte = (raw >> (8 * j)) & 0xFF;
        let nb = if j == n - 1 { 8 - pad } else { 8 };
        for b in (0..nb).rev() {
            bits.push((byte >> b) & 1 == 1);
        }
    }
    bits
}

/// Place a `width` wide integer at the MSB0 bit position `start`.
pub fn set_integer(buf: &mut [u8], start: usize, width: usize, raw: u64, lsb: bool) {
    for (k, bit) in integer_bits(raw, width, lsb).into_iter().enumerate() {
        set_bit_msb0(buf, start + k, bit);
    }
}

/// Place a byte slice at the MSB0 bit position `start`.
pub fn set_bytes(buf: &mut [u8], start: usize, bytes: &[u8]) {
    for (i, b) in bytes.iter().enumerate() {
        set_integer(buf, start + i * 8, 8, *b as u64, false);
    }
}

pub fn mask(width: usize) -> u64 {
    if width >= 64 { !0 } else { (1u64 << width) - 1 }
}

/// Sign extend a `width` wide raw value.
pub fn sign_extend(raw: u64, width: usize) -> i64 {
    let s = 64 - width;
    ((raw << s) as i64) >> s
}

/// Interesting raw values for a `width` wide integer.
pub fn test_values(width: usize) -> Vec<u64> {
    let m = mask(width);
    let mut v = vec![
        0,
        1,
        m,
        m >> 1,
        1 << (width - 1),
        0xA5A5_A5A5_A5A5_A5A5 & m,
        0x5A5A_5A5A_5A5A_5A5A & m,
        0x0123_4567_89AB_CDEF & m,
        0xFEDC_BA98_7654_3210 & m,
    ];
    v.sort();
    v.dedup();
    v
}
