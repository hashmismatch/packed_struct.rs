use crate::{PackedStruct, PackedStructInfo, PackingError, PackingResult};

impl<const N: usize> PackedStruct for [u8; N] {
    type ByteArray = [u8; N];

    #[inline]
    fn pack(&self) -> PackingResult<Self::ByteArray> {
        Ok(*self)
    }

    #[inline]
    fn unpack(src: &Self::ByteArray) -> Result<Self::ByteArray, PackingError> {
        Ok(*src)
    }    
}


impl<const N: usize> PackedStructInfo for [u8; N] {
    #[inline]
    fn packed_bits() -> usize {
        N * 8
    } 
}