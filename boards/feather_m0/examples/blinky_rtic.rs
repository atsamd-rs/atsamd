//! Uses RTIC with the RTC as time source to blink an LED.
//!
//! The idle task is sleeping the CPU, so in practice this gives similar power
//! figure as the "sleeping_timer_rtc" example.
//!
#![no_std]
#![no_main]

extern crate cortex_m;
extern crate cortex_m_semihosting;
extern crate feather_m0 as hal;
#[cfg(not(feature = "use_semihosting"))]
extern crate panic_halt;
#[cfg(feature = "use_semihosting")]
extern crate panic_semihosting;
extern crate rtic;

use hal::clock::{enable_internal_32kosc, ClockGenId, ClockSource, GenericClockController};
use hal::delay::Delay;
use hal::entry;
use hal::pac::{CorePeripherals, Peripherals};
use hal::prelude::*;
use hal::rtc::{Rtc, RtcMonotonic, Instant, U32Ext as _};

#[rtic::app(device = hal::pac, peripherals = true, monotonic = crate::RtcMonotonic)]
const APP: () = {
  struct Resources {
    red_led: hal::gpio::Pin<hal::gpio::v2::PA17, hal::gpio::v2::Output<hal::gpio::v2::PushPull>>,
  }

  #[init(spawn = [blink])]
  fn init(cx: init::Context) -> init::LateResources {
    let mut peripherals: Peripherals = cx.device;
    let mut pins = hal::Pins::new(peripherals.PORT);
    let mut core: rtic::Peripherals = cx.core;
    let mut clocks = GenericClockController::with_external_32kosc(
      peripherals.GCLK,
      &mut peripherals.PM,
      &mut peripherals.SYSCTRL,
      &mut peripherals.NVMCTRL,
    );
    let gclk = clocks.gclk0();
    let rtc_clock_src = clocks
        .configure_gclk_divider_and_source(ClockGenId::GCLK2, 1, ClockSource::XOSC32K, false)
        .unwrap();
    clocks.configure_standby(ClockGenId::GCLK2, true);
    let rtc_clock = clocks.rtc(&rtc_clock_src).unwrap();
    let rtc = Rtc::new(peripherals.RTC, rtc_clock.freq(), &mut peripherals.PM);
    let _: RtcMonotonic = rtc.into();
    let mut red_led = pins.d13.into_open_drain_output(&mut pins.port);

    // We can use the RTC in standby for maximum power savings
    core.SCB.set_sleepdeep();

    // Start the blink task
    cx.spawn.blink().unwrap();

    init::LateResources {
      red_led,
    }
  }

  #[task(schedule = [blink], resources = [red_led])]
  fn blink(mut cx: blink::Context) {
    cx.resources.red_led.toggle();
    cx.schedule.blink(cx.scheduled + 32768.cycles()); // 1 second
  }

  // RTIC requires that unused interrupts are declared in an extern block when
  // using software tasks; these free interrupts will be used to dispatch the
  // software tasks.
  extern "C" {
    fn EVSYS();
  }
};