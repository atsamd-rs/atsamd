#![no_std]
#![no_main]
#![allow(static_mut_refs)]

use panic_halt as _;

use core::cell::OnceCell;
use rtic::app;
use usb_device::bus::UsbBusAllocator;
use usb_device::prelude::*;
use usbd_serial::{SerialPort, USB_CLASS_CDC};

use bsp::hal;
use metro_m0 as bsp;

use bsp::RedLed;
use hal::clock::{ClockGenId, ClockSource, GenericClockController};
use hal::prelude::*;
use hal::rtc::rtic::rtc_clock;
use hal::usb::UsbBus;

hal::rtc_monotonic!(Mono, rtc_clock::ClockCustom<2_048>);

#[app(device = bsp::pac, dispatchers = [EVSYS])]
mod app {
    use super::*;

    static mut USB_ALLOCATOR: OnceCell<UsbBusAllocator<UsbBus>> = OnceCell::new();

    #[local]
    struct Resources {
        usb_bus: UsbDevice<'static, UsbBus>,
        usb_serial: SerialPort<'static, UsbBus>,
        red_led: RedLed,
    }

    #[shared]
    struct Shared {}

    #[init]
    fn init(ctx: init::Context) -> (Shared, Resources) {
        let mut peripherals = ctx.device;
        let mut clocks = GenericClockController::with_internal_32kosc(
            peripherals.gclk,
            &mut peripherals.pm,
            &mut peripherals.sysctrl,
            &mut peripherals.nvmctrl,
        );
        let pins = bsp::Pins::new(peripherals.port);
        let red_led: bsp::RedLed = pins.d13.into();

        // Set the RTC clock to use a 2.048 kHz clock derived from the external
        // 32 kHz oscillator.
        let rtc_clock_src = clocks
            .configure_gclk_divider_and_source(ClockGenId::Gclk2, 16, ClockSource::Xosc32k, false)
            .unwrap();
        clocks.configure_standby(ClockGenId::Gclk2, true);
        let _ = clocks.rtc(&rtc_clock_src).unwrap();

        // Start the monotonic
        Mono::start(peripherals.rtc);

        // Setup the USB
        let bus_allocator = unsafe {
            let _ = USB_ALLOCATOR.set(bsp::usb_allocator(
                peripherals.usb,
                &mut clocks,
                &mut peripherals.pm,
                pins.usb_dm,
                pins.usb_dp,
            ));
            USB_ALLOCATOR.get().unwrap()
        };

        let usb_serial = SerialPort::new(bus_allocator);
        let usb_bus = UsbDeviceBuilder::new(bus_allocator, UsbVidPid(0x16c0, 0x27dd))
            .strings(&[StringDescriptors::new(LangID::EN)
                .manufacturer("Fake company")
                .product("Serial port")
                .serial_number("TEST")])
            .expect("Failed to set strings")
            .device_class(USB_CLASS_CDC)
            .build();

        // Spawn the LED task
        blink_led::spawn().unwrap();

        (
            Shared {},
            Resources {
                usb_bus,
                usb_serial,
                red_led,
            },
        )
    }

    // Task  to blink the LED
    #[task(priority = 1, local=[red_led])]
    async fn blink_led(ctx: blink_led::Context) {
        loop {
            ctx.local.red_led.toggle().unwrap();
            Mono::delay(500u64.millis()).await;
        }
    }

    // Polling interrupt handler that simply echoes back whatever bytes were
    // received
    #[task(binds = USB, local = [usb_bus, usb_serial])]
    fn poll(ctx: poll::Context) {
        ctx.local.usb_bus.poll(&mut [ctx.local.usb_serial]);
        let mut buf = [0u8; 64];

        if let Ok(count) = ctx.local.usb_serial.read(&mut buf) {
            for (i, c) in buf.iter().enumerate() {
                if i >= count {
                    break;
                }
                ctx.local.usb_serial.write(&[*c]).ok();
            }
        };
    }
}
