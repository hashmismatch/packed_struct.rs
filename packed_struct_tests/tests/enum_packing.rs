use packed_struct::prelude::*;

#[derive(PackedStruct, PartialEq, Debug, Copy, Clone)]
#[packed_struct(bit_numbering="msb0")]
pub struct TestPack {
    #[packed_field(bits = "2:3", ty="enum")]
    mode: SelfTestMode,
    #[packed_field(bits = "6")]
    enabled: bool
}

#[derive(PrimitiveEnum_u8, PartialEq, Debug, Clone, Copy)]
pub enum SelfTestMode {
    NormalMode = 0,
    PositiveSignSelfTest = 1,
    NegativeSignSelfTest = 2,
    DebugMode = 3,
}

#[test]
fn prim() {
    let a = SelfTestMode::DebugMode;
    assert_eq!(3, a.to_primitive());
    
    let test = TestPack {
        mode: SelfTestMode::DebugMode,
        enabled: true
    };

    let packed = test.pack().unwrap();
    assert_eq!([0b00110010], packed);

    let unpacked = TestPack::unpack(&packed).unwrap();
    assert_eq!(unpacked, test);
}
