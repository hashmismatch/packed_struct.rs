extern crate quote;
extern crate syn;

use std::ops::*;
use pack_parse::*;

#[derive(Debug)]
pub struct FieldInfo {
    pub ident: syn::Ident,
    pub ty: syn::Ty,

    pub serialization_wrapper_ty: Option<syn::Ident>,
    
    pub bit_width: usize,
    pub bits_position: BitsPositionParsed,

    pub field_kind: FieldKind
}

#[derive(Debug)]
pub enum FieldKind {
    Normal,
    Array { size: usize, ident: syn::Ident },
    Enum { pack_ty: syn::Ident }
}

#[derive(Debug)]
pub struct FieldExpanded {
    pub info: FieldInfo,
    /// The range as parsed by our scripts. A single byte: 0..7
    pub bit_range: Range<usize>,
    /// The range that can be used by rust's slices. A single byte: 0..8
    pub bit_range_rust: Range<usize>
}


#[derive(Debug)]
pub struct PackStruct {
    pub ast: syn::MacroInput,    
    pub fields: Vec<FieldExpanded>,
    pub num_bytes: usize,
    pub num_bits: usize
}









