//! Structures with `byte_order="lsb"`, laid out as a single little-endian integer.

use packed_struct::prelude::*;

/// The LIFX frame header from issue #29, as written there, plus `byte_order="lsb"`.
#[derive(PackedStruct, Debug, Copy, Clone, PartialEq, Default)]
#[packed_struct(bit_numbering = "lsb0", size_bytes = "8", byte_order = "lsb")]
pub struct Header {
    #[packed_field(bits = "15:0", endian = "lsb")]
    pub size: u16,
    #[packed_field(bits = "27:16", endian = "lsb")]
    pub protocol: u16,
    #[packed_field(bits = "28:28", endian = "lsb")]
    pub addressable: bool,
    #[packed_field(bits = "29:29", endian = "lsb")]
    pub tagged: bool,
    #[packed_field(bits = "31:30", endian = "lsb")]
    _reserved1: ReservedZero<packed_bits::Bits::<2>>,
    #[packed_field(bits = "63:32", endian = "lsb")]
    pub source: u32,
}

#[test]
fn issue_29_lifx_frame_header() {
    let header = Header {
        size: 49,
        protocol: 1024,
        addressable: true,
        tagged: false,
        source: 2,
        ..Default::default()
    };

    // the 12-bit protocol is split: the low byte, then the low nibble of the next byte
    let packed = header.pack().unwrap();
    assert_eq!(packed, [0x31, 0x00, 0x00, 0x14, 0x02, 0x00, 0x00, 0x00]);
    assert_eq!(Header::unpack(&packed).unwrap(), header);

    let header = Header { protocol: 0xABC, addressable: true, tagged: true, ..Default::default() };
    assert_eq!(header.pack().unwrap(), [0x00, 0x00, 0xBC, 0x3A, 0x00, 0x00, 0x00, 0x00]);
}

#[test]
fn the_same_as_a_little_endian_integer() {
    let header = Header {
        size: 0x1234,
        protocol: 0x567,
        addressable: true,
        tagged: false,
        source: 0x89AB_CDEF,
        ..Default::default()
    };

    let n: u64 = 0x1234 | (0x567 << 16) | (1 << 28) | (0x89AB_CDEF << 32);
    assert_eq!(header.pack().unwrap(), n.to_le_bytes());
}

#[derive(PackedStruct, Debug, Copy, Clone, PartialEq, Default)]
#[packed_struct(bit_numbering = "lsb0", size_bytes = "12", byte_order = "lsb")]
pub struct Arrays {
    #[packed_field(bits = "31:0")]
    pub bytes: [u8; 4],
    #[packed_field(bits = "47:32", element_size_bits = "4")]
    pub nibbles: [Integer<u8, packed_bits::Bits::<4>>; 4],
    #[packed_field(bits = "79:48")]
    pub words: [u16; 2],
    #[packed_field(bits = "87:80")]
    pub flags: [bool; 8],
    #[packed_field(bits = "95:88")]
    pub tail: u8,
}

#[test]
fn arrays_start_at_the_lowest_address() {
    let a = Arrays {
        bytes: [1, 2, 3, 4],
        nibbles: [0x1.into(), 0x2.into(), 0x3.into(), 0x4.into()],
        words: [0x1234, 0x5678],
        flags: [true, false, false, false, false, false, true, false],
        tail: 0xEE,
    };

    let packed = a.pack().unwrap();
    assert_eq!(packed, [1, 2, 3, 4, 0x21, 0x43, 0x34, 0x12, 0x78, 0x56, 0b0100_0001, 0xEE]);
    assert_eq!(Arrays::unpack(&packed).unwrap(), a);
}

/// A big-endian structure, nested at a byte boundary.
#[derive(PackedStruct, Debug, Copy, Clone, PartialEq, Default)]
#[packed_struct(bit_numbering = "msb0", endian = "msb")]
pub struct Point {
    pub x: u8,
    pub y: u16,
}

/// A little-endian structure narrower than its bytes, nested at an unaligned position.
#[derive(PackedStruct, Debug, Copy, Clone, PartialEq, Default)]
#[packed_struct(bit_numbering = "lsb0", size_bytes = "2", byte_order = "lsb")]
pub struct Inner12 {
    #[packed_field(bits = "11:0")]
    pub v: Integer<u16, packed_bits::Bits::<12>>,
}

#[derive(PackedStruct, Debug, Copy, Clone, PartialEq, Default)]
#[packed_struct(bit_numbering = "lsb0", size_bytes = "6", byte_order = "lsb")]
pub struct Nested {
    #[packed_field(bits = "3:0")]
    pub lo: Integer<u8, packed_bits::Bits::<4>>,
    #[packed_field(bits = "15:4")]
    pub inner: Inner12,
    #[packed_field(bits = "39:16")]
    pub point: Point,
    #[packed_field(bits = "47:40")]
    pub last: u8,
}

#[test]
fn nested_structs() {
    let n = Nested {
        lo: 0xA.into(),
        inner: Inner12 { v: 0xBCD.into() },
        point: Point { x: 0x11, y: 0x2233 },
        last: 0x44,
    };

    let packed = n.pack().unwrap();
    assert_eq!(packed, [0xDA, 0xBC, 0x11, 0x22, 0x33, 0x44]);
    assert_eq!(Nested::unpack(&packed).unwrap(), n);
}

#[derive(PrimitiveEnum_u8, Debug, Copy, Clone, PartialEq, Default)]
pub enum Mode {
    #[default]
    Off = 0,
    Low = 1,
    High = 5,
}

#[derive(PackedStruct, Debug, Copy, Clone, PartialEq, Default)]
#[packed_struct(bit_numbering = "lsb0", size_bytes = "2", byte_order = "lsb")]
pub struct Register {
    #[packed_field(bits = "5:0")]
    pub low: Integer<u8, packed_bits::Bits::<6>>,
    #[packed_field(bits = "8:6", ty = "enum")]
    pub mode: Mode,
    #[packed_field(bits = "9")]
    pub enabled: bool,
    #[packed_field(bits = "15:10")]
    pub _reserved: ReservedOnes<packed_bits::Bits::<6>>,
}

#[test]
fn enum_across_a_byte_boundary() {
    let r = Register { mode: Mode::High, enabled: true, ..Default::default() };

    let packed = r.pack().unwrap();
    // High = 0b101: bits 6 and 8, enabled is bit 9, the reserved ones fill the rest of byte 1
    assert_eq!(packed, [0b0100_0000, 0b1111_1111]);
    assert_eq!(Register::unpack(&packed).unwrap(), r);
    assert_eq!(Register::unpack(&[0x00, 0b1111_1110]).unwrap().mode, Mode::Off);
}

/// The fixed supply PDO from `test_usb_pd.rs`, as it is sent over the wire: a little-endian `u32`.
#[derive(PackedStruct, Debug, Default, Copy, Clone, PartialEq)]
#[packed_struct(size_bytes = "4", bit_numbering = "lsb0", byte_order = "lsb")]
pub struct PowerDataObjectFixed {
    #[packed_field(bits = "31:30")]
    pub supply: Integer<u8, packed_bits::Bits::<2>>,
    #[packed_field(bits = "29")]
    pub dual_role_power: bool,
    #[packed_field(bits = "28")]
    pub usb_suspend_supported: bool,
    #[packed_field(bits = "27")]
    pub unconstrained_power: bool,
    #[packed_field(bits = "26")]
    pub usb_communications_capable: bool,
    #[packed_field(bits = "25")]
    pub dual_role_data: bool,
    #[packed_field(bits = "21:20")]
    pub peak_current: Integer<u8, packed_bits::Bits::<2>>,
    #[packed_field(bits = "19:10")]
    pub voltage: Integer<u16, packed_bits::Bits::<10>>,
    #[packed_field(bits = "9:0")]
    pub maximum_current: Integer<u16, packed_bits::Bits::<10>>,
}

#[test]
fn usb_pd_fixed_pdo() {
    // 5 V (100 * 50 mV), 3 A (300 * 10 mA)
    let pdo = PowerDataObjectFixed {
        dual_role_data: true,
        voltage: 100.into(),
        maximum_current: 300.into(),
        ..Default::default()
    };

    let n: u32 = (1 << 25) | (100 << 10) | 300;
    let packed = pdo.pack().unwrap();
    assert_eq!(packed, n.to_le_bytes());
    assert_eq!(PowerDataObjectFixed::unpack(&packed).unwrap(), pdo);
}

#[test]
fn display_shows_lsb0_positions() {
    let header = Header { size: 49, protocol: 1024, addressable: true, source: 2, ..Default::default() };
    let s = format!("{}", header);

    assert!(s.contains("protocol | bits  27:16"), "{}", s);
    assert!(s.contains("0b010000000000"), "{}", s);
}
