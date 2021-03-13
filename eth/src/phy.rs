/// Basic control register base address.
pub const PHY_REG_BCR: u8 = 0x00;

/// Soft reset.
/// This bit is self clearing. When setting this bit do 
/// not set other bits in the register.
///
/// # Value
///
/// Reset value = 1
pub const PHY_REG_BCR_SOFT_RESET: u16 = 1 << 15;

/// Loopback.
///
/// # Options
///
/// - Normal mode = 0
/// - Loopback mode = 1
pub const PHY_REG_BCR_LOOPBACK: u16 = 1 << 14;

/// Speed selection.
///
/// # Options
///
/// - 10 Mbps = 0
/// - 100 Mbps = 1
///
/// # Notes
///
/// Ignored if auto-negotiation is is enabled.
pub const PHY_REG_BCR_SPEED_SELECT: u16 = 1 << 13;

/// Auto-negotation enable.
///
/// # Options
///
/// - Disable = 0
/// - Enable = 1
pub const PHY_REG_BCR_AUTO_NEGOTIATION: u16 =  1 << 12;

/// Power down.
///
/// # Options
///
/// - Normal operation = 0
/// - General power down mode = 1
pub const PHY_REG_BCR_POWER_DOWN: u16 = 1 << 11;

/// Isolate
///
/// # Options
///
/// - Normal operation = 0
/// - Electrical isolation of PHY from the RMII = 1
pub const PHY_REG_BCR_ISOLATE: u16 = 1 << 10;

/// Restart auto negotiation.
///
/// # Options
///
/// - Normal operation = 0
/// - Restart auto-negotiation process = 1
///
/// # Notes
/// 
/// This bit is self clearing.
pub const PHY_REG_BCR_RESTART_AUTO_NEGOTIATE: u16 = 1 << 9;

/// Duplex mode
///
/// # Options
/// 
/// - Half duplex = 0
/// - Full duplex = 1
///
/// # Notes
///
/// Ignored if auto-negotiation is enabled.
pub const PHY_REG_BCR_DUPLEX_MODE: u16 = 1 << 8;

