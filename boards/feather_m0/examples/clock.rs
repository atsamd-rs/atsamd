#![no_std]
#![no_main]

extern crate feather_m0 as hal;
use panic_halt as _;

use cortex_m::interrupt::free as disable_interrupts;
use cortex_m::peripheral::NVIC;

use hal::clock::{ClockGenId, ClockSource, GenericClockController};
use hal::delay::Delay;
use hal::pac::{interrupt, CorePeripherals, Peripherals};
use hal::prelude::*;
use hal::{entry, Pins};

use core::fmt::Write;
use hal::rtc;
use heapless::consts::U16;
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
    let mut pins = Pins::new(peripherals.PORT);
    let mut red_led = pins.d13.into_open_drain_output(&mut pins.port);

    // get the internal 32k running at 1024 Hz for the RTC
    let timer_clock = clocks
        .configure_gclk_divider_and_source(ClockGenId::GCLK2, 32, ClockSource::OSC32K, true)
        .unwrap();
    clocks.configure_standby(ClockGenId::GCLK2, true);
    let rtc_clock = clocks.rtc(&timer_clock).unwrap();
    let mut rtc = rtc::Rtc::new(peripherals.RTC, rtc_clock.freq(), &mut peripherals.PM);
    rtc.clock_mode();

    unsafe {
        RTC = Some(rtc);
    }

    // initialize USB
    let bus_allocator = unsafe {
        USB_ALLOCATOR = Some(hal::usb_allocator(
            peripherals.USB,
            &mut clocks,
            &mut peripherals.PM,
            pins.usb_dm,
            pins.usb_dp,
            &mut pins.port,
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
        red_led.toggle();
        let time =
            disable_interrupts(|_| unsafe { RTC.as_mut().map(|rtc| rtc.current_time()) }).unwrap();

        let mut data = String::<U16>::new();
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
static mut RTC: Option<rtc::Rtc> = None;

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

#[macro_use]
extern crate nom;
use drogue_nom_utils::parse_usize;

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
