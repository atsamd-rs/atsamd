#![no_main]
#![no_std]

use panic_halt as _;

use core::fmt::Write as _;

use atsamd_hal::{
    clock::v2::{
        self as clock,
        dpll::Dpll,
        gclk::{Gclk, Gclk1Div, GclkDiv},
        gclkio::GclkOut,
        osculp32k::OscUlp32k,
        pclk::Pclk,
        rtcosc::RtcOsc,
        xosc32k::{ControlGainMode, Xosc1k, Xosc32k, XoscBase},
    },
    ehal::serial::Read as _,
    ehal::serial::Write,
    gpio::v2::{Pins, PA04, PA05},
    rtc::{ClockMode, Rtc},
    sercom::v2::{
        uart::{self, BaudMode, Flags, Oversampling},
        IoSet3, Sercom0,
    },
    time::U32Ext,
};

use rtic::app;

type Pads = uart::PadsFromIds<Sercom0, IoSet3, PA05, PA04>;
type Uart = uart::Uart<uart::Config<Pads>, uart::Duplex>;

#[app(device = atsamd_hal::pac, peripherals = true)]
mod app {
    use super::*;

    #[shared]
    struct SharedResources {
        uart: Uart,
        rtc: Rtc<ClockMode>,
    }

    #[local]
    struct LocalResources {}

    #[init]
    fn init(cx: init::Context) -> (SharedResources, LocalResources, init::Monotonics()) {
        let mut device = cx.device;

        // Get the clocks & tokens
        let (_buses, clocks, tokens) = clock::por_state(
            device.OSCCTRL,
            device.OSC32KCTRL,
            device.GCLK,
            device.MCLK,
            &mut device.NVMCTRL,
        );

        // This is required because the `sercom` and `rtc` modules have not yet
        // been update to use `clock::v2`
        let (_, _, _, mut mclk) = unsafe { clocks.pac.steal() };

        // Get the pins
        let pins = Pins::new(device.PORT);

        // Take `Dfll` 48 MHz, divide down to `2 MHz` through `Gclk1`
        let (gclk1, dfll) = Gclk::new(tokens.gclks.gclk1, clocks.dfll);
        let gclk1 = gclk1.div(Gclk1Div::Div(24)).enable();

        // Output `Gclk1` on PB15 pin
        let (_gclk_out1, gclk1) =
            GclkOut::enable(tokens.gclk_io.gclk_out1, pins.pb15, gclk1, false);

        // Setup a peripheral channel to power up `Dpll0` from `Gclk1`
        let (pclk_dpll0, gclk1) = Pclk::enable(tokens.pclks.dpll0, gclk1);

        // Configure `Dpll0` with `2 * 60 + 0/32 = 120 MHz` frequency
        let dpll0 = Dpll::from_pclk(tokens.dpll0, pclk_dpll0)
            .set_loop_div(60, 0)
            .enable()
            .unwrap_or_else(|_| panic!("Dpll0 failed to pass assertions"));

        // Swap source of `Gclk0` from Dfll to Dpll0, `48 Mhz -> 120 MHz`
        let (gclk0, _dfll, _dpll0) = clocks.gclk0.swap(dfll, dpll0);

        // Output `Gclk0` on pin PB14
        let (_gclk_out0, gclk0) =
            GclkOut::enable(tokens.gclk_io.gclk_out0, pins.pb14, gclk0, false);

        // Setup a peripheral channel to power up `Dpll1` from `Gclk1`
        let (pclk_dpll1, _gclk1) = Pclk::enable(tokens.pclks.dpll1, gclk1);

        // Configure `Dpll1` with `2 * 50 + 0/32 = 100 MHz` frequency
        let dpll1 = Dpll::from_pclk(tokens.dpll1, pclk_dpll1)
            .set_loop_div(50, 0)
            .enable()
            .unwrap_or_else(|_| panic!("Dpll1 failed to pass assertions"));

        // Output `Dpll1` on PB20 pin via `Gclk6`, divided by 200 resulting in 0.5 MHz
        // output frequency
        let (gclk6, _dpll1) = Gclk::new(tokens.gclks.gclk6, dpll1);
        let gclk6 = gclk6.div(GclkDiv::Div(200)).enable();
        let (_gclk_out6, _gclk6) =
            GclkOut::enable(tokens.gclk_io.gclk_out6, pins.pb12, gclk6, false);

        // Configure `Xosc32k` with both outputs (1kHz, 32kHz) activated
        let xosc_base = XoscBase::from_crystal(tokens.xosc32k.base, pins.pa00, pins.pa01)
            .control_gain_mode(ControlGainMode::HighSpeed)
            .on_demand(false)
            .run_standby(true)
            .enable();
        let (xosc1k, xosc_base) = Xosc1k::enable(tokens.xosc32k.xosc1k, xosc_base);
        let (xosc32k, _xosc_base) = Xosc32k::enable(tokens.xosc32k.xosc32k, xosc_base);

        // Output `Xosc32k` on PB16 pin via `Gclk2`, divided by 2 resulting in 16 kHz
        // output frequency
        let (gclk2, _xosc32k) = Gclk::new(tokens.gclks.gclk2, xosc32k);
        let gclk2 = gclk2.div(GclkDiv::Div(2)).enable();
        let (_gclk_out2, _gclk2) =
            GclkOut::enable(tokens.gclk_io.gclk_out2, pins.pb16, gclk2, false);

        // Output `OscUlp32k` on PB11 pin via `Gclk5`, without any division resulting in
        // 32 kHz output frequency
        let (osculp32k, _osculp_base) =
            OscUlp32k::enable(tokens.osculp.osculp32k, clocks.osculp_base);
        let (gclk5, _osculp32k) = Gclk::new(tokens.gclks.gclk5, osculp32k);
        let gclk5 = gclk5.enable();
        let (_gclk_out5, _gclk5) =
            GclkOut::enable(tokens.gclk_io.gclk_out5, pins.pb11, gclk5, false);

        // Setup a peripheral channel to power up `Uart` from `Gclk0`
        let (pclk_sercom0, _gclk0) = Pclk::enable(tokens.pclks.sercom0, gclk0);

        use atsamd_hal::sercom::v2::uart;

        let pads = uart::Pads::default().rx(pins.pa05).tx(pins.pa04);
        // In the future, the `Uart` will take ownership of the `Pclk` and will
        // take an `ApbClk` instead of `&MCLK`
        let mut uart = uart::Config::new(&mclk, device.SERCOM0, pads, pclk_sercom0.freq())
            .baud(115_200.hz(), BaudMode::Arithmetic(Oversampling::Bits16))
            .enable();
        uart.enable_interrupts(Flags::RXC);

        // Initialize the RTC oscillator from the 1 kHz output of XOSC32K
        let (rtc_osc, _xosc1k) = RtcOsc::enable(tokens.rtcosc, xosc1k);

        // Setup an `Rtc` in `ClockMode`
        // In the future, the `Rtc` will take ownership of the `RtcOsc`
        let rtc = Rtc::clock_mode(device.RTC, rtc_osc.freq(), &mut mclk);

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
