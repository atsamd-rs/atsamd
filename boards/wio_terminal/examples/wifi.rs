#![no_std]
#![no_main]

use embedded_graphics as eg;
use panic_halt as _;
use wio_terminal as wio;

use wio::hal::clock::GenericClockController;
use wio::hal::delay::Delay;
use wio::pac::{CorePeripherals, Peripherals};
use wio::prelude::*;
use wio::wifi_prelude::*;
use wio::{entry, rpc, wifi_singleton, Pins, Sets};

use core::fmt::Write;
use cortex_m::interrupt::free as disable_interrupts;
use eg::fonts::{Font6x12, Text};
use eg::pixelcolor::Rgb565;
use eg::prelude::*;
use eg::primitives::rectangle::Rectangle;
use eg::style::{PrimitiveStyleBuilder, TextStyle};
use heapless::{consts::U256, String};

#[entry]
fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap();
    let mut core = CorePeripherals::take().unwrap();

    let mut clocks = GenericClockController::with_external_32kosc(
        peripherals.GCLK,
        &mut peripherals.MCLK,
        &mut peripherals.OSC32KCTRL,
        &mut peripherals.OSCCTRL,
        &mut peripherals.NVMCTRL,
    );
    let mut delay = Delay::new(core.SYST, &mut clocks);
    let mut sets: Sets = Pins::new(peripherals.PORT).split();

    // Set up the display so we can log our progress.
    let (display, _backlight) = sets
        .display
        .init(
            &mut clocks,
            peripherals.SERCOM7,
            &mut peripherals.MCLK,
            &mut sets.port,
            24.mhz(),
            &mut delay,
        )
        .unwrap();
    let mut terminal = Terminal::new(display);
    let mut textbuffer = String::<U256>::new();

    let mut user_led = sets.user_led.into_open_drain_output(&mut sets.port);
    user_led.set_high().unwrap();

    // Initialize the wifi peripheral.
    let args = (
        sets.wifi,
        peripherals.SERCOM0,
        &mut clocks,
        &mut peripherals.MCLK,
        &mut sets.port,
        &mut delay,
    );
    let nvic = &mut core.NVIC;
    disable_interrupts(|cs| unsafe {
        wifi_init(cs, args.0, args.1, args.2, args.3, args.4, args.5).unwrap();
        WIFI.as_mut().map(|wifi| {
            wifi.enable(cs, nvic);
        });
    });

    let version = unsafe {
        WIFI.as_mut()
            .map(|wifi| wifi.blocking_rpc(rpc::GetVersion {}).unwrap())
            .unwrap()
    };
    writeln!(
        textbuffer,
        "Ameba is running firmware version {:?}",
        version
    )
    .unwrap();
    terminal.write_str(textbuffer.as_str());
    textbuffer.truncate(0);

    let mac = unsafe {
        WIFI.as_mut()
            .map(|wifi| wifi.blocking_rpc(rpc::GetMacAddress {}).unwrap())
            .unwrap()
    };
    writeln!(textbuffer, "mac = {:?}", mac).unwrap();
    terminal.write_str(textbuffer.as_str());
    textbuffer.truncate(0);

    let ss = unsafe {
        WIFI.as_mut()
            .map(|wifi| wifi.blocking_rpc(rpc::ScanStart {}).unwrap())
            .unwrap()
    };
    writeln!(textbuffer, "scan.start() = {:?}", ss).unwrap();
    terminal.write_str(textbuffer.as_str());
    textbuffer.truncate(0);

    let mut scanning = true;
    while scanning {
        scanning = unsafe {
            WIFI.as_mut()
                .map(|wifi| wifi.blocking_rpc(rpc::IsScanning {}).unwrap())
                .unwrap()
        };
    }

    let num = unsafe {
        WIFI.as_mut()
            .map(|wifi| wifi.blocking_rpc(rpc::ScanGetNumAPs {}).unwrap())
            .unwrap()
    };
    writeln!(textbuffer, "num scanning APs = {:?}", num).unwrap();
    terminal.write_str(textbuffer.as_str());
    textbuffer.truncate(0);

    let aps = unsafe {
        WIFI.as_mut()
            .map(|wifi| {
                wifi.blocking_rpc(rpc::ScanGetAP::<generic_array::typenum::consts::U3>::new())
            })
            .unwrap()
    };
    for ap in aps.unwrap().0 {
        writeln!(textbuffer, "{:?}", ap).unwrap();
        terminal.write_str(textbuffer.as_str());
        textbuffer.truncate(0);
    }

    loop {
        user_led.toggle();
        delay.delay_ms(90u16);

        disable_interrupts(|_cs| unsafe {
            WIFI.as_mut().map(|wifi| {
                // writeln!(textbuffer, "debug: {:08b}", wifi.debug_usart().bits()).unwrap();
                // terminal.write_str(textbuffer.as_str());
                // textbuffer.truncate(0);

                if let Some(fault) = wifi.fault() {
                    writeln!(textbuffer, "fault!: {:?}", fault).unwrap();
                    terminal.write_str(textbuffer.as_str());
                    textbuffer.truncate(0);
                    loop {}
                }

                if let Ok(mut reader) = wifi.debug_read() {
                    writeln!(textbuffer, "read: {:?}", reader.buf()).unwrap();
                    terminal.write_str(textbuffer.as_str());
                    textbuffer.truncate(0);
                    reader.to_release(reader.len());
                }
            });
        });
    }
}

wifi_singleton!(WIFI);

/// Handly helper for logging text to the screen.
struct Terminal {
    text_style: TextStyle<Rgb565, Font6x12>,
    cursor: Point,
    display: wio::LCD,
}

impl Terminal {
    pub fn new(mut display: wio::LCD) -> Self {
        // Clear the screen.
        let style = PrimitiveStyleBuilder::new()
            .fill_color(Rgb565::BLACK)
            .build();
        let backdrop = Rectangle::new(Point::new(0, 0), Point::new(320, 320)).into_styled(style);
        backdrop.draw(&mut display).ok().unwrap();

        Self {
            text_style: TextStyle::new(Font6x12, Rgb565::WHITE),
            cursor: Point::new(0, 0),
            display,
        }
    }

    pub fn write_str(&mut self, str: &str) {
        for character in str.chars() {
            self.write_character(character);
        }
    }

    pub fn write_character(&mut self, c: char) {
        if self.cursor.x >= 320 || c == '\n' {
            self.cursor = Point::new(0, self.cursor.y + Font6x12::CHARACTER_SIZE.height as i32);
        }
        if self.cursor.y >= 240 {
            // Clear the screen.
            let style = PrimitiveStyleBuilder::new()
                .fill_color(Rgb565::BLACK)
                .build();
            let backdrop =
                Rectangle::new(Point::new(0, 0), Point::new(320, 320)).into_styled(style);
            backdrop.draw(&mut self.display).ok().unwrap();
            self.cursor = Point::new(0, 0);
        }

        if c != '\n' {
            let mut buf = [0u8; 8];
            Text::new(c.encode_utf8(&mut buf), self.cursor)
                .into_styled(self.text_style)
                .draw(&mut self.display)
                .ok()
                .unwrap();

            self.cursor.x += (Font6x12::CHARACTER_SIZE.width + Font6x12::CHARACTER_SPACING) as i32;
        }
    }
}