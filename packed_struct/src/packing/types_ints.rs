use prelude::v1::*;

use super::packing::*;

/// The Most Significant Byte is the first one, unsigned.
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct MsbU16(pub u16);

impl Deref for MsbU16 {
    type Target = u16;

    #[inline]
    fn deref(&self) -> &u16 {
        &self.0
    }
}

impl PackedStruct<[u8; 2]> for MsbU16 {
    #[inline]
    fn pack(&self) -> [u8; 2] {
        [
            ((self.0 & 0xFF00) >> 8) as u8,
            (self.0 & 0x00FF) as u8
        ]
    }

    #[inline]
    fn unpack(src: &[u8; 2]) -> Result<MsbU16, PackingError> {
        Ok(MsbU16(((src[0] as u16) << 8) | src[1] as u16))
    }
}

impl PackedStructInfo for MsbU16 {
    #[inline]
    fn packed_bits() -> usize {
        16
    }
}

#[cfg(any(feature="core_collections", feature="std"))]
impl Display for MsbU16 {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        write!(f, "{}", self.0)
    }
}

packing_slice!(MsbU16; 2);

/// The Most Significant Byte is the first one, signed.
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct MsbI16(pub i16);

impl Deref for MsbI16 {
    type Target = i16;

    #[inline]
    fn deref(&self) -> &i16 {
        &self.0
    }
}

impl PackedStruct<[u8; 2]> for MsbI16 {
    #[inline]
    fn pack(&self) -> [u8; 2] {
        [
            (((self.0 as u16) & 0xFF00) >> 8) as u8,
            (self.0 & 0x00FF) as u8
        ]
    }

    #[inline]
    fn unpack(src: &[u8; 2]) -> Result<MsbI16, PackingError> {
        Ok(MsbI16(((src[0] as i16) << 8) | src[1] as i16))
    }
}

impl PackedStructInfo for MsbI16 {
    #[inline]
    fn packed_bits() -> usize {
        16
    }
}

packing_slice!(MsbI16; 2);

#[cfg(any(feature="core_collections", feature="std"))]
impl Display for MsbI16 {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        write!(f, "{}", self.0)
    }
}

/// The Least Significant Byte is the first one, unsigned.
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct LsbU16(pub u16);

impl Deref for LsbU16 {
    type Target = u16;
    #[inline]
    fn deref(&self) -> &u16 {
        &self.0
    }
}

impl PackedStruct<[u8; 2]> for LsbU16 {
    #[inline]
    fn pack(&self) -> [u8; 2] {
        [
            (self.0 & 0x00FF) as u8,
            ((self.0 & 0xFF00) >> 8) as u8
        ]
    }

    #[inline]
    fn unpack(src: &[u8; 2]) -> Result<LsbU16, PackingError> {
        Ok(LsbU16(((src[1] as u16) << 8) | src[0] as u16))
    }
}

impl PackedStructInfo for LsbU16 {
    #[inline]
    fn packed_bits() -> usize {
        16
    }
}

packing_slice!(LsbU16; 2);

#[cfg(any(feature="core_collections", feature="std"))]
impl Display for LsbU16 {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        write!(f, "{}", self.0)
    }
}

/// The Least Significant Byte is the first one, signed.
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct LsbI16(pub i16);

impl Deref for LsbI16 {
    type Target = i16;
    #[inline]
    fn deref(&self) -> &i16 {
        &self.0
    }
}

impl PackedStruct<[u8; 2]> for LsbI16 {
    #[inline]
    fn pack(&self) -> [u8; 2] {
        [
            (self.0 & 0x00FF) as u8,
            ((self.0 as u16 & 0xFF00) >> 8) as u8
        ]
    }

    #[inline]
    fn unpack(src: &[u8; 2]) -> Result<LsbI16, PackingError> {
        Ok(LsbI16(((src[1] as i16) << 8) | src[0] as i16))
    }
}

impl PackedStructInfo for LsbI16 {
    #[inline]
    fn packed_bits() -> usize {
        16
    }
}

packing_slice!(LsbI16; 2);

#[cfg(any(feature="core_collections", feature="std"))]
impl Display for LsbI16 {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        write!(f, "{}", self.0)
    }
}


/// The Least Significant Byte is the first one, unsigned.
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct LsbU32(pub u32);

impl Deref for LsbU32 {
    type Target = u32;
    #[inline]
    fn deref(&self) -> &u32 {
        &self.0
    }
}

impl PackedStruct<[u8; 4]> for LsbU32 {
    #[inline]
    fn pack(&self) -> [u8; 4] {
        [
            (self.0  & 0x000000FF) as u8,
            ((self.0 & 0x0000FF00) >> 8) as u8,
            ((self.0 & 0x00FF0000) >> 16) as u8,
            ((self.0 & 0xFF000000) >> 24) as u8
        ]
    }

    #[inline]
    fn unpack(src: &[u8; 4]) -> Result<LsbU32, PackingError> {
        Ok(LsbU32(
            src[0] as u32 |
            ((src[1] as u32) << 8) |
            ((src[2] as u32) << 16) |
            ((src[3] as u32) << 24)
        ))
    }
}

impl PackedStructInfo for LsbU32 {
    #[inline]
    fn packed_bits() -> usize {
        32
    }
}

packing_slice!(LsbU32; 4);

#[cfg(any(feature="core_collections", feature="std"))]
impl Display for LsbU32 {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        write!(f, "{}", self.0)
    }
}

/// The Least Significant Byte is the first one, signed.
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct LsbI32(pub i32);

impl Deref for LsbI32 {
    type Target = i32;
    #[inline]
    fn deref(&self) -> &i32 {
        &self.0
    }
}

impl PackedStruct<[u8; 4]> for LsbI32 {
    #[inline]
    fn pack(&self) -> [u8; 4] {
        [
            (self.0  & 0x000000FF) as u8,
            ((self.0 & 0x0000FF00) >> 8) as u8,
            ((self.0 & 0x00FF0000) >> 16) as u8,
            ((self.0 as u32 & 0xFF000000) >> 24) as u8
        ]
    }

    #[inline]
    fn unpack(src: &[u8; 4]) -> Result<LsbI32, PackingError> {
        Ok(LsbI32(
            src[0] as i32 |
            ((src[1] as i32) << 8) |
            ((src[2] as i32) << 16) |
            ((src[3] as i32) << 24)
        ))
    }    
}

impl PackedStructInfo for LsbI32 {
    #[inline]
    fn packed_bits() -> usize {
        32
    }
}

packing_slice!(LsbI32; 4);

#[cfg(any(feature="core_collections", feature="std"))]
impl Display for LsbI32 {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        write!(f, "{}", self.0)
    }
}


/// The Most Significant Byte is the first one, unsigned.
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct MsbU32(pub u32);

impl Deref for MsbU32 {
    type Target = u32;
    #[inline]
    fn deref(&self) -> &u32 {
        &self.0
    }
}

impl PackedStruct<[u8; 4]> for MsbU32 {
    #[inline]
    fn pack(&self) -> [u8; 4] {
        [
            ((self.0 & 0xFF000000) >> 24) as u8,
            ((self.0 & 0x00FF0000) >> 16) as u8,
            ((self.0 & 0x0000FF00) >> 8) as u8,
            (self.0  & 0x000000FF) as u8,
        ]
    }

    #[inline]
    fn unpack(src: &[u8; 4]) -> Result<MsbU32, PackingError> {
        Ok(MsbU32(
            ((src[0] as u32) << 24) |
            ((src[1] as u32) << 16) | 
            ((src[2] as u32) << 8) |
            (src[3] as u32)
        ))
    }
}

impl PackedStructInfo for MsbU32 {
    #[inline]
    fn packed_bits() -> usize {
        32
    }
}

packing_slice!(MsbU32; 4);

#[cfg(any(feature="core_collections", feature="std"))]
impl Display for MsbU32 {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        write!(f, "{}", self.0)
    }
}





/// The Most Significant Byte is the first one, unsigned.
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct MsbU64(pub u64);

impl Deref for MsbU64 {
    type Target = u64;
    #[inline]
    fn deref(&self) -> &u64 {
        &self.0
    }
}

impl PackedStruct<[u8; 8]> for MsbU64 {
    #[inline]
    fn pack(&self) -> [u8; 8] {
        [
            ((self.0 & 0xFF00000000000000) >> 56) as u8,
            ((self.0 & 0x00FF000000000000) >> 48) as u8,
            ((self.0 & 0x0000FF0000000000) >> 40) as u8,
            ((self.0 & 0x000000FF00000000) >> 32) as u8,
            ((self.0 & 0x00000000FF000000) >> 24) as u8,
            ((self.0 & 0x0000000000FF0000) >> 16) as u8,
            ((self.0 & 0x000000000000FF00) >> 8) as u8,
            (self.0  & 0x00000000000000FF) as u8,
        ]
    }

    #[inline]
    fn unpack(src: &[u8; 8]) -> Result<MsbU64, PackingError> {
        Ok(MsbU64(
            ((src[0] as u64) << 56) |
            ((src[1] as u64) << 48) | 
            ((src[2] as u64) << 40) |
            ((src[3] as u64) << 32) |
            ((src[4] as u64) << 24) |
            ((src[5] as u64) << 16) | 
            ((src[6] as u64) << 8) |
            (src[7] as u64)
        ))
    }
}

impl PackedStructInfo for MsbU64 {
    #[inline]
    fn packed_bits() -> usize {
        64
    }
}

packing_slice!(MsbU64; 8);

#[cfg(any(feature="core_collections", feature="std"))]
impl Display for MsbU64 {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        write!(f, "{}", self.0)
    }
}




/// The Most Significant Byte is the first one, unsigned.
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct MsbU24(pub u32);

impl Deref for MsbU24 {
    type Target = u32;
    #[inline]
    fn deref(&self) -> &u32 {
        &self.0
    }
}

impl PackedStruct<[u8; 3]> for MsbU24 {
    #[inline]
    fn pack(&self) -> [u8; 3] {
        [
            ((self.0 & 0x00FF0000) >> 16) as u8,
            ((self.0 & 0x0000FF00) >> 8) as u8,
            (self.0  & 0x000000FF) as u8,
        ]
    }

    #[inline]
    fn unpack(src: &[u8; 3]) -> Result<MsbU24, PackingError> {
        Ok(MsbU24(
            ((src[0] as u32) << 16) | 
            ((src[1] as u32) << 8) |
            (src[2] as u32)
        ))
    }
}

impl PackedStructInfo for MsbU24 {
    #[inline]
    fn packed_bits() -> usize {
        24
    }
}

packing_slice!(MsbU24; 3);

#[cfg(any(feature="core_collections", feature="std"))]
impl Display for MsbU24 {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        write!(f, "{}", self.0)
    }
}



/// The Most Significant Byte is the first one, signed.
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct MsbI32(pub i32);

impl Deref for MsbI32 {
    type Target = i32;
    #[inline]
    fn deref(&self) -> &i32 {
        &self.0
    }
}

impl PackedStruct<[u8; 4]> for MsbI32 {
    #[inline]
    fn pack(&self) -> [u8; 4] {
        [
            ((self.0 as u32 & 0xFF000000) >> 24) as u8,
            ((self.0 & 0x00FF0000) >> 16) as u8,
            ((self.0 & 0x0000FF00) >> 8) as u8,
            (self.0  & 0x000000FF) as u8,
        ]
    }

    #[inline]
    fn unpack(src: &[u8; 4]) -> Result<MsbI32, PackingError> {
        Ok(MsbI32(
            ((src[0] as i32) << 24) | 
            ((src[1] as i32) << 16) | 
            ((src[2] as i32) << 8) |
            (src[3] as i32)
        ))
    }    
}

impl PackedStructInfo for MsbI32 {
    #[inline]
    fn packed_bits() -> usize {
        32
    }
}

packing_slice!(MsbI32; 4);

#[cfg(any(feature="core_collections", feature="std"))]
impl Display for MsbI32 {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        write!(f, "{}", self.0)
    }
}


