//! Bit-level packing and unpacking for Rust
//! ===========================================
//!
//! [![Crates.io][crates-badge]][crates-url]
//! [![Documentation](https://docs.rs/packed_struct/badge.svg)](https://docs.rs/packed_struct)
//! ![master](https://github.com/hashmismatch/packed_struct.rs/workflows/Rust/badge.svg)
//!
//! # Introduction
//!
//! Packing and unpacking bit-level structures is usually a programming tasks that needlessly reinvents the wheel. This library provides
//! a meta-programming approach, using attributes to define fields and how they should be packed. The resulting trait implementations
//! provide safe packing, unpacking and runtime debugging formatters with per-field documentation generated for each structure.
//!
//! # Features
//!
//!  * Plain Rust structures, decorated with attributes
//!  * MSB or LSB integers of user-defined bit widths
//!  * Primitive enum code generation helper
//!  * MSB0 or LSB0 bit positioning
//!  * Documents the field's packing table
//!  * Runtime packing visualization
//!  * Nested packed types
//!  * Arrays of packed structures as fields
//!  * Reserved fields, their bits are always 0 or 1
//!
//! # Crate-level feature flags
//!  * `std`: use the Rust standard library. Default.
//!  * `alloc`: use the `alloc` crate for `no_std` + `alloc` scenarios. Requires nightly Rust.
//!  * `use_serde`: add serialization support to the built-in helper types.
//!  * `byte_types_64`, `byte_types_256`: enlarge the size of the generated array, byte and bit width types.
//!
//! # Sample usage
//!
//! ## Cargo.toml
//!
//! ```toml
//! [dependencies]
//! packed_struct = "0.6"
//! ```
//! ## Importing the library with the the most common traits and the derive macros
//!
//! ```rust
//! // This is only needed for pre Rust 2018
//! #[macro_use] extern crate packed_struct;
//! // Prelude import with the common imports
//! use packed_struct::prelude::*;
//! # fn main() {
//! # }
//! ```
//!
//! ## Example of a single-byte structure, with a 3 bit integer, primitive enum and a bool field.
//!
//! ```rust
//! use packed_struct::prelude::*;
//!
//! #[derive(PackedStruct)]
//! #[packed_struct(bit_numbering="msb0")]
//! pub struct TestPack {
//!     #[packed_field(bits="0..=2")]
//!     tiny_int: Integer<u8, packed_bits::Bits3>,
//!     #[packed_field(bits="3..=4", ty="enum")]
//!     mode: SelfTestMode,
//!     #[packed_field(bits="7")]
//!     enabled: bool
//! }
//!
//! #[derive(PrimitiveEnum_u8, Clone, Copy, Debug, PartialEq)]
//! pub enum SelfTestMode {
//!     NormalMode = 0,
//!     PositiveSignSelfTest = 1,
//!     NegativeSignSelfTest = 2,
//!     DebugMode = 3,
//! }
//!
//! fn main() -> Result<(), PackingError> {
//!     let test = TestPack {
//!         tiny_int: 5.into(),
//!         mode: SelfTestMode::DebugMode,
//!         enabled: true
//!     };
//!
//!     // pack into a byte array
//!     let packed: [u8; 1] = test.pack()?;
//!     assert_eq!([0b10111001], packed);
//!
//!     // unpack from a byte array
//!     let unpacked = TestPack::unpack(&packed)?;
//!     assert_eq!(*unpacked.tiny_int, 5);
//!     assert_eq!(unpacked.mode, SelfTestMode::DebugMode);
//!     assert_eq!(unpacked.enabled, true);
//!
//!     // or unpack from a slice
//!     let unpacked = TestPack::unpack_from_slice(&packed[..])?;
//!
//!     Ok(())
//! }
//! ```
//!
//! # Packing attributes
//!
//! ## Syntax
//!
//! ```rust
//! use packed_struct::prelude::*;
//!
//! #[derive(PackedStruct)]
//! #[packed_struct(attr1="val", attr2="val")]
//! pub struct Structure {
//!     #[packed_field(attr1="val", attr2="val")]
//!     field: u8
//! }
//! # fn main() {
//! # }
//! ```
//!
//! ## Per-structure attributes
//!
//! Attribute | Values | Comment
//! :--|:--|:--
//! ```size_bytes``` | ```1``` ... n | Size of the packed byte stream
//! ```bit_numbering``` | ```msb0``` or ```lsb0``` | Bit numbering for bit positioning of fields. Required if the bits attribute field is used.
//! ```endian``` | ```msb``` or ```lsb``` | Default integer endianness
//!
//! ## Per-field attributes
//!
//! Attribute | Values | Comment
//! :--|:--|:--
//! ```bits``` | ```0```, ```0..1```, ... | Position of the field in the packed structure. Three modes are supported: a single bit, the starting bit, or a range of bits. See details below.
//! ```bytes``` | ```0```, ```0..1```, ... | Same as above, multiplied by 8.
//! ```size_bits``` | ```1```, ... | Specifies the size of the packed structure. Mandatory for certain types. Specifying a range of bits like ```bits="0..2"``` can substite the required usage of ```size_bits```.
//! ```size_bytes``` | ```1```, ... | Same as above, multiplied by 8.
//! ```element_size_bits``` | ```1```, ... | For packed arrays, specifies the size of a single element of the array. Explicitly stating the size of the entire array can substite the usage of this attribute.
//! ```element_size_bytes``` | ```1```, ... | Same as above, multiplied by 8.
//! ```ty``` | ```enum``` | Packing helper for primitive enums.
//! ```endian``` | ```msb``` or ```lsb``` | Integer endianness. Applies to u16/i16 and larger types.
//! 
//! ## Bit and byte positioning
//! 
//! Used for either ```bits``` or ```bytes``` on fields. The examples are for MSB0 positioning.
//! 
//! Value | Comment
//! :--|:--
//! ```0``` | A single bit or byte
//! ```0..```, ```0:``` | The field starts at bit zero
//! ```0..2``` | Exclusive range, bits zero and one
//! ```0:1```, ```0..=1``` | Inclusive range, bits zero and one
//!
//! # More examples
//!
//! ## Mixed endian integers
//!
//! ```rust
//! use packed_struct::prelude::*;
//!
//! #[derive(PackedStruct)]
//! pub struct EndianExample {
//!     #[packed_field(endian="lsb")]
//!     int1: u16,
//!     #[packed_field(endian="msb")]
//!     int2: i32
//! }
//!
//! fn main() -> Result<(), PackingError> {
//!     let example = EndianExample {
//!         int1: 0xBBAA,
//!         int2: 0x11223344
//!     };
//!
//!     let packed = example.pack()?;
//!     assert_eq!([0xAA, 0xBB, 0x11, 0x22, 0x33, 0x44], packed);
//!     Ok(())
//! }
//! ```
//!
//! ## 24 bit LSB integers
//!
//! ```rust
//! use packed_struct::prelude::*;
//!
//! #[derive(PackedStruct)]
//! #[packed_struct(endian="lsb")]
//! pub struct LsbIntExample {
//!     int1: Integer<u32, packed_bits::Bits24>,
//! }
//!
//! fn main() -> Result<(), PackingError> {
//!     let example = LsbIntExample {
//!         int1: 0xCCBBAA.into()
//!     };
//!
//!     let packed = example.pack()?;
//!     assert_eq!([0xAA, 0xBB, 0xCC], packed);
//!     Ok(())
//! }
//! ```
//!
//! ## Nested packed types
//! 
//! ```rust
//! use packed_struct::prelude::*;
//! #[derive(PackedStruct, Debug, PartialEq)]
//! #[packed_struct(endian="lsb")]
//! pub struct Duration {
//!     minutes: u8,
//!     seconds: u8,
//! }
//! #[derive(PackedStruct, Debug, PartialEq)]
//! pub struct Record {
//!     #[packed_field(element_size_bytes="2")]
//!     span: Duration,
//!     events: u8,
//! }
//! fn main() -> Result<(), PackingError> {
//!     let example = Record {
//!         span: Duration {
//!             minutes: 10,
//!             seconds: 34,
//!         },
//!         events: 3,
//!     };
//!     let packed = example.pack()?;
//!     let unpacked = Record::unpack(&packed)?;
//!     assert_eq!(example, unpacked);
//!     Ok(())
//! }
//! ```
//!
//! ## Nested packed types within arrays
//!
//! ```rust
//! use packed_struct::prelude::*;
//!
//! #[derive(PackedStruct, Default, Debug, PartialEq)]
//! #[packed_struct(bit_numbering="msb0")]
//! pub struct TinyFlags {
//!     _reserved: ReservedZero<packed_bits::Bits4>,
//!     flag1: bool,
//!     val1: Integer<u8, packed_bits::Bits2>,
//!     flag2: bool
//! }
//!
//! #[derive(PackedStruct, Debug, PartialEq)]
//! pub struct Settings {
//!     #[packed_field(element_size_bits="4")]
//!     values: [TinyFlags; 4]
//! }
//!
//! fn main() -> Result<(), PackingError> {
//!     let example = Settings {
//!         values: [
//!             TinyFlags { flag1: true,  val1: 1.into(), flag2: false, .. TinyFlags::default() },
//!             TinyFlags { flag1: true,  val1: 2.into(), flag2: true,  .. TinyFlags::default() },
//!             TinyFlags { flag1: false, val1: 3.into(), flag2: false, .. TinyFlags::default() },
//!             TinyFlags { flag1: true,  val1: 0.into(), flag2: false, .. TinyFlags::default() },
//!         ]
//!     };
//!
//!     let packed = example.pack()?;
//!     let unpacked = Settings::unpack(&packed)?;
//!
//!     assert_eq!(example, unpacked);
//!     Ok(())
//! }
//! ```
//! 
//! # Primitive enums with simple discriminants
//! 
//! Supported backing integer types: ```u8```, ```u16```, ```u32```, ```u64```, ```i8```, ```i16```, ```i32```, ```i64```.
//! 
//! Explicit or implicit backing type:
//! 
//! ```rust
//! use packed_struct::prelude::*;
//!
//! #[derive(PrimitiveEnum, Clone, Copy, PartialEq, Debug)]
//! pub enum ImplicitType {
//!     VariantMin = 0,
//!     VariantMax = 255
//! }
//! 
//! #[derive(PrimitiveEnum_i16, Clone, Copy)]
//! pub enum ExplicitType {
//!     VariantMin = -32768,
//!     VariantMax = 32767
//! }
//! 
//! fn main() {
//!     use packed_struct::PrimitiveEnum;
//!     
//!     let t = ImplicitType::VariantMin;
//!     let tn: u8 = t.to_primitive();
//!     assert_eq!(0, tn);
//!
//!     let t = ImplicitType::from_primitive(255).unwrap();
//!     assert_eq!(ImplicitType::VariantMax, t);
//! }
//! ```
//! 
//! # Primitive enum packing with support for catch-all unknown values
//! 
//! ```rust
//! use packed_struct::prelude::*;
//! 
//! #[derive(PrimitiveEnum_u8, Debug, Clone, Copy)]
//! pub enum Field {
//!     A = 1,
//!     B = 2,
//!     C = 3
//! }
//! 
//! #[derive(PackedStruct, Debug, PartialEq)]
//! #[packed_struct(bit_numbering="msb0")]
//! pub struct Register {
//!     #[packed_field(bits="0..4", ty="enum")]
//!     field: EnumCatchAll<Field>
//! }
//! 
//! # fn main() {}
//! ```
//! [crates-badge]: https://img.shields.io/crates/v/packed_struct.svg
//! [crates-url]: https://crates.io/crates/packed_struct

#![cfg_attr(not(feature = "std"), no_std)]

#![cfg_attr(feature="alloc", feature(alloc))]

extern crate packed_struct_codegen;

#[cfg(feature="alloc")]
#[macro_use]
extern crate alloc;

#[cfg(feature = "use_serde")]
extern crate serde;
#[cfg(feature = "use_serde")]
#[macro_use] extern crate serde_derive;

mod internal_prelude;

#[macro_use]
mod packing;

mod primitive_enum;

pub use primitive_enum::*;

#[cfg(any(feature="alloc", feature="std"))]
pub mod debug_fmt;

mod types_array;
mod types_basic;
mod types_bits;
mod types_generic;
mod types_num;
mod types_reserved;

pub mod types_tuples;

#[cfg(any(feature="alloc", feature="std"))]
mod types_vec;

/// Implementations and wrappers for various packing types.
pub mod types {
    pub use super::types_basic::*;

    /// Types that specify the exact number of bits a packed integer should occupy.
    pub mod bits {
        pub use super::super::types_bits::*;
    }

    pub use super::types_num::*;
    pub use super::types_array::*;
    pub use super::types_reserved::*;
    pub use super::types_generic::*;
    #[cfg(any(feature="alloc", feature="std"))]
    pub use super::types_vec::*;
}

pub use self::packing::*;

/// The derivation macros for packing and enums.
pub mod derive {
    pub use packed_struct_codegen::PackedStruct;
    pub use packed_struct_codegen::PrimitiveEnum;
    pub use packed_struct_codegen::{PrimitiveEnum_u8, PrimitiveEnum_u16, PrimitiveEnum_u32, PrimitiveEnum_u64};
    pub use packed_struct_codegen::{PrimitiveEnum_i8, PrimitiveEnum_i16, PrimitiveEnum_i32, PrimitiveEnum_i64};
}

pub mod prelude {
    //! Re-exports the most useful traits and types. Meant to be glob imported.

    pub use super::derive::*;

    pub use crate::{PackedStruct, PackedStructSlice, PackingError};

    pub use crate::PrimitiveEnum;
    #[cfg(any(feature="alloc", feature="std"))]
    pub use crate::PrimitiveEnumDynamicStr;

    #[cfg(not(any(feature="alloc", feature="std")))]
    pub use crate::PrimitiveEnumStaticStr;


    pub use crate::EnumCatchAll;

    pub use crate::types::*;
    pub use crate::types::bits as packed_bits;
}

use internal_prelude::v1::*;

fn lib_get_slice<T, I: slice::SliceIndex<[T]>>(src: &[T], index: I) -> Result<&<I as slice::SliceIndex<[T]>>::Output, PackingError> {
    let slice_len = src.len();
    src.get(index).ok_or(PackingError::SliceIndexingError { slice_len })
}

fn lib_get_mut_slice<T, I: slice::SliceIndex<[T]>>(src: &mut [T], index: I) -> Result<&mut <I as slice::SliceIndex<[T]>>::Output, PackingError> {
    let slice_len = src.len();
    src.get_mut(index).ok_or(PackingError::SliceIndexingError { slice_len })
}