use internal_prelude::v1::*;

/// An enum type that can be packed or unpacked from a simple primitive integer.
pub trait PrimitiveEnum where Self: Sized + Copy {
    /// The primitve type into which we serialize and deserialize ourselves.
    type Primitive: Sized + Copy + Debug;

    /// Convert from a primitive, might fail.
    fn from_primitive(val: Self::Primitive) -> Option<Self>;
    /// Convert to a primitive value.
    fn to_primitive(&self) -> Self::Primitive;
    /// Display value, same as the name of a particular variant.
    fn to_display_str(&self) -> &'static str;
    /// Convert from a string value representing the variant. Case sensitive.
    fn from_str(s: &str) -> Option<Self>;
    /// Convert from a string value representing the variant. Lowercase.
    fn from_str_lower(s: &str) -> Option<Self>;
    /// A list all possible string variants.
    fn all_variants() -> &'static [Self];
}

