extern crate syn;

#[cfg(feature="std")]
pub fn collections_prefix() -> syn::Ty {
    syn::parse_type("::std").unwrap()
}

#[cfg(not(feature="std"))]
pub fn collections_prefix() -> syn::Ty {
    syn::parse_type("::collections").unwrap()
}




#[cfg(any(feature="std", feature="core_collections"))]
pub fn include_debug_codegen() -> bool {
    true
}

#[cfg(not(any(feature="std", feature="core_collections")))]
pub fn include_debug_codegen() -> bool {
    false
}