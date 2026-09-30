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
    Array(Box<FieldArray>)
}

/// An array of uniform elements, laid out back to back. Only the first element is stored,
/// the rest are derived from it, so that the codegen doesn't scale with the array's size.
pub struct FieldArray {
    pub ident: syn::Ident,
    pub size: usize,
    /// The first element of the array, with its absolute bit positions.
    pub element: FieldRegular
}

impl FieldArray {
    pub fn element_bits(&self) -> usize {
        self.element.bit_width
    }

    /// The bits of the i-th element, in the same format as `FieldRegular::bit_range`.
    pub fn element_bit_range(&self, i: usize) -> Range<usize> {
        let start = self.element.bit_range.start + (i * self.element_bits());
        start..(start + self.element_bits() - 1)
    }

    /// The bits of the whole array, in the same format as `FieldRegular::bit_range`.
    pub fn bit_range(&self) -> Range<usize> {
        self.element.bit_range.start..self.element_bit_range(self.size - 1).end
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

impl FieldRegular {
    /// The same field, moved to start at a different bit.
    pub fn with_start_bit(&self, start: usize) -> FieldRegular {
        let bit_range = start..(start + self.bit_width - 1);
        FieldRegular {
            ty: self.ty.clone(),
            serialization_wrappers: self.serialization_wrappers.clone(),
            bit_width: self.bit_width,
            bit_range_rust: bit_range.start..(bit_range.end + 1),
            bit_range
        }
    }
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
