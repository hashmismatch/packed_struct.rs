use quote::quote;
use proc_macro2::Span;
use quote::TokenStreamExt;
use syn::spanned::Spanned;
use crate::utils::*;
use crate::common::collections_prefix;

pub fn derive(ast: &syn::DeriveInput, mut prim_type: Option<syn::Type>) -> syn::Result<proc_macro2::TokenStream> {

    let stdlib_prefix = collections_prefix();

    let name = &ast.ident;
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
        let ty = match v.iter().find(|d| !d.suffix.is_empty()) {
            // the match arms reuse the suffixed literals, so they dictate the type
            Some(d) => d.suffix.clone(),
            None => infer_primitive_type(v.iter().map(|d| d.value()))
                .ok_or_else(|| syn::Error::new(ast.ident.span(), "No primitive integer type can hold all the discriminants of this enum."))?
                .to_string()
        };

        prim_type = Some(syn::parse_str(&ty)?);
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
    fn value(&self) -> i128 {
        if self.negative { -(self.discriminant as i128) } else { self.discriminant as i128 }
    }

    fn get_discriminant(&self) -> proc_macro2::TokenStream {
        let s = format!("{}{}",
            self.discriminant,
            self.suffix
        );
        let v: syn::LitInt = syn::parse_str(&s).expect("Error mid-parsing for disc value");

        if self.negative {
            quote! {
                - #v
            }
        } else {
            quote! { #v }
        }
    }
}


fn get_unitary_enum(input: &syn::DeriveInput) -> syn::Result<Vec<Variant>> {
    let data_enum = if let syn::Data::Enum(data_enum) = &input.data {
        data_enum
    } else {
        return Err(syn::Error::new(input.span(), "Only enums are supported."));
    };

    let mut r = Vec::new();

    let mut d: Option<u64> = None;
    let mut neg = false;

    for variant in &data_enum.variants {
        
        match variant.fields {
            syn::Fields::Named(_) | syn::Fields::Unnamed(_) => {
                break;
            }
            syn::Fields::Unit => {}
        }

        let (discriminant, negative, suffix) = match &variant.discriminant {
            Some((_, syn::Expr::Lit(syn::ExprLit { lit: syn::Lit::Int(lit_int), .. }))) => {
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
                match d {
                    None => (0, false, "".into()),
                    Some(d) => {
                        if neg {
                            (d-1, d-1 != 0, "".into())
                        } else {
                            (d+1, false, "".into())
                        }
                    }
                }
            }
        };

        r.push(Variant {
            variant: variant.clone(),
            discriminant,
            negative,
            suffix
        });

        d = Some(discriminant);
        neg = negative;                
    }
    
    Ok(r)
}

/// The smallest integer type that holds all the discriminant values. Unsigned
/// types are only used when none of the values is negative.
fn infer_primitive_type(values: impl Iterator<Item = i128>) -> Option<&'static str> {
    let (min, max) = values.fold((0, 0), |(min, max), v| (min.min(v), max.max(v)));

    let candidates: [(&str, i128, i128); 4] = if min < 0 {
        [
            ("i8", i8::MIN as i128, i8::MAX as i128),
            ("i16", i16::MIN as i128, i16::MAX as i128),
            ("i32", i32::MIN as i128, i32::MAX as i128),
            ("i64", i64::MIN as i128, i64::MAX as i128)
        ]
    } else {
        [
            ("u8", 0, u8::MAX as i128),
            ("u16", 0, u16::MAX as i128),
            ("u32", 0, u32::MAX as i128),
            ("u64", 0, u64::MAX as i128)
        ]
    };

    candidates.iter().find(|(_, lo, hi)| *lo <= min && max <= *hi).map(|(ty, _, _)| *ty)
}

#[test]
fn test_infer_primitive_type() {
    let infer = |values: &[i128]| infer_primitive_type(values.iter().copied());

    assert_eq!(Some("u8"), infer(&[]));
    assert_eq!(Some("u8"), infer(&[0, 255]));
    assert_eq!(Some("u16"), infer(&[0, 256]));
    assert_eq!(Some("u32"), infer(&[0, u32::MAX as i128]));
    assert_eq!(Some("u64"), infer(&[0, u32::MAX as i128 + 1]));
    assert_eq!(Some("u64"), infer(&[u64::MAX as i128]));

    assert_eq!(Some("i8"), infer(&[-128, 127]));
    assert_eq!(Some("i16"), infer(&[-129]));
    assert_eq!(Some("i16"), infer(&[-1, 128]));
    assert_eq!(Some("i16"), infer(&[-1, 200]));
    assert_eq!(Some("i32"), infer(&[-1, 40_000]));
    assert_eq!(Some("i32"), infer(&[i32::MIN as i128]));
    assert_eq!(Some("i64"), infer(&[i32::MIN as i128 - 1]));
    assert_eq!(Some("i64"), infer(&[-3_000_000_000]));
    assert_eq!(Some("i64"), infer(&[-1, u32::MAX as i128 + 1]));
    assert_eq!(Some("i64"), infer(&[i64::MIN as i128, i64::MAX as i128]));

    assert_eq!(None, infer(&[-1, i64::MAX as i128 + 1]));
    assert_eq!(None, infer(&[-1, u64::MAX as i128]));
}
