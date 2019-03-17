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
#![doc(html_root_url = "https://docs.rs/adxl343/0.0.0")]

extern crate embedded_hal as hal;

use hal::blocking::i2c::{Write, WriteRead};

/// ADXL343 driver
pub struct Adxl343<I2C>(I2C);

impl<I2C, E> Adxl343<I2C>
where
    I2C: WriteRead<Error = E> + Write<Error = E>,
{
    /// Create a new ADXL343 driver from the given I2C peripheral
    pub fn new(i2c: I2C) -> Result<Self, E> {
        Ok(Adxl343(i2c))
    }
}
