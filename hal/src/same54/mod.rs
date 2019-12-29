pub mod calibration;
pub mod clock;
pub mod pwm;
pub mod sercom;
pub mod timer;
pub mod trng;
pub mod watchdog;

mod serial_number;
pub use serial_number::*;

//#[cfg(feature = "unproven")]
//pub mod adc;
//
//#[cfg(feature = "usb")]
//pub mod usb;
//
//#[cfg(feature = "use_uart_debug")]
//pub mod uart_debug;
//
//#[cfg(feature = "use_uart_debug")]
//#[macro_export]
//macro_rules! dbgprint {
//    ($($arg:tt)*) => {
//        {
//            use cortex_m::interrupt::free as disable_interrupts;
//            disable_interrupts(|_| unsafe {
//                {
//                    use core::fmt::Write;
//                    uart_debug::WRITER.write_fmt(format_args!($($arg)*)).unwrap();
//                };
//            });
//        }
//    };
//}

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
            1 => ResetCause::POR,
            2 => ResetCause::BOD12,
            4 => ResetCause::BOD33,
            8 => ResetCause::NVM,
            16 => ResetCause::External,
            32 => ResetCause::Watchdog,
            64 => ResetCause::System,
            128 => ResetCause::Backup,
            _ => ResetCause::Unknown,
        }
    }
}

/// Returns the cause of the last reset.
pub fn reset_cause<'a>(rstc: &'a crate::target_device::RSTC) -> ResetCause {
    ResetCause::from(rstc.rcause.read().bits())
}
