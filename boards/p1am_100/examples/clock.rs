#![no_std]
#![no_main]

use bsp::hal;
use p1am_100 as bsp;

#[cfg(not(feature = "use_semihosting"))]
use panic_halt as _;
#[cfg(feature = "use_semihosting")]
use panic_semihosting as _;

use cortex_m::interrupt::free as disable_interrupts;
use cortex_m::peripheral::NVIC;

use bsp::{entry, Pins};
use hal::clock::{ClockGenId, ClockSource, GenericClockController};
use hal::delay::Delay;
use hal::pac::{interrupt, CorePeripherals, Peripherals};
use hal::prelude::*;

use core::fmt::Write;
use hal::rtc;
use heapless::String;

use hal::usb::UsbBus;
use usb_device::bus::UsbBusAllocator;
use usb_device::prelude::*;
use usbd_serial::{SerialPort, USB_CLASS_CDC};

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
    let pins = Pins::new(peripherals.PORT);
    let mut led: bsp::Led = pins.led.into();

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
            pins.usb_dm.into(),
            pins.usb_dp.into(),
        ));
        USB_ALLOCATOR.as_ref().unwrap()
    };
    unsafe {
        USB_SERIAL = Some(SerialPort::new(&bus_allocator));
        USB_BUS = Some(
            UsbDeviceBuilder::new(&bus_allocator, UsbVidPid(0x16c0, 0x27dd))
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
        led.toggle().unwrap();
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
        USB_SERIAL.as_mut().map(|serial| {
            serial.write(bytes).unwrap();
        });
    }
}

fn poll_usb() {
    unsafe {
        USB_BUS.as_mut().map(|usb_dev| {
            USB_SERIAL.as_mut().map(|serial| {
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
                                    RTC.as_mut().map(|rtc| {
                                        rtc.set_time(rtc::Datetime {
                                            seconds: time.second as u8,
                                            minutes: time.minute as u8,
                                            hours: time.hour as u8,
                                            day: 0,
                                            month: 0,
                                            year: 0,
                                        });
                                    });
                                });
                            }
                            _ => break,
                        };
                    }
                };
            });
        });
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
