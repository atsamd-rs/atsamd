/// ResetCause represents the reason the MCU was reset.
#[derive(Debug, Clone, Copy)]
pub enum ResetCause {
    Unknown,
    POR,
    BOD12,
    BOD33,
    External,
    Watchdog,
    System,
}

impl From<u8> for ResetCause {
    fn from(rcause_val: u8) -> ResetCause {
        match rcause_val {
            1 => Self::POR,
            2 => Self::BOD12,
            4 => Self::BOD33,
            16 => Self::External,
            32 => Self::Watchdog,
            64 => Self::System,
            _ => Self::Unknown,
        }
    }
}

/// Returns the cause of the last reset.
pub fn reset_cause(pm: &crate::pac::PM) -> ResetCause {
    ResetCause::from(pm.rcause.read().bits())
}
