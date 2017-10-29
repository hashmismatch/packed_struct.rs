extern crate packed_struct;
#[macro_use]
extern crate packed_struct_codegen;

use packed_struct::*;

#[test]
#[cfg(test)]
fn test_packed_primitives() {

    #[derive(PackedStruct)]
    #[packed_struct(size_bytes="6", bit_numbering="msb0")]
    pub struct Ints2 {
        #[packed_field(bits="2..17", endian="msb")]
        num1: u16,
        #[packed_field(bits="18")]
        bool1: bool,
        #[packed_field(bits="19..34", endian="lsb")]
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
