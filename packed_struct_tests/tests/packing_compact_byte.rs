use packed_struct::prelude::*;

#[test]
#[cfg(test)]
fn test_packed_compact_byte() {
    
    #[derive(PackedStruct, PartialEq, Debug, Eq)]
    #[packed_struct(size_bytes="1", bit_numbering="msb0")]
    pub struct RegA {
        #[packed_field(bits="0:2")]
        field_a: Integer<u8, packed_bits::Bits::<3>>,
        #[packed_field(bits="3:4")]
        field_b: Integer<u8, packed_bits::Bits::<2>>,
        #[packed_field(bits="5:7")]
        field_c: Integer<u8, packed_bits::Bits::<3>>
    }
    
    let reg = RegA {
        field_a: 0b101.into(),
        field_b: 0b11.into(),
        field_c: 0b010.into()
    };

    let packed = reg.pack().unwrap();
    assert_eq!(&packed, &[0b10111010]);

    let unpacked = RegA::unpack(&packed).unwrap();
    assert_eq!(&unpacked, &reg);
}

