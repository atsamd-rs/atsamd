//! This module is intentionally private. Its contents are publicly exported
//! from the `v2` module, which is where the corresponding documentation will
//! appear.

use typenum::U1;

use crate::pac::{GCLK, MCLK, NVMCTRL, OSC32KCTRL, OSCCTRL};

use super::*;

/// Collection of low-level PAC structs
///
/// This struct serves to guard access to the low-level PAC structs. It places
/// them behind an `unsafe` barrier.
///
/// Normally, users trade the low-level PAC structs for the higher-level
/// `clock::v2` API. However, in some cases, the `clock::v2` API may not be
/// sufficient. In these cases, users can access the registers directly by
/// calling [`Pac::steal`] to recover the PAC structs.
pub struct Pac {
    oscctrl: OSCCTRL,
    osc32kctrl: OSC32KCTRL,
    gclk: GCLK,
    mclk: MCLK,
}

impl Pac {
    /// Escape hatch allowing to access low-level PAC structs
    ///
    /// Consume the [`Pac`] and return the low-level PAC structs. This is
    /// useful when the `clock::v2` API does not provide a necessary feature, or
    /// when dealing with the legacy `clock::v1` API. For example, many of the
    /// `clock::v1` functions require access to the [`MCLK`] peripheral.
    ///
    /// # Safety
    ///
    /// Directly configuring clocks through the PAC API can invalidate the
    /// type-level guarantees of the `clock` module API.
    pub unsafe fn steal(self) -> (OSCCTRL, OSC32KCTRL, GCLK, MCLK) {
        (self.oscctrl, self.osc32kctrl, self.gclk, self.mclk)
    }
}

pub struct Buses {
    pub ahb: ahb::Ahb,
    pub apb: apb::Apb,
}

/// Enabled clocks at power-on reset
///
/// This type is constructed using the [`por_state`] function, which consumes
/// the PAC-level clocking structs and returns the HAL-level clocking structs in
/// their reset state.
///
/// This type represents the clocks as they are configured at power-on reset.
/// The main clock, [`Gclk0`](gclk::Gclk0), runs at 48 MHz using the
/// [`Dfll`](dfll::Dfll) in [`OpenLoop`](dfll::OpenLoop) [`Mode`](dfll::Mode).
/// The ultra-low power [base oscillator](osculp32k::OscUlpBase) is also enabled
/// and running, as it can never be disabled.
///
/// As described in the [top-level](super::super) documentation for the `clock`
/// module, only [`Enabled`] clocks can be used as a [`Source`] for downstream
/// clocks. This struct contains all of the `Enabled` clocks at reset.
///
/// This struct also contains the [`Pac`] wrapper struct, which provides
/// `unsafe` access to the low-level PAC structs.
pub struct Clocks {
    /// Wrapper providing `unsafe` access to low-level PAC structs
    pub pac: Pac,
    /// Enabled AHB clocks
    pub ahbs: ahb::AhbClks,
    /// Enabled APB clocks
    pub apbs: apb::ApbClks,
    /// Main system clock, driven at 48 MHz by the DFLL in open loop mode
    pub gclk0: Enabled<gclk::Gclk0<dfll::DfllId>, U1>,
    /// DFLL48 in open loop mode
    pub dfll: Enabled<dfll::Dfll<dfll::OpenLoop>, U1>,
    /// Always-enabled base oscillator for the [`OscUlp1k`](osculp32k::OscUlp1k)
    /// and [`OscUlp32k`](osculp32k::OscUlp32k) clocks.
    pub osculp_base: Enabled<osculp32k::OscUlpBase>,
}

/// Type-level tokens for unused clocks at power-on reset
///
/// This type is constructed using the [`por_state`] function, which consumes
/// the PAC-level clocking structs and returns the HAL-level clocking structs in
/// their reset state.
///
/// As described in the [top-level](super::super) documentation for the `clock`
/// module, token types are used to guanrantee the uniqueness of each clock. To
/// configure or enable a clock, you must provide the corresponding token.
///
/// For example, to enable the peripheral channel clock for [`Sercom1`], you
/// must provide the corresponding [`PclkToken`](pclk::PclkToken).
///
/// ```
/// use atsamd_hal::thumbv7em::clock::v2::{self as clock, pclk::Pclk};
///
/// let (buses, clocks, tokens) = clock::por_state(oscctrl, osc32kctrl, gclk, mclk, nvmctrl);
/// let pclk_sercom1 = Pclk::enable(tokens.pclks.sercom1, clocks.gclk0);
/// ```
///
/// [`Sercom1`]: crate::sercom::v2::Sercom1
pub struct Tokens {
    /// Tokens to create [`apb::ApbClk`]s
    pub apbs: apb::ApbTokens,
    /// Token to create [`dpll::Dpll0`]
    pub dpll0: dpll::DpllToken<dpll::Dpll0Id>,
    /// Token to create [`dpll::Dpll1`]
    pub dpll1: dpll::DpllToken<dpll::Dpll1Id>,
    /// Tokens to create [`gclkio::GclkIo`]s
    pub gclk_io: gclkio::Tokens,
    /// Tokens to create [`gclk::Gclk`]
    pub gclks: gclk::Tokens,
    /// Tokens to create [`pclk::Pclk`]s
    pub pclks: pclk::Tokens,
    /// Tokens to create [`rtcosc::RtcOsc`]
    pub rtcosc: rtcosc::RtcOscToken,
    /// Tokens [`xosc::Xosc0`]
    pub xosc0: xosc::XoscToken<xosc::Xosc0Id>,
    /// Token to create [`xosc::Xosc1`]
    pub xosc1: xosc::XoscToken<xosc::Xosc1Id>,
    /// Tokens to create [`xosc32k::XoscBase`], [`xosc32k::Xosc1k`] and
    /// [`xosc32k::Xosc32k`]
    pub xosc32k: xosc32k::Tokens,
    /// Tokens to create [`osculp32k::OscUlp1k`] and [`osculp32k::OscUlp32k`]
    pub osculp: osculp32k::Tokens,
}

/// Consume the PAC clocking structs and return a HAL-level
/// representation of the clocks at power-on reset
///
/// This function consumes the [`OSCCTRL`], [`OSC32KCTRL`], [`GCLK`] and
/// [`MCLK`] PAC structs and returns the [`Buses`], [`Clocks`] and [`Tokens`].
/// The `Buses` provide access to enable or disable the AHB and APB bus clocks.
/// The `Clocks` represent the set of [`Enabled`] clocks at reset. And the
/// `Tokens` can be used to configure and enable the remaining clocks.
///
/// For example, the following code demonstrates a number of common operations.
/// First, the PAC structs are traded for HAL types. Next, the GCLK5 token is
/// used to create an instance of [`Gclk5`], sourced from the already running
/// [`Dfll`](dfll::Dfll). The [`GclkDivider`](gclk::GclkDivider) is set to 24,
/// and `Gclk5` is [`Enabled`] with a 2 MHz output frequency. Next, `Gclk5` is
/// used as the [`Pclk`](pclk::Pclk) [`Source`] for [`Dpll0`](dpll::Dpll0). Once
/// the peripheral channel clock has been enabled, the `Dpll0` itself can be
/// created from it. The loop divider is set to 60, which raises the output
/// frequency to 120 MHz. Finally, the main clock, [`Gclk0`](gclk::Gclk0) is
/// swapped to use `Dpll0` instead of the `Dfll`.
///
/// ```
/// use atsamd_hal::thumbv7em::clock::v2::{self as clock, gclk, pclk, dpll};
///
/// let (_buses, clocks, tokens) = clock::por_state(oscctrl, osc32kctrl, gclk, mclk, nvmctrl);
/// let (gclk5, dfll) = gclk::Gclk::new(tokens.gclks.gclk5, clocks.dfll);
/// let gclk5 = gclk5.div(gclk::GclkDiv::Div(24)).enable();
/// let (pclk_dpll0, gclk5) = pclk::Pclk::enable(tokens.pclks.dpll0, gclk5);
/// let dpll0 = dpll::Dpll0::from_pclk(tokens.dpll0, pclk_dpll0)
///     .set_loop_div(60, 0)
///     .enable()
///     .unwrap_or_else(|_| panic!("Dpll did not pass assertion checks!"));
/// let (gclk0, dfll, dpll0) = clocks.gclk0.swap(dfll, dpll0);
/// ```
///
/// See the [top-level](super::super) documentation of the `clock` module for
/// more details.
#[inline]
pub fn por_state(
    oscctrl: OSCCTRL,
    osc32kctrl: OSC32KCTRL,
    gclk: GCLK,
    mclk: MCLK,
    nvmctrl: &mut NVMCTRL,
) -> (Buses, Clocks, Tokens) {
    // Safe because no bus, clock or token struct is instantiated more than once
    // We also know that the PAC structs cannot be obtained more than once
    // without `unsafe` code
    unsafe {
        let buses = Buses {
            ahb: ahb::Ahb::new(),
            apb: apb::Apb::new(),
        };
        let pac = Pac {
            oscctrl,
            osc32kctrl,
            gclk,
            mclk,
        };
        let dfll = Enabled::<_>::new(dfll::Dfll::in_open_mode(dfll::DfllToken::new()));
        let (gclk0, dfll) = gclk::Gclk0::new(gclk::GclkToken::new(), dfll);
        let gclk0 = Enabled::new(gclk0);
        let clocks = Clocks {
            pac,
            ahbs: ahb::AhbClks::new(),
            apbs: apb::ApbClks::new(),
            gclk0,
            dfll,
            osculp_base: osculp32k::OscUlpBase::new(),
        };
        let tokens = Tokens {
            apbs: apb::ApbTokens::new(),
            dpll0: dpll::DpllToken::new(),
            dpll1: dpll::DpllToken::new(),
            gclk_io: gclkio::Tokens::new(),
            gclks: gclk::Tokens::new(nvmctrl),
            pclks: pclk::Tokens::new(),
            rtcosc: rtcosc::RtcOscToken::new(),
            xosc0: xosc::XoscToken::new(),
            xosc1: xosc::XoscToken::new(),
            xosc32k: xosc32k::Tokens::new(),
            osculp: osculp32k::Tokens::new(),
        };
        (buses, clocks, tokens)
    }
}
