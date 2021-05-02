extern crate packed_struct;
extern crate serde;


use packed_struct::prelude::*;

#[test]
fn test_serialization_traits() {

    check_serde_support::<Integer<u8, packed_bits::Bits::<8>>>();
    check_serde_support::<PackingError>();
    check_serde_support::<BitOne>();
    check_serde_support::<BitZero>();
    check_serde_support::<ReservedBits<u8, packed_bits::Bits::<8>>>();
}

fn check_serde_support<'a, T>() 
    where 
        T: serde::Serialize,
        T: serde::Deserialize<'a>
{
}