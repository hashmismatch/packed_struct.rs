#![allow(clippy::enum_clike_unportable_variant)]
use packed_struct::prelude::*;

#[derive(PrimitiveEnum, PartialEq, Eq, Debug, Clone, Copy)]
pub enum EnumU8 {
    VariantMin = 0,
    VariantMax = 255
}

#[derive(PrimitiveEnum, PartialEq, Eq, Debug, Clone, Copy)]
pub enum EnumU16 {
    VariantMin = 0,
    VariantMax = 65535
}

#[repr(u32)]
#[derive(PrimitiveEnum, PartialEq, Eq, Debug, Clone, Copy)]
pub enum EnumU32 {
    VariantMin = 0,
    VariantMax = 4294967295
}

#[cfg(target_pointer_width = "64")]
#[derive(PrimitiveEnum, PartialEq, Eq, Debug, Clone, Copy)]
pub enum EnumU64 {
    VariantMin = 0,
    VariantMax = 1844674407370955165
}

#[derive(PrimitiveEnum, PartialEq, Eq, Debug, Clone, Copy)]
pub enum EnumI8 {
    VariantMin = -128,
    VariantMax = 127
}

#[derive(PrimitiveEnum, PartialEq, Eq, Debug, Clone, Copy)]
pub enum EnumI16 {
    VariantMin = -32768,
    VariantMax = 32767
}

#[derive(PrimitiveEnum, PartialEq, Eq, Debug, Clone, Copy)]
pub enum EnumI32 {
    VariantMin = -2147483648,
    VariantMax = 2147483647
}

#[cfg(target_pointer_width = "64")]
#[derive(PrimitiveEnum, PartialEq, Eq, Debug, Clone, Copy)]
pub enum EnumI64 {
    VariantMin = -9223372036854775808,
    VariantMax = 9223372036854775807
}

#[test]
fn prim_ty() {
    assert_eq!(0_u8, EnumU8::VariantMin.to_primitive());
    assert_eq!(255_u8, EnumU8::VariantMax.to_primitive());

    assert_eq!(0_u16, EnumU16::VariantMin.to_primitive());
    assert_eq!(65535_u16, EnumU16::VariantMax.to_primitive());

    assert_eq!(0_u32, EnumU32::VariantMin.to_primitive());
    assert_eq!(4294967295_u32, EnumU32::VariantMax.to_primitive());

    #[cfg(target_pointer_width = "64")]
    {
        assert_eq!(0_u64, EnumU64::VariantMin.to_primitive());
        assert_eq!(1844674407370955165_u64, EnumU64::VariantMax.to_primitive());
    }

    assert_eq!(-128_i8, EnumI8::VariantMin.to_primitive());
    assert_eq!(127_i8, EnumI8::VariantMax.to_primitive());

    assert_eq!(-32768_i16, EnumI16::VariantMin.to_primitive());
    assert_eq!(32767_i16, EnumI16::VariantMax.to_primitive());    

    assert_eq!(-2147483648_i32, EnumI32::VariantMin.to_primitive());
    assert_eq!(2147483647_i32, EnumI32::VariantMax.to_primitive());

    #[cfg(target_pointer_width = "64")]
    {
        assert_eq!(-9223372036854775808_i64, EnumI64::VariantMin.to_primitive());
        assert_eq!(9223372036854775807_i64, EnumI64::VariantMax.to_primitive());
    }
}

#[derive(PrimitiveEnum, PartialEq, Eq, Debug, Clone, Copy)]
pub enum EnumMixedI16 {
    VariantMin = -1,
    VariantMax = 200
}

#[derive(PrimitiveEnum, PartialEq, Eq, Debug, Clone, Copy)]
pub enum EnumMixedI32 {
    VariantMin = -1,
    VariantMax = 40000
}

#[repr(i64)]
#[derive(PrimitiveEnum, PartialEq, Eq, Debug, Clone, Copy)]
pub enum EnumLargeNegativeI64 {
    VariantMin = -3000000000,
    VariantMax = 0
}

#[test]
fn prim_ty_inferred_from_min_and_max() {
    assert_eq!(-1_i16, EnumMixedI16::VariantMin.to_primitive());
    assert_eq!(200_i16, EnumMixedI16::VariantMax.to_primitive());
    assert_eq!(Some(EnumMixedI16::VariantMax), EnumMixedI16::from_primitive(200));

    assert_eq!(-1_i32, EnumMixedI32::VariantMin.to_primitive());
    assert_eq!(40000_i32, EnumMixedI32::VariantMax.to_primitive());
    assert_eq!(Some(EnumMixedI32::VariantMax), EnumMixedI32::from_primitive(40000));

    assert_eq!(-3000000000_i64, EnumLargeNegativeI64::VariantMin.to_primitive());
    assert_eq!(Some(EnumLargeNegativeI64::VariantMin), EnumLargeNegativeI64::from_primitive(-3000000000));
}

#[repr(u16)]
#[derive(PrimitiveEnum, PartialEq, Eq, Debug, Clone, Copy)]
pub enum EnumSuffixedU16 {
    VariantMin = 0u16,
    VariantMax = 1
}

#[test]
fn prim_ty_from_literal_suffix() {
    assert_eq!(0_u16, EnumSuffixedU16::VariantMin.to_primitive());
    assert_eq!(1_u16, EnumSuffixedU16::VariantMax.to_primitive());
}
