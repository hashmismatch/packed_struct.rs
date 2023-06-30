use packed_struct::prelude::*;

#[test]
#[cfg(test)]
fn test_tuple() {
    let p = [127, 127, 127];
    type T = (u8, u8, u8);
    let (a, b, c) = T::unpack_from_slice(&p).unwrap();
    assert_eq!(a, 127);
    assert_eq!(b, 127);
    assert_eq!(c, 127);
}

#[test]
#[cfg(test)]
fn test_tuple_undersized() {
    let p = [127];
    type T = (u8, u8, Vec<u8>);
    let res = T::unpack_from_slice(&p);
    assert!(res.is_err());    
}
