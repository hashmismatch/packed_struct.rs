//! Generated: integers of many widths in `byte_order="lsb"` structures, at bit offsets 0-7 and 13,
//! checked against a reference that writes them into a little-endian integer bit by bit.

use packed_struct::prelude::*;

mod oracle;
use oracle::*;

macro_rules! oracle_le_int {
    ($name:ident, $native:ident, $signed:expr, $bits:tt, $size_bytes:tt, $size:expr, $off:expr, $w:expr) => {
        #[test]
        #[allow(clippy::unnecessary_cast)]
        fn $name() {
            #[derive(PackedStruct, Debug, Default, Copy, Clone, PartialEq, Eq)]
            #[packed_struct(bit_numbering="lsb0", size_bytes=$size_bytes, byte_order="lsb")]
            struct T {
                #[packed_field(bits=$bits)]
                f: $native,
            }

            for raw in test_values($w) {
                let f: $native = if $signed { sign_extend(raw, $w) as $native } else { raw as $native };
                let t = T { f };

                let mut expected = [0u8; $size];
                set_integer_le(&mut expected, $off, $w, raw);
                let packed = t.pack().unwrap();
                assert_eq!(packed, expected, "pack, raw={:#x}", raw);
                assert_eq!(T::unpack(&packed).unwrap(), t, "unpack, raw={:#x}", raw);

                // every bit outside of the field is set
                let mut noisy = [0xFFu8; $size];
                set_integer_le(&mut noisy, $off, $w, raw);
                assert_eq!(T::unpack(&noisy).unwrap(), t, "unpack with surrounding ones, raw={:#x}", raw);
            }
        }
    };
}

macro_rules! oracle_le_int_msb {
    ($name:ident, $native:ident, $bits:tt, $size_bytes:tt, $size:expr, $off:expr, $w:expr) => {
        #[test]
        #[allow(clippy::unnecessary_cast)]
        fn $name() {
            #[derive(PackedStruct, Debug, Default, Copy, Clone, PartialEq, Eq)]
            #[packed_struct(bit_numbering="lsb0", size_bytes=$size_bytes, byte_order="lsb")]
            struct T {
                #[packed_field(bits=$bits, endian="msb")]
                f: $native,
            }

            for raw in test_values($w) {
                let t = T { f: raw as $native };

                // a big-endian field keeps its bytes in big-endian order
                let mut expected = [0u8; $size];
                for i in 0..($w / 8) {
                    expected[$off / 8 + i] = (raw >> ($w - 8 - i * 8)) as u8;
                }
                let packed = t.pack().unwrap();
                assert_eq!(packed, expected, "pack, raw={:#x}", raw);
                assert_eq!(T::unpack(&packed).unwrap(), t, "unpack, raw={:#x}", raw);
            }
        }
    };
}

oracle_le_int!(u8_b1_o0, u8, false, "0:0", "2", 2, 0, 1);
oracle_le_int!(u8_b1_o1, u8, false, "1:1", "2", 2, 1, 1);
oracle_le_int!(u8_b1_o2, u8, false, "2:2", "2", 2, 2, 1);
oracle_le_int!(u8_b1_o3, u8, false, "3:3", "2", 2, 3, 1);
oracle_le_int!(u8_b1_o4, u8, false, "4:4", "2", 2, 4, 1);
oracle_le_int!(u8_b1_o5, u8, false, "5:5", "2", 2, 5, 1);
oracle_le_int!(u8_b1_o6, u8, false, "6:6", "2", 2, 6, 1);
oracle_le_int!(u8_b1_o7, u8, false, "7:7", "2", 2, 7, 1);
oracle_le_int!(u8_b1_o13, u8, false, "13:13", "3", 3, 13, 1);
oracle_le_int!(i8_b1_o0, i8, true, "0:0", "2", 2, 0, 1);
oracle_le_int!(i8_b1_o1, i8, true, "1:1", "2", 2, 1, 1);
oracle_le_int!(i8_b1_o2, i8, true, "2:2", "2", 2, 2, 1);
oracle_le_int!(i8_b1_o3, i8, true, "3:3", "2", 2, 3, 1);
oracle_le_int!(i8_b1_o4, i8, true, "4:4", "2", 2, 4, 1);
oracle_le_int!(i8_b1_o5, i8, true, "5:5", "2", 2, 5, 1);
oracle_le_int!(i8_b1_o6, i8, true, "6:6", "2", 2, 6, 1);
oracle_le_int!(i8_b1_o7, i8, true, "7:7", "2", 2, 7, 1);
oracle_le_int!(i8_b1_o13, i8, true, "13:13", "3", 3, 13, 1);
oracle_le_int!(u8_b3_o0, u8, false, "2:0", "2", 2, 0, 3);
oracle_le_int!(u8_b3_o1, u8, false, "3:1", "2", 2, 1, 3);
oracle_le_int!(u8_b3_o2, u8, false, "4:2", "2", 2, 2, 3);
oracle_le_int!(u8_b3_o3, u8, false, "5:3", "2", 2, 3, 3);
oracle_le_int!(u8_b3_o4, u8, false, "6:4", "2", 2, 4, 3);
oracle_le_int!(u8_b3_o5, u8, false, "7:5", "2", 2, 5, 3);
oracle_le_int!(u8_b3_o6, u8, false, "8:6", "3", 3, 6, 3);
oracle_le_int!(u8_b3_o7, u8, false, "9:7", "3", 3, 7, 3);
oracle_le_int!(u8_b3_o13, u8, false, "15:13", "3", 3, 13, 3);
oracle_le_int!(i8_b3_o0, i8, true, "2:0", "2", 2, 0, 3);
oracle_le_int!(i8_b3_o1, i8, true, "3:1", "2", 2, 1, 3);
oracle_le_int!(i8_b3_o2, i8, true, "4:2", "2", 2, 2, 3);
oracle_le_int!(i8_b3_o3, i8, true, "5:3", "2", 2, 3, 3);
oracle_le_int!(i8_b3_o4, i8, true, "6:4", "2", 2, 4, 3);
oracle_le_int!(i8_b3_o5, i8, true, "7:5", "2", 2, 5, 3);
oracle_le_int!(i8_b3_o6, i8, true, "8:6", "3", 3, 6, 3);
oracle_le_int!(i8_b3_o7, i8, true, "9:7", "3", 3, 7, 3);
oracle_le_int!(i8_b3_o13, i8, true, "15:13", "3", 3, 13, 3);
oracle_le_int!(u8_b7_o0, u8, false, "6:0", "2", 2, 0, 7);
oracle_le_int!(u8_b7_o1, u8, false, "7:1", "2", 2, 1, 7);
oracle_le_int!(u8_b7_o2, u8, false, "8:2", "3", 3, 2, 7);
oracle_le_int!(u8_b7_o3, u8, false, "9:3", "3", 3, 3, 7);
oracle_le_int!(u8_b7_o4, u8, false, "10:4", "3", 3, 4, 7);
oracle_le_int!(u8_b7_o5, u8, false, "11:5", "3", 3, 5, 7);
oracle_le_int!(u8_b7_o6, u8, false, "12:6", "3", 3, 6, 7);
oracle_le_int!(u8_b7_o7, u8, false, "13:7", "3", 3, 7, 7);
oracle_le_int!(u8_b7_o13, u8, false, "19:13", "4", 4, 13, 7);
oracle_le_int!(i8_b7_o0, i8, true, "6:0", "2", 2, 0, 7);
oracle_le_int!(i8_b7_o1, i8, true, "7:1", "2", 2, 1, 7);
oracle_le_int!(i8_b7_o2, i8, true, "8:2", "3", 3, 2, 7);
oracle_le_int!(i8_b7_o3, i8, true, "9:3", "3", 3, 3, 7);
oracle_le_int!(i8_b7_o4, i8, true, "10:4", "3", 3, 4, 7);
oracle_le_int!(i8_b7_o5, i8, true, "11:5", "3", 3, 5, 7);
oracle_le_int!(i8_b7_o6, i8, true, "12:6", "3", 3, 6, 7);
oracle_le_int!(i8_b7_o7, i8, true, "13:7", "3", 3, 7, 7);
oracle_le_int!(i8_b7_o13, i8, true, "19:13", "4", 4, 13, 7);
oracle_le_int!(u8_b8_o0, u8, false, "7:0", "2", 2, 0, 8);
oracle_le_int!(u8_b8_o1, u8, false, "8:1", "3", 3, 1, 8);
oracle_le_int!(u8_b8_o2, u8, false, "9:2", "3", 3, 2, 8);
oracle_le_int!(u8_b8_o3, u8, false, "10:3", "3", 3, 3, 8);
oracle_le_int!(u8_b8_o4, u8, false, "11:4", "3", 3, 4, 8);
oracle_le_int!(u8_b8_o5, u8, false, "12:5", "3", 3, 5, 8);
oracle_le_int!(u8_b8_o6, u8, false, "13:6", "3", 3, 6, 8);
oracle_le_int!(u8_b8_o7, u8, false, "14:7", "3", 3, 7, 8);
oracle_le_int!(u8_b8_o13, u8, false, "20:13", "4", 4, 13, 8);
oracle_le_int!(i8_b8_o0, i8, true, "7:0", "2", 2, 0, 8);
oracle_le_int!(i8_b8_o1, i8, true, "8:1", "3", 3, 1, 8);
oracle_le_int!(i8_b8_o2, i8, true, "9:2", "3", 3, 2, 8);
oracle_le_int!(i8_b8_o3, i8, true, "10:3", "3", 3, 3, 8);
oracle_le_int!(i8_b8_o4, i8, true, "11:4", "3", 3, 4, 8);
oracle_le_int!(i8_b8_o5, i8, true, "12:5", "3", 3, 5, 8);
oracle_le_int!(i8_b8_o6, i8, true, "13:6", "3", 3, 6, 8);
oracle_le_int!(i8_b8_o7, i8, true, "14:7", "3", 3, 7, 8);
oracle_le_int!(i8_b8_o13, i8, true, "20:13", "4", 4, 13, 8);
oracle_le_int!(u16_b9_o0, u16, false, "8:0", "3", 3, 0, 9);
oracle_le_int!(u16_b9_o1, u16, false, "9:1", "3", 3, 1, 9);
oracle_le_int!(u16_b9_o2, u16, false, "10:2", "3", 3, 2, 9);
oracle_le_int!(u16_b9_o3, u16, false, "11:3", "3", 3, 3, 9);
oracle_le_int!(u16_b9_o4, u16, false, "12:4", "3", 3, 4, 9);
oracle_le_int!(u16_b9_o5, u16, false, "13:5", "3", 3, 5, 9);
oracle_le_int!(u16_b9_o6, u16, false, "14:6", "3", 3, 6, 9);
oracle_le_int!(u16_b9_o7, u16, false, "15:7", "3", 3, 7, 9);
oracle_le_int!(u16_b9_o13, u16, false, "21:13", "4", 4, 13, 9);
oracle_le_int!(i16_b9_o0, i16, true, "8:0", "3", 3, 0, 9);
oracle_le_int!(i16_b9_o1, i16, true, "9:1", "3", 3, 1, 9);
oracle_le_int!(i16_b9_o2, i16, true, "10:2", "3", 3, 2, 9);
oracle_le_int!(i16_b9_o3, i16, true, "11:3", "3", 3, 3, 9);
oracle_le_int!(i16_b9_o4, i16, true, "12:4", "3", 3, 4, 9);
oracle_le_int!(i16_b9_o5, i16, true, "13:5", "3", 3, 5, 9);
oracle_le_int!(i16_b9_o6, i16, true, "14:6", "3", 3, 6, 9);
oracle_le_int!(i16_b9_o7, i16, true, "15:7", "3", 3, 7, 9);
oracle_le_int!(i16_b9_o13, i16, true, "21:13", "4", 4, 13, 9);
oracle_le_int!(u16_b12_o0, u16, false, "11:0", "3", 3, 0, 12);
oracle_le_int!(u16_b12_o1, u16, false, "12:1", "3", 3, 1, 12);
oracle_le_int!(u16_b12_o2, u16, false, "13:2", "3", 3, 2, 12);
oracle_le_int!(u16_b12_o3, u16, false, "14:3", "3", 3, 3, 12);
oracle_le_int!(u16_b12_o4, u16, false, "15:4", "3", 3, 4, 12);
oracle_le_int!(u16_b12_o5, u16, false, "16:5", "4", 4, 5, 12);
oracle_le_int!(u16_b12_o6, u16, false, "17:6", "4", 4, 6, 12);
oracle_le_int!(u16_b12_o7, u16, false, "18:7", "4", 4, 7, 12);
oracle_le_int!(u16_b12_o13, u16, false, "24:13", "5", 5, 13, 12);
oracle_le_int!(i16_b12_o0, i16, true, "11:0", "3", 3, 0, 12);
oracle_le_int!(i16_b12_o1, i16, true, "12:1", "3", 3, 1, 12);
oracle_le_int!(i16_b12_o2, i16, true, "13:2", "3", 3, 2, 12);
oracle_le_int!(i16_b12_o3, i16, true, "14:3", "3", 3, 3, 12);
oracle_le_int!(i16_b12_o4, i16, true, "15:4", "3", 3, 4, 12);
oracle_le_int!(i16_b12_o5, i16, true, "16:5", "4", 4, 5, 12);
oracle_le_int!(i16_b12_o6, i16, true, "17:6", "4", 4, 6, 12);
oracle_le_int!(i16_b12_o7, i16, true, "18:7", "4", 4, 7, 12);
oracle_le_int!(i16_b12_o13, i16, true, "24:13", "5", 5, 13, 12);
oracle_le_int!(u16_b15_o0, u16, false, "14:0", "3", 3, 0, 15);
oracle_le_int!(u16_b15_o1, u16, false, "15:1", "3", 3, 1, 15);
oracle_le_int!(u16_b15_o2, u16, false, "16:2", "4", 4, 2, 15);
oracle_le_int!(u16_b15_o3, u16, false, "17:3", "4", 4, 3, 15);
oracle_le_int!(u16_b15_o4, u16, false, "18:4", "4", 4, 4, 15);
oracle_le_int!(u16_b15_o5, u16, false, "19:5", "4", 4, 5, 15);
oracle_le_int!(u16_b15_o6, u16, false, "20:6", "4", 4, 6, 15);
oracle_le_int!(u16_b15_o7, u16, false, "21:7", "4", 4, 7, 15);
oracle_le_int!(u16_b15_o13, u16, false, "27:13", "5", 5, 13, 15);
oracle_le_int!(i16_b15_o0, i16, true, "14:0", "3", 3, 0, 15);
oracle_le_int!(i16_b15_o1, i16, true, "15:1", "3", 3, 1, 15);
oracle_le_int!(i16_b15_o2, i16, true, "16:2", "4", 4, 2, 15);
oracle_le_int!(i16_b15_o3, i16, true, "17:3", "4", 4, 3, 15);
oracle_le_int!(i16_b15_o4, i16, true, "18:4", "4", 4, 4, 15);
oracle_le_int!(i16_b15_o5, i16, true, "19:5", "4", 4, 5, 15);
oracle_le_int!(i16_b15_o6, i16, true, "20:6", "4", 4, 6, 15);
oracle_le_int!(i16_b15_o7, i16, true, "21:7", "4", 4, 7, 15);
oracle_le_int!(i16_b15_o13, i16, true, "27:13", "5", 5, 13, 15);
oracle_le_int!(u16_b16_o0, u16, false, "15:0", "3", 3, 0, 16);
oracle_le_int!(u16_b16_o1, u16, false, "16:1", "4", 4, 1, 16);
oracle_le_int!(u16_b16_o2, u16, false, "17:2", "4", 4, 2, 16);
oracle_le_int!(u16_b16_o3, u16, false, "18:3", "4", 4, 3, 16);
oracle_le_int!(u16_b16_o4, u16, false, "19:4", "4", 4, 4, 16);
oracle_le_int!(u16_b16_o5, u16, false, "20:5", "4", 4, 5, 16);
oracle_le_int!(u16_b16_o6, u16, false, "21:6", "4", 4, 6, 16);
oracle_le_int!(u16_b16_o7, u16, false, "22:7", "4", 4, 7, 16);
oracle_le_int!(u16_b16_o13, u16, false, "28:13", "5", 5, 13, 16);
oracle_le_int!(i16_b16_o0, i16, true, "15:0", "3", 3, 0, 16);
oracle_le_int!(i16_b16_o1, i16, true, "16:1", "4", 4, 1, 16);
oracle_le_int!(i16_b16_o2, i16, true, "17:2", "4", 4, 2, 16);
oracle_le_int!(i16_b16_o3, i16, true, "18:3", "4", 4, 3, 16);
oracle_le_int!(i16_b16_o4, i16, true, "19:4", "4", 4, 4, 16);
oracle_le_int!(i16_b16_o5, i16, true, "20:5", "4", 4, 5, 16);
oracle_le_int!(i16_b16_o6, i16, true, "21:6", "4", 4, 6, 16);
oracle_le_int!(i16_b16_o7, i16, true, "22:7", "4", 4, 7, 16);
oracle_le_int!(i16_b16_o13, i16, true, "28:13", "5", 5, 13, 16);
oracle_le_int!(u32_b17_o0, u32, false, "16:0", "4", 4, 0, 17);
oracle_le_int!(u32_b17_o1, u32, false, "17:1", "4", 4, 1, 17);
oracle_le_int!(u32_b17_o2, u32, false, "18:2", "4", 4, 2, 17);
oracle_le_int!(u32_b17_o3, u32, false, "19:3", "4", 4, 3, 17);
oracle_le_int!(u32_b17_o4, u32, false, "20:4", "4", 4, 4, 17);
oracle_le_int!(u32_b17_o5, u32, false, "21:5", "4", 4, 5, 17);
oracle_le_int!(u32_b17_o6, u32, false, "22:6", "4", 4, 6, 17);
oracle_le_int!(u32_b17_o7, u32, false, "23:7", "4", 4, 7, 17);
oracle_le_int!(u32_b17_o13, u32, false, "29:13", "5", 5, 13, 17);
oracle_le_int!(i32_b17_o0, i32, true, "16:0", "4", 4, 0, 17);
oracle_le_int!(i32_b17_o1, i32, true, "17:1", "4", 4, 1, 17);
oracle_le_int!(i32_b17_o2, i32, true, "18:2", "4", 4, 2, 17);
oracle_le_int!(i32_b17_o3, i32, true, "19:3", "4", 4, 3, 17);
oracle_le_int!(i32_b17_o4, i32, true, "20:4", "4", 4, 4, 17);
oracle_le_int!(i32_b17_o5, i32, true, "21:5", "4", 4, 5, 17);
oracle_le_int!(i32_b17_o6, i32, true, "22:6", "4", 4, 6, 17);
oracle_le_int!(i32_b17_o7, i32, true, "23:7", "4", 4, 7, 17);
oracle_le_int!(i32_b17_o13, i32, true, "29:13", "5", 5, 13, 17);
oracle_le_int!(u32_b24_o0, u32, false, "23:0", "4", 4, 0, 24);
oracle_le_int!(u32_b24_o1, u32, false, "24:1", "5", 5, 1, 24);
oracle_le_int!(u32_b24_o2, u32, false, "25:2", "5", 5, 2, 24);
oracle_le_int!(u32_b24_o3, u32, false, "26:3", "5", 5, 3, 24);
oracle_le_int!(u32_b24_o4, u32, false, "27:4", "5", 5, 4, 24);
oracle_le_int!(u32_b24_o5, u32, false, "28:5", "5", 5, 5, 24);
oracle_le_int!(u32_b24_o6, u32, false, "29:6", "5", 5, 6, 24);
oracle_le_int!(u32_b24_o7, u32, false, "30:7", "5", 5, 7, 24);
oracle_le_int!(u32_b24_o13, u32, false, "36:13", "6", 6, 13, 24);
oracle_le_int!(i32_b24_o0, i32, true, "23:0", "4", 4, 0, 24);
oracle_le_int!(i32_b24_o1, i32, true, "24:1", "5", 5, 1, 24);
oracle_le_int!(i32_b24_o2, i32, true, "25:2", "5", 5, 2, 24);
oracle_le_int!(i32_b24_o3, i32, true, "26:3", "5", 5, 3, 24);
oracle_le_int!(i32_b24_o4, i32, true, "27:4", "5", 5, 4, 24);
oracle_le_int!(i32_b24_o5, i32, true, "28:5", "5", 5, 5, 24);
oracle_le_int!(i32_b24_o6, i32, true, "29:6", "5", 5, 6, 24);
oracle_le_int!(i32_b24_o7, i32, true, "30:7", "5", 5, 7, 24);
oracle_le_int!(i32_b24_o13, i32, true, "36:13", "6", 6, 13, 24);
oracle_le_int!(u32_b31_o0, u32, false, "30:0", "5", 5, 0, 31);
oracle_le_int!(u32_b31_o1, u32, false, "31:1", "5", 5, 1, 31);
oracle_le_int!(u32_b31_o2, u32, false, "32:2", "6", 6, 2, 31);
oracle_le_int!(u32_b31_o3, u32, false, "33:3", "6", 6, 3, 31);
oracle_le_int!(u32_b31_o4, u32, false, "34:4", "6", 6, 4, 31);
oracle_le_int!(u32_b31_o5, u32, false, "35:5", "6", 6, 5, 31);
oracle_le_int!(u32_b31_o6, u32, false, "36:6", "6", 6, 6, 31);
oracle_le_int!(u32_b31_o7, u32, false, "37:7", "6", 6, 7, 31);
oracle_le_int!(u32_b31_o13, u32, false, "43:13", "7", 7, 13, 31);
oracle_le_int!(i32_b31_o0, i32, true, "30:0", "5", 5, 0, 31);
oracle_le_int!(i32_b31_o1, i32, true, "31:1", "5", 5, 1, 31);
oracle_le_int!(i32_b31_o2, i32, true, "32:2", "6", 6, 2, 31);
oracle_le_int!(i32_b31_o3, i32, true, "33:3", "6", 6, 3, 31);
oracle_le_int!(i32_b31_o4, i32, true, "34:4", "6", 6, 4, 31);
oracle_le_int!(i32_b31_o5, i32, true, "35:5", "6", 6, 5, 31);
oracle_le_int!(i32_b31_o6, i32, true, "36:6", "6", 6, 6, 31);
oracle_le_int!(i32_b31_o7, i32, true, "37:7", "6", 6, 7, 31);
oracle_le_int!(i32_b31_o13, i32, true, "43:13", "7", 7, 13, 31);
oracle_le_int!(u32_b32_o0, u32, false, "31:0", "5", 5, 0, 32);
oracle_le_int!(u32_b32_o1, u32, false, "32:1", "6", 6, 1, 32);
oracle_le_int!(u32_b32_o2, u32, false, "33:2", "6", 6, 2, 32);
oracle_le_int!(u32_b32_o3, u32, false, "34:3", "6", 6, 3, 32);
oracle_le_int!(u32_b32_o4, u32, false, "35:4", "6", 6, 4, 32);
oracle_le_int!(u32_b32_o5, u32, false, "36:5", "6", 6, 5, 32);
oracle_le_int!(u32_b32_o6, u32, false, "37:6", "6", 6, 6, 32);
oracle_le_int!(u32_b32_o7, u32, false, "38:7", "6", 6, 7, 32);
oracle_le_int!(u32_b32_o13, u32, false, "44:13", "7", 7, 13, 32);
oracle_le_int!(i32_b32_o0, i32, true, "31:0", "5", 5, 0, 32);
oracle_le_int!(i32_b32_o1, i32, true, "32:1", "6", 6, 1, 32);
oracle_le_int!(i32_b32_o2, i32, true, "33:2", "6", 6, 2, 32);
oracle_le_int!(i32_b32_o3, i32, true, "34:3", "6", 6, 3, 32);
oracle_le_int!(i32_b32_o4, i32, true, "35:4", "6", 6, 4, 32);
oracle_le_int!(i32_b32_o5, i32, true, "36:5", "6", 6, 5, 32);
oracle_le_int!(i32_b32_o6, i32, true, "37:6", "6", 6, 6, 32);
oracle_le_int!(i32_b32_o7, i32, true, "38:7", "6", 6, 7, 32);
oracle_le_int!(i32_b32_o13, i32, true, "44:13", "7", 7, 13, 32);
oracle_le_int!(u64_b33_o0, u64, false, "32:0", "6", 6, 0, 33);
oracle_le_int!(u64_b33_o1, u64, false, "33:1", "6", 6, 1, 33);
oracle_le_int!(u64_b33_o2, u64, false, "34:2", "6", 6, 2, 33);
oracle_le_int!(u64_b33_o3, u64, false, "35:3", "6", 6, 3, 33);
oracle_le_int!(u64_b33_o4, u64, false, "36:4", "6", 6, 4, 33);
oracle_le_int!(u64_b33_o5, u64, false, "37:5", "6", 6, 5, 33);
oracle_le_int!(u64_b33_o6, u64, false, "38:6", "6", 6, 6, 33);
oracle_le_int!(u64_b33_o7, u64, false, "39:7", "6", 6, 7, 33);
oracle_le_int!(u64_b33_o13, u64, false, "45:13", "7", 7, 13, 33);
oracle_le_int!(i64_b33_o0, i64, true, "32:0", "6", 6, 0, 33);
oracle_le_int!(i64_b33_o1, i64, true, "33:1", "6", 6, 1, 33);
oracle_le_int!(i64_b33_o2, i64, true, "34:2", "6", 6, 2, 33);
oracle_le_int!(i64_b33_o3, i64, true, "35:3", "6", 6, 3, 33);
oracle_le_int!(i64_b33_o4, i64, true, "36:4", "6", 6, 4, 33);
oracle_le_int!(i64_b33_o5, i64, true, "37:5", "6", 6, 5, 33);
oracle_le_int!(i64_b33_o6, i64, true, "38:6", "6", 6, 6, 33);
oracle_le_int!(i64_b33_o7, i64, true, "39:7", "6", 6, 7, 33);
oracle_le_int!(i64_b33_o13, i64, true, "45:13", "7", 7, 13, 33);
oracle_le_int!(u64_b48_o0, u64, false, "47:0", "7", 7, 0, 48);
oracle_le_int!(u64_b48_o1, u64, false, "48:1", "8", 8, 1, 48);
oracle_le_int!(u64_b48_o2, u64, false, "49:2", "8", 8, 2, 48);
oracle_le_int!(u64_b48_o3, u64, false, "50:3", "8", 8, 3, 48);
oracle_le_int!(u64_b48_o4, u64, false, "51:4", "8", 8, 4, 48);
oracle_le_int!(u64_b48_o5, u64, false, "52:5", "8", 8, 5, 48);
oracle_le_int!(u64_b48_o6, u64, false, "53:6", "8", 8, 6, 48);
oracle_le_int!(u64_b48_o7, u64, false, "54:7", "8", 8, 7, 48);
oracle_le_int!(u64_b48_o13, u64, false, "60:13", "9", 9, 13, 48);
oracle_le_int!(i64_b48_o0, i64, true, "47:0", "7", 7, 0, 48);
oracle_le_int!(i64_b48_o1, i64, true, "48:1", "8", 8, 1, 48);
oracle_le_int!(i64_b48_o2, i64, true, "49:2", "8", 8, 2, 48);
oracle_le_int!(i64_b48_o3, i64, true, "50:3", "8", 8, 3, 48);
oracle_le_int!(i64_b48_o4, i64, true, "51:4", "8", 8, 4, 48);
oracle_le_int!(i64_b48_o5, i64, true, "52:5", "8", 8, 5, 48);
oracle_le_int!(i64_b48_o6, i64, true, "53:6", "8", 8, 6, 48);
oracle_le_int!(i64_b48_o7, i64, true, "54:7", "8", 8, 7, 48);
oracle_le_int!(i64_b48_o13, i64, true, "60:13", "9", 9, 13, 48);
oracle_le_int!(u64_b63_o0, u64, false, "62:0", "9", 9, 0, 63);
oracle_le_int!(u64_b63_o1, u64, false, "63:1", "9", 9, 1, 63);
oracle_le_int!(u64_b63_o2, u64, false, "64:2", "10", 10, 2, 63);
oracle_le_int!(u64_b63_o3, u64, false, "65:3", "10", 10, 3, 63);
oracle_le_int!(u64_b63_o4, u64, false, "66:4", "10", 10, 4, 63);
oracle_le_int!(u64_b63_o5, u64, false, "67:5", "10", 10, 5, 63);
oracle_le_int!(u64_b63_o6, u64, false, "68:6", "10", 10, 6, 63);
oracle_le_int!(u64_b63_o7, u64, false, "69:7", "10", 10, 7, 63);
oracle_le_int!(u64_b63_o13, u64, false, "75:13", "11", 11, 13, 63);
oracle_le_int!(i64_b63_o0, i64, true, "62:0", "9", 9, 0, 63);
oracle_le_int!(i64_b63_o1, i64, true, "63:1", "9", 9, 1, 63);
oracle_le_int!(i64_b63_o2, i64, true, "64:2", "10", 10, 2, 63);
oracle_le_int!(i64_b63_o3, i64, true, "65:3", "10", 10, 3, 63);
oracle_le_int!(i64_b63_o4, i64, true, "66:4", "10", 10, 4, 63);
oracle_le_int!(i64_b63_o5, i64, true, "67:5", "10", 10, 5, 63);
oracle_le_int!(i64_b63_o6, i64, true, "68:6", "10", 10, 6, 63);
oracle_le_int!(i64_b63_o7, i64, true, "69:7", "10", 10, 7, 63);
oracle_le_int!(i64_b63_o13, i64, true, "75:13", "11", 11, 13, 63);
oracle_le_int!(u64_b64_o0, u64, false, "63:0", "9", 9, 0, 64);
oracle_le_int!(u64_b64_o1, u64, false, "64:1", "10", 10, 1, 64);
oracle_le_int!(u64_b64_o2, u64, false, "65:2", "10", 10, 2, 64);
oracle_le_int!(u64_b64_o3, u64, false, "66:3", "10", 10, 3, 64);
oracle_le_int!(u64_b64_o4, u64, false, "67:4", "10", 10, 4, 64);
oracle_le_int!(u64_b64_o5, u64, false, "68:5", "10", 10, 5, 64);
oracle_le_int!(u64_b64_o6, u64, false, "69:6", "10", 10, 6, 64);
oracle_le_int!(u64_b64_o7, u64, false, "70:7", "10", 10, 7, 64);
oracle_le_int!(u64_b64_o13, u64, false, "76:13", "11", 11, 13, 64);
oracle_le_int!(i64_b64_o0, i64, true, "63:0", "9", 9, 0, 64);
oracle_le_int!(i64_b64_o1, i64, true, "64:1", "10", 10, 1, 64);
oracle_le_int!(i64_b64_o2, i64, true, "65:2", "10", 10, 2, 64);
oracle_le_int!(i64_b64_o3, i64, true, "66:3", "10", 10, 3, 64);
oracle_le_int!(i64_b64_o4, i64, true, "67:4", "10", 10, 4, 64);
oracle_le_int!(i64_b64_o5, i64, true, "68:5", "10", 10, 5, 64);
oracle_le_int!(i64_b64_o6, i64, true, "69:6", "10", 10, 6, 64);
oracle_le_int!(i64_b64_o7, i64, true, "70:7", "10", 10, 7, 64);
oracle_le_int!(i64_b64_o13, i64, true, "76:13", "11", 11, 13, 64);

oracle_le_int_msb!(u16_b16_msb_o0, u16, "15:0", "3", 3, 0, 16);
oracle_le_int_msb!(u16_b16_msb_o8, u16, "23:8", "4", 4, 8, 16);
oracle_le_int_msb!(u16_b16_msb_o16, u16, "31:16", "5", 5, 16, 16);
oracle_le_int_msb!(u32_b24_msb_o0, u32, "23:0", "4", 4, 0, 24);
oracle_le_int_msb!(u32_b24_msb_o8, u32, "31:8", "5", 5, 8, 24);
oracle_le_int_msb!(u32_b24_msb_o16, u32, "39:16", "6", 6, 16, 24);
oracle_le_int_msb!(u32_b32_msb_o0, u32, "31:0", "5", 5, 0, 32);
oracle_le_int_msb!(u32_b32_msb_o8, u32, "39:8", "6", 6, 8, 32);
oracle_le_int_msb!(u32_b32_msb_o16, u32, "47:16", "7", 7, 16, 32);
oracle_le_int_msb!(u64_b64_msb_o0, u64, "63:0", "9", 9, 0, 64);
oracle_le_int_msb!(u64_b64_msb_o8, u64, "71:8", "10", 10, 8, 64);
oracle_le_int_msb!(u64_b64_msb_o16, u64, "79:16", "11", 11, 16, 64);
