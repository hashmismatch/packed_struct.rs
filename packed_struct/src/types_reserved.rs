//! Reserved space in a packed structure, either just zeroes or ones.

use internal_prelude::v1::*;

/// Packs into a set of zeroes. Ignores the input when unpacking.
pub type ReservedZero<B> = ReservedBits<BitZero, B>;
pub type ReservedZeroes<B> = ReservedZero<B>;

/// Packs into a set of ones. Ignores the input when unpacking.
pub type ReservedOne<B> = ReservedBits<BitOne, B>;
pub type ReservedOnes<B> = ReservedOne<B>;

pub trait ReservedBitValue {
    fn get_reserved_bit_value_byte() -> u8;
}

#[derive(Default, Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BitOne;
impl ReservedBitValue for BitOne {
    fn get_reserved_bit_value_byte() -> u8 {
        0xFF
    }
}

#[derive(Default, Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BitZero;
impl ReservedBitValue for BitZero {
    fn get_reserved_bit_value_byte() -> u8 {
        0
    }
}

/// Always packs into the associated bit value. Ignores the input when unpacking.
#[derive(Default, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReservedBits<V, B> {
    value: V,
    bits: PhantomData<B>
}

impl<B> Debug for ReservedBits<BitZero, B> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Reserved - always 0")
    }
}

impl<B> Display for ReservedBits<BitZero, B> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Reserved - always 0")
    }
}

impl<B> Debug for ReservedBits<BitOne, B> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Reserved - always 1")
    }
}

impl<B> Display for ReservedBits<BitOne, B> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Reserved - always 1")
    }
}



use packing::*;
use types_bits::{NumberOfBits, NumberOfBytes, ByteArray};

impl<V, B> PackedStruct for ReservedBits<V, B> where Self: Default, V: ReservedBitValue, B: NumberOfBits {
    type ByteArray = <<B as NumberOfBits>::Bytes as NumberOfBytes>::AsBytes;
    fn pack(&self) -> <<B as NumberOfBits>::Bytes as NumberOfBytes>::AsBytes {
        <<<B as NumberOfBits>::Bytes as NumberOfBytes>::AsBytes>::new(V::get_reserved_bit_value_byte())
    }

    fn unpack(_src: &<<B as NumberOfBits>::Bytes as NumberOfBytes>::AsBytes) -> Result<Self, PackingError> {
        Ok(Self:: default())
    }
}

impl<V, B> PackedStructInfo for ReservedBits<V, B> where B: NumberOfBits {
    #[inline]
    fn packed_bits() -> usize {
        B::number_of_bits() as usize
    }
}