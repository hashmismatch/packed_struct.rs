//! Helper structures for runtime packing visualization.

use crate::internal_prelude::v1::*;

/// Per-field formatting of a packed structure. Implemented by `#[derive(PackedStruct)]`.
#[cfg(any(feature="alloc", feature="std"))]
pub trait PackedStructDebug {
    /// Writes a table of the fields, with their bit positions, packed bits and values.
    fn fmt_fields(&self, fmt: &mut Formatter) -> Result<(), FmtError>;
    /// The name and packed size of the structure.
    fn packed_struct_display_header() -> &'static str;
}

/// Formats a range of bits of a byte slice with `{:b}`, MSB0 numbered.
pub struct DebugBinaryByteSlice<'a> {
    /// The bits to format. The `end` of the range is the last bit, inclusive.
    pub bits: &'a Range<usize>,
    /// The packed bytes.
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

/// A field of a packed structure, as shown by [`packable_fmt_fields`].
pub struct DebugBitField<'a> { 
	/// The name of the field.
	pub name: Cow<'a, str>,
	/// The field's MSB0 bit position. The `end` of the range is the last bit, inclusive.
	pub bits: Range<usize>,
	/// The field's value, formatted for display.
	pub display_value: Cow<'a, str>
}


/// Writes a table of the fields, with their bit positions, packed bits and values.
/// The packed bits are omitted if any of the fields is wider than 32 bits.
pub fn packable_fmt_fields(f: &mut Formatter, packed_bytes: &[u8], fields: &[DebugBitField]) -> fmt::Result {
    if fields.is_empty() {
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
                        spaces = max_bit_width - field.bits.len(),
                        name = field.name
                        )?;
        }
    }

    Ok(())
}

/// A `Display` formatter that packs the structure and shows its bytes and fields.
/// Each section can be turned off.
pub struct PackedStructDisplay<'a, P: 'a> {
    /// The structure to display.
    pub packed_struct: &'a P,
    /// Show the structure's name and size.
    pub header: bool,
    /// Show the packed bytes in decimal.
    pub raw_decimal: bool,
    /// Show the packed bytes in hexadecimal.
    pub raw_hex: bool,
    /// Show the packed bytes in binary.
    pub raw_binary: bool,
    /// Show the table of fields.
    pub fields: bool
}

impl<'a, P> PackedStructDisplay<'a, P> {
    /// A formatter with all the sections enabled.
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
            for (i, p) in packed.iter().enumerate().take(l) {
                write!(f, "{}", p)?;
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
            for (i, p) in packed.iter().enumerate().take(l) {
                write!(f, "0x{:X}", p)?;
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

            for (i, p) in packed.iter().enumerate().take(l) {
                write!(f, "0b{:08b}", p)?;
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
