extern crate packed_struct;
#[macro_use]
extern crate packed_struct_codegen;

#[derive(PackedStruct, Debug, Copy, Clone, Default)]
#[packed_struct(endian="msb")]
pub struct Identification {
    pub manufacturer: u16,
    pub memory_type: u16,
    pub memory_capacity: u16,
    pub customized_factory_data_length: u8,
    pub customized_factory_data: [u8; 16]
}

#[test]
fn test_debug_id() {
    let id: Identification = Default::default();

    println!("{}", id);
}

#[derive(PackedStruct, Debug, Copy, Clone, Default)]
pub struct JedecId {
    pub manufacturer: u8,
    pub memory_type: u8,
    pub memory_capacity: u8
}

#[test]
fn test_debug_jedec_id() {
    use packed_struct::PackedStruct;

    let id = JedecId::unpack(&[50, 100, 150]).unwrap();

    println!("{}", id);
}
