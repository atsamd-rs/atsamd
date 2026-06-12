#![no_std]
#![no_main]
/// This example emulates a really bad Iambic Morse Key,
/// i.e. it is not usable because the timings are not useful presumably because of the polling rate and the ability to "interleave" the dots and dashes.
/// That being said, when the TIP and RING_1 pins of the TRRS jack are pulled low,
/// They give the keystrokes of '.' and '-' respectively.
/// This example was adapted from the code at https://github.com/atsamd-rs/atsamd/blob/ef02fe3e21031da27aaef629368105962363b370/boards/itsybitsy_m0/examples/twitching_usb_mouse.rs
use atsamd_hal::gpio::Input;
use atsamd_hal::gpio::Pin;
use atsamd_hal::gpio::PullUp;
use atsamd_hal::gpio::{PA02, PA06};
use cortex_m::asm::wfi;
use cortex_m::interrupt::Mutex;
#[cfg(not(feature = "use_semihosting"))]
use panic_halt as _;
#[cfg(feature = "use_semihosting")]
use panic_semihosting as _;

use cortex_m::peripheral::NVIC;
use usb_device::bus::UsbBusAllocator;
use usb_device::prelude::*;

use bsp::hal;
use bsp::pac;
use trrs_trinkey as bsp;

use bsp::entry;
use core::cell::OnceCell;
use cortex_m::interrupt::free as disable_interrupts;
use hal::clock::GenericClockController;
use hal::prelude::*;
use hal::usb::UsbBus;
use pac::{interrupt, CorePeripherals, Peripherals};
use usbd_hid::descriptor::generator_prelude::*;
use usbd_hid::descriptor::KeyboardReport;
use usbd_hid::hid_class::HIDClass;

static TR1PINS: Mutex<OnceCell<(Pin<PA02, Input<PullUp>>, Pin<PA06, Input<PullUp>>)>> =
    Mutex::new(OnceCell::new());

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

    // Setup so an LED could be driven by being connected between the Tip and Sleeve pins.
    // WARNING: from what I have found, the max current capability is 10mA, so it is suggested to keep the current below 7mA
    // I do not have any guidance on how much current the IO pins can SINK.
    // Source: https://forums.adafruit.com/viewtopic.php?t=123728, https://onlinedocs.microchip.com/oxy/GUID-22527069-B4D6-49B9-BACC-3AF1C52EB48C-en-US-20/GUID-AD9164C2-015D-4DEA-9A54-44165FBE92D0.html
    let tip_pin = pins.tip.into_pull_up_input();
    let ring1_pin = pins.ring_1.into_pull_up_input();

    // Disabling these pins to be certain.
    let _tip_sw = pins.tip_switch.into_floating_disabled();
    let _ring1_sw = pins.ring_1_switch.into_floating_disabled();
    let _ring2 = pins.ring_2.into_floating_disabled();

    disable_interrupts(|cs| {
        let tr1_pins = TR1PINS.borrow(cs);
        let _ = tr1_pins.get_or_init(|| (tip_pin, ring1_pin));
    });

    // Configure the sleve pin to sink current (be ground)
    let mut sleeve_pin = pins.sleeve.into_push_pull_output();
    sleeve_pin.set_drive_strength(true);
    sleeve_pin.set_low();

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
        USB_HID = Some(HIDClass::new(bus_allocator, KeyboardReport::desc(), 1));
        USB_BUS = Some(
            UsbDeviceBuilder::new(bus_allocator, UsbVidPid(0x16c0, 0x27dd))
                .strings(&[StringDescriptors::new(LangID::EN)
                    .manufacturer("Fake company")
                    .product("Twitchy Mousey")
                    .serial_number("TEST")])
                .expect("Failed to set strings")
                .build(),
        );
    }

    unsafe {
        core.NVIC.set_priority(interrupt::USB, 1);
        NVIC::unmask(interrupt::USB);
    }

    unsafe {
        core.NVIC.set_priority(interrupt::USB, 1);
        NVIC::unmask(interrupt::USB);
    }

    loop {
        wfi();
    }
}

const DAH_KEYCODE: u8 = 0x2D;
const DIT_KEYCODE: u8 = 0x37;

static mut USB_ALLOCATOR: Option<UsbBusAllocator<UsbBus>> = None;
static mut USB_BUS: Option<UsbDevice<UsbBus>> = None;
static mut USB_HID: Option<HIDClass<UsbBus>> = None;

fn poll_usb() {
    let mut tip_low = false;
    let mut ring_low = false;
    disable_interrupts(|cs| {
        let pins = TR1PINS.borrow(cs).get().unwrap();
        tip_low = pins.0.is_low().unwrap();
        ring_low = pins.1.is_low().unwrap();
    });

    let mut report = KeyboardReport {
        modifier: 0,
        reserved: 0,
        leds: 0,
        keycodes: [0x0, 0, 0, 0, 0, 0],
    };

    if tip_low {
        report.keycodes[0] = DIT_KEYCODE;
    }

    if ring_low {
        report.keycodes[1] = DAH_KEYCODE;
    }

    disable_interrupts(|_| unsafe { USB_HID.as_mut().map(|hid| hid.push_input(&report)) });

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
