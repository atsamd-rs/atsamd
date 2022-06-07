#![no_std]
#![no_main]

use bsp::hal;
use itsybitsy_m0 as bsp;

#[cfg(not(feature = "use_semihosting"))]
use panic_halt as _;
#[cfg(feature = "use_semihosting")]
use panic_semihosting as _;

use bsp::entry;
use hal::clock::GenericClockController;
use hal::pac::{interrupt, CorePeripherals, Peripherals};
use hal::prelude::*;

use hal::usb::UsbBus;
use usb_device::bus::UsbBusAllocator;
use usb_device::prelude::*;
use usbd_hid::descriptor::generator_prelude::*;
use usbd_hid::descriptor::MouseReport;
use usbd_hid::hid_class::HIDClass;

use cortex_m::asm::delay as cycle_delay;
use cortex_m::interrupt::free as disable_interrupts;
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
    let mut red_led: bsp::RedLed = pins.d13.into();
    red_led.set_low().unwrap();

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
        USB_HID = Some(HIDClass::new(bus_allocator, MouseReport::desc(), 60));
        USB_BUS = Some(
            UsbDeviceBuilder::new(bus_allocator, UsbVidPid(0x16c0, 0x27dd))
                .manufacturer("Fake company")
                .product("Twitchy Mousey")
                .serial_number("TEST")
                .build(),
        );
    }

    unsafe {
        core.NVIC.set_priority(interrupt::USB, 1);
        NVIC::unmask(interrupt::USB);
    }

    loop {
        cycle_delay(25 * 1024 * 1024);
        push_mouse_movement(MouseReport {
            x: 0,
            y: 4,
            buttons: 0,
        })
        .ok()
        .unwrap_or(0);
        cycle_delay(25 * 1024 * 1024);
        push_mouse_movement(MouseReport {
            x: 0,
            y: -4,
            buttons: 0,
        })
        .ok()
        .unwrap_or(0);
    }
}

fn push_mouse_movement(report: MouseReport) -> Result<usize, usb_device::UsbError> {
    disable_interrupts(|_| unsafe { USB_HID.as_mut().map(|hid| hid.push_input(&report)) }).unwrap()
}

static mut USB_ALLOCATOR: Option<UsbBusAllocator<UsbBus>> = None;
static mut USB_BUS: Option<UsbDevice<UsbBus>> = None;
static mut USB_HID: Option<HIDClass<UsbBus>> = None;

fn poll_usb() {
    unsafe {
        if let (Some(usb_dev), Some(hid)) = (USB_BUS.as_mut(), USB_HID.as_mut()) {
            usb_dev.poll(&mut [hid]);
        }
    };
}

#[interrupt]
fn USB() {
    poll_usb();
}
