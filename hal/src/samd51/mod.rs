pub mod calibration;
pub mod clock;
pub mod pwm;
pub mod sercom;
pub mod timer;
pub mod trng;

#[cfg(feature="unproven")]
pub mod adc;

#[cfg(feature = "usb")]
pub mod usb;

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
