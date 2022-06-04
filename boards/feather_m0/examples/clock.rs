#![no_std]
#![no_main]

use core::fmt::Write;

#[cfg(not(feature = "use_semihosting"))]
use panic_halt as _;
#[cfg(feature = "use_semihosting")]
use panic_semihosting as _;

use cortex_m::interrupt::free as disable_interrupts;
use cortex_m::peripheral::NVIC;
use heapless::String;
use usb_device::bus::UsbBusAllocator;
use usb_device::prelude::*;
use usbd_serial::{SerialPort, USB_CLASS_CDC};

use bsp::hal;
use bsp::pac;
use feather_m0 as bsp;

use bsp::{entry, pin_alias};
use hal::clock::{ClockGenId, ClockSource, GenericClockController};
use hal::delay::Delay;
use hal::prelude::*;
use hal::rtc;
use hal::usb::UsbBus;
use pac::{interrupt, CorePeripherals, Peripherals};

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

    let mut delay = Delay::new(core.SYST, &mut clocks);
    let pins = bsp::Pins::new(peripherals.PORT);
    let mut red_led: bsp::RedLed = pin_alias!(pins.red_led).into();

    // get the internal 32k running at 1024 Hz for the RTC
    let timer_clock = clocks
        .configure_gclk_divider_and_source(ClockGenId::GCLK3, 32, ClockSource::OSC32K, true)
        .unwrap();
    clocks.configure_standby(ClockGenId::GCLK3, true);
    let rtc_clock = clocks.rtc(&timer_clock).unwrap();
    let rtc = rtc::Rtc::clock_mode(peripherals.RTC, rtc_clock.freq(), &mut peripherals.PM);

    unsafe {
        RTC = Some(rtc);
    }

    // initialize USB
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
        USB_SERIAL = Some(SerialPort::new(bus_allocator));
        USB_BUS = Some(
            UsbDeviceBuilder::new(bus_allocator, UsbVidPid(0x16c0, 0x27dd))
                .manufacturer("Fake company")
                .product("Serial port")
                .serial_number("TEST")
                .device_class(USB_CLASS_CDC)
                .build(),
        );
    }
    unsafe {
        core.NVIC.set_priority(interrupt::USB, 1);
        NVIC::unmask(interrupt::USB);
    }

    // Print the time forever!
    loop {
        red_led.toggle().ok();
        let time =
            disable_interrupts(|_| unsafe { RTC.as_mut().map(|rtc| rtc.current_time()) }).unwrap();

        let mut data = String::<16>::new();
        write!(
            data,
            "{:02}:{:02}:{:02}\r\n",
            time.hours, time.minutes, time.seconds
        )
        .ok()
        .unwrap();
        write_serial(data.as_bytes());

        delay.delay_ms(1_000_u32);
    }
}

static mut USB_ALLOCATOR: Option<UsbBusAllocator<UsbBus>> = None;
static mut USB_BUS: Option<UsbDevice<UsbBus>> = None;
static mut USB_SERIAL: Option<SerialPort<UsbBus>> = None;
static mut RTC: Option<rtc::Rtc<rtc::ClockMode>> = None;

fn write_serial(bytes: &[u8]) {
    unsafe {
        if let Some(serial) = USB_SERIAL.as_mut() {
            serial.write(bytes).unwrap();
        };
    }
}

fn poll_usb() {
    unsafe {
        if let Some(usb_dev) = USB_BUS.as_mut() {
            if let Some(serial) = USB_SERIAL.as_mut() {
                usb_dev.poll(&mut [serial]);
                let mut buf = [0u8; 32];

                if let Ok(count) = serial.read(&mut buf) {
                    let mut buffer: &[u8] = &buf[..count];
                    // echo to terminal
                    serial.write(buffer).unwrap();

                    // Look for setting of time
                    while buffer.len() > 5 {
                        match timespec(buffer) {
                            Ok((remaining, time)) => {
                                buffer = remaining;
                                disable_interrupts(|_| {
                                    if let Some(rtc) = RTC.as_mut() {
                                        rtc.set_time(rtc::Datetime {
                                            seconds: time.second as u8,
                                            minutes: time.minute as u8,
                                            hours: time.hour as u8,
                                            day: 0,
                                            month: 0,
                                            year: 0,
                                        });
                                    };
                                });
                            }
                            _ => break,
                        };
                    }
                };
            };
        };
    };
}

#[interrupt]
fn USB() {
    poll_usb();
}

#[derive(Clone)]
pub struct Time {
    hour: usize,
    minute: usize,
    second: usize,
}

use drogue_nom_utils::parse_usize;
use nom::{char, do_parse, named, opt, tag};

named!(
    pub timespec<Time>,
    do_parse!(
        opt!( char!('\r') ) >>
        tag!("time=") >>
        hour: parse_usize >>
        char!(':') >>
        minute: parse_usize >>
        char!(':') >>
        second: parse_usize >>
        tag!("\r") >>
        (
            Time{ hour, minute, second }
        )
    )
);
