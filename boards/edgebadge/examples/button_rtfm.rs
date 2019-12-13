#![no_std]
#![no_main]

#[allow(unused_imports)]
use panic_halt;

use edgebadge as hal;

use hal::clock::GenericClockController;
use hal::gpio::{OpenDrain, Output, Pa23};
use hal::pins::Keys;
use hal::prelude::*;
use rtfm::app;

#[app(device = crate::hal::pac)]
const APP: () = {
    static mut RED_LED: Pa23<Output<OpenDrain>> = ();
    static mut TIMER: hal::timer::TimerCounter3 = ();
    static mut BUTTONS: hal::pins::ButtonReader = ();

    /// This function is called each time the tc3 interrupt triggers.
    /// We use it to toggle the LED.  The `wait()` call is important
    /// because it checks and resets the counter ready for the next
    /// period.
    #[interrupt(resources = [TIMER,RED_LED, BUTTONS])]
    fn TC3() {
        if resources.TIMER.wait().is_ok() {
            for event in resources.BUTTONS.events() {
                match event {
                    Keys::SelectDown => {
                        resources.RED_LED.set_high().ok();
                    }
                    Keys::SelectUp => {
                        resources.RED_LED.set_low().ok();
                    }
                    _ => {}
                }
            }
        }
    }

    #[init]
    fn init() {
        let mut clocks = GenericClockController::with_internal_32kosc(
            device.GCLK,
            &mut device.MCLK,
            &mut device.OSC32KCTRL,
            &mut device.OSCCTRL,
            &mut device.NVMCTRL,
        );

        let mut pins = hal::Pins::new(device.PORT).split();

        let gclk0 = clocks.gclk0();
        let timer_clock = clocks.tc2_tc3(&gclk0).unwrap();

        let mut tc3 = hal::timer::TimerCounter::tc3_(&timer_clock, device.TC3, &mut device.MCLK);

        tc3.start(200.hz());
        tc3.enable_interrupt();

        BUTTONS = pins.buttons.init(&mut pins.port);
        RED_LED = pins.led_pin.into_open_drain_output(&mut pins.port);
        TIMER = tc3;
    }
};
