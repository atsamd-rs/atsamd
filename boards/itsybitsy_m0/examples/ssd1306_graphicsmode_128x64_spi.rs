//! Draw a square, circle and triangle bounded inside of a box on the screen
//! borders of an OLED using the `embedded_graphics` crate, and blink the red
//! LED connected to pin13
//!
//! This example is for the _Adafruit ItsyBitsy M0_ series boards connected to
//! an SPI OLED display board based on the SSD1306 chipset which has 128x64
//! pixel resolution.
//!
//! This example is based on:
//! - https://github.com/jamwaffles/ssd1306/blob/master/examples/graphics.rs
//! - A previous example located in boards/itsybitsy_m0/examples/spi_ssd1306.rs
//!   which is not available anymore
//!
//! You can either use the USB port on the ItsyBitsy M0 + BOSSAC or a SWD
//! debugger/programmer to load this example onto the ItsyBitsy M0 board.
//!
//! Adafruit ItsyBitsy M0 Board: https://www.adafruit.com/product/3727
//!
//! Adafruit OLED module that was used for this demo:
//! - https://www.adafruit.com/product/938
//!
//! All Adafruit OLED modules (SPI/I2C)
//! - https://www.adafruit.com/category/98
//!
//! Other "generic" SPI SSD1306 modules found on Amazon/Ebay/Ali* should work
//! with this example as well.
//!
//! For this demo, it's assumed that the SSD1306 display is the only device on
//! the SPI bus, so the "CS" pin of the display needs to be connected to GND
//! for the demo to work.
//!
//! Wiring connections for the Adafruit/generic OLED SSD1306 displays:
//!
//! ```
//! ItsyBitsy M0  -> SPI SSD1306 OLED Display
//! ---------------------------------------
//!         GND -> GND
//!         3V3 -> VCC
//!         SCK -> Clk (Adafruit) or D0 (generic)
//!        MOSI -> Data (Adafruit) or D1 (generic)
//!          D9 -> Rst (Adafruit) or RES (generic)
//!          D7 -> DC
//!         GND -> CS (see note above about grounding 'CS' pin)
//! ```
//!
//! Build this example with: `cargo build --example
//! ssd1306_graphicsmode_128x64_spi`

#![no_std]
#![no_main]

#[cfg(not(feature = "use_semihosting"))]
use panic_halt as _;
#[cfg(feature = "use_semihosting")]
use panic_semihosting as _;

use embedded_graphics::pixelcolor::BinaryColor;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::{Circle, PrimitiveStyleBuilder, Rectangle, Triangle};
use ssd1306::{prelude::*, Ssd1306};

use bsp::hal;
use bsp::pac;
use itsybitsy_m0 as bsp;

use bsp::entry;
use hal::clock::GenericClockController;
use hal::delay::Delay;
use hal::prelude::*;
use hal::time::MegaHertz;
use pac::{CorePeripherals, Peripherals};

#[entry]
fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap();
    let core = CorePeripherals::take().unwrap();
    let mut clocks = GenericClockController::with_internal_32kosc(
        peripherals.GCLK,
        &mut peripherals.PM,
        &mut peripherals.SYSCTRL,
        &mut peripherals.NVMCTRL,
    );
    let pins = bsp::Pins::new(peripherals.PORT);
    let mut red_led: bsp::RedLed = pins.d13.into();
    let mut delay = Delay::new(core.SYST, &mut clocks);

    let spi_sercom: bsp::SpiSercom = peripherals.SERCOM4;
    let spi = bsp::spi_master(
        &mut clocks,
        MegaHertz(10),
        spi_sercom,
        &mut peripherals.PM,
        pins.sclk,
        pins.mosi,
        pins.miso,
    );

    let dc = pins.d7.into_push_pull_output();
    let mut rst = pins.d9.into_push_pull_output();

    // default DisplaySize: 128x64 pixels; see the
    // ssd1306::builder::Builder::new() method definition for more info
    let interface = SPIInterfaceNoCS::new(spi, dc);
    let mut disp = Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate0)
        .into_buffered_graphics_mode();

    disp.reset(&mut rst, &mut delay).unwrap();
    disp.init().unwrap();

    let yoffset = 24;
    let x_max = 127;
    let y_max = 63;

    let style = PrimitiveStyleBuilder::new()
        .stroke_color(BinaryColor::On)
        .stroke_width(1)
        .build();

    // screen outline
    Rectangle::with_corners(Point::new(0, 0), Point::new(x_max, y_max))
        .into_styled(style)
        .draw(&mut disp)
        .unwrap();

    // triangle
    // 'Triangle' requires 'embedded_graphics' 0.6 or newer...
    Triangle::new(
        Point::new(16, 16 + yoffset),
        Point::new(16 + 16, 16 + yoffset),
        Point::new(16 + 8, yoffset),
    )
    .into_styled(style)
    .draw(&mut disp)
    .unwrap();

    // square
    Rectangle::with_corners(Point::new(54, yoffset), Point::new(54 + 16, 16 + yoffset))
        .into_styled(style)
        .draw(&mut disp)
        .unwrap();

    // circle
    Circle::new(Point::new(100, 8 + yoffset), 8)
        .into_styled(style)
        .draw(&mut disp)
        .unwrap();

    disp.flush().unwrap();

    loop {
        delay.delay_ms(400_u16);
        red_led.set_high().unwrap();
        delay.delay_ms(400_u16);
        red_led.set_low().unwrap();
    }
}
