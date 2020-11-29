//! Helpers for vectors of slice-packable structures

use internal_prelude::v1::*;

use crate::{PackedStructSlice, PackingError, lib_get_mut_slice, lib_get_slice};

/// This can only be used as a vector of structures that have a statically known size
impl<T> PackedStructSlice for Vec<T> where T: PackedStructSlice {
    fn pack_to_slice(&self, output: &mut [u8]) -> Result<(), PackingError> {
        let expected_size = Self::packed_bytes_size(Some(self))?;
        if output.len() != expected_size {
            return Err(crate::PackingError::BufferSizeMismatch { expected: expected_size, actual: output.len() });
        }

        let size = T::packed_bytes_size(None)?;

        for (i, item) in self.iter().enumerate() {
            let mut item_out = lib_get_mut_slice(output, (i * size)..((i+1)*size))?;
            item.pack_to_slice(item_out)?;
        }

        Ok(())
    }

    fn unpack_from_slice(src: &[u8]) -> Result<Self, PackingError> {
        let item_size = T::packed_bytes_size(None)?;
        if item_size == 0 || src.len() == 0 { return Ok(vec![]); }
        let modulo = src.len() % item_size;
        if modulo != 0 {
            return Err(crate::PackingError::BufferModMismatch { actual_size: src.len(), modulo_required: item_size });
        }
        let n = src.len() / item_size;

        let mut vec = Vec::with_capacity(n);
        for i in 0..n {
            let item_src = lib_get_slice(src, (i*item_size)..((i+1)*item_size))?;
            let item = T::unpack_from_slice(item_src)?;
            vec.push(item);
        }        

        Ok(vec)
    }

    fn packed_bytes_size(opt_self: Option<&Self>) -> Result<usize, PackingError> {
        match opt_self {
            None => Err(PackingError::InstanceRequiredForSize),
            Some(s) => {
                Ok(s.len() * T::packed_bytes_size(None)?)
            }
        }
    }
}