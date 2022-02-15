#![no_std]
#![no_main]

use bsp::hal;
use p1am_100 as bsp;

#[cfg(not(feature = "use_semihosting"))]
use panic_halt as _;
#[cfg(feature = "use_semihosting")]
use panic_semihosting as _;

use usb_device;
use usbd_serial;

use hal::clock::GenericClockController;
use hal::delay::Delay;
use hal::prelude::*;

use hal::usb::UsbBus;
use usb_device::bus::UsbBusAllocator;

use usb_device::prelude::*;
use usbd_serial::{SerialPort, USB_CLASS_CDC};

#[rtic::app(device = p1am_100::pac, peripherals = true)]
mod app {
    use super::*;

    #[local]
    struct Local {
        led: bsp::Led,
        usb_serial: SerialPort<'static, UsbBus>,
        usb_dev: UsbDevice<'static, UsbBus>,
        delay: Delay,
    }

    #[shared]
    struct Shared {}
    #[init()]
    fn init(cx: init::Context) -> (Shared, Local, init::Monotonics) {
        static mut USB_ALLOCATOR: Option<UsbBusAllocator<UsbBus>> = None;

        let mut peripherals = cx.device;
        let mut clocks = GenericClockController::with_internal_32kosc(
            peripherals.GCLK,
            &mut peripherals.PM,
            &mut peripherals.SYSCTRL,
            &mut peripherals.NVMCTRL,
        );
        let pins = bsp::Pins::new(peripherals.PORT);
        let led: bsp::Led = pins.led.into();

        unsafe {
            USB_ALLOCATOR = Some(bsp::usb_allocator(
                peripherals.USB,
                &mut clocks,
                &mut peripherals.PM,
                pins.usb_dm.into(),
                pins.usb_dp.into(),
            ));
        }

        let usb_allocator = unsafe { USB_ALLOCATOR.as_ref().unwrap() };

        let usb_serial = SerialPort::new(&usb_allocator);
        let usb_dev = UsbDeviceBuilder::new(&usb_allocator, UsbVidPid(0x16c0, 0x27dd))
            .manufacturer("Fake company")
            .product("Serial port RTIC")
            .serial_number("TEST")
            .device_class(USB_CLASS_CDC)
            .build();

        let delay = Delay::new(cx.core.SYST, &mut clocks);

        (
            Shared {},
            Local {
                led,
                usb_serial,
                usb_dev,
                delay,
            },
            init::Monotonics(),
        )
    }

    #[idle(local=[led, delay])]
    fn idle(cx: idle::Context) -> ! {
        // Flash the LED in a spin loop to demonstrate that USB is
        // entirely interrupt driven.
        loop {
            cx.local.delay.delay_ms(200u32);
            cx.local.led.toggle().unwrap();
        }
    }

    #[task(binds = USB, local=[usb_dev, usb_serial], priority = 2)]
    fn poll_usb(cx: poll_usb::Context) {
        cx.local.usb_dev.poll(&mut [cx.local.usb_serial]);
        let mut buf = [0u8; 64];

        if let Ok(count) = cx.local.usb_serial.read(&mut buf) {
            for (i, c) in buf.iter().enumerate() {
                if i >= count {
                    break;
                }
                cx.local.usb_serial.write(&[c.clone()]).ok();
            }
        };
    }
}
