use prelude::v1::*;

pub trait PackedStruct<B> where Self: Sized {
    fn pack(&self) -> B;
    fn unpack(src: &B) -> Result<Self, PackingError>;
}

pub trait PackedStructInfo {
    fn packed_bits() -> usize;
}

pub trait PackedStructSlice where Self: Sized {
    fn pack_to_slice(&self, output: &mut [u8]) -> Result<(), PackingError>;
    fn unpack_from_slice(src: &[u8]) -> Result<Self, PackingError>;
    fn packed_bytes() -> usize;
}



#[cfg(any(feature="core_collections", feature="std"))]
pub trait PackedStructDebug {
    fn fmt_fields(&self, fmt: &mut Formatter) -> Result<(), FmtError>;
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum PackingError {
    InvalidValue,
    BitsError,
    BufferTooSmall,
    NotImplemented
}


macro_rules! packing_slice {
    ($T: path; $num_bytes: expr) => (

        impl PackedStructSlice for $T {
            #[inline]
            fn pack_to_slice(&self, output: &mut [u8]) -> Result<(), PackingError> {
                if output.len() != $num_bytes {
                    return Err(PackingError::BufferTooSmall);
                }
                let packed = self.pack();                
                &mut output[..].copy_from_slice(&packed[..]);
                Ok(())
            }

            #[inline]
            fn unpack_from_slice(src: &[u8]) -> Result<Self, PackingError> {
                if src.len() != $num_bytes {
                    return Err(PackingError::BufferTooSmall);
                }
                let mut s = [0; $num_bytes];
                &mut s[..].copy_from_slice(src);
                Self::unpack(&s)
            }

            #[inline]
            fn packed_bytes() -> usize {
                $num_bytes
            }
        }

    )
}


