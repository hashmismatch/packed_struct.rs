use packed_struct::prelude::*;

#[derive(PackedStruct, PartialEq, Eq, Debug)]
#[packed_struct(bit_numbering="msb0")]
pub struct SmallInts {    
    #[packed_field(bits="0:2")]
    pub val1: Integer<u8, packed_bits::Bits::<3>>,
    #[packed_field(bits="3:4")]
    pub val2: Integer<u8, packed_bits::Bits::<2>>,
    pub val3: bool,
    #[packed_field(bits="6")]
    pub val4: bool,
    #[packed_field(bits="7..")]
    pub val5: bool
}

#[test]
fn test_packing_bit_positions() {
    let a = SmallInts {
        val1: 7.into(),
        val2: 3.into(),
        val3: true,
        val4: true,
        val5: true
    };

    let packed = a.pack().unwrap();
    assert_eq!([255], packed);

    let unpacked = SmallInts::unpack(&packed).unwrap();
    assert_eq!(a, unpacked);
}


#[derive(PackedStruct, PartialEq, Eq, Debug)]
#[packed_struct(size_bytes="1", bit_numbering="lsb0")]
pub struct SmallIntsLsb {    
    #[packed_field(bits="2:0")]
    pub val1: Integer<u8, packed_bits::Bits::<3>>,
    #[packed_field(bits="6")]
    pub val2: bool
}

#[test]
fn test_packing_bit_positions_lsb() {
    let a = SmallIntsLsb {
        val1: 0b111.into(),
        val2: true
    };

    let packed = a.pack().unwrap();
    assert_eq!(&[0b01000111], &packed);

    let unpacked = SmallIntsLsb::unpack(&packed).unwrap();
    assert_eq!(a, unpacked);
}


#[derive(PackedStruct, PartialEq, Debug)]
#[packed_struct(size_bytes="4", bit_numbering="lsb0", endian="lsb")]
pub struct BigIntsLsb {
    #[packed_field(bits="26:24")]
    pub val1: Integer<u8, packed_bits::Bits3>,
    #[packed_field(bits="18")]
    pub val2: bool,
    #[packed_field(bits="15:0")]
    pub val3: Integer<u16, packed_bits::Bits16>,
}

#[test]
/// This test should verify the packing/unpacking a hypothetical 32-bit register that contains 0xbeef0407
fn test_packing_bit_positions_bigints_lsb() {
    let a = BigIntsLsb {
        val1: 7.into(),
        val2: true,
        val3: 0xbeef.into(),
    };

    let packed = a.pack();
    assert_eq!(&[0x07u8, 0x04, 0xef, 0xbe], &packed, "mismatch received: {}", a);

    let unpacked = BigIntsLsb::unpack(&packed).unwrap();
    assert_eq!(a, unpacked);
}


#[test]
fn test_packing_byte_position() {
    #[derive(Copy, Clone, Debug, PartialEq, Eq, PackedStruct)]
    #[packed_struct(bit_numbering="msb0", endian="msb")]
    pub struct BufferChecksum {
        #[packed_field(bytes="0")]
        pub version: u8,
        #[packed_field(bytes="1:4")]
        pub size: u32,
        #[packed_field(bytes="5..")]
        pub checksum: u64
    }

    let b = BufferChecksum {
        version: 101,
        size: 52748273,
        checksum: 869034217895
    };
    
    let packed = b.pack().unwrap();
    assert_eq!(packed.len(), 13);

    let unpacked = BufferChecksum::unpack(&packed).unwrap();

    assert_eq!(b, unpacked);
}
