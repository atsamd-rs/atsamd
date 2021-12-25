//! This module is intentionally private. Its contents are publicly exported
//! from the `v2` module, which is where the corresponding documentation will
//! appear.

use typenum::U1;

use crate::pac::{GCLK, MCLK, NVMCTRL, OSC32KCTRL, OSCCTRL};

use super::super::*;

/// Enabled clocks at power-on reset
///
/// This type is constructed using the [`reset_clocks_tokens`] function, which
/// consumes the PAC-level clocking structs and returns the HAL-level clocking
/// structs in their reset state.
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
pub struct ResetClocks {
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
/// This type is constructed using the [`reset_clocks_tokens`] function, which
/// consumes the PAC-level clocking structs and returns the HAL-level clocking
/// structs in their reset state.
///
/// As described in the [top-level](super::super) documentation for the `clock`
/// module, token types are used to guanrantee the uniqueness of each clock. To
/// configure or enable a clock, you must provide the corresponding token.
///
/// For example, to enable the peripheral channel clock for
/// [`Sercom1`](crate::sercom::v2::Sercom1), you must provide the corresponding
/// [`PclkToken`](pclk::PclkToken).
///
/// ```
/// let (clocks, tokens) = reset_clocks_tokens(oscctrl, osc32kctrl, gclk, mclk, nvmctrl);
/// let pclk_sercom1 = Pclk::enable(tokens.pclks.sercom1, clocks.gclk0);
/// ```
pub struct ResetTokens {
    /// Wrapper for low-level PAC -- can be unsafely stolen if needed
    pub pac: PacStructs,
    /// Synchronous clocking domain clocks -- AHB bus
    pub ahbs: ahb::AhbClks,
    /// Synchronous clocking domain clocks -- APB buses
    pub apbs: apb::ApbClks,
    /// Construction token for [`dpll::Dpll0`]
    pub dpll0: dpll::DpllToken<dpll::DpllId0>,
    /// Construction token for [`dpll::Dpll1`]
    pub dpll1: dpll::DpllToken<dpll::DpllId1>,
    /// Construction tokens for [`gclkio::GclkIo`]
    pub gclk_io: gclkio::Tokens,
    /// Construction tokens for [`gclk::Gclk`]
    pub gclks: gclk::Tokens,
    /// Construction tokens for [`pclk::Pclk`]
    pub pclks: pclk::Tokens,
    /// Construction token for [`rtc::RtcOsc`]
    pub rtcosc: rtcosc::RtcOscToken,
    /// Construction token for [`xosc::Xosc0`]
    pub xosc0: xosc::XoscToken<xosc::XoscId0>,
    /// Construction token for [`xosc::Xosc1`]
    pub xosc1: xosc::XoscToken<xosc::XoscId1>,
    /// Construction tokens for [`xosc32k::XoscBase`], [`xosc32k::Xosc1k`] and [`xosc32k::Xosc32k`]
    pub xosc32k: xosc32k::Tokens,
    /// Construction tokens for [`osculp32k::OscUlp1k`] and [`osculp32k::OscUlp32k`]
    pub osculp: osculp32k::Tokens,
}

/// Consumes the PAC-level clocking structs and return a HAL-level
/// representation of the clocks at power-on reset
///
/// This function consumes the [`OSCCTRL`], [`OSC32KCTRL`], [`GCLK`] and
/// [`MCLK`] PAC structs and returns the [`ResetClocks`] and [`ResetTokens`].
/// The `ResetClock` represent the set of [`Enabled`] clocks at reset, while the
/// `ResetTokens` can be used to configure and enable all of the remaining
/// clocks.
///
/// For example, the following code demonstrates a number of common operations.
/// First, the `Reset` types are created from the PAC structs. Next, the GCLK5
/// token is used to create an instance of [`Gclk5`], sourced from the already
/// running [`Dfll`](dfll::Dfll). The [`GclkDivider`](gclk::GclkDivider) is set
/// to 24, and `Gclk5` is [`Enabled`] with a 2 MHz output frequency. Next,
/// `Gclk5` is used as the [`Pclk`](pclk::Pclk) [`Source`] for
/// [`Dpll0`](dpll::Dpll0). Once the peripheral channel clock has been enabled,
/// the `Dpll0` itself can be created from it. The loop divider is set to 60,
/// which raises the output frequency to 120 MHz. Finally, the main clock,
/// [`Gclk0`](gclk::Gclk0) is swapped to use `Dpll0` instead of the `Dfll`.
///
/// ```
/// let (clocks, tokens) = reset_clocks_tokens(oscctrl, osc32kctrl, gclk, mclk, nvmctrl);
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
pub fn reset_clocks_tokens(
    oscctrl: OSCCTRL,
    osc32kctrl: OSC32KCTRL,
    gclk: GCLK,
    mclk: MCLK,
    nvmctrl: &mut NVMCTRL,
) -> (ResetClocks, ResetTokens) {
    // Safe because registers are instantiated only once
    unsafe {
        let tokens = ResetTokens {
            pac: PacStructs {
                oscctrl,
                osc32kctrl,
                gclk,
                mclk,
            },
            ahbs: ahb::AhbClks::new(),
            apbs: apb::ApbClks::new(),
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
        let dfll: Enabled<_> = Enabled::new(dfll::Dfll::in_open_mode(dfll::DfllToken::new()));
        let (gclk0, dfll) = gclk::Gclk0::new(gclk::GclkToken::new(), dfll);
        let gclk0 = Enabled::new(gclk0);
        let osculp_base = osculp32k::OscUlpBase::new();
        let clocks = ResetClocks {
            gclk0,
            dfll,
            osculp_base,
        };
        (clocks, tokens)
    }
}
