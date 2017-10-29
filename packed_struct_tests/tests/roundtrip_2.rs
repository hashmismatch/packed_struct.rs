extern crate packed_struct;
#[macro_use]
extern crate packed_struct_codegen;

use packed_struct::*;

mod common;
use common::*;

#[test]
fn test_roundtrip_2() {

    #[derive(PackedStruct, Debug, PartialEq)]
    #[packed_struct(bit_numbering="msb0")]
    pub struct RoundtripUnaligned {
        #[packed_field(bits="1..")]
        f1: u8,
        f2: i8,
        
        #[packed_field(endian="msb")]
        f3: u16,
        #[packed_field(endian="msb")]
        f4: i16,

        #[packed_field(endian="lsb")]
        f5: u16,
        #[packed_field(endian="lsb")]
        f6: i16,

        #[packed_field(endian="msb")]
        f7: u32,
        #[packed_field(endian="msb")]
        f8: i32,

        #[packed_field(endian="lsb")]
        f9: u32,
        #[packed_field(endian="lsb")]
        f10: i32,

        f11: [u8; 1],
        f12: [u8; 2],
        f13: [u8; 3],
        f14: [u8; 4],

        f15: bool
    }


    let mut rnd = Rnd::new(1);

    for i in 0..100 {
        let s = RoundtripUnaligned {
            f1: rnd.next_num(u8::max_value() as u64) as u8,
            f2: rnd.next_num(i8::max_value() as u64) as i8,

            f3: rnd.next_num(u16::max_value() as u64) as u16,
            f4: rnd.next_num(u16::max_value() as u64) as i16,

            f5: rnd.next_num(u16::max_value() as u64) as u16,
            f6: rnd.next_num(u16::max_value() as u64) as i16,

            f7: rnd.next_num(u32::max_value() as u64) as u32,
            f8: rnd.next_num(i32::max_value() as u64) as i32,

            f9: rnd.next_num(u32::max_value() as u64) as u32,
            f10: rnd.next_num(i32::max_value() as u64) as i32,

            f11: [rnd.next_num(u8::max_value() as u64) as u8],
            f12: [rnd.next_num(u8::max_value() as u64) as u8, rnd.next_num(u8::max_value() as u64) as u8],
            f13: [rnd.next_num(u8::max_value() as u64) as u8, rnd.next_num(u8::max_value() as u64) as u8, rnd.next_num(u8::max_value() as u64) as u8],
            f14: [rnd.next_num(u8::max_value() as u64) as u8, rnd.next_num(u8::max_value() as u64) as u8, rnd.next_num(u8::max_value() as u64) as u8, rnd.next_num(u8::max_value() as u64) as u8],

            f15: if (rnd.next() % 2) == 0 { true } else { false }
        };

        let packed = s.pack();

        let unpacked = RoundtripUnaligned::unpack(&packed).unwrap();
        assert_eq!(&s, &unpacked);
    }  

}