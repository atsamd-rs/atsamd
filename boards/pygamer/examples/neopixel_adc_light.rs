//! Display light sensor reading on the neopixels.
//!
//! Note leds may appear white during debug. Either build for release or add
//! opt-level = 2 to profile.dev in Cargo.toml

#![no_std]
#![no_main]

use bsp::{entry, hal, pac, LightSensor, Pins};
#[cfg(not(feature = "panic_led"))]
use panic_halt as _;
use pygamer as bsp;

use embedded_hal_02::adc::OneShot;
use hal::adc::Adc;
use hal::clock::GenericClockController;
use hal::delay::Delay;
use hal::ehal::delay::DelayNs;
use hal::time::Hertz;
use hal::timer::TimerCounter;
use hal::timer_traits::InterruptDrivenTimer;
use pac::gclk::pchctrl::GENSELECT_A::GCLK11;
use pac::{CorePeripherals, Peripherals};
use smart_leds::hsv::{hsv2rgb, Hsv, RGB8};
use smart_leds::SmartLedsWrite;
use ws2812_timer_delay as ws2812;

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
    let pins = Pins::new(peripherals.PORT);

    let mut adc1 = Adc::adc1(peripherals.ADC1, &mut peripherals.MCLK, &mut clocks, GCLK11);
    let mut light: LightSensor = pins.light.into();

    let gclk0 = clocks.gclk0();
    let tc2_3 = clocks.tc2_tc3(&gclk0).unwrap();
    let mut timer = TimerCounter::tc3_(&tc2_3, peripherals.TC3, &mut peripherals.MCLK);
    timer.start(Hertz::MHz(3).into_duration());

    let neopixel_pin = pins.neopixel.into_push_pull_output();
    let mut neopixel = ws2812::Ws2812::new(timer, neopixel_pin);

    let mut delay = Delay::new(core.SYST, &mut clocks);

    const NUM_LEDS: usize = 5;
    let mut j: u8 = 0;

    loop {
        let light_data: u16 = adc1.read(&mut light).unwrap();

        let pos: usize = if light_data < 100 {
            0
        } else if (light_data >= 147) && (light_data < 1048) {
            1
        } else if (light_data >= 1048) && (light_data < 3048) {
            2
        } else if (light_data >= 3048) && (light_data < 3948) {
            3
        } else {
            4
        };

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

        delay.delay_ms(10);
    }
}
