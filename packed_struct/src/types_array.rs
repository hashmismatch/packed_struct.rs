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

/// Builds an array by calling `f` with each index, stopping at the first error.
///
/// Used by the derive macro to unpack array fields with a loop, instead of
/// unrolling the code for every element.
#[inline]
pub fn try_array_from_fn<T, F, const N: usize>(mut f: F) -> Result<[T; N], PackingError>
    where F: FnMut(usize) -> Result<T, PackingError>
{
    let mut error = None;
    let elements: [Option<T>; N] = core::array::from_fn(|i| {
        if error.is_some() {
            return None;
        }

        match f(i) {
            Ok(element) => Some(element),
            Err(e) => {
                error = Some(e);
                None
            }
        }
    });

    if let Some(e) = error {
        return Err(e);
    }

    Ok(elements.map(|element| element.expect("all the elements were unpacked")))
}
