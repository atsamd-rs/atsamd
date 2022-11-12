//! examples/edge_counter.rs
//! The edge counter example demonstrates a simple interrupt-based
//! application using the RTIC library with two primary tasks.
//! The interrupting task will increment the shared counter variable when the
//! SAME54 XPlained Pro user button (SW0) is pressed, and spawn the blink task.
//! The blink task will toggle the LED, and print the shared counter variable
//! to the user's console if connected via the real time transfer protocol.

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
    use hal::eic;
    use hal::eic::pin::*;
    use hal::prelude::*;
    use rtt_target::{rprintln, rtt_init_print};

    hal::bsp_pins!(PB31 {
        aliases: { PullUpInterrupt: IntButton }
    });

    #[shared]
    struct SharedData {
        count: i32,
    }

    #[local]
    struct LocalData {
        led: bsp::Led,
        button: ExtInt15<IntButton>,
    }

    #[init]
    fn init(cx: init::Context) -> (SharedData, LocalData, init::Monotonics) {
        rtt_init_print!();

        let _core = cx.core;
        let mut device: atsame54p::Peripherals = cx.device;

        let mut clocks = GenericClockController::with_external_32kosc(
            device.GCLK,
            &mut device.MCLK,
            &mut device.OSC32KCTRL,
            &mut device.OSCCTRL,
            &mut device.NVMCTRL,
        );

        let pins = bsp::Pins::new(device.PORT);
        let led = bsp::pin_alias!(pins.led).into_push_pull_output();
        let mut button = bsp::pin_alias!(pins.button).into_pull_up_ei();
        let count: i32 = 0;

        let gclk0 = GenericClockController::gclk0(&mut clocks);
        let eic_clock = GenericClockController::eic(&mut clocks, &gclk0).unwrap();
        let mut cfg_eic = eic::init_with_ulp32k(&mut device.MCLK, eic_clock, device.EIC);

        // TODO: The current method of getting the button ID is not working,
        // so the button interrupt ID, equal to 15, is hard-coded in.
        let button_interrupt_id = 15;
        // let button_interrupt_id = button.id();

        cfg_eic.button_debounce_pins(&[button_interrupt_id]);
        button.sense(&mut cfg_eic, Sense::FALL);
        button.enable_interrupt(&mut cfg_eic);
        cfg_eic.finalize();

        (
            SharedData { count },
            LocalData { led, button },
            init::Monotonics(),
        )
    }

    #[task(binds=EIC_EXTINT_15, local=[button], shared=[count])]
    fn on_interrupt(mut cx: on_interrupt::Context) {
        cx.local.button.clear_interrupt();
        cx.shared.count.lock(|count| *count += 1);
        blink::spawn().ok();
    }

    #[task(local=[led], shared=[count])]
    fn blink(mut cx: blink::Context) {
        rprintln!("{}", cx.shared.count.lock(|count| *count));
        cx.local.led.toggle().unwrap();
    }

    #[idle]
    fn idle(_cx: idle::Context) -> ! {
        loop {}
    }
}
