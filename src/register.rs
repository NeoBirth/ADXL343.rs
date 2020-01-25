//! ADXL343 register addresses
#![allow(non_camel_case_types, clippy::unreadable_literal)]

use bitflags::bitflags;

/// Register addresses
/// Taken from the ADXL343 data sheet (Register Map, p.21)
/// <https://www.analog.com/media/en/technical-documentation/data-sheets/adxl343.pdf>
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum Register {
    /// Device ID (Read Only)
    ///
    /// "The DEVID register holds a fixed device ID code of 0xE5 (345 octal)."
    DEVID = 0x00,

    /// Tap threshold (Read/Write)
    ///
    /// "The THRESH_TAP register is eight bits and holds the threshold
    /// value for tap interrupts. The data format is unsigned, therefore,
    /// the magnitude of the tap event is compared with the value
    /// in THRESH_TAP for normal tap detection. The scale factor is
    /// 62.5 mg/LSB (that is, 0xFF = 16 g). A value of 0 may result in
    /// undesirable behavior if single tap/double tap interrupts are
    /// enabled."
    THRESH_TAP = 0x1D,

    /// X-axis offset (Read/Write)
    ///
    /// "The OFSX, OFSY, and OFSZ registers are each eight bits and
    /// offer user-set offset adjustments in twos complement format
    /// with a scale factor of 15.6 mg/LSB (that is, 0x7F = 2 g). The
    /// value stored in the offset registers is automatically added to the
    /// acceleration data, and the resulting value is stored in the output
    /// data registers."
    OFSX = 0x1E,

    /// Y-axis offset (Read/Write)
    ///
    /// See OFSX notes.
    OFSY = 0x1F,

    /// Z-axis offset (Read/Write)
    ///
    /// See OFSX notes.
    OFSZ = 0x20,

    /// Tap duration (Read/Write)
    ///
    /// "The DUR register is eight bits and contains an unsigned time
    /// value representing the maximum time that an event must be
    /// above the THRESH_TAP threshold to qualify as a tap event. The
    /// scale factor is 625 µs/LSB. A value of 0 disables the single tap/
    /// double tap functions."
    DUR = 0x21,

    /// Tap latency (Read/Write)
    ///
    /// "The latent register is eight bits and contains an unsigned time
    /// value representing the wait time from the detection of a tap
    /// event to the start of the time window (defined by the window
    /// register) during which a possible second tap event can be detected.
    /// The scale factor is 1.25 ms/LSB. A value of 0 disables the double tap
    /// function"
    LATENT = 0x22,

    /// Tap window (Read/Write)
    ///
    /// "The window register is eight bits and contains an unsigned time
    /// value representing the amount of time after the expiration of the
    /// latency time (determined by the latent register) during which a
    /// second valid tap can begin. The scale factor is 1.25 ms/LSB. A
    /// value of 0 disables the double tap function."
    WINDOW = 0x23,

    /// Activity threshold (Read/Write)
    ///
    /// "The THRESH_ACT register is eight bits and holds the threshold
    /// value for detecting activity. The data format is unsigned,
    /// therefore, the magnitude of the activity event is compared
    /// with the value in the THRESH_ACT register. The scale factor
    /// is 62.5 mg/LSB. A value of 0 may result in undesirable behavior
    /// if the activity interrupt is enabled."
    THRESH_ACT = 0x24,

    /// Inactivity threshold (Read/Write)
    ///
    /// "The THRESH_INACT register is eight bits and holds the threshold
    /// value for detecting inactivity. The data format is unsigned,
    /// therefore, the magnitude of the inactivity event is compared
    /// with the value in the THRESH_INACT register. The scale factor
    /// is 62.5 mg/LSB. A value of 0 may result in undesirable behavior
    /// if the inactivity interrupt is enabled."
    THRESH_INACT = 0x25,

    /// Inactivity time (Read/Write)
    ///
    /// "The TIME_INACT register is eight bits and contains an unsigned
    /// time value representing the amount of time that acceleration
    /// must be less than the value in the THRESH_INACT register for
    /// inactivity to be declared. The scale factor is 1 sec/LSB. Unlike
    /// the other interrupt functions, which use unfiltered data (see the
    /// Threshold commands), the inactivity function uses filtered output
    /// data. At least one output sample must be generated for the
    /// inactivity interrupt to be triggered. This results in the function
    /// appearing unresponsive if the TIME_INACT register is set to a
    /// value less than the time constant of the output data rate. A value
    /// of 0 results in an interrupt when the output data is less than the
    /// value in the THRESH_INACT register."
    TIME_INACT = 0x26,

    /// Axis enable control for activity and inactivity detection (Read/Write)
    ///
    /// See data sheet for documentation (p.22)
    ACT_INACT_CTL = 0x27,

    /// Free-fall threshold (Read/Write)
    ///
    /// "The THRESH_FF register is eight bits and holds the threshold
    /// value, in unsigned format, for free-fall detection. The acceleration on
    /// all axes is compared with the value in THRESH_FF to determine if
    /// a free-fall event occurred. The scale factor is 62.5 mg/LSB. Note
    /// that a value of 0 mg may result in undesirable behavior if the
    /// free-fall interrupt is enabled. Values between 300 mg and 600 mg
    /// (0x05 to 0x09) are recommended."
    THRESH_FF = 0x28,

    /// Free-fall time (Read/Write)
    ///
    /// "The TIME_FF register is eight bits and stores an unsigned time
    /// value representing the minimum time that the value of all axes
    /// must be less than THRESH_FF to generate a free-fall interrupt.
    /// The scale factor is 5 ms/LSB. A value of 0 may result in undesirable
    /// behavior if the free-fall interrupt is enabled. Values between 100 ms
    /// and 350 ms (0x14 to 0x46) are recommended."
    TIME_FF = 0x29,

    /// Axis control for single/double tap (Read/Write)
    ///
    /// See data sheet for documentation (p.23)
    TAP_AXES = 0x2A,

    /// Source for single/double tap (Read Only)
    ///
    /// See data sheet for documentation (p.23)
    ACT_TAP_STATUS = 0x2B,

    /// Data rate and power mode control (Read/Write)
    ///
    /// See data sheet for documentation (p.23)
    BW_RATE = 0x2C,

    /// Power-saving features control (Read/Write)
    ///
    /// See data sheet for documentation (p.23)
    POWER_CTL = 0x2D,

    /// Interrupt enable control (Read/Write)
    ///
    /// See data sheet for table (p.24)
    ///
    /// "Setting bits in this register to a value of 1 enables their respective
    /// functions to generate interrupts, whereas a value of 0 prevents
    /// the functions from generating interrupts. The DATA_READY,
    /// watermark, and overrun bits enable only the interrupt output;
    /// the functions are always enabled. It is recommended that interrupts
    /// be configured before enabling their outputs."
    INT_ENABLE = 0x2E,

    /// Interrupt mapping control (Read/Write)
    ///
    /// See data sheet for table (p.24)
    ///
    /// "Any bits set to 0 in this register send their respective interrupts to
    /// the INT1 pin, whereas bits set to 1 send their respective interrupts
    /// to the INT2 pin. All selected interrupts for a given pin are OR’ed"
    INT_MAP = 0x2F,

    /// Source of interrupts (Read Only)
    ///
    /// See data sheet for table (p.24)
    ///
    /// "Bits set to 1 in this register indicate that their respective functions
    /// have triggered an event, whereas a value of 0 indicates that the
    /// corresponding event has not occurred. The DATA_READY,
    /// watermark, and overrun bits are always set if the corresponding
    /// events occur, regardless of the INT_ENABLE register settings,
    /// and are cleared by reading data from the DATAX, DATAY, and
    /// DATAZ registers. The DATA_READY and watermark bits may
    /// require multiple reads, as indicated in the FIFO mode descriptions
    /// in the FIFO section. Other bits, and the corresponding interrupts,
    /// are cleared by reading the INT_SOURCE register."
    INT_SOURCE = 0x30,

    /// Data format control (Read/Write)
    ///
    /// See `DataFormatFlags` below and data sheet for full documentation (p.24)
    ///
    /// "The DATA_FORMAT register controls the presentation of data
    /// to Register 0x32 through Register 0x37."
    DATA_FORMAT = 0x31,

    /// X-axis data 0 (Read Only)
    ///
    /// "These six bytes (Register 0x32 to Register 0x37) are eight bits
    /// each and hold the output data for each axis. Register 0x32 and
    /// Register 0x33 hold the output data for the x-axis, Register 0x34 and
    /// Register 0x35 hold the output data for the y-axis, and Register 0x36
    /// and Register 0x37 hold the output data for the z-axis. The output
    /// data is twos complement, with DATAx0 as the least significant
    /// byte and DATAx1 as the most significant byte, where x represent X,
    /// Y, or Z. The DATA_FORMAT register (Address 0x31) controls
    /// the format of the data. It is recommended that a multiple-byte
    /// read of all registers be performed to prevent a change in data
    /// between reads of sequential registers."
    DATAX0 = 0x32,

    /// X-axis data 1 (Read Only)
    ///
    /// See DATAX0 notes.
    DATAX1 = 0x33,

    /// Y-axis data 0 (Read Only)
    ///
    /// See DATAX0 notes.
    DATAY0 = 0x34,

    /// Y-axis data 1 (Read Only)
    ///
    /// See DATAX0 notes.
    DATAY1 = 0x35,

    /// Z-axis data 0 (Read Only)
    ///
    /// See DATAX0 notes.
    DATAZ0 = 0x36,

    /// Z-axis data 1 (Read Only)
    ///
    /// See DATAX0 notes.
    DATAZ1 = 0x37,

    /// FIFO control (Read/Write)
    ///
    /// See data sheet for documentation (p.25)
    FIFO_CTL = 0x38,

    /// FIFO status (Read Only)
    ///
    /// See data sheet for documentation (p.25)
    FIFO_STATUS = 0x39,
}

impl Register {
    /// Get register address
    pub fn addr(self) -> u8 {
        self as u8
    }

    /// Is the register read-only?
    pub fn read_only(self) -> bool {
        match self {
            Register::DEVID
            | Register::ACT_TAP_STATUS
            | Register::INT_SOURCE
            | Register::DATAX0
            | Register::DATAX1
            | Register::DATAY0
            | Register::DATAY1
            | Register::DATAZ0
            | Register::DATAZ1
            | Register::FIFO_STATUS => true,
            _ => false,
        }
    }
}

bitflags! {
    /// Flags passed as operands to `Register::DATA_FORMAT`
    ///
    /// "The DATA_FORMAT register controls the presentation of data
    /// to Register 0x32 through Register 0x37. All data, except that for
    /// the ±16 g range, must be clipped to avoid rollover."
    pub struct DataFormatFlags: u8 {
        /// "A setting of 1 in the SELF_TEST bit applies a self-test force to
        /// the sensor, causing a shift in the output data. A value of 0 disables
        /// the self-test force."
        const SELF_TEST = 0b10000000;

        /// "A value of 1 in the SPI bit sets the device to 3-wire SPI mode,
        /// and a value of 0 sets the device to 4-wire SPI mode"
        const SPI = 0b01000000;

        /// "A value of 0 in the INT_INVERT bit sets the interrupts to active
        /// high, and a value of 1 sets the interrupts to active low."
        const INT_INVERT = 0b00100000;

        /// "When this bit is set to a value of 1, the device is in full resolution
        /// mode, where the output resolution increases with the g range
        /// set by the range bits to maintain a 4 mg/LSB scale factor. When
        /// the FULL_RES bit is set to 0, the device is in 10-bit mode, and
        /// the range bits determine the maximum g range and scale factor"
        const FULL_RES = 0b00001000;

        /// A setting of 1 in the justify bit selects left-justified (MSB) mode,
        /// and a setting of 0 selects right-justified mode with sign extension.
        const JUSTIFY = 0b00000100;

        /// Range high bit (see `DataFormatRange`)
        const RANGE_HI = 0b00000010;

        /// Range low bit (see `DataFormatRange`)
        const RANGE_LO = 0b00000001;
    }
}

impl DataFormatFlags {
    /// Get the [`DataFormatRange`] from the flags
    pub fn range(self) -> DataFormatRange {
        if self.contains(DataFormatFlags::RANGE_HI) {
            if self.contains(DataFormatFlags::RANGE_LO) {
                DataFormatRange::PLUSMINUS_16G
            } else {
                DataFormatRange::PLUSMINUS_8G
            }
        } else if self.contains(DataFormatFlags::RANGE_LO) {
            DataFormatRange::PLUSMINUS_4G
        } else {
            DataFormatRange::PLUSMINUS_2G
        }
    }
}

/// Default `DATA_FORMAT` settings:
///
/// - `SELF_TEST`: false
/// - `SPI`: false
/// - `INT_INVERT`: false
/// - `FULL_RES`: false
/// - `JUSTIFY`: false
/// - Range: ±2g (i.e. 0)
impl Default for DataFormatFlags {
    fn default() -> Self {
        DataFormatFlags::empty()
    }
}

impl From<DataFormatRange> for DataFormatFlags {
    fn from(range: DataFormatRange) -> DataFormatFlags {
        range.bits()
    }
}

/// g-Range setting flags which can be OR'd with `DataFormatFlags` and passed as
/// operands to `Register::DATA_FORMAT`
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum DataFormatRange {
    /// ±2g
    PLUSMINUS_2G = 0b00,

    /// ±4g
    PLUSMINUS_4G = 0b01,

    /// ±8g
    PLUSMINUS_8G = 0b10,

    /// ±16g
    PLUSMINUS_16G = 0b11,
}

impl DataFormatRange {
    /// Get `DataFormatFlags` representation
    pub fn bits(self) -> DataFormatFlags {
        match self {
            DataFormatRange::PLUSMINUS_2G => DataFormatFlags::empty(),
            DataFormatRange::PLUSMINUS_4G => DataFormatFlags::RANGE_LO,
            DataFormatRange::PLUSMINUS_8G => DataFormatFlags::RANGE_HI,
            DataFormatRange::PLUSMINUS_16G => DataFormatFlags::RANGE_HI | DataFormatFlags::RANGE_LO,
        }
    }
}

impl From<DataFormatRange> for f32 {
    fn from(range: DataFormatRange) -> f32 {
        match range {
            DataFormatRange::PLUSMINUS_2G => 2.0,
            DataFormatRange::PLUSMINUS_4G => 4.0,
            DataFormatRange::PLUSMINUS_8G => 8.0,
            DataFormatRange::PLUSMINUS_16G => 16.0,
        }
    }
}
