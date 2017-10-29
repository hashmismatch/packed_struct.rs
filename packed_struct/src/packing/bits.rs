use prelude::v1::*;

pub struct NextBits;
pub trait BitsRange {
    fn get_bits_range(&self, packed_bit_width: usize, prev_range: &Option<Range<usize>>) -> Range<usize>; 
}

impl BitsRange for usize {
    fn get_bits_range(&self, packed_bit_width: usize, _prev_range: &Option<Range<usize>>) -> Range<usize> {
        *self..(*self + packed_bit_width as usize - 1)
    }
}

impl BitsRange for Range<usize> {
    fn get_bits_range(&self, _packed_bit_width: usize, _prev_range: &Option<Range<usize>>) -> Range<usize> {
        self.start..self.end
    }
}

impl BitsRange for NextBits {    
    fn get_bits_range(&self, packed_bit_width: usize, prev_range: &Option<Range<usize>>) -> Range<usize> {
        if let &Some(ref prev_range) = prev_range {
            (prev_range.end + 1)..((prev_range.end + 1) + (packed_bit_width as usize) - 1)
        } else {
            0..((packed_bit_width as usize) - 1)
        }
    }
}



pub fn ones_u8(n: u8) -> u8 {    
    match n {
        0 => 0b00000000,
        1 => 0b10000000,
        2 => 0b11000000,
        3 => 0b11100000,
        4 => 0b11110000,
        5 => 0b11111000,
        6 => 0b11111100,
        7 => 0b11111110,
        _ => 0b11111111
    }
}


#[test]
fn test_ones_u8() {
    let t = [
        (0, 0),
        (1, 0b10000000),
        (2, 0b11000000),
        (3, 0b11100000),
        (4, 0b11110000),
        (5, 0b11111000),
        (6, 0b11111100),
        (7, 0b11111110),
        (8, 0b11111111),
        (9, 0b11111111)
    ];

    for &(n, expected) in &t {
        assert_eq!(expected, ones_u8(n));
    }
}
