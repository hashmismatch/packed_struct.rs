//! Exclusive byte ranges, `bytes="x..y"`, must cover the same bytes as `bytes="x..=(y-1)"`.
//! https://github.com/hashmismatch/packed_struct.rs/issues/107

use packed_struct::prelude::*;

#[derive(Debug, Clone, PartialEq, PackedStruct)]
#[packed_struct(endian="lsb", bit_numbering="msb0", size_bytes="2")]
pub struct StructA {
    #[packed_field(bytes="0..=1", size_bytes="2")]
    field_a: u16,
}

#[derive(Debug, Clone, PartialEq, PackedStruct)]
#[packed_struct(endian="lsb", bit_numbering="msb0", size_bytes="2")]
pub struct StructB {
    #[packed_field(bytes="0..2", size_bytes="2")]
    field_b: u16,
}

#[derive(Debug, Clone, PartialEq, PackedStruct)]
#[packed_struct(endian="msb", bit_numbering="msb0", size_bytes="4")]
pub struct StructC {
    #[packed_field(bytes="0")]
    head: u8,
    #[packed_field(bytes="1..3")]
    mid: u16,
    #[packed_field(bytes="3..4")]
    tail: u8,
}

#[test]
fn issue_107() {
    let a = StructA { field_a: 0x1234 };
    let b = StructB { field_b: 0x1234 };

    let packed_a = a.pack().unwrap();
    let packed_b = b.pack().unwrap();
    assert_eq!([0x34, 0x12], packed_a);
    assert_eq!(packed_a, packed_b);

    assert_eq!(StructA::unpack(&packed_a).unwrap(), a);
    assert_eq!(StructB::unpack(&packed_b).unwrap(), b);
}

#[test]
fn exclusive_byte_range_in_the_middle() {
    let c = StructC { head: 0xAA, mid: 0x1234, tail: 0xBB };
    let packed = c.pack().unwrap();
    assert_eq!([0xAA, 0x12, 0x34, 0xBB], packed);
    assert_eq!(StructC::unpack(&packed).unwrap(), c);
}
