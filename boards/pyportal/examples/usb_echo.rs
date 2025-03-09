#![no_std]
#![no_main]

use bsp::hal;
use pyportal as bsp;

#[cfg(not(feature = "use_semihosting"))]
use panic_halt as _;
#[cfg(feature = "use_semihosting")]
use panic_semihosting as _;

use bsp::entry;
use bsp::pin_alias;
use hal::clock::GenericClockController;
use hal::pac::{interrupt, CorePeripherals, Peripherals};
use hal::prelude::*;
use hal::usb::UsbBus;

use usb_device::bus::UsbBusAllocator;
use usb_device::prelude::*;
use usbd_serial::{SerialPort, USB_CLASS_CDC};

use cortex_m::asm::delay as cycle_delay;
use cortex_m::interrupt::Mutex;
use cortex_m::peripheral::NVIC;

use core::cell::RefCell;
use core::mem::MaybeUninit;

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
    let pins = bsp::Pins::new(peripherals.port);
    let mut red_led: bsp::RedLed = pin_alias!(pins.red_led).into();

    let bus_allocator = unsafe {
        USB_ALLOCATOR.write(bsp::usb_allocator(
            pins.usb_dm,
            pins.usb_dp,
            peripherals.usb,
            &mut clocks,
            &mut peripherals.mclk,
        ))
    };

    cortex_m::interrupt::free(|cs| {
        USB_SERIAL
            .borrow(cs)
            .replace(Some(SerialPort::new(bus_allocator)));
        USB_BUS.borrow(cs).replace(Some(
            UsbDeviceBuilder::new(bus_allocator, UsbVidPid(0x16c0, 0x27dd))
                .device_class(USB_CLASS_CDC)
                .strings(&[StringDescriptors::new(LangID::EN)
                    .manufacturer("Fake company")
                    .product("Serial port")
                    .serial_number("TEST")])
                .unwrap()
                .build(),
        ));
    });

    unsafe {
        core.NVIC.set_priority(interrupt::USB_OTHER, 1);
        core.NVIC.set_priority(interrupt::USB_TRCPT0, 1);
        core.NVIC.set_priority(interrupt::USB_TRCPT1, 1);
        NVIC::unmask(interrupt::USB_OTHER);
        NVIC::unmask(interrupt::USB_TRCPT0);
        NVIC::unmask(interrupt::USB_TRCPT1);
    }

    loop {
        cycle_delay(5 * 1024 * 1024);
        red_led.toggle().unwrap();
    }
}

static mut USB_ALLOCATOR: MaybeUninit<UsbBusAllocator<UsbBus>> = MaybeUninit::uninit();
static USB_BUS: Mutex<RefCell<Option<UsbDevice<UsbBus>>>> = Mutex::new(RefCell::new(None));
static USB_SERIAL: Mutex<RefCell<Option<SerialPort<UsbBus>>>> = Mutex::new(RefCell::new(None));

fn poll_usb() {
    // Disable interrupts while accessing USB_SERIAL and USB_BUS to prevent possible
    // race conditions
    cortex_m::interrupt::free(|cs| {
        if let Some(usb_dev) = USB_BUS.borrow(cs).borrow_mut().as_mut() {
            if let Some(serial) = USB_SERIAL.borrow(cs).borrow_mut().as_mut() {
                usb_dev.poll(&mut [serial]);
                let mut buf = [0u8; 64];

                if let Ok(count) = serial.read(&mut buf) {
                    for (i, c) in buf.iter().enumerate() {
                        if i >= count {
                            break;
                        }
                        serial.write(&[*c]).unwrap();
                    }
                };
            };
        };
    });
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
