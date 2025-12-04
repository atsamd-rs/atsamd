#![no_std]
#![no_main]
#![allow(clippy::bool_comparison)]

use bsp::ehal;
use bsp::hal;
use feather_m4 as bsp;

#[cfg(not(feature = "use_semihosting"))]
use panic_halt as _;
#[cfg(feature = "use_semihosting")]
use panic_semihosting as _;

use bsp::entry;
use ehal::digital::StatefulOutputPin;
use hal::clock::v2::{
    clock_system_at_reset, dfll,
    gclk::{Gclk, Gclk1Id},
    pclk::Pclk,
};
use hal::nvm::{smart_eeprom, Nvm};
use hal::pac::{interrupt, CorePeripherals, Peripherals};
use hal::usb::UsbBus;

use usb_device::bus::UsbBusAllocator;
use usb_device::prelude::*;
use usbd_serial::{SerialPort, USB_CLASS_CDC};

use cortex_m::asm::delay as cycle_delay;
use cortex_m::peripheral::NVIC;

use core::sync::atomic;

#[entry]
fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap();
    let mut core = CorePeripherals::take().unwrap();
    let (mut buses, clocks, tokens) = clock_system_at_reset(
        peripherals.oscctrl,
        peripherals.osc32kctrl,
        peripherals.gclk,
        peripherals.mclk,
        &mut peripherals.nvmctrl,
    );
    let pins = bsp::Pins::new(peripherals.port);
    let mut red_led = pins.d13.into_push_pull_output();
    let mut nvm = Nvm::new(peripherals.nvmctrl);

    // Set up USB clocking
    let (dfll_usb, _) = clocks.dfll.into_mode(dfll::FromUsb, |_| {});
    // GCLK1 comes from DFLL, outputs to USB
    let (gclk_1, _) = Gclk::from_source(tokens.gclks.gclk1, dfll_usb);
    let gclk_1_48mhz = gclk_1.enable();
    let (pclk_usb, _) = Pclk::enable(tokens.pclks.usb, gclk_1_48mhz);

    let usb_bus = UsbBus::new(
        pclk_usb,
        clocks.ahbs.usb,
        buses.apb.enable(tokens.apbs.usb),
        pins.usb_dm,
        pins.usb_dp,
        peripherals.usb,
    )
    .unwrap();

    let bus_allocator = unsafe {
        USB_ALLOCATOR = Some(UsbBusAllocator::new(usb_bus));
        USB_ALLOCATOR.as_ref().unwrap()
    };

    unsafe {
        USB_SERIAL = Some(SerialPort::new(bus_allocator));
        USB_BUS = Some(
            UsbDeviceBuilder::new(bus_allocator, UsbVidPid(0x1209, 0x0001))
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

    while USER_PRESENT.load(atomic::Ordering::Acquire) == false {
        cycle_delay(25 * 1024 * 1024);
        red_led.toggle().ok();
    }

    match nvm.smart_eeprom() {
        Ok(se) => {
            use smart_eeprom::SmartEepromMode::*;
            let mut seeprom = match se {
                Unlocked(se) => se,
                Locked(se) => se.unlock(),
            };
            let write_buf = [0x01u8, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef];
            serial_writeln!("Write dummy data");
            seeprom.set(0x10, &write_buf);
            seeprom.set(0x18, &write_buf);
            serial_writeln!("Read dummy data");
            let mut read_buf = [0u8; 14];
            seeprom.get(0x14, &mut read_buf);
            read_buf.rotate_left(4);
            if read_buf[..8] == write_buf {
                serial_writeln!("Smart EEPROM test successful");
            } else {
                serial_writeln!("Smart EEPROM test failed");
            }
        }
        Err(e) => {
            serial_writeln!("Failed to initialize Smart EEPROM: {:?}!", e);
        }
    }

    loop {
        cycle_delay(5 * 1024 * 1024);
        red_led.toggle().ok();
    }
}

static mut USB_ALLOCATOR: Option<UsbBusAllocator<UsbBus<Gclk1Id>>> = None;
static mut USB_BUS: Option<UsbDevice<UsbBus<Gclk1Id>>> = None;
static mut USB_SERIAL: Option<SerialPort<UsbBus<Gclk1Id>>> = None;

static USER_PRESENT: atomic::AtomicBool = atomic::AtomicBool::new(false);

/// Borrows the global singleton `UsbSerial` for a brief period with interrupts
/// disabled
///
/// # Arguments
/// `borrower`: The closure that gets run borrowing the global `UsbSerial`
///
/// # Safety
/// the global singleton `UsbSerial` can be safely borrowed because we disable
/// interrupts while it is being borrowed, guaranteeing that interrupt handlers
/// like `USB` cannot mutate `UsbSerial` while we are as well.
///
/// # Panic
/// If `init` has not been called and we haven't initialized our global
/// singleton `UsbSerial`, we will panic.
fn usbserial_get<T, R>(borrower: T) -> R
where
    T: Fn(&mut SerialPort<UsbBus<Gclk1Id>>) -> R,
{
    usb_free(|_| unsafe {
        let usb_serial = USB_SERIAL.as_mut().expect("UsbSerial not initialized");
        borrower(usb_serial)
    })
}

/// Execute closure `f` in an interrupt-free context.
///
/// This as also known as a "critical section".
#[inline]
fn usb_free<F, R>(f: F) -> R
where
    F: FnOnce(&cortex_m::interrupt::CriticalSection) -> R,
{
    NVIC::mask(interrupt::USB_OTHER);
    NVIC::mask(interrupt::USB_TRCPT0);
    NVIC::mask(interrupt::USB_TRCPT1);

    let r = f(unsafe { &cortex_m::interrupt::CriticalSection::new() });

    unsafe {
        NVIC::unmask(interrupt::USB_OTHER);
        NVIC::unmask(interrupt::USB_TRCPT0);
        NVIC::unmask(interrupt::USB_TRCPT1);
    };

    r
}

/// Writes the given message out over USB serial.
///
/// # Arguments
/// * println args: variable arguments passed along to `core::write!`
///
/// # Warning
/// as this function deals with a static mut, and it is also accessed in the
/// USB interrupt handler, we both have unsafe code for unwrapping a static mut
/// as well as disabling of interrupts while we do so.
///
/// # Safety
/// the only time the static mut is used, we have interrupts disabled so we know
/// we have sole access
#[macro_export]
macro_rules! serial_writeln {
    ($($tt:tt)+) => {{
        use core::fmt::Write;

        let mut s: heapless::String<256> = heapless::String::new();
        core::write!(&mut s, $($tt)*).unwrap();
        usbserial_get(|usbserial| {
            usbserial.write(s.as_bytes()).ok();
            usbserial.write("\r\n".as_bytes()).ok();
        });
    }};
}

fn poll_usb() {
    unsafe {
        if let Some(usb_dev) = USB_BUS.as_mut() {
            if let Some(serial) = USB_SERIAL.as_mut() {
                usb_dev.poll(&mut [serial]);
                let mut buf = [0u8; 64];

                if let Ok(count) = serial.read(&mut buf) {
                    if count > 0 {
                        USER_PRESENT.store(true, atomic::Ordering::Release);
                    }
                    serial.write(&buf[..count]).unwrap();
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
