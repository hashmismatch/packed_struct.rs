extern crate quote;
extern crate syn;

use std::ops::*;
use crate::pack_parse::*;

#[derive(Debug)]
pub struct FieldMidPositioning {
    pub bit_width: usize,
    pub bits_position: BitsPositionParsed,
}

pub enum FieldKind {
    Regular {
        ident: syn::Ident,
        field: FieldRegular
    },
    Array {
        ident: syn::Ident,
        size: usize,
        elements: Vec<FieldRegular>
    }
}

pub struct FieldRegular {
    pub ty: syn::Type,
    pub serialization_wrappers: Vec<SerializationWrapper>,
    pub bit_width: usize,
    /// The range as parsed by our parser. A single byte: 0..7
    pub bit_range: Range<usize>,
    /// The range that can be used by rust's slices. A single byte: 0..8
    pub bit_range_rust: Range<usize>
}

#[derive(Clone)]
pub enum SerializationWrapper {
    IntegerWrapper {
        integer: syn::Type,
    },
    EndiannesWrapper {
        endian: syn::Type
    },
    PrimitiveEnumWrapper
}


pub struct PackStruct<'a> {
    pub fields: Vec<FieldKind>,
    pub num_bytes: usize,
    pub num_bits: usize,
    pub data_struct: &'a syn::DataStruct,
    pub derive_input: &'a syn::DeriveInput
}








