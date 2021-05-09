use crate::PackingError;

/// +1 shift to the right by one bit
/// -1 shift to the left by one bit
fn shift_by_bits<const N: usize>(src: &[u8; N], shift_by_bits: isize) -> Result<[u8; N], PackingError> {
    use bitvec::prelude::*;

    if shift_by_bits == 0 {
        return Ok(src.clone());
    }
    if shift_by_bits >= (src.len() * 8) as isize {
        return Err(PackingError::BitsError);
    }

    let mut data = src.clone();
    let bits = BitSlice::<Msb0, _>::from_slice_mut(&mut data).map_err(|e| PackingError::BitsError)?;
    if shift_by_bits > 0 {
        bits.shift_right(shift_by_bits as usize);
    } else {
        bits.shift_left((-shift_by_bits) as usize);
    }

    Ok(data)
}

#[test]
fn test_bit_shift() {
    let a = [0b010];
    assert_eq!([0b001], shift_by_bits(&a, 1).unwrap());
    assert_eq!([0b100], shift_by_bits(&a, -1).unwrap());

    assert_eq!([0], shift_by_bits(&a, 2).unwrap());

    let b = [0b1, 0];
    assert_eq!([0, 0b1000_0000], shift_by_bits(&b, 1).unwrap());

    let c = [0, 0, 1, 2, 3, 0, 0];
    assert_eq!([0, 0, 0, 1, 2, 3, 0], shift_by_bits(&c, 8).unwrap());
    assert_eq!([0, 1, 2, 3, 0, 0, 0], shift_by_bits(&c, -8).unwrap());

    let d = [0b1000_0000, 0, 0, 0];
    assert_eq!([0, 0, 0, 0], shift_by_bits(&d, -1).unwrap());
    assert_eq!([0, 0b1000_0000, 0, 0], shift_by_bits(&d, 8).unwrap());
    assert_eq!([0, 0b0100_0000, 0, 0], shift_by_bits(&d, 9).unwrap());
    assert_eq!([0, 0, 0, 1], shift_by_bits(&d, 31).unwrap());

    let e = [0, 0, 0, 1];
    assert_eq!([0, 0, 0, 0], shift_by_bits(&e, 1).unwrap());
    assert_eq!([1, 0, 0, 0], shift_by_bits(&e, -24).unwrap());
    assert_eq!([0b1000_0000, 0, 0, 0], shift_by_bits(&e, -31).unwrap());
}