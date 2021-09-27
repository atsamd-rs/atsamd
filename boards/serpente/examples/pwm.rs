#![no_std]
#![no_main]

use bsp::hal;
use serpente as bsp;

#[cfg(not(feature = "use_semihosting"))]
use panic_halt as _;
#[cfg(feature = "use_semihosting")]
use panic_semihosting as _;

use bsp::entry;
use hal::clock::GenericClockController;
use hal::delay::Delay;
use hal::pac::{CorePeripherals, Peripherals};
use hal::prelude::*;
use hal::pwm::{Channel, Pwm0};

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
    let gclk0 = clocks.gclk0();

    let mut pins = bsp::Pins::new(peripherals.PORT);
    let mut _red_led = pins.led_r.into_function_f(&mut pins.port);
    let mut _green_led = pins.led_g.into_function_f(&mut pins.port);
    let mut _blue_led = pins.led_b.into_function_f(&mut pins.port);

    let mut pwm0 = Pwm0::new(
        &clocks.tcc0_tcc1(&gclk0).unwrap(),
        1.khz(),
        peripherals.TCC0,
        &mut peripherals.PM,
    );
    let max_duty = pwm0.get_max_duty();

    let mut delay = Delay::new(core.SYST, &mut clocks);
    loop {
        for i in 0..255 {
            let rgb = wheel(i);
            pwm0.set_duty(Channel::_0, max_duty * (rgb.r as u32) / 255);
            pwm0.set_duty(Channel::_2, max_duty * (rgb.g as u32) / 255);
            pwm0.set_duty(Channel::_3, max_duty * (rgb.b as u32) / 255);
            delay.delay_ms(5u16);
        }
    }
}

pub struct RGB {
    r: u8,
    g: u8,
    b: u8,
}

fn wheel(mut wheel_pos: u8) -> RGB {
    wheel_pos = 255 - wheel_pos;
    if wheel_pos < 85 {
        return RGB {
            r: 255 - wheel_pos * 3,
            g: 0,
            b: wheel_pos * 3,
        };
    }
    if wheel_pos < 170 {
        wheel_pos -= 85;
        return RGB {
            r: 0,
            g: wheel_pos * 3,
            b: 255 - wheel_pos * 3,
        };
    }
    wheel_pos -= 170;
    RGB {
        r: wheel_pos * 3,
        g: 255 - wheel_pos * 3,
        b: 0,
    }
}
