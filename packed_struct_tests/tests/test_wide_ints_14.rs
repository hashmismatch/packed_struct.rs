extern crate packed_struct;
#[macro_use]
extern crate packed_struct_codegen;

use packed_struct::prelude::*;

macro_rules! test_int_14 {
    ($f: ident, $fi: tt) => {
        #[test]
        fn $f() {

            #[derive(PackedStruct, Debug, Default, Copy, Clone, PartialEq)]
            #[packed_struct(bit_numbering="msb0", endian="msb")]
            pub struct Test {
                #[packed_field(bits= $fi )]
                pub int1: Integer<u16, packed_bits::Bits14>
            }

            let roundtrip = |x: u16| {
                let mut t: Test = Default::default();
                t.int1 = x.into();
                let packed = t.pack().unwrap();

                let unpacked = Test::unpack(&packed).unwrap();
                assert_eq!(unpacked, t);
            };

            roundtrip(0b00_101010_10101010);
            roundtrip(0b00_010101_01010101);
            roundtrip(0b00_111111_11111111);
            roundtrip(0b00_111111_01111111);
            roundtrip(0b00_111110_11111111);
            roundtrip(0b00_111110_01111111);
            roundtrip(0b00_100000_00000001);
        }
    };
}


test_int_14!(test_10_0, "0..");
test_int_14!(test_10_1, "1..");
test_int_14!(test_10_2, "2..");
test_int_14!(test_10_3, "3..");
test_int_14!(test_10_4, "4..");
test_int_14!(test_10_5, "5..");
test_int_14!(test_10_6, "6..");
test_int_14!(test_10_7, "7..");
test_int_14!(test_10_8, "8..");
test_int_14!(test_10_9, "9..");
test_int_14!(test_10_10, "10..");
test_int_14!(test_10_11, "11..");
test_int_14!(test_10_12, "12..");
test_int_14!(test_10_13, "13..");
test_int_14!(test_10_14, "14..");
test_int_14!(test_10_15, "15..");
test_int_14!(test_10_16, "16..");
test_int_14!(test_10_17, "17..");
test_int_14!(test_10_18, "18..");
test_int_14!(test_10_19, "19..");


test_int_14!(test_10_100, "100..");
