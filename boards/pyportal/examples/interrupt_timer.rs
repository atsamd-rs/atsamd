#![no_std]
#![no_main]

#[cfg(not(feature = "use_semihosting"))]
use panic_halt as _;
#[cfg(feature = "use_semihosting")]
use panic_semihosting as _;

use bsp::{hal, pac, pin_alias};
use pyportal as bsp;

use hal::{
    clock::GenericClockController,
    ehal::digital::StatefulOutputPin,
    rtc::modes::{
        mode0::{Compare0, RtcMode0},
        RtcMode,
    },
};
use pac::{interrupt, Interrupt, NVIC};

use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;

#[entry]
fn main() -> ! {
    let mut peripherals = pac::Peripherals::take().unwrap();

    let _clocks = GenericClockController::with_internal_32kosc(
        peripherals.gclk,
        &mut peripherals.mclk,
        &mut peripherals.osc32kctrl,
        &mut peripherals.oscctrl,
        &mut peripherals.nvmctrl,
    );

    // Select the 32khz source
    peripherals
        .osc32kctrl
        .rtcctrl()
        .write(|w| w.rtcsel().ulp1k());

    let rtc = peripherals.rtc;

    rtc.mode0().dbgctrl().write(|w| w.dbgrun().set_bit());
    RtcMode0::disable(&rtc);
    RtcMode0::reset(&rtc);
    RtcMode0::set_mode(&rtc);

    RtcMode0::start_and_initialize(&rtc);
    RtcMode0::clear_interrupt_flag::<Compare0>(&rtc);
    RtcMode0::enable_interrupt::<Compare0>(&rtc);
    rtc.mode0().evctrl().write(|w| w.cmpeo0().set_bit());
    while rtc.mode0().syncbusy().read().

    unsafe {
        let mut nvic: cortex_m::peripheral::NVIC = core::mem::transmute(());
        nvic.set_priority(Interrupt::RTC, 1);
        NVIC::unmask(Interrupt::RTC);
    }

    RtcMode0::enable(&rtc);

    let now = RtcMode0::count(&rtc);
    let at = now + 2_000;
    hprintln!("Currently {}, alarm set at {}", now, at);

    RtcMode0::set_compare(&rtc, 0, at);

    loop {}
}

#[interrupt]
fn RTC() {
    hprintln!("Hello");
    let peripherals = unsafe { pac::Peripherals::steal() };
    let pins = bsp::Pins::new(peripherals.port);
    let mut red_led: bsp::RedLed = pin_alias!(pins.red_led).into();

    red_led.toggle().unwrap();
}
