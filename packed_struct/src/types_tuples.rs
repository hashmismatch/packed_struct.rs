//! Tuples of types that can be packed together.

use internal_prelude::v1::*;

use crate::{PackedStructSlice, PackingError};

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
        let t1 = A::unpack_from_slice(&src[i..(i+n)])?;
        i += n;

        let t2 = B::unpack_from_slice(&src[i..])?;
        
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
