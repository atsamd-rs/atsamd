//! LIS3DH accelerometer example. Move the neopixel led by tilting left and right.

#![no_std]
#![no_main]

#[allow(unused_imports)]
use panic_halt;
use pygamer as hal;

use hal::entry;
use hal::pac::{CorePeripherals, Peripherals};
use hal::prelude::*;
use hal::time::KiloHertz;
use hal::timer::SpinTimer;
use hal::{clock::GenericClockController, delay::Delay};

use smart_leds::{
    hsv::{hsv2rgb, Hsv, RGB8},
    SmartLedsWrite,
};

use lis3dh::{accelerometer::Accelerometer, Lis3dh};

#[entry]
fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap();
    let core_peripherals = CorePeripherals::take().unwrap();

    let mut clocks = GenericClockController::with_internal_32kosc(
        peripherals.GCLK,
        &mut peripherals.MCLK,
        &mut peripherals.OSC32KCTRL,
        &mut peripherals.OSCCTRL,
        &mut peripherals.NVMCTRL,
    );

    let mut delay = Delay::new(core_peripherals.SYST, &mut clocks);
    let mut pins = hal::Pins::new(peripherals.PORT).split();

    // neopixels
    let timer = SpinTimer::new(4);
    let mut neopixel = pins.neopixel.init(timer, &mut pins.port);

    // i2c
    let i2c = pins.i2c.init(
        &mut clocks,
        KiloHertz(400),
        peripherals.SERCOM2,
        &mut peripherals.MCLK,
        &mut pins.port,
    );

    let mut lis3dh = Lis3dh::new(i2c, 0x19).unwrap();
    lis3dh.set_range(lis3dh::Range::G2).unwrap();
    lis3dh.set_datarate(lis3dh::DataRate::Hz_100).unwrap();

    const NUM_LEDS: usize = 5;

    let threshold: i16 = 20;
    let mut pos: usize = 2;
    let mut tilt: i16 = 0;

    let mut j: u8 = 0;
    loop {
        let lis = lis3dh.acceleration().unwrap();

        //what about like.. momentum, more angle or longer its been at angle stops slower
        //like.. steps larger so it gets easier. also on a bigger number tilt?

        // naive solution.. threshold tilt
        // better.. delay filter?

        // actually 2 thresholds, first you have to be tilted enough (gt / lt 1000) to be counted
        if lis.x > 1000 {
            tilt += 1;
        } else if lis.x < -1000 {
            tilt -= 1;
        }

        // then we need threshold amount of counted tilts to inc/dec position
        if tilt.abs() > threshold {
            //todo clamp is nightly
            if tilt.is_negative() {
                if pos > 0 {
                    pos -= 1;
                }
            } else {
                if pos < 4 {
                    pos += 1;
                }
            }
            tilt = 0;
        }

        //finally paint the one led wherever the position is
        let _ = neopixel.write((0..NUM_LEDS).map(|i| {
            if i == pos {
                hsv2rgb(Hsv {
                    hue: j,
                    sat: 255,
                    val: 32,
                })
            } else {
                RGB8::default()
            }
        }));

        //incremement the hue easing
        j = j.wrapping_add(1);

        //don't update faster than the accell is reading
        delay.delay_ms(10u8);
    }
}
