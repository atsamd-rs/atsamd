#![no_std]
#![no_main]

use metro_m4 as bsp;

use bsp::ehal;
use bsp::entry;
use bsp::hal;
use bsp::pac;
use bsp::pin_alias;

use cortex_m::asm::delay as cycle_delay;
use cortex_m::peripheral::NVIC;
use ehal::digital::StatefulOutputPin;
use usb_device::bus::UsbBusAllocator;
use usb_device::prelude::*;
use usbd_serial::{SerialPort, USB_CLASS_CDC};

use hal::clock::v2::{
    clock_system_at_reset, dfll,
    gclk::{Gclk, Gclk1Id},
    pclk::Pclk,
};
use hal::usb::UsbBus;
use pac::{interrupt, CorePeripherals, Peripherals};

#[cfg(not(feature = "use_semihosting"))]
use panic_halt as _;
#[cfg(feature = "use_semihosting")]
use panic_semihosting as _;

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
    let mut red_led: bsp::RedLed = pin_alias!(pins.red_led).into();

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
        core.NVIC.set_priority(interrupt::USB_TRCPT0, 1);
        NVIC::unmask(interrupt::USB_TRCPT0);
        core.NVIC.set_priority(interrupt::USB_TRCPT1, 1);
        NVIC::unmask(interrupt::USB_TRCPT1);
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

static mut USB_ALLOCATOR: Option<UsbBusAllocator<UsbBus<Gclk1Id>>> = None;
static mut USB_BUS: Option<UsbDevice<UsbBus<Gclk1Id>>> = None;
static mut USB_SERIAL: Option<SerialPort<UsbBus<Gclk1Id>>> = None;

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
fn USB_OTHER() {
    poll_usb();
}
