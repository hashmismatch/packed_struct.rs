Bit-level packing and unpacking for Rust
===========================================

[![Build Status](https://travis-ci.org/hashmismatch/packed_struct.rs.svg?branch=master)](https://travis-ci.org/hashmismatch/packed_struct.rs)

# Sample usage

## Cargo.toml

```toml
[dependencies]
packed_struct = { git = "https://github.com/hashmismatch/packed_struct.rs" }
packed_struct_codegen = { git = "https://github.com/hashmismatch/packed_struct.rs" }
```
## lib.rs

```rust
extern crate packed_struct;
#[macro_use]
extern crate packed_struct_codegen;
```

## example.rs

```rust
#[derive(PackedStruct, PartialEq, Debug, Copy, Clone)]
#[packed_struct(bit_numbering="msb0")]
pub struct TestPack {
    #[packed_field(bits = "2..3", ty="enum")]
    mode: SelfTestMode,
    #[packed_field(bits = "6")]
    enabled: bool
}

#[derive(PrimitiveEnum_u8, PartialEq, Debug, Clone, Copy)]
pub enum SelfTestMode {
    NormalMode = 0,
    PositiveSignSelfTest = 1,
    NegativeSignSelfTest = 2,
    DebugMode = 3,
}

#[test]
fn sample_usage() {
    use packed_struct::*;

    let a = SelfTestMode::DebugMode;
    assert_eq!(3, a.to_primitive());
    
    let test = TestPack {
        mode: SelfTestMode::DebugMode,
        enabled: true
    };

    let packed = test.pack();
    assert_eq!([0b00110010], packed);

    let unpacked = TestPack::unpack(&packed).unwrap();
    assert_eq!(unpacked, test);    
}
```

# Packing attributes

Syntax: 

```rust
#[packed_struct(attr1="val", attr2="val")]
struct Structure {
    #[packed_field(attr1="val", attr2="val")]
    field: u8
}
```

## Per-structure

Attribute | Values | Comment
:--|:--|:--
```size_bytes``` | ```1``` ... n | Size of the packed byte stream
```bit_numbering``` | ```msb0``` or ```lsb0``` | Bit numbering for bit positioning of fields. Required if the bits attribute field is used.
```endian``` | ```msb``` or ```lsb``` | Default integer endianness

## Per-field

Attribute | Values | Comment
:--|:--|:--
```bits``` | ```0```, ```0..``` or ```0..2``` | Position of the field in the packed structure. Three modes are supported: a single bit, the starting bit, or the range of bits, inclusive. ```0..2``` occupies 3 bits.
```bytes``` | ```0```, ```0..``` or ```0..2``` | Same as above, multiplied by 8.
```size_bits``` | ```1```, ... | Specifies the size of the packed structure. Mandatory for certain types. Specifying a range of bits like ```bits="0..2"``` can substite the required usage of ```size_bits```.
```size_bytes``` | ```1```, ... | Same as above, multiplied by 8.
```ty``` | ```enum``` | Packing helper for primitive enums.
```endian``` | ```msb``` or ```lsb``` | Integer endianness. Applies to u16/i16 and larger types.

