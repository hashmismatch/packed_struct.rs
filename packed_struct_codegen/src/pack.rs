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
        field: Box<FieldRegular>
    },
    Array {
        ident: syn::Ident,
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
    Integer {
        integer: syn::Type,
    },
    Endiannes {
        endian: syn::Type
    },
    PrimitiveEnum
}


pub struct PackStruct<'a> {
    pub fields: Vec<FieldKind>,
    pub num_bytes: usize,
    pub num_bits: usize,
    pub derive_input: &'a syn::DeriveInput
}








