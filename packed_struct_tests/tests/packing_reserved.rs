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


// Bits<512> is the widest type generated with the `byte_types_64` feature
#[derive(PackedStruct, Default, Copy, Clone, PartialEq, Eq)]
#[packed_struct(bit_numbering="msb0")]
pub struct StructFullWidthReserved {
    #[packed_field(bytes="0..64")]
    pub _reserved: ReservedOne<packed_bits::Bits::<512>>,
    #[packed_field(bytes="64")]
    pub value: u8
}

#[test]
fn test_packed_reserved_full_width() {
    let s = StructFullWidthReserved { value: 0x5A, ..Default::default() };
    let packed = s.pack().unwrap();
    assert_eq!([0xFF; 64], packed[..64]);
    assert_eq!(0x5A, packed[64]);

    let unpacked = StructFullWidthReserved::unpack(&packed).unwrap();
    assert_eq!(0x5A, unpacked.value);
}
