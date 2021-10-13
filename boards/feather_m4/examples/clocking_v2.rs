#![no_main]
#![no_std]

use panic_halt as _;

use core::fmt::Write as _;

use atsamd_hal::{
    clock::v2::{
        dpll::Dpll,
        gclk,
        gclk::{Gclk1Div, GclkDiv},
        gclkio::GclkOut,
        pclk::Pclk,
        retrieve_clocks,
        rtc::{Freq1k, Rtc},
        xosc32k::{marker::Xosc32k as Xosc32kMarker, Xosc32k},
    },
    ehal::serial::Read as _,
    ehal::serial::Write,
    gpio::v2::{Alternate, Pin, Pins, D, PA04, PA05},
    rtc::ClockMode,
    sercom::v2::{uart::*, IoSet3, Sercom0},
    time::U32Ext,
};

use rtic::app;

#[app(device = atsamd_hal::pac, peripherals = true)]
mod app {
    use super::*;

    #[shared]
    struct SharedResources {
        uart: Uart<
            Config<Pads<Sercom0, IoSet3, Pin<PA05, Alternate<D>>, Pin<PA04, Alternate<D>>>>,
            Duplex,
        >,
        rtc: Rtc<ClockMode, Freq1k, Xosc32kMarker>,
    }

    #[local]
    struct LocalResources {}

    #[init]
    fn init(cx: init::Context) -> (SharedResources, LocalResources, init::Monotonics()) {
        let mut device = cx.device;

        // Get the clocks & tokens
        let (gclk0, dfll, osculp32k, tokens) = retrieve_clocks(
            device.OSCCTRL,
            device.OSC32KCTRL,
            device.GCLK,
            device.MCLK,
            &mut device.NVMCTRL,
        );
        let (_, _, _, mut mclk) = unsafe { tokens.pac.steal() };

        // Get the pins
        let pins = Pins::new(device.PORT);

        // Take `Dfll` 48 MHz, divide down to `2 MHz` through `Gclk1`
        let (gclk1, dfll) = gclk::Gclk::new(tokens.gclks.gclk1, dfll);
        let gclk1 = gclk1.div(Gclk1Div::Div(24)).enable();

        // Output `Gclk1` on PB15 pin
        let (_, gclk1) = GclkOut::enable(tokens.gclk_io.gclk_out1, pins.pb15, gclk1, false);

        // Setup a peripheral channel to power up `Dpll0` from `Gclk1`
        let (pclk_dpll0, gclk1) = Pclk::enable(tokens.pclks.dpll0, gclk1);

        // Configure `Dpll0` with `2 * 60 + 0/32 = 120 MHz` frequency
        let dpll0 = Dpll::from_pclk(tokens.dpll0, pclk_dpll0)
            .set_loop_div(60, 0)
            .enable()
            .unwrap_or_else(|_| panic!("Dpll0 failed to pass assertions"));

        //// Swap source of `Gclk0` from Dfll to Dpll0, `48 Mhz -> 120 MHz`
        let (gclk0, _dfll, _dpll0) = gclk0.swap(dfll, dpll0);

        //// Output `Gclk0` on pin PB14
        let (_, gclk0) = GclkOut::enable(tokens.gclk_io.gclk_out0, pins.pb14, gclk0, false);

        // Setup a peripheral channel to power up `Dpll1` from `Gclk1`
        let (pclk_dpll1, _gclk1) = Pclk::enable(tokens.pclks.dpll1, gclk1);

        // Configure `Dpll1` with `2 * 50 + 0/32 = 100 MHz` frequency
        let dpll1 = Dpll::from_pclk(tokens.dpll1, pclk_dpll1)
            .set_loop_div(50, 0)
            .enable()
            .unwrap_or_else(|_| panic!("Dpll1 failed to pass assertions"));

        // Output `Dpll1` on PB20 pin via `Gclk6`, divided by 200 resulting in 0.5 MHz
        // output frequency
        let (gclk6, _dpll1) = gclk::Gclk::new(tokens.gclks.gclk6, dpll1);
        let gclk6 = gclk6.div(GclkDiv::Div(200)).enable();
        let (_, _gclk6) = GclkOut::enable(tokens.gclk_io.gclk_out6, pins.pb12, gclk6, false);

        // Configure `Xosc32k` with both outputs (1kHz, 32kHz) activated
        let xosc32k = Xosc32k::from_crystal(tokens.xosc32k, pins.pa00, pins.pa01)
            .set_gain_mode(true)
            .set_on_demand(false)
            .set_run_standby(true)
            .enable()
            .activate_1k()
            .activate_32k();

        // Output `Xosc32k` on PB16 pin via `Gclk2`, divided by 2 resulting in 16 kHz
        // output frequency
        let (gclk2, xosc32k) = gclk::Gclk::new(tokens.gclks.gclk2, xosc32k);
        let gclk2 = gclk2.div(gclk::GclkDiv::Div(2)).enable();
        let (_, _gclk2) = GclkOut::enable(tokens.gclk_io.gclk_out2, pins.pb16, gclk2, false);

        // Output `OscUlp32k` on PB11 pin via `Gclk5`, without any division resulting in
        // 32 kHz output frequency
        // Note: Div(0) is equivalent to Div(1)
        let (gclk5, _osculp32k) = gclk::Gclk::new(tokens.gclks.gclk5, osculp32k);
        let gclk5 = gclk5.div(gclk::GclkDiv::Div(0)).enable();
        let (_, _gclk5) = GclkOut::enable(tokens.gclk_io.gclk_out5, pins.pb11, gclk5, false);

        // Setup a peripheral channel to power up `Uart` from `Gclk0`
        let (pclk_sercom0, _gclk0) = Pclk::enable(tokens.pclks.sercom0, gclk0);

        let mut uart = Config::new(
            &mclk,
            device.SERCOM0,
            Pads::default().rx(pins.pa05).tx(pins.pa04),
            pclk_sercom0.freq(),
        )
        .baud(115_200.hz(), BaudMode::Arithmetic(Oversampling::Bits16))
        .enable();
        uart.enable_interrupts(Flags::RXC);

        // Setup an `Rtc` in `ClockMode` while using `Xosc32k`'s 1 kHz signal
        let (rtc, _xosc32k) = Rtc::clock_mode(device.RTC, xosc32k, &mut mclk);

        writeln!(&mut uart as &mut dyn Write<_, Error = _>, "RTIC booted!").unwrap();

        (
            SharedResources { uart, rtc },
            LocalResources {},
            init::Monotonics(),
        )
    }

    #[task(binds = SERCOM0_2, shared = [uart, rtc])]
    fn uart(cx: uart::Context) {
        let mut uart = cx.shared.uart;
        let mut rtc = cx.shared.rtc;
        // Read from `Uart` to clean interrupt flag
        let _ = uart.lock(|u| u.read().unwrap());

        // Print out `DateTime` coming from `Rtc`
        uart.lock(|u| {
            writeln!(
                u as &mut dyn Write<_, Error = _>,
                "{:#?}",
                rtc.lock(|r| r.current_time())
            )
            .unwrap()
        });
    }
}
