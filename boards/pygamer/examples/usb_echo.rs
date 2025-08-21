//! Makes the pygamer appear as a USB serial port loop back device.
//! Repeats back all characters sent to it, but in upper case.

#![no_std]
#![no_main]
#![allow(static_mut_refs)]

use core::cell::OnceCell;
use cortex_m::asm::delay as cycle_delay;
use cortex_m::peripheral::NVIC;
#[cfg(not(feature = "panic_led"))]
use panic_halt as _;
use usb_device::bus::UsbBusAllocator;
use usb_device::prelude::*;
use usbd_serial::{SerialPort, USB_CLASS_CDC};

use pygamer as bsp;

use bsp::{entry, hal, pac, Pins};
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
        &mut peripherals.mclk,
        &mut peripherals.osc32kctrl,
        &mut peripherals.oscctrl,
        &mut peripherals.nvmctrl,
    );
    let pins = Pins::new(peripherals.port).split();
    let mut red_led: bsp::RedLed = pins.led_pin.into();

    let bus_allocator = unsafe {
        let _ = USB_ALLOCATOR.set(pins.usb.init(
            peripherals.usb,
            &mut clocks,
            &mut peripherals.mclk,
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
        core.NVIC.set_priority(interrupt::USB_OTHER, 1);
        core.NVIC.set_priority(interrupt::USB_TRCPT0, 1);
        core.NVIC.set_priority(interrupt::USB_TRCPT1, 1);
        NVIC::unmask(interrupt::USB_OTHER);
        NVIC::unmask(interrupt::USB_TRCPT0);
        NVIC::unmask(interrupt::USB_TRCPT1);
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
fn USB_OTHER() {
    poll_usb();
}

#[interrupt]
fn USB_TRCPT0() {
    poll_usb();
}

#[interrupt]
fn USB_TRCPT1() {
    poll_usb();
}
