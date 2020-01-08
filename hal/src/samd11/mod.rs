pub mod calibration;
pub mod clock;
pub mod pwm;
pub mod sercom;
pub mod timer;

mod serial_number;
pub use serial_number::*;

#[cfg(feature = "unproven")]
pub mod adc;

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
            1 => ResetCause::POR,
            2 => ResetCause::BOD12,
            4 => ResetCause::BOD33,
            16 => ResetCause::External,
            32 => ResetCause::Watchdog,
            64 => ResetCause::System,
            _ => ResetCause::Unknown,
        }
    }
}

/// Returns the cause of the last reset.
pub fn reset_cause<'a>(pm: &'a crate::target_device::PM) -> ResetCause {
    ResetCause::from(pm.rcause.read().bits())
}
