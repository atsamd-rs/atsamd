//! Makes the pygamer appear as a USB serial port loop back device.
//! Repeats back all characters sent to it, but in upper case.

#![no_std]
#![no_main]

use bsp::{entry, hal, pac, Pins, RedLed};
#[cfg(not(feature = "panic_led"))]
use panic_halt as _;
use pygamer as bsp;

use hal::clock::v2::{clock_system_at_reset, dfll, gclk::Gclk, pclk::Pclk};
use hal::prelude::*;
use hal::usb::UsbBus;
use pac::{CorePeripherals, Peripherals};
use usb_device::bus::UsbBusAllocator;
use usb_device::prelude::*;
use usbd_serial::{SerialPort, USB_CLASS_CDC};

#[entry]
fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap();
    let mut _core = CorePeripherals::take().unwrap();
    let (mut buses, clocks, tokens) = clock_system_at_reset(
        peripherals.oscctrl,
        peripherals.osc32kctrl,
        peripherals.gclk,
        peripherals.mclk,
        &mut peripherals.nvmctrl,
    );

    let pins = Pins::new(peripherals.port).split();

    // Set up USB clocking
    let (dfll_usb, _) = clocks.dfll.into_mode(dfll::FromUsb, |_| {});
    // GCLK1 comes from DFLL, outputs to USB
    let (gclk_1, _) = Gclk::from_source(tokens.gclks.gclk1, dfll_usb);
    let gclk_1_48mhz = gclk_1.enable();
    let (pclk_usb, _) = Pclk::enable(tokens.pclks.usb, gclk_1_48mhz);

    let usb_bus = UsbBus::new(
        pclk_usb,
        clocks.ahbs.usb,
        buses.apb.enable(tokens.apbs.usb),
        pins.usb.dm,
        pins.usb.dp,
        peripherals.usb,
    )
    .unwrap();

    let bus_allocator = UsbBusAllocator::new(usb_bus);

    let mut serial = SerialPort::new(&bus_allocator);
    let mut led: RedLed = pins.led_pin.into();

    let mut usb_dev = UsbDeviceBuilder::new(&bus_allocator, UsbVidPid(0x1209, 0x0001))
        .strings(&[StringDescriptors::new(LangID::EN)
            .manufacturer("Fake company")
            .product("Serial port")
            .serial_number("TEST")])
        .expect("Failed to set strings")
        .device_class(USB_CLASS_CDC)
        .build();

    let mut led_state = false;
    let _ = led.set_low(); // Turn off

    loop {
        if !usb_dev.poll(&mut [&mut serial]) {
            continue;
        }

        let mut buf = [0u8; 64];

        match serial.read(&mut buf) {
            Ok(count) if count > 0 => {
                // Echo back in upper case
                for c in buf[0..count].iter_mut() {
                    led_state = if led_state {
                        let _ = led.set_low(); // Turn off
                        false
                    } else {
                        let _ = led.set_high(); // Turn on
                        true
                    };

                    if 0x61 <= *c && *c <= 0x7a {
                        *c &= !0x20;
                    }
                }

                let mut write_offset = 0;
                while write_offset < count {
                    match serial.write(&buf[write_offset..count]) {
                        Ok(len) if len > 0 => {
                            write_offset += len;
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }
}
