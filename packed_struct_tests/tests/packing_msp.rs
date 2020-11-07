extern crate packed_struct;
#[macro_use]
extern crate packed_struct_codegen;

use packed_struct::prelude::*;

#[test]
#[cfg(test)]
fn test_packed_struct_msp() {
    
    #[derive(PackedStruct, PartialEq, Debug)]
    #[packed_struct(size_bytes="4", endian="lsb")]
    pub struct MspStatus {
        cycle_time: u16,
        i2c_errors: u16
    }

    let reg = MspStatus {
        cycle_time: 0xAABB,
        i2c_errors: 1
    };

    let packed = reg.pack().unwrap();
    assert_eq!(&packed, &[0xBB, 0xAA, 0x01, 0x00]);

    let unpacked = MspStatus::unpack(&packed).unwrap();
    assert_eq!(&unpacked, &reg);
}

