#![no_std]
#![no_main]

use bsp::hal;
use circuit_playground_express as bsp;
use core::mem::MaybeUninit;
use hal::rtc::rtic::rtc_clock;

#[cfg(not(feature = "use_semihosting"))]
use panic_halt as _;
#[cfg(feature = "use_semihosting")]
use panic_semihosting as _;

hal::rtc_monotonic!(Mono, rtc_clock::ClockCustom<8_192>);

#[rtic::app(device = bsp::pac, dispatchers = [EVSYS])]
mod app {
    use super::*;
    use usb_device::bus::UsbBusAllocator;
    use usb_device::prelude::*;
    use usbd_serial::{SerialPort, USB_CLASS_CDC};

    use bsp::pin_alias;
    use hal::clock::{ClockGenId, ClockSource, GenericClockController};
    use hal::pac::Peripherals;
    use hal::prelude::*;
    use hal::usb::UsbBus;

    #[local]
    struct Local {
        // usb_allocator: UsbBusAllocator<UsbBus>,
    }

    #[shared]
    struct Shared {
        // The LED could be a local resource, since it is only used in one task
        // But we want to showcase shared resources and locking
        red_led: bsp::RedLed,
        usb_bus: UsbDevice<'static, UsbBus>,
        usb_serial: SerialPort<'static, UsbBus>,
    }

    #[init(local=[usb_allocator: MaybeUninit<UsbBusAllocator<UsbBus>> = MaybeUninit::uninit()])]
    fn init(cx: init::Context) -> (Shared, Local) {
        let mut peripherals: Peripherals = cx.device;
        let mut core: rtic::export::Peripherals = cx.core;
        let mut clocks = GenericClockController::with_internal_32kosc(
            peripherals.gclk,
            &mut peripherals.pm,
            &mut peripherals.sysctrl,
            &mut peripherals.nvmctrl,
        );
        let pins = bsp::Pins::new(peripherals.port);

        let red_led: bsp::RedLed = pin_alias!(pins.red_led).into();

        *cx.local.usb_allocator = MaybeUninit::new(bsp::usb_allocator(
            peripherals.usb,
            &mut clocks,
            &mut peripherals.pm,
            pins.usb_dm,
            pins.usb_dp,
        ));
        // The usb allocator is initialized just above, which makes sure that
        // usb_allocator is allocated by this point. The reason it is done this
        // way is to avoid runtime checks for wheather the allocator is update
        // that would be required if we use an Option<UsbAllocator instead of
        // a MaybeUninit.
        let usb_allocator = unsafe { cx.local.usb_allocator.assume_init_ref() };
        let usb_serial = SerialPort::new(usb_allocator);
        let usb_bus = UsbDeviceBuilder::new(usb_allocator, UsbVidPid(0x16c0, 0x27dd))
            .strings(&[StringDescriptors::new(LangID::EN)
                .manufacturer("Fake company")
                .product("Serial port")
                .serial_number("TEST")])
            .expect("Failed to set strings")
            .device_class(USB_CLASS_CDC)
            .build();

        // Set the RTC clock to use a 8.192 kHz clock derived from the internal 32 kHz
        // oscillator.
        let rtc_clock_src = clocks
            .configure_gclk_divider_and_source(ClockGenId::Gclk2, 4, ClockSource::Osc32k, true)
            .unwrap();
        clocks.configure_standby(ClockGenId::Gclk2, true);
        let _ = clocks.rtc(&rtc_clock_src).unwrap();

        Mono::start(peripherals.rtc);

        core.SCB.set_sleepdeep();

        blink::spawn().unwrap();

        (
            Shared {
                red_led,
                usb_bus,
                usb_serial,
            },
            Local {},
        )
    }

    /// A task to blink the LED to show that life on the board.
    #[task(shared = [red_led])]
    async fn blink(mut cx: blink::Context) {
        loop {
            // If the LED were a local resource, the lock would not be necessary
            cx.shared.red_led.lock(|led| led.toggle().unwrap());

            Mono::delay(1u64.secs()).await;
        }
    }

    #[task(binds = USB, shared = [usb_bus, usb_serial])]
    fn poll_usb(cx: poll_usb::Context) {
        let mut serial = cx.shared.usb_serial;
        let mut usb_bus = cx.shared.usb_bus;

        (&mut serial, &mut usb_bus).lock(|s, b| {
            if !b.poll(&mut [s]) {
                return;
            }
            let mut buf = [0u8; 64];
            if let Ok(count) = s.read(&mut buf) {
                for (i, c) in buf.iter().enumerate() {
                    if i >= count {
                        break;
                    }
                    s.write(&[*c]).ok();
                }
            };
        })
    }
}
