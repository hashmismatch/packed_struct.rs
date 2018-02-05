extern crate packed_struct;
#[macro_use]
extern crate packed_struct_codegen;

use packed_struct::prelude::*;

#[test]
fn test_serialization_codegen() {

    #[derive(PackedStruct, PartialEq, Debug)]
    #[packed_struct(size_bytes="4", bit_numbering="msb0")]
    pub struct Bools {
        #[packed_field(bits="0")]
        bool1: bool,
        #[packed_field(bits="3:10")]
        num1: u8,
        #[packed_field(bits="11:18")]
        num2: u8,
        #[packed_field(bits="29")]
        bool2: bool,
        #[packed_field(bits="31")]
        bool3: bool 
    }

    let b = Bools {
            bool1: true,
            num1: 0b10100101,
            num2: 0b01010101,
            bool2: true,
            bool3: true
        };

    let packed = b.pack();
    assert_eq!(&packed, &[0b10010100, 0b10101010, 0b10100000, 0b00000101]);

    let unpacked = Bools::unpack(&packed).unwrap();
    assert_eq!(&unpacked, &b);
}