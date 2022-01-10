#![no_std]
#![no_main]
use panic_halt as _;

use cortex_m::asm::delay as cycle_delay;
use cortex_m::peripheral::NVIC;
use usb_device::bus::UsbBusAllocator;
use usb_device::prelude::*;
use usbd_serial::{SerialPort, USB_CLASS_CDC};

use wio_lite_w600 as bsp;

use bsp::entry;
use bsp::hal;
use bsp::pac;

use hal::clock::GenericClockController;
use hal::usb::UsbBus;
use pac::{interrupt, CorePeripherals, Peripherals};

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
    let pins = bsp::Pins::new(peripherals.PORT);

    let bus_allocator = unsafe {
        USB_ALLOCATOR = Some(bsp::usb_allocator(
            peripherals.USB,
            &mut clocks,
            &mut peripherals.PM,
            pins.usb_dm,
            pins.usb_dp,
        ));
        USB_ALLOCATOR.as_ref().unwrap()
    };

    unsafe {
        USB_SERIAL = Some(SerialPort::new(&bus_allocator));
        USB_BUS = Some(
            UsbDeviceBuilder::new(&bus_allocator, UsbVidPid(0x16c0, 0x27dd))
                .manufacturer("Fake company")
                .product("Serial port")
                .serial_number("WIO_LITE_W600")
                .device_class(USB_CLASS_CDC)
                .build(),
        );
    }

    unsafe {
        core.NVIC.set_priority(interrupt::USB, 1);
        NVIC::unmask(interrupt::USB);
    }

    loop {
        cycle_delay(15 * 1024 * 1024);
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
                    for (i, c) in buf.iter().enumerate() {
                        if i >= count {
                            break;
                        }
                        serial.write("Received: ".as_bytes()).ok();
                        serial.write(&[c.clone()]).ok();
                        serial.write("\r\n".as_bytes()).ok();
                    }
                };
            });
        });
    };
}

#[interrupt]
fn USB() {
    // Note: USB is the name of the interrupt, you can not attach the #[interrupt]
    // tag to poll_usb. Although you could add the contents of poll_usb into
    // this function, separating them allows you to add more functions to run on
    // the USB interrupt in the future.
    poll_usb();
}
