//! Platform-agnostic ADXL343 accelerometer driver which uses I2C via
//! [embedded-hal] and implements the [`Accelerometer` trait][trait]
//! from the `accelerometer` crate.
//!
//! [embedded-hal]: https://docs.rs/embedded-hal
//! [trait]: https://docs.rs/accelerometer/latest/accelerometer/trait.Accelerometer.html

#![no_std]
#![doc(html_root_url = "https://docs.rs/adxl343/0.8.0")]
#![forbid(unsafe_code)]
#![warn(missing_docs, rust_2018_idioms, unused_qualifications)]

mod register;
mod transport;

pub use crate::register::{DataFormatFlags, DataFormatRange};
pub use accelerometer;

use crate::register::Register;
#[cfg(feature = "u16x3")]
use accelerometer::vector::U16x3;
#[cfg(feature = "i16x3")]
use accelerometer::{
    vector::{F32x3, I16x3},
    Accelerometer,
};
use accelerometer::{Error, ErrorKind, RawAccelerometer};
use core::fmt::Debug;
use transport::Transport;
pub use transport::{I2cTransport, SpiTransport, TransportError};

/// ADXL343 I2C address.
/// Assumes ALT address pin low
pub const ADDRESS: u8 = 0x53;

/// ADXL343 device ID
pub const DEVICE_ID: u8 = 0xE5;

/// ADXL343 driver
pub struct Adxl343<T> {
    /// Underlying device transport
    transport: T,

    /// Current data format
    data_format: DataFormatFlags,
}

impl<T, EBUS, EPIN> Adxl343<T>
where
    T: Transport<BusError = EBUS, PinError = EPIN>,
    EBUS: Debug,
    EPIN: Debug,
{
    /// Create a new ADXL343 driver from the given peripheral
    ///
    /// Default tap detection level: 2G, 31.25ms duration, single tap only
    pub fn new(transport: T) -> Result<Self, Error<TransportError<EBUS, EPIN>>> {
        Self::new_with_data_format(transport, DataFormatFlags::default())
    }

    /// Create a new ADXL343 driver configured with the given data format
    pub fn new_with_data_format<F>(
        transport: T,
        data_format: F,
    ) -> Result<Self, Error<TransportError<EBUS, EPIN>>>
    where
        F: Into<DataFormatFlags>,
    {
        let mut adxl343 = Adxl343 {
            transport,
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

    /// Set the device data format
    pub fn data_format<F>(
        &mut self,
        data_format: F,
    ) -> Result<(), Error<TransportError<EBUS, EPIN>>>
    where
        F: Into<DataFormatFlags>,
    {
        let f = data_format.into();
        self.transport
            .write_register(Register::DATA_FORMAT, f.bits())?;
        self.data_format = f;
        Ok(())
    }

    /// Write to the given register
    // TODO: make this an internal API after enough functionality is wrapped
    pub fn write_register(
        &mut self,
        register: Register,
        value: u8,
    ) -> Result<(), Error<TransportError<EBUS, EPIN>>> {
        // Preserve the invariant around self.data_format
        assert_ne!(
            register,
            Register::DATA_FORMAT,
            "set data format with Adxl343::data_format"
        );

        self.transport.write_register(register, value)?;
        Ok(())
    }

    /// Read from a given register
    // TODO: make this an internal API after enough functionality is wrapped
    pub fn read_register<const N: usize>(
        &mut self,
        register: Register,
    ) -> Result<[u8; N], Error<TransportError<EBUS, EPIN>>> {
        let b = self.transport.read_register(register)?;
        Ok(b)
    }

    /// Get the device ID
    fn get_device_id(&mut self) -> Result<u8, TransportError<EBUS, EPIN>> {
        let output: [u8; 1] = self.transport.read_register(Register::DEVID)?;
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
    fn write_read_i16(&mut self, register: Register) -> Result<i16, TransportError<EBUS, EPIN>> {
        let buffer: [u8; 2] = self.transport.read_register(register)?;
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
    fn write_read_u16(&mut self, register: Register) -> Result<u16, TransportError<EBUS, EPIN>> {
        let buffer: [u8; 2] = self.read_register(register)?;
        Ok(u16::from_le_bytes(buffer))
    }
}

#[cfg(feature = "i16x3")]
impl<T, EBUS, EPIN> Accelerometer for Adxl343<T>
where
    T: Transport<BusError = EBUS, PinError = EPIN>,
    EBUS: Debug,
    EPIN: Debug,
{
    type Error = TransportError<EBUS, EPIN>;

    /// Get normalized ±g reading from the accelerometer.
    fn accel_norm(&mut self) -> Result<F32x3, Error<Self::Error>> {
        let raw_data: I16x3 = self.accel_raw()?;
        let range: f32 = self.data_format.range().into();

        let x = (raw_data.x as f32 / core::i16::MAX as f32) * range;
        let y = (raw_data.y as f32 / core::i16::MAX as f32) * range;
        let z = (raw_data.z as f32 / core::i16::MAX as f32) * range;

        Ok(F32x3::new(x, y, z))
    }

    /// Get sample rate of accelerometer in Hz.
    ///
    /// This is presently hardcoded to 100Hz - the default data rate.
    /// See "Register 0x2C - BW_RATE" documentation in ADXL343 data sheet (p.23):
    /// <https://www.analog.com/media/en/technical-documentation/data-sheets/adxl343.pdf>
    ///
    /// "The default value is 0x0A, which translates to a 100 Hz output data rate."
    fn sample_rate(&mut self) -> Result<f32, Error<Self::Error>> {
        Ok(100.0)
    }
}

#[cfg(feature = "i16x3")]
impl<T, EBUS, EPIN> RawAccelerometer<I16x3> for Adxl343<T>
where
    T: Transport<BusError = EBUS, PinError = EPIN>,
    EBUS: Debug,
    EPIN: Debug,
{
    type Error = TransportError<EBUS, EPIN>;

    /// Get acceleration reading from the accelerometer
    fn accel_raw(&mut self) -> Result<I16x3, Error<Self::Error>> {
        if self.data_format.contains(DataFormatFlags::JUSTIFY) {
            return Err(Error::new(ErrorKind::Mode));
        }

        let x = self.write_read_i16(Register::DATAX0)?;
        let y = self.write_read_i16(Register::DATAY0)?;
        let z = self.write_read_i16(Register::DATAZ0)?;

        Ok(I16x3::new(x, y, z))
    }
}

#[cfg(feature = "u16x3")]
impl<T, EBUS, EPIN> RawAccelerometer<U16x3> for Adxl343<T>
where
    T: Transport<BusError = EBUS, PinError = EPIN>,
    EBUS: Debug,
    EPIN: Debug,
{
    type Error = TransportError<EBUS, EPIN>;

    /// Get acceleration reading from the accelerometer
    fn accel_raw(&mut self) -> Result<U16x3, Error<Self::Error>> {
        if !self.data_format.contains(DataFormatFlags::JUSTIFY) {
            return Err(Error::new(ErrorKind::Mode));
        }

        let x = self.write_read_u16(Register::DATAX0)?;
        let y = self.write_read_u16(Register::DATAY0)?;
        let z = self.write_read_u16(Register::DATAZ0)?;

        Ok(U16x3::new(x, y, z))
    }
}
