#![no_std]
#![no_main]

use bsp::hal;
use metro_m4 as bsp;

use panic_probe as _; //for probe-run
use rtt_target::{rprintln, rtt_init_print};
//use panic_rtt as _; //for cargo embed

use bsp::entry;
use hal::clock::GenericClockController;
use hal::pac::{CorePeripherals, Peripherals};

#[entry]
fn main() -> ! {
    rtt_init_print!();

    let mut peripherals = Peripherals::take().unwrap();
    let _core = CorePeripherals::take().unwrap();
    let _clocks = GenericClockController::with_external_32kosc(
        peripherals.GCLK,
        &mut peripherals.MCLK,
        &mut peripherals.OSC32KCTRL,
        &mut peripherals.OSCCTRL,
        &mut peripherals.NVMCTRL,
    );

    loop {
        // panic!('goodbye');
        rprintln!("hellooooo");
        cortex_m::asm::bkpt();
    }
}
