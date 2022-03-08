extern crate quote;
extern crate syn;

use proc_macro2::Span;
use quote::TokenStreamExt;
use syn::spanned::Spanned;
use crate::utils::*;
use crate::common::collections_prefix;

pub fn derive(ast: &syn::DeriveInput, prim_type: syn::Type) -> syn::Result<proc_macro2::TokenStream> {

    let stdlib_prefix = collections_prefix();

    let name = &ast.ident;
    let v = get_unitary_enum(ast)?;

    let from_primitive_match: Vec<_> = v.iter().map(|x| {
        let n = &x.variant.ident;
        quote! {
            if val == #name::#n as #prim_type {
                return Some(#name::#n);
            }
        }
    }).collect();

    let to_display_str: Vec<_> = v.iter().map(|x| {
        let n = &x.variant.ident;
        let d = n.to_string();
        quote! {
            #name::#n => (#d)
    }}).collect();

    let from_str: Vec<_> = v.iter().map(|x| {
        let n = &x.variant.ident;
        let d = n.to_string();
        quote! {
            #d => Some(#name::#n)
    }}).collect();

    let from_str_lower: Vec<_> = v.iter().map(|x| {
        let n = &x.variant.ident;
        let d = n.to_string().to_lowercase();
        quote! {
            #d => Some(#name::#n)
    }}).collect();

    let all_variants: Vec<_> = v.iter().map(|x| {
        let n = &x.variant.ident;
        quote! { #name::#n }
    }).collect();
    let all_variants_len = all_variants.len();

    let all_variants_const_ident = syn::Ident::new(&format!("{}_ALL", to_snake_case(&name.to_string())).to_uppercase(), Span::call_site());
    
    let mut str_format = {
        let to_display_str = to_display_str.clone();
        let all_variants_const_ident = all_variants_const_ident.clone();

        quote! {
            impl ::packed_struct::PrimitiveEnumStaticStr for #name {
                #[inline]
                fn to_display_str(&self) -> &'static str {
                    match *self {
                        #(#to_display_str),*
                    }
                }

                #[inline]
                fn all_variants() -> &'static [Self] {
                    #all_variants_const_ident
                }
            }
        }
    };


    if crate::common::alloc_supported() {
        str_format.append_all(quote! {
            impl ::packed_struct::PrimitiveEnumDynamicStr for #name {
                #[inline]
                fn to_display_str(&self) -> #stdlib_prefix::borrow::Cow<'static, str> {
                    let s = match *self {
                        #(#to_display_str),*
                    };
                    s.into()
                }

                #[inline]
                fn all_variants() -> #stdlib_prefix::borrow::Cow<'static, [Self]> {
                    #stdlib_prefix::borrow::Cow::Borrowed(#all_variants_const_ident)
                }
            }
        });
    };

    let q = quote! {

        const #all_variants_const_ident: &'static [#name; #all_variants_len] = &[ #(#all_variants),* ];

        impl ::packed_struct::PrimitiveEnum for #name {
            type Primitive = #prim_type;

            #[inline]
            fn from_primitive(val: #prim_type) -> Option<Self> {
                #(#from_primitive_match)*

                None
            }

            #[inline]
            fn to_primitive(&self) -> #prim_type {
                *self as #prim_type
            }

            #[inline]
            fn from_str(s: &str) -> Option<Self> {
                match s {
                    #(#from_str),* ,
                    _ => None
                }
            }
            
            #[inline]
            fn from_str_lower(s: &str) -> Option<Self> {
                match s {
                    #(#from_str_lower),* ,
                    _ => None
                }
            }
        }

        #str_format
    };
    Ok(q)
}

struct Variant {
    variant: syn::Variant
}

fn get_unitary_enum(input: &syn::DeriveInput) -> syn::Result<Vec<Variant>> {
    let data_enum = if let syn::Data::Enum(data_enum) = &input.data {
        data_enum
    } else {
        return Err(syn::Error::new(input.span(), "Only enums are supported."));
    };

    let mut r = Vec::new();

    for variant in &data_enum.variants {
        
        match variant.fields {
            syn::Fields::Named(_) | syn::Fields::Unnamed(_) => {
                break;
            }
            syn::Fields::Unit => {}
        }

        r.push(Variant {
            variant: variant.clone()
        });
    }
    
    Ok(r)
}