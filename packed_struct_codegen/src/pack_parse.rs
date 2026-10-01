use crate::pack::*;
use crate::pack_parse_attributes::*;

use syn::Meta;
use syn::Token;
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use crate::utils::*;

use std::ops::Range;

use crate::utils_syn::{get_expr_int_val, get_single_segment, tokens_to_string};

pub fn parse_sub_attributes(attributes: &[syn::Attribute], main_attribute: &str, wrong_attribute: &str) -> syn::Result<Vec<(String, String)>> {
    let mut r = vec![];

    for attr in attributes {
        if attr.path().is_ident(wrong_attribute) {
            return Err(syn::Error::new(attr.path().span(), format!("This attribute is not supported here, did you mean {:?}?", main_attribute)));
        }

        if attr.path().is_ident(main_attribute) {
            let nested = attr.parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated);
            let nested = if let Ok(nested) = nested {
                nested
            } else {
                continue;
            };
            for meta in nested {
                match meta {            
                    syn::Meta::Path(_) => (),
                    syn::Meta::List(_) => (),
                    syn::Meta::NameValue(nv) => {
                        if let (Some(key), syn::Expr::Lit(lit)) = (nv.path.get_ident(), &nv.value) {
                            if let syn::Lit::Str(lit) = &lit.lit {
                                r.push((key.to_string(), lit.value()));
                            }
                        }
                    }
                }
            }
        }
    }

    Ok(r)
}


#[derive(Clone, Copy, Debug, PartialEq, Eq)]
/// https://en.wikipedia.org/wiki/Bit_numbering
pub enum BitNumbering {
    Lsb0,
    Msb0
}

impl BitNumbering {
    pub fn from_str(s: &str) -> Option<Self> {
        let s = s.to_lowercase();
        match s.as_str() {
            "lsb0" => Some(BitNumbering::Lsb0),
            "msb0" => Some(BitNumbering::Msb0),
            _ => None
        }
    }
}


#[derive(Clone, Copy, Debug)]
/// https://en.wikipedia.org/wiki/Endianness
pub enum IntegerEndianness {
    Msb,
    Lsb
}

impl IntegerEndianness {
    pub fn from_str(s: &str) -> Option<Self> {
        let s = s.to_lowercase();
        match s.as_str() {
            "lsb" | "le" => Some(IntegerEndianness::Lsb),
            "msb" | "be" => Some(IntegerEndianness::Msb),
            _ => None
        }
    }
}


fn get_builtin_type_bit_width(p: &syn::PathSegment) -> syn::Result<Option<usize>> {
    match p.ident.to_string().as_str() {
        "bool" => Ok(Some(1)),
        "u8" | "i8" => Ok(Some(8)),
        "u16" | "i16" => Ok(Some(16)),
        "u32" | "i32" => Ok(Some(32)),
        "u64" | "i64" => Ok(Some(64)),
        "ReservedZero" | "ReservedZeroes" | "ReservedOne" | "ReservedOnes" |
        "Integer" => {
            match p.arguments {
                ::syn::PathArguments::AngleBracketed(ref args) => {
                    for t in &args.args {
                        if let syn::GenericArgument::Type(ty) = t {                            
                            let ty_str = tokens_to_string(ty);                            
                            let p = " Bits ";
                            if let Some(bits_pos) = ty_str.find(p) {
                                let ty_start = &ty_str[(bits_pos+p.len())..];                                
                                let start = ty_start.find(|p: char| p.is_numeric());
                                if let Some(start) = start {
                                    let num_start = &ty_start[start..];                                    
                                    let end = num_start.find(|p: char| !p.is_numeric());
                                    if let Some(end) = end {
                                        let num = &num_start[..end];
                                        if let Ok(bits) = num.parse::<usize>() {
                                            return Ok(Some(bits));
                                        }
                                    }
                                }
                            }
                        }
                    }

                    Ok(None)
                },
                _ => Ok(None)
            }
        },
        _ => {
            Ok(None)
        }
    }
}


fn get_field_mid_positioning(field: &syn::Field) -> syn::Result<FieldMidPositioning> {
    
    let mut array_size = 1;
    let bit_width_builtin: Option<usize>;

    let _ty = match &field.ty {
        syn::Type::Path(type_path) => {
            let segment = get_single_segment(type_path)?;

            bit_width_builtin = get_builtin_type_bit_width(segment)?;
            segment.clone()
        },
        syn::Type::Array(type_array) => {
            
            let path = match *type_array.elem {
                syn::Type::Path(ref p) => p,
                _ => return Err(syn::Error::new(type_array.elem.span(), "Unknown array path type"))
            };

            let segment = get_single_segment(path)?;
            
            bit_width_builtin = get_builtin_type_bit_width(segment)?;
            let size = get_expr_int_val(&type_array.len)?;

            if size == 0 { 
                return Err(syn::Error::new(type_array.len.span(), "Arrays sized 0 are not supported."));
            }            
            
            array_size = size;

            segment.clone()
        },
        _ => { return Err(syn::Error::new(field.ty.span(), "Unsupported type")); }
    };

    let field_attributes = PackFieldAttribute::parse_all(&parse_sub_attributes(&field.attrs, "packed_field", "packed_struct")?);

    let bits_position = field_attributes.iter().filter_map(|a| match a {
        &PackFieldAttribute::BitPosition(b) | &PackFieldAttribute::BytePosition(b) => Some(b),
        _ => None
    }).next().unwrap_or(BitsPositionParsed::Next);

    let bit_width = if let Some(bits) = field_attributes.iter().filter_map(|a| if let PackFieldAttribute::SizeBits(bits) = *a { Some(bits) } else { None }).next() {
        if array_size > 1 {
            return Err(syn::Error::new(field.span(), "Please use the 'element_size_bits' or 'element_size_bytes' for arrays."));
        }
        bits
    } else if let Some(bits) = field_attributes.iter().filter_map(|a| if let PackFieldAttribute::ElementSizeBits(bits) = *a { Some(bits) } else { None }).next() {
        bits * array_size
    } else if let BitsPositionParsed::Range(a, b) = bits_position {
        (b as isize - a as isize).unsigned_abs() + 1
    } else if let Some(bit_width_builtin) = bit_width_builtin {
        // todo: is it even possible to hit this branch?
        bit_width_builtin * array_size
    } else {
        return Err(syn::Error::new(field.span(), "Couldn't determine the bit/byte width for this field."));
    };

    if let BitsPositionParsed::Range(a, b) = bits_position {
        let range_width = (b as isize - a as isize).unsigned_abs() + 1;
        if range_width != bit_width {
            return Err(syn::Error::new(field.span(), format!("The field's position covers {} bits, but its size is {} bits.", range_width, bit_width)));
        }
    }

    Ok(FieldMidPositioning {
        bit_width,
        bits_position
    })
}


fn parse_field(field: &syn::Field, mp: &FieldMidPositioning, bit_range: &Range<usize>, default_endianness: Option<IntegerEndianness>, little_endian: bool) -> syn::Result<FieldKind> {

    match &field.ty {
        syn::Type::Path(_) => {
            return Ok(
                FieldKind::Regular {
                    field: Box::new(parse_reg_field(field, &field.ty, bit_range, default_endianness, little_endian)?),
                    ident: field.ident.clone().ok_or_else(|| syn::Error::new(field.span(), "Missing ident!"))?
                }
            );
        },
        syn::Type::Array(type_array) => {

            let size = get_expr_int_val(&type_array.len)?;

            if mp.bit_width < size || (mp.bit_width % size) != 0 {
                return Err(syn::Error::new(type_array.span(), format!("The array's {} bits can't be evenly split into {} elements.", mp.bit_width, size)));
            }
            let element_size_bits = mp.bit_width / size;

            // all the elements share the type and width, so only the first one is parsed
            let first_element_bit_range = bit_range.start..(bit_range.start + element_size_bits - 1);
            let element = parse_reg_field(field, &type_array.elem, &first_element_bit_range, default_endianness, little_endian)?;

            return Ok(FieldKind::Array(Box::new(FieldArray {
                ident: field.ident.clone().ok_or_else(|| syn::Error::new(field.span(), "Missing ident!"))?,
                size,
                element,
                mirrored: little_endian
            })));
        },
        _ => ()
    };

    Err(syn::Error::new(field.span(), "Field not supported."))
}

/// With `little_endian`, the struct is a single little-endian integer. The derive packs it big-endian
/// and reverses all of its bytes, so the integer fields use the opposite endianness internally and
/// the bytes of the other fields are reversed.
fn parse_reg_field(field: &syn::Field, ty: &syn::Type, bit_range: &Range<usize>, default_endianness: Option<IntegerEndianness>, little_endian: bool) -> syn::Result<FieldRegular> {
    
    let mut wrappers = vec![];

    let bit_width = (bit_range.end - bit_range.start) + 1;
    
    let ty_str = tokens_to_string(ty);
    let field_attributes = PackFieldAttribute::parse_all(&parse_sub_attributes(&field.attrs, "packed_field", "packed_struct")?);


    let is_enum_ty = field_attributes.iter().filter_map(|a| match *a {
        PackFieldAttribute::Ty(TyKind::Enum) => Some(()),
        _ => None
    }).next().is_some();

    let needs_int_wrap = {
        let int_types = ["u8", "i8", "u16", "i16", "u32", "i32", "u64", "i64"];
        is_enum_ty || int_types.iter().any(|t| t == &ty_str)
    };

    let needs_endiannes_wrap = {
        let our_int_ty = ty_str.starts_with("Integer < ") && ty_str.contains("Bits");
        our_int_ty || needs_int_wrap
    };

    if is_enum_ty {
        wrappers.push(SerializationWrapper::PrimitiveEnum);
    }

    if needs_int_wrap {
        let ty = if is_enum_ty {
            format!("<{} as PrimitiveEnum>::Primitive", tokens_to_string(ty))
        } else {
            ty_str.clone()
        };
        let integer_wrap_ty = syn::parse_str(&format!("Integer<{}, Bits::<{}>>", ty, bit_width))?;
        wrappers.push(SerializationWrapper::Integer { integer: integer_wrap_ty });
    }

    if needs_endiannes_wrap {
        let mut endiannes = if let Some(endiannes) = field_attributes
            .iter()
            .filter_map(|a| if let PackFieldAttribute::IntEndiannes(endiannes) = a {
                                Some(*endiannes)
                            } else {
                                None
                            }).next()
        {
            Some(endiannes)
        } else {
            default_endianness
        };

        if little_endian {
            endiannes = match endiannes {
                Some(IntegerEndianness::Lsb) => Some(IntegerEndianness::Msb),
                Some(IntegerEndianness::Msb) if bit_width > 8 => {
                    if (bit_range.start % 8) != 0 || (bit_width % 8) != 0 {
                        return Err(syn::Error::new(field.span(), "Big-endian fields in a byte_order=\"lsb\" structure have to start and end on a byte boundary."));
                    }
                    Some(IntegerEndianness::Lsb)
                },
                e => e
            };
        }

        if bit_width <= 8 {
            endiannes = Some(IntegerEndianness::Msb);
        }

        if endiannes.is_none() {
            panic!("Missing serialization wrapper for simple type {:?} - did you specify the integer endiannes on the field or a default for the struct?", ty_str);
        }

        let ty_prefix = match endiannes.unwrap() {
            IntegerEndianness::Msb => "Msb",
            IntegerEndianness::Lsb => "Lsb"
        };

        let endiannes_wrap_ty = syn::parse_str(&format!("{}Integer", ty_prefix)).unwrap();
        wrappers.push(SerializationWrapper::Endiannes { endian: endiannes_wrap_ty });
    }

    Ok(FieldRegular {
        ty: ty.clone(),
        serialization_wrappers: wrappers,
        bit_width,
        reverse_bytes: little_endian && !needs_endiannes_wrap,
        bit_range: bit_range.clone(),
        bit_range_rust: bit_range.start..(bit_range.end + 1)
    })
}



#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum BitsPositionParsed {
    Next,
    Start(usize),
    Range(usize, usize)
}

impl BitsPositionParsed {
    fn to_bits_position(self) -> Box<dyn BitsRange> {
        match self {
            BitsPositionParsed::Next => Box::new(NextBits),
            BitsPositionParsed::Start(s) => Box::new(s),
            BitsPositionParsed::Range(a, b) => Box::new(a..b)
        }
    }

    pub fn range_in_order(a: usize, b: usize) -> Self {
        BitsPositionParsed::Range(::std::cmp::min(a, b), ::std::cmp::max(a, b))
    }
}



pub fn parse_num(s: &str) -> Result<usize, String> {
    let s = s.trim();

    if s.starts_with("0x") || s.starts_with("0X") {
        usize::from_str_radix(&s[2..], 16).map_err(|e| { format!("Invalid hex number: {:?}, parse error: {:?}", s, e) })
    } else {
        s.parse().map_err(|e| format!("Invalid decimal number: {:?}, parse error: {:?}", s, e))
    }
}



pub fn parse_struct(ast: &syn::DeriveInput) -> syn::Result<PackStruct<'_>> {
    let attributes = PackStructAttribute::parse_all(&parse_sub_attributes(&ast.attrs, "packed_struct", "packed_field")?);

    let data_struct = match &ast.data {
        syn::Data::Struct(data) => data,
        _ => return Err(syn::Error::new(ast.span(), "#[derive(PackedStruct)] can only be used with braced structs"))
    };
    let fields: Vec<_> = data_struct.fields.iter().collect();

    if !ast.generics.params.is_empty() {
        return Err(syn::Error::new(ast.span(), "Structures with generic fields currently aren't supported."));
    }

    let bit_positioning = {
        attributes.iter().filter_map(|a| match *a {
            PackStructAttribute::BitNumbering(b) => Some(b),
            _ => None
        }).next()
    };

    let byte_order = attributes.iter().filter_map(|a| match *a {
        PackStructAttribute::ByteOrder(b) => Some(b),
        _ => None
    }).next();

    if byte_order.is_none() {
        // parse_all() skips the values it can't parse, report them here
        let raw = parse_sub_attributes(&ast.attrs, "packed_struct", "packed_field")?;
        if let Some((_, val)) = raw.iter().find(|(name, _)| name == "byte_order") {
            return Err(syn::Error::new(ast.ident.span(), format!("Invalid byte_order value: {}. Use \"msb\" or \"lsb\".", val)));
        }
    }

    let little_endian = matches!(byte_order, Some(IntegerEndianness::Lsb));

    if little_endian && bit_positioning != Some(BitNumbering::Lsb0) {
        return Err(syn::Error::new(ast.ident.span(), "byte_order=\"lsb\" requires bit_numbering=\"lsb0\": the bits are numbered from the least significant bit of the little-endian integer."));
    }

    let default_int_endianness = attributes.iter().filter_map(|a| match *a {
        PackStructAttribute::DefaultIntEndianness(i) => Some(i),
        _ => None
    }).next().or(if little_endian { Some(IntegerEndianness::Lsb) } else { None });

    let struct_size_bytes = attributes.iter().filter_map(|a| {
        if let PackStructAttribute::SizeBytes(size_bytes) = *a {
            Some(size_bytes)
        } else {
            None
        }
    }).next();

    let first_field_is_auto_positioned = {
        if let Some(field) = fields.first() {
            let mp = get_field_mid_positioning(field)?;
            mp.bits_position == BitsPositionParsed::Next
        } else {
            false
        }
    };

    let mut fields_parsed: Vec<FieldKind> = vec![];
    {
        let mut prev_bit_range = None;
        for field in &fields {
            let mp = get_field_mid_positioning(field)?;
            let bits_position = match (bit_positioning, mp.bits_position) {
                (Some(BitNumbering::Lsb0), BitsPositionParsed::Next) | (Some(BitNumbering::Lsb0), BitsPositionParsed::Start(_)) => {
                    return Err(syn::Error::new(field.span(), "LSB0 field positioning currently requires explicit, full field positions."));
                },
                (Some(BitNumbering::Lsb0), BitsPositionParsed::Range(start, end)) => {
                    if let Some(struct_size_bytes) = struct_size_bytes {
                        BitsPositionParsed::range_in_order( (struct_size_bytes * 8) - 1 - start, (struct_size_bytes * 8) - 1 - end )
                    } else {
                        return Err(syn::Error::new(field.span(), "LSB0 field positioning currently requires explicit struct byte size."));
                    }
                },

                (None, p @ BitsPositionParsed::Next) => p,
                (Some(BitNumbering::Msb0), p) => p,

                (None, _) => {
                    return Err(syn::Error::new(field.span(), "Please explicitly specify the bit numbering mode on the struct with an attribute: #[packed_struct(bit_numbering=\"msb0\")] or \"lsb0\"."));
                }
            };
            let bit_range = bits_position.to_bits_position().get_bits_range(mp.bit_width, &prev_bit_range);

            fields_parsed.push(parse_field(field, &mp, &bit_range, default_int_endianness, little_endian)?);

            prev_bit_range = Some(bit_range);
        }
    }

    let num_bits: usize = {
        if let Some(struct_size_bytes) = struct_size_bytes {
            struct_size_bytes * 8
        } else {
            fields_parsed.iter().map(|f| match f {
                FieldKind::Regular { field, .. } => field.bit_range_rust.end,
                FieldKind::Array(array) => array.bit_range().end + 1
            }).max().unwrap()
        }
    };

    let num_bytes = (num_bits as f32 / 8.0).ceil() as usize;

    if first_field_is_auto_positioned && (num_bits % 8) != 0 && struct_size_bytes.is_none() {
        return Err(syn::Error::new(fields[0].span(), "Please explicitly position the bits of the first field of this structure, as the alignment isn't obvious to the end user."));
    }

    // check for overlaps
    {
        // the name of the field (or array element) that occupies the given bit
        let field_name_at = |field: &FieldKind, bit: usize| match field {
            FieldKind::Regular { ident, .. } => ident.to_string(),
            FieldKind::Array(array) => {
                let i = (bit - array.element.bit_range.start) / array.element_bits();
                format!("{}[{}]", array.ident, i)
            }
        };

        let mut bits: Vec<Option<&FieldKind>> = vec![None; num_bytes * 8];
        for field in &fields_parsed {
            let range = match field {
                FieldKind::Regular { field, .. } => field.bit_range.clone(),
                FieldKind::Array(array) => array.bit_range()
            };

            for i in range.start .. (range.end+1) {
                if let Some(Some(other)) = bits.get(i) {
                    let name = field_name_at(field, i);
                    return Err(syn::Error::new(name.span(), format!("Overlap in bits between fields {} and {}", field_name_at(other, i), name)));
                }

                bits[i] = Some(field);
            }
        }
    }
    
    Ok(PackStruct {
        derive_input: ast,
        fields: fields_parsed,
        num_bytes,
        num_bits,
        little_endian
    })
}
