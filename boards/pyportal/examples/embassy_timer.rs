#![no_std]
#![no_main]

#[cfg(not(feature = "use_semihosting"))]
use panic_halt as _;
#[cfg(feature = "use_semihosting")]
use panic_semihosting as _;

use bsp::{hal, pac, pin_alias};
use pyportal as bsp;
use hal::{
    ehal::digital::StatefulOutputPin,
};

use embassy_time::Timer;

#[embassy_executor::main]
async fn main(_s: embassy_executor::Spawner) {
    let peripherals = pac::Peripherals::take().unwrap();
    let _core = pac::CorePeripherals::take().unwrap();
    let pins = bsp::Pins::new(peripherals.port);
    let mut red_led: bsp::RedLed = pin_alias!(pins.red_led).into();

    // Select the 32khz source
    // SAFETY: not in a critical section
    unsafe {
        hal::rtc::embassy::init();
    }

    loop {
        red_led.toggle().unwrap();
        Timer::after_secs(1).await;
    }
}
