#![no_std]
#![no_main]

use panic_halt as _;

use bsp::{pins::ButtonReader, pins::Keys, Pins};
use edgebadge as bsp;

#[rtic::app(device = bsp::pac, peripherals = true)]
mod app {
    use super::*;

    use bsp::clock::GenericClockController;
    use bsp::gpio::{OpenDrain, Output, Pa23};
    use bsp::prelude::*;

    #[local]
    struct Local {
        red_led: Pa23<Output<OpenDrain>>,
        timer: bsp::timer::TimerCounter3,
        buttons: ButtonReader,
    }

    #[shared]
    struct Shared {}

    /// This function is called each time the tc3 interrupt triggers.
    /// We use it to toggle the LED.  The `wait()` call is important
    /// because it checks and resets the counter ready for the next
    /// period.
    #[task(binds = TC3, local = [timer, red_led, buttons])]
    fn tc3(c: tc3::Context) {
        if c.local.timer.wait().is_ok() {
            for event in c.local.buttons.events() {
                match event {
                    Keys::SelectDown => {
                        c.local.red_led.set_high().ok();
                    }
                    Keys::SelectUp => {
                        c.local.red_led.set_low().ok();
                    }
                    _ => {}
                }
            }
        }
    }

    #[init]
    fn init(c: init::Context) -> (Shared, Local, init::Monotonics) {
        let mut device = c.device;
        let mut clocks = GenericClockController::with_internal_32kosc(
            device.GCLK,
            &mut device.MCLK,
            &mut device.OSC32KCTRL,
            &mut device.OSCCTRL,
            &mut device.NVMCTRL,
        );

        let mut pins = Pins::new(device.PORT).split();

        let gclk0 = clocks.gclk0();
        let timer_clock = clocks.tc2_tc3(&gclk0).unwrap();

        let mut tc3 = bsp::timer::TimerCounter::tc3_(&timer_clock, device.TC3, &mut device.MCLK);

        tc3.start(200.hz());
        tc3.enable_interrupt();

        (
            Shared {},
            Local {
                buttons: pins.buttons.init(&mut pins.port),
                red_led: pins.led_pin.into_open_drain_output(&mut pins.port),
                timer: tc3,
            },
            init::Monotonics(),
        )
    }
}
