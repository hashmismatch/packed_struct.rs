extern crate packed_struct;
#[macro_use]
extern crate packed_struct_codegen;

use packed_struct::prelude::*;

#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PrimitiveEnum_u8)]	
#[repr(u8)]
pub enum DataRate {
    /// No output data is produced.
    PowerDown = 0,
    Rate3_125Hz = 1,
    Rate6_25Hz = 2,
    Rate12_5Hz = 3,
    Rate25Hz = 4,
    Rate50Hz = 5,
    Rate100Hz = 6,
    Rate400Hz = 7,
    Rate800Hz = 8,
    Rate1600Hz = 9
}

// Imaginary register, for test purposes only
#[derive(PackedStruct, Copy, Clone, Debug, PartialEq)]
#[packed_struct(bit_numbering="msb0")]
/// Control register 4
pub struct ControlRegister4 {
    /// Data rate selection
    #[packed_field(bits="1:4", ty="enum")]
    pub output_data_rate: DataRate, 		
    /// Z-axis enabled?
    #[packed_field(bits="6")]
    pub z_axis_enabled: bool
}

#[test]
fn test_reg() {
    let r = ControlRegister4 {
        output_data_rate: DataRate::Rate6_25Hz,
        z_axis_enabled: true
    };

    let b = r.pack().unwrap();
    assert_eq!([0b00010010], b);
}