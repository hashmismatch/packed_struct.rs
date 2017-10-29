extern crate quote;
extern crate syn;

use packed_struct::*;

use pack::*;
use common::*;
use utils::*;

pub fn derive_pack(parsed: &PackStruct) -> quote::Tokens {

    let stdlib_prefix = collections_prefix();

    let name = &parsed.ast.ident;
    let snake_name = to_snake_case(name.as_ref());
    let (impl_generics, ty_generics, where_clause) = parsed.ast.generics.split_for_impl();
    let debug_fields_fn = syn::Ident::from(format!("debug_fields_{}", snake_name));

    let num_bytes = parsed.num_bytes;
    let num_bits = parsed.num_bits;
    let num_fields = parsed.fields.len();

    
    let mut field_actions = quote! { };
    let mut unpack_fields = Vec::new();
    
    //panic!("fields: {:?}", fields_expanded);
    
    for field in &parsed.fields {

        let ref name = field.info.ident;
        let name_unpack_bytes = name.clone();

        {
            let f = field_pack(field);
            field_actions.append(quote! {
                let packed_field = {
                    #f
                };
            }.as_str());
        }               

        // can we do a memcpy?
        if (field.bit_range_rust.start % 8) == 0 && (field.bit_range_rust.end % 8) == 0 &&
           (field.bit_range_rust.len() % 8) == 0 && field.bit_range_rust.len() >= 8 {

            let start = field.bit_range_rust.start / 8;
            let end = field.bit_range_rust.end / 8;
            
            field_actions = quote! {
                #field_actions

                &mut p[#start..#end].copy_from_slice(&packed_field);
            };

            match field.info.field_kind {
                FieldKind::Array { ref size, ref ident } if ident != &syn::Ident::new("u8") => {
                    let mut p = Vec::new();
                    let mut arr = Vec::new();

                    let w = (field.info.bit_width / 8) / size;
                                    

                    for i in 0..*size {
                        let start = start + (i*w);
                        let end = start + w;

                        let mut field_id = syn::Ident::new(format!("f_{}", i));

                        p.push(quote! {
                            let mut #field_id = [0; #w];
                            &mut #field_id[..].copy_from_slice(&src[#start..#end]);
                        });
                        arr.push(quote! {
                            #field_id
                        });
                    }

                    unpack_fields.push(quote! {
                        let #name_unpack_bytes = {
                            #(#p)*

                            [ #(#arr),* ]
                        };
                    });
                },
                _ => {
                    unpack_fields.push(quote! {
                        let mut #name_unpack_bytes = [0; (#end - #start)];
                        &mut #name_unpack_bytes[..].copy_from_slice(&src[#start..#end]);
                    });
                }
            }
            
        } else {
            // do a byte by byte copy, with shifting

            let start_byte = field.bit_range_rust.start / 8;

            let shift = field.bit_range_rust.start - (start_byte * 8);
            let mut l = field.bit_range_rust.len() as isize;
            

            let mut dst_byte = start_byte;
            let packed_field_len = (field.info.bit_width as f32 / 8.0).ceil() as usize; 
            //let mut end_byte = start_byte + packed_field_len;

            let mut field_unpack = quote! {
                let mut #name_unpack_bytes = [0; #packed_field_len];
            };

            for i in 0..packed_field_len {
                let src_mask = ones_u8(l as u8);
                
                field_actions = quote! {
                    #field_actions
                    p[#dst_byte] |= (packed_field[#i] & #src_mask) >> #shift;  
                };

                field_unpack.append(quote! {
                    #name_unpack_bytes[#i] |= (src[#dst_byte] << #shift) & #src_mask;
                }.as_str());

                let spillover = {
                    let t = (dst_byte + 1) * 8;
                    let s = (dst_byte * 8) + (l as usize) + shift;
                    s > t
                };

                //if (l > shift as isize) && shift > 0 {
                if spillover {
                    field_actions = quote! {
                        #field_actions
                        p[#dst_byte + 1] |= (((packed_field[#i] & #src_mask) as u16) << (8 - #shift)) as u8;  
                    };

                    field_unpack.append(quote! {
                        #name_unpack_bytes[#i] |= (((src[#dst_byte + 1] as u16) >> (8 - #shift)) & #src_mask as u16) as u8;
                    }.as_str());
                }
                
                dst_byte += 1;
                l -= 8;                
            }

            unpack_fields.push(field_unpack);
        }     
        
    }

    let unpack_struct_set: Vec<_> = parsed.fields.iter().map(|x| { field_unpack(x) }).collect();

    let debug_fields: Vec<_> = parsed.fields.iter().map(|x| {
        let ref ident = x.info.ident;
        let ref name_str = x.info.ident.as_ref().to_string();
        let bits = syn::parse_expr(&format!("{}..{}", x.bit_range.start, x.bit_range.end)).unwrap();
        

        quote! {
            ::packed_struct::DebugBitField {
                name: #name_str.into(),
                bits: #bits,
                display_value: format!("{:?}", src.#ident).into()
            }
        }
    }).collect();

    let mut debug_fmt = quote! {};

    if include_debug_codegen() {

        let display_header = format!("{} ({} {})",
            name,
            num_bytes,
            if num_bytes == 1 { "byte" } else { "bytes" }
        );

        debug_fmt = quote! {
            pub fn #debug_fields_fn(src: &#name) -> [::packed_struct::DebugBitField<'static>; #num_fields] {
                [#(#debug_fields),*]
            }

            #[allow(unused_imports)]
            impl #impl_generics ::packed_struct::PackedStructDebug for #name #ty_generics #where_clause {
                /*
                fn fmt_bits(&self, fmt: &mut #stdlib_prefix::fmt::Formatter) -> Result<(), #stdlib_prefix::fmt::Error> {
                    let fields = #debug_fields_fn(self);
                    ::packed_struct::packable_fmt_bits(fmt, &fields)
                }
                */
                fn fmt_fields(&self, fmt: &mut #stdlib_prefix::fmt::Formatter) -> Result<(), #stdlib_prefix::fmt::Error> {
                    use ::packed_struct::PackedStruct;
                    
                    let fields = #debug_fields_fn(self);
                    let packed: Vec<_> = self.pack()[..].into();
                    ::packed_struct::packable_fmt_fields(fmt, &packed, &fields)
                }
            }

            #[allow(unused_imports)]
            impl #impl_generics #stdlib_prefix::fmt::Display for #name #ty_generics #where_clause {
                #[allow(unused_imports)]
                fn fmt(&self, f: &mut #stdlib_prefix::fmt::Formatter) -> #stdlib_prefix::fmt::Result {                
                    use ::packed_struct::*;

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
        };
    }


    let type_documentation = {
        let mut doc = quote! {};

        let mut doc_html = format!("/// Structure that can be packed an unpacked into {size_bytes} bytes.\r\n",
            size_bytes = num_bytes
        );

        doc_html.push_str("/// <table>\r\n");
        doc_html.push_str("/// <thead><tr><td>Bit, MSB0</td><td>Name</td><td>Type</td></tr></thead>\r\n");
        doc_html.push_str("/// <tbody>\r\n");

        for field in &parsed.fields {
            let ty = &field.info.ty;
            let ref name_str = field.info.ident.as_ref().to_string();
            let bits = format!("{}..{}", field.bit_range.start, field.bit_range.end);

            doc_html.push_str(&format!("/// <tr><td>{}</td><td>{}</td><td>{}</td></tr>\r\n", bits, name_str, ty_to_string(ty)));
        }


        doc_html.push_str("/// </tbody>\r\n");
        doc_html.push_str("/// </table>\r\n");

        doc.append(&doc_html);

        doc
    };


    quote! {
        #type_documentation
        impl #impl_generics ::packed_struct::PackedStruct<[u8; #num_bytes]> for #name #ty_generics #where_clause {
            #[inline]
            #[allow(unused_imports)]
            fn pack(&self) -> [u8; #num_bytes] {
                use ::packed_struct::*;

                let mut p = [0 as u8; #num_bytes];

                #field_actions

                p
            }

            #[inline]
            #[allow(unused_imports)]
            fn unpack(src: &[u8; #num_bytes]) -> Result<#name, ::packed_struct::PackingError> {
                use ::packed_struct::*;

                #(#unpack_fields)*
                
                Ok(#name {
                    #(#unpack_struct_set),*
                })
            }
        }

        impl ::packed_struct::PackedStructInfo for #name {
            #[inline]
            fn packed_bits() -> usize {
                #num_bits
            }
        }

        impl #impl_generics ::packed_struct::PackedStructSlice for #name #ty_generics #where_clause {
            #[inline]
            #[allow(unused_imports)]
            fn pack_to_slice(&self, output: &mut [u8]) -> Result<(), ::packed_struct::PackingError> {
                use ::packed_struct::*;

                if output.len() != #num_bytes {
                    return Err(::packed_struct::PackingError::BufferTooSmall);
                }
                let packed = self.pack();                
                &mut output[..].copy_from_slice(&packed[..]);
                Ok(())
            }

            #[inline]
            #[allow(unused_imports)]
            fn unpack_from_slice(src: &[u8]) -> Result<Self, ::packed_struct::PackingError> {
                use ::packed_struct::*;

                if src.len() != #num_bytes {
                    return Err(::packed_struct::PackingError::BufferTooSmall);
                }
                let mut s = [0; #num_bytes];
                &mut s[..].copy_from_slice(src);
                Self::unpack(&s)
            }

            #[inline]
            fn packed_bytes() -> usize {
                #num_bytes
            }
        }

        #debug_fmt
    }
}

fn field_pack(field: &FieldExpanded) -> quote::Tokens {
    let ref name = field.info.ident;

    match field.info.field_kind {
        FieldKind::Normal => {
            if let Some(ref wrapper_ty) = field.info.serialization_wrapper_ty {
                quote! {
                    #wrapper_ty ( self.#name ) .pack()
                }
            } else {            
                quote! {
                    self.#name.pack()
                }
            }
        },
        FieldKind::Enum { ref pack_ty } => {
            quote! {
                {
                    use ::packed_struct::*;

                    let n = (self.#name).to_primitive();
                    let p: #pack_ty = n.into();
                    p.pack()
                }
            }
        },
        FieldKind::Array { ref size, ref ident } => {
            if ident == &syn::Ident::new("u8") {
                quote! {
                    self.#name.pack()
                }
            } else {

                if (field.bit_range_rust.start % 8) != 0 || (field.bit_range_rust.end % 8) != 0 {
                    panic!("Bit level packing on arrays isn't supported yet.");
                }
                
                let mut n = 0;
                let w = field.info.bit_width / 8;
                let w_f = w / size;

                let mut q = quote! {
                    let mut o = [0; #w];
                };
                
                for i in 0..*size {
                    q.append(quote! {
                        let p = self.#name[#i].pack();
                        &mut o[#n .. (#n + #w_f)].copy_from_slice(&p[..]);
                    }.as_str());

                    n += w_f;
                }

                q.append(quote! {
                    o
                }.as_str());

                q
            }
        }
    }
}

fn field_unpack(field: &FieldExpanded) -> quote::Tokens {
    let ref name = field.info.ident;    

    match field.info.field_kind {
        FieldKind::Normal => {
            if let Some(ref wrapper_ty) = field.info.serialization_wrapper_ty {
                quote! {
                    #name: try!(<#wrapper_ty>::unpack(&#name)).0
                }
            } else {
                let ref ty = field.info.ty;            
                quote! {
                    #name: try!(<#ty>::unpack(&#name))
                }
            }
        },
        FieldKind::Enum { ref pack_ty } => {
            let ref ty = field.info.ty;            
            quote! {
                #name: {
                    use ::packed_struct::*;
                    let p = <#pack_ty>::unpack(&#name)?;
                    let p: u8 = p.into();
                    <#ty>::from_primitive(p).ok_or(PackingError::InvalidValue)?
                }
            }
        },
        FieldKind::Array { ref size, ref ident } => {

            let ty = ident;
            let mut f = Vec::new();

            for i in 0..*size {
                if ident == &syn::Ident::new("u8") {
                    f.push(quote! {
                        #name[#i]
                    });
                } else {
                    f.push(quote! {
                        try!(<#ty>::unpack(&#name[#i]))
                    });
                }
            }

            quote! {
                #name: [
                    #(#f),*
                ]
            }

        }
    }
    
    
}


pub fn ty_to_string(ty: &syn::Ty) -> String {
    use quote::ToTokens;
    
    let mut t = quote::Tokens::new();
    ty.to_tokens(&mut t);
    t.as_str().into()
}