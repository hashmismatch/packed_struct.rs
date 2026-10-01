//! Derive macros for the `packed_struct` crate. Use them through `packed_struct`,
//! which also documents the packing attributes.

#![recursion_limit = "192"]
#![warn(missing_docs)]
#![allow(clippy::redundant_clone)]

use proc_macro::TokenStream;
use syn::{DeriveInput, parse_macro_input};

mod pack;
mod pack_codegen;
mod pack_codegen_docs;
mod pack_parse;
mod pack_parse_attributes;

mod primitive_enum;
mod common;
mod utils;
mod utils_syn;

/// The derive macro that generates the packing and unpacking code for your structure.
///
/// Implements `PackedStruct` and `PackedStructInfo`. With the `std` or `alloc`
/// features, it also implements `PackedStructDebug` and `Display`, and adds a
/// `packed_struct_display_formatter()` method.
///
/// The layout is configured with the `#[packed_struct]` and `#[packed_field]`
/// attributes, described in the `packed_struct` crate documentation.
#[proc_macro_derive(PackedStruct, attributes(packed_struct, packed_field))]
pub fn derive_packable_bytes(tokens: TokenStream) -> TokenStream {
    let input = parse_macro_input!(tokens as DeriveInput);
    
    let parsed = match pack_parse::parse_struct(&input) {
        Ok(p) => p,
        Err(e) => return e.to_compile_error().into()
    };

    pack_codegen::derive_pack(&parsed)
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}

/// A derive macro that generates packing and unpacking code for simple enum variants.
/// It helps with converting your enums into integer types and back, with many other helper
/// traits.
///
/// Implements `PrimitiveEnum` and `PrimitiveEnumStaticStr`, plus `PrimitiveEnumDynamicStr`
/// with the `std` or `alloc` features. The primitive type is the smallest integer that holds
/// all the discriminants, unsigned unless any of them is negative. A suffixed discriminant
/// literal, like `5u16`, sets the type instead. Use one of the `PrimitiveEnum_*` macros to
/// choose it explicitly.
#[proc_macro_derive(PrimitiveEnum)]
pub fn derive_primitive_detect(input: TokenStream) -> TokenStream {
    derive_primitive(input, None)
}

/// Same as `PrimitiveEnum`, with `u8` as the primitive type.
#[proc_macro_derive(PrimitiveEnum_u8)]
pub fn derive_primitive_u8(input: TokenStream) -> TokenStream {
    derive_primitive(input, Some(syn::parse_str::<syn::Type>("u8").unwrap()))
}

/// Same as `PrimitiveEnum`, with `u16` as the primitive type.
#[proc_macro_derive(PrimitiveEnum_u16)]
pub fn derive_primitive_u16(input: TokenStream) -> TokenStream {
    derive_primitive(input, Some(syn::parse_str::<syn::Type>("u16").unwrap()))
}

/// Same as `PrimitiveEnum`, with `u32` as the primitive type.
#[proc_macro_derive(PrimitiveEnum_u32)]
pub fn derive_primitive_u32(input: TokenStream) -> TokenStream {
    derive_primitive(input, Some(syn::parse_str::<syn::Type>("u32").unwrap()))
}

/// Same as `PrimitiveEnum`, with `u64` as the primitive type.
#[proc_macro_derive(PrimitiveEnum_u64)]
pub fn derive_primitive_u64(input: TokenStream) -> TokenStream {
    derive_primitive(input, Some(syn::parse_str::<syn::Type>("u64").unwrap()))
}

/// Same as `PrimitiveEnum`, with `i8` as the primitive type.
#[proc_macro_derive(PrimitiveEnum_i8)]
pub fn derive_primitive_i8(input: TokenStream) -> TokenStream {
    derive_primitive(input, Some(syn::parse_str::<syn::Type>("i8").unwrap()))
}

/// Same as `PrimitiveEnum`, with `i16` as the primitive type.
#[proc_macro_derive(PrimitiveEnum_i16)]
pub fn derive_primitive_i16(input: TokenStream) -> TokenStream {
    derive_primitive(input, Some(syn::parse_str::<syn::Type>("i16").unwrap()))
}

/// Same as `PrimitiveEnum`, with `i32` as the primitive type.
#[proc_macro_derive(PrimitiveEnum_i32)]
pub fn derive_primitive_i32(input: TokenStream) -> TokenStream {
    derive_primitive(input, Some(syn::parse_str::<syn::Type>("i32").unwrap()))
}

/// Same as `PrimitiveEnum`, with `i64` as the primitive type.
#[proc_macro_derive(PrimitiveEnum_i64)]
pub fn derive_primitive_i64(input: TokenStream) -> TokenStream {
    derive_primitive(input, Some(syn::parse_str::<syn::Type>("i64").unwrap()))
}

fn derive_primitive(input: TokenStream, ty: Option<syn::Type>) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    primitive_enum::derive(&input, ty)
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}
