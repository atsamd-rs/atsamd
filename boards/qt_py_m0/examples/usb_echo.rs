#![no_std]
#![no_main]

//! USB CDC example for the Adafruit QT Py board. Demonstrates creating a USB
//! CDC ACM serial port accessible over the on-board USB-C connector.

use panic_halt as _;

use usb_device::prelude::*;
use usbd_serial::SerialPort;
use usbd_serial::USB_CLASS_CDC;

use bsp::hal;
use bsp::pac;
use qt_py_m0 as bsp;

use bsp::entry;
use hal::clock::GenericClockController;
use pac::Peripherals;

#[entry]
fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap();
    let mut clocks = GenericClockController::with_internal_32kosc(
        peripherals.gclk,
        &mut peripherals.pm,
        &mut peripherals.sysctrl,
        &mut peripherals.nvmctrl,
    );
    let pins = bsp::Pins::new(peripherals.port).split();

    let usb_bus = pins.usb.init(peripherals.usb, &mut clocks, &mut peripherals.pm);

    let mut serial = SerialPort::new(&usb_bus);
    let mut usb_device = UsbDeviceBuilder::new(&usb_bus, UsbVidPid(0x239a, 0x00cb))
        .strings(&[StringDescriptors::new(LangID::EN)
            .manufacturer("Fake company")
            .product("Serial port")
            .serial_number("TEST")])
        .expect("Failed to set strings")
        .device_class(USB_CLASS_CDC)
        .build();

    loop {
        if !usb_device.poll(&mut [&mut serial]) {
            continue;
        }

        let mut buf = [0u8; 64];
        if let Ok(count) = serial.read(&mut buf) {
            let _ = serial.write(&buf[..count]);
        }
    }
}
