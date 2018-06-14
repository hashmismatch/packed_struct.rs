use internal_prelude::v1::*;

/// An enum type that can be packed or unpacked from a simple primitive integer.
pub trait PrimitiveEnum<T> where T: Sized + Copy + Debug, Self: Sized + Copy {
    /// Convert from a primitive, might fail.
    fn from_primitive(val: T) -> Option<Self>;
    /// Convert to a primitive value.
    fn to_primitive(&self) -> T;
    /// Display value, same as the name of a particular variant.
    fn to_display_str(&self) -> Cow<'static, str>;
    /// Convert from a string value representing the variant. Case sensitive.
    fn from_str(s: &str) -> Option<Self>;
    /// Convert from a string value representing the variant. Lowercase.
    fn from_str_lower(s: &str) -> Option<Self>;
    /// A list all possible string variants.
    fn all_variants() -> Cow<'static, [Self]>;
}

/// A wrapper for primitive enums that supports catching and retaining any values
/// that don't have defined discriminants.
#[derive(Copy, Clone, Debug)]
pub enum EnumCatchAll<E, T> where E: PrimitiveEnum<T>, T: Sized + Copy + Debug {
    /// A matched discriminant
    Enum(E),
    /// Some other value, stored as the primitive type
    CatchAll(T)
}

impl<E, T> EnumCatchAll<E, T> where E: PrimitiveEnum<T>, T: Sized + Copy + Debug {
    pub fn from_enum(v: E) -> Self {
        EnumCatchAll::Enum(v)
    }
}

impl<E, T> From<E> for EnumCatchAll<E, T> where E: PrimitiveEnum<T>, T: Sized + Copy + Debug {
    fn from(v: E) -> Self {
        EnumCatchAll::Enum(v)
    }
}

impl<E, T> PartialEq<Self> for EnumCatchAll<E, T> where E: PrimitiveEnum<T>, T: Sized + Copy + Debug + PartialEq {
    fn eq(&self, other: &Self) -> bool {
        self.to_primitive() == other.to_primitive()
    }
}

impl<E, T> PrimitiveEnum<T> for EnumCatchAll<E, T> where E: PrimitiveEnum<T>, T: Sized + Copy + Debug {
    fn from_primitive(val: T) -> Option<Self> {
        match E::from_primitive(val) {
            Some(p) => Some(EnumCatchAll::Enum(p)),
            None => Some(EnumCatchAll::CatchAll(val))
        }
    }

    fn to_primitive(&self) -> T {
        match *self {
            EnumCatchAll::Enum(p) => p.to_primitive(),
            EnumCatchAll::CatchAll(v) => v
        }
    }

    /// Display value, same as the name of a particular variant.
    fn to_display_str(&self) -> Cow<'static, str> {
        match *self {
            EnumCatchAll::Enum(p) => p.to_display_str(),
            EnumCatchAll::CatchAll(v) => format!("Unknown value: {:?}", v).into()
        }
    }

    fn from_str(s: &str) -> Option<Self> {
        E::from_str(s).map(|e| EnumCatchAll::Enum(e))
    }

    fn from_str_lower(s: &str) -> Option<Self> {
        E::from_str_lower(s).map(|e| EnumCatchAll::Enum(e))
    }

    fn all_variants() -> Cow<'static, [Self]> {
        let l: Vec<_> = E::all_variants().iter().map(|v| EnumCatchAll::Enum(*v)).collect();
        Cow::from(l)
    }
}