//! examples/blinky_rtic.rs
//! The blinky rtic example demonstrates a simple blinky application turning on
//! and off once each second using the RTIC library with the systick peripheral.
//! The blinky task will print and increment the local counter variable,
//! toggle the led, and spawn the blink task after another 500 ms.

#![no_main]
#![no_std]

#[cfg(not(feature = "use_semihosting"))]
use panic_halt as _;
#[cfg(feature = "use_semihosting")]
use panic_semihosting as _;

use rtic::app;

#[app(device=atsame54p, peripherals=true, dispatchers=[EVSYS_0])]
mod app {
    use atsame54_xpro as bsp;
    use bsp::hal;
    use hal::clock::GenericClockController;
    use hal::prelude::*;
    use rtt_target::{rprintln, rtt_init_print};
    use systick_monotonic::*;

    #[shared]
    struct SharedData {}

    #[local]
    struct LocalData {
        led: bsp::Led,
        count: i32,
    }

    #[monotonic(binds = SysTick, default = true)]
    type SysTickMonotonic = Systick<1000>;

    #[init]
    fn init(cx: init::Context) -> (SharedData, LocalData, init::Monotonics) {
        rtt_init_print!();

        let core = cx.core;
        let mut device: atsame54p::Peripherals = cx.device;

        let _clocks = GenericClockController::with_external_32kosc(
            device.GCLK,
            &mut device.MCLK,
            &mut device.OSC32KCTRL,
            &mut device.OSCCTRL,
            &mut device.NVMCTRL,
        );

        let mono = SysTickMonotonic::new(core.SYST, 120_000_000);
        let pins = bsp::Pins::new(device.PORT);
        let led = bsp::pin_alias!(pins.led).into_push_pull_output();
        let count: i32 = 0;

        blink::spawn().unwrap();

        (
            SharedData {},
            LocalData { led, count },
            init::Monotonics(mono),
        )
    }

    #[task(local=[led, count], priority=2)]
    fn blink(cx: blink::Context) {
        rprintln!("{}", cx.local.count);
        *cx.local.count += 1;
        cx.local.led.toggle().unwrap();
        blink::spawn_after(500.millis()).unwrap();
    }

    #[idle]
    fn idle(_cx: idle::Context) -> ! {
        loop {}
    }
}
