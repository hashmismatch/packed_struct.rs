use packed_struct::prelude::*;

#[derive(PackedStruct, Default, Copy, Clone, PartialEq, Eq)]
#[packed_struct(bit_numbering="msb0")]
pub struct StructOne {
    #[packed_field(bits="0:3")]
    pub _reserved1: ReservedZero<packed_bits::Bits::<4>>,
    #[packed_field(bits="4")]
    pub bool1: bool,
    #[packed_field(bits="5:7")]
    pub _reserved2: ReservedOne<packed_bits::Bits::<3>>
}

#[test]
#[cfg(test)]
fn test_packed_reserved_fields() {
    let s = StructOne::default();
    let packed = s.pack().unwrap();
    assert_eq!([0b0000_0111], packed);

    let unpacked = StructOne::unpack(&[0b1111_1000]).unwrap();
    assert!(unpacked.bool1);
}
