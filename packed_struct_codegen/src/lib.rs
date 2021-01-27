#![recursion_limit = "192"]

extern crate proc_macro;
extern crate proc_macro2;

extern crate syn;
#[macro_use]
extern crate quote;

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
#[proc_macro_derive(PrimitiveEnum)]
pub fn derive_primitive_detect(input: TokenStream) -> TokenStream {
    derive_primitive(input, None)
}

#[proc_macro_derive(PrimitiveEnum_u8)]
pub fn derive_primitive_u8(input: TokenStream) -> TokenStream {
    derive_primitive(input, Some(syn::parse_str::<syn::Type>("u8").unwrap()))
}

#[proc_macro_derive(PrimitiveEnum_u16)]
pub fn derive_primitive_u16(input: TokenStream) -> TokenStream {
    derive_primitive(input, Some(syn::parse_str::<syn::Type>("u16").unwrap()))
}

#[proc_macro_derive(PrimitiveEnum_u32)]
pub fn derive_primitive_u32(input: TokenStream) -> TokenStream {
    derive_primitive(input, Some(syn::parse_str::<syn::Type>("u32").unwrap()))
}

#[proc_macro_derive(PrimitiveEnum_u64)]
pub fn derive_primitive_u64(input: TokenStream) -> TokenStream {
    derive_primitive(input, Some(syn::parse_str::<syn::Type>("u64").unwrap()))
}

#[proc_macro_derive(PrimitiveEnum_i8)]
pub fn derive_primitive_i8(input: TokenStream) -> TokenStream {
    derive_primitive(input, Some(syn::parse_str::<syn::Type>("i8").unwrap()))
}

#[proc_macro_derive(PrimitiveEnum_i16)]
pub fn derive_primitive_i16(input: TokenStream) -> TokenStream {
    derive_primitive(input, Some(syn::parse_str::<syn::Type>("i16").unwrap()))
}

#[proc_macro_derive(PrimitiveEnum_i32)]
pub fn derive_primitive_i32(input: TokenStream) -> TokenStream {
    derive_primitive(input, Some(syn::parse_str::<syn::Type>("i32").unwrap()))
}

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
