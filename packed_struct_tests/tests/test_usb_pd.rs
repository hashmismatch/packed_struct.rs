use packed_struct::prelude::*;

#[test]
fn test_serialization_codegen() {



    #[derive(PackedStruct, Debug, Default, Copy, Clone)]
    #[packed_struct(size_bytes="4", bit_numbering="lsb0", endian="msb")]
    pub struct PowerDataObjectFixed {
        #[packed_field(bits="31:30")]
        pub supply: Integer<u8, packed_bits::Bits::<2>>,
        #[packed_field(bits="29")]
        pub dual_role_power: bool,
        #[packed_field(bits="28")]
        pub usb_suspend_supported: bool,
        #[packed_field(bits="27")]
        pub unconstrained_power: bool,
        #[packed_field(bits="26")]
        pub usb_communications_capable: bool,
        #[packed_field(bits="25")]
        pub dual_role_data: bool,
        #[packed_field(bits="21:20")]
        pub peak_current: Integer<u8, packed_bits::Bits::<2>>,
        #[packed_field(bits="19:10")]
        pub voltage: Integer<u16, packed_bits::Bits::<10>>,
        #[packed_field(bits="9:0")]
        pub maximum_current: Integer<u16, packed_bits::Bits::<10>>
    }


    let mut p: PowerDataObjectFixed = Default::default();
    p.dual_role_data = true;
    p.voltage = 0b10_11111111.into();
    p.maximum_current = 0b10_10101010.into();
        
    let packed = p.pack().unwrap();
    assert_eq!([0b00000010, 0b0000_1011, 0b111111_10, 0b10101010], packed);

    let unpacked = PowerDataObjectFixed::unpack(&packed).unwrap();
    assert_eq!(unpacked.dual_role_data, true);
    assert_eq!(*unpacked.voltage, *p.voltage);
    assert_eq!(*unpacked.maximum_current, *p.maximum_current);

}
