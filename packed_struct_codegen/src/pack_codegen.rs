extern crate quote;
extern crate syn;

use packed_struct::*;

use pack::*;
use pack_codegen_docs::*;
use pack_parse::syn_to_string;
use common::*;
use utils::*;





pub fn derive_pack(parsed: &PackStruct) -> quote::Tokens {

    let (impl_generics, ty_generics, where_clause) = parsed.ast.generics.split_for_impl();
    let name = &parsed.ast.ident;
    let snake_name = to_snake_case(name.as_ref());

    let type_documentation = type_docs(parsed);
    let debug_fmt = if include_debug_codegen() {
        struct_runtime_formatter(parsed)
    } else {
        quote! {}
    };
    let num_bytes = parsed.num_bytes;
    let num_bits = parsed.num_bits;
    let num_fields = parsed.fields.len();


    let mut pack_fields = vec![];
    let mut unpack_fields = vec![];
    let mut unpack_struct_set = vec![];

    {
        let mut reg = |src: &syn::Ident, target: &syn::Ident, field: &FieldRegular| {
            let bits = pack_bits(field);

            let pack = pack_field(src, field);
            let unpack = unpack_field(field);

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
        };


        for field in &parsed.fields {
            match field {
                &FieldKind::Regular { ref ident, ref field } => {
                    reg(ident, ident, field);

                    unpack_struct_set.push(quote! {
                        #ident: #ident
                    });
                },
                &FieldKind::Array { ref ident, size, ref elements } => {
                    let mut array_unpacked_elements = vec![];
                    for (i, field) in elements.iter().enumerate() {
                        let src = syn::Ident::new(format!("{}[{}]", syn_to_string(ident), i));
                        let target = syn::Ident::new(format!("{}_{}", syn_to_string(ident), i));
                        reg(&src, &target, field);
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

    quote! {
        #type_documentation
        impl #impl_generics ::packed_struct::PackedStruct<[u8; #num_bytes]> for #name #ty_generics #where_clause {
            #[inline]
            #[allow(unused_imports)]
            fn pack(&self) -> [u8; #num_bytes] {
                use ::packed_struct::*;

                let mut target = [0 as u8; #num_bytes];

                #(#pack_fields)*

                target
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



struct PackBitsCopy {
    pack: quote::Tokens,
    unpack: quote::Tokens
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
                &mut target[#start..#end].copy_from_slice(&packed);
            },
            unpack: quote! {
                let mut b = [0; (#end - #start)];
                &mut b[..].copy_from_slice(&src[#start..#end]);
                b
            }
        }
    } else {
        let packed_field_len = (field.bit_width as f32 / 8.0).ceil() as usize; 

        let start_byte = field.bit_range_rust.start / 8;
        let shift = (8 - (field.bit_width as isize)) % 8 - (field.bit_range_rust.start as isize - (start_byte as isize * 8));
        
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

        let mut l = field.bit_range_rust.len() as isize;
        let mut dst_byte = start_byte;

        let mut pack = vec![];
        let mut unpack = vec![];

        for i in 0..packed_field_len {
            let src_mask = ones_u8(l as u8);
            
            let bit_shift = emit_shift(shift);
            pack.push(quote! {
                target[#dst_byte] |= (packed[#i] & #src_mask) #bit_shift;  
            });
            
            let bit_shift = emit_shift(-shift);
            unpack.push(quote! {
                b[#i] |= (src[#dst_byte] #bit_shift) & #src_mask;
            });

            if shift < 0 {
                let shift = shift + 8;

                let bit_shift = emit_shift(shift);                
                pack.push(quote! {
                    target[#dst_byte + 1] |= (((packed[#i] & #src_mask) as u16) #bit_shift) as u8;  
                });

                let bit_shift = emit_shift(-shift);
                unpack.push(quote! {
                    b[#i] |= (((src[#dst_byte + 1] as u16) #bit_shift) & #src_mask as u16) as u8;
                });
            }

            dst_byte += 1;
            l -= 8;                
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


fn pack_field(name: &syn::Ident, field: &FieldRegular) -> quote::Tokens {
    let mut output = quote! { (self.#name) };

    for wrapper in &field.serialization_wrappers {
        match wrapper {
            &SerializationWrapper::PrimitiveEnumWrapper => {
                output = quote! {
                    {
                        use ::packed_struct::PrimitiveEnum;

                        let primitive_integer = { #output }.to_primitive();
                        primitive_integer
                    }
                };
            },
            &SerializationWrapper::IntegerWrapper { ref integer } => {
                output = quote! {
                    {
                        use ::packed_struct::types::*;
                        use ::packed_struct::types::bits::*;                        

                        let sized_integer: #integer = { #output }.into();
                        sized_integer
                    }
                };
            },
            &SerializationWrapper::EndiannesWrapper { ref endian } => {
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
            { & #output }.pack()
        }
    }
}

fn unpack_field(field: &FieldRegular) -> quote::Tokens {
    let wrappers: Vec<_> = field.serialization_wrappers.iter().rev().cloned().collect();

    let mut unpack = quote! { bytes };

    let mut i = 0;
    loop {
        match (wrappers.get(i), wrappers.get(i+1)) {
            (Some(&SerializationWrapper::EndiannesWrapper { ref endian }), Some(&SerializationWrapper::IntegerWrapper { ref integer })) => {
                
                unpack = quote! {
                    use ::packed_struct::types::*;
                    use ::packed_struct::types::bits::*;

                    let res: Result<#endian <_, _, #integer >, PackingError> = <#endian <_, _, _>>::unpack(& #unpack );
                    let unpacked = try!(res);
                    **unpacked
                };

                i += 1;
            }
            (Some(&SerializationWrapper::PrimitiveEnumWrapper), _) => {
                let ty = &field.ty;
                
                unpack = quote! {
                    use ::packed_struct::PrimitiveEnum;

                    let primitive_integer: u8 = { #unpack };
                    let r = <#ty>::from_primitive(primitive_integer).ok_or(PackingError::InvalidValue);
                    r?
                };
            },
            (Some(&SerializationWrapper::EndiannesWrapper { ref endian }), _) => {
                let integer_ty = &field.ty;

                unpack = quote! {
                    use ::packed_struct::types::*;
                    use ::packed_struct::types::bits::*;

                    let res: Result<#endian <_, _, #integer_ty >, PackingError> = <#endian <_, _, #integer_ty >>::unpack(& #unpack );
                    let unpacked = try!(res);
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
                panic!("unsupported wrappers: {:#?}", wrappers);
            }            
        }

        i += 1;

        if wrappers.len() == 0 || i > wrappers.len() - 1 { break; }
    }

    unpack
}
