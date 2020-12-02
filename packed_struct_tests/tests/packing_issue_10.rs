extern crate packed_struct;
#[macro_use]
extern crate packed_struct_codegen;

#[macro_use]
extern crate error_chain;

mod errors {
    error_chain! {
        foreign_links {
            PackedStruct(::packed_struct::PackingError);
        }
    }
}

use errors::*;

use packed_struct::prelude::*;

#[derive(Debug, PackedStruct)]
#[packed_struct(bit_numbering = "msb0")]
pub struct LedDriverConfig {
    #[packed_field(bits = "46:47")]
    lodvth: Integer<u8, ::packed_bits::Bits2>,
    #[packed_field(bits = "44:45")]
    sel_td0: Integer<u8, ::packed_bits::Bits2>,
    #[packed_field(bits = "43")]
    sel_gdly: Integer<u8, ::packed_bits::Bits1>,
    #[packed_field(bits = "42")]
    xrefresh: Integer<u8, ::packed_bits::Bits1>,
    #[packed_field(bits = "41")]
    sel_gck_edge: Integer<u8, ::packed_bits::Bits1>,
    #[packed_field(bits = "40")]
    sel_pchg: Integer<u8, ::packed_bits::Bits1>,
    #[packed_field(bits = "39")]
    espwm: Integer<u8, ::packed_bits::Bits1>,
    #[packed_field(bits = "38")]
    lgse3: Integer<u8, ::packed_bits::Bits1>,
    #[packed_field(bits = "37")]
    sel_sck_edge: Integer<u8, ::packed_bits::Bits1>,
    #[packed_field(bits = "34:36")]
    lgse1: Integer<u8, ::packed_bits::Bits3>,
    #[packed_field(bits = "25:33", endian = "msb")]
    ccb: Integer<u16, ::packed_bits::Bits9>,
    #[packed_field(bits = "16:24", endian = "msb")]
    ccg: Integer<u16, ::packed_bits::Bits9>,
    #[packed_field(bits = "7:15", endian = "msb")]
    ccr: Integer<u16, ::packed_bits::Bits9>,
    #[packed_field(bits = "4:6")]
    bc: Integer<u8, ::packed_bits::Bits3>,
    #[packed_field(bits = "3")]
    poker_trans_mode: Integer<u8, ::packed_bits::Bits1>,
    #[packed_field(bits = "0:2")]
    lgse2: Integer<u8, ::packed_bits::Bits3>,
}

#[test]
#[cfg(test)]
fn test_packed_struct_issue_10() {
    run().unwrap();
}

fn run() -> Result<()> {
    let data = [0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF];
    let _unpacked = LedDriverConfig::unpack(&data)
        .chain_err(|| "unable to unpack LED driver config")?;

    Ok(())
}