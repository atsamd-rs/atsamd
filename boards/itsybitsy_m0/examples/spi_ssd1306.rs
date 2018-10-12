#![no_std]
#![no_main]

extern crate itsybitsy_m0 as hal;
extern crate panic_abort;

extern crate embedded_graphics;
extern crate ssd1306;

use hal::clock::GenericClockController;
use hal::delay::Delay;
use hal::prelude::*;
use hal::{entry, CorePeripherals, Peripherals};
use hal::time::MegaHertz;

use embedded_graphics::prelude::*;
use embedded_graphics::primitives::{Circle, Line, Rect};
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

    let spi = hal::spi_master(
        &mut clocks,
        MegaHertz(10),
        peripherals.SERCOM4,
        &mut peripherals.PM,
        pins.sck,
        pins.mosi,
        pins.miso,
        &mut pins.port,
        );



    let dc = pins.d9.into_open_drain_output(&mut pins.port);
    let mut rst = pins.d7.into_open_drain_output(&mut pins.port);

    let mut disp: GraphicsMode<_> = Builder::new().connect_spi(spi, dc).into();

    disp.reset(&mut rst, &mut delay);
    disp.init().unwrap();
    disp.flush().unwrap();

    disp.draw(
        Line::new(Coord::new(8, 16 + 16), Coord::new(8 + 16, 16 + 16))
        .with_stroke(Some(1u8.into()))
        .into_iter(),
        );
    disp.draw(
        Line::new(Coord::new(8, 16 + 16), Coord::new(8 + 8, 16))
        .with_stroke(Some(1u8.into()))
        .into_iter(),
        );
    disp.draw(
        Line::new(Coord::new(8 + 16, 16 + 16), Coord::new(8 + 8, 16))
        .with_stroke(Some(1u8.into()))
        .into_iter(),
        );

    disp.draw(
        Rect::new(Coord::new(48, 16), Coord::new(48 + 16, 16 + 16))
        .with_stroke(Some(1u8.into()))
        .into_iter(),
        );

    disp.draw(
        Circle::new(Coord::new(96, 16 + 8), 8)
        .with_stroke(Some(1u8.into()))
        .into_iter(),
        );

    disp.flush().unwrap();



    loop {
        delay.delay_ms(200u8);
        red_led.set_high();
        delay.delay_ms(200u8);
        red_led.set_low();
    }
}
