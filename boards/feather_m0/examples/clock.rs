#![no_std]
#![no_main]
#![allow(static_mut_refs)]

#[cfg(not(feature = "use_semihosting"))]
use panic_halt as _;
#[cfg(feature = "use_semihosting")]
use panic_semihosting as _;

use core::cell::OnceCell;
use core::sync::atomic;
use cortex_m::peripheral::NVIC;
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
        peripherals.gclk,
        &mut peripherals.pm,
        &mut peripherals.sysctrl,
        &mut peripherals.nvmctrl,
    );

    let mut delay = Delay::new(core.SYST, &mut clocks);
    let pins = bsp::Pins::new(peripherals.port);
    let mut red_led: bsp::RedLed = pin_alias!(pins.red_led).into();

    // get the internal 32k running at 1024 Hz for the RTC
    let timer_clock = clocks
        .configure_gclk_divider_and_source(ClockGenId::Gclk3, 32, ClockSource::Osc32k, true)
        .unwrap();
    clocks.configure_standby(ClockGenId::Gclk3, true);
    let rtc_clock = clocks.rtc(&timer_clock).unwrap();
    let rtc = rtc::Rtc::clock_mode(peripherals.rtc, rtc_clock.freq(), &mut peripherals.pm);

    unsafe {
        let _ = RTC.set(rtc);
    }

    let bus_allocator = unsafe {
        let _ = USB_ALLOCATOR.set(bsp::usb_allocator(
            peripherals.usb,
            &mut clocks,
            &mut peripherals.pm,
            pins.usb_dm,
            pins.usb_dp,
        ));
        USB_ALLOCATOR.get().unwrap()
    };

    unsafe {
        let _ = USB_SERIAL.set(SerialPort::new(bus_allocator));
        let _ = USB_BUS.set(
            UsbDeviceBuilder::new(bus_allocator, UsbVidPid(0x16c0, 0x27dd))
                .strings(&[StringDescriptors::new(LangID::EN)
                    .manufacturer("Fake company")
                    .product("Serial port")
                    .serial_number("TEST")])
                .expect("Failed to set strings")
                .device_class(USB_CLASS_CDC)
                .build(),
        );

        // Enable USB interrupt
        core.NVIC.set_priority(interrupt::USB, 1);
        NVIC::unmask(interrupt::USB);
    }

    // Wait to receive something
    while !USB_DATA_RECEIVED.load(atomic::Ordering::Relaxed) {
        delay.delay_ms(250_u32);
        red_led.toggle().unwrap();
    }
    red_led.set_high().unwrap();

    // Print the time forever!
    loop {
        let time = usb_free(|_| unsafe { RTC.get_mut().map(|rtc| rtc.current_time()) }).unwrap();

        serial_writeln!("{:02}:{:02}:{:02}", time.hours, time.minutes, time.seconds);

        delay.delay_ms(1_000_u32);
    }
}

static mut USB_ALLOCATOR: OnceCell<UsbBusAllocator<UsbBus>> = OnceCell::new();
static mut USB_BUS: OnceCell<UsbDevice<UsbBus>> = OnceCell::new();
static mut USB_SERIAL: OnceCell<SerialPort<UsbBus>> = OnceCell::new();
static USB_DATA_RECEIVED: atomic::AtomicBool = atomic::AtomicBool::new(false);
static mut RTC: OnceCell<rtc::Rtc<rtc::ClockMode>> = OnceCell::new();

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
        let usb_serial = USB_SERIAL.get_mut().expect("UsbSerial not initialized");
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
    NVIC::mask(interrupt::USB);

    let r = f(&unsafe { cortex_m::interrupt::CriticalSection::new() });

    unsafe {
        NVIC::unmask(interrupt::USB);
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

#[inline]
fn poll_usb() {
    unsafe {
        if let Some(usb_dev) = USB_BUS.get_mut() {
            if let Some(serial) = USB_SERIAL.get_mut() {
                usb_dev.poll(&mut [serial]);
                let mut buf = [0u8; 64];

                if let Ok(count) = serial.read(&mut buf) {
                    let mut buffer: &[u8] = &buf[..count];

                    if buffer.len() > 0 {
                        // echo to terminal
                        serial.write(buffer).unwrap();

                        USB_DATA_RECEIVED.store(true, atomic::Ordering::Relaxed);
                    }

                    // Look for setting of time
                    while buffer.len() > 5 {
                        match Time::parse(core::str::from_utf8(buffer).unwrap()) {
                            Ok((remaining, time)) => {
                                buffer = remaining.as_bytes();

                                if let Some(rtc) = RTC.get_mut() {
                                    rtc.set_time(time.into());
                                };
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

#[derive(Debug)]
pub struct Time {
    hour: u8,
    minute: u8,
    second: u8,
}
impl Into<rtc::Datetime> for Time {
    fn into(self) -> rtc::Datetime {
        rtc::Datetime {
            seconds: self.second,
            minutes: self.minute,
            hours: self.hour,
            day: 0,
            month: 0,
            year: 0,
        }
    }
}

use nom::{
    bytes::complete::tag,
    character::complete::{char, u8 as u8p},
    combinator::{map, opt},
    IResult, Parser,
};

impl Time {
    pub fn parse(input: &str) -> IResult<&str, Self> {
        map(
            (
                opt(char('\r')),
                tag("time="),
                u8p,
                char(':'),
                u8p,
                char(':'),
                u8p,
                char('\r'),
            ),
            |(_, _, hour, _, minute, _, second, _)| Self {
                hour,
                minute,
                second,
            }, /* |(_, _, h, _, m, _, s, _): (Option<char>, _, u32, _, u32, _, u32, _)| Self {
                   hour: h.into(),
                   minute: m.into(),
                   second: s.into(),
               }, */
        )
        .parse(input)
    }
}
