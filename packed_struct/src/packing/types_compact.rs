use prelude::v1::*;

use super::packing::*;

macro_rules! packed_compact_u8_bits {
    ($N: expr, $T: ident) => (

        #[derive(Debug, Copy, Clone, PartialEq)]
        pub struct $T(u8);
        
        impl PackedStruct<[u8; 1]> for $T {
            #[inline]
            fn pack(&self) -> [u8; 1] {
                [self.0 << (8-$N)]
            }

            #[inline]
            fn unpack(src: &[u8; 1]) -> Result<Self, PackingError> {
                let v = src[0] >> (8-$N);
                let b = $T(v);
                Ok(b)
            }
        }

        impl PackedStructInfo for $T {
            #[inline]
            fn packed_bits() -> usize {
                $N
            }
        }

        packing_slice!($T; 1);

        impl Deref for $T {
            type Target = u8;
            #[inline]
            fn deref(&self) -> &u8 {
                &self.0
            }
        }

        impl From<u8> for $T {
            #[inline]
            fn from(v: u8) -> Self {
                // todo: bit mask the value?
                $T(v)
            }
        }

        impl From<$T> for u8 {
            #[inline]
            fn from(v: $T) -> Self {
                v.0
            }
        }

    )
}

packed_compact_u8_bits!(1, UIntBits1);
packed_compact_u8_bits!(2, UIntBits2);
packed_compact_u8_bits!(3, UIntBits3);
packed_compact_u8_bits!(4, UIntBits4);
packed_compact_u8_bits!(5, UIntBits5);
packed_compact_u8_bits!(6, UIntBits6);
packed_compact_u8_bits!(7, UIntBits7);


#[test]
fn test_conv() {
    let c: UIntBits1 = 1.into();
    let b: u8 = *c;

    assert_eq!(b, 1);
}
