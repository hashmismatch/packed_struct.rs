use quote::quote;
use crate::pack::*;
use crate::pack_codegen_docs::*;
use crate::common::*;
use syn::spanned::Spanned;
use crate::utils::*;


pub fn derive_pack(parsed: &PackStruct) -> syn::Result<proc_macro2::TokenStream> {

    let (impl_generics, ty_generics, where_clause) = parsed.derive_input.generics.split_for_impl();
    let name = &parsed.derive_input.ident;

    let type_documentation = type_docs(parsed);
    let num_bytes = parsed.num_bytes;
    let num_bits = parsed.num_bits;
    

    let mut pack_fields = vec![];
    let mut unpack_fields = vec![];
    let mut unpack_struct_set = vec![];

    for field in &parsed.fields {
        match field {
            FieldKind::Regular { ident, field } => {
                let bits = pack_bits(field, None);

                let pack = pack_field(ident, field);
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
                    let #ident = {
                        let bytes = { #unpack_bits };
                        #unpack
                    };
                });
            },
            FieldKind::Array(array) => {
                let codegen = array_codegen(array)?;

                pack_fields.push(codegen.pack);
                unpack_fields.push(codegen.unpack);
            }
        }

        let ident = match field {
            FieldKind::Regular { ident, .. } => ident,
            FieldKind::Array(array) => &array.ident
        };
        unpack_struct_set.push(quote! {
            #ident
        });
    }

    let result_ty = result_type();

    // a little-endian struct is packed big-endian and then reversed as a whole
    let (pack_reverse, unpack_reverse) = if parsed.little_endian {
        (
            quote! { target.reverse(); },
            quote! { let src: &[u8; #num_bytes] = &{ let mut s = *src; s.reverse(); s }; }
        )
    } else {
        (quote! {}, quote! {})
    };

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

                #pack_reverse

                Ok(target)
            }

            #[inline]
            #[allow(unused_imports, unused_parens)]
            fn unpack(src: &Self::ByteArray) -> #result_ty <#name, ::packed_struct::PackingError> {
                use ::packed_struct::*;

                #unpack_reverse

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



struct ArrayCodegen {
    pack: proc_macro2::TokenStream,
    unpack: proc_macro2::TokenStream
}

/// Packs and unpacks the array with a loop over its elements, so that the amount of
/// generated code doesn't depend on the size of the array.
///
/// The elements only differ in their bit offset. The offset within a byte repeats every
/// `period` elements, so the code is generated once for each of those alignments and
/// selected at runtime, together with the element's starting byte.
fn array_codegen(array: &FieldArray) -> syn::Result<ArrayCodegen> {
    fn gcd(a: usize, b: usize) -> usize {
        if b == 0 { a } else { gcd(b, a % b) }
    }

    let ident = &array.ident;
    let size = array.size;
    let element_ty = &array.element.ty;
    let element_bits = array.element_bits();
    let start_bit = array.element.bit_range.start;

    let period = 8 / gcd(element_bits, 8);
    // bytes taken by `period` elements
    let stride = period * element_bits / 8;

    let base = syn::Ident::new("base", proc_macro2::Span::call_site());
    let chunk = if period == 1 { quote! { i } } else { quote! { (i / #period) } };
    let chunk_offset = if stride == 1 { chunk } else { quote! { #chunk * #stride } };

    let mut pack_arms = vec![];
    let mut unpack_arms = vec![];

    for j in 0..period.min(size) {
        let element_start_bit = start_bit + (j * element_bits);
        let start_byte = element_start_bit / 8;
        let element = array.element.with_start_bit(element_start_bit % 8);

        let base_value = if start_byte == 0 { chunk_offset.clone() } else { quote! { #start_byte + #chunk_offset } };
        let bits = pack_bits(&element, Some(&base));
        let pack = pack_field(&quote! { #ident[e] }, &element);
        let unpack = unpack_field(&element)?;

        let pack_bits = bits.pack;
        let unpack_bits = bits.unpack;

        pack_arms.push(quote! {
            let #base = #base_value;
            let packed = { #pack };
            #pack_bits
        });

        unpack_arms.push(quote! {
            let #base = #base_value;
            let bytes = { #unpack_bits };
            Ok({ #unpack })
        });
    }

    let select_arm = |arms: Vec<proc_macro2::TokenStream>| {
        if arms.len() == 1 {
            return arms.into_iter().next().unwrap();
        }

        let last = arms.len() - 1;
        let arms = arms.iter().enumerate().map(|(j, arm)| {
            if j == last {
                quote! { _ => { #arm } }
            } else {
                quote! { #j => { #arm } }
            }
        });

        quote! {
            match i % #period {
                #(#arms)*
            }
        }
    };

    let pack = select_arm(pack_arms);
    let unpack = select_arm(unpack_arms);
    let result_ty = result_type();

    // `i` is the element's slot in the packed bits. Mirrored arrays store the elements in
    // reverse order, so that they end up in order once the struct's bytes are reversed.
    let (element_index, slot_index) = if array.mirrored {
        (quote! { #size - 1 - i }, quote! { let i = #size - 1 - e; })
    } else {
        (quote! { i }, quote! { let i = e; })
    };

    Ok(ArrayCodegen {
        pack: quote! {
            for i in 0..#size {
                let e = #element_index;
                #pack
            }
        },
        unpack: quote! {
            let #ident: [#element_ty; #size] = ::packed_struct::__private::try_array_from_fn(|e| -> #result_ty <#element_ty, ::packed_struct::PackingError> {
                #slot_index
                #unpack
            })?;
        }
    })
}

struct PackBitsCopy {
    pack: proc_macro2::TokenStream,
    unpack: proc_macro2::TokenStream
}

/// Emits the index of a byte in the packed buffer, relative to the runtime byte `base`, if any.
fn byte_index(base: Option<&syn::Ident>, offset: usize) -> proc_macro2::TokenStream {
    match (base, offset) {
        (None, offset) => quote! { #offset },
        (Some(base), 0) => quote! { #base },
        (Some(base), offset) => quote! { #base + #offset }
    }
}

/// Copies the bits between the packed field and the buffer. When `base` is set, the field's
/// byte positions are emitted relative to it, so that the same code can be reused at
/// different offsets (array elements).
fn pack_bits(field: &FieldRegular, base: Option<&syn::Ident>) -> PackBitsCopy {
    // memcpy
    if (field.bit_range_rust.start % 8) == 0 && (field.bit_range_rust.end % 8) == 0 &&
       (field.bit_range_rust.len() % 8) == 0 && field.bit_range_rust.len() >= 8 
    {
        let start = field.bit_range_rust.start / 8;
        let end = field.bit_range_rust.end / 8;
        let start_index = byte_index(base, start);
        let end_index = byte_index(base, end);
        
        PackBitsCopy {
            pack: quote! {
                target[#start_index..#end_index].copy_from_slice(&packed);
            },
            unpack: quote! {
                let mut b = [0; (#end - #start)];
                b[..].copy_from_slice(&src[#start_index..#end_index]);
                b
            }
        }
    } else {
        let packed_field_len = (field.bit_width as f32 / 8.0).ceil() as usize; 
        let start_byte = (field.bit_range_rust.start as f32 / 8.0).floor() as usize;
        let shift = ((packed_field_len as isize*8) - (field.bit_width as isize)) - (field.bit_range_rust.start as isize - (start_byte as isize * 8));

        let emit_shift = |s: isize| {
            match s {
                0 => quote! {},
                _ if s > 0 => quote! { << #s },
                _ => {
                    let s = -s;
                    quote! { >> #s }
                }
            }
        };

        let mut l = 8 - ((packed_field_len as isize*8) - field.bit_width as isize);

        let mut pack = vec![];
        let mut unpack = vec![];

        for (i, dst_byte) in (start_byte..start_byte + packed_field_len).enumerate() {
            let dst_index = byte_index(base, dst_byte);
            let dst_next_index = byte_index(base, dst_byte + 1);
            // `l` grows by 8 per byte, casting it to u8 would wrap for fields of 32+ bytes
            let src_mask = if l >= 8 { 0xFF } else { ones_u8(l as u8) };
            let bit_shift = emit_shift(shift);
            pack.push(quote! {
                let _a = #i;
                target[#dst_index] |= (packed[#i] & #src_mask) #bit_shift;  
            });
            
            let bit_shift = emit_shift(-shift);
            unpack.push(quote! {
                let _a = #i;
                b[#i] |= (src[#dst_index] #bit_shift) & #src_mask;
            });

            if shift < 0 && (dst_byte - start_byte) <= packed_field_len {
                let shift = 8+shift;
                let src_mask = ones_u8(8-shift as u8);

                let bit_shift = emit_shift(shift);                
                pack.push(quote! {
                    let _b = #i;
                    target[#dst_next_index] |= (((packed[#i] & #src_mask) as u16) #bit_shift) as u8;  
                });

                let bit_shift = emit_shift(-shift);
                unpack.push(quote! {
                    let _b = #i;
                    b[#i] |= (((src[#dst_next_index] as u16) #bit_shift) & #src_mask as u16) as u8;
                });
            } else if shift > 0 && (dst_byte - start_byte) <= packed_field_len && i < packed_field_len - 1 {
                let shift = -(8-shift);
                let bit_shift = emit_shift(shift);
                let src_mask = !ones_u8(-shift as u8);

                pack.push(quote! {
                    let _c = #i;
                    target[#dst_index] |= (((packed[#i + 1] & #src_mask) as u16) #bit_shift) as u8;  
                });

                let bit_shift = emit_shift(-shift);
                unpack.push(quote! {
                    let _c = #i;
                    b[#i + 1] |= (((src[#dst_index] as u16) #bit_shift) & #src_mask as u16) as u8;
                });
            }

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
            SerializationWrapper::PrimitiveEnum => {
                output = quote! {
                    {
                        use ::packed_struct::PrimitiveEnum;

                        let primitive_integer = { #output }.to_primitive();
                        primitive_integer
                    }
                };
            },
            SerializationWrapper::Integer { integer } => {
                output = quote! {
                    {
                        use ::packed_struct::types::*;
                        use ::packed_struct::types::bits::*;                        

                        let sized_integer: #integer = { #output }.into();
                        sized_integer
                    }
                };
            },
            SerializationWrapper::Endiannes { endian } => {
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

    if field.reverse_bytes {
        quote! {
            {
                let mut packed = (& #output).pack()?;
                ::packed_struct::types::bits::ByteArray::as_mut_bytes_slice(&mut packed).reverse();
                packed
            }
        }
    } else {
        quote! {
            {
                (& #output).pack()?
            }
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
            (Some(SerializationWrapper::Endiannes { endian }), Some(SerializationWrapper::Integer { integer })) => {
                
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
            (Some(SerializationWrapper::Endiannes { endian }), _) => {
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
                unpack = if field.reverse_bytes {
                    quote! {
                        let mut reversed = #unpack;
                        reversed.reverse();
                        <#ty>::unpack(&reversed)?
                    }
                } else {
                    quote! {
                        <#ty>::unpack(& #unpack)?
                    }
                };
            },
            (_, _) => {
                return Err(syn::Error::new(field.ty.span(), "Unsupported serialization wrappers encountered!"));
            }            
        }

        i += 1;

        if wrappers.is_empty() || i > wrappers.len() - 1 { break; }
    }

    Ok(unpack)
}