extern crate syn;

#[cfg(feature="std")]
pub fn collections_prefix() -> syn::Type {
    syn::parse_str("::std").unwrap()
}

#[cfg(not(feature="std"))]
pub fn collections_prefix() -> syn::Type {
    syn::parse_str("::alloc").unwrap()
}

#[cfg(feature="std")]
pub fn result_type() -> syn::Type {
    syn::parse_str("::std::result::Result").expect("result type parse error")
}

#[cfg(not(feature="std"))]
pub fn result_type() -> syn::Type {
    syn::parse_str("::core::result::Result").expect("result type parse error")
}


pub fn alloc_supported() -> bool {
    #[cfg(any(feature="std", feature="alloc"))]
    {
        true
    }
    #[cfg(not(any(feature="std", feature="alloc")))]
    {
        false
    }}


pub fn include_debug_codegen() -> bool {
    alloc_supported()    
}
