extern crate quote;
extern crate syn;

use crate::pack::*;
use crate::pack_codegen_docs::*;
use crate::common::*;
use syn::spanned::Spanned;
use crate::utils::*;

use crate::utils_syn::tokens_to_string;

pub fn derive_pack(parsed: &PackStruct) -> syn::Result<proc_macro2::TokenStream> {

    let (impl_generics, ty_generics, where_clause) = parsed.derive_input.generics.split_for_impl();
    let name = &parsed.derive_input.ident;

    let type_documentation = type_docs(parsed);
    let num_bytes = parsed.num_bytes;
    let num_bits = parsed.num_bits;
    

    let mut pack_fields = vec![];
    let mut unpack_fields = vec![];
    let mut unpack_struct_set = vec![];

    {
        let mut reg  = |src: &dyn quote::ToTokens, target: &dyn quote::ToTokens, field: &FieldRegular| -> syn::Result<()> {
            let bits = pack_bits(field);

            let pack = pack_field(src, field);
            let unpack = unpack_field(field)?;

            let pack_bits = bits.pack;
            let unpack_bits = bits.unpack;

            pack_fields.push(quote! {
                {
                    let packed = { #pack };
                    #pack_bits
                }
            });

            unpack_fields.push(quote! {
                let #target = {
                    let bytes = { #unpack_bits };
                    #unpack
                };
            });

            Ok(())
        };


        for field in &parsed.fields {
            match field {
                FieldKind::Regular { ref ident, ref field } => {
                    reg(ident, ident, field)?;

                    unpack_struct_set.push(quote! {
                        #ident: #ident
                    });
                },
                FieldKind::Array { ref ident, ref elements, .. } => {
                    let mut array_unpacked_elements = vec![];
                    for (i, field) in elements.iter().enumerate() {
                        let src: syn::ExprIndex = syn::parse_str(&format!("{}[{}]", tokens_to_string(ident), i))?;
                        let target: syn::Ident = syn::parse_str(&format!("{}_{}", tokens_to_string(ident), i))?;

                        reg(&src, &target, field)?;
                        array_unpacked_elements.push(target);
                    }

                    unpack_struct_set.push(quote! {
                        #ident: [
                            #(#array_unpacked_elements),*
                        ]
                    });
                }
            }        
        }

    }

    let result_ty = result_type();

    let debug_fmt = if include_debug_codegen() {
        let q = struct_runtime_formatter(parsed)?;

        quote! {
            #q

            impl #impl_generics #name #ty_generics #where_clause {
                #[allow(dead_code)]
                /// Display formatter for console applications
                pub fn packed_struct_display_formatter<'a>(&'a self) -> ::packed_struct::debug_fmt::PackedStructDisplay<'a, Self> {
                    ::packed_struct::debug_fmt::PackedStructDisplay::new(self)
                }
            }

        }
    } else {
        quote! {}
    };

    let q = quote! {
        #type_documentation
        impl #impl_generics ::packed_struct::PackedStruct for #name #ty_generics #where_clause {
            type ByteArray = [u8; #num_bytes];

            #[inline]
            #[allow(unused_imports, unused_parens)]
            fn pack(&self) -> ::packed_struct::PackingResult<Self::ByteArray> {
                use ::packed_struct::*;

                let mut target = [0 as u8; #num_bytes];

                #(#pack_fields)*

                Ok(target)
            }

            #[inline]
            #[allow(unused_imports, unused_parens)]
            fn unpack(src: &Self::ByteArray) -> #result_ty <#name, ::packed_struct::PackingError> {
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
        
        #debug_fmt
    };

    Ok(q)
}



struct PackBitsCopy {
    pack: proc_macro2::TokenStream,
    unpack: proc_macro2::TokenStream
}

fn pack_bits(field: &FieldRegular) -> PackBitsCopy {
    // memcpy
    if (field.bit_range_rust.start % 8) == 0 && (field.bit_range_rust.end % 8) == 0 &&
       (field.bit_range_rust.len() % 8) == 0 && field.bit_range_rust.len() >= 8 
    {
        let start = field.bit_range_rust.start / 8;
        let end = field.bit_range_rust.end / 8;
        
        PackBitsCopy {
            pack: quote! {
                target[#start..#end].copy_from_slice(&packed);
            },
            unpack: quote! {
                let mut b = [0; (#end - #start)];
                b[..].copy_from_slice(&src[#start..#end]);
                b
            }
        }
    } else {
        let packed_field_len = (field.bit_width as f32 / 8.0).ceil() as usize; 
        let start_byte = (field.bit_range_rust.start as f32 / 8.0).floor() as usize;
        let shift = ((packed_field_len as isize*8) - (field.bit_width as isize)) - (field.bit_range_rust.start as isize - (start_byte as isize * 8));

        let emit_shift = |s: isize| {
            if s == 0 {
                quote! {}
            } else if s > 0 {
                quote! { << #s }
            } else {
                let s = -s;
                quote! { >> #s }
            }
        };

        let mut l = 8 - ((packed_field_len as isize*8) - field.bit_width as isize);
        let mut dst_byte = start_byte;

        let mut pack = vec![];
        let mut unpack = vec![];

        for i in 0..packed_field_len {
            let src_mask = ones_u8(l as u8);                        
            let bit_shift = emit_shift(shift);
            pack.push(quote! {
                let _a = #i;
                target[#dst_byte] |= (packed[#i] & #src_mask) #bit_shift;  
            });
            
            let bit_shift = emit_shift(-shift);
            unpack.push(quote! {
                let _a = #i;
                b[#i] |= (src[#dst_byte] #bit_shift) & #src_mask;
            });

            if shift < 0 && (dst_byte - start_byte) <= packed_field_len {
                let shift = 8+shift;
                let src_mask = ones_u8(8-shift as u8);

                let bit_shift = emit_shift(shift);                
                pack.push(quote! {
                    let _b = #i;
                    target[#dst_byte + 1] |= (((packed[#i] & #src_mask) as u16) #bit_shift) as u8;  
                });

                let bit_shift = emit_shift(-shift);
                unpack.push(quote! {
                    let _b = #i;
                    b[#i] |= (((src[#dst_byte + 1] as u16) #bit_shift) & #src_mask as u16) as u8;
                });
            } else if shift > 0 && (dst_byte - start_byte) <= packed_field_len && i < packed_field_len - 1 {
                let shift = -(8-shift);
                let bit_shift = emit_shift(shift);
                let src_mask = !ones_u8(-shift as u8);

                pack.push(quote! {
                    let _c = #i;
                    target[#dst_byte] |= (((packed[#i + 1] & #src_mask) as u16) #bit_shift) as u8;  
                });

                let bit_shift = emit_shift(-shift);
                unpack.push(quote! {
                    let _c = #i;
                    b[#i + 1] |= (((src[#dst_byte] as u16) #bit_shift) & #src_mask as u16) as u8;
                });
            }

            dst_byte += 1;
            l += 8;                
        }
        
        PackBitsCopy {
            pack: quote! {
                #(#pack)*
            },
            unpack: quote! {
                let mut b = [0; #packed_field_len];
                #(#unpack)*
                b
            }
        }  
    }
}


fn pack_field(name: &dyn quote::ToTokens, field: &FieldRegular) -> proc_macro2::TokenStream {
    let mut output = quote! { (self.#name) };

    for wrapper in &field.serialization_wrappers {
        match wrapper {
            &SerializationWrapper::PrimitiveEnum => {
                output = quote! {
                    {
                        use ::packed_struct::PrimitiveEnum;

                        let primitive_integer = { #output }.to_primitive();
                        primitive_integer
                    }
                };
            },
            &SerializationWrapper::Integer { ref integer } => {
                output = quote! {
                    {
                        use ::packed_struct::types::*;
                        use ::packed_struct::types::bits::*;                        

                        let sized_integer: #integer = { #output }.into();
                        sized_integer
                    }
                };
            },
            &SerializationWrapper::Endiannes { ref endian } => {
                output = quote! {
                    {
                        use ::packed_struct::types::*;
                        use ::packed_struct::types::bits::*;

                        let wrapper: #endian <_, _, _> = { #output }.into();
                        wrapper
                    }
                };
            }
        }
    }

    quote! {
        {
            { & #output }.pack()?
        }
    }
}

fn unpack_field(field: &FieldRegular) -> syn::Result<proc_macro2::TokenStream> {
    let wrappers: Vec<_> = field.serialization_wrappers.iter().rev().cloned().collect();

    let result_ty = result_type();
    let mut unpack = quote! { bytes };

    let mut i = 0;
    loop {
        match (wrappers.get(i), wrappers.get(i+1)) {
            (Some(&SerializationWrapper::Endiannes { ref endian }), Some(&SerializationWrapper::Integer { ref integer })) => {
                
                unpack = quote! {
                    use ::packed_struct::types::*;
                    use ::packed_struct::types::bits::*;

                    let res: #result_ty <#endian <_, _, #integer >, PackingError> = <#endian <_, _, _>>::unpack(& #unpack );
                    let unpacked = res?;
                    **unpacked
                };

                i += 1;
            }
            (Some(&SerializationWrapper::PrimitiveEnum), _) => {
                let ty = &field.ty;
                
                unpack = quote! {
                    use ::packed_struct::PrimitiveEnum;

                    let primitive_integer: <#ty as PrimitiveEnum>::Primitive = { #unpack };
                    let r = <#ty>::from_primitive(primitive_integer).ok_or(PackingError::InvalidValue);
                    r?
                };
            },
            (Some(&SerializationWrapper::Endiannes { ref endian }), _) => {
                let integer_ty = &field.ty;

                unpack = quote! {
                    use ::packed_struct::types::*;
                    use ::packed_struct::types::bits::*;

                    let res: #result_ty <#endian <_, _, #integer_ty >, PackingError> = <#endian <_, _, #integer_ty >>::unpack(& #unpack );
                    let unpacked = res?;
                    *unpacked
                };
            },
            (None, None) => {
                let ty = &field.ty;
                unpack = quote! {
                    <#ty>::unpack(& #unpack)?
                };
            },
            (_, _) => {
                return Err(syn::Error::new(field.ty.span(), "Unsupported serialization wrappers encountered!"));
            }            
        }

        i += 1;

        if wrappers.len() == 0 || i > wrappers.len() - 1 { break; }
    }

    Ok(unpack)
}