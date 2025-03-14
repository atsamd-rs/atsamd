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
    clock::{GenericClockController, ClockGenId, ClockSource},
    rtc::modes::{mode0::RtcMode0, RtcMode}
};

use cortex_m_semihosting::hprintln;

use embassy_time::Timer;

#[embassy_executor::main]
async fn main(_s: embassy_executor::Spawner) {
    let mut peripherals = pac::Peripherals::take().unwrap();
    let _core = pac::CorePeripherals::take().unwrap();

    let mut clocks = GenericClockController::with_internal_32kosc(
        peripherals.gclk,
        &mut peripherals.mclk,
        &mut peripherals.osc32kctrl,
        &mut peripherals.oscctrl,
        &mut peripherals.nvmctrl
    );
    let pins = bsp::Pins::new(peripherals.port);
    let mut red_led: bsp::RedLed = pin_alias!(pins.red_led).into();

    // Select the 32khz source
    peripherals.osc32kctrl.rtcctrl().write(|w| w.rtcsel().ulp32k());

    peripherals.rtc.mode0().dbgctrl().write(|w| w.dbgrun().set_bit());

    unsafe { hal::rtc::embassy::init(); }


    loop {
        red_led.toggle().unwrap();
        hprintln!("The time is {}", RtcMode0::count(&peripherals.rtc));
        Timer::after_secs(1).await;
    }
}
