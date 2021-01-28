//! Integers that are limited by a bit width, with methods to store them
//! as a native type, packing and unpacking into byte arrays, with MSB/LSB
//! support.

use crate::internal_prelude::v1::*;
use crate::{PackingResult, lib_get_slice, lib_get_mut_slice};

use super::types_bits::*;


/// A bit-limited integer, stored in a native type that is at least
/// as many bits wide as the desired size.
#[derive(Default, Copy, Clone)]
pub struct Integer<T, B> {
    num: T,
    bits: PhantomData<B>
}

impl<T, B: NumberOfBits> Integer<T, B> {
    /// Number of bits that are to be used for signed integer's sign extension.
    fn sign_extend_bits() -> usize {
        let native_bit_count = 8 * core::mem::size_of::<T>();
        native_bit_count - B::number_of_bits()
    }
}

impl<T, B> Debug for Integer<T, B> where T: Debug {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.num)
    }
}

impl<T, B> Display for Integer<T, B> where T: Display {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.num)
    }
}

#[cfg(feature = "use_serde")]
mod serialize {
    use serde::ser::{Serialize, Serializer};
    use serde::de::{Deserialize, Deserializer};

    impl<T, B> Serialize for super::Integer<T, B> where T: Serialize {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where S: Serializer
        {
            self.num.serialize(serializer)
        }
    }

    impl<'de, T, B> Deserialize<'de> for super::Integer<T, B> where T: Deserialize<'de>, T: Into<super::Integer<T, B>> {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where D: Deserializer<'de>
        {
            <T>::deserialize(deserializer).map(|n| n.into())
        }
    }
}

impl<T, B> PartialEq for Integer<T, B> where T: PartialEq {
    fn eq(&self, other: &Self) -> bool {
        self.num.eq(&other.num)
    }
}

impl<T, B> Eq for Integer<T, B> where T: Eq {}

impl<T, B> Hash for Integer<T, B> where T: Hash {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.num.hash(state);
    }
}

impl<T, B> Integer<T, B> where Self: Copy {
    /// Convert into a MSB packing helper
    pub fn as_packed_msb(&self) -> MsbInteger<T, B, Self> {
        MsbInteger(*self, Default::default(), Default::default())
    }

    /// Convert into a LSB packing helper
    pub fn as_packed_lsb(&self) -> LsbInteger<T, B, Self> {
        LsbInteger(*self, Default::default(), Default::default())
    }
}

/// Convert an integer of a specific bit width into native types.
pub trait SizedInteger<T, B: NumberOfBits> where Self: Sized {
    /// The bit mask that is used for all incoming values. For an integer
    /// of width 8, that is 0xFF.
    fn value_bit_mask() -> T;
    /// Convert from the platform native type, applying the value mask and preserving the correct signedness.
    fn from_primitive(val: T) -> Self;
    /// Convert to the platform's native type.
    fn to_primitive(&self) -> T;
    /// Convert to a MSB byte representation. 0xAABB is converted into [0xAA, 0xBB].
    fn to_msb_bytes(&self) -> PackingResult<<<B as NumberOfBits>::Bytes as NumberOfBytes>::AsBytes>;
    /// Convert to a LSB byte representation. 0xAABB is converted into [0xBB, 0xAA].
    fn to_lsb_bytes(&self) -> PackingResult<<<B as NumberOfBits>::Bytes as NumberOfBytes>::AsBytes> where B: BitsFullBytes;
    /// Convert from a MSB byte array.
    fn from_msb_bytes(bytes: &<<B as NumberOfBits>::Bytes as NumberOfBytes>::AsBytes) -> PackingResult<Self>;
    /// Convert from a LSB byte array.
    fn from_lsb_bytes(bytes: &<<B as NumberOfBits>::Bytes as NumberOfBytes>::AsBytes) -> PackingResult<Self> where B: BitsFullBytes;
}

/// A helper for converting specific bit width signed integers into the native type.
pub trait SizedIntegerSigned<T, B> : SizedInteger<T, B>
    where B: NumberOfBits
{
    /// Sign-extends the packed value into a properly signed representation in one's complement.
    fn from_unpacked_to_signed(val: T) -> T;
}

/// Convert a native platform integer type into a byte array.
pub trait IntegerAsBytes where Self: Sized {
    /// The byte array type, for instance [u8; 2].
    type AsBytes;

    /// Convert into a MSB byte array.
    fn to_msb_bytes(&self) -> Self::AsBytes;
    /// Convert into a LSB byte array.
    fn to_lsb_bytes(&self) -> Self::AsBytes;
    /// Convert from a MSB byte array.
    fn from_msb_bytes(bytes: &Self::AsBytes) -> Self;
    /// Convert from a LSB byte array.
    fn from_lsb_bytes(bytes: &Self::AsBytes) -> Self;
}

macro_rules! as_bytes {
    (1, $v: expr) => {
        [
            (($v >> 0) as u8 & 0xFF)
        ]
    };
    (2, $v: expr) => {
        [
            (($v >> 8) as u8 & 0xFF),
            (($v >> 0) as u8 & 0xFF),
        ]
    };
    (4, $v: expr) => {
        [
            (($v >> 24) as u8 & 0xFF),
            (($v >> 16) as u8 & 0xFF),
            (($v >> 8) as u8 & 0xFF),
            (($v >> 0) as u8 & 0xFF)
        ]
    };
    (8, $v: expr) => {
        [
            (($v >> 56) as u8 & 0xFF),
            (($v >> 48) as u8 & 0xFF),
            (($v >> 40) as u8 & 0xFF),
            (($v >> 32) as u8 & 0xFF),
            (($v >> 24) as u8 & 0xFF),
            (($v >> 16) as u8 & 0xFF),
            (($v >> 8) as u8 & 0xFF),
            (($v >> 0) as u8 & 0xFF)
        ]
    }
}

macro_rules! from_bytes {
    (1, $v: expr, $T: ident) => {
        $v[0] as $T
    };
    (2, $v: expr, $T: ident) => {
        (($v[0] as $T) << 8) |
        (($v[1] as $T) << 0)
    };
    (4, $v: expr, $T: ident) => {
        (($v[0] as $T) << 24) |
        (($v[1] as $T) << 16) |
        (($v[2] as $T) << 8) |
        (($v[3] as $T) << 0)
    };
    (8, $v: expr, $T: ident) => {
        (($v[0] as $T) << 56) |
        (($v[1] as $T) << 48) |
        (($v[2] as $T) << 40) |
        (($v[3] as $T) << 32) |
        (($v[4] as $T) << 24) |
        (($v[5] as $T) << 16) |
        (($v[6] as $T) << 8) |
        (($v[7] as $T) << 0)
    };
}

macro_rules! integer_as_bytes {
    ($T: ident, $N: tt) => {
        impl IntegerAsBytes for $T {
            type AsBytes = [u8; $N];

            #[inline]
            fn to_msb_bytes(&self) -> [u8; $N] {
                as_bytes!($N, self)
            }

            #[inline]
            fn to_lsb_bytes(&self) -> [u8; $N] {
                let n = self.swap_bytes();
                as_bytes!($N, n)
            }
            
            #[inline]
            fn from_msb_bytes(bytes: &[u8; $N]) -> Self {
                from_bytes!($N, bytes, $T)
            }

            #[inline]
            fn from_lsb_bytes(bytes: &[u8; $N]) -> Self {
                let n = from_bytes!($N, bytes, $T);
                n.swap_bytes()
            }
        }
    };
}

integer_as_bytes!(u8, 1);
integer_as_bytes!(i8, 1);

integer_as_bytes!(u16, 2);
integer_as_bytes!(i16, 2);

integer_as_bytes!(u32, 4);
integer_as_bytes!(i32, 4);

integer_as_bytes!(u64, 8);
integer_as_bytes!(i64, 8);

macro_rules! integer_bytes_impl {
    ($T: ident, $TB: ident; unsigned) => {
        integer_bytes_impl!($T, $TB, unsigned);
    };
    ($T: ident, $TB: ident; signed) => {

        impl SizedIntegerSigned<$T, $TB> for Integer<$T, $TB> {
            fn from_unpacked_to_signed(val: $T) -> $T {
                let sign_extend_bits = Integer::<$T, $TB>::sign_extend_bits();
                (val << sign_extend_bits) >> sign_extend_bits
            }
        }

        integer_bytes_impl!($T, $TB, signed);
    };
    ($VAL: ident; unsigned) => {
        $VAL
    };
    ($VAL: ident; signed) => {
        Self::from_unpacked_to_signed($VAL)
    };
    ($T: ident, $TB: ident, $SIGN: tt) => {
        impl SizedInteger<$T, $TB> for Integer<$T, $TB> {
            #[inline]
            fn value_bit_mask() -> $T {
                ones($TB::number_of_bits() as u64) as $T
            }

            #[inline]
            fn from_primitive(val: $T) -> Self {
                let v = val & Self::value_bit_mask();
                let v = integer_bytes_impl!(v; $SIGN);

                Integer { num: v, bits: Default::default() }
            }

            #[inline]
            fn to_primitive(&self) -> $T {
                self.num
            }

            #[inline]
            fn to_msb_bytes(&self) -> PackingResult<<<$TB as NumberOfBits>::Bytes as NumberOfBytes>::AsBytes>
            {
                let mut ret: <<$TB as NumberOfBits>::Bytes as NumberOfBytes>::AsBytes = Default::default();
                let b = self.num.to_msb_bytes();
                let skip = b.len() - ret.len();
                let b = lib_get_slice(&b, skip..)?;
                ret.copy_from_slice(b);
                Ok(ret)
            }

            #[inline]
            fn to_lsb_bytes(&self) -> PackingResult<<<$TB as NumberOfBits>::Bytes as NumberOfBytes>::AsBytes>
            {
                let mut ret: <<$TB as NumberOfBits>::Bytes as NumberOfBytes>::AsBytes = Default::default();
                let b = self.num.to_lsb_bytes();
                let take = ret.len();
                let b = lib_get_slice(&b, 0..take)?;
                ret.copy_from_slice(b);
                Ok(ret)
            }

            #[inline]
            fn from_msb_bytes(bytes: &<<$TB as NumberOfBits>::Bytes as NumberOfBytes>::AsBytes) -> PackingResult<Self>
            {
                let mut native_bytes = Default::default();
                {
                    // hack that infers the size of the native array...
                    <$T>::from_msb_bytes(&native_bytes);
                }
                let skip = native_bytes.len() - bytes.len();
                {
                    let native_bytes = lib_get_mut_slice(&mut native_bytes, skip..)?;
                    native_bytes.copy_from_slice(&bytes[..]);
                }
                let v = <$T>::from_msb_bytes(&native_bytes);
                Ok(Self::from_primitive(v))
            }

            #[inline]
            fn from_lsb_bytes(bytes: &<<$TB as NumberOfBits>::Bytes as NumberOfBytes>::AsBytes) -> PackingResult<Self>
            {
                let mut native_bytes = Default::default();
                {
                    // hack that infers the size of the native array...
                    <$T>::from_lsb_bytes(&native_bytes);
                }

                {
                    let take = bytes.len();
                    let native_bytes = lib_get_mut_slice(&mut native_bytes, ..take)?;
                    native_bytes.copy_from_slice(&bytes[..]);
                }

                let v = <$T>::from_lsb_bytes(&native_bytes);
                Ok(Self::from_primitive(v))
            }
        }

        impl From<$T> for Integer<$T, $TB> {
            fn from(v: $T) -> Self {
                Self::from_primitive(v)
            }
        }

        impl From<Integer<$T, $TB>> for $T {
            fn from(v: Integer<$T, $TB>) -> Self {
                v.to_primitive()
            }
        }

        impl Deref for Integer<$T, $TB> {
            type Target = $T;
            
            fn deref(&self) -> &$T {
                &self.num
            }
        }
    };
}

macro_rules! bytes1_impl {
    ($T: ident, $IS_SIGNED: tt) => {
        integer_bytes_impl!($T, Bits1; $IS_SIGNED);
        integer_bytes_impl!($T, Bits2; $IS_SIGNED);
        integer_bytes_impl!($T, Bits3; $IS_SIGNED);
        integer_bytes_impl!($T, Bits4; $IS_SIGNED);
        integer_bytes_impl!($T, Bits5; $IS_SIGNED);
        integer_bytes_impl!($T, Bits6; $IS_SIGNED);
        integer_bytes_impl!($T, Bits7; $IS_SIGNED);
        integer_bytes_impl!($T, Bits8; $IS_SIGNED);
    };
}

macro_rules! bytes2_impl {
    ($T: ident, $IS_SIGNED: tt) => {
        integer_bytes_impl!($T, Bits9; $IS_SIGNED);
        integer_bytes_impl!($T, Bits10; $IS_SIGNED);
        integer_bytes_impl!($T, Bits11; $IS_SIGNED);
        integer_bytes_impl!($T, Bits12; $IS_SIGNED);
        integer_bytes_impl!($T, Bits13; $IS_SIGNED);
        integer_bytes_impl!($T, Bits14; $IS_SIGNED);
        integer_bytes_impl!($T, Bits15; $IS_SIGNED);
        integer_bytes_impl!($T, Bits16; $IS_SIGNED);
    };
}

macro_rules! bytes3_impl {
    ($T: ident, $IS_SIGNED: tt) => {
        integer_bytes_impl!($T, Bits17; $IS_SIGNED);
        integer_bytes_impl!($T, Bits18; $IS_SIGNED);
        integer_bytes_impl!($T, Bits19; $IS_SIGNED);
        integer_bytes_impl!($T, Bits20; $IS_SIGNED);
        integer_bytes_impl!($T, Bits21; $IS_SIGNED);
        integer_bytes_impl!($T, Bits22; $IS_SIGNED);
        integer_bytes_impl!($T, Bits23; $IS_SIGNED);
        integer_bytes_impl!($T, Bits24; $IS_SIGNED);
    };
}

macro_rules! bytes4_impl {
    ($T: ident, $IS_SIGNED: tt) => {
        integer_bytes_impl!($T, Bits25; $IS_SIGNED);
        integer_bytes_impl!($T, Bits26; $IS_SIGNED);
        integer_bytes_impl!($T, Bits27; $IS_SIGNED);
        integer_bytes_impl!($T, Bits28; $IS_SIGNED);
        integer_bytes_impl!($T, Bits29; $IS_SIGNED);
        integer_bytes_impl!($T, Bits30; $IS_SIGNED);
        integer_bytes_impl!($T, Bits31; $IS_SIGNED);
        integer_bytes_impl!($T, Bits32; $IS_SIGNED);
    };
}

macro_rules! bytes5_impl {
    ($T: ident, $IS_SIGNED: tt) => {
        integer_bytes_impl!($T, Bits33; $IS_SIGNED);
        integer_bytes_impl!($T, Bits34; $IS_SIGNED);
        integer_bytes_impl!($T, Bits35; $IS_SIGNED);
        integer_bytes_impl!($T, Bits36; $IS_SIGNED);
        integer_bytes_impl!($T, Bits37; $IS_SIGNED);
        integer_bytes_impl!($T, Bits38; $IS_SIGNED);
        integer_bytes_impl!($T, Bits39; $IS_SIGNED);
        integer_bytes_impl!($T, Bits40; $IS_SIGNED);
    };
}

macro_rules! bytes6_impl {
    ($T: ident, $IS_SIGNED: tt) => {
        integer_bytes_impl!($T, Bits41; $IS_SIGNED);
        integer_bytes_impl!($T, Bits42; $IS_SIGNED);
        integer_bytes_impl!($T, Bits43; $IS_SIGNED);
        integer_bytes_impl!($T, Bits44; $IS_SIGNED);
        integer_bytes_impl!($T, Bits45; $IS_SIGNED);
        integer_bytes_impl!($T, Bits46; $IS_SIGNED);
        integer_bytes_impl!($T, Bits47; $IS_SIGNED);
        integer_bytes_impl!($T, Bits48; $IS_SIGNED);
    };
}

macro_rules! bytes7_impl {
    ($T: ident, $IS_SIGNED: tt) => {
        integer_bytes_impl!($T, Bits49; $IS_SIGNED);
        integer_bytes_impl!($T, Bits50; $IS_SIGNED);
        integer_bytes_impl!($T, Bits51; $IS_SIGNED);
        integer_bytes_impl!($T, Bits52; $IS_SIGNED);
        integer_bytes_impl!($T, Bits53; $IS_SIGNED);
        integer_bytes_impl!($T, Bits54; $IS_SIGNED);
        integer_bytes_impl!($T, Bits55; $IS_SIGNED);
        integer_bytes_impl!($T, Bits56; $IS_SIGNED);
    };
}

macro_rules! bytes8_impl {
    ($T: ident, $IS_SIGNED: tt) => {
        integer_bytes_impl!($T, Bits57; $IS_SIGNED);
        integer_bytes_impl!($T, Bits58; $IS_SIGNED);
        integer_bytes_impl!($T, Bits59; $IS_SIGNED);
        integer_bytes_impl!($T, Bits60; $IS_SIGNED);
        integer_bytes_impl!($T, Bits61; $IS_SIGNED);
        integer_bytes_impl!($T, Bits62; $IS_SIGNED);
        integer_bytes_impl!($T, Bits63; $IS_SIGNED);
        integer_bytes_impl!($T, Bits64; $IS_SIGNED);
    };
}

bytes1_impl!(u8, unsigned);
bytes1_impl!(i8, signed);

bytes2_impl!(u16, unsigned);
bytes2_impl!(i16, signed);

bytes3_impl!(u32, unsigned);
bytes3_impl!(i32, signed);

bytes4_impl!(u32, unsigned);
bytes4_impl!(i32, signed);

bytes5_impl!(u64, unsigned);
bytes5_impl!(i64, signed);

bytes6_impl!(u64, unsigned);
bytes6_impl!(i64, signed);

bytes7_impl!(u64, unsigned);
bytes7_impl!(i64, signed);

bytes8_impl!(u64, unsigned);
bytes8_impl!(i64, signed);

/// A positive bit mask of the desired width.
/// 
/// ones(1) => 0b1
/// ones(2) => 0b11
/// ones(3) => 0b111
/// ...
const fn ones(n: u64) -> u64 {
	if n == 0 { return 0; }
	if n >= 64 { return !0; }

	(1 << n) - 1
}

#[test]
fn test_u8() {
    let byte: Integer<u8, Bits8> = 0.into();
    assert_eq!(0, *byte);
    assert_eq!(0xFF, <Integer<u8, Bits8>>::value_bit_mask());
}

#[test]
fn test_u16() {
    let val = 0xABCD;
    let num: Integer<u16, Bits16> = val.into();
    assert_eq!(val, *num);
    assert_eq!([0xAB, 0xCD], num.to_msb_bytes().unwrap());
    assert_eq!([0xCD, 0xAB], num.to_lsb_bytes().unwrap());
}

#[test]
fn test_u32() {
    let val = 0x4589ABCD;
    let num: Integer<u32, Bits32> = val.into();
    assert_eq!(val, *num);
    assert_eq!([0x45, 0x89, 0xAB, 0xCD], num.to_msb_bytes().unwrap());
    assert_eq!([0xCD, 0xAB, 0x89, 0x45], num.to_lsb_bytes().unwrap());
}

#[test]
fn test_u64() {
    let val = 0x1122334455667788;
    let num: Integer<u64, Bits64> = val.into();
    assert_eq!(val, *num);
    assert_eq!([0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88], num.to_msb_bytes().unwrap());
    assert_eq!([0x88, 0x77, 0x66, 0x55, 0x44, 0x33, 0x22, 0x11], num.to_lsb_bytes().unwrap());
}

#[test]
fn test_roundtrip_u32() {
    let val = 0x11223344;
    let num: Integer<u32, Bits32> = val.into();
    let msb_bytes = num.to_msb_bytes().unwrap();
    let from_msb = u32::from_msb_bytes(&msb_bytes);
    assert_eq!(val, from_msb);

    let lsb_bytes = num.to_lsb_bytes().unwrap();
    let from_lsb = u32::from_lsb_bytes(&lsb_bytes);
    assert_eq!(val, from_lsb);
}

#[test]
fn test_roundtrip_u24() {
    let val = 0xCCBBAA;
    let num: Integer<u32, Bits24> = val.into();
    let msb_bytes = num.to_msb_bytes().unwrap();
    assert_eq!([0xCC, 0xBB, 0xAA], msb_bytes);
    let from_msb = <Integer<u32, Bits24>>::from_msb_bytes(&msb_bytes).unwrap();
    assert_eq!(val, *from_msb);

    let lsb_bytes = num.to_lsb_bytes().unwrap();
    assert_eq!([0xAA, 0xBB, 0xCC], lsb_bytes);
    let from_lsb = <Integer<u32, Bits24>>::from_lsb_bytes(&lsb_bytes).unwrap();
    assert_eq!(val, *from_lsb);
}

#[test]
fn test_roundtrip_u20() {
    let val = 0xFBBAA;
    let num: Integer<u32, Bits20> = val.into();
    let msb_bytes = num.to_msb_bytes().unwrap();
    assert_eq!([0x0F, 0xBB, 0xAA], msb_bytes);
    let from_msb = <Integer<u32, Bits20>>::from_msb_bytes(&msb_bytes).unwrap();
    assert_eq!(val, *from_msb);    
}


use super::packing::{PackingError, PackedStruct, PackedStructInfo};

/// A wrapper that packages the integer as a MSB packaged byte array. Usually
/// invoked using code generation.
pub struct MsbInteger<T, B, I>(I, PhantomData<T>, PhantomData<B>);
impl<T, B, I> Deref for MsbInteger<T, B, I> {
    type Target = I;

    fn deref(&self) -> &I {
        &self.0
    }
}
impl<T, B, I> From<I> for MsbInteger<T, B, I> {
    fn from(i: I) -> Self {
        MsbInteger(i, Default::default(), Default::default())
    }
}

impl<T, B, I> Debug for MsbInteger<T, B, I> where I: Debug {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

impl<T, B, I> Display for MsbInteger<T, B, I> where I: Display {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<T, B, I> PackedStruct for MsbInteger<T, B, I>
    where B: NumberOfBits, I: SizedInteger<T, B>
{
    type ByteArray = <<B as NumberOfBits>::Bytes as NumberOfBytes>::AsBytes;

    fn pack(&self) -> PackingResult<<<B as NumberOfBits>::Bytes as NumberOfBytes>::AsBytes> {
        self.0.to_msb_bytes()
    }

    #[inline]
    fn unpack(src: &<<B as NumberOfBits>::Bytes as NumberOfBytes>::AsBytes) -> Result<Self, PackingError> {
        let n = I::from_msb_bytes(src)?;
        let n = MsbInteger(n, Default::default(), Default::default());
        Ok(n)
    }
}

impl<T, B, I> PackedStructInfo for MsbInteger<T, B, I> where B: NumberOfBits {
    #[inline]
    fn packed_bits() -> usize {
        B::number_of_bits() as usize
    }
}


/// A wrapper that packages the integer as a LSB packaged byte array. Usually
/// invoked using code generation.
pub struct LsbInteger<T, B, I>(I, PhantomData<T>, PhantomData<B>);
impl<T, B, I> Deref for LsbInteger<T, B, I> where B: BitsFullBytes {
    type Target = I;

    fn deref(&self) -> &I {
        &self.0
    }
}
impl<T, B, I> From<I> for LsbInteger<T, B, I> where B: BitsFullBytes {
    fn from(i: I) -> Self {
        LsbInteger(i, Default::default(), Default::default())
    }
}

impl<T, B, I> Debug for LsbInteger<T, B, I> where I: Debug {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

impl<T, B, I> Display for LsbInteger<T, B, I> where I: Display {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<T, B, I> PackedStruct for LsbInteger<T, B, I>
    where B: NumberOfBits, I: SizedInteger<T, B>, B: BitsFullBytes
{
    type ByteArray = <<B as NumberOfBits>::Bytes as NumberOfBytes>::AsBytes;

    fn pack(&self) -> PackingResult<<<B as NumberOfBits>::Bytes as NumberOfBytes>::AsBytes> {
        self.0.to_lsb_bytes()
    }

    #[inline]
    fn unpack(src: &<<B as NumberOfBits>::Bytes as NumberOfBytes>::AsBytes) -> PackingResult<Self> {
        let n = I::from_lsb_bytes(src)?;
        let n = LsbInteger(n, Default::default(), Default::default());
        Ok(n)
    }
}

impl<T, B, I> PackedStructInfo for LsbInteger<T, B, I> where B: NumberOfBits {
    #[inline]
    fn packed_bits() -> usize {
        B::number_of_bits() as usize
    }
}


#[test]
fn test_packed_int_msb() {
    let val = 0xAABBCCDD;
    let typed: Integer<u32, Bits32> = val.into();
    let endian = typed.as_packed_msb();
    let packed = endian.pack().unwrap();
    assert_eq!([0xAA, 0xBB, 0xCC, 0xDD], packed);
    
    let unpacked: MsbInteger<_, _, Integer<u32, Bits32>> = MsbInteger::unpack(&packed).unwrap();
    assert_eq!(val, **unpacked);
}

#[test]
fn test_packed_int_partial() {
    let val = 0b10_10101010;
    let typed: Integer<u16, Bits10> = val.into();
    let endian = typed.as_packed_msb();
    let packed = endian.pack().unwrap();
    assert_eq!([0b00000010, 0b10101010], packed);
    
    let unpacked: MsbInteger<_, _, Integer<u16, Bits10>> = MsbInteger::unpack(&packed).unwrap();
    assert_eq!(val, **unpacked);
}

#[test]
fn test_packed_int_lsb() {
    let val = 0xAABBCCDD;
    let typed: Integer<u32, Bits32> = val.into();
    let endian = typed.as_packed_lsb();
    let packed = endian.pack().unwrap();
    assert_eq!([0xDD, 0xCC, 0xBB, 0xAA], packed);
    
    let unpacked: LsbInteger<_, _, Integer<u32, Bits32>> = LsbInteger::unpack(&packed).unwrap();
    assert_eq!(val, **unpacked);
}

#[test]
fn test_struct_info() {
    fn get_bits<P: PackedStructInfo>(_s: &P) -> usize { P::packed_bits() }

    let typed: Integer<u32, Bits30> = 123.into();
    let msb = typed.as_packed_msb();
    assert_eq!(30, get_bits(&msb));
}

#[test]
fn test_slice_packing() {
    use crate::packing::PackedStructSlice;

    let mut data = vec![0xAA, 0xBB, 0xCC, 0xDD];
    let unpacked = <MsbInteger<_, _, Integer<u32, Bits32>>>::unpack_from_slice(&data).unwrap();
    assert_eq!(0xAABBCCDD, **unpacked);

    unpacked.pack_to_slice(&mut data).unwrap();
    assert_eq!(&[0xAA, 0xBB, 0xCC, 0xDD], &data[..]);
}

#[test]
fn test_packed_int_lsb_sub() {
    let val = 0xAABBCC;
    let typed: Integer<u32, Bits24> = val.into();
    let endian = typed.as_packed_lsb();
    let packed = endian.pack().unwrap();
    assert_eq!([0xCC, 0xBB, 0xAA], packed);
}

#[test]
fn test_big_slice_unpacking() {
    use crate::packing::PackedStructSlice;
    
    let data = vec![0xAA, 0xBB, 0xCC, 0xDD];
    let unpacked = <MsbInteger<_, _, Integer<u32, Bits32>>>::unpack_from_slice(&data).unwrap();
    assert_eq!(0xAABBCCDD, **unpacked);
}

/// test if the value is properly first masked and then expanded for signedness
#[test]
fn test_sign_extension() {
    let val: Integer<i8, Bits4> = (127 as i8).into();
    assert_eq!(*val, -1);
    let val: Integer<i8, Bits4> = (63 as i8).into();
    assert_eq!(*val, -1);
    let val: Integer<i8, Bits4> = (31 as i8).into();
    assert_eq!(*val, -1);
    let val: Integer<i8, Bits4> = (15 as i8).into();
    assert_eq!(*val, -1);
}

/// test if sign extension conversion properly handles min and max values
#[test]
fn test_sign_extension_limits() {
    for i in -8..=7 {
        let val: Integer<i8, Bits4> = (i as i8).into();
        assert_eq!(i, *val);
    }

    for i in -64..=63 {
        let val: Integer<i8, Bits7> = (i as i8).into();
        assert_eq!(i, *val);
    }

    let val: Integer<i8, Bits8> = (i8::MIN).into();
    assert_eq!(*val, i8::MIN);
    let val: Integer<i8, Bits8> = (i8::MAX).into();
    assert_eq!(*val, i8::MAX);

    let val: Integer<i16, Bits16> = (i16::MIN).into();
    assert_eq!(*val, i16::MIN);
    let val: Integer<i16, Bits16> = (i16::MAX).into();
    assert_eq!(*val, i16::MAX);

    let val: Integer<i32, Bits32> = (i32::MIN).into();
    assert_eq!(*val, i32::MIN);
    let val: Integer<i32, Bits32> = (i32::MAX).into();
    assert_eq!(*val, i32::MAX);

    let val: Integer<i64, Bits64> = (i64::MIN).into();
    assert_eq!(*val, i64::MIN);
    let val: Integer<i64, Bits64> = (i64::MAX).into();
    assert_eq!(*val, i64::MAX);
}