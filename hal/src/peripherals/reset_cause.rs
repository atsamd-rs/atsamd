use atsamd_hal_macros::{hal_cfg, hal_macro_helper};

/// ResetCause represents the reason the MCU was reset.
#[hal_macro_helper]
#[derive(Debug, Clone, Copy)]
pub enum ResetCause {
    Unknown,
    POR,
    BOD12,
    BOD33,
    #[hal_cfg("rstc-d5x")]
    NVM,
    External,
    Watchdog,
    System,
    #[hal_cfg("rstc-d5x")]
    Backup,
}

impl From<u8> for ResetCause {
    #[hal_macro_helper]
    fn from(rcause_val: u8) -> ResetCause {
        match rcause_val {
            1 => Self::POR,
            2 => Self::BOD12,
            4 => Self::BOD33,
            #[hal_cfg("rstc-d5x")]
            8 => Self::NVM,
            16 => Self::External,
            32 => Self::Watchdog,
            64 => Self::System,
            #[hal_cfg("rstc-d5x")]
            128 => Self::Backup,
            _ => Self::Unknown,
        }
    }
}

/// Returns the cause of the last reset.
#[hal_cfg(any("pm-d11", "pm-d21"))]
pub fn reset_cause(pm: &crate::pac::Pm) -> ResetCause {
    ResetCause::from(pm.rcause().read().bits())
}

/// Returns the cause of the last reset.
#[hal_cfg("rstc-d5x")]
pub fn reset_cause(rstc: &crate::pac::Rstc) -> ResetCause {
    ResetCause::from(rstc.rcause().read().bits())
}
