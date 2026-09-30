use packed_struct::prelude::*;

// https://github.com/hashmismatch/packed_struct.rs/issues/110
// Array fields are packed with a loop, so this shouldn't take long to compile.
#[derive(PackedStruct, Debug, PartialEq)]
#[packed_struct(endian = "msb")]
pub struct LargeArray {
    x: u16,
    y: u16,
    d: [u16; 1000],
}

#[test]
fn test_large_array() {
    let mut s = LargeArray { x: 0x1234, y: 0x5678, d: [0; 1000] };
    for (i, d) in s.d.iter_mut().enumerate() {
        *d = (i as u16).wrapping_mul(7919);
    }

    let packed = s.pack().unwrap();
    assert_eq!(packed.len(), 4 + 1000 * 2);
    assert_eq!(&packed[..4], &[0x12, 0x34, 0x56, 0x78]);
    for i in [0, 1, 2, 499, 999] {
        assert_eq!(&packed[4 + i * 2..4 + i * 2 + 2], &s.d[i].to_be_bytes());
    }

    let unpacked = LargeArray::unpack(&packed).unwrap();
    assert!(unpacked == s);
}

#[derive(PackedStruct, Debug, PartialEq)]
#[packed_struct(bit_numbering="msb0")]
pub struct Int3Array {
    #[packed_field(bits="0:4")]
    _pad: ReservedZeroes<packed_bits::Bits::<5>>,
    #[packed_field(bits="5:37", element_size_bits="3")]
    a: [Integer<u8, packed_bits::Bits::<3>>; 11],
}

#[derive(PackedStruct, Debug, PartialEq)]
#[packed_struct(bit_numbering="msb0")]
pub struct Int3Scalars {
    #[packed_field(bits="5:7")]
    a0: Integer<u8, packed_bits::Bits::<3>>,
    #[packed_field(bits="8:10")]
    a1: Integer<u8, packed_bits::Bits::<3>>,
    #[packed_field(bits="11:13")]
    a2: Integer<u8, packed_bits::Bits::<3>>,
    #[packed_field(bits="14:16")]
    a3: Integer<u8, packed_bits::Bits::<3>>,
    #[packed_field(bits="17:19")]
    a4: Integer<u8, packed_bits::Bits::<3>>,
    #[packed_field(bits="20:22")]
    a5: Integer<u8, packed_bits::Bits::<3>>,
    #[packed_field(bits="23:25")]
    a6: Integer<u8, packed_bits::Bits::<3>>,
    #[packed_field(bits="26:28")]
    a7: Integer<u8, packed_bits::Bits::<3>>,
    #[packed_field(bits="29:31")]
    a8: Integer<u8, packed_bits::Bits::<3>>,
    #[packed_field(bits="32:34")]
    a9: Integer<u8, packed_bits::Bits::<3>>,
    #[packed_field(bits="35:37")]
    a10: Integer<u8, packed_bits::Bits::<3>>,
}

#[test]
fn test_int3_array_matches_scalars() {
    let array = Int3Array {
        _pad: Default::default(),
        a: [3.into(), 0.into(), 5.into(), 2.into(), 7.into(), 4.into(), 1.into(), 6.into(), 3.into(), 0.into(), 5.into()],
    };
    let scalars = Int3Scalars {
        a0: 3.into(),
        a1: 0.into(),
        a2: 5.into(),
        a3: 2.into(),
        a4: 7.into(),
        a5: 4.into(),
        a6: 1.into(),
        a7: 6.into(),
        a8: 3.into(),
        a9: 0.into(),
        a10: 5.into(),
    };

    let packed = array.pack().unwrap();
    assert_eq!(packed, scalars.pack().unwrap());
    assert_eq!(Int3Array::unpack(&packed).unwrap(), array);

    // unpacking arbitrary bytes must also agree
    let mut bytes = packed;
    for seed in 0..32u32 {
        for (i, b) in bytes.iter_mut().enumerate() {
            *b = (seed.wrapping_mul(2654435761).wrapping_add(i as u32 * 40503) >> 8) as u8;
        }
        // keep the reserved padding zeroed
        bytes[0] &= 0xFF >> 5;
        let array = Int3Array::unpack(&bytes).unwrap();
        let scalars = Int3Scalars::unpack(&bytes).unwrap();
        assert_eq!(array.a[0], scalars.a0);
        assert_eq!(array.a[1], scalars.a1);
        assert_eq!(array.a[2], scalars.a2);
        assert_eq!(array.a[3], scalars.a3);
        assert_eq!(array.a[4], scalars.a4);
        assert_eq!(array.a[5], scalars.a5);
        assert_eq!(array.a[6], scalars.a6);
        assert_eq!(array.a[7], scalars.a7);
        assert_eq!(array.a[8], scalars.a8);
        assert_eq!(array.a[9], scalars.a9);
        assert_eq!(array.a[10], scalars.a10);
    }
}

#[derive(PackedStruct, Debug, PartialEq)]
#[packed_struct(bit_numbering="msb0")]
pub struct FlagsArray {
    #[packed_field(bits="0:4")]
    _pad: ReservedZeroes<packed_bits::Bits::<5>>,
    #[packed_field(bits="5:17", element_size_bits="1")]
    a: [bool; 13],
}

#[derive(PackedStruct, Debug, PartialEq)]
#[packed_struct(bit_numbering="msb0")]
pub struct FlagsScalars {
    #[packed_field(bits="5")]
    a0: bool,
    #[packed_field(bits="6")]
    a1: bool,
    #[packed_field(bits="7")]
    a2: bool,
    #[packed_field(bits="8")]
    a3: bool,
    #[packed_field(bits="9")]
    a4: bool,
    #[packed_field(bits="10")]
    a5: bool,
    #[packed_field(bits="11")]
    a6: bool,
    #[packed_field(bits="12")]
    a7: bool,
    #[packed_field(bits="13")]
    a8: bool,
    #[packed_field(bits="14")]
    a9: bool,
    #[packed_field(bits="15")]
    a10: bool,
    #[packed_field(bits="16")]
    a11: bool,
    #[packed_field(bits="17")]
    a12: bool,
}

#[test]
fn test_flags_array_matches_scalars() {
    let array = FlagsArray {
        _pad: Default::default(),
        a: [true, false, false, true, false, false, true, false, false, true, false, false, true],
    };
    let scalars = FlagsScalars {
        a0: true,
        a1: false,
        a2: false,
        a3: true,
        a4: false,
        a5: false,
        a6: true,
        a7: false,
        a8: false,
        a9: true,
        a10: false,
        a11: false,
        a12: true,
    };

    let packed = array.pack().unwrap();
    assert_eq!(packed, scalars.pack().unwrap());
    assert_eq!(FlagsArray::unpack(&packed).unwrap(), array);

    // unpacking arbitrary bytes must also agree
    let mut bytes = packed;
    for seed in 0..32u32 {
        for (i, b) in bytes.iter_mut().enumerate() {
            *b = (seed.wrapping_mul(2654435761).wrapping_add(i as u32 * 40503) >> 8) as u8;
        }
        // keep the reserved padding zeroed
        bytes[0] &= 0xFF >> 5;
        let array = FlagsArray::unpack(&bytes).unwrap();
        let scalars = FlagsScalars::unpack(&bytes).unwrap();
        assert_eq!(array.a[0], scalars.a0);
        assert_eq!(array.a[1], scalars.a1);
        assert_eq!(array.a[2], scalars.a2);
        assert_eq!(array.a[3], scalars.a3);
        assert_eq!(array.a[4], scalars.a4);
        assert_eq!(array.a[5], scalars.a5);
        assert_eq!(array.a[6], scalars.a6);
        assert_eq!(array.a[7], scalars.a7);
        assert_eq!(array.a[8], scalars.a8);
        assert_eq!(array.a[9], scalars.a9);
        assert_eq!(array.a[10], scalars.a10);
        assert_eq!(array.a[11], scalars.a11);
        assert_eq!(array.a[12], scalars.a12);
    }
}

#[derive(PackedStruct, Debug, PartialEq)]
#[packed_struct(bit_numbering="msb0")]
pub struct BytesArray {
    #[packed_field(bits="0:3")]
    _pad: ReservedZeroes<packed_bits::Bits::<4>>,
    #[packed_field(bits="4:43", element_size_bits="8")]
    a: [u8; 5],
}

#[derive(PackedStruct, Debug, PartialEq)]
#[packed_struct(bit_numbering="msb0")]
pub struct BytesScalars {
    #[packed_field(bits="4:11")]
    a0: u8,
    #[packed_field(bits="12:19")]
    a1: u8,
    #[packed_field(bits="20:27")]
    a2: u8,
    #[packed_field(bits="28:35")]
    a3: u8,
    #[packed_field(bits="36:43")]
    a4: u8,
}

#[test]
fn test_bytes_array_matches_scalars() {
    let array = BytesArray {
        _pad: Default::default(),
        a: [13, 110, 207, 48, 145],
    };
    let scalars = BytesScalars {
        a0: 13,
        a1: 110,
        a2: 207,
        a3: 48,
        a4: 145,
    };

    let packed = array.pack().unwrap();
    assert_eq!(packed, scalars.pack().unwrap());
    assert_eq!(BytesArray::unpack(&packed).unwrap(), array);

    // unpacking arbitrary bytes must also agree
    let mut bytes = packed;
    for seed in 0..32u32 {
        for (i, b) in bytes.iter_mut().enumerate() {
            *b = (seed.wrapping_mul(2654435761).wrapping_add(i as u32 * 40503) >> 8) as u8;
        }
        // keep the reserved padding zeroed
        bytes[0] &= 0xFF >> 4;
        let array = BytesArray::unpack(&bytes).unwrap();
        let scalars = BytesScalars::unpack(&bytes).unwrap();
        assert_eq!(array.a[0], scalars.a0);
        assert_eq!(array.a[1], scalars.a1);
        assert_eq!(array.a[2], scalars.a2);
        assert_eq!(array.a[3], scalars.a3);
        assert_eq!(array.a[4], scalars.a4);
    }
}

#[derive(PackedStruct, Debug, PartialEq)]
#[packed_struct(bit_numbering="msb0", endian="lsb")]
pub struct Lsb16Array {
    #[packed_field(bits="0:2")]
    _pad: ReservedZeroes<packed_bits::Bits::<3>>,
    #[packed_field(bits="3:50", element_size_bits="16")]
    a: [u16; 3],
}

#[derive(PackedStruct, Debug, PartialEq)]
#[packed_struct(bit_numbering="msb0", endian="lsb")]
pub struct Lsb16Scalars {
    #[packed_field(bits="3:18")]
    a0: u16,
    #[packed_field(bits="19:34")]
    a1: u16,
    #[packed_field(bits="35:50")]
    a2: u16,
}

#[test]
fn test_lsb16_array_matches_scalars() {
    let array = Lsb16Array {
        _pad: Default::default(),
        a: [4660, 45163, 20130],
    };
    let scalars = Lsb16Scalars {
        a0: 4660,
        a1: 45163,
        a2: 20130,
    };

    let packed = array.pack().unwrap();
    assert_eq!(packed, scalars.pack().unwrap());
    assert_eq!(Lsb16Array::unpack(&packed).unwrap(), array);

    // unpacking arbitrary bytes must also agree
    let mut bytes = packed;
    for seed in 0..32u32 {
        for (i, b) in bytes.iter_mut().enumerate() {
            *b = (seed.wrapping_mul(2654435761).wrapping_add(i as u32 * 40503) >> 8) as u8;
        }
        // keep the reserved padding zeroed
        bytes[0] &= 0xFF >> 3;
        let array = Lsb16Array::unpack(&bytes).unwrap();
        let scalars = Lsb16Scalars::unpack(&bytes).unwrap();
        assert_eq!(array.a[0], scalars.a0);
        assert_eq!(array.a[1], scalars.a1);
        assert_eq!(array.a[2], scalars.a2);
    }
}

#[derive(PackedStruct, Debug, PartialEq)]
#[packed_struct(bit_numbering="msb0", endian="msb")]
pub struct Int12Array {
    #[packed_field(bits="0:1")]
    _pad: ReservedZeroes<packed_bits::Bits::<2>>,
    #[packed_field(bits="2:85", element_size_bits="12")]
    a: [Integer<u16, packed_bits::Bits::<12>>; 7],
}

#[derive(PackedStruct, Debug, PartialEq)]
#[packed_struct(bit_numbering="msb0", endian="msb")]
pub struct Int12Scalars {
    #[packed_field(bits="2:13")]
    a0: Integer<u16, packed_bits::Bits::<12>>,
    #[packed_field(bits="14:25")]
    a1: Integer<u16, packed_bits::Bits::<12>>,
    #[packed_field(bits="26:37")]
    a2: Integer<u16, packed_bits::Bits::<12>>,
    #[packed_field(bits="38:49")]
    a3: Integer<u16, packed_bits::Bits::<12>>,
    #[packed_field(bits="50:61")]
    a4: Integer<u16, packed_bits::Bits::<12>>,
    #[packed_field(bits="62:73")]
    a5: Integer<u16, packed_bits::Bits::<12>>,
    #[packed_field(bits="74:85")]
    a6: Integer<u16, packed_bits::Bits::<12>>,
}

#[test]
fn test_int12_array_matches_scalars() {
    let array = Int12Array {
        _pad: Default::default(),
        a: [291.into(), 2068.into(), 3845.into(), 1526.into(), 3303.into(), 984.into(), 2761.into()],
    };
    let scalars = Int12Scalars {
        a0: 291.into(),
        a1: 2068.into(),
        a2: 3845.into(),
        a3: 1526.into(),
        a4: 3303.into(),
        a5: 984.into(),
        a6: 2761.into(),
    };

    let packed = array.pack().unwrap();
    assert_eq!(packed, scalars.pack().unwrap());
    assert_eq!(Int12Array::unpack(&packed).unwrap(), array);

    // unpacking arbitrary bytes must also agree
    let mut bytes = packed;
    for seed in 0..32u32 {
        for (i, b) in bytes.iter_mut().enumerate() {
            *b = (seed.wrapping_mul(2654435761).wrapping_add(i as u32 * 40503) >> 8) as u8;
        }
        // keep the reserved padding zeroed
        bytes[0] &= 0xFF >> 2;
        let array = Int12Array::unpack(&bytes).unwrap();
        let scalars = Int12Scalars::unpack(&bytes).unwrap();
        assert_eq!(array.a[0], scalars.a0);
        assert_eq!(array.a[1], scalars.a1);
        assert_eq!(array.a[2], scalars.a2);
        assert_eq!(array.a[3], scalars.a3);
        assert_eq!(array.a[4], scalars.a4);
        assert_eq!(array.a[5], scalars.a5);
        assert_eq!(array.a[6], scalars.a6);
    }
}

#[derive(PackedStruct, Debug, PartialEq)]
#[packed_struct(bit_numbering="msb0")]
pub struct Int6Array {
    #[packed_field(bits="0:53", element_size_bits="6")]
    a: [Integer<u8, packed_bits::Bits::<6>>; 9],
}

#[derive(PackedStruct, Debug, PartialEq)]
#[packed_struct(bit_numbering="msb0")]
pub struct Int6Scalars {
    #[packed_field(bits="0:5")]
    a0: Integer<u8, packed_bits::Bits::<6>>,
    #[packed_field(bits="6:11")]
    a1: Integer<u8, packed_bits::Bits::<6>>,
    #[packed_field(bits="12:17")]
    a2: Integer<u8, packed_bits::Bits::<6>>,
    #[packed_field(bits="18:23")]
    a3: Integer<u8, packed_bits::Bits::<6>>,
    #[packed_field(bits="24:29")]
    a4: Integer<u8, packed_bits::Bits::<6>>,
    #[packed_field(bits="30:35")]
    a5: Integer<u8, packed_bits::Bits::<6>>,
    #[packed_field(bits="36:41")]
    a6: Integer<u8, packed_bits::Bits::<6>>,
    #[packed_field(bits="42:47")]
    a7: Integer<u8, packed_bits::Bits::<6>>,
    #[packed_field(bits="48:53")]
    a8: Integer<u8, packed_bits::Bits::<6>>,
}

#[test]
fn test_int6_array_matches_scalars() {
    let array = Int6Array {
        a: [5.into(), 28.into(), 51.into(), 10.into(), 33.into(), 56.into(), 15.into(), 38.into(), 61.into()],
    };
    let scalars = Int6Scalars {
        a0: 5.into(),
        a1: 28.into(),
        a2: 51.into(),
        a3: 10.into(),
        a4: 33.into(),
        a5: 56.into(),
        a6: 15.into(),
        a7: 38.into(),
        a8: 61.into(),
    };

    let packed = array.pack().unwrap();
    assert_eq!(packed, scalars.pack().unwrap());
    assert_eq!(Int6Array::unpack(&packed).unwrap(), array);

    // unpacking arbitrary bytes must also agree
    let mut bytes = packed;
    for seed in 0..32u32 {
        for (i, b) in bytes.iter_mut().enumerate() {
            *b = (seed.wrapping_mul(2654435761).wrapping_add(i as u32 * 40503) >> 8) as u8;
        }
        let array = Int6Array::unpack(&bytes).unwrap();
        let scalars = Int6Scalars::unpack(&bytes).unwrap();
        assert_eq!(array.a[0], scalars.a0);
        assert_eq!(array.a[1], scalars.a1);
        assert_eq!(array.a[2], scalars.a2);
        assert_eq!(array.a[3], scalars.a3);
        assert_eq!(array.a[4], scalars.a4);
        assert_eq!(array.a[5], scalars.a5);
        assert_eq!(array.a[6], scalars.a6);
        assert_eq!(array.a[7], scalars.a7);
        assert_eq!(array.a[8], scalars.a8);
    }
}

#[derive(PrimitiveEnum_u8, Clone, Copy, Debug, PartialEq)]
pub enum Mode {
    Off = 0,
    Low = 1,
    High = 2
}

#[derive(PackedStruct, Debug, PartialEq)]
#[packed_struct(bit_numbering="msb0")]
pub struct Modes {
    #[packed_field(bits="0:7", element_size_bits="2", ty="enum")]
    modes: [Mode; 4]
}

#[test]
fn test_enum_array_invalid_element() {
    let modes = Modes { modes: [Mode::High, Mode::Off, Mode::Low, Mode::High] };
    let packed = modes.pack().unwrap();
    assert_eq!(packed, [0b10_00_01_10]);
    assert_eq!(Modes::unpack(&packed).unwrap(), modes);

    // 0b11 isn't a valid Mode
    assert_eq!(Modes::unpack(&[0b10_00_11_10]), Err(PackingError::InvalidValue));
}

#[test]
fn test_array_debug_fields() {
    let modes = Modes { modes: [Mode::High, Mode::Off, Mode::Low, Mode::High] };
    let s = format!("{}", modes);
    assert!(s.contains("modes[0] | bits   0:1   | 0b10 | \"High\""), "{}", s);
    assert!(s.contains("modes[3] | bits   6:7   | 0b10 | \"High\""), "{}", s);
}
