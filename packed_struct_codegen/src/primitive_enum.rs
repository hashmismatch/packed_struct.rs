extern crate quote;
extern crate syn;

use utils::*;

pub fn derive(ast: &syn::DeriveInput, prim_type: syn::Ty) -> quote::Tokens {

    let ref name = ast.ident;
    let v = get_unitary_enum(ast);
    //panic!("v: {:?}", v);

    let from_primitive_match: Vec<_> = v.iter().map(|x| {
        let d = x.discriminant;
        let d = syn::Lit::Int(d, syn::IntTy::Unsuffixed);

        let n = &x.variant.ident;
        quote! {
            #d => Some(#name::#n)
    }}).collect();

    let to_display_str: Vec<_> = v.iter().map(|x| {
        let n = &x.variant.ident;
        let d = n.as_ref().to_string();
        quote! {
            #name::#n => #d
    }}).collect();

    let from_str: Vec<_> = v.iter().map(|x| {
        let n = &x.variant.ident;
        let d = n.as_ref().to_string();
        quote! {
            #d => Some(#name::#n)
    }}).collect();

    let all_variants: Vec<_> = v.iter().map(|x| {
        let n = &x.variant.ident;
        quote! { #name::#n }
    }).collect();
    let all_variants_len = all_variants.len();

    /*
    let max_value = v.iter().map(|x| x.discriminant).max().expect("Missing discriminants?");    
    let max_bits = 8;
    
    if max_bits > 8 {
        panic!("More than u8 as the base type for an enum isn't supported at the moment!");
    }
    */

    let all_variants_const_ident = syn::Ident::from(format!("{}_ALL", to_snake_case(name.as_ref()).to_uppercase() ));

    quote! {

        const #all_variants_const_ident: &'static [#name; #all_variants_len] = &[ #(#all_variants),* ];

        impl ::packed_struct::PrimitiveEnum<#prim_type> for #name {
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
            fn to_display_str(&self) -> &'static str {
                match *self {
                    #(#to_display_str),*
                }
            }

            #[inline]
            fn from_str(s: &str) -> Option<Self> {
                match s {
                    #(#from_str),* ,
                    _ => None
                }
            }

            #[inline]
            fn all_variants() -> &'static [Self] {
                #all_variants_const_ident
            }
        }
    }
}

#[derive(Debug)]
struct Variant {
    variant: syn::Variant,
    discriminant: u64
}


fn get_unitary_enum(input: &syn::DeriveInput) -> Vec<Variant> {
    match input.body {
        syn::Body::Enum(ref variants) => {
            let mut r = Vec::new();

            let mut d = 0;

            for variant in variants {
                if variant.data != syn::VariantData::Unit {
                    break;
                }

                let discriminant = match variant.discriminant {
                    Some(syn::ConstExpr::Lit(syn::Lit::Int(discrimimant,_))) => { discrimimant },
                    Some(ref p @ _) => {
                        panic!("Unsupported const expr: {:?}", p);
                    },
                    None => {
                        d + 1
                    }
                };

                r.push(Variant {
                    variant: variant.clone(),
                    discriminant: discriminant
                });

                d = discriminant;
            }
            return r;
        },
        _ => () 
    }

    panic!("Enum's variants must be unitary.");
}