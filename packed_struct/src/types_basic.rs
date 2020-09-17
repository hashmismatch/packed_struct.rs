use super::packing::*;

impl PackedStruct for bool {
    type ByteArray = [u8; 1];

    #[inline]
    fn pack(&self) -> [u8; 1] {
        if *self { [1] } else { [0] }
    }

    #[inline]
    fn unpack(src: &[u8; 1]) -> Result<bool, PackingError> {
        match src[0] {
            1 => Ok(true),
            0 => Ok(false),
            _ => Err(PackingError::InvalidValue)
        }
    }
}

impl PackedStructInfo for bool {
    #[inline]
    fn packed_bits() -> usize {
        1
    }
}


impl PackedStruct for u8 {
    type ByteArray = [u8; 1];

    #[inline]
    fn pack(&self) -> [u8; 1] {
        [*self]
    }

    #[inline]
    fn unpack(src: &[u8; 1]) -> Result<u8, PackingError> {
        Ok(src[0])
    }
}

impl PackedStructInfo for u8 {
    #[inline]
    fn packed_bits() -> usize {
        8
    }
}


impl PackedStruct for i8 {
    type ByteArray = [u8; 1];

    #[inline]
    fn pack(&self) -> Self::ByteArray {
        [*self as u8]
    }

    #[inline]
    fn unpack(src: &Self::ByteArray) -> Result<i8, PackingError> {
        Ok(src[0] as i8)
    }
}

impl PackedStructInfo for i8 {
    #[inline]
    fn packed_bits() -> usize {
        8
    }
}


impl PackedStruct for () {
    type ByteArray = [u8; 0];

    #[inline]
    fn pack(&self) -> [u8; 0] {
        []
    }

    #[inline]
    fn unpack(_src: &[u8; 0]) -> Result<(), PackingError> {
        Ok(())
    }
}

impl PackedStructInfo for () {
    #[inline]
    fn packed_bits() -> usize {
        0
    }
}