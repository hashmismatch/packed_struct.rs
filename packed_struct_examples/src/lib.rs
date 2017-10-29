extern crate packed_struct;
#[macro_use]
extern crate packed_struct_codegen;

use packed_struct::*;

/// MultiWii status structure
#[derive(PackedStruct, PartialEq, Debug)]
#[packed_struct(size_bytes="4", endian="lsb")]
pub struct MspStatus {
    /// Cycle time, milliseconds
    pub cycle_time: u16,
    /// Number of I2C bus errors since last reboot
    pub i2c_errors: u16
}

#[derive(PackedStruct, PartialEq, Debug)]
#[packed_struct(size_bytes="1", bit_numbering="lsb0")]
pub struct SmallIntsLsb {    
    #[packed_field(bits="2..0")]
    pub val1: UIntBits3,
    #[packed_field(bits="6")]
    pub val2: bool
}


#[derive(PackedStruct, Debug, PartialEq)]
pub struct RoundtripAligned {
    f1: u8,
    f2: i8,
    
    #[packed_field(endian="msb")]
    f3: u16,
    #[packed_field(endian="msb")]
    f4: i16,

    #[packed_field(endian="lsb")]
    f5: u16,
    #[packed_field(endian="lsb")]
    f6: i16,

    #[packed_field(endian="msb")]
    f7: u32,
    #[packed_field(endian="msb")]
    f8: i32,

    #[packed_field(endian="lsb")]
    f9: u32,
    #[packed_field(endian="lsb")]
    f10: i32,

    #[packed_field(endian="msb")]
    u64_1: u64,

    f11: [u8; 1],
    f12: [u8; 2],
    f13: [u8; 3],
    f14: [u8; 4],

    f15: bool
}