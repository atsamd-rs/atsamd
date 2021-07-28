#![no_std]
#![no_main]

//! USB CDC example for the Adafruit QT Py board. Demonstrates creating a USB
//! CDC ACM serial port accessible over the on-board USB-C connector.

use panic_halt as _;

use cortex_m::asm::wfi;
use cortex_m::peripheral::NVIC;
use usb_device::bus::UsbBusAllocator;
use usb_device::prelude::*;
use usbd_serial::SerialPort;
use usbd_serial::USB_CLASS_CDC;

use bsp::hal;
use bsp::pac;
use qt_py_m0 as bsp;

use bsp::entry;
use hal::clock::GenericClockController;
use hal::usb::UsbBus;
use pac::interrupt;
use pac::CorePeripherals;
use pac::Peripherals;

#[entry]
fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap();
    let mut core = CorePeripherals::take().unwrap();
    let mut clocks = GenericClockController::with_internal_32kosc(
        peripherals.GCLK,
        &mut peripherals.PM,
        &mut peripherals.SYSCTRL,
        &mut peripherals.NVMCTRL,
    );
    let pins = bsp::Pins::new(peripherals.PORT).split();

    let bus_allocator = unsafe {
        USB_ALLOCATOR = Some(
            pins.usb
                .init(peripherals.USB, &mut clocks, &mut peripherals.PM),
        );
        USB_ALLOCATOR.as_ref().unwrap()
    };

    unsafe {
        USB_SERIAL = Some(SerialPort::new(&bus_allocator));
        USB_BUS = Some(
            UsbDeviceBuilder::new(&bus_allocator, UsbVidPid(0x16c0, 0x27dd))
                .manufacturer("Fake company")
                .product("Serial port")
                .serial_number("TEST")
                .device_class(USB_CLASS_CDC)
                .build(),
        );
    }

    unsafe {
        core.NVIC.set_priority(interrupt::USB, 1);
        NVIC::unmask(interrupt::USB);
    }

    loop {
        wfi();
    }
}

static mut USB_ALLOCATOR: Option<UsbBusAllocator<UsbBus>> = None;
static mut USB_BUS: Option<UsbDevice<UsbBus>> = None;
static mut USB_SERIAL: Option<SerialPort<UsbBus>> = None;

fn poll_usb() {
    unsafe {
        USB_BUS.as_mut().map(|usb_dev| {
            USB_SERIAL.as_mut().map(|serial| {
                usb_dev.poll(&mut [serial]);
                let mut buf = [0u8; 64];

                if let Ok(count) = serial.read(&mut buf) {
                    serial.write(&buf[..count]).ok();
                };
            });
        });
    };
}

#[interrupt]
fn USB() {
    poll_usb();
}
