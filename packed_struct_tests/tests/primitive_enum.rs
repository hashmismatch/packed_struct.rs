extern crate packed_struct;
#[macro_use]
extern crate packed_struct_codegen;


#[derive(PrimitiveEnum_u8, PartialEq, Debug, Clone, Copy)]
pub enum SelfTestMode {
    NormalMode = 0,
    PositiveSignSelfTest,
    NegativeSignSelfTest = 2,
    NotAllowed = 3,
}

#[derive(PrimitiveEnum_u8, Copy, Clone)]
enum AddressCommand {
    PageProgram = 0x02,
    SectorErase = 0xD8,
    Read = 0x03
}

use packed_struct::*;

#[derive(PrimitiveEnum_u8, Copy, Clone)]
enum TestSmall {
    PageProgram = 0,
    SectorErase = 1
}

#[test]
fn prim() {
    let a = SelfTestMode::PositiveSignSelfTest;
    assert_eq!(1, a as u8);

    let a = SelfTestMode::from_primitive(1).unwrap();
    assert_eq!(SelfTestMode::PositiveSignSelfTest, a);
    assert_eq!("PositiveSignSelfTest", a.to_display_str());

    let a = SelfTestMode::from_primitive(2).unwrap();
    assert_eq!(SelfTestMode::NegativeSignSelfTest, a);    
    assert_eq!("NegativeSignSelfTest", a.to_display_str());

    let a = SelfTestMode::from_str("NegativeSignSelfTest").unwrap();
    assert_eq!(SelfTestMode::NegativeSignSelfTest, a);    

    let all = SelfTestMode::all_variants();
    assert_eq!(&[SelfTestMode::NormalMode, SelfTestMode::PositiveSignSelfTest, SelfTestMode::NegativeSignSelfTest, SelfTestMode::NotAllowed], all);
}