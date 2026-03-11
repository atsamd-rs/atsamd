#![no_std]
#![no_main]
#![allow(static_mut_refs)]

use panic_halt as _;

use core::cell::OnceCell;
use cortex_m::asm::delay as cycle_delay;
use cortex_m::peripheral::NVIC;
use usb_device::bus::UsbBusAllocator;
use usb_device::prelude::*;
use usbd_serial::{SerialPort, USB_CLASS_CDC};

use bsp::hal;
use bsp::pac;
use metro_m0 as bsp;

use bsp::entry;
use hal::clock::GenericClockController;
use hal::prelude::*;
use hal::usb::UsbBus;
use pac::{interrupt, CorePeripherals, Peripherals};

#[entry]
fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap();
    let mut core = CorePeripherals::take().unwrap();
    let mut clocks = GenericClockController::with_internal_32kosc(
        peripherals.gclk,
        &mut peripherals.pm,
        &mut peripherals.sysctrl,
        &mut peripherals.nvmctrl,
    );
    let pins = bsp::Pins::new(peripherals.port);
    let mut red_led: bsp::RedLed = pins.d13.into();

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

    unsafe {
        let _ = USB_SERIAL.set(SerialPort::new(bus_allocator));
        let _ = USB_BUS.set(
            UsbDeviceBuilder::new(bus_allocator, UsbVidPid(0x16c0, 0x27dd))
                .strings(&[StringDescriptors::new(LangID::EN)
                    .manufacturer("Fake company")
                    .product("Serial port")
                    .serial_number("TEST")])
                .expect("Failed to set strings")
                .device_class(USB_CLASS_CDC)
                .build(),
        );
    }

    unsafe {
        core.NVIC.set_priority(interrupt::USB, 1);
        NVIC::unmask(interrupt::USB);
    }

    // Flash the LED in a spin loop to demonstrate that USB is
    // entirely interrupt driven.
    loop {
        cycle_delay(15 * 1024 * 1024);
        red_led.toggle().ok();
    }
}

static mut USB_ALLOCATOR: OnceCell<UsbBusAllocator<UsbBus>> = OnceCell::new();
static mut USB_BUS: OnceCell<UsbDevice<UsbBus>> = OnceCell::new();
static mut USB_SERIAL: OnceCell<SerialPort<UsbBus>> = OnceCell::new();

fn poll_usb() {
    unsafe {
        if let Some(usb_bus) = USB_BUS.get_mut() {
            if let Some(serial) = USB_SERIAL.get_mut() {
                usb_bus.poll(&mut [serial]);
                let mut buf = [0u8; 64];

                if let Ok(count) = serial.read(&mut buf) {
                    for (i, c) in buf.iter().enumerate() {
                        if i >= count {
                            break;
                        }
                        serial.write(&[*c]).ok();
                    }
                };
            };
        }
    };
}

#[interrupt]
fn USB() {
    poll_usb();
}
