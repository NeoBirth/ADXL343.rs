//! Platform-agnostic ADXL343 accelerometer driver which uses I2C via
//! [embedded-hal] and implements the [`Accelerometer` trait][trait]
//! from the `accelerometer` crate.
//!
//! [embedded-hal]: https://docs.rs/embedded-hal
//! [trait]: https://docs.rs/accelerometer/latest/accelerometer/trait.Accelerometer.html

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
#![doc(html_root_url = "https://docs.rs/adxl343/0.3.0")]

extern crate embedded_hal as hal;

mod register;

use self::register::Register;
use accelerometer::{Accelerometer, U16x3};
use hal::blocking::i2c::{Write, WriteRead};

/// ADXL343 I2C address.
/// Assumes ALT address pin low
pub const ADDRESS: u8 = 0x53;

/// ADXL343 device ID
pub const DEVICE_ID: u8 = 0xE5;

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

impl<I2C, E> Accelerometer<U16x3, E> for Adxl343<I2C>
where
    I2C: WriteRead<Error = E> + Write<Error = E>,
{
    /// Get acceleration reading from the accelerometer
    fn acceleration(&mut self) -> Result<U16x3, E> {
        let x = self.read_register16(Register::DATAX0)?;
        let y = self.read_register16(Register::DATAY0)?;
        let z = self.read_register16(Register::DATAZ0)?;

        Ok(U16x3::new(x, y, z))
    }
}
