use crate::internal_prelude::v1::*;

use crate::types_bits::ByteArray;

/// A structure that can be packed and unpacked from a byte array.
///
/// In case the structure occupies less bits than there are in the byte array,
/// the packed data should be aligned to the end of the array, with the leading
/// bits being ignored. This is how the derived code reads and writes nested
/// fields that are narrower than their byte array.
///
/// 10 bits packs into: [0b00000011, 0b11111111]
pub trait PackedStruct where Self: Sized {
    /// The appropriately sized byte array into which this structure will be packed, for example [u8; 2]. 
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

    /// Pack the structure into a newly allocated vector, sized by `packed_bytes_size`.
    #[cfg(any(feature="alloc", feature="std"))]
    fn pack_to_vec(&self) -> PackingResult<Vec<u8>> {
        let size = Self::packed_bytes_size(Some(self))?;
        let mut buf = vec![0; size];
        self.pack_to_slice(&mut buf)?;
        Ok(buf)
    }
}

#[cfg_attr(feature = "use_serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
/// Packing errors that might occur during packing or unpacking
pub enum PackingError {
    /// The packed bits don't map to a valid value, for example a `bool` that
    /// isn't 0 or 1, or an undefined primitive enum discriminant.
    InvalidValue,
    /// Not returned by this crate, available for custom implementations.
    BitsError,
    /// Not returned by this crate, available for custom implementations.
    BufferTooSmall,
    /// Not returned by this crate, available for custom implementations.
    NotImplemented,
    /// The packed size of a dynamically sized type, like a `Vec`, can only
    /// be determined from an instance.
    InstanceRequiredForSize,
    /// A tuple contains more than one dynamically sized type.
    MoreThanOneDynamicType,
    /// The buffer's length doesn't match the packed size.
    BufferSizeMismatch {
        /// The required length in bytes.
        expected: usize,
        /// The length of the provided buffer.
        actual: usize
    },
    /// The buffer's length isn't a multiple of the element size.
    BufferModMismatch {
        /// The length of the provided buffer.
        actual_size: usize,
        /// The size of a single element, in bytes.
        modulo_required: usize
    },
    /// An index or range was out of the slice's bounds.
    SliceIndexingError {
        /// The length of the indexed slice.
        slice_len: usize
    },
    /// An internal invariant was violated. Indicates a bug in this crate.
    InternalError
}

impl crate::Display for PackingError {
    fn fmt(&self, f: &mut crate::fmt::Formatter) -> crate::fmt::Result {
        write!(f, "{:?}", self)
    }    
}

impl ::core::error::Error for PackingError {
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

impl From<PackingError> for crate::fmt::Error {
    fn from(_: PackingError) -> Self {
        Self
    }
}

/// The result of a packing or unpacking operation.
pub type PackingResult<T> = Result<T, PackingError>;