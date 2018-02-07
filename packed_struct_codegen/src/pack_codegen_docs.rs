extern crate quote;
extern crate syn;

use packed_struct::*;

use pack::*;
use pack_parse::syn_to_string;
use common::*;
use utils::*;




pub fn struct_runtime_formatter(parsed: &PackStruct) -> quote::Tokens {
    let (impl_generics, ty_generics, where_clause) = parsed.ast.generics.split_for_impl();
    let name = &parsed.ast.ident;
    let snake_name = to_snake_case(name.as_ref());
    let stdlib_prefix = collections_prefix();
    let debug_fields_fn = syn::Ident::from(format!("debug_fields_{}", snake_name));

    let display_header = format!("{} ({} {})",
        name,
        parsed.num_bytes,
        if parsed.num_bytes == 1 { "byte" } else { "bytes" }
    );
    
    let mut debug_fields = vec![];
    for field in &parsed.fields {
        match field {
            &FieldKind::Regular { ref ident, ref field } => {
                let ref name_str = ident.as_ref().to_string();
                let bits = syn::parse_expr(&format!("{}..{}", field.bit_range.start, field.bit_range.end)).unwrap();
                
                debug_fields.push(quote! {
                    ::packed_struct::debug_fmt::DebugBitField {
                        name: #name_str.into(),
                        bits: #bits,
                        display_value: format!("{:?}", src.#ident).into()
                    }
                });
            },
            &FieldKind::Array { ref ident, size, ref elements } => {
                for (i, field) in elements.iter().enumerate() {
                    let name_str = format!("{}[{}]", syn_to_string(ident), i);
                    let bits = syn::parse_expr(&format!("{}..{}", field.bit_range.start, field.bit_range.end)).unwrap();
                    
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
    let result_ty = result_type();

    quote! {
        #[doc(hidden)]
        pub fn #debug_fields_fn(src: &#name) -> [::packed_struct::debug_fmt::DebugBitField<'static>; #num_fields] {
            [#(#debug_fields),*]
        }

        #[allow(unused_imports)]
        impl #impl_generics ::packed_struct::debug_fmt::PackedStructDebug for #name #ty_generics #where_clause {
            fn fmt_fields(&self, fmt: &mut #stdlib_prefix::fmt::Formatter) -> #result_ty <(), #stdlib_prefix::fmt::Error> {
                use ::packed_struct::PackedStruct;
                
                let fields = #debug_fields_fn(self);
                let packed: Vec<_> = self.pack()[..].into();
                ::packed_struct::debug_fmt::packable_fmt_fields(fmt, &packed, &fields)
            }
        }

        #[allow(unused_imports)]
        impl #impl_generics #stdlib_prefix::fmt::Display for #name #ty_generics #where_clause {
            #[allow(unused_imports)]
            fn fmt(&self, f: &mut #stdlib_prefix::fmt::Formatter) -> #stdlib_prefix::fmt::Result {                
                use ::packed_struct::*;
                use ::packed_struct::debug_fmt::*;

                let packed = self.pack();
                let l = packed.len();
                
                try!(f.write_str(#display_header));
                try!(f.write_str("\r\n"));
                try!(f.write_str("\r\n"));

                // decimal
                try!(f.write_str("Decimal\r\n"));
                try!(f.write_str("["));
                for i in 0..l {
                    try!(write!(f, "{}", packed[i]));
                    if (i + 1) != l {
                        try!(f.write_str(", "));
                    }
                }
                try!(f.write_str("]"));

                try!(f.write_str("\r\n"));
                try!(f.write_str("\r\n"));
                                
                // hex
                try!(f.write_str("Hex\r\n"));
                try!(f.write_str("["));
                for i in 0..l {
                    try!(write!(f, "0x{:X}", packed[i]));
                    if (i + 1) != l {
                        try!(f.write_str(", "));
                    }
                }
                try!(f.write_str("]"));
                try!(f.write_str("\r\n"));
                try!(f.write_str("\r\n"));

                try!(f.write_str("Binary\r\n"));
                try!(f.write_str("["));
                for i in 0..l {
                    try!(write!(f, "0b{:08b}", packed[i]));
                    if (i + 1) != l {
                        try!(f.write_str(", "));
                    }
                }
                try!(f.write_str("]"));
                try!(f.write_str("\r\n"));
                try!(f.write_str("\r\n"));


                try!(self.fmt_fields(f));
                Ok(())
            }
        }
    }
}



pub fn type_docs(parsed: &PackStruct) -> quote::Tokens {
    let mut doc = quote! {};

    let mut doc_html = format!("/// Structure that can be packed an unpacked into {size_bytes} bytes.\r\n",
        size_bytes = parsed.num_bytes
    );

    doc_html.push_str("/// <table>\r\n");
    doc_html.push_str("/// <thead><tr><td>Bit, MSB0</td><td>Name</td><td>Type</td></tr></thead>\r\n");
    doc_html.push_str("/// <tbody>\r\n");

    for field in &parsed.fields {
        match field {
            &FieldKind::Regular { ref ident, ref field } => {
                let ty = &field.ty;
                let ref name_str = ident.as_ref().to_string();
                let bits = format!("{}..{}", field.bit_range.start, field.bit_range.end);

                doc_html.push_str(&format!("/// <tr><td>{}</td><td>{}</td><td>{}</td></tr>\r\n", bits, name_str, syn_to_string(ty)));
            },
            &FieldKind::Array { ref ident, size, ref elements } => {
                for (i, field) in elements.iter().enumerate() {
                    let ty = &field.ty;
                    let name_str = format!("{}[{}]", syn_to_string(ident), i);
                    let bits = format!("{}..{}", field.bit_range.start, field.bit_range.end);

                    doc_html.push_str(&format!("/// <tr><td>{}</td><td>{}</td><td>{}</td></tr>\r\n", bits, name_str, syn_to_string(ty)));
                }                
            }
        }
        
    }


    doc_html.push_str("/// </tbody>\r\n");
    doc_html.push_str("/// </table>\r\n");

    doc.append(&doc_html);

    doc
}