#![no_std]

#[macro_use]
extern crate packed_struct;

use packed_struct::prelude::*;

/// Control register, address 0xA0.
#[derive(PackedStruct, Debug, Copy, Clone, PartialEq)]
#[packed_struct(bit_numbering="msb0")]
pub struct ControlRegister {
    /// Sensor's power mode
    #[packed_field(bits="0:1", ty="enum")]
    pub power_mode: PowerMode,
    /// Voltage on the input. [mV]
    #[packed_field(bits="3:7")]
    pub voltage_milli_volts: Integer<u8, packed_bits::Bits::<5>>,
    /// Is the standby LED enabled?
    #[packed_field(bits="8")]
    pub standby_led_enabled: bool,
    /// Which of the four gain stages are active
    #[packed_field(bits="9:12")]
    pub gain_stages: [bool; 4],
    /// Reserved bits, always 1
    #[packed_field(bits="13:15")]
    pub _reserved: ReservedOnes<packed_bits::Bits::<3>>,
    /// Sensor's reading
    #[packed_field(bits="16:31", endian="lsb")]
    pub sensor_value: i16
}

#[derive(PrimitiveEnum_u8, Debug, Copy, Clone, PartialEq)]
pub enum PowerMode {
    /// The sensor is turned off
    Off = 0,
    /// The sensor can be triggered by an external source
    Standby = 1,
    /// Digital logic is on and waiting
    LowPower = 2,
    /// The sensor is enabled and turned on
    On = 3
}


#[cfg(test)]
mod tests {
    #[test]
    fn nostd_usage() {
        use packed_struct::prelude::*;

        use ControlRegister;
        use PowerMode;

        let reg = ControlRegister {
            power_mode: PowerMode::LowPower,
            voltage_milli_volts: 11.into(),
            standby_led_enabled: true,
            gain_stages: [true, true, false, false],
            _reserved: Default::default(),
            sensor_value: -1503
        };

        let packed: [u8; 4] = reg.pack().unwrap();
        assert_eq!([0x8B, 0xE7, 0x21, 0xFA], packed);
        let unpacked = ControlRegister::unpack(&[0x8B, 0xE7, 0x21, 0xFA]).unwrap();
        assert_eq!(unpacked, reg);
    }
}
