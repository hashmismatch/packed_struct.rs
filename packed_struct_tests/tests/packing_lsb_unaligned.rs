use packed_struct::prelude::*;

// LSB integers with non-byte-aligned widths go through the bit shifting path
// in LsbInteger's pack/unpack.

#[derive(PackedStruct, Debug, Copy, Clone, PartialEq)]
#[packed_struct(bit_numbering="msb0", endian="lsb")]
pub struct LsbUnaligned {
    #[packed_field(bits="0:9")]
    a: Integer<u16, packed_bits::Bits::<10>>,
    #[packed_field(bits="10:21")]
    b: Integer<u16, packed_bits::Bits::<12>>,
    #[packed_field(bits="22:41")]
    c: Integer<u32, packed_bits::Bits::<20>>,
    #[packed_field(bits="42:44")]
    d: Integer<u8, packed_bits::Bits::<3>>,
    #[packed_field(bits="45:71")]
    e: Integer<u32, packed_bits::Bits::<27>>
}

#[test]
fn test_lsb_unaligned() {
    let s = LsbUnaligned {
        a: 0b10_1100_0101.into(),
        b: 0xA5C.into(),
        c: 0xB_3C5A.into(),
        d: 0b101.into(),
        e: 0x5A3_C1E7.into()
    };

    let packed = s.pack().unwrap();
    assert_eq!(packed, [197, 151, 41, 104, 242, 239, 62, 13, 29]);

    let unpacked = LsbUnaligned::unpack(&packed).unwrap();
    assert_eq!(s, unpacked);
}

#[test]
fn test_lsb_unaligned_roundtrip() {
    let mut x: u32 = 0x1234_5678;
    for _ in 0..1000 {
        let mut next = || { x = x.wrapping_mul(1_103_515_245).wrapping_add(12345); x };
        let s = LsbUnaligned {
            a: ((next() >> 8) as u16 & 0x3FF).into(),
            b: ((next() >> 8) as u16 & 0xFFF).into(),
            c: ((next() >> 4) & 0xF_FFFF).into(),
            d: ((next() >> 8) as u8 & 0x7).into(),
            e: ((next() >> 2) & 0x7FF_FFFF).into()
        };
        let packed = s.pack().unwrap();
        let unpacked = LsbUnaligned::unpack(&packed).unwrap();
        assert_eq!(s, unpacked);
    }
}
