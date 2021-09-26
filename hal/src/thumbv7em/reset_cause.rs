/// ResetCause represents the reason the MCU was reset.
#[derive(Debug, Clone, Copy)]
pub enum ResetCause {
    Unknown,
    POR,
    BOD12,
    BOD33,
    NVM,
    External,
    Watchdog,
    System,
    Backup,
}

impl From<u8> for ResetCause {
    fn from(rcause_val: u8) -> ResetCause {
        match rcause_val {
            1 => Self::POR,
            2 => Self::BOD12,
            4 => Self::BOD33,
            8 => Self::NVM,
            16 => Self::External,
            32 => Self::Watchdog,
            64 => Self::System,
            128 => Self::Backup,
            _ => Self::Unknown,
        }
    }
}

/// Returns the cause of the last reset.
pub fn reset_cause(rstc: &crate::pac::RSTC) -> ResetCause {
    ResetCause::from(rstc.rcause.read().bits())
}
