use packed_struct::prelude::*;

#[derive(PackedStruct)]
#[packed_struct(endian = "msb", size_bytes = "7")]
pub struct TestStructBE {
    a: u8,
    b: u32,
    c: u16,
}

#[derive(PackedStruct)]
#[packed_struct(endian = "lsb", size_bytes = "7")]
pub struct TestStructLE {
    a: u8,
    b: u32,
    c: u16,
}

#[test]
#[cfg(test)]
fn test_struct_be_unpack() {
    let buf = &[0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77];
    let unpacked: TestStructBE = TestStructBE::unpack_from_slice(buf).unwrap();
    assert_eq!(unpacked.a, 0x11);
    assert_eq!(unpacked.b, 0x22334455);
    assert_eq!(unpacked.c, 0x6677);
}

#[test]
#[cfg(test)]
fn test_struct_le_unpack() {
    let buf = &[0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77];
    let unpacked: TestStructLE = TestStructLE::unpack_from_slice(buf).unwrap();
    assert_eq!(unpacked.a, 0x11);
    assert_eq!(unpacked.b, 0x55443322);
    assert_eq!(unpacked.c, 0x7766);
}