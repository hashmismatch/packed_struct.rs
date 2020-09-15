extern crate packed_struct;
#[macro_use]
extern crate packed_struct_codegen;

use packed_struct::prelude::*;

#[test]
#[cfg(test)]
fn test_packed_arrays() {
    
    #[derive(PackedStruct, PartialEq, Debug)]
    #[packed_struct(size_bytes="7")]
    pub struct Arrays {
        arr1: [u8; 7]
    }

    let a = Arrays {
        arr1: [1, 2, 3, 4, 5, 6, 7]
    };

    let packed = a.pack();
    assert_eq!(&packed, &[1, 2, 3, 4, 5, 6, 7]);
    
    let unpacked = Arrays::unpack(&packed).unwrap();
    assert_eq!(&a, &unpacked);
}

#[test]
fn test_packed_array_of_structs() {

    #[derive(PackedStruct, Debug, PartialEq)]
    #[packed_struct(size_bytes="6", endian="msb")]
    pub struct Simple {
        f1: u16,
        f2: u32
    }

    #[derive(PackedStruct, Debug, PartialEq)]
    pub struct Packaged {
        #[packed_field(element_size_bytes="6")]
        p: [Simple; 4]
    }


    let p = Packaged {
        p: [
            Simple { f1: 1, f2: 2 },
            Simple { f1: 50000, f2: 6000000 },
            Simple { f1: 51000, f2: 6200000 },
            Simple { f1: 23029, f2: 75827424 },
        ]
    };

    let packed: [u8; 4*6] = p.pack();

    let unpacked = Packaged::unpack(&packed).unwrap();
    assert_eq!(&p, &unpacked);
}