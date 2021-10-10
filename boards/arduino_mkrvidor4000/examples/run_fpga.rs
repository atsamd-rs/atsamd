#![no_std]
#![no_main]

use arduino_mkrvidor4000 as bsp;
use bsp::hal;

#[cfg(not(feature = "use_semihosting"))]
use panic_halt as _;
#[cfg(feature = "use_semihosting")]
use panic_semihosting as _;

use bsp::entry;
use hal::clock::GenericClockController;
use hal::pac::{CorePeripherals, Peripherals};

#[entry]
fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap();
    let _corecore = CorePeripherals::take().unwrap();
    let _clocks = GenericClockController::with_external_32kosc(
        peripherals.GCLK,
        &mut peripherals.PM,
        &mut peripherals.SYSCTRL,
        &mut peripherals.NVMCTRL,
    );
    let mut pins = bsp::Pins::new(peripherals.PORT);

    // Enable 48MHZ clock output for FPGA
    // https://github.com/arduino/ArduinoCore-samd/blob/master/variants/mkrvidor4000/variant.cpp#L229
    let _fpga_clk = pins.gclk.into_function_h(&mut pins.port);

    loop {}
}
