#![no_std]
#![no_main]

use metro_m4 as bsp;

use bsp::ehal;
use bsp::hal;
use bsp::pac;

#[cfg(not(feature = "use_semihosting"))]
use panic_halt as _;
#[cfg(feature = "use_semihosting")]
use panic_semihosting as _;

use cortex_m::asm::delay as cycle_delay;
use cortex_m::peripheral::NVIC;
use ehal::digital::v2::ToggleableOutputPin;
use usb_device::bus::UsbBusAllocator;
use usb_device::prelude::*;
use usbd_serial::{SerialPort, USB_CLASS_CDC};

use bsp::entry;
use hal::clock::GenericClockController;
use hal::usb::UsbBus;
use pac::{interrupt, CorePeripherals, Peripherals};

#[entry]
fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap();
    let mut core = CorePeripherals::take().unwrap();
    let mut clocks = GenericClockController::with_external_32kosc(
        peripherals.GCLK,
        &mut peripherals.MCLK,
        &mut peripherals.OSC32KCTRL,
        &mut peripherals.OSCCTRL,
        &mut peripherals.NVMCTRL,
    );

    let pins = bsp::Pins::new(peripherals.PORT);
    let mut red_led = pins.d13.into_push_pull_output();

    let bus_allocator = unsafe {
        USB_ALLOCATOR = Some(bsp::usb_allocator(
            peripherals.USB,
            &mut clocks,
            &mut peripherals.MCLK,
            pins.usb_dm,
            pins.usb_dp,
        ));
        USB_ALLOCATOR.as_ref().unwrap()
    };

    unsafe {
        USB_SERIAL = Some(SerialPort::new(bus_allocator));
        USB_BUS = Some(
            UsbDeviceBuilder::new(bus_allocator, UsbVidPid(0x2222, 0x3333))
                .manufacturer("Fake company")
                .product("Serial port")
                .serial_number("TEST")
                .device_class(USB_CLASS_CDC)
                .build(),
        );
    }

    unsafe {
        core.NVIC.set_priority(interrupt::USB_TRCPT0, 1);
        NVIC::unmask(interrupt::USB_TRCPT0);
        core.NVIC.set_priority(interrupt::USB_TRCPT1, 1);
        NVIC::unmask(interrupt::USB_TRCPT1);
        core.NVIC.set_priority(interrupt::USB_SOF_HSOF, 1);
        NVIC::unmask(interrupt::USB_SOF_HSOF);
        core.NVIC.set_priority(interrupt::USB_OTHER, 1);
        NVIC::unmask(interrupt::USB_OTHER);
    }

    // Flash the LED in a spin loop to demonstrate that USB is
    // entirely interrupt driven.
    loop {
        cycle_delay(5 * 1024 * 1024);
        red_led.toggle().unwrap();

        // Turn off interrupts so we don't fight with the interrupt
        cortex_m::interrupt::free(|_| unsafe {
            if USB_BUS.as_mut().is_some() {
                if let Some(serial) = USB_SERIAL.as_mut() {
                    let _ = serial.write("Hello USB\n".as_bytes());
                }
            }
        });
    }
}

static mut USB_ALLOCATOR: Option<UsbBusAllocator<UsbBus>> = None;
static mut USB_BUS: Option<UsbDevice<UsbBus>> = None;
static mut USB_SERIAL: Option<SerialPort<UsbBus>> = None;

fn poll_usb() {
    unsafe {
        if let Some(usb_dev) = USB_BUS.as_mut() {
            if let Some(serial) = USB_SERIAL.as_mut() {
                usb_dev.poll(&mut [serial]);

                // Make the other side happy
                let mut buf = [0u8; 16];
                let _ = serial.read(&mut buf);
            };
        }
    };
}

#[interrupt]
fn USB_TRCPT0() {
    poll_usb();
}

#[interrupt]
fn USB_TRCPT1() {
    poll_usb();
}

#[interrupt]
fn USB_SOF_HSOF() {
    poll_usb();
}

#[interrupt]
fn USB_OTHER() {
    poll_usb();
}
