//! Issue #92: `bit_numbering="lsb0"` numbers bits across the whole byte array as one big-endian
//! number, bit 0 being the LSB of the last byte. `endian` only affects the byte order inside
//! multi-byte fields, it doesn't turn the structure into a little-endian register.

use packed_struct::prelude::*;

#[derive(Copy, Clone, Debug, Eq, PartialEq, PackedStruct)]
#[packed_struct(size_bytes="4", bit_numbering="lsb0", endian="msb")]
pub struct Pack {
    #[packed_field(bits="0..4")]
    pub a: u8,
    #[packed_field(bits="4..8")]
    pub b: u8,
    #[packed_field(bits="8..12")]
    pub c: u8,
    #[packed_field(bits="12..16")]
    pub d: u8,
    #[packed_field(bits="16..32")]
    pub e: u16,
}

/// The struct from the issue report.
#[derive(Copy, Clone, Debug, Eq, PartialEq, PackedStruct)]
#[packed_struct(size_bytes="4", bit_numbering="lsb0", endian="lsb")]
pub struct PackLsbEndian {
    #[packed_field(bits="0..4")]
    pub a: u8,
    #[packed_field(bits="4..8")]
    pub b: u8,
    #[packed_field(bits="8..12")]
    pub c: u8,
    #[packed_field(bits="12..16")]
    pub d: u8,
    #[packed_field(bits="16..32")]
    pub e: u16,
}

#[test]
fn lsb0_is_big_endian_across_the_struct() {
    let fields = Pack { a: 0x8, b: 0x7, c: 0x6, d: 0x5, e: 0x1234 };

    let packed_bytes = fields.pack().unwrap();
    assert_eq!(packed_bytes, [0x12, 0x34, 0x56, 0x78]);
    assert_eq!(u32::from_be_bytes(packed_bytes), 0x12345678);

    assert_eq!(Pack::unpack(&packed_bytes).unwrap(), fields);
}

#[test]
fn little_endian_register() {
    let wire = 0x12345678u32.to_le_bytes();

    let unpacked = Pack::unpack(&u32::from_le_bytes(wire).to_be_bytes()).unwrap();
    assert_eq!(unpacked, Pack { a: 0x8, b: 0x7, c: 0x6, d: 0x5, e: 0x1234 });

    let repacked = u32::from_be_bytes(unpacked.pack().unwrap()).to_le_bytes();
    assert_eq!(repacked, wire);
}

#[test]
fn lsb_endian_only_swaps_bytes_inside_fields() {
    // The reporter's input. The nibbles still come from the last two bytes, and `e` from the
    // first two, with its two bytes swapped.
    let wire = 0x12345678u32.to_le_bytes();
    assert_eq!(wire, [0x78, 0x56, 0x34, 0x12]);

    let unpacked = PackLsbEndian::unpack(&wire).unwrap();
    assert_eq!(unpacked, PackLsbEndian { a: 0x2, b: 0x1, c: 0x4, d: 0x3, e: 0x5678 });
    assert_eq!(unpacked.pack().unwrap(), wire);
}
