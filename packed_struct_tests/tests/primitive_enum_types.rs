extern crate packed_struct;
#[macro_use]
extern crate packed_struct_codegen;

use packed_struct::prelude::*;

#[derive(PrimitiveEnum, PartialEq, Debug, Clone, Copy)]
pub enum EnumU8 {
    VariantMin = 0,
    VariantMax = 255
}

#[derive(PrimitiveEnum, PartialEq, Debug, Clone, Copy)]
pub enum EnumU16 {
    VariantMin = 0,
    VariantMax = 65535
}

#[derive(PrimitiveEnum, PartialEq, Debug, Clone, Copy)]
pub enum EnumU32 {
    VariantMin = 0,
    VariantMax = 4294967295
}

#[cfg(target_pointer_width = "64")]
#[derive(PrimitiveEnum, PartialEq, Debug, Clone, Copy)]
pub enum EnumU64 {
    VariantMin = 0,
    VariantMax = 1844674407370955165
}

#[derive(PrimitiveEnum, PartialEq, Debug, Clone, Copy)]
pub enum EnumI8 {
    VariantMin = -128,
    VariantMax = 127
}

#[derive(PrimitiveEnum, PartialEq, Debug, Clone, Copy)]
pub enum EnumI16 {
    VariantMin = -32768,
    VariantMax = 32767
}

#[derive(PrimitiveEnum, PartialEq, Debug, Clone, Copy)]
pub enum EnumI32 {
    VariantMin = -2147483648,
    VariantMax = 2147483647
}

#[cfg(target_pointer_width = "64")]
#[derive(PrimitiveEnum, PartialEq, Debug, Clone, Copy)]
pub enum EnumI64 {
    VariantMin = -9223372036854775808,
    VariantMax = 9223372036854775807
}

use packed_struct::prelude::*;

#[test]
fn prim_ty() {
    assert_eq!(0 as u8, EnumU8::VariantMin.to_primitive());
    assert_eq!(255 as u8, EnumU8::VariantMax.to_primitive());

    assert_eq!(0 as u16, EnumU16::VariantMin.to_primitive());
    assert_eq!(65535 as u16, EnumU16::VariantMax.to_primitive());

    assert_eq!(0 as u32, EnumU32::VariantMin.to_primitive());
    assert_eq!(4294967295 as u32, EnumU32::VariantMax.to_primitive());

    #[cfg(target_pointer_width = "64")]
    {
        assert_eq!(0 as u64, EnumU64::VariantMin.to_primitive());
        assert_eq!(1844674407370955165 as u64, EnumU64::VariantMax.to_primitive());
    }

    assert_eq!(-128 as i8, EnumI8::VariantMin.to_primitive());
    assert_eq!(127 as i8, EnumI8::VariantMax.to_primitive());

    assert_eq!(-32768 as i16, EnumI16::VariantMin.to_primitive());
    assert_eq!(32767 as i16, EnumI16::VariantMax.to_primitive());    

    assert_eq!(-2147483648 as i32, EnumI32::VariantMin.to_primitive());
    assert_eq!(2147483647 as i32, EnumI32::VariantMax.to_primitive());

    #[cfg(target_pointer_width = "64")]
    {
        assert_eq!(-9223372036854775808 as i64, EnumI64::VariantMin.to_primitive());
        assert_eq!(9223372036854775807 as i64, EnumI64::VariantMax.to_primitive());
    }
}