use std::any::type_name;

use packed_struct::{PrimitiveEnumStaticStr, prelude::*};

#[derive(PrimitiveEnum_u8, Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnumImplicit {
    Zero,
    One,
    Two,
    Three
}

#[derive(PrimitiveEnum_u8, Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnumExplicit {
    Zero = 0,
    One,
    Two,
    Three
}

#[derive(PrimitiveEnum_u8, Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnumExplicitHalf {
    Zero,
    One = 1,
    Two,
    Three
}

#[derive(PrimitiveEnum_u8, Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnumExplicitHalfZero {
    Zero = 0,
    One,
    Two = 2,
    Three
}

#[derive(PrimitiveEnum_u8, Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnumExplicitFull {
    Zero = 0,
    One = 1,
    Two = 2,
    Three = 3
}

#[derive(PrimitiveEnum_i8, Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnumExplicitNegative {
    MinusTwo = -2,
    MinusOne = -1,
    Zero,
    One = 1,
    Two = 2
}

#[derive(PrimitiveEnum_i8, Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnumExplicitNegativeOne {
    MinusOne = -1,
    Zero,
    One,
    Two
}


#[derive(PrimitiveEnum_i8, Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnumExplicitNegativeTwo {
    MinusTwo = -2,
    MinusOne,
    Zero,
    One,
    Two
}

#[test]
fn prim() {
    test_enum_positive::<EnumImplicit>();
    test_enum_positive::<EnumExplicit>();
    test_enum_positive::<EnumExplicitHalf>();
    test_enum_positive::<EnumExplicitHalfZero>();

    test_enum_negative::<EnumExplicitNegative>();
    test_enum_negative::<EnumExplicitNegativeOne>();
    test_enum_negative::<EnumExplicitNegativeTwo>();
}

fn test_enum_positive<E: PrimitiveEnum<Primitive = u8> + PrimitiveEnumStaticStr + std::fmt::Debug + PartialEq + 'static>() {
    for (i, x) in E::all_variants().iter().enumerate() {
        let prim = x.to_primitive();
        assert_eq!(prim, i as u8);

        let from_prim = E::from_primitive(prim).unwrap_or_else(|| panic!("Expected a successful parse of {}, ty {}", prim, type_name::<E>()));
        assert_eq!(from_prim, *x);

        let display_str = x.to_display_str();
        let from_display_str = E::from_str(display_str).unwrap_or_else(|| panic!("Expected a successful parse of display string {}, ty {}", display_str, type_name::<E>()));
        assert_eq!(from_display_str, *x);
    }
}

fn test_enum_negative<E: PrimitiveEnum<Primitive = i8> + PrimitiveEnumStaticStr + std::fmt::Debug + PartialEq + 'static>() {
    for x in E::all_variants().iter() {
        let prim = x.to_primitive();

        let from_prim = E::from_primitive(prim).unwrap_or_else(|| panic!("Expected a successful parse of {}, ty {}", prim, type_name::<E>()));
        assert_eq!(from_prim, *x);

        let display_str = x.to_display_str();
        let from_display_str = E::from_str(display_str).unwrap_or_else(|| panic!("Expected a successful parse of display string {}, ty {}", display_str, type_name::<E>()));
        assert_eq!(from_display_str, *x);
    }
}