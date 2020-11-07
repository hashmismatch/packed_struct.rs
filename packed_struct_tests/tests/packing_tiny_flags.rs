extern crate packed_struct;
#[macro_use]
extern crate packed_struct_codegen;

use packed_struct::prelude::*;

#[test]
fn test_tiny_flags() {

    #[derive(PackedStruct, Debug, PartialEq)]
    #[packed_struct(bit_numbering="msb0")]
    pub struct TinyFlags {
        #[packed_field(bits="4")]
        flag1: bool,
        val1: Integer<u8, ::packed_bits::Bits2>,
        flag2: bool
    }

    let flag = TinyFlags {
        flag1: true,
        val1: 3.into(),
        flag2: false
    };

    let packed = flag.pack().unwrap();
    assert_eq!([0b00001110], packed);
    let unpacked = TinyFlags::unpack(&packed).unwrap();
    assert_eq!(unpacked, flag);

    #[derive(PackedStruct, Debug, PartialEq)]
    #[packed_struct]
    pub struct Settings {
        #[packed_field(element_size_bits="4")]
        values: [TinyFlags; 4]
    }
    
    let example = Settings {
        values: [
            TinyFlags { flag1: true,  val1: 1.into(), flag2: false },
            TinyFlags { flag1: true,  val1: 2.into(), flag2: true },
            TinyFlags { flag1: false, val1: 3.into(), flag2: false },
            TinyFlags { flag1: true,  val1: 0.into(), flag2: false },
        ]
    };

    let packed = example.pack().unwrap();
    let unpacked = Settings::unpack(&packed).unwrap();

    assert_eq!(example, unpacked);
}