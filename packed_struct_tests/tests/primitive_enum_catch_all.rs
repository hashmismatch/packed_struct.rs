use packed_struct::prelude::*;

#[derive(PrimitiveEnum_u8, PartialEq, Debug, Clone, Copy)]
pub enum Field {
    A = 1,
    B = 2,
    C = 3
}

#[derive(PackedStruct, Debug, Clone, Copy)]
#[packed_struct(bit_numbering="msb0")]
pub struct Register {
    #[packed_field(bits="0..4", ty="enum")]
    pub field: EnumCatchAll<Field>
}

#[test]
fn prim_catch_all() {
    assert_eq!(2, Field::B as u8);

    let r = Register {
        field: Field::B.into()
    };
    let packed = r.pack().unwrap();
    assert_eq!([0b0010_0000], packed);
    assert_eq!(2, r.field.to_primitive());

    let packed_unknown_value = [0b0100_0000];
    let unpacked_unknown_value = Register::unpack(&packed_unknown_value).unwrap();
    assert_eq!(0b0100, unpacked_unknown_value.field.to_primitive());
    let repacked = unpacked_unknown_value.pack().unwrap();
    assert_eq!([0b0100_0000], repacked);

    println!("unknown: {:#?}", unpacked_unknown_value);
}

