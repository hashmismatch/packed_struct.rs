use packed_struct::prelude::*;

pub const A1: u16 = 0;
pub const A2: u16 = 1;
pub const B1: u16 = 100;
pub const C1: u16 = 200;

#[derive(PrimitiveEnum_u16, Clone, Copy, Debug, PartialEq)]
#[repr(u16)]
pub enum ConstEnum {
    A1 = A1,
    A2 = A2,
    B1 = B1,
    B2,
    C1 = C1,
    C2,
    D1 = 300,
    D2
}

#[test]
fn prim() {
    assert_eq!(0, ConstEnum::A1.to_primitive());
    assert_eq!(1, ConstEnum::A2.to_primitive());    
    assert_eq!(100, ConstEnum::B1.to_primitive());
    assert_eq!(101, ConstEnum::B2.to_primitive());
    assert_eq!(200, ConstEnum::C1.to_primitive());
    assert_eq!(201, ConstEnum::C2.to_primitive());
    assert_eq!(300, ConstEnum::D1.to_primitive());
    assert_eq!(301, ConstEnum::D2.to_primitive());
}