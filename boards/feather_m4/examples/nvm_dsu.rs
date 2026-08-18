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
use hal::clock::v2::{clock_system_at_reset, pclk::Pclk};
use hal::dsu::Dsu;
use hal::nvm::{retrieve_bank_size, Bank, Nvm, WriteGranularity, BLOCKSIZE};
use hal::pac::{interrupt, CorePeripherals, Peripherals};
use hal::usb::UsbBus;

use usb_device::bus::UsbBusAllocator;
use usb_device::prelude::*;
use usbd_serial::{SerialPort, USB_CLASS_CDC};

use core::sync::atomic;
use cortex_m::asm::delay as cycle_delay;
use cortex_m::peripheral::NVIC;

#[entry]
fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap();
    let mut core = CorePeripherals::take().unwrap();

    // Clocks configured as they are on reset - Gclk0 at 48Mhz
    let (mut buses, clocks, tokens) = clock_system_at_reset(
        peripherals.oscctrl,
        peripherals.osc32kctrl,
        peripherals.gclk,
        peripherals.mclk,
        &mut peripherals.nvmctrl,
    );
    // This is required because the `usb` module have not yet
    // been update to use `clock::v2`
    let (_, _, _, mut mclk) = unsafe { clocks.pac.steal() };

    let pins = bsp::Pins::new(peripherals.port);
    let mut red_led = pins.d13.into_push_pull_output();
    let mut nvm = Nvm::new(peripherals.nvmctrl);

    let ahb_dsu = clocks.ahbs.dsu;
    let apb_dsu = clocks.apbs.dsu;
    let mut dsu = Dsu::new(peripherals.dsu, ahb_dsu, apb_dsu, &peripherals.pac).unwrap();
    // USB Can be ran off 48Mhz clock, so we can derive the Pclk directly from Gclk0

    let (pclk_usb, _gclk0) = Pclk::enable(tokens.pclks.usb, clocks.gclk0);
    let bus_allocator = unsafe {
        // Not using the BSP USB constructor as that has not yet
        // been ported to use clock::v2
        USB_ALLOCATOR = Some(UsbBusAllocator::new(UsbBus::new(
            &pclk_usb.into(),
            &mut mclk,
            pins.usb_dm,
            pins.usb_dp,
            peripherals.usb,
        )));
        USB_ALLOCATOR.as_ref().unwrap()
    };

    unsafe {
        USB_SERIAL = Some(SerialPort::new(bus_allocator));
        USB_BUS = Some(
            UsbDeviceBuilder::new(bus_allocator, UsbVidPid(0x16c0, 0x27dd))
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

    serial_writeln!("Booted - Active bank: {:?}!\r\n", nvm.first_bank());

    unsafe {
        let active_bank_address = Bank::Active.address();
        let inactive_bank_address = Bank::Inactive.address();
        let bank_size_in_bytes = retrieve_bank_size();
        let bank_size_in_words = retrieve_bank_size() / 4;
        let bank_size_in_blocks = retrieve_bank_size() / BLOCKSIZE;
        let crc32_checksum_active_bank =
            dsu.crc32(active_bank_address, bank_size_in_bytes).unwrap();
        let crc32_checksum_inactive_bank = dsu
            .crc32(inactive_bank_address, bank_size_in_bytes)
            .unwrap();
        serial_writeln!(
            "Active bank CRC32 checksum: {:0X}",
            crc32_checksum_active_bank
        );
        serial_writeln!(
            "Inactive bank CRC32 checksum: {:0X}",
            crc32_checksum_inactive_bank
        );
        if crc32_checksum_active_bank != crc32_checksum_inactive_bank {
            serial_writeln!("Checksums differ: overwrite inactive bank with active one");
            serial_writeln!("Erase inactive bank");
            nvm.erase_flash(inactive_bank_address as *mut _, bank_size_in_blocks)
                .unwrap();
            serial_writeln!("Overwrite inactive bank with active bank");
            nvm.write_flash(
                inactive_bank_address as *mut _,
                active_bank_address as *const _,
                bank_size_in_words,
                WriteGranularity::Page,
            )
            .unwrap();
            serial_writeln!("Swapping banks & reset!");
            nvm.bank_swap();
        }
    }

    serial_writeln!("Checksums are the same: endless loop");

    loop {
        cycle_delay(5 * 1024 * 1024);
        red_led.toggle().ok();
    }
}

static mut USB_ALLOCATOR: Option<UsbBusAllocator<UsbBus>> = None;
static mut USB_BUS: Option<UsbDevice<UsbBus>> = None;
static mut USB_SERIAL: Option<SerialPort<UsbBus>> = None;

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
    T: Fn(&mut SerialPort<UsbBus>) -> R,
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
        };
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
