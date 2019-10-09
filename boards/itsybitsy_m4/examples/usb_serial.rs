#![no_std]
#![no_main]

extern crate itsybitsy_m4 as hal;
extern crate panic_halt;

use hal::clock::GenericClockController;

use hal::entry;
use hal::pac::{interrupt, CorePeripherals, Peripherals};

use hal::usb::UsbBus;
use usb_device::bus::UsbBusAllocator;

use usb_device::prelude::*;
use hal::prelude::*;

use hal::{uart, uart_debug};
use hal::dbgprint;
use hal::time::Hertz;

#[entry]
fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap();
    let mut core = CorePeripherals::take().unwrap();
    let mut clocks = GenericClockController::with_internal_32kosc(
        peripherals.GCLK,
        &mut peripherals.MCLK,
        &mut peripherals.OSC32KCTRL,
        &mut peripherals.OSCCTRL,
        &mut peripherals.NVMCTRL,
    );
    let mut pins = hal::Pins::new(peripherals.PORT).split();

    uart_debug::wire_uart(uart(pins.uart, &mut clocks, Hertz(115200), peripherals.SERCOM3, &mut peripherals.MCLK, &mut pins.port));
    dbgprint!("\n\n\n\n~========== STARTING ==========~\n");

    let bus_allocator = unsafe {
        USB_ALLOCATOR = Some(pins.usb.usb_allocator(peripherals.USB, &mut clocks, &mut peripherals.MCLK, &mut pins.port));
        USB_ALLOCATOR.as_ref().unwrap()
    };

    unsafe {
        USB_BUS = Some(UsbDeviceBuilder::new(&bus_allocator, UsbVidPid(0x16c0, 0x27dd))
            .manufacturer("Fake company")
            .product("Serial port")
            .serial_number("TEST")
            // .device_class(0)
            .build());
    }


    unsafe {
        core.NVIC.set_priority(interrupt::USB_OTHER, 1);
        core.NVIC.set_priority(interrupt::USB_TRCPT0, 1);
        core.NVIC.set_priority(interrupt::USB_TRCPT1, 1);
        core.NVIC.set_priority(interrupt::USB_TRCPT1, 1);
        core.NVIC.set_priority(interrupt::USB_SOF_HSOF, 1);
    }
    core.NVIC.enable(interrupt::USB_OTHER);
    core.NVIC.enable(interrupt::USB_TRCPT0);
    core.NVIC.enable(interrupt::USB_TRCPT1);
    core.NVIC.enable(interrupt::USB_SOF_HSOF);

    loop {

    }
}

static mut USB_ALLOCATOR: Option<UsbBusAllocator<UsbBus>> = None;
static mut USB_BUS: Option<UsbDevice<UsbBus>> = None;
 fn poll_usb() {
    unsafe {
        USB_BUS.as_mut().map(|usb_dev| {
            usb_dev.poll(&mut []);
        });
    };
}

#[interrupt]
fn USB_SOF_HSOF() {
    poll_usb();
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
