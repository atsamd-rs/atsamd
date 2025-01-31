#![no_std]
#![no_main]

use bsp::{hal, ButtonReader, Keys, Pins, RedLed};
#[cfg(not(feature = "panic_led"))]
use panic_halt as _;
use pygamer as bsp;

use hal::clock::GenericClockController;
use hal::prelude::*;
use hal::time::Hertz;
use hal::timer::TimerCounter;
use rtic::app;

#[app(device = bsp::pac, dispatchers = [EVSYS_0])]
mod app {
    use super::*;

    #[local]
    struct Resources {
        red_led: RedLed,
        timer: hal::timer::TimerCounter3,
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
        if InterruptDrivenTimer::wait(c.local.timer).is_ok() {
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
    fn init(c: init::Context) -> (Shared, Resources) {
        let mut device = c.device;
        let mut clocks = GenericClockController::with_internal_32kosc(
            device.gclk,
            &mut device.mclk,
            &mut device.osc32kctrl,
            &mut device.oscctrl,
            &mut device.nvmctrl,
        );

        let pins = Pins::new(device.port).split();

        let gclk0 = clocks.gclk0();
        let timer_clock = clocks.tc2_tc3(&gclk0).unwrap();

        let mut tc3 = TimerCounter::tc3_(&timer_clock, device.tc3, &mut device.mclk);

        InterruptDrivenTimer::start(&mut tc3, Hertz::Hz(200).into_duration());

        tc3.enable_interrupt();

        (
            Shared {},
            Resources {
                buttons: pins.buttons.init(),
                red_led: pins.led_pin.into(),
                timer: tc3,
            },
        )
    }
}
