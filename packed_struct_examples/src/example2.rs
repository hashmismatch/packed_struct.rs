//! Documentation example for the header and footer attributes and traits

use packed_struct::{prelude::*, PackedStructHeader, PackedStructFooter};

// Sometimes you want a static header or footer on a packed struckt. For example when you want to send the packed struct as a command via serial and you need the command to have a header
#[derive(PackedStruct, Debug, Copy, Clone, PartialEq)]
#[packed_struct(bit_numbering = "msb0", endian = "msb", header = [0x80, 0x60], footer = 0x16)]
pub struct FunctionCommand {
    #[packed_field(bytes = "0..=1")]
    pub address: u16,
    #[packed_field(bits = "16")]
    pub f1: bool,
    #[packed_field(bits = "17")]
    pub f2: bool,
    #[packed_field(bits = "18")]
    pub f3: bool,
    #[packed_field(bits = "19")]
    pub f4: bool,
    #[packed_field(bits = "20")]
    pub f5: bool,
    #[packed_field(bits = "21")]
    pub f6: bool,
    #[packed_field(bits = "22")]
    pub f7: bool,
    #[packed_field(bits = "23")]
    pub f8: bool,
}

#[test]
fn test_static_header_and_footer() {
    let fun = FunctionCommand {
        address: 1,
        f1: true,
        f2: false,
        f3: true,
        f4: false,
        f5: false,
        f6: true,
        f7: false,
        f8: true
    };

    let packed = fun.pack().unwrap();
    assert_eq!(&packed, &[0x80 as u8, 0x60, 0x00, 0x01, 0b10100101, 0x16]);

    let _unpacked = FunctionCommand::unpack(&[0x80 as u8, 0x60, 0x00, 0x01, 0b10100101, 0x00]).unwrap();

    println!("{}", fun);
}

// Other times you might need to specify the header dynamically, for example if the header is different if all fields are false. You might also want to make sure your struct gets received correctly put an xor value in the footer
#[derive(PackedStruct, Debug, Clone, PartialEq)]
#[packed_struct(bit_numbering = "msb0", endian = "msb", header_size = 1, footer_size = 1)]
pub struct DynamicCommand {
    #[packed_field(bits = "0")]
    pub f1: bool,
    #[packed_field(bits = "1")]
    pub f2: bool,
    #[packed_field(bits = "2")]
    pub f3: bool,
    #[packed_field(bits = "3")]
    pub f4: bool,
    #[packed_field(bits = "4")]
    pub f5: bool,
    #[packed_field(bits = "5")]
    pub f6: bool,
    #[packed_field(bits = "6")]
    pub f7: bool,
    #[packed_field(bits = "7")]
    pub f8: bool,
}

impl PackedStructHeader for DynamicCommand {
    type HeaderByteArray = [u8; 1];

    fn get_header(&self, data: &[u8]) -> packed_struct::PackingResult<Self::HeaderByteArray> {
        let header = if data[1] == 0 {
            [0x00]
        } else {
            [0x80]
        };
        Ok(header)
    }
}

impl PackedStructFooter for DynamicCommand {
    type FooterByteArray = [u8; 1];

    fn get_footer(&self, data: &[u8]) -> packed_struct::PackingResult<Self::FooterByteArray> {
        // Note that the header is already part of the data at this point
        let mut xor: u8 = 0;
        data.into_iter().for_each(|value| xor ^= value);
        Ok([xor])
    }

    fn validate_footer(src: &[u8]) -> packed_struct::PackingResult<()> {
        let mut xor: u8 = 0;
        src[0..src.len()].into_iter().for_each(|value| xor ^= value);
        if src.ends_with(&[xor]) {
            Ok(())
        } else {
            Err(PackingError::UserError(format!("Invalid xor: {} to {:?}", xor, src.last().unwrap())))
        }
    }
}

#[test]
fn test_dynamic_header_and_footer() {
    let dy = DynamicCommand {
        f1: true,
        f2: false,
        f3: true,
        f4: false,
        f5: false,
        f6: true,
        f7: false,
        f8: true
    };

    let packed = dy.pack().unwrap();
    assert_eq!(&packed, &[0x80 as u8,0b10100101, 0x25]);

    // We've set the footer to zero so it will fail to unpack
    let unpacked_result = DynamicCommand::unpack(&[0x80 as u8, 0b10100101, 0x00]);

    assert_eq!(unpacked_result, Err(PackingError::UserError("Invalid xor: 37 to 0".to_owned())));

    println!("{}", dy);
}