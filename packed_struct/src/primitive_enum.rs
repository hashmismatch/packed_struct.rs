use prelude::v1::*;

pub trait PrimitiveEnum<T> where T: Sized + Copy + Debug, Self: Sized + Copy {
    fn from_primitive(val: T) -> Option<Self>;
    fn to_primitive(&self) -> T;
    fn to_display_str(&self) -> &'static str;
    fn from_str(s: &str) -> Option<Self>;
    fn all_variants() -> &'static [Self];
}

