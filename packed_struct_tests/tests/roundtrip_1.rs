use packed_struct::prelude::*;

mod common;
use common::*;

#[test]
fn test_roundtrip_1() {

    #[derive(PackedStruct, Debug, PartialEq, Eq)]
    #[packed_struct(bit_numbering="msb0")]
    pub struct RoundtripAligned {
        #[packed_field(bits="0..")]
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
    
        #[packed_field(endian="msb")]
        u64_1: u64,

        f11: [u8; 1],
        f12: [u8; 2],
        f13: [u8; 3],
        f14: [u8; 4],

        f15: bool
    }


    let mut rnd = Rnd::new(1);

    for _ in 0..100 {
        let s = RoundtripAligned {
            f1: rnd.rnd_num(u8::max_value() as u64) as u8,
            f2: rnd.rnd_num(i8::max_value() as u64) as i8,

            f3: rnd.rnd_num(u16::max_value() as u64) as u16,
            f4: rnd.rnd_num(u16::max_value() as u64) as i16,

            f5: rnd.rnd_num(u16::max_value() as u64) as u16,
            f6: rnd.rnd_num(u16::max_value() as u64) as i16,

            f7: rnd.rnd_num(u32::max_value() as u64) as u32,
            f8: rnd.rnd_num(i32::max_value() as u64) as i32,

            f9: rnd.rnd_num(u32::max_value() as u64) as u32,
            f10: rnd.rnd_num(i32::max_value() as u64) as i32,

            u64_1: rnd.rnd_num(u64::max_value()),

            f11: [rnd.rnd_num(u8::max_value() as u64) as u8],
            f12: [rnd.rnd_num(u8::max_value() as u64) as u8, rnd.rnd_num(u8::max_value() as u64) as u8],
            f13: [rnd.rnd_num(u8::max_value() as u64) as u8, rnd.rnd_num(u8::max_value() as u64) as u8, rnd.rnd_num(u8::max_value() as u64) as u8],
            f14: [rnd.rnd_num(u8::max_value() as u64) as u8, rnd.rnd_num(u8::max_value() as u64) as u8, rnd.rnd_num(u8::max_value() as u64) as u8, rnd.rnd_num(u8::max_value() as u64) as u8],

            f15: (rnd.rnd() % 2) == 0
        };

        let packed = s.pack().unwrap();

        let unpacked = RoundtripAligned::unpack(&packed).unwrap();
        assert_eq!(&s, &unpacked);
    }  

}