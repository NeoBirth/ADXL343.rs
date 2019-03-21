//! ADXL343 accelerometer driver which uses I2C via [embedded-hal]
//!
//! [embedded-hal]: https://docs.rs/embedded-hal

#![no_std]
#![deny(
    warnings,
    missing_docs,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unused_import_braces,
    unused_qualifications
)]
#![forbid(unsafe_code)]
#![doc(html_root_url = "https://docs.rs/adxl343/0.2.0")]

extern crate embedded_hal as hal;

use hal::blocking::i2c::{Write, WriteRead};

/// ADXL343 I2C address.
/// Assumes ALT address pin low
pub const ADDRESS: u8 = 0x53;

/// ADXL343 device ID
pub const DEVICE_ID: u8 = 0xE5;

/// 4mg per lsb
pub const SENSITIVITY: f32 = 256.0;

/// Register addresses
#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug)]
pub enum Register {
    /// Device ID
    DEVICE_ID = 0x00,

    /// Tap threshold
    THRESH_TAP = 0x1D,

    /// X-axis offset
    OFFSET_X = 0x1E,

    /// Y-axis offset
    OFFSET_Y = 0x1F,

    /// Z-axis offset
    OFFSET_Z = 0x20,

    /// Tap duration
    TAP_DURATION = 0x21,

    /// Tap latency
    TAP_LATENCY = 0x22,

    /// Tap window
    TAP_WINDOW = 0x23,

    /// Activity threshold
    THRESH_ACT = 0x24,

    /// Inactivity threshold
    THRESH_INACT = 0x25,

    /// Inactivity time
    TIME_INACT = 0x26,

    /// Axis enable control for activity and inactivity detection
    ACT_INACT_CTRL = 0x27,

    /// Free-fall threshold
    THRESH_FF = 0x28,

    /// Free-fall time
    TIME_FF = 0x29,

    /// Axis control for single/double tap
    TAP_AXES = 0x2A,

    /// Source for single/double tap
    ACT_TAP_STATUS = 0x2B,

    /// Data rate and power mode control
    BW_RATE = 0x2C,

    /// Power-saving features control
    POWER_CTRL = 0x2D,

    /// Interrupt enable control
    INT_ENABLE = 0x2E,

    /// Interrupt mapping control
    INT_MAP = 0x2F,

    /// Source of interrupts
    INT_SOURCE = 0x30,

    /// Data format control
    DATA_FORMAT = 0x31,

    /// X-axis data 0
    DATAX0 = 0x32,

    /// X-axis data 1
    DATAX1 = 0x33,

    /// Y-axis data 0
    DATAY0 = 0x34,

    /// Y-axis data 1
    DATAY1 = 0x35,

    /// Z-axis data 0
    DATAZ0 = 0x36,

    /// Z-axis data 1
    DATAZ1 = 0x37,

    /// FIFO control
    FIFO_CTRL = 0x38,

    /// FIFO status
    FIFO_STATUS = 0x39,
}

impl Register {
    /// Get register address
    fn addr(self) -> u8 {
        self as u8
    }
}

/// XYZ triple representing raw values
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub struct Xyz {
    /// X component
    pub x: u16,

    /// Y component
    pub y: u16,

    /// Z component
    pub z: u16,
}

/// XYZ triple representing acceleration
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct Ax3 {
    /// X component
    pub x: f32,

    /// Y component
    pub y: f32,

    /// Z component
    pub z: f32,
}

impl From<Xyz> for Ax3 {
    fn from(raw: Xyz) -> Ax3 {
        Ax3 {
            x: f32::from(raw.x) / SENSITIVITY,
            y: f32::from(raw.y) / SENSITIVITY,
            z: f32::from(raw.z) / SENSITIVITY,
        }
    }
}

/// ADXL343 driver
pub struct Adxl343<I2C> {
    i2c: I2C,
}

impl<I2C, E> Adxl343<I2C>
where
    I2C: WriteRead<Error = E> + Write<Error = E>,
{
    /// Create a new ADXL343 driver from the given I2C peripheral
    pub fn new(i2c: I2C) -> Result<Self, E> {
        let mut adxl343 = Adxl343 { i2c };
        let device_id = adxl343.get_device_id()?;

        // TODO: return an error here instead of panicking?
        assert_eq!(device_id, DEVICE_ID, "unexpected device ID: {}", device_id);

        // Default tap detection level: 2G, 31.25ms duration, single tap only
        //
        // Disable interrupts
        adxl343.write_register(Register::INT_ENABLE, 0)?;

        // 62.5 mg/LSB
        adxl343.write_register(Register::THRESH_TAP, 20)?;

        // Tap duration: 625 µs/LSB
        adxl343.write_register(Register::TAP_DURATION, 50)?;

        // Tap latency: 1.25 ms/LSB (0 = no double tap)
        adxl343.write_register(Register::TAP_LATENCY, 0)?;

        // Waiting period: 1.25 ms/LSB (0 = no double tap)
        adxl343.write_register(Register::TAP_WINDOW, 0)?;

        // Enable XYZ axis for tap
        adxl343.write_register(Register::TAP_AXES, 0x7)?;

        // Enable measurements
        adxl343.write_register(Register::POWER_CTRL, 0x08)?;

        Ok(adxl343)
    }

    /// Get raw acceleration values
    pub fn xyz(&mut self) -> Result<Xyz, E> {
        let x = self.read_register16(Register::DATAX0)?;
        let y = self.read_register16(Register::DATAY0)?;
        let z = self.read_register16(Register::DATAZ0)?;

        Ok(Xyz { x, y, z })
    }

    /// Get computed acceleration within (?) range
    pub fn accel(&mut self) -> Result<Ax3, E> {
        Ok(self.xyz()?.into())
    }

    /// Get the device ID
    fn get_device_id(&mut self) -> Result<u8, E> {
        let mut buffer = [0u8];

        self.i2c
            .write_read(ADDRESS, &[Register::DEVICE_ID.addr()], &mut buffer)?;

        Ok(buffer[0])
    }

    /// Read a 16-byte value from the given register
    fn read_register16(&mut self, register: Register) -> Result<u16, E> {
        let mut buffer = [0u8; 2];

        self.i2c
            .write_read(ADDRESS, &[register.addr()], &mut buffer)?;

        Ok(u16::from(buffer[0]) << 8 | u16::from(buffer[1]))
    }

    /// Write to the given register
    fn write_register(&mut self, register: Register, value: u8) -> Result<(), E> {
        self.i2c.write(ADDRESS, &[register.addr(), value])
    }
}
