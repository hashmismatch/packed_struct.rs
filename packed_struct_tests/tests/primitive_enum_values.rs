use packed_struct::prelude::*;

#[derive(PrimitiveEnum, PartialEq, Debug, Clone, Copy)]
pub enum EnumI8 {
    VariantMin = -128,
    
    VariantNegA = -110,
    VariantNegB,
    VariantNegC,
    
    VariantMidA = -2,
    VariantMidB,
    VariantMidC,
    VariantMidD,
    VariantMidE,

    VariantPosA = 110,
    VariantPosB,
    VariantPosC,
    
    VariantMax = 127
}

#[test]
fn prim_var_ty() {
    assert_eq!(-128, EnumI8::VariantMin as i8);
    
    assert_eq!(-110, EnumI8::VariantNegA as i8);
    assert_eq!(-109, EnumI8::VariantNegB as i8);
    assert_eq!(-108, EnumI8::VariantNegC as i8);
    
    assert_eq!(-2, EnumI8::VariantMidA as i8);
    assert_eq!(-1, EnumI8::VariantMidB as i8);
    assert_eq!(0, EnumI8::VariantMidC as i8);
    assert_eq!(1, EnumI8::VariantMidD as i8);
    assert_eq!(2, EnumI8::VariantMidE as i8);

    assert_eq!(110, EnumI8::VariantPosA as i8);
    assert_eq!(111, EnumI8::VariantPosB as i8);
    assert_eq!(112, EnumI8::VariantPosC as i8);
    
    assert_eq!(127, EnumI8::VariantMax as i8);


    assert_eq!(Some(EnumI8::VariantMin), EnumI8::from_primitive(-128));
    
    assert_eq!(Some(EnumI8::VariantNegA), EnumI8::from_primitive(-110));
    assert_eq!(Some(EnumI8::VariantNegB), EnumI8::from_primitive(-109));
    assert_eq!(Some(EnumI8::VariantNegC), EnumI8::from_primitive(-108));
    
    assert_eq!(Some(EnumI8::VariantMidA), EnumI8::from_primitive(-2));
    assert_eq!(Some(EnumI8::VariantMidB), EnumI8::from_primitive(-1));
    assert_eq!(Some(EnumI8::VariantMidC), EnumI8::from_primitive(0));
    assert_eq!(Some(EnumI8::VariantMidD), EnumI8::from_primitive(1));
    assert_eq!(Some(EnumI8::VariantMidE), EnumI8::from_primitive(2));
    
    assert_eq!(Some(EnumI8::VariantPosA), EnumI8::from_primitive(110));
    assert_eq!(Some(EnumI8::VariantPosB), EnumI8::from_primitive(111));
    assert_eq!(Some(EnumI8::VariantPosC), EnumI8::from_primitive(112));
    
    assert_eq!(Some(EnumI8::VariantMax), EnumI8::from_primitive(127));
}

