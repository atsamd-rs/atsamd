#![no_std]
#![no_main]

use arduino_nano33iot as bsp;
use bsp::hal;
use bsp::hal::prelude::*;

use usb_device;
use usbd_serial;

#[cfg(not(feature = "use_semihosting"))]
use panic_halt as _;
#[cfg(feature = "use_semihosting")]
use panic_semihosting as _;

use bsp::entry;
use hal::clock::GenericClockController;
use hal::pac::{interrupt, CorePeripherals, Peripherals};

use hal::usb::UsbBus;
use usb_device::bus::UsbBusAllocator;
use usb_device::prelude::*;
use usbd_serial::{SerialPort, USB_CLASS_CDC};

use cortex_m::asm::delay as cycle_delay;
use cortex_m::peripheral::NVIC;

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
    let mut led: bsp::Led = pins.led_sck.into();

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
            UsbDeviceBuilder::new(&bus_allocator, UsbVidPid(0x2222, 0x3333))
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

    // Flash the LED in a spin loop to demonstrate that USB is
    // entirely interrupt driven.
    loop {
        cycle_delay(5 * 1024 * 1024);
        led.toggle().unwrap();

        // Turn off interrupts so we don't fight with the interrupt
        cortex_m::interrupt::free(|_| unsafe {
            USB_BUS.as_mut().map(|_| {
                USB_SERIAL.as_mut().map(|serial| {
                    // Skip errors so we can continue the program
                    let _ = serial.write("log line\r\n".as_bytes());
                });
            })
        });
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

                // Make the other side happy
                let mut buf = [0u8; 16];
                let _ = serial.read(&mut buf);
            });
        });
    };
}

#[interrupt]
fn USB() {
    poll_usb();
}
