use crate::{PackedStructSlice, PackedStruct, types_bits::ByteArray, PackingError};

/// Slice unpacking for byte arrays
impl<T> PackedStructSlice for T where T: PackedStruct, T::ByteArray : ByteArray {
    fn pack_to_slice(&self, output: &mut [u8]) -> Result<(), crate::PackingError> {
        if output.len() != <T::ByteArray as ByteArray>::len() {
            return Err(PackingError::BufferSizeMismatch { expected: <T::ByteArray as ByteArray>::len(), actual: output.len() });
        }
        let packed = self.pack();                
        &mut output[..].copy_from_slice(&packed.as_bytes_slice());
        Ok(())
    }

    fn unpack_from_slice(src: &[u8]) -> Result<Self, crate::PackingError> {
        if src.len() != <T::ByteArray as ByteArray>::len() {
            return Err(PackingError::BufferSizeMismatch { expected: <T::ByteArray as ByteArray>::len(), actual: src.len() });
        }

        let mut s = <T::ByteArray as ByteArray>::new(0);
        s.as_mut_bytes_slice().copy_from_slice(src);
        Self::unpack(&s)
    }

    fn packed_bytes_size(_opt_self: Option<&Self>) -> Result<usize, crate::PackingError> {
        Ok(<T::ByteArray as ByteArray>::len())
    }
}
