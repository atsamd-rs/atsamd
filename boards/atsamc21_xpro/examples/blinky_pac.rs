#![no_std]
#![no_main]

use atsamc21_xpro as bsp;
use atsamc21j as pac;
use cortex_m::asm::delay as cycle_delay;

#[cfg(not(feature = "use_semihosting"))]
use panic_halt as _;
#[cfg(feature = "use_semihosting")]
use panic_semihosting as _;

use bsp::entry;

#[entry]
fn main() -> ! {
    let mut peripherals = pac::Peripherals::take().unwrap();
    let pa = &peripherals.PORT.group0;
    unsafe {
        pa.dirset.write(|w| w.bits(0x8000));
    }
    loop {
        // After reset clock runs at 4MHz
        cycle_delay(4_000_000 / 2);
        unsafe {
            pa.outtgl.write(|w| w.bits(0x8000));
        }
    }
}
