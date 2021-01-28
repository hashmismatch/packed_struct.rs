//! Tuples of types that can be packed together. Only byte-sized structures can be chained together.
//!
//! Supports having one dynamically sized packed structure type within the tuple.
//!
//! # Example with ad-hoc chained structures
//!
//! ```rust
//! use packed_struct::prelude::*;
//!
//! type Message = (u8, [u8; 4], u8);
//!
//! fn main() {
//!     let raw = [0x10, 0x20, 0x21, 0x22, 0x23, 0x30];
//!     let unpacked = Message::unpack_from_slice(&raw).unwrap();
//!     assert_eq!(0x10, unpacked.0);
//!     assert_eq!([0x20, 0x21, 0x22, 0x23], unpacked.1);
//!     assert_eq!(0x30, unpacked.2);
//!     let packed = unpacked.pack_to_vec().unwrap();
//!     assert_eq!(&raw[..], &packed[..]);
//! }
//! ```
//!
//! # Example with a dynamically sized structure
//!
//! ```rust
//! extern crate packed_struct;
//!
//! use packed_struct::prelude::*;
//!
//! type Message = (u8, Vec<u8>, u8);
//!
//! fn main() {
//!     let raw = [0x10, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x30];
//!     let unpacked = Message::unpack_from_slice(&raw).unwrap();
//!     assert_eq!(0x10, unpacked.0);
//!     assert_eq!(&[0x20, 0x21, 0x22, 0x23, 0x24, 0x25], &unpacked.1[..]);
//!     assert_eq!(0x30, unpacked.2);
//!     let packed = unpacked.pack_to_vec().unwrap();
//!     assert_eq!(&raw[..], &packed[..]);
//! }
//! ```

use crate::internal_prelude::v1::*;

use crate::{PackedStructSlice, PackingError, PackingResult, lib_get_mut_slice, lib_get_slice};

#[derive(Debug, Copy, Clone, PartialEq)]
enum StructLength {
    Empty,
    Dynamic,
    Static(usize)
}

struct StructLengthBuilder {
    lengths: [StructLength; 16],
    i: usize
}

impl StructLengthBuilder {
    fn new() -> Self {
        StructLengthBuilder {
            lengths: [StructLength::Empty; 16],
            i: 0
        }
    }

    fn add<S: PackedStructSlice>(&mut self, st: Option<&S>) -> PackingResult<StructLength> {
        let len = match S::packed_bytes_size(st) {
            Err(PackingError::InstanceRequiredForSize) => StructLength::Dynamic,
            Err(e) => return Err(e),
            Ok(s) => StructLength::Static(s)
        };
        let target = lib_get_mut_slice(&mut self.lengths, self.i)?;
        *target = len;
        self.i += 1;
        Ok(len)
    }

    fn build(self, total_length: usize) -> PackingResult<StructLengths> {
        // at most one can be dynamic!
        let lengths = lib_get_slice(&self.lengths, ..self.i)?;
        let dy = lengths.iter().filter(|l| l == &&StructLength::Dynamic).count();
        if dy > 1 {
            return Err(PackingError::MoreThanOneDynamicType);
        }

        let len_static: usize = lengths.iter().filter_map(|l| if let StructLength::Static(s) = l { Some(*s) } else { None }).sum();
        let len_dy = if dy == 1 {
            total_length - len_static
        } else {
            0
        };

        if len_static + len_dy != total_length {
            return Err(PackingError::BufferSizeMismatch { expected: len_static + len_dy, actual: total_length });
        }

        let mut output_lengths = [0; 16];
        for (i, l) in lengths.iter().enumerate() {
            let output = lib_get_mut_slice(&mut output_lengths, i)?;
            *output = match l {
                StructLength::Empty => return Err(PackingError::InternalError), /* shouldn't happen */
                StructLength::Dynamic => len_dy,
                StructLength::Static(s) => *s
            };
        }

        StructLengths::new(output_lengths, self.i)
    }
}

struct StructLengths {
    ranges: [Range<usize>; 16],
    len: usize
}

impl StructLengths {
    fn new(lengths: [usize; 16], len: usize) -> PackingResult<Self> {
        let mut ranges = [0..0, 0..0, 0..0, 0..0, 0..0, 0..0, 0..0, 0..0, 0..0, 0..0, 0..0, 0..0 , 0..0, 0..0, 0..0, 0..0];
        let mut n = 0;
        for i in 0..len {
            let l = lib_get_slice(&lengths, i)?;
            let ranges_out = lib_get_mut_slice(&mut ranges, i)?;
            *ranges_out = n..(n+l);
            n += l;
        }

        Ok(StructLengths {
            ranges,
            len
        })
    }

    fn get_ranges(&self) -> PackingResult<&[Range<usize>]> {
        lib_get_slice(&self.ranges, ..self.len)
    }
}

macro_rules! tuple_impls {
    () => {}; // no more

    (($idx:tt => $typ:ident), $( ($nidx:tt => $ntyp:ident), )*) => {
        /*
         * Invoke recursive reversal of list that ends in the macro expansion implementation
         * of the reversed list
        */
        tuple_impls!([($idx, $typ);] $( ($nidx => $ntyp), )*);
        tuple_impls!($( ($nidx => $ntyp), )*); // invoke macro on tail
    };

    /*
     * ([accumulatedList], listToReverse); recursively calls tuple_impls until the list to reverse
     + is empty (see next pattern)
    */
    ([$(($accIdx: tt, $accTyp: ident);)+]  ($idx:tt => $typ:ident), $( ($nidx:tt => $ntyp:ident), )*) => {
      tuple_impls!([($idx, $typ); $(($accIdx, $accTyp); )*] $( ($nidx => $ntyp), ) *);
    };

    ([($idx:tt, $typ:ident); $( ($nidx:tt, $ntyp:ident); )*]) => {

        impl<$typ, $( $ntyp ),*> PackedStructSlice for ($typ, $( $ntyp ),*) 
        where 
            $typ: PackedStructSlice,
            $( $ntyp: PackedStructSlice ),*
        {
            fn pack_to_slice(&self, output: &mut [u8]) -> PackingResult<()> {
                let lengths = {
                    let mut builder = StructLengthBuilder::new();
                    builder.add::<$typ>(Some(&self.$idx))?;
                    $( builder.add::<$ntyp>(Some(&self.$nidx))?; )*
                    builder.build(output.len())?
                };

                let ranges = lengths.get_ranges()?;

                self.$idx.pack_to_slice(lib_get_mut_slice(output, ranges.get($idx).ok_or(crate::PackingError::InternalError)?.clone())?)?;
                $( self.$nidx.pack_to_slice(lib_get_mut_slice(output, ranges.get($nidx).ok_or(crate::PackingError::InternalError)?.clone())?)?; )*
                
                Ok(())
            }

            fn unpack_from_slice(src: &[u8]) -> PackingResult<Self> {
                let lengths = {
                    let mut builder = StructLengthBuilder::new();                    
                    builder.add::<$typ>(None)?;
                    $( builder.add::<$ntyp>(None)?; )*
                    builder.build(src.len())?
                };

                let ranges = lengths.get_ranges()?;

                Ok(
                    (
                        $typ::unpack_from_slice(lib_get_slice(src, ranges.get($idx).ok_or(crate::PackingError::InternalError)?.clone())?)?,
                        $( $ntyp::unpack_from_slice(lib_get_slice(src, ranges.get($nidx).ok_or(crate::PackingError::InternalError)?.clone())?)? ),*
                    )
                )
            }

            fn packed_bytes_size(opt_self: Option<&Self>) -> PackingResult<usize> {
                let sizes = [
                    $typ::packed_bytes_size(opt_self.map(|s| &s.$idx))?,
                    $( $ntyp::packed_bytes_size(opt_self.map(|s| &s.$nidx))? ),*
                ];

                Ok(sizes.iter().sum())
            }
        }
    };
}

tuple_impls!(
    (9 => J),
    (8 => I),
    (7 => H),
    (6 => G),
    (5 => F),
    (4 => E),
    (3 => D),
    (2 => C),
    (1 => B),
    (0 => A),
);