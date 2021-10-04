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
const APP: () = {
    struct Resources {
        led: bsp::Led,
        uart: bsp::Uart,
        delay: Delay,
    }
    #[init()]
    fn init(cx: init::Context) -> init::LateResources {
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

        init::LateResources { led, uart, delay }
    }

    #[idle(resources=[led, delay])]
    fn idle(mut cx: idle::Context) -> ! {
        // Flash the LED in a spin loop to confirm we are alive.
        loop {
            cx.resources.delay.delay_ms(200u32);
            cx.resources.led.lock(|l| l.toggle().unwrap());
        }
    }

    #[task(binds = SERCOM5, resources=[uart, led], priority = 2)]
    fn poll_uart(cx: poll_uart::Context) {
        match cx.resources.uart.read() {
            Ok(byte) => {
                cx.resources.led.toggle().unwrap();
                nb::block!(cx.resources.uart.write(byte)).unwrap();
            }
            Err(_) => {}
        };
    }
};
