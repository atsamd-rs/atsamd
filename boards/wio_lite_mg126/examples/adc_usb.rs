#![no_std]
#![no_main]

#[cfg(not(feature = "use_semihosting"))]
use panic_halt as _;
#[cfg(feature = "use_semihosting")]
use panic_semihosting as _;

use bsp::hal;
use wio_lite_mg126 as bsp;

use bsp::entry;
use hal::adc::Adc;
use hal::clock::GenericClockController;
use hal::pac::{interrupt, CorePeripherals, Peripherals};
use hal::prelude::*;
use hal::usb::UsbBus;
use usb_device::bus::UsbBusAllocator;
use usb_device::prelude::*;
use usbd_serial::{SerialPort, USB_CLASS_CDC};

use cortex_m::peripheral::NVIC;
use numtoa::NumToA;

static mut USB_ALLOCATOR: Option<UsbBusAllocator<UsbBus>> = None;
static mut USB_BUS: Option<UsbDevice<UsbBus>> = None;
static mut USB_SERIAL: Option<SerialPort<UsbBus>> = None;

#[entry]
fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap();
    let mut core = CorePeripherals::take().unwrap();
    let mut clocks = GenericClockController::with_external_32kosc(
        peripherals.GCLK,
        &mut peripherals.PM,
        &mut peripherals.SYSCTRL,
        &mut peripherals.NVMCTRL,
    );
    let mut pins = bsp::Pins::new(peripherals.PORT);
    let mut adc = Adc::adc(peripherals.ADC, &mut peripherals.PM, &mut clocks);
    let mut a0 = pins.a0.into_function_b(&mut pins.port);

    let bus_allocator = unsafe {
        USB_ALLOCATOR = Some(bsp::usb_allocator(
            peripherals.USB,
            &mut clocks,
            &mut peripherals.PM,
            pins.usb_dm, // PA24, also usb_dm
            pins.usb_dp, // PA24 also usb_dp
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

    loop {
        // Turn off interrupts so we don't fight with the interrupt
        cortex_m::interrupt::free(|_| unsafe {
            USB_BUS.as_mut().map(|_| {
                USB_SERIAL.as_mut().map(|serial| {
                    // Skip errors so we can continue the program
                    let data: u16 = adc.read(&mut a0).unwrap();
                    let mut buffer = [0u8; 20];
                    let _ = serial.write(data.numtoa(10, &mut buffer));
                    let _ = serial.write("\n".as_bytes());
                });
            })
        });
    }
}

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
