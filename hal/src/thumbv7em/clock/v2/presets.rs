/// This macro provides an opinionated clocking setup that consists of
///
/// a following chain of configured components:
///
/// - [`Dfll<OpenLoop>`][`super::dfll::Dfll`] (`48 MHz`)
/// - [`Gclk5<marker::Dfll>`][`super::gclk::Gclk5`] (`48 MHz / 24 -> 2 MHz`)
/// - [`Pclk<marker::Dpll0, marker::Gclk5>`][`super::pclk::Pclk`]
/// - [`Dpll0<PclkDriven<marker::Dpll0, marker::Gclk5>>`][`super::dpll::Dpll0`] (`2 MHz * 60 ->
///   120 MHz`)
/// - [`Gclk0<marker::Dpll0>`][`super::gclk::Gclk0`] (`120 MHz`)
///
/// Input arguments for this macro can be retrieved by calling
/// [`retrieve_clocks`][`crate::clock::v2::retrieve_clocks`] function.
///
/// Example of usage:
///
/// ```no_run
/// let mut pac = atsamd_hal::pac::Peripherals::take().unwrap();
///
/// let (gclk0, dfll, _, tokens) = atsamd_hal::clock::v2::retrieve_clocks(
///     pac.OSCCTRL,
///     pac.OSC32KCTRL,
///     pac.GCLK,
///     pac.MCLK,
///     &mut pac.NVMCTRL,
/// );
///
/// let _ = atsamd_hal::clocking_preset_gclk0_120mhz_gclk5_2mhz!(gclk0, dfll, tokens);
/// ```
#[macro_export]
macro_rules! clocking_preset_gclk0_120mhz_gclk5_2mhz {
    (
        $gclk0:expr,
        $dfll:expr,
        $tokens:expr
    ) => {{
        use atsamd_hal::clock::v2::*;
        let (gclk5, dfll) = gclk::Gclk::new($tokens.gclks.gclk5, $dfll);
        let gclk5 = gclk5.div(gclk::GclkDiv::Div(24)).enable();
        let (pclk_dpll0, gclk5) = pclk::Pclk::enable($tokens.pclks.dpll0, gclk5);
        let dpll0 = dpll::Dpll0::from_pclk($tokens.dpll0, pclk_dpll0)
            .set_loop_div(60, 0)
            .enable();
        let (gclk0, dfll, dpll0) = $gclk0.swap(dfll, dpll0);
        (gclk0, gclk5, dpll0, dfll)
    }};
}

/// This macro provides an opinionated clocking setup that consists of
/// a following chain of configured components:
///
/// - [`Dfll<OpenLoop>`][`super::dfll::Dfll`] (`48 MHz`)
/// - [`Gclk5<marker::Dfll>`][`super::gclk::Gclk5`] (`48 MHz / 24 -> 2 MHz`)
/// - [`Pclk<marker::Dpll0, marker::Gclk5>`][`super::pclk::Pclk`]
/// - [`Dpll0<PclkDriven<marker::Dpll0, marker::Gclk5>>`][`super::dpll::Dpll0`] (`2 MHz * 60 ->
///   120 MHz`)
/// - [`Gclk0<marker::Dpll0>`][`super::gclk::Gclk0`] (`120 MHz`)
///
/// with configured external 32k crystal oscillator:
///
/// - [`Xosc32k<Xosc32kMode>`][`super::xosc32k::Xosc32k`] (`32 kHz`)
/// - [`Gclk1<marker::Xosc32k>`][`super::gclk::Gclk1`] (`32 kHz`)
///
/// Input arguments for this macro can be retrieved by calling
/// [`retrieve_clocks`][`crate::clock::v2::retrieve_clocks`] function.
///
/// Example of usage:
///
/// ```no_run
/// let mut pac = atsamd_hal::pac::Peripherals::take().unwrap();
/// let pins = atsamd_hal::gpio::v2::Pins::new(pac.PORT);
///
/// let (gclk0, dfll, _, tokens) = atsamd_hal::clock::v2::retrieve_clocks(
///     pac.OSCCTRL,
///     pac.OSC32KCTRL,
///     pac.GCLK,
///     pac.MCLK,
///     &mut pac.NVMCTRL,
/// );
///
/// let _ = atsamd_hal::clocking_preset_gclk0_120mhz_gclk5_2mhz_gclk1_external_32khz!(
///     gclk0, dfll, pins.pa00, pins.pa01, tokens
/// );
/// ```
#[macro_export]
macro_rules! clocking_preset_gclk0_120mhz_gclk5_2mhz_gclk1_external_32khz {
    (
        $gclk0:expr,
        $dfll:expr,
        $xosc32k_in:expr,
        $xosc32k_out:expr,
        $tokens:expr
    ) => {{
        use atsamd_hal::clock::v2::*;
        let (gclk0, gclk5, dpll0, dfll) =
            atsamd_hal::clocking_preset_gclk0_120mhz_gclk5_2mhz!($gclk0, $dfll, $tokens);
        let xosc32k = xosc32k::Xosc32k::from_crystal($tokens.xosc32k, $xosc32k_in, $xosc32k_out)
            .enable()
            .activate_32k()
            .activate_1k();
        let (gclk1, xosc32k) = gclk::Gclk::new($tokens.gclks.gclk1, xosc32k);
        (gclk0, gclk5, gclk1, xosc32k, dpll0, dfll)
    }};
}

/// This macro provides an opinionated clocking setup that consists of
/// a following chain of configured components:
///
/// - [`Dfll<OpenLoop>`][`super::dfll::Dfll`] (`48 MHz`)
/// - [`Gclk5<marker::Dfll>`][`super::gclk::Gclk5`] (`48 MHz / 24 -> 2 MHz`)
/// - [`Pclk<marker::Dpll0, marker::Gclk5>`][`super::pclk::Pclk`]
/// - [`Dpll0<PclkDriven<marker::Dpll0, marker::Gclk5>>`][`super::dpll::Dpll0`] (`2 MHz * 60 ->
///   120 MHz`)
/// - [`Gclk0<marker::Dpll0>`][`super::gclk::Gclk0`] (`120 MHz`)
///
/// with internal, ultra low power 32k oscillator:
///
/// - [`OscUlp32k`][`super::osculp32k::OscUlp32k`] (`32 kHz`)
/// - [`Gclk1<marker::OscUlp32k>`][`super::gclk::Gclk1`] (`32 kHz`)
///
/// Input arguments for this macro can be retrieved by calling
/// [`retrieve_clocks`][`crate::clock::v2::retrieve_clocks`] function.
///
/// Example of usage:
///
/// ```no_run
/// let mut pac = atsamd_hal::pac::Peripherals::take().unwrap();
///
/// let (gclk0, dfll, osculp32k, tokens) = atsamd_hal::clock::v2::retrieve_clocks(
///     pac.OSCCTRL,
///     pac.OSC32KCTRL,
///     pac.GCLK,
///     pac.MCLK,
///     &mut pac.NVMCTRL,
/// );
///
/// let _ = atsamd_hal::clocking_preset_gclk0_120mhz_gclk5_2mhz_gclk1_internal_32khz!(
///     gclk0, dfll, osculp32k, tokens
/// );
/// ```
#[macro_export]
macro_rules! clocking_preset_gclk0_120mhz_gclk5_2mhz_gclk1_internal_32khz {
    (
        $gclk0:expr,
        $dfll:expr,
        $osculp32k:expr,
        $tokens:expr
    ) => {{
        use atsamd_hal::clock::v2::*;
        let (gclk0, gclk5, dpll0, dfll) =
            atsamd_hal::clocking_preset_gclk0_120mhz_gclk5_2mhz!($gclk0, $dfll, $tokens);
        let (gclk1, osculp32k) = gclk::Gclk::new($tokens.gclks.gclk1, $osculp32k);
        (gclk0, gclk5, gclk1, osculp32k, dpll0, dfll)
    }};
}
