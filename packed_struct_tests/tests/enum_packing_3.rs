use packed_struct::prelude::*;

use packed_struct::prelude::*;

#[derive(PackedStruct, PartialEq, Debug, Copy, Clone)]
#[packed_struct(endian="msb")]
pub struct TestPack {
    a1: i32,
    a2: i32,
    a3: i32,

    #[packed_field(size_bytes="1", ty="enum")]
    mode: SelfTestMode 
}

#[derive(PackedStruct, PartialEq, Debug, Copy, Clone)]
pub struct TestPackMode {
    #[packed_field(size_bytes="1", ty="enum")]
    mode: SelfTestMode 
}

#[derive(PrimitiveEnum_u8, PartialEq, Debug, Clone, Copy)]
pub enum SelfTestMode {
    NormalMode = 0,
    PositiveSignSelfTest = 1,
    NegativeSignSelfTest = 2,
    DebugMode = 3,
}

#[test]
fn enum_packing_3() {
    let a = SelfTestMode::DebugMode;
    assert_eq!(3, a.to_primitive());
    
    let test = TestPack {
        a1: 100,
        a2: -100,
        a3: 131414,
        mode: SelfTestMode::DebugMode,
    };

    let packed = test.pack().unwrap();
    let unpacked = TestPack::unpack(&packed).unwrap();
    assert_eq!(&test, &unpacked);


    let m = TestPackMode {
        mode: a
    };
    let p = m.pack().unwrap();
    assert_eq!(&[3], &p);
}
