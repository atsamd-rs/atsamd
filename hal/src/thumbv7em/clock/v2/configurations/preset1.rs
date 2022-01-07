//! Clocking preset #1
//!
//! The [`reset_clocks_tokens`] function returns a representation of the clock
//! system in its reset state. As a alternative, the HAL also provides several
//! preset clocking configurations, like this one.
//!
//! The [`preset1_clocks_tokens`] function acts similarly to
//! `reset_clocks_tokens`. It consumes the PAC structs and returns the
//! [`Preset1Clocks`] and [`Preset1Tokens`] structs, which represent the
//! following clock configuration:
//!
//!
//! ```text
//! DFLL (48 MHz)
//! └── GCLK5 (2 MHz)
//!     └── DPLL (120 MHz)
//!         └── GCLK0 (120 MHz)
//!
//! OSCULP32K base oscillator
//! ```
//!
//! Note that the `Preset1Clocks` struct contains more [`Enabled`] clocks than
//! found in [`ResetClocks`]. Consequently, the `Preset1Tokens` struct does not
//! contain the corresponding tokens used to create and configure those clocks.

use typenum::U1;

use crate::pac::{GCLK, MCLK, NVMCTRL, OSC32KCTRL, OSCCTRL};

use super::super::*;
use gclk::GclkToken;
use pclk::{ids::*, PclkToken};

//==============================================================================
// Clocks
//==============================================================================

/// Configured clocks for this preset clock tree
///
/// See the [module-level](self) documentation for more details
pub struct Preset1Clocks {
    /// DFLL48 in open loop mode
    pub dfll: Enabled<dfll::Dfll<dfll::OpenLoop>, U1>,
    /// GCLK5 driven by the DFLL. Divides by 24 for 2 MHz output
    pub gclk5: Enabled<gclk::Gclk5<dfll::DfllId>, U1>,
    /// DPLL0 driven by GCLK5. Multiplies by 60 for 120 MHz output
    pub dpll0: Enabled<dpll::Dpll0<gclk::Gclk5Id>, U1>,
    /// Main system clock driven by DPLL0 at 120 MHz
    pub gclk0: Enabled<gclk::Gclk0<dpll::Dpll0Id>, U1>,
    /// Always-enabled base oscillator for the [`OscUlp1k`](osculp32k::OscUlp1k)
    /// and [`OscUlp32k`](osculp32k::OscUlp32k) clocks.
    pub osculp_base: Enabled<osculp32k::OscUlpBase>,
}

//==============================================================================
// GclkTokens
//==============================================================================

/// Contains all of the [`GclkToken`]s except the one for [`Gclk5`](gclk::Gclk5)
#[allow(missing_docs)]
pub struct Preset1GclkTokens {
    pub gclk1: GclkToken<Gclk1Id>,
    pub gclk2: GclkToken<Gclk2Id>,
    pub gclk3: GclkToken<Gclk3Id>,
    pub gclk4: GclkToken<Gclk4Id>,
    pub gclk6: GclkToken<Gclk6Id>,
    pub gclk7: GclkToken<Gclk7Id>,
    pub gclk8: GclkToken<Gclk8Id>,
    pub gclk9: GclkToken<Gclk9Id>,
    pub gclk10: GclkToken<Gclk10Id>,
    pub gclk11: GclkToken<Gclk11Id>,
}

//==============================================================================
// PclkTokens
//==============================================================================

super::define_filtered_pclk_token_struct!(
    #[doc = "Remaining [`PclkToken`]s for this preset clock tree

    This struct contains all tokens found in [`pclk::Tokens`] except the one
    for DPLL0."]
    Preset1PclkTokens,
    dpll0
);

//==============================================================================
// Tokens
//==============================================================================

/// Contains everythin in [`ResetTokens`] except what is used to configure the
/// [`Preset1Clocks`]
pub struct Preset1Tokens {
    /// Wrapper for low-level PAC -- can be unsafely stolen if needed
    pub pac: PacStructs,
    /// Synchronous clocking domain clocks -- AHB bus
    pub ahbs: ahb::AhbClks,
    /// Synchronous clocking domain clocks -- APB buses
    pub apbs: apb::ApbClks,
    /// Construction token for [`dpll::Dpll1`]
    pub dpll1: dpll::DpllToken<dpll::Dpll1Id>,
    /// Construction tokens for [`gclkio::GclkIo`]
    pub gclk_io: gclkio::Tokens,
    /// Construction tokens for [`gclk::Gclk`]
    pub gclks: Preset1GclkTokens,
    /// Construction tokens for [`pclk::Pclk`]
    pub pclks: Preset1PclkTokens,
    /// Construction token for [`rtc::RtcOsc`]
    pub rtcosc: rtcosc::RtcOscToken,
    /// Construction token for [`xosc::Xosc0`]
    pub xosc0: xosc::XoscToken<xosc::Xosc0Id>,
    /// Construction token for [`xosc::Xosc1`]
    pub xosc1: xosc::XoscToken<xosc::Xosc1Id>,
    /// Construction tokens for [`xosc32k::XoscBase`], [`xosc32k::Xosc1k`] and [`xosc32k::Xosc32k`]
    pub xosc32k: xosc32k::Tokens,
    /// Construction tokens for [`osculp32k::OscUlp1k`] and [`osculp32k::OscUlp32k`]
    pub osculp: osculp32k::Tokens,
}

//==============================================================================
// Setup
//==============================================================================

/// Consume the [PAC](crate::pac) clock structs and return configured clocks and
/// clock tokens
///
/// The clocks are configured according to the [module-level](self)
/// documentation.
pub fn preset1_clocks_tokens(
    oscctrl: OSCCTRL,
    osc32kctrl: OSC32KCTRL,
    gclk: GCLK,
    mclk: MCLK,
    nvmctrl: &mut NVMCTRL,
) -> (Preset1Clocks, Preset1Tokens) {
    let (clocks, tokens) = reset_clocks_tokens(oscctrl, osc32kctrl, gclk, mclk, nvmctrl);
    let gclk::Tokens {
        gclk1,
        gclk2,
        gclk3,
        gclk4,
        gclk5,
        gclk6,
        gclk7,
        gclk8,
        gclk9,
        gclk10,
        gclk11,
    } = tokens.gclks;
    let gclks = Preset1GclkTokens {
        gclk1,
        gclk2,
        gclk3,
        gclk4,
        gclk6,
        gclk7,
        gclk8,
        gclk9,
        gclk10,
        gclk11,
    };
    let (gclk5, dfll) = gclk::Gclk::new(gclk5, clocks.dfll);
    let gclk5 = gclk5.div(gclk::GclkDiv::Div(24)).enable();
    let (pclk_dpll0, gclk5) = pclk::Pclk::enable(tokens.pclks.dpll0, gclk5);
    let dpll0 = dpll::Dpll0::from_pclk(tokens.dpll0, pclk_dpll0)
        .set_loop_div(60, 0)
        .enable()
        .unwrap_or_else(|_| panic!("Dpll did not pass assertion checks!"));
    let (gclk0, dfll, dpll0) = clocks.gclk0.swap(dfll, dpll0);
    let osculp_base = clocks.osculp_base;
    let clocks = Preset1Clocks {
        gclk0,
        gclk5,
        dpll0,
        dfll,
        osculp_base,
    };
    let tokens = Preset1Tokens {
        pac: tokens.pac,
        ahbs: tokens.ahbs,
        apbs: tokens.apbs,
        dpll1: tokens.dpll1,
        gclk_io: tokens.gclk_io,
        gclks,
        // SAFETY: Only the DPLL0 token was used, the rest will be dropped and
        // recreated here
        pclks: Preset1PclkTokens::new(),
        rtcosc: tokens.rtcosc,
        xosc0: tokens.xosc0,
        xosc1: tokens.xosc1,
        xosc32k: tokens.xosc32k,
        osculp: tokens.osculp,
    };
    (clocks, tokens)
}
