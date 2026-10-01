//! The [LIFX LAN protocol](https://lan.developer.lifx.com/docs/encoding-a-packet) header and the
//! `SetColor` message.
//!
//! LIFX, like most formats described as C structs with bitfields, numbers the bits inside
//! little-endian words: the first 8 bytes of the header are a little-endian `u64`, with the 12-bit
//! `protocol` field at bits 16 to 27. On the wire, that field is split around the flags next to it:
//! its low 8 bits are byte 2, and its high 4 bits are the *low* nibble of byte 3.
//!
//! With `byte_order="lsb"`, a structure is packed as a single little-endian integer and its fields
//! are positioned with the bit numbers from the specification.

use packed_struct::prelude::*;

/// Frame header, 8 bytes.
#[derive(PackedStruct, Debug, Copy, Clone, PartialEq, Default)]
#[packed_struct(bit_numbering="lsb0", size_bytes="8", byte_order="lsb")]
pub struct FrameHeader {
    /// Size of the entire message in bytes, including this field
    #[packed_field(bits="15:0")]
    pub size: u16,
    /// Protocol number, must be 1024
    #[packed_field(bits="27:16")]
    pub protocol: Integer<u16, packed_bits::Bits::<12>>,
    /// Message includes a target address, must be true
    #[packed_field(bits="28")]
    pub addressable: bool,
    /// Determines the usage of the target field
    #[packed_field(bits="29")]
    pub tagged: bool,
    /// Message origin indicator, must be 0
    #[packed_field(bits="31:30")]
    pub origin: Integer<u8, packed_bits::Bits::<2>>,
    /// Source identifier, chosen by the client and echoed in the responses
    #[packed_field(bits="63:32")]
    pub source: u32
}

/// Frame address, 16 bytes.
#[derive(PackedStruct, Debug, Copy, Clone, PartialEq, Default)]
#[packed_struct(bit_numbering="lsb0", size_bytes="16", byte_order="lsb")]
pub struct FrameAddress {
    /// The device's MAC address followed by two zero bytes, or all zeroes for all the devices.
    /// The first element is the first byte on the wire.
    #[packed_field(bits="63:0")]
    pub target: [u8; 8],
    #[packed_field(bits="111:64")]
    pub _reserved_1: ReservedZero<packed_bits::Bits::<48>>,
    /// A state response message is required
    #[packed_field(bits="112")]
    pub res_required: bool,
    /// An acknowledgement message is required
    #[packed_field(bits="113")]
    pub ack_required: bool,
    #[packed_field(bits="119:114")]
    pub _reserved_2: ReservedZero<packed_bits::Bits::<6>>,
    /// Wrap-around message sequence number
    #[packed_field(bits="127:120")]
    pub sequence: u8
}

/// Protocol header, 12 bytes.
#[derive(PackedStruct, Debug, Copy, Clone, PartialEq, Default)]
#[packed_struct(bit_numbering="lsb0", size_bytes="12", byte_order="lsb")]
pub struct ProtocolHeader {
    #[packed_field(bits="63:0")]
    pub _reserved_1: ReservedZero<packed_bits::Bits::<64>>,
    /// Message type, determines the payload
    #[packed_field(bits="79:64")]
    pub pkt_type: u16,
    #[packed_field(bits="95:80")]
    pub _reserved_2: ReservedZero<packed_bits::Bits::<16>>
}

/// The complete header, 36 bytes. The parts start on byte boundaries, so they are
/// simply placed one after another.
#[derive(PackedStruct, Debug, Copy, Clone, PartialEq, Default)]
#[packed_struct(bit_numbering="msb0")]
pub struct Header {
    #[packed_field(bytes="0..=7")]
    pub frame_header: FrameHeader,
    #[packed_field(bytes="8..=23")]
    pub frame_address: FrameAddress,
    #[packed_field(bytes="24..=35")]
    pub protocol_header: ProtocolHeader
}

/// `SetColor` payload, message type 102, 13 bytes.
#[derive(PackedStruct, Debug, Copy, Clone, PartialEq, Default)]
#[packed_struct(bit_numbering="lsb0", size_bytes="13", byte_order="lsb")]
pub struct SetColor {
    #[packed_field(bits="7:0")]
    pub _reserved: ReservedZero<packed_bits::Bits::<8>>,
    /// 0 to 65535, scaled to 0° to 360°
    #[packed_field(bits="23:8")]
    pub hue: u16,
    /// 0 to 65535, scaled to 0% to 100%
    #[packed_field(bits="39:24")]
    pub saturation: u16,
    /// 0 to 65535, scaled to 0% to 100%
    #[packed_field(bits="55:40")]
    pub brightness: u16,
    /// Color temperature, 2500 K to 9000 K
    #[packed_field(bits="71:56")]
    pub kelvin: u16,
    /// Transition time in milliseconds
    #[packed_field(bits="103:72")]
    pub duration: u32
}

impl SetColor {
    /// The message type of `SetColor`.
    pub const PKT_TYPE: u16 = 102;
}

/// A `SetColor` message, 49 bytes.
#[derive(PackedStruct, Debug, Copy, Clone, PartialEq, Default)]
#[packed_struct(bit_numbering="msb0")]
pub struct SetColorMessage {
    #[packed_field(bytes="0..=35")]
    pub header: Header,
    #[packed_field(bytes="36..=48")]
    pub payload: SetColor
}

/// The example from the LIFX documentation: set the bulb `d0:73:d5:00:13:37` to green.
pub fn example_set_color() -> SetColorMessage {
    SetColorMessage {
        header: Header {
            frame_header: FrameHeader {
                size: 49,
                protocol: 1024.into(),
                addressable: true,
                tagged: false,
                origin: 0.into(),
                source: 2
            },
            frame_address: FrameAddress {
                target: [0xD0, 0x73, 0xD5, 0x00, 0x13, 0x37, 0x00, 0x00],
                ack_required: true,
                res_required: false,
                sequence: 1,
                ..Default::default()
            },
            protocol_header: ProtocolHeader {
                pkt_type: SetColor::PKT_TYPE,
                ..Default::default()
            }
        },
        payload: SetColor {
            hue: 21845,
            saturation: 65535,
            brightness: 65535,
            kelvin: 3500,
            duration: 0,
            ..Default::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: [u8; 49] = [
        0x31, 0x00, 0x00, 0x14, 0x02, 0x00, 0x00, 0x00, 0xd0, 0x73, 0xd5, 0x00, 0x13, 0x37, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x66, 0x00, 0x00, 0x00, 0x00, 0x55, 0x55, 0xff, 0xff, 0xff, 0xff, 0xac, 0x0d, 0x00, 0x00, 0x00,
        0x00
    ];

    #[test]
    fn pack_set_color() {
        assert_eq!(example_set_color().pack().unwrap(), EXAMPLE);
    }

    #[test]
    fn unpack_set_color() {
        let msg = SetColorMessage::unpack(&EXAMPLE).unwrap();
        assert_eq!(msg, example_set_color());

        let frame_header = msg.header.frame_header;
        assert_eq!(*frame_header.protocol, 1024);
        assert!(frame_header.addressable);
        assert!(!frame_header.tagged);
        assert_eq!(msg.header.protocol_header.pkt_type, SetColor::PKT_TYPE);
    }

    #[test]
    fn display_frame_header() {
        println!("{}", example_set_color().header.frame_header);
    }
}
