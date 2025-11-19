#![no_std]
#![no_main]

//! Makes the grand_central_m4 appear as a USB serial port. The color of the
//! neopixel LED can be changed by sending bytes to the serial port.
//!
//! Sending the characters R, G, and O set the LED red, green, and off
//! respectively. For example:
//! $> sudo stty -F /dev/ttyACM0 115200 raw -echo
//! $> sudo bash -c "echo 'R' > /dev/ttyACM0"
//! $> sudo bash -c "echo 'G' > /dev/ttyACM0"
//! $> sudo bash -c "echo 'O' > /dev/ttyACM0"
//!
//! Note leds may appear white during debug. Either build for release or add
//! opt-level = 2 to profile.dev in Cargo.toml

use grand_central_m4 as bsp;

use bsp::hal;

#[cfg(not(feature = "use_semihosting"))]
use panic_halt as _;
#[cfg(feature = "use_semihosting")]
use panic_semihosting as _;

use bsp::entry;
use cortex_m::interrupt::free as disable_interrupts;
use cortex_m::peripheral::NVIC;
use hal::clock::v2::{
    clock_system_at_reset, dfll,
    gclk::{Gclk, Gclk1Id},
    pclk::Pclk,
};
use hal::pac::{interrupt, CorePeripherals, Peripherals};
use hal::prelude::*;
use hal::time::Hertz;
use hal::timer::TimerCounter;
use hal::usb::UsbBus;
use smart_leds::{colors, hsv::RGB8, SmartLedsWrite};
use usb_device::bus::UsbBusAllocator;
use usb_device::prelude::*;
use usbd_serial::{SerialPort, USB_CLASS_CDC};
use ws2812_timer_delay as ws2812;

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
    let mut mclk = unsafe { clocks.pac.steal().3 }; // THIS IS ONLY TEMPORARY

    let (tc2_3, _gclk0) = Pclk::enable(tokens.pclks.tc2_tc3, clocks.gclk0);
    let mut timer = TimerCounter::tc3_(&tc2_3.into(), peripherals.tc3, &mut mclk);
    InterruptDrivenTimer::start(&mut timer, Hertz::MHz(3).into_duration());

    let pins = bsp::Pins::new(peripherals.port);
    let neopixel_pin = pins.neopixel.into_push_pull_output();
    let mut neopixel = ws2812::Ws2812::new(timer, neopixel_pin);
    let _ = neopixel.write((0..5).map(|_| RGB8::default()));

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
                .strings(&[StringDescriptors::new(LangID::EN_US)
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

    loop {
        let pending = disable_interrupts(|_| unsafe {
            let pending = PENDING_COLOR;
            PENDING_COLOR = None;
            pending
        });
        if let Some(color) = pending {
            let _ = neopixel.write((0..5).map(|_| color));
        }
    }
}

static mut USB_ALLOCATOR: Option<UsbBusAllocator<UsbBus<Gclk1Id>>> = None;
static mut USB_BUS: Option<UsbDevice<UsbBus<Gclk1Id>>> = None;
static mut USB_SERIAL: Option<SerialPort<UsbBus<Gclk1Id>>> = None;
static mut PENDING_COLOR: Option<RGB8> = None;

fn poll_usb() {
    unsafe {
        USB_BUS.as_mut().map(|usb_dev| {
            USB_SERIAL.as_mut().map(|serial| {
                usb_dev.poll(&mut [serial]);

                let mut buf = [0u8; 64];

                if let Ok(count) = serial.read(&mut buf) {
                    let last = buf[count - 1] as char;
                    let color = match last {
                        'R' => colors::RED,
                        'G' => colors::GREEN,
                        'O' => colors::ORANGE,
                        _ => RGB8::default(),
                    };

                    PENDING_COLOR = Some(color);
                };
            });
        });
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
