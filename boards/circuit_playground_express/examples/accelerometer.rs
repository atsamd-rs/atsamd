//! This example creates a usb serial device and then probes the accelerometer
//! via i2c for it's device ID.

#![no_std]
#![no_main]

#[cfg(not(feature = "use_semihosting"))]
use panic_halt as _;
#[cfg(feature = "use_semihosting")]
use panic_semihosting as _;

use cortex_m::asm::delay as cycle_delay;
use cortex_m::peripheral::NVIC;
use usb_device::bus::UsbBusAllocator;
use usb_device::prelude::*;
use usbd_serial::{SerialPort, USB_CLASS_CDC};

use bsp::hal;
use bsp::pac;
use circuit_playground_express as bsp;

use bsp::entry;
use bsp::periph_alias;
use bsp::pin_alias;
use hal::clock::GenericClockController;
use hal::delay::Delay;
use hal::dmac::DmaController;
use hal::dmac::PriorityLevel;
use hal::ehal::delay::DelayNs;
use hal::ehal::i2c::I2c;
use hal::gpio::Pin;
use hal::prelude::*;
use hal::sercom::i2c;
use hal::usb::UsbBus;
use pac::interrupt;
use pac::CorePeripherals;
use pac::Peripherals;

const ADDRESS: u8 = 0x19;
const WHOAMI: u8 = 0x0f;

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

    // take red_led
    let mut red_led: bsp::RedLed = pin_alias!(pins.red_led).into();

    setup_usb(
        &mut core.NVIC,
        &mut peripherals.pm,
        peripherals.usb,
        &mut clocks,
        pin_alias!(pins.usb_dm),
        pin_alias!(pins.usb_dp),
    );

    // Take SDA and SCL
    let (sda, scl) = (pin_alias!(pins.accel_sda), pin_alias!(pins.accel_scl));

    // Setup DMA channels for later use
    let mut dmac = DmaController::init(peripherals.dmac, &mut peripherals.pm);
    let channels = dmac.split();
    let chan0 = channels.0.init(PriorityLevel::Lvl0);

    let gclk0 = clocks.gclk0();
    let sercom1_clock = &clocks.sercom1_core(&gclk0).unwrap();
    let pads = i2c::Pads::new(sda, scl);
    let mut i2c = i2c::Config::new(
        &peripherals.pm,
        periph_alias!(peripherals.accel_sercom),
        pads,
        sercom1_clock.freq(),
    )
    .baud(400.kHz())
    .enable()
    .with_dma_channel(chan0);

    let mut buf = [0x00; 1];

    // Flash the LED in a spin loop
    loop {
        cycle_delay(15 * 1024 * 1024);
        red_led.toggle().ok();

        // Test writing then reading from an I2C chip
        // This particular call will always get 0x33 from the device.
        i2c.write_read(ADDRESS, &[WHOAMI], &mut buf).unwrap();

        unsafe {
            if let Some(usb_dev) = USB_BUS.as_mut() {
                if let Some(serial) = USB_SERIAL.as_mut() {
                    let s = buf;
                    for c in s {
                        let msg = heapless::format!(20; "{:x} ", c).unwrap();
                        let msg = msg.as_bytes();
                        serial.write(&*msg).ok();
                        cycle_delay(1 * 1024 * 1024);
                    }
                    serial.write(b"\r\n".as_slice()).ok();
                }
            }
        }
    }
}

fn setup_usb(
    nvic: &mut cortex_m::peripheral::NVIC,
    pm: &mut pac::Pm,
    usb: pac::Usb,
    clocks: &mut GenericClockController,
    usb_dm: Pin<circuit_playground_express::pins::UsbDmId, hal::gpio::FloatingDisabled>,
    usb_dp: Pin<circuit_playground_express::pins::UsbDpId, hal::gpio::FloatingDisabled>,
) -> () {
    let bus_allocator = unsafe {
        USB_ALLOCATOR = Some(bsp::usb_allocator(usb, clocks, pm, usb_dm, usb_dp));
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
        nvic.set_priority(interrupt::USB, 1);
        NVIC::unmask(interrupt::USB);
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
            };
        };
    };
}

// The usb must be polled via interrupte for this to work.
#[interrupt]
fn USB() {
    poll_usb();
}
