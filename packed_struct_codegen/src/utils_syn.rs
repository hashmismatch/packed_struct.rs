use proc_macro2::TokenStream;
use quote::TokenStreamExt;
use syn::{Error, PathSegment, Result, spanned::Spanned, TypePath};

pub fn get_single_segment(type_path: &TypePath) -> Result<&PathSegment> {
    if type_path.path.segments.len() == 1 {
        let ref segment = type_path.path.segments[0];
        return Ok(segment);
    }

    Err(Error::new(type_path.span(), "A single path only!"))
}

pub fn get_expr_int_val(expr: &syn::Expr) -> Result<usize> {
    match expr {
        syn::Expr::Lit(syn::ExprLit { lit: syn::Lit::Int(lit_int), ..})  => {
            Ok(lit_int.base10_parse()?)
        },
        _ => Err(Error::new(expr.span(), "Unsupported extraction of int value"))
    }
}

pub fn tokens_to_string<T: quote::ToTokens>(t: &T) -> String {
    let mut tokens = TokenStream::new();
    t.to_tokens(&mut tokens);
    tokens.to_string()
}