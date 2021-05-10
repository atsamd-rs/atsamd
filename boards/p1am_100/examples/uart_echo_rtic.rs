#![no_std]
#![no_main]

extern crate cortex_m;
extern crate p1am_100 as hal;
extern crate panic_halt;

use hal::clock::GenericClockController;
use hal::delay::Delay;
use hal::pac::Interrupt;
use hal::prelude::*;

use nb;
use nb::block;

#[rtic::app(device = hal::pac, peripherals = true)]
const APP: () = {
    struct Resources {
        led: hal::Led,
        uart: hal::Uart,
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
        let pins = hal::Pins::new(peripherals.PORT);
        let led: hal::Led = pins.led.into();

        let delay = Delay::new(cx.core.SYST, &mut clocks);

        let mut uart = hal::uart(
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
            block!(uart.write(*byte)).unwrap();
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
                block!(cx.resources.uart.write(byte)).unwrap();
            }
            Err(_) => {}
        };
    }
};
