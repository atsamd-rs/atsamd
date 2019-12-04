//! Place a bitmap image on the screen.
//! Convert a png to .raw bytes
//! * With imagemagick `convert ferris.png -flip -type truecolor -define bmp:subtype=RGB565 -depth 16 -strip ferris.bmp`
//! * Or export images directly from GIMP by saving as .bmp and choosing 16bit R5 G6 B5
//! Then `tail -c 11008 ferris.bmp > ferris.raw` where c is width*height*2 and our ferris.png was 86x64

#![no_std]
#![no_main]

#[allow(unused_imports)]
use panic_halt;
use pygamer as hal;

use embedded_graphics::image::Image16BPP;
use embedded_graphics::pixelcolor::PixelColorU16;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::Rect;
use embedded_graphics::Drawing;

use hal::clock::GenericClockController;
use hal::entry;
use hal::pac::{CorePeripherals, Peripherals};

#[entry]
fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap();
    let core = CorePeripherals::take().unwrap();
    let mut clocks = GenericClockController::with_internal_32kosc(
        peripherals.GCLK,
        &mut peripherals.MCLK,
        &mut peripherals.OSC32KCTRL,
        &mut peripherals.OSCCTRL,
        &mut peripherals.NVMCTRL,
    );
    let mut pins = hal::Pins::new(peripherals.PORT).split();
    let mut delay = hal::delay::Delay::new(core.SYST, &mut clocks);

    let (mut display, _backlight) = pins
        .display
        .init(
            &mut clocks,
            peripherals.SERCOM4,
            &mut peripherals.MCLK,
            peripherals.TC2,
            &mut delay,
            &mut pins.port,
        )
        .unwrap();

    let black_backdrop: Rect<PixelColorU16> =
        Rect::new(Coord::new(0, 0), Coord::new(160, 128)).with_fill(Some(0x0000u16.into()));
    display.draw(black_backdrop.into_iter());
    let ferris =
        Image16BPP::new(include_bytes!("./ferris.raw"), 86, 64).translate(Coord::new(42, 32));
    display.draw(ferris.into_iter());

    loop {}
}
