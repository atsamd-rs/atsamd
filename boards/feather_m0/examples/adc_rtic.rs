//! Uses RTIC with the RTC as time source to read the battery level.
//! This example uses the SysTick as the Monotonic tier.
//!
//! The idle task is sleeping the CPU, so in practice this gives similar power
//! figure as the "sleeping_timer_rtc" example.
#![no_std]
#![no_main]

use feather_m0 as bsp;

#[cfg(not(feature = "use_semihosting"))]
use panic_halt as _;

#[cfg(feature = "use_semihosting")]
use panic_semihosting as _;

#[rtic::app(device = bsp::pac, peripherals = true, dispatchers = [SERCOM5])]
mod app {
    use super::*;
    use bsp::{hal, pin_alias};
    use hal::adc::{Adc, Resolution};
    use hal::clock::GenericClockController;
    use hal::pac::{Peripherals, ADC};
    use hal::prelude::*;
    use hal::time::Hertz;
    use rtt_target::{rprintln, rtt_init_print};
    use systick_monotonic::*;

    #[local]
    struct Local {
        adc: Adc<ADC>,
        adc_pin: bsp::Battery,
    }

    #[shared]
    struct Shared {}

    #[monotonic(binds = SysTick, default = true)]
    type SysTickMonotonic = Systick<100>;

    #[init]
    fn init(cx: init::Context) -> (Shared, Local, init::Monotonics) {
        rtt_init_print!();
        let mut peripherals: Peripherals = cx.device;
        let pins = bsp::Pins::new(peripherals.PORT);
        let mut core: rtic::export::Peripherals = cx.core;
        let mut clocks = GenericClockController::with_external_32kosc(
            peripherals.GCLK,
            &mut peripherals.PM,
            &mut peripherals.SYSCTRL,
            &mut peripherals.NVMCTRL,
        );
        let gclk = clocks.gclk0();
        let freq: Hertz = gclk.into();
        let systick = Systick::new(core.SYST, freq.0);

        let mut adc = Adc::adc(peripherals.ADC, &mut peripherals.PM, &mut clocks);
        adc.resolution(Resolution::_12BIT);
        let adc_pin: bsp::Battery = pin_alias!(pins.battery).into();
        // We can use the RTC in standby for maximum power savings
        core.SCB.set_sleepdeep();

        read_battery::spawn().unwrap();

        (Shared {}, Local { adc, adc_pin }, init::Monotonics(systick))
    }

    #[task(local = [adc, adc_pin])]
    fn read_battery(cx: read_battery::Context) {
        let data: u16 = cx.local.adc.read(cx.local.adc_pin).unwrap();
        const RESOLUTION_BITS: u32 = 12;
        let max_range = 1 << RESOLUTION_BITS;
        let voltage = ((data as f32) * 3.3 * 2.0) / (max_range as f32);
        rprintln!("Battery level {} - {}", voltage, data);
        read_battery::spawn_after(fugit::Duration::<u64, 1, 100>::secs(5)).ok();
    }
}
