/// This code will periodically read 2 bytes from an I2C device and store them.
/// When something is written to the serial interface (cannot just send a blank message, send a newline or smth),
/// It will respond with whatever was read form the device
/// in the case of the example, read the raw temperature value from the TMP117 and print it over serial 
#![no_std]
#![no_main]

use core::sync::atomic::{AtomicU16, AtomicU8};

#[cfg(not(feature = "use_semihosting"))]
use panic_halt as _;
#[cfg(feature = "use_semihosting")]
use panic_semihosting as _;

use cortex_m::asm::delay as cycle_delay;
use cortex_m::peripheral::NVIC;
use usb_device::bus::UsbBusAllocator;
use usb_device::prelude::*;
use usbd_serial::embedded_io::Write;
use usbd_serial::{SerialPort, USB_CLASS_CDC};

use bsp::hal;
use bsp::pac;
use bsp::{entry, pin_alias};
use hal::clock::GenericClockController;
use hal::dmac::{DmaController, PriorityLevel};
use hal::ehal::i2c::I2c;
use hal::fugit::RateExtU32;
use hal::prelude::*;
use hal::usb::UsbBus;
use pac::{interrupt, CorePeripherals, Peripherals};
use trrs_trinkey as bsp;
use trrs_trinkey::i2c_master;

static TEMP: AtomicU16 = AtomicU16::new(0);

#[entry]
fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap();
    let mut core = CorePeripherals::take().unwrap();
    let mut clocks = GenericClockController::with_internal_32kosc(
        peripherals.gclk,
        &mut peripherals.pm,
        &mut peripherals.sysctrl,
        &mut peripherals.nvmctrl,
    );

    let pins = bsp::Pins::new(peripherals.port);

    // USB setup so that data can be logged to the host machine over serial
    // This code was taken from the usb_echo example.
    // This example was copied from an example for another board.s
    {
        let bus_allocator = unsafe {
            USB_ALLOCATOR = Some(bsp::usb_allocator(
                peripherals.usb,
                &mut clocks,
                &mut peripherals.pm,
                pins.usb_dm,
                pins.usb_dp,
            ));
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
            core.NVIC.set_priority(interrupt::USB, 1);
            NVIC::unmask(interrupt::USB);
        }
    }

    // I2C setup items
    let mut pm = peripherals.pm;
    let dmac = peripherals.dmac;

    // Take SDA and SCL
    let (sda, scl) = (pins.sda, pins.scl);

    // Setup DMA channels for later use (Optional)
    let mut dmac = DmaController::init(dmac, &mut pm);
    let channels = dmac.split();
    let chan0 = channels.0.init(PriorityLevel::Lvl0);

    // See the source for this function if more detailed configuration is desired.
    let mut i2c = i2c_master(
        &mut clocks,
        100.kHz(),
        peripherals.sercom2,
        &mut pm,
        sda,
        scl,
    )
    .with_dma_channel(chan0); //Optional

    let mut received = [0x00; 2];

    loop {
        cycle_delay(10 * 1024 * 1024);

        // Address of the temperature sensor I have (TMP117)
        const I2C_DEV_ADDR: u8 = 0x48;
        i2c.read(I2C_DEV_ADDR, &mut received).unwrap();

        let tmp = u16::from_be_bytes(received);

        TEMP.store(tmp, core::sync::atomic::Ordering::Relaxed);
    }
}

static mut USB_ALLOCATOR: Option<UsbBusAllocator<UsbBus>> = None;
static mut USB_BUS: Option<UsbDevice<UsbBus>> = None;
static mut USB_SERIAL: Option<SerialPort<UsbBus>> = None;

fn poll_usb() {
    unsafe {
        if let Some(usb_dev) = USB_BUS.as_mut() {
            let current_temp = TEMP.load(core::sync::atomic::Ordering::Relaxed);

            if let Some(serial) = USB_SERIAL.as_mut() {
                usb_dev.poll(&mut [serial]);
                let mut buf = [0u8; 64];

                if let Ok(count) = serial.read(&mut buf) {
                    writeln!(serial, "Raw {current_temp}").ok();
                };
            };
        };
    };
}

#[interrupt]
fn USB() {
    poll_usb();
}
