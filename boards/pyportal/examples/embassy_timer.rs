#![no_std]
#![no_main]

#[cfg(not(feature = "use_semihosting"))]
use panic_halt as _;
#[cfg(feature = "use_semihosting")]
use panic_semihosting as _;

use bsp::{hal, pac, pin_alias};
use hal::{ehal::digital::StatefulOutputPin,
          clock::v2::{rtcosc::RtcOsc, osculp32k::OscUlp32k, clock_system_at_reset}};
use pyportal as bsp;

use embassy_time::Timer;

hal::embassy_time!(Driver);

#[embassy_executor::main]
async fn main(_s: embassy_executor::Spawner) {
    let mut peripherals = pac::Peripherals::take().unwrap();
    let _core = pac::CorePeripherals::take().unwrap();
    let pins = bsp::Pins::new(peripherals.port);
    let mut red_led: bsp::RedLed = pin_alias!(pins.red_led).into();

    // Select the 32khz source
    let (_, clocks, tokens) = clock_system_at_reset(
        peripherals.oscctrl,
        peripherals.osc32kctrl,
        peripherals.gclk,
        peripherals.mclk,
        &mut peripherals.nvmctrl,
    );

    let (osculp32k, _) = OscUlp32k::enable(tokens.osculp32k.osculp32k, clocks.osculp32k_base);

    let (rtc, _) = RtcOsc::enable(tokens.rtcosc, osculp32k);

    // SAFETY: not in a critical section

    unsafe {
        Driver::init(rtc);
    }

    loop {
        red_led.toggle().unwrap();
        Timer::after_secs(1).await;
    }
}
