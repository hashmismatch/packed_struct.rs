use packed_struct::{PrimitiveEnumStaticStr, prelude::*};

#[derive(PrimitiveEnum_u8, Clone, Copy, Debug, PartialEq)]
pub enum EnumImplicit {
    Zero,
    One,
    Two,
    Three
}

#[derive(PrimitiveEnum_u8, Clone, Copy, Debug, PartialEq)]
pub enum EnumExplicit {
    Zero = 0,
    One,
    Two,
    Three
}

#[derive(PrimitiveEnum_u8, Clone, Copy, Debug, PartialEq)]
pub enum EnumExplicitHalf {
    Zero,
    One = 1,
    Two,
    Three
}

#[derive(PrimitiveEnum_u8, Clone, Copy, Debug, PartialEq)]
pub enum EnumExplicitFull {
    Zero = 0,
    One = 1,
    Two = 2,
    Three = 3
}

#[test]
fn prim() {
    test_enum::<EnumImplicit>();
    test_enum::<EnumExplicit>();
    test_enum::<EnumExplicitHalf>();
    test_enum::<EnumExplicitFull>();
}

fn test_enum<E: PrimitiveEnum<Primitive = u8> + PrimitiveEnumStaticStr + std::fmt::Debug + PartialEq + 'static>() {
    for (i, x) in E::all_variants().iter().enumerate() {
        let prim = x.to_primitive();
        assert_eq!(prim, i as u8);

        let from_prim = E::from_primitive(prim).unwrap();
        assert_eq!(from_prim, *x);

        let display_str = x.to_display_str();
        let from_display_str = E::from_str(display_str).unwrap();
        assert_eq!(from_display_str, *x);
    }
}