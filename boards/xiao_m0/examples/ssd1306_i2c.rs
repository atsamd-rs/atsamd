#![no_std]
#![no_main]

extern crate panic_halt;

use embedded_graphics::{
    pixelcolor::BinaryColor,
    prelude::*,
    primitives::{Circle, PrimitiveStyleBuilder},
};
use hal::{clock::GenericClockController, delay::Delay, prelude::*, time::KiloHertz};
use pac::{CorePeripherals, Peripherals};
use ssd1306::{prelude::*, I2CDisplayInterface, Ssd1306};

use bsp::{entry, hal, pac};
use xiao_m0 as bsp;

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
    let mut delay = Delay::new(core.SYST, &mut clocks);
    let i2c = bsp::i2c_master(
        &mut clocks,
        KiloHertz(400),
        peripherals.SERCOM0,
        &mut peripherals.PM,
        pins.a4,
        pins.a5,
    );

    let interface = I2CDisplayInterface::new(i2c);
    let mut display = Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate0)
        .into_buffered_graphics_mode();

    display.init().unwrap();
    display.flush().unwrap();

    let style = PrimitiveStyleBuilder::new()
        .fill_color(BinaryColor::On)
        .build();

    let width = display.size().width as f32;
    let height = display.size().height as f32;
    let mut states = [0.05, 0.15, 0.25, 0.35, 0.45, 0.55, 0.65, 0.75, 0.85, 0.95]
        .map(|p| State::new(width * p, height / 2. * (1. + p), 0., 0., 0., -5.));

    loop {
        display.clear();
        states.iter_mut().for_each(|state| {
            Circle::new(state.into(), 8)
                .into_styled(style)
                .draw(&mut display)
                .unwrap();
            state.update(0.1);
        });
        display.flush().unwrap();
        delay.delay_ms(10u8);
    }
}

impl From<&mut State> for Point {
    fn from(s: &mut State) -> Self {
        Point::new(s.xy.x as i32, s.xy.y as i32)
    }
}

impl Vec {
    pub fn new(x: f32, y: f32) -> Vec {
        Vec { x, y }
    }
}

impl State {
    pub fn new(x: f32, y: f32, vx: f32, vy: f32, ax: f32, ay: f32) -> State {
        let xy = Vec::new(x, y);
        let v = Vec::new(vx, vy);
        let a = Vec::new(ax, ay);
        State { xy, v, a }
    }
    pub fn update(&mut self, dt: f32) {
        self.xy.x = self.xy.x + self.v.x * dt + self.a.x * dt * dt * 0.5;
        self.xy.y = self.xy.y + self.v.y * dt + self.a.y * dt * dt * 0.5;

        self.v.x = self.v.x + self.a.x * dt;
        self.v.y = self.v.y + self.a.y * dt;

        self.check_bounce();
    }

    fn check_bounce(&mut self) {
        if self.xy.x < 0. {
            self.v.x = -self.v.x;
            self.xy.x = 0.;
        }
        if self.xy.y < 0. {
            self.v.y = -self.v.y;
            self.xy.y = 0.;
        }
    }
}

struct Vec {
    pub x: f32,
    pub y: f32,
}

struct State {
    xy: Vec,
    v: Vec,
    a: Vec,
}
