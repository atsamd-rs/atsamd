#![no_std]
#![no_main]
#![allow(static_mut_refs)]

/// Makes the wio_terminal appear as a USB serial port, and display
/// the time on the screen.
///
/// You can tell the terminal the time by running (on linux):
///  - sudo stty -F /dev/ttyACM0 115200 raw -echo
///  - sudo bash -c "echo 'time=hour:minute:second' > /dev/ttyACM0"
use embedded_graphics as eg;
use panic_halt as _;
use wio_terminal as wio;

use eg::mono_font::{ascii::FONT_10X20, MonoTextStyle};
use eg::pixelcolor::Rgb565;
use eg::prelude::*;
use eg::primitives::{PrimitiveStyleBuilder, Rectangle};
use eg::text::{Baseline, Text};

use cortex_m::interrupt::free as disable_interrupts;
use cortex_m::peripheral::NVIC;

use wio::entry;
use wio::hal::clock::GenericClockController;
use wio::hal::delay::Delay;
use wio::pac::{interrupt, CorePeripherals, Peripherals};
use wio::prelude::*;

use core::fmt::Write;
use heapless::String;
use wio::hal::rtc;

use usb_device::bus::UsbBusAllocator;
use usb_device::prelude::*;
use usbd_serial::{SerialPort, USB_CLASS_CDC};
use wio::hal::usb::UsbBus;

#[entry]
fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap();
    let mut core = CorePeripherals::take().unwrap();

    let mut clocks = GenericClockController::with_external_32kosc(
        peripherals.gclk,
        &mut peripherals.mclk,
        &mut peripherals.osc32kctrl,
        &mut peripherals.oscctrl,
        &mut peripherals.nvmctrl,
    );

    let mut delay = Delay::new(core.SYST, &mut clocks);
    let sets = wio::Pins::new(peripherals.port).split();

    // Configure the RTC. a 1024 Hz clock is configured for us when enabling our
    // main clock
    let rtc = rtc::Rtc::clock_mode(peripherals.rtc, 1024.Hz(), &mut peripherals.mclk);

    unsafe {
        RTC = Some(rtc);
    }

    // Initialize the ILI9341-based LCD display. Create a black backdrop the size of
    // the screen.
    let (mut display, _backlight) = sets
        .display
        .init(
            &mut clocks,
            peripherals.sercom7,
            &mut peripherals.mclk,
            58.MHz(),
            &mut delay,
        )
        .unwrap();
    Rectangle::with_corners(Point::new(0, 0), Point::new(320, 240))
        .into_styled(
            PrimitiveStyleBuilder::new()
                .fill_color(Rgb565::BLACK)
                .build(),
        )
        .draw(&mut display)
        .ok()
        .unwrap();

    // Initialize USB.
    let bus_allocator = unsafe {
        USB_ALLOCATOR = Some(sets.usb.usb_allocator(
            peripherals.usb,
            &mut clocks,
            &mut peripherals.mclk,
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
        core.NVIC.set_priority(interrupt::USB_OTHER, 1);
        core.NVIC.set_priority(interrupt::USB_TRCPT0, 1);
        core.NVIC.set_priority(interrupt::USB_TRCPT1, 1);
        NVIC::unmask(interrupt::USB_OTHER);
        NVIC::unmask(interrupt::USB_TRCPT0);
        NVIC::unmask(interrupt::USB_TRCPT1);
    }

    let style = MonoTextStyle::new(&FONT_10X20, Rgb565::WHITE);

    loop {
        delay.delay_ms(1000_u16);
        let time =
            disable_interrupts(|_| unsafe { RTC.as_mut().map(|rtc| rtc.current_time()) }).unwrap();

        let mut data = String::<16>::new();
        write!(
            data,
            "{:02}:{:02}:{:02}",
            time.hours, time.minutes, time.seconds
        )
        .ok()
        .unwrap();

        Rectangle::with_corners(Point::new(55, 80), Point::new(250, 112))
            .into_styled(
                PrimitiveStyleBuilder::new()
                    .fill_color(Rgb565::BLACK)
                    .build(),
            )
            .draw(&mut display)
            .ok()
            .unwrap();

        Text::with_baseline(data.as_str(), Point::new(55, 80), style, Baseline::Top)
            .draw(&mut display)
            .ok()
            .unwrap();
    }
}

static mut USB_ALLOCATOR: Option<UsbBusAllocator<UsbBus>> = None;
static mut USB_BUS: Option<UsbDevice<UsbBus>> = None;
static mut USB_SERIAL: Option<SerialPort<UsbBus>> = None;
static mut RTC: Option<rtc::Rtc<rtc::ClockMode>> = None;

fn poll_usb() {
    unsafe {
        if let Some(usb_dev) = USB_BUS.as_mut() {
            if let Some(serial) = USB_SERIAL.as_mut() {
                usb_dev.poll(&mut [serial]);
                let mut buf = [0u8; 32];

                if let Ok(count) = serial.read(&mut buf) {
                    // terminal.enqueue((buf, count)).ok().unwrap();
                    let mut buffer: &[u8] = &buf[..count];

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
                                    }
                                });
                            }
                            _ => break,
                        };
                    }
                };
            }
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

#[derive(Clone)]
pub struct Time {
    hour: u32,
    minute: u32,
    second: u32,
}

fn atoi(digits: &[u8]) -> u32 {
    let mut num: u32 = 0;
    let len = digits.len();
    for (i, digit) in digits.iter().enumerate() {
        let digit = (*digit - b'0') as u32;
        let mut exp = 1;
        for _ in 0..(len - i - 1) {
            exp *= 10;
        }
        num += exp * digit;
    }
    num
}

use nom::{
    bytes::complete::tag,
    character::complete::{char, digit1},
    combinator::opt,
    sequence::{delimited, preceded, terminated},
    IResult, Parser,
};

fn timespec(input: &[u8]) -> IResult<&[u8], Time> {
    let (input, hour) = preceded((opt(char('\n')), tag("time=")), digit1).parse(input)?;
    let (input, minute) = delimited(char(':'), digit1, char(':')).parse(input)?;
    let (input, second) = terminated(digit1, char('\n')).parse(input)?;

    Ok((
        input,
        Time {
            hour: atoi(hour),
            minute: atoi(minute),
            second: atoi(second),
        },
    ))
}
