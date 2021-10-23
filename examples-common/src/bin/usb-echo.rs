#![no_std]
#![no_main]

use bsp::{hal, pac, pac::interrupt};
use examples_common::*;

use hal::prelude::*;

use cortex_m::{asm::delay as cycle_delay, peripheral::NVIC};
use usb_device::{bus::UsbBusAllocator, prelude::*};
use usbd_serial::{SerialPort, USB_CLASS_CDC};

#[bsp::entry]
fn main() -> ! {
    let peripherals = pac::Peripherals::take().unwrap();
    let mut core = pac::CorePeripherals::take().unwrap();
    let (_clocks, led, usb) = OurBoard::init(peripherals);

    let bus_allocator = unsafe {
        USB_ALLOCATOR = Some(usb);
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
        for irq in USB_INTERRUPTS {
            core.NVIC.set_priority(irq, 1);
        }
        for irq in USB_INTERRUPTS {
            NVIC::unmask(irq);
        }
    }

    let mut red_led = led;

    loop {
        cycle_delay(5 * 1024 * 1024);
        red_led.toggle().unwrap();
    }
}

static mut USB_ALLOCATOR: Option<UsbBusAllocator<UsbBus>> = None;
static mut USB_BUS: Option<UsbDevice<UsbBus>> = None;
static mut USB_SERIAL: Option<SerialPort<UsbBus>> = None;

fn poll_usb() {
    unsafe {
        USB_BUS.as_mut().map(|usb_dev| {
            USB_SERIAL.as_mut().map(|serial| {
                if usb_dev.poll(&mut [serial]) {
                    let mut buf = [0u8; 64];

                    if let Ok(count) = serial.read(&mut buf) {
                        for (i, c) in buf.iter().enumerate() {
                            if i >= count {
                                break;
                            }
                            serial.write(&[c.clone()]).ok();
                        }
                    };
                }
            });
        });
    };
}

#[interrupt]
#[allow(non_snake_case)]
fn USB_OTHER() {
    poll_usb();
}

#[interrupt]
#[allow(non_snake_case)]
fn USB_TRCPT0() {
    poll_usb();
}

#[interrupt]
#[allow(non_snake_case)]
fn USB_TRCPT1() {
    poll_usb();
}
