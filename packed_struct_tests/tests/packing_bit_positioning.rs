extern crate packed_struct;
#[macro_use]
extern crate packed_struct_codegen;

use packed_struct::*;


#[derive(PackedStruct, PartialEq, Debug)]
#[packed_struct(bit_numbering="msb0")]
pub struct SmallInts {    
    #[packed_field(bits="0..2")]
    pub val1: UIntBits3,
    #[packed_field(bits="3..4")]
    pub val2: UIntBits2,
    pub val3: bool,
    #[packed_field(bits="6")]
    pub val4: bool,
    #[packed_field(bits="7..")]
    pub val5: bool
}

#[test]
fn test_packing_bit_positions() {
    let a = SmallInts {
        val1: 1.into(),
        val2: 1.into(),
        val3: true,
        val4: true,
        val5: true
    };

    let packed = a.pack();
    println!("packed: {:?}", packed);

    let unpacked = SmallInts::unpack(&packed).unwrap();
    assert_eq!(a, unpacked);
}



#[derive(PackedStruct, PartialEq, Debug)]
#[packed_struct(size_bytes="1", bit_numbering="lsb0")]
pub struct SmallIntsLsb {    
    #[packed_field(bits="2..0")]
    pub val1: UIntBits3,
    #[packed_field(bits="6")]
    pub val2: bool
}

#[test]
fn test_packing_bit_positions_lsb() {
    let a = SmallIntsLsb {
        val1: 0b111.into(),
        val2: true
    };

    let packed = a.pack();
    println!("packed: {:?}", packed);
    assert_eq!(&[0b01000111], &packed);

    let unpacked = SmallIntsLsb::unpack(&packed).unwrap();
    assert_eq!(a, unpacked);
}
