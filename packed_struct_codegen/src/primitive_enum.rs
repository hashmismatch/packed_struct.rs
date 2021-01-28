extern crate quote;
extern crate syn;

use proc_macro2::Span;
use quote::TokenStreamExt;
use syn::spanned::Spanned;
use crate::utils::*;
use crate::common::collections_prefix;

pub fn derive(ast: &syn::DeriveInput, mut prim_type: Option<syn::Type>) -> syn::Result<proc_macro2::TokenStream> {

    let stdlib_prefix = collections_prefix();

    let ref name = ast.ident;
    let v = get_unitary_enum(ast)?;

    let from_primitive_match: Vec<_> = v.iter().map(|x| {
        let d = x.get_discriminant();
        let n = &x.variant.ident;
        quote! {
            #d => Some(#name::#n)
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

    if prim_type.is_none() {
        let min_ty: Vec<String> = v.iter().map(|d| {
            if !d.suffix.is_empty() {
                d.suffix.clone()
            } else {
                if d.negative {
                    let n = d.discriminant as i64;
                    if n < <i32>::min_value() as i64 {
                        "i64".into()
                    } else {
                        let n = -n;
                        if n < <i16>::min_value() as i64 {
                            "i32".into()
                        } else if n < <i8>::min_value() as i64 {
                            "i16".into()
                        } else {
                            "i8".into()
                        }
                    }
                } else {
                    let n = d.discriminant as u64;
                    if n > <u32>::max_value() as u64 {
                        "u64".into()
                    } else if n > <u16>::max_value() as u64 {
                        "u32".into()
                    } else if n > <u8>::max_value() as u64 {
                        "u16".into()
                    } else {
                        "u8".into()
                    }
                }
            }
        }).collect();

        // first mention, higher priority
        let priority = [
            "i64",
            "i32",
            "i16",
            "i8",
            "u64",
            "u32",
            "u16",
            "u8"
        ];
        
        let mut ty = "u8".to_string();
        for t in min_ty {
            if priority.iter().position(|&x| x == t).unwrap() < priority.iter().position(|&x| x == ty).unwrap() {
                ty = t;
            }
        }
        
        prim_type = Some(syn::parse_str(&ty).expect("int ty parsing failed"));
    }    

    let prim_type = prim_type.expect("Unable to detect the primitive type for this enum.");

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
                match val {
                    #(#from_primitive_match),* ,
                    _ => None
                }
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
    variant: syn::Variant,
    discriminant: u64,
    negative: bool,
    suffix: String
}

impl Variant {
    fn get_discriminant(&self) -> proc_macro2::TokenStream {
        let s = format!("{}{}",
            self.discriminant,
            self.suffix
        );
        let v: syn::LitInt = syn::parse_str(&s).expect("Error mid-parsing for disc value");

        let q = if self.negative {
            quote! {
                - #v
            }
        } else {
            quote! { #v }
        };
        
        q
    }
}


fn get_unitary_enum(input: &syn::DeriveInput) -> syn::Result<Vec<Variant>> {
    let data_enum = if let syn::Data::Enum(data_enum) = &input.data {
        data_enum
    } else {
        return Err(syn::Error::new(input.span(), "Only enums are supported."));
    };

    let mut r = Vec::new();

    let mut d = 0;
    let mut neg = false;

    for variant in &data_enum.variants {
        
        match variant.fields {
            syn::Fields::Named(_) | syn::Fields::Unnamed(_) => {
                break;
            }
            syn::Fields::Unit => {}
        }

        let (discriminant, negative, suffix) = match &variant.discriminant {
            Some((_, syn::Expr::Lit(syn::ExprLit { lit: syn::Lit::Int(ref lit_int), .. }))) => {
                (lit_int.base10_parse()?, false, lit_int.suffix().into())
            },
            Some((_,
                syn::Expr::Unary(syn::ExprUnary {
                    op: syn::UnOp::Neg(_),
                    expr,
                    ..
                }) 
            )) => {

                match **expr {
                    syn::Expr::Lit(syn::ExprLit { lit: syn::Lit::Int(ref lit_int), .. }) => {
                        (lit_int.base10_parse()?, true, lit_int.suffix().into())
                    },
                    _ => return Err(syn::Error::new(expr.span(), "Unsupported enum const expr (negated)"))
                }
            }
            Some(_) => {
                return Err(syn::Error::new(variant.span(), "Unsupported enum const expr"));
            },
            None => {
                if neg {
                    (d-1, if d-1 == 0 { false } else { true }, "".into())
                } else {
                    (d+1, false, "".into())
                }
            }
        };

        r.push(Variant {
            variant: variant.clone(),
            discriminant,
            negative,
            suffix
        });

        d = discriminant;                
        neg = negative;                
    }
    
    Ok(r)
}