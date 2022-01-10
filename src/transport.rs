use crate::register::Register;
use crate::ADDRESS;
use core::fmt::Debug;
use embedded_hal::blocking::spi;
use embedded_hal::{blocking::i2c, digital::v2::OutputPin};

/// Error type for sensor transport
pub enum TransportError<EBUS, EPIN> {
    /// Error variant for the transport bus itself
    BusError(EBUS),
    /// Error variant for pins associated with transport (SPI Chip Select)
    PinError(EPIN),
}

pub trait Transport {
    type BusError;
    type PinError;
    fn write_register(
        &mut self,
        register: Register,
        value: u8,
    ) -> Result<(), TransportError<Self::BusError, Self::PinError>>;
    fn read_register<const N: usize>(
        &mut self,
        register: Register,
    ) -> Result<[u8; N], TransportError<Self::BusError, Self::PinError>>;
}

impl<EBUS, EPIN> Debug for TransportError<EBUS, EPIN>
where
    EBUS: Debug,
    EPIN: Debug,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        match self {
            Self::BusError(e) => write!(f, "{:?}", e),
            Self::PinError(e) => write!(f, "{:?}", e),
        }
    }
}

/// Device transport using I2C
pub struct I2cTransport<I> {
    i2c: I,
}

impl<I> I2cTransport<I> {
    /// Create a new I2C transport
    pub fn new(i2c: I) -> Self {
        Self { i2c }
    }
}

impl<I, E> Transport for I2cTransport<I>
where
    I: i2c::Write<Error = E> + i2c::WriteRead<Error = E>,
{
    type BusError = E;
    type PinError = ();
    fn write_register(
        &mut self,
        register: Register,
        value: u8,
    ) -> Result<(), TransportError<Self::BusError, Self::PinError>> {
        debug_assert!(!register.read_only(), "can't write to read-only register");
        self.i2c
            .write(ADDRESS, &[register.addr(), value])
            .map_err(|e| TransportError::BusError(e))?;
        Ok(())
    }

    fn read_register<const N: usize>(
        &mut self,
        register: Register,
    ) -> Result<[u8; N], TransportError<Self::BusError, Self::PinError>> {
        let mut buffer: [u8; N] = [0; N];
        self.i2c
            .write_read(ADDRESS, &[register.addr()], &mut buffer)
            .map_err(|e| TransportError::BusError(e))?;
        Ok(buffer)
    }
}

/// Device transport using SPI
pub struct SpiTransport<SPI, CS> {
    spi: SPI,
    cs: CS,
}

impl<SPI, CS> SpiTransport<SPI, CS> {
    /// Create a new SPI transport
    pub fn new(spi: SPI, cs: CS) -> Self {
        Self { spi, cs }
    }
}

impl<SPI, CS, EBUS, EPIN> Transport for SpiTransport<SPI, CS>
where
    SPI: spi::Transfer<u8, Error = EBUS> + spi::Write<u8, Error = EBUS>,
    CS: OutputPin<Error = EPIN>,
{
    type BusError = EBUS;
    type PinError = EPIN;
    fn write_register(
        &mut self,
        register: Register,
        value: u8,
    ) -> Result<(), TransportError<Self::BusError, Self::PinError>> {
        debug_assert!(!register.read_only(), "can't write to read-only register");

        self.cs.set_low().map_err(|e| TransportError::PinError(e))?;
        self.spi
            .write(&[register.addr(), value])
            .map_err(|e| TransportError::BusError(e))?;
        self.cs
            .set_high()
            .map_err(|e| TransportError::PinError(e))?;
        Ok(())
    }

    fn read_register<const N: usize>(
        &mut self,
        register: Register,
    ) -> Result<[u8; N], TransportError<Self::BusError, Self::PinError>> {
        self.cs.set_low().map_err(|e| TransportError::PinError(e))?;
        self.spi
            .write(&[register.addr() | 0x80])
            .map_err(|e| TransportError::BusError(e))?;
        let mut buffer: [u8; N] = [0; N];
        self.spi
            .transfer(&mut buffer)
            .map_err(|e| TransportError::BusError(e))?;
        self.cs
            .set_high()
            .map_err(|e| TransportError::PinError(e))?;
        Ok(buffer)
    }
}
