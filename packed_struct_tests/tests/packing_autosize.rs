extern crate packed_struct;
#[macro_use]
extern crate packed_struct_codegen;

use packed_struct::*;

#[test]
fn test_serialization_autosize_msb0() {

    #[derive(PackedStruct, PartialEq, Debug)]
    #[packed_struct(bit_numbering="msb0")]
    pub struct Bools {
        #[packed_field(bits="8")]
        bool2: bool,
        #[packed_field(bits="0")]
        bool1: bool
    }

    let b = Bools {
            bool1: true,
            bool2: true
        };

    let packed = b.pack();
    assert_eq!(&packed, &[0b10000000, 0b10000000]);

    let unpacked = Bools::unpack(&packed).unwrap();
    assert_eq!(&unpacked, &b);
}

#[test]
fn test_serialization_autosize_lsb0() {
    // todo: LSB0 mode should be able to figure out its own size
    #[derive(PackedStruct, PartialEq, Debug)]
    #[packed_struct(size_bytes="2", bit_numbering="lsb0")]
    pub struct Bools {
        #[packed_field(bits="15")]
        bool2: bool,
        #[packed_field(bits="7")]
        bool1: bool
    }

    let b = Bools {
            bool1: true,
            bool2: true
        };

    let packed = b.pack();
    assert_eq!(&packed, &[0b10000000, 0b10000000]);

    let unpacked = Bools::unpack(&packed).unwrap();
    assert_eq!(&unpacked, &b);
}
