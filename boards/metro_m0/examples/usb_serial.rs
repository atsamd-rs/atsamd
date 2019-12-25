#![no_std]
#![no_main]

extern crate metro_m0 as hal;
extern crate panic_rtt;

use cortex_m_rt::{exception, ExceptionFrame};
use hal::clock::GenericClockController;
use hal::prelude::*;
use hal::{entry};
use hal::pac::{interrupt, Interrupt, CorePeripherals, Peripherals};

use hal::usb::UsbBus;
use hal::usb_allocator;
use usb_device::bus::UsbBusAllocator;
use usb_device::prelude::*;
use usbd_serial::{SerialPort, USB_CLASS_CDC};

static mut USB_ALLOCATOR: Option<UsbBusAllocator<UsbBus>> = None;
static mut USB_BUS: Option<UsbDevice<UsbBus>> = None;
static mut USB_SERIAL: Option<SerialPort<UsbBus>> = None;

#[entry]
fn main() -> ! {
    //dbgprint!("main entered");
    let mut peripherals = Peripherals::take().unwrap();
    let mut clocks = GenericClockController::with_external_32kosc(
        peripherals.GCLK,
        &mut peripherals.PM,
        &mut peripherals.SYSCTRL,
        &mut peripherals.NVMCTRL,
    );
    let mut pins = hal::Pins::new(peripherals.PORT);
    let mut core = CorePeripherals::take().unwrap();
    /* let mut delay = Delay::new(core.SYST, &mut clocks);
     */
    let mut red_led = pins.d13.into_open_drain_output(&mut pins.port);
    red_led.set_high().unwrap();

    //dbgprint!("make usb_bus");

    let bus_allocator = unsafe {
        USB_ALLOCATOR = Some(usb_allocator(
                peripherals.USB,
                &mut clocks,
                &mut peripherals.PM,
                pins.usb_dm,
                pins.usb_dp,
                &mut pins.port,
        ));
        USB_ALLOCATOR.as_ref().unwrap()
    };

    unsafe {
        USB_BUS = Some(
            UsbDeviceBuilder::new(&bus_allocator, UsbVidPid(0x16c0, 0x27dd))
                .manufacturer("Fake company")
                .product("Serial port")
                .serial_number("TEST")
                .device_class(USB_CLASS_CDC)
                .build(),
            );
    };

    //dbgprint!("make serial");
    unsafe {
        USB_SERIAL = Some(SerialPort::new(&bus_allocator));
    };

    unsafe {
        core.NVIC.set_priority(Interrupt::USB, 0);
    }
    core.NVIC.enable(Interrupt::USB);

    //dbgprint!("do loop");
    loop {}
}

fn poll_usb() {
    unsafe {
        match (USB_BUS.as_mut(), USB_SERIAL.as_mut()) {
            (Some(usb_dev), Some(serial)) => {
                usb_dev.poll(&mut [serial]);

                let mut buf = [0u8; 8];

                match serial.read(&mut buf) {
                    Ok(count) if count > 0 => {
                        // Echo back in upper case
                        for c in buf[0..count].iter_mut() {
                            if 0x61 <= *c && *c <= 0x7a {
                                *c &= !0x20;
                            }
                        }

                        serial.write(&buf[0..count]).ok();
                    }
                    _ => {}
                }
            },
            _ => {}
        };
    };
}

#[exception]
fn HardFault(ef: &ExceptionFrame) -> ! {
    //dbgprint!("hard_fault");
    panic!("{:#?}", ef);
}

#[interrupt]
fn USB() {
    poll_usb();
}

#[exception]
fn DefaultHandler(irqn: i16) {
    //dbgprint!("default_handler");
    panic!("Unhandled exception (IRQn = {})", irqn);
}
