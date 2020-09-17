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
            len_dy = len_static - total_length;
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


impl<A, B> PackedStructSlice for (A, B) where A: PackedStructSlice, B: PackedStructSlice {
    fn pack_to_slice(&self, output: &mut [u8]) -> Result<(), crate::PackingError> {
        let lengths = {
            let mut builder = StructLengthBuilder::new();
            builder.add::<A>(Some(&self.0))?;
            builder.add::<B>(Some(&self.1))?;
            builder.build(output.len())?
        };

        let ranges = lengths.get_ranges();

        self.0.pack_to_slice(lib_get_mut_slice(output, ranges.get(0).ok_or(crate::PackingError::InternalError)?.clone())?)?;
        self.1.pack_to_slice(lib_get_mut_slice(output, ranges.get(1).ok_or(crate::PackingError::InternalError)?.clone())?)?;
        
        Ok(())
    }

    fn unpack_from_slice(src: &[u8]) -> Result<Self, crate::PackingError> {
        let lengths = {
            let mut builder = StructLengthBuilder::new();
            builder.add::<A>(None)?;
            builder.add::<B>(None)?;
            builder.build(src.len())?
        };

        let ranges = lengths.get_ranges();

        Ok(
            (
                A::unpack_from_slice(lib_get_slice(src, ranges.get(0).ok_or(crate::PackingError::InternalError)?.clone())?)?,
                B::unpack_from_slice(lib_get_slice(src, ranges.get(1).ok_or(crate::PackingError::InternalError)?.clone())?)?
            )
        )
    }

    fn packed_bytes_size(opt_self: Option<&Self>) -> Result<usize, PackingError> {
        Ok(

            A::packed_bytes_size(opt_self.map(|s| &s.0))?
            +
            B::packed_bytes_size(opt_self.map(|s| &s.1))?
        )
    }
}

/*
fn build<A, B>() -> Result<StructLengths, crate::PackingError> {
    let builder = StructLengthBuilder::new();
    builder.add::<A>(None);
    builder.add::<B>(None);
    builder.build(src.len())
}
*/


/*
impl<A, B> PackedStructSlice for (A, B) where A: PackedStructSlice, B: PackedStructSlice {
    fn pack_to_slice(&self, output: &mut [u8]) -> Result<(), crate::PackingError> {
        let expected_size = Self::packed_bytes_size(Some(self))?;
        if output.len() != expected_size {
            return Err(crate::PackingError::BufferSizeMismatch { expected: expected_size, actual: output.len() });
        }

        let mut i = 0;

        let n = A::packed_bytes_size(Some(&self.0))?;
        self.0.pack_to_slice(&mut output[i..(i+n)])?;
        i += n;

        let n = B::packed_bytes_size(Some(&self.1))?;
        self.1.pack_to_slice(&mut output[i..(i+n)])?;
        i += n;

        Ok(())
    }

    fn unpack_from_slice(src: &[u8]) -> Result<Self, crate::PackingError> {
        let mut i = 0;

        let n = A::packed_bytes_size(None)?;
        let src_a = lib_get_slice(src, i..(i+n))?;
        let t1 = A::unpack_from_slice(src_a)?;
        i += n;

        let src_b = lib_get_slice(src, i..)?;
        let t2 = B::unpack_from_slice(src_b)?;
        
        Ok((t1, t2))
    }

    fn packed_bytes_size(opt_self: Option<&Self>) -> Result<usize, PackingError> {
        Ok(
            A::packed_bytes_size(opt_self.map(|m| &m.0))?
            +
            B::packed_bytes_size(opt_self.map(|m| &m.1))?
        )
    }
}
*/


macro_rules! for_each_tuple_ {
    ( $m:ident !! ) => (
        $m! { }
    );
    ( $m:ident !! $h:ident, $($t:ident,)* ) => (
        $m! { $h $($t)* }
        for_each_tuple_! { $m !! $($t,)* }
    );
}
macro_rules! for_each_tuple {
    ($m:ident) => {
        for_each_tuple_! { $m !! A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, }
    };
}