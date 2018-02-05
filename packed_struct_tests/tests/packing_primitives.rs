extern crate packed_struct;
#[macro_use]
extern crate packed_struct_codegen;

use packed_struct::prelude::*;

#[test]
#[cfg(test)]
fn test_packed_primitives() {

    #[derive(PackedStruct)]
    #[packed_struct(size_bytes="6", bit_numbering="msb0")]
    pub struct Ints2 {
        #[packed_field(bits="2:17", endian="msb")]
        num1: u16,
        #[packed_field(bits="18")]
        bool1: bool,
        #[packed_field(bits="19:34", endian="lsb")]
        num2: u16
    }

    let i = Ints2 {
        num1: 0b1010101010101010,
        bool1: true,
        num2: 0b0101010111010101
    };

    let packed = i.pack();
    assert_eq!(&packed, &[0b00101010, 0b10101010, 0b10111010, 0b10101010, 0b10100000, 0]);
}

#[test]
#[cfg(test)]
fn test_packed_int_msb_endian() {

    #[derive(Copy, Clone, Debug, Default, PartialEq, PackedStruct)]
    #[packed_struct(endian="msb", bit_numbering="msb0")]
    pub struct IntsMsb {
        #[packed_field(bytes="0")]
        pub a: u8,
        #[packed_field(bytes="1")]
        pub b: i8,
        pub c: u16,
        pub d: i16,
        pub e: u32,
        pub f: i32,
        pub g: u64
    }

    let i = IntsMsb {
        a: 85,
        b: 85,
        ..Default::default()
    };

    let packed = i.pack();
    let unpacked = IntsMsb::unpack(&packed).unwrap();
    assert_eq!(i, unpacked);
}

#[test]
#[cfg(test)]
fn test_packed_int_lsb_endian() {

    #[derive(Copy, Clone, Debug, Default, PartialEq, PackedStruct)]
    #[packed_struct(endian="lsb", bit_numbering="msb0")]
    pub struct IntsLsb {
        #[packed_field(bytes="0")]
        pub a: u8,
        #[packed_field(bytes="1")]
        pub b: i8,
        pub c: u16,
        pub d: i16,
        pub e: u32,
        pub f: i32,
        pub g: u64
    }

    let i = IntsLsb {
        a: 85,
        b: 85,
        ..Default::default()
    };

    let packed = i.pack();
    let unpacked = IntsLsb::unpack(&packed).unwrap();
    assert_eq!(i, unpacked);
}
