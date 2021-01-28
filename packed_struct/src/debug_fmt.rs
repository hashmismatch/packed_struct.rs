//! Helper structures for runtime packing visualization.

use crate::internal_prelude::v1::*;

#[cfg(any(feature="alloc", feature="std"))]
pub trait PackedStructDebug {
    fn fmt_fields(&self, fmt: &mut Formatter) -> Result<(), FmtError>;
    fn packed_struct_display_header() -> &'static str;
}

pub struct DebugBinaryByteSlice<'a> {
    pub bits: &'a Range<usize>,
    pub slice: &'a [u8]
}

impl<'a> fmt::Binary for DebugBinaryByteSlice<'a> {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        for i in self.bits.start..(self.bits.end + 1) {
            let byte = i / 8;
            let bit = i % 8;
            let bit = 7 - bit;

            let src_byte = self.slice[byte];
            let src_bit = (src_byte & (1 << bit)) == (1 << bit);

            let s = if src_bit { "1" } else { "0" };
            fmt.write_str(s)?;
        }

        Ok(())
    }
}

pub struct DebugBitField<'a> { 
	pub name: Cow<'a, str>,
	pub bits: Range<usize>,
	pub display_value: Cow<'a, str>
}


pub fn packable_fmt_fields(f: &mut Formatter, packed_bytes: &[u8], fields: &[DebugBitField]) -> fmt::Result {
    if fields.len() == 0 {
		return Ok(());
	}

    let max_field_length_name = fields.iter().map(|x| x.name.len()).max().unwrap();
	let max_bit_width = fields.iter().map(|x| x.bits.len()).max().unwrap();

    if max_bit_width > 32 {
        for field in fields {
            write!(f, "{name:>0$} | {base_value:?}\r\n",
                        max_field_length_name + 1,
                        base_value = field.display_value,
                        name = field.name
                        )?;
        }
    } else {    
        for field in fields {

            let debug_binary = DebugBinaryByteSlice {
                bits: &field.bits,
                slice: packed_bytes
            };

            write!(f, "{name:>0$} | bits {bits_start:>3}:{bits_end:<3} | 0b{binary_value:>0width_bits$b}{dummy:>0spaces$} | {base_value:?}\r\n",
                        max_field_length_name + 1,
                        base_value = field.display_value,
                        binary_value = debug_binary,
                        dummy = "",
                        bits_start = field.bits.start,
                        bits_end = field.bits.end,
                        width_bits = field.bits.len(),
                        spaces = (max_bit_width - field.bits.len()) as usize,
                        name = field.name
                        )?;
        }
    }

    Ok(())
}

pub struct PackedStructDisplay<'a, P: 'a> {
    pub packed_struct: &'a P,
    pub header: bool,
    pub raw_decimal: bool,
    pub raw_hex: bool,
    pub raw_binary: bool,
    pub fields: bool
}

impl<'a, P> PackedStructDisplay<'a, P> {
    pub fn new(packed_struct: &'a P) -> Self {
        PackedStructDisplay {
            packed_struct,            
            header: true,
            raw_decimal: true,
            raw_hex: true,
            raw_binary: true,
            fields: true
        }
    }
}

use crate::packing::PackedStruct;
use crate::types_bits::ByteArray;

impl<'a, P> fmt::Display for PackedStructDisplay<'a, P> where P: PackedStruct + PackedStructDebug {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let packed = match self.packed_struct.pack() {
            Ok(packed) => packed,
            Err(e) => {
                return f.write_fmt(format_args!("Error while packing: {:?}", e));                
            }
        };
        let packed = packed.as_bytes_slice();
        let l = packed.len();

        if self.header {
            f.write_str(P::packed_struct_display_header())?;
            f.write_str("\r\n")?;
            f.write_str("\r\n")?;
        }

        // decimal
        if self.raw_decimal {
            f.write_str("Decimal\r\n")?;
            f.write_str("[")?;
            for i in 0..l {
                write!(f, "{}", packed[i])?;
                if (i + 1) != l {
                    f.write_str(", ")?;
                }
            }
            f.write_str("]")?;

            f.write_str("\r\n")?;
            f.write_str("\r\n")?;
        }
                        
        // hex
        if self.raw_hex {
            f.write_str("Hex\r\n")?;
            f.write_str("[")?;
            for i in 0..l {
                write!(f, "0x{:X}", packed[i])?;
                if (i + 1) != l {
                    f.write_str(", ")?;
                }
            }
            f.write_str("]")?;
            f.write_str("\r\n")?;
            f.write_str("\r\n")?;
        }

        if self.raw_binary {
            f.write_str("Binary\r\n")?;
            f.write_str("[")?;
            for i in 0..l {
                write!(f, "0b{:08b}", packed[i])?;
                if (i + 1) != l {
                    f.write_str(", ")?;
                }
            }
            f.write_str("]")?;
            f.write_str("\r\n")?;
            f.write_str("\r\n")?;
        }

        if self.fields {
            self.packed_struct.fmt_fields(f)?;
        }
    
        Ok(())
    }
}
