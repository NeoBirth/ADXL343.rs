/// Register addresses
#[allow(dead_code, non_camel_case_types)]
#[derive(Copy, Clone, Debug)]
pub(crate) enum Register {
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
    pub(crate) fn addr(self) -> u8 {
        self as u8
    }
}
