#![no_std]
#![no_main]

//! Draw a square, circle and triangle on the screen of a SSD1306-backed
//! display using the `embedded_graphics` crate.
//!
//! This example is for the _Adafruit Feather M0_ series boards, using I2C to
//! communicate between the board and the display module/board.
//!
//! This example is based on:
//! - https://github.com/jamwaffles/ssd1306/blob/master/examples/graphics_i2c.rs
//! - https://github.com/atsamd-rs/atsamd/blob/master/boards/itsybitsy_m0/examples/i2c_ssd1306.rs
//!
//! You can either use the USB port + BOSSAC or a SWD debugger/programmer to
//! load this example onto the Feather M0 board.
//!
//! Adafruit Feather M0 Boards:
//! - Feather M0 Adalogger: https://www.adafruit.com/product/2796
//! - Feather M0 Basic: https://www.adafruit.com/product/2772
//!
//! Adafruit sells a display with a SSD1306 chipset
//! (https://www.adafruit.com/product/2900) as a "Featherwing" (can snap in to
//! other Feather boards if the correct headers are used, or snap into a
//! breadboard).  They also sell I2C and SPI "modules" that have a display and
//! the SSD1306 chip on a single board, which you then connect to the Feather
//! M0 with wires.  SSD1306 modules can also be found on various shopping
//! websites (Amazon, Ebay, //! Ali*).  Note that most SSD1306-backed display
//! modules are usually either 128x64 pixels or 128x32 pixels in size, but
//! there are other sizes running around in the wild.
//!
//! Wiring connections for the Adafruit OLED Featherwing:
//!
//! ```
//! OLED Featherwing -> Feather M0
//!     GND -> GND
//!     +5V -> VCC
//! GPIOSDA -> PA22
//! GPIOSCL -> PA23
//! ```
//!
//! Wiring connections for generic OLED modules:
//!
//! ```
//!      OLED module -> Feather M0
//! (black)  GND -> GND
//! (red)    +5V -> VCC
//! (yellow) SDA -> PA22
//! (green)  SCL -> PA23
//! ```
//!
//! Build this example with: `cargo build --example ssd1306_graphicsmode_i2c.rs`
//!

extern crate feather_m0 as hal;
extern crate embedded_graphics;
extern crate ssd1306;

// how to panic...
#[cfg(not(feature = "use_semihosting"))]
extern crate panic_halt;
#[cfg(feature = "use_semihosting")]
extern crate panic_semihosting;

use hal::clock::GenericClockController;
use hal::delay::Delay;
use hal::prelude::*;
use hal::time::KiloHertz;
use hal::entry;
use hal::pac::{CorePeripherals, Peripherals};

use embedded_graphics::prelude::*;
use embedded_graphics::primitives::{Circle, Line, Rectangle};
use embedded_graphics::pixelcolor::BinaryColor;
use ssd1306::prelude::*;
use ssd1306::Builder;

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
    let mut pins = hal::Pins::new(peripherals.PORT);
    let mut red_led = pins.d13.into_open_drain_output(&mut pins.port);
    let mut delay = Delay::new(core.SYST, &mut clocks);

    let i2c = hal::i2c_master(
        &mut clocks,
        KiloHertz(400),
        peripherals.SERCOM3,
        &mut peripherals.PM,
        pins.sda,
        pins.scl,
        &mut pins.port,
    );

    let mut disp: GraphicsMode<_> = Builder::new().connect_i2c(i2c).into();

    disp.init().unwrap();
    disp.flush().unwrap();

    // triangle
    disp.draw(
        Line::new(Point::new(8, 16 + 16), Point::new(8 + 16, 16 + 16))
            .stroke(Some(BinaryColor::On))
            .into_iter(),
    );
    disp.draw(
        Line::new(Point::new(8, 16 + 16), Point::new(8 + 8, 16))
            .stroke(Some(BinaryColor::On))
            .into_iter(),
    );
    disp.draw(
        Line::new(Point::new(8 + 16, 16 + 16), Point::new(8 + 8, 16))
            .stroke(Some(BinaryColor::On))
            .into_iter(),
    );

    // square
    disp.draw(
        Rectangle::new(Point::new(48, 16), Point::new(48 + 16, 16 + 16))
            .stroke(Some(BinaryColor::On))
            .into_iter(),
    );

    // circle
    disp.draw(
        Circle::new(Point::new(96, 16 + 8), 8)
            .stroke(Some(BinaryColor::On))
            .into_iter(),
    );

    disp.flush().unwrap();

    // blink the onboard blinkenlight (digital pin 13)
    loop {
        delay.delay_ms(200u8);
        red_led.set_high().unwrap();
        delay.delay_ms(200u8);
        red_led.set_low().unwrap();
    }
}
