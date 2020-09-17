//! Tuples of types that can be packed together.
//!
//! Supports having one dynamically sized packed structure type within the tuple.

use internal_prelude::v1::*;

use crate::{PackedStructSlice, PackingError, lib_get_slice, lib_get_mut_slice};

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

    fn add<S: PackedStructSlice>(&mut self, st: Option<&S>) -> Result<StructLength, PackingError> {
        let len = match S::packed_bytes_size(st) {
            Err(PackingError::InstanceRequiredForSize) => StructLength::Dynamic,
            Err(e) => return Err(e),
            Ok(s) => StructLength::Static(s)
        };
        self.lengths[self.i] = len;
        self.i += 1;
        Ok(len)
    }

    fn build(self, total_length: usize) -> Result<StructLengths, PackingError> {
        // at most one can be dynamic!
        let lengths = &self.lengths[..self.i];
        let dy = lengths.iter().filter(|l| l == &&StructLength::Dynamic).count();
        if dy > 1 {
            return Err(PackingError::InstanceRequiredForSize);
        }

        let len_static: usize = lengths.iter().filter_map(|l| if let StructLength::Static(s) = l { Some(*s) } else { None }).sum();
        let mut len_dy = 0;
        if dy == 1 {
            len_dy = total_length - len_static;
        }

        if len_static + len_dy != total_length {
            return Err(PackingError::BufferSizeMismatch { expected: len_static + len_dy, actual: total_length });
        }

        let mut output_lengths = [0; 16];
        for (i, l) in lengths.iter().enumerate() {
            output_lengths[i] = match l {
                StructLength::Empty => panic!("shouldn't happen"),
                StructLength::Dynamic => len_dy,
                StructLength::Static(s) => *s
            };
        }

        Ok(StructLengths::new(output_lengths, self.i))
    }
}

struct StructLengths {
    lengths: [usize; 16],
    ranges: [Range<usize>; 16],
    len: usize
}

impl StructLengths {
    fn new(lengths: [usize; 16], len: usize) -> Self {
        let mut ranges = [0..0, 0..0, 0..0, 0..0, 0..0, 0..0, 0..0, 0..0, 0..0, 0..0, 0..0, 0..0 , 0..0, 0..0, 0..0, 0..0];
        let mut n = 0;
        for i in 0..len {
            let l = lengths[i];
            ranges[i] = n..(n+l);
            n += l;
        }

        StructLengths {
            lengths,
            ranges,
            len
        }
    }

    fn get_lengths(&self) -> &[usize] {
        &self.lengths[..self.len]
    }

    fn get_ranges(&self) -> &[Range<usize>] {
        &self.ranges[..self.len]
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
            fn pack_to_slice(&self, output: &mut [u8]) -> Result<(), crate::PackingError> {
                let lengths = {
                    let mut builder = StructLengthBuilder::new();
                    builder.add::<$typ>(Some(&self.$idx))?;
                    $( builder.add::<$ntyp>(Some(&self.$nidx))?; )*
                    builder.build(output.len())?
                };

                let ranges = lengths.get_ranges();

                self.$idx.pack_to_slice(lib_get_mut_slice(output, ranges.get($idx).ok_or(crate::PackingError::InternalError)?.clone())?)?;
                $( self.$nidx.pack_to_slice(lib_get_mut_slice(output, ranges.get($nidx).ok_or(crate::PackingError::InternalError)?.clone())?)?; )*
                
                Ok(())
            }

            fn unpack_from_slice(src: &[u8]) -> Result<Self, crate::PackingError> {
                let lengths = {
                    let mut builder = StructLengthBuilder::new();                    
                    builder.add::<$typ>(None)?;
                    $( builder.add::<$ntyp>(None)?; )*
                    builder.build(src.len())?
                };

                let ranges = lengths.get_ranges();

                Ok(
                    (
                        $typ::unpack_from_slice(lib_get_slice(src, ranges.get($idx).ok_or(crate::PackingError::InternalError)?.clone())?)?,
                        $( $ntyp::unpack_from_slice(lib_get_slice(src, ranges.get($nidx).ok_or(crate::PackingError::InternalError)?.clone())?)? ),*
                    )
                )
            }

            fn packed_bytes_size(opt_self: Option<&Self>) -> Result<usize, PackingError> {
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