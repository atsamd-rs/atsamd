pub mod gpio;
pub mod timer;
pub mod trng;

mod reset_cause;
pub use reset_cause::*;

mod serial_number;
pub use serial_number::*;

#[cfg(feature = "unproven")]
pub mod adc;

#[cfg(feature = "unproven")]
pub mod pwm;

#[cfg(feature = "unproven")]
pub mod watchdog;

#[cfg(feature = "use_uart_debug")]
pub mod uart_debug;

#[cfg(feature = "use_uart_debug")]
#[macro_export]
macro_rules! dbgprint {
    ($($arg:tt)*) => {
        {
            use cortex_m::interrupt::free as disable_interrupts;
            disable_interrupts(|_| unsafe {
                {
                    use core::fmt::Write;
                    uart_debug::WRITER.write_fmt(format_args!($($arg)*)).unwrap();
                };
            });
        }
    };
}
