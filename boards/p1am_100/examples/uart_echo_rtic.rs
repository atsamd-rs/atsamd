#![no_std]
#![no_main]

use bsp::hal;
use p1am_100 as bsp;

#[cfg(not(feature = "use_semihosting"))]
use panic_halt as _;
#[cfg(feature = "use_semihosting")]
use panic_semihosting as _;

use hal::clock::GenericClockController;
use hal::delay::Delay;
use hal::pac::Interrupt;
use hal::prelude::*;

#[rtic::app(device = p1am_100::pac, peripherals = true)]
mod app {
    use super::*;

    #[local]
    struct Local {
        uart: bsp::Uart,
        delay: Delay,
    }

    #[shared]
    struct Shared {
        led: bsp::Led,
    }
    #[init()]
    fn init(cx: init::Context) -> (Shared, Local, init::Monotonics) {
        let mut peripherals = cx.device;
        let mut clocks = GenericClockController::with_internal_32kosc(
            peripherals.GCLK,
            &mut peripherals.PM,
            &mut peripherals.SYSCTRL,
            &mut peripherals.NVMCTRL,
        );
        let pins = bsp::Pins::new(peripherals.PORT);
        let led: bsp::Led = pins.led.into();

        let delay = Delay::new(cx.core.SYST, &mut clocks);

        let mut uart = bsp::uart(
            &mut clocks,
            9600.hz(),
            peripherals.SERCOM5,
            &mut peripherals.PM,
            pins.d13.into(),
            pins.d14.into(),
        );

        uart.intenset(|w| {
            w.rxc().set_bit();
        });

        for byte in b"Hello, world!\r\n" {
            nb::block!(uart.write(*byte)).unwrap();
        }

        rtic::pend(Interrupt::SERCOM5);

        (Shared { led }, Local { uart, delay }, init::Monotonics())
    }

    #[idle(shared=[led], local=[delay])]
    fn idle(mut cx: idle::Context) -> ! {
        // Flash the LED in a spin loop to confirm we are alive.
        loop {
            cx.local.delay.delay_ms(200u32);
            cx.shared.led.lock(|l| l.toggle().unwrap());
        }
    }

    #[task(binds = SERCOM5, shared=[led], local=[uart], priority = 2)]
    fn poll_uart(mut cx: poll_uart::Context) {
        match cx.local.uart.read() {
            Ok(byte) => {
                cx.shared.led.lock(|l| l.toggle().unwrap());
                nb::block!(cx.local.uart.write(byte)).unwrap();
            }
            Err(_) => {}
        };
    }
}
