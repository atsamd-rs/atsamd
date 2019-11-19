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
use hal::pac::{CorePeripherals, Peripherals};
use hal::{display, entry};

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
    let mut pins = hal::Pins::new(peripherals.PORT);
    let mut delay = hal::delay::Delay::new(core.SYST, &mut clocks);

    let (mut display, _backlight) = display(
        &mut clocks,
        peripherals.SERCOM4,
        &mut peripherals.MCLK,
        pins.accel_irq,
        pins.tft_mosi,
        pins.tft_sck,
        pins.tft_reset,
        pins.tft_cs,
        pins.tft_dc,
        pins.tft_backlight,
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
