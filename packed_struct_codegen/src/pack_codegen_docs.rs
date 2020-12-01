extern crate quote;
extern crate syn;

use pack::*;
use common::*;
use proc_macro2::Span;
use quote::{ToTokens, TokenStreamExt};
use utils::*;




pub fn struct_runtime_formatter(parsed: &PackStruct) -> syn::Result<proc_macro2::TokenStream> {
    let (impl_generics, ty_generics, where_clause) = parsed.derive_input.generics.split_for_impl();
    let name = &parsed.derive_input.ident;
    let snake_name = to_snake_case(&name.to_string());
    let stdlib_prefix = collections_prefix();
    let debug_fields_fn = syn::Ident::new(&format!("debug_fields_{}", snake_name), Span::call_site());

    let display_header = format!("{} ({} {})",
        name,
        parsed.num_bytes,
        if parsed.num_bytes == 1 { "byte" } else { "bytes" }
    );
    
    let mut debug_fields = vec![];
    for field in &parsed.fields {
        match field {
            &FieldKind::Regular { ref ident, ref field } => {
                let ref name_str = ident.to_string();
                let bits: syn::ExprRange = syn::parse_str(&format!("{}..{}", field.bit_range.start, field.bit_range.end))?;
                
                debug_fields.push(quote! {
                    ::packed_struct::debug_fmt::DebugBitField {
                        name: #name_str.into(),
                        bits: #bits,
                        display_value: format!("{:?}", src.#ident).into()
                    }
                });
            },
            &FieldKind::Array { ref ident, ref elements, .. } => {
                for (i, field) in elements.iter().enumerate() {
                    let name_str = format!("{}[{}]", ident.to_string(), i);
                    let bits: syn::ExprRange = syn::parse_str(&format!("{}..{}", field.bit_range.start, field.bit_range.end))?;
                    
                    debug_fields.push(quote! {
                        ::packed_struct::debug_fmt::DebugBitField {
                            name: #name_str.into(),
                            bits: #bits,
                            display_value: format!("{:?}", src.#ident[#i]).into()
                        }
                    });
                }
                
            }
        }
    }

    let num_fields = debug_fields.len();
    let num_bytes = parsed.num_bytes;
    let result_ty = result_type();

    let q = quote! {
        #[doc(hidden)]
        pub fn #debug_fields_fn(src: &#name) -> [::packed_struct::debug_fmt::DebugBitField<'static>; #num_fields] {
            [#(#debug_fields),*]
        }

        #[allow(unused_imports)]
        impl #impl_generics ::packed_struct::debug_fmt::PackedStructDebug for #name #ty_generics #where_clause {
            fn fmt_fields(&self, fmt: &mut #stdlib_prefix::fmt::Formatter) -> #result_ty <(), #stdlib_prefix::fmt::Error> {
                use ::packed_struct::PackedStruct;
                
                let fields = #debug_fields_fn(self);
                let packed: [u8; #num_bytes] = self.pack()?;
                ::packed_struct::debug_fmt::packable_fmt_fields(fmt, &packed, &fields)
            }

            fn packed_struct_display_header() -> &'static str {
                #display_header
            }
        }

        #[allow(unused_imports)]
        impl #impl_generics #stdlib_prefix::fmt::Display for #name #ty_generics #where_clause {
            #[allow(unused_imports)]
            fn fmt(&self, f: &mut #stdlib_prefix::fmt::Formatter) -> #stdlib_prefix::fmt::Result {                
                let display = ::packed_struct::debug_fmt::PackedStructDisplay::new(self);
                display.fmt(f)
            }
        }
    };
    
    Ok(q)
}

use std::ops::Range;

use crate::utils_syn::tokens_to_string;


pub fn type_docs(parsed: &PackStruct) -> proc_macro2::TokenStream {
    let mut doc = quote! {};

    let mut doc_html = format!("/// Structure that can be packed an unpacked into {size_bytes} bytes.\r\n",
        size_bytes = parsed.num_bytes
    );

    doc_html.push_str("/// <table>\r\n");
    doc_html.push_str("/// <thead><tr><td>Bit, MSB0</td><td>Name</td><td>Type</td></tr></thead>\r\n");
    doc_html.push_str("/// <tbody>\r\n");

    {
        let mut emit_field_docs = |bits: &Range<usize>, field_ident, ty| {

            let bits_str = {
                if bits.start == bits.end {
                    format!("{}", bits.start)
                } else {
                    format!("{}:{}", bits.start, bits.end)
                }
            };

            // todo: friendly integer, reserved types. add LSB/MSB integer info.            


            doc_html.push_str(&format!("/// <tr><td>{}</td><td>{}</td><td>{}</td></tr>\r\n", bits_str, field_ident, tokens_to_string(ty)));
        };

        for field in &parsed.fields {
            match field {
                &FieldKind::Regular { ref ident, ref field } => {
                    emit_field_docs(&field.bit_range, ident.to_string(), &field.ty);
                },
                &FieldKind::Array { ref ident, ref elements, .. } => {
                    for (i, field) in elements.iter().enumerate() {
                        emit_field_docs(&field.bit_range, format!("{}[{}]", ident.to_string(), i), &field.ty);
                    }
                }
            }            
        }
    }


    doc_html.push_str("/// </tbody>\r\n");
    doc_html.push_str("/// </table>\r\n");

    //doc.append(doc_html);
    //doc_html.to_tokens(&mut doc);

    doc
}