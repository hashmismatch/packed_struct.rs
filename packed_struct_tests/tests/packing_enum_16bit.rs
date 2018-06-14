extern crate packed_struct;
#[macro_use]
extern crate packed_struct_codegen;

use packed_struct::prelude::*;

#[derive(PrimitiveEnum_u16, PartialEq, Debug, Clone, Copy)]
pub enum LargeEnum {
    Value1 = 1,
    Value1024 = 1024,
    Value4096 = 4096
}

#[derive(PackedStruct, PartialEq, Debug, Clone, Copy)]
#[packed_struct(size_bytes="2", bit_numbering="msb0", endian="msb")]
pub struct StructWithBitsEnum {
    #[packed_field(bits="0..16", ty="enum")]
    large: LargeEnum
}

#[test]
fn prim() {
    let st = StructWithBitsEnum {
        large: LargeEnum::Value1024
    };

    let packed = st.pack();
    assert_eq!([0b0100_0000, 0b0000_0000], packed);

    let unpacked = StructWithBitsEnum::unpack(&packed).unwrap();
    assert_eq!(st, unpacked);
}