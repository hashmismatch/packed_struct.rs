//! Arrays of integers that aren't byte aligned, and large fields at unaligned offsets,
//! checked against a bit-by-bit reference implementation.

use packed_struct::prelude::*;

mod oracle;
use oracle::*;

/// Raw element values: all zeroes, all ones and a few patterns that differ per element.
fn element_values(len: usize, width: usize) -> Vec<Vec<u64>> {
    let m = mask(width);
    let pattern = |seed: u64| (0..len).map(|i| seed.rotate_left((i * 7) as u32) & m).collect();
    vec![
        vec![0; len],
        vec![m; len],
        pattern(0xA5A5_A5A5_A5A5_A5A5),
        pattern(0x0123_4567_89AB_CDEF),
        (0..len).map(|i| (i as u64 + 1) & m).collect(),
    ]
}

macro_rules! check_array {
    ($T: ident, $size: expr, $off: expr, $len: expr, $w: expr, $lsb: expr, $make: expr) => {{
        for raws in element_values($len, $w) {
            let t: $T = $make(&raws);

            let mut expected = [0u8; $size];
            for (i, raw) in raws.iter().enumerate() {
                set_integer(&mut expected, $off + i * $w, $w, *raw, $lsb);
            }
            let packed = t.pack().unwrap();
            assert_eq!(packed, expected, "pack, raws={:x?}", raws);
            assert_eq!($T::unpack(&packed).unwrap(), t, "unpack, raws={:x?}", raws);

            // every bit outside of the array is set
            let mut noisy = [0xFFu8; $size];
            for (i, raw) in raws.iter().enumerate() {
                set_integer(&mut noisy, $off + i * $w, $w, *raw, $lsb);
            }
            assert_eq!($T::unpack(&noisy).unwrap(), t, "unpack with surrounding ones, raws={:x?}", raws);
        }
    }};
}

#[test]
fn array_of_3_bit_integers() {
    #[derive(PackedStruct, Debug, PartialEq)]
    #[packed_struct(bit_numbering="msb0", size_bytes="3")]
    struct T {
        #[packed_field(bits="3..")]
        arr: [Integer<u8, packed_bits::Bits::<3>>; 5]
    }

    check_array!(T, 3, 3, 5, 3, false, |r: &[u64]| T { arr: core::array::from_fn(|i| (r[i] as u8).into()) });
}

#[test]
fn array_of_5_bit_u8() {
    #[derive(PackedStruct, Debug, PartialEq)]
    #[packed_struct(bit_numbering="msb0", size_bytes="8")]
    struct T {
        #[packed_field(bits="7..", element_size_bits="5")]
        arr: [u8; 11]
    }

    check_array!(T, 8, 7, 11, 5, false, |r: &[u64]| T { arr: core::array::from_fn(|i| r[i] as u8) });
}

#[test]
fn array_of_bools() {
    #[derive(PackedStruct, Debug, PartialEq)]
    #[packed_struct(bit_numbering="msb0", size_bytes="3")]
    struct T {
        #[packed_field(bits="2..")]
        arr: [bool; 11]
    }

    check_array!(T, 3, 2, 11, 1, false, |r: &[u64]| T { arr: core::array::from_fn(|i| r[i] == 1) });
}

#[test]
fn array_of_12_bit_u16() {
    #[derive(PackedStruct, Debug, PartialEq)]
    #[packed_struct(bit_numbering="msb0", size_bytes="6", endian="msb")]
    struct Msb {
        #[packed_field(bits="5..", element_size_bits="12")]
        arr: [u16; 3]
    }

    #[derive(PackedStruct, Debug, PartialEq)]
    #[packed_struct(bit_numbering="msb0", size_bytes="6", endian="lsb")]
    struct Lsb {
        #[packed_field(bits="5..", element_size_bits="12")]
        arr: [u16; 3]
    }

    check_array!(Msb, 6, 5, 3, 12, false, |r: &[u64]| Msb { arr: core::array::from_fn(|i| r[i] as u16) });
    check_array!(Lsb, 6, 5, 3, 12, true, |r: &[u64]| Lsb { arr: core::array::from_fn(|i| r[i] as u16) });
}

#[test]
fn array_of_12_bit_u32() {
    #[derive(PackedStruct, Debug, PartialEq)]
    #[packed_struct(bit_numbering="msb0", size_bytes="8", endian="msb")]
    struct Msb {
        #[packed_field(bits="1..", element_size_bits="12")]
        arr: [u32; 4]
    }

    #[derive(PackedStruct, Debug, PartialEq)]
    #[packed_struct(bit_numbering="msb0", size_bytes="8", endian="lsb")]
    struct Lsb {
        #[packed_field(bits="1..", element_size_bits="12")]
        arr: [u32; 4]
    }

    check_array!(Msb, 8, 1, 4, 12, false, |r: &[u64]| Msb { arr: core::array::from_fn(|i| r[i] as u32) });
    check_array!(Lsb, 8, 1, 4, 12, true, |r: &[u64]| Lsb { arr: core::array::from_fn(|i| r[i] as u32) });
}

#[test]
fn array_of_20_bit_i64() {
    #[derive(PackedStruct, Debug, PartialEq)]
    #[packed_struct(bit_numbering="msb0", size_bytes="19", endian="msb")]
    struct Msb {
        #[packed_field(bits="3..", element_size_bits="20")]
        arr: [i64; 7]
    }

    #[derive(PackedStruct, Debug, PartialEq)]
    #[packed_struct(bit_numbering="msb0", size_bytes="19", endian="lsb")]
    struct Lsb {
        #[packed_field(bits="3..", element_size_bits="20")]
        arr: [i64; 7]
    }

    check_array!(Msb, 19, 3, 7, 20, false, |r: &[u64]| Msb { arr: core::array::from_fn(|i| sign_extend(r[i], 20)) });
    check_array!(Lsb, 19, 3, 7, 20, true, |r: &[u64]| Lsb { arr: core::array::from_fn(|i| sign_extend(r[i], 20)) });
}

#[test]
fn array_of_52_bit_integers() {
    #[derive(PackedStruct, Debug, PartialEq)]
    #[packed_struct(bit_numbering="msb0", size_bytes="21", endian="lsb")]
    struct T {
        #[packed_field(bits="4..")]
        arr: [Integer<u64, packed_bits::Bits::<52>>; 3]
    }

    check_array!(T, 21, 4, 3, 52, true, |r: &[u64]| T { arr: core::array::from_fn(|i| r[i].into()) });
}

/// A field of 32 bytes or more at an unaligned offset.
#[test]
fn large_nested_struct_at_unaligned_offset() {
    #[derive(PackedStruct, Debug, PartialEq, Clone, Copy)]
    #[packed_struct(size_bytes="40")]
    struct Inner {
        data: [u8; 40]
    }

    #[derive(PackedStruct, Debug, PartialEq)]
    #[packed_struct(bit_numbering="msb0", size_bytes="42")]
    struct Outer {
        #[packed_field(bits="3..", size_bytes="40")]
        inner: Inner
    }

    let data: [u8; 40] = core::array::from_fn(|i| (i as u8).wrapping_mul(37) ^ 0xA5);
    let t = Outer { inner: Inner { data } };

    let mut expected = [0u8; 42];
    set_bytes(&mut expected, 3, &data);
    let packed = t.pack().unwrap();
    assert_eq!(packed, expected);
    assert_eq!(Outer::unpack(&packed).unwrap(), t);

    let mut noisy = [0xFFu8; 42];
    set_bytes(&mut noisy, 3, &data);
    assert_eq!(Outer::unpack(&noisy).unwrap(), t);
}

#[test]
fn large_reserved_ones_at_unaligned_offset() {
    #[derive(PackedStruct, Debug, PartialEq, Default)]
    #[packed_struct(bit_numbering="msb0", size_bytes="40")]
    struct T {
        #[packed_field(bits="3..")]
        reserved: ReservedOnes<packed_bits::Bits::<300>>
    }

    let mut expected = [0u8; 40];
    for bit in 3..303 {
        set_bit_msb0(&mut expected, bit, true);
    }
    assert_eq!(T::default().pack().unwrap(), expected);
}
