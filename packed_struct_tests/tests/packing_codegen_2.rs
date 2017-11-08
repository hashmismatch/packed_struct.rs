extern crate packed_struct;
#[macro_use]
extern crate packed_struct_codegen;

use packed_struct::prelude::*;

#[derive(PackedStruct, Debug, PartialEq)]
#[packed_struct(bit_numbering="msb0")]
pub struct SampleRegister {
    #[packed_field(bits="1")]
    pub power_enabled: bool,
    #[packed_field(bits="3")]
    pub test_mode: bool,
    #[packed_field(bits="4..")]
    pub high_quality: bool,
    pub low_power: bool
}


#[test]
#[cfg(test)]
fn test_packed_struct_u8() {
   
    let reg = SampleRegister {
        power_enabled: true,
        test_mode: true,
        high_quality: false,
        low_power: true
    };

    let packed = reg.pack();
    assert_eq!(&[0b01010100], &packed);
}


#[derive(PackedStruct)]
#[packed_struct(size_bytes="6", bit_numbering="msb0")]
pub struct Ints {
    #[packed_field(bits="2..17", endian="msb")]
    num1: u16,
    #[packed_field(bits="18")]
    bool1: bool,
    #[packed_field(bits="19..34", endian="lsb")]
    num2: u16
}


#[test]
#[cfg(test)]
fn test_packed_struct_range() {
    {
        let i = Ints {
            num1: 0b1110101010101010,
            bool1: true,
            num2: 0b1101010111010101
        };

        let packed = i.pack();
        assert_eq!(&packed, &[0b00111010, 0b10101010, 0b10111010, 0b10111010, 0b10100000, 0]);
    }
}    
