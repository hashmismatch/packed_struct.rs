extern crate quote;
extern crate syn;

use packed_struct::*;

use pack::*;
use pack_parse_attributes::*;


pub fn parse_sub_attributes(attributes: &Vec<syn::Attribute>, main_attribute: &str) -> Vec<(String, String)> {
    let mut r = vec![];

    for attr in attributes {        
        if let &syn::Attribute { value: syn::MetaItem::List(ref ident, ref list), .. } = attr {
            if ident.as_ref() != main_attribute { continue; }

            for item in list {
                if let &syn::NestedMetaItem::MetaItem(syn::MetaItem::NameValue(ref ident, ref lit)) = item {
                    let n = ident.as_ref();
                    
                    if let &syn::Lit::Str(ref v, _) = lit {
                        r.push((n.to_string(), v.to_string()));
                    }
                }
            }
        }
    }

    r
}


/// Also finds #[pack(name="val")]
pub fn get_attribute_value_by_name(attributes: &Vec<syn::Attribute>, name: &str) -> Option<String> {
    for attr in attributes {
        if let &syn::Attribute { value: syn::MetaItem::NameValue(ref ident, ref lit), .. } = attr {
            let n = ident.as_ref();
            if n != name { continue; }

            if let &syn::Lit::Str(ref v, _) = lit {
                return Some(v.to_string());
            }
        }

        if let &syn::Attribute { value: syn::MetaItem::List(ref ident, ref list), .. } = attr {
            if ident.as_ref() != "pack" { continue; }

            for item in list {
                if let &syn::NestedMetaItem::MetaItem(syn::MetaItem::NameValue(ref ident, ref lit)) = item {
                    let n = ident.as_ref();
                    if n != name { continue; }

                    if let &syn::Lit::Str(ref v, _) = lit {
                        return Some(v.to_string());
                    }
                }
            }
        }
    }

    None
}

/*
pub fn get_attribute_value_by_first_name(attributes: &Vec<syn::Attribute>, names: &[&str]) -> Option<String> {
    for name in names.iter().map(|name| get_attribute_value_by_name(attributes, name)) {
        if let Some(name) = name {
            return Some(name);
        }
    }

    None
}
*/

#[derive(Clone, Copy, Debug, PartialEq)]
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


fn get_builtin_type_bit_width(ident: &syn::Ident) -> Option<usize> {

    match ident.as_ref() {
        "bool" => Some(1),
        
        "UIntBits1" => Some(1),
        "UIntBits2" => Some(2),
        "UIntBits3" => Some(3),
        "UIntBits4" => Some(4),
        "UIntBits5" => Some(5),
        "UIntBits6" => Some(6),
        "UIntBits7" => Some(7),

        "u8" | "i8" => Some(8),

        "u16" | "i16" | "MsbU16" | "MsbI16" | "LsbU16" | "LsbI16" => Some(16),
        "u32" | "i32" | "MsbU32" | "MsbI32" | "LsbU32" | "LsbI32" => Some(32),
        "u64" => Some(64),

        _ => None
    }
}


fn get_field_info(field: &syn::Field, default_endianness: Option<IntegerEndianness>) -> Result<FieldInfo, ()> {

    let mut bit_width: Option<usize> = None;
    let mut simple_type = None;

    let mut field_kind = FieldKind::Normal;

    let mut bit_width_builtin: Option<usize> = None;

    match field.ty {
        syn::Ty::Path (None, syn::Path { ref segments, .. }) => {
            if segments.len() == 1 {
                let ref segment = segments[0];
                simple_type = Some(segment.clone());

                bit_width_builtin = get_builtin_type_bit_width(&segment.ident);
            }
        },
        syn::Ty::Array(ref ty, ref size) => {
            if let syn::Ty::Path (None, syn::Path { ref segments, .. }) = **ty {
                if segments.len() == 1 {
                    if let &syn::ConstExpr::Lit(syn::Lit::Int(size, _)) = size {
                        let ref segment = segments[0];
                        if let Some(w) = get_builtin_type_bit_width(&segment.ident) {
                            bit_width_builtin = Some(w * size as usize);
                        }
                        
                        field_kind = FieldKind::Array { size: size as usize, ident: segment.ident.clone() };
                    }
                }
            }
        },
        _ => ()
    };

    let field_attributes = PackFieldAttribute::parse_all(&parse_sub_attributes(&field.attrs, "packed_field"));


    let bits_position = field_attributes.iter().filter_map(|a| match a {
        &PackFieldAttribute::BitPosition(b) | &PackFieldAttribute::BytePosition(b) => Some(b),
        _ => None
    }).next().unwrap_or(BitsPositionParsed::Next);

    if bit_width.is_none() {
        let mult: usize = if let &FieldKind::Array { size, .. } = &field_kind {
            size
        } else {
            1
        };

        // try to get the bit number from an attribute
        if let Some(bits) = field_attributes.iter().filter_map(|a| if let &PackFieldAttribute::SizeBits(bits) = a { Some(bits) } else { None }).next() {
            bit_width = Some(bits * mult);
        } else if let BitsPositionParsed::Range(a, b) = bits_position {
            bit_width = Some((b as isize - a as isize).abs() as usize + 1);
        } else {
            bit_width = bit_width_builtin;
        }
    }

    
    let is_enum_ty = field_attributes.iter().filter_map(|a| match a {
        &PackFieldAttribute::Ty(TyKind::Enum) => Some(()),
        _ => None
    }).next().is_some();
    
    if is_enum_ty {
        if let Some(bit_width) = bit_width {
            if bit_width == 0 || bit_width > 8 {
                panic!("Unsupported enum bit width: {}", bit_width);
            }

            let ty = match bit_width {
                8 => "u8".into(),
                _ => format!("UIntBits{}", bit_width)
            };

            field_kind = FieldKind::Enum { pack_ty: syn::Ident::new(ty) };
        } else {
            panic!("Missing bit width for the enum type field!");
        }        
    }
    
    let mut serialization_wrapper_ty = None;
    let needs_wrap = {
        if let Some(ref simple_type) = simple_type {
            simple_type.ident == syn::Ident::new("u16") ||
            simple_type.ident == syn::Ident::new("i16") ||
            simple_type.ident == syn::Ident::new("u32") ||
            simple_type.ident == syn::Ident::new("i32") ||
            simple_type.ident == syn::Ident::new("u64")
        } else {
            false
        }
    };

    if needs_wrap {

        let endiannes = if let Some(endiannes) = field_attributes
            .iter()
            .filter_map(|a| if let &PackFieldAttribute::IntEndiannes(endiannes) = a {
                                Some(endiannes)
                            } else {
                                None
                            }).next()
        {
            Some(endiannes)
        } else {
            default_endianness
        };        

        if endiannes.is_none() {
            panic!("Missing serialization wrapper for simple type {:?} - did you specify the integer endiannes on the field or a default for the struct?", simple_type);
        }

        let simple_type = simple_type.unwrap();

        let t = match endiannes.unwrap() {
            IntegerEndianness::Msb => format!("Msb{}", simple_type.ident.as_ref().to_uppercase()),
            IntegerEndianness::Lsb => format!("Lsb{}", simple_type.ident.as_ref().to_uppercase()),
        };
        let t = format!("::packed_struct::{}", t);

        serialization_wrapper_ty = Some(syn::Ident::new(t))
    }


    if bit_width.is_none() {
        panic!("unknown bit width for type: {:?}", field.ty);
    }

    match (bit_width_builtin, bit_width) {
        (Some(b1), Some(b2)) => {
            if b1 != b2 {
                panic!("Builtin bit width for the type {:?} is {}, but is specified by attribute as {}. Field: {:?}", field.ty, b1, b2, field);
            }
        },
        (_, _) => ()
    }
    

    Ok(FieldInfo {
        ident: field.ident.as_ref().unwrap().clone(),
        ty: field.ty.clone(),
        serialization_wrapper_ty: serialization_wrapper_ty,
        bit_width: bit_width.unwrap(),
        bits_position: bits_position,
        field_kind: field_kind
    })
} 

#[derive(Copy, Clone, Debug)]
pub enum BitsPositionParsed {
    Next,
    Start(usize),
    Range(usize, usize)
}

impl BitsPositionParsed {
    fn to_bits_position(&self) -> Box<BitsRange> {
        match *self {
            BitsPositionParsed::Next => Box::new(NextBits),
            BitsPositionParsed::Start(s) => Box::new(s),
            BitsPositionParsed::Range(a, b) => Box::new(a..b)
        }
    }
}



pub fn parse_num(s: &str) -> usize {
    let s = s.trim();

    if s.starts_with("0x") || s.starts_with("0X") {
        usize::from_str_radix(&s[2..], 16).expect(&format!("Invalid hex number: {:?}", s))
    } else {
        s.parse().expect(&format!("Invalid decimal number: {:?}", s))
    }
}



pub fn parse_struct(ast: &syn::MacroInput) -> PackStruct {
    let attributes = PackStructAttribute::parse_all(&parse_sub_attributes(&ast.attrs, "packed_struct"));

    let fields: Vec<_> = match ast.body {
        syn::Body::Struct(syn::VariantData::Struct(ref fields)) => {
            fields.iter().collect()
        },
        _ => panic!("#[derive(PackedStruct)] can only be used with braced structs"),
    };

    let bit_positioning = {
        attributes.iter().filter_map(|a| match a {
            &PackStructAttribute::BitNumbering(b) => Some(b),
            _ => None
        }).next()
    };

    let default_int_endianness = attributes.iter().filter_map(|a| match a {
        &PackStructAttribute::DefaultIntEndianness(i) => Some(i),
        _ => None
    }).next();

    let struct_size_bytes = attributes.iter().filter_map(|a| {
        if let &PackStructAttribute::SizeBytes(size_bytes) = a {
            Some(size_bytes)
        } else {
            None
        }}).next();

    let mut fields_expanded = Vec::new();
    {
        let mut prev_bit_range = None;
        for field in &fields {
            let info = get_field_info(field, default_int_endianness).unwrap();
            let bits_position = match (bit_positioning, info.bits_position) {
                (Some(BitNumbering::Lsb0), BitsPositionParsed::Next) | (Some(BitNumbering::Lsb0), BitsPositionParsed::Start(_)) => {
                    panic!("LSB0 field positioning currently requires explicit, full field positions.");
                },
                (Some(BitNumbering::Lsb0), BitsPositionParsed::Range(start, end)) => {
                    if let Some(struct_size_bytes) = struct_size_bytes {
                        BitsPositionParsed::Range( (struct_size_bytes * 8) - 1 - start, (struct_size_bytes * 8) - 1 - end )
                    } else {
                        panic!("LSB0 field positioning currently requires explicit struct byte size.");
                    }
                },

                (None, p @ BitsPositionParsed::Next) => p,
                (Some(BitNumbering::Msb0), p) => p,

                (None, _) => {
                    panic!("Please explicitly specify the bit numbering mode on the struct with an attribute: #[packed_struct(bit_numbering=\"msb0\")] or \"lsb0\".");
                }
            };
            let bit_range = bits_position.to_bits_position().get_bits_range(info.bit_width, &prev_bit_range);

            fields_expanded.push(FieldExpanded {
                info: info,
                bit_range: bit_range.clone(),
                bit_range_rust: bit_range.start..(bit_range.end + 1)
            });

            prev_bit_range = Some(bit_range);
        }        
    }

    let num_bytes: usize = {
        if let Some(struct_size_bytes) = struct_size_bytes {
            struct_size_bytes
        } else {
            let last_bit = fields_expanded.last().unwrap().bit_range_rust.end;
            (last_bit as f32 / 8.0).ceil() as usize
        }
    };

    let num_bits: usize = num_bytes * 8;    

    PackStruct {
        ast: ast.clone(),
        fields: fields_expanded,
        num_bytes: num_bytes,
        num_bits: num_bits
    }
}