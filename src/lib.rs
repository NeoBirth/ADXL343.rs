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
#![doc(html_root_url = "https://docs.rs/adxl343/0.4.2")]

mod register;

pub use crate::register::{DataFormatFlags, DataFormatRange};
pub use accelerometer;
use embedded_hal as hal;

use crate::register::Register;
#[cfg(feature = "i16x3")]
use accelerometer::I16x3;
#[cfg(feature = "u16x3")]
use accelerometer::U16x3;
use accelerometer::{Accelerometer, Error, ErrorKind, Tracker};
use core::fmt::Debug;
use hal::blocking::i2c::{Write, WriteRead};

/// ADXL343 I2C address.
/// Assumes ALT address pin low
pub const ADDRESS: u8 = 0x53;

/// ADXL343 device ID
pub const DEVICE_ID: u8 = 0xE5;

/// ADXL343 driver
pub struct Adxl343<I2C> {
    /// Underlying I2C device
    i2c: I2C,

    /// Current data format
    data_format: DataFormatFlags,
}

impl<I2C, E> Adxl343<I2C>
where
    I2C: WriteRead<Error = E> + Write<Error = E>,
    E: Debug,
{
    /// Create a new ADXL343 driver from the given I2C peripheral
    ///
    /// Default tap detection level: 2G, 31.25ms duration, single tap only
    pub fn new(i2c: I2C) -> Result<Self, Error<E>> {
        Self::new_with_data_format(i2c, DataFormatFlags::default())
    }

    /// Create a new ADXL343 driver configured with the given data format
    pub fn new_with_data_format<F>(i2c: I2C, data_format: F) -> Result<Self, Error<E>>
    where
        F: Into<DataFormatFlags>,
    {
        let mut adxl343 = Adxl343 {
            i2c,
            data_format: data_format.into(),
        };

        // Ensure we have the correct device ID for the ADLX343
        if adxl343.get_device_id()? != DEVICE_ID {
            ErrorKind::Device.err()?;
        }

        // Configure the data format
        adxl343.data_format(adxl343.data_format)?;

        // Disable interrupts
        adxl343.write_register(Register::INT_ENABLE, 0)?;

        // 62.5 mg/LSB
        adxl343.write_register(Register::THRESH_TAP, 20)?;

        // Tap duration: 625 µs/LSB
        adxl343.write_register(Register::DUR, 50)?;

        // Tap latency: 1.25 ms/LSB (0 = no double tap)
        adxl343.write_register(Register::LATENT, 0)?;

        // Waiting period: 1.25 ms/LSB (0 = no double tap)
        adxl343.write_register(Register::WINDOW, 0)?;

        // Enable XYZ axis for tap
        adxl343.write_register(Register::TAP_AXES, 0x7)?;

        // Enable measurements
        adxl343.write_register(Register::POWER_CTL, 0x08)?;

        Ok(adxl343)
    }

    /// Use this accelerometer as an orientation tracker
    pub fn try_into_tracker(mut self) -> Result<Tracker<Self, I16x3>, Error<E>> {
        self.data_format(DataFormatRange::PLUSMINUS_8G)?;
        Ok(Tracker::new(self, 12000))
    }

    /// Set the device data format
    pub fn data_format<F>(&mut self, data_format: F) -> Result<(), Error<E>>
    where
        F: Into<DataFormatFlags>,
    {
        let f = data_format.into();
        let input = [Register::DATA_FORMAT.addr(), f.bits()];
        self.i2c.write(ADDRESS, &input)?;
        self.data_format = f;
        Ok(())
    }

    /// Write to the given register
    // TODO: make this an internal API after enough functionality is wrapped
    pub fn write_register(&mut self, register: Register, value: u8) -> Result<(), Error<E>> {
        // Preserve the invariant around self.data_format
        assert_ne!(
            register,
            Register::DATA_FORMAT,
            "set data format with Adxl343::data_format"
        );

        debug_assert!(!register.read_only(), "can't write to read-only register");
        self.i2c.write(ADDRESS, &[register.addr(), value])?;
        Ok(())
    }

    /// Write to a given register, then read the result
    // TODO: make this an internal API after enough functionality is wrapped
    pub fn write_read_register(&mut self, register: Register, buffer: &mut [u8]) -> Result<(), E> {
        self.i2c.write_read(ADDRESS, &[register.addr()], buffer)
    }

    /// Get the device ID
    fn get_device_id(&mut self) -> Result<u8, E> {
        let input = [Register::DEVID.addr()];
        let mut output = [0u8];
        self.i2c.write_read(ADDRESS, &input, &mut output)?;
        Ok(output[0])
    }

    /// Write to a given register, then read a `i16` result
    ///
    /// From the ADXL343 data sheet (p.25):
    /// <https://www.analog.com/media/en/technical-documentation/data-sheets/adxl343.pdf>
    ///
    /// "The output data is twos complement, with DATAx0 as the least
    /// significant byte and DATAx1 as the most significant byte"
    #[cfg(feature = "i16x3")]
    fn write_read_i16(&mut self, register: Register) -> Result<i16, E> {
        let mut buffer = [0u8; 2];
        self.write_read_register(register, &mut buffer)?;
        Ok(i16::from_be_bytes(buffer))
    }

    /// Write to a given register, then read a `u16` result
    ///
    /// Used for reading `JUSTIFY`-mode data. From the ADXL343 data sheet (p.25):
    /// <https://www.analog.com/media/en/technical-documentation/data-sheets/adxl343.pdf>
    ///
    /// "A setting of 1 in the justify bit selects left-justified (MSB) mode,
    /// and a setting of 0 selects right-justified mode with sign extension."
    #[cfg(feature = "u16x3")]
    fn write_read_u16(&mut self, register: Register) -> Result<u16, E> {
        let mut buffer = [0u8; 2];
        self.write_read_register(register, &mut buffer)?;
        Ok(u16::from_le_bytes(buffer))
    }
}

#[cfg(feature = "i16x3")]
impl<I2C, E> Accelerometer<I16x3> for Adxl343<I2C>
where
    I2C: WriteRead<Error = E> + Write<Error = E>,
    E: Debug,
{
    type Error = Error<E>;

    /// Get acceleration reading from the accelerometer
    fn acceleration(&mut self) -> Result<I16x3, Error<E>> {
        // TODO: return an error instead of panicking
        assert!(
            !self.data_format.contains(DataFormatFlags::JUSTIFY),
            "can only read I16x3 in non-justified mode"
        );

        let x = self.write_read_i16(Register::DATAX0)?;
        let y = self.write_read_i16(Register::DATAY0)?;
        let z = self.write_read_i16(Register::DATAZ0)?;
        Ok(I16x3::new(x, y, z))
    }
}

#[cfg(feature = "u16x3")]
impl<I2C, E> Accelerometer<U16x3> for Adxl343<I2C>
where
    I2C: WriteRead<Error = E> + Write<Error = E>,
    E: Debug,
{
    type Error = Error<E>;

    /// Get acceleration reading from the accelerometer
    fn acceleration(&mut self) -> Result<U16x3, Error<E>> {
        // TODO: return an error instead of panicking
        assert!(
            self.data_format.contains(DataFormatFlags::JUSTIFY),
            "can only read I16x3 in non-justified mode"
        );

        let x = self.write_read_u16(Register::DATAX0)?;
        let y = self.write_read_u16(Register::DATAY0)?;
        let z = self.write_read_u16(Register::DATAZ0)?;
        Ok(U16x3::new(x, y, z))
    }
}
