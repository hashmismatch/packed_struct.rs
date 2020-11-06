use internal_prelude::v1::*;

use crate::types_bits::ByteArray;

/// A structure that can be packed and unpacked from a byte array.
/// 
/// In case the structure occupies less bits than there are in the byte array,
/// the packed that should be aligned to the end of the array, with leading bits
/// being ignored.
/// 
/// 10 bits packs into: [0b00000011, 0b11111111]
pub trait PackedStruct where Self: Sized {
    type ByteArray : ByteArray;
    /// Packs the structure into a byte array.
    fn pack(&self) -> PackingResult<Self::ByteArray>;
    /// Unpacks the structure from a byte array.
    fn unpack(src: &Self::ByteArray) -> PackingResult<Self>;
}

/// Infos about a particular type that can be packaged.
pub trait PackedStructInfo {
    /// Number of bits that this structure occupies when being packed.
    fn packed_bits() -> usize;
}

/// A structure that can be packed and unpacked from a slice of bytes.
pub trait PackedStructSlice where Self: Sized {
    /// Pack the structure into an output buffer.
    fn pack_to_slice(&self, output: &mut [u8]) -> PackingResult<()>;
    /// Unpack the structure from a buffer.
    fn unpack_from_slice(src: &[u8]) -> PackingResult<Self>;
    /// Number of bytes that the type or this particular instance of this structure demands for packing or unpacking.
    fn packed_bytes_size(opt_self: Option<&Self>) -> PackingResult<usize>;

    #[cfg(any(feature="alloc", feature="std"))]
    fn pack_to_vec(&self) -> PackingResult<Vec<u8>> {
        let size = Self::packed_bytes_size(Some(self))?;
        let mut buf = vec![0; size];
        self.pack_to_slice(&mut buf)?;
        Ok(buf)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Serialize)]
/// Packing errors that might occur during packing or unpacking
pub enum PackingError {
    InvalidValue,
    BitsError,
    BufferTooSmall,
    NotImplemented,
    InstanceRequiredForSize,
    MoreThanOneDynamicType,
    BufferSizeMismatch { expected: usize, actual: usize },
    BufferModMismatch { actual_size: usize, modulo_required: usize },
    SliceIndexingError { slice_len: usize },
    InternalError
}

impl Display for PackingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }    
}

#[cfg(feature="std")]
impl ::std::error::Error for PackingError {
    fn description(&self) -> &str {
        match *self {
            PackingError::InvalidValue => "Invalid value",
            PackingError::BitsError => "Bits error",
            PackingError::BufferTooSmall => "Buffer too small",            
            PackingError::BufferSizeMismatch { .. } => "Buffer size mismatched",
            PackingError::NotImplemented => "Not implemented",
            PackingError::InstanceRequiredForSize => "This structure's packing size can't be determined statically, an instance is required.",
            PackingError::BufferModMismatch { .. } => "The structure's size is not a multiple of the item's size",
            PackingError::SliceIndexingError { .. } => "Failed to index into a slice",
            PackingError::MoreThanOneDynamicType => "Only one dynamically sized type is supported in the tuple",
            PackingError::InternalError => "Internal error"
        }
    }
}

pub type PackingResult<T> = Result<T, PackingError>;