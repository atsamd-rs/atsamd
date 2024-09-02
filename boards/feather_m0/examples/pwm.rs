#![no_std]
#![no_main]

use cortex_m_rt::entry;

#[cfg(not(feature = "use_semihosting"))]
use panic_halt as _;
#[cfg(feature = "use_semihosting")]
use panic_semihosting as _;

use bsp::hal;
use bsp::pac;
use feather_m0 as bsp;

use bsp::pin_alias;
use hal::clock::GenericClockController;
use hal::delay::Delay;
use hal::ehal::{delay::DelayNs, pwm::SetDutyCycle};
use hal::fugit::RateExtU32;
use hal::pwm::Pwm3;
use pac::{CorePeripherals, Peripherals};

#[entry]
fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap();
    let core = CorePeripherals::take().unwrap();

    let mut clocks = GenericClockController::with_external_32kosc(
        peripherals.gclk,
        &mut peripherals.pm,
        &mut peripherals.sysctrl,
        &mut peripherals.nvmctrl,
    );
    let mut delay = Delay::new(core.SYST, &mut clocks);
    let pins = bsp::Pins::new(peripherals.port);

    let _d5: bsp::D5Pwm = pin_alias!(pins.d5_pwm).into();

    let gclk0 = clocks.gclk0();
    let mut pwm3 = Pwm3::new(
        &clocks.tcc2_tc3(&gclk0).unwrap(),
        1.kHz(),
        peripherals.tc3,
        &mut peripherals.pm,
    );
    let max_duty = pwm3.max_duty_cycle();

    loop {
        // The embedded-hal spec requires that set_duty_cycle returns a Result.
        // In our case, the function is infaillible so we can safely ignore the result.
        let _ = pwm3.set_duty_cycle(max_duty / 2);
        delay.delay_ms(1000);
        let _ = pwm3.set_duty_cycle(max_duty / 8);
        delay.delay_ms(1000);
    }
}
