extern crate packed_struct;
#[macro_use]
extern crate packed_struct_codegen;

use packed_struct::prelude::*;

// both orders (high-low, low-high) should be supported!

#[derive(PackedStruct, Copy, Clone, Debug, PartialEq)]
#[packed_struct(size_bytes="1", bit_numbering="lsb0")]
pub struct IntsLsbPosBits {
    #[packed_field(bits="0:3")]
    num1: Integer<u8, packed_bits::Bits4>,
    #[packed_field(bits="7:4")]
    num2: Integer<u8, packed_bits::Bits4>    
}

#[test]
fn test_pos() {
    let s = IntsLsbPosBits {
        num1: 9.into(),
        num2: 28.into()
    };

    let packed = s.pack();
    let unpacked = IntsLsbPosBits::unpack(&packed).unwrap();

    assert_eq!(unpacked, s);
}