//! Clocking preset #3
//!
//! The [`reset_clocks_tokens`] function returns a representation of the clock
//! system in its reset state. As a alternative, the HAL also provides several
//! preset clocking configurations, like this one.
//!
//! The [`preset3_clocks_tokens`] function acts similarly to
//! `reset_clocks_tokens`. It consumes the PAC structs and returns the
//! [`Preset3Clocks`] and [`Preset3Tokens`] structs, which represent the
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
//! └── OSCULP32K 32 kHz output
//!     └── GCLK1 (32 kHz)
//!
//! ```
//!
//! Note that the `Preset3Clocks` struct contains more [`Enabled`] clocks than
//! found in [`ResetClocks`]. Consequently, the `Preset3Tokens` struct does not
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
pub struct Preset3Clocks {
    /// DFLL48 in open loop mode
    pub dfll: Enabled<dfll::Dfll<dfll::OpenLoop>, U1>,
    /// GCLK5 driven by the DFLL. Divides by 24 for 2 MHz output
    pub gclk5: Enabled<gclk::Gclk5<dfll::DfllId>, U1>,
    /// DPLL0 driven by GCLK5. Multiplies by 60 for 120 MHz output
    pub dpll0: Enabled<dpll::Dpll0<gclk::GclkId5>, U1>,
    /// Main system clock driven by DPLL0 at 120 MHz
    pub gclk0: Enabled<gclk::Gclk0<dpll::DpllId0>, U1>,
    /// Always-enabled base oscillator for the [`OscUlp1k`](osculp32k::OscUlp1k)
    /// and [`OscUlp32k`](osculp32k::OscUlp32k) clocks.
    pub osculp_base: Enabled<osculp32k::OscUlpBase, U1>,
    /// 32 kHz output from the OSCULP32K
    pub osculp32k: Enabled<osculp32k::OscUlp32k, U1>,
    /// GCLK1 driven by OSCULP32K at 32 kHz
    pub gclk1: Enabled<gclk::Gclk1<osculp32k::OscUlp32kId>>,
}

//==============================================================================
// GclkTokens
//==============================================================================

/// Contains all of the [`GclkToken`]s except the one for [`Gclk5`](gclk::Gclk5)
#[allow(missing_docs)]
pub struct Preset3GclkTokens {
    pub gclk2: GclkToken<GclkId2>,
    pub gclk3: GclkToken<GclkId3>,
    pub gclk4: GclkToken<GclkId4>,
    pub gclk6: GclkToken<GclkId6>,
    pub gclk7: GclkToken<GclkId7>,
    pub gclk8: GclkToken<GclkId8>,
    pub gclk9: GclkToken<GclkId9>,
    pub gclk10: GclkToken<GclkId10>,
    pub gclk11: GclkToken<GclkId11>,
}

//==============================================================================
// PclkTokens
//==============================================================================

super::define_filtered_pclk_token_struct!(
    #[doc = "Remaining [`PclkToken`]s for this preset clock tree

    This struct contains all tokens found in [`pclk::Tokens`] except the one
    for DPLL0."]
    Preset3PclkTokens,
    dpll0
);

//==============================================================================
// Tokens
//==============================================================================

/// Contains everythin in [`DefaultTokens`] except what is used to configure the
/// [`Preset1Clocks`]
pub struct Preset3Tokens {
    /// Wrapper for low-level PAC -- can be unsafely stolen if needed
    pub pac: PacStructs,
    /// Synchronous clocking domain clocks -- AHB bus
    pub ahbs: ahb::AhbClks,
    /// Synchronous clocking domain clocks -- APB buses
    pub apbs: apb::ApbClks,
    /// Construction token for [`dpll::Dpll1`]
    pub dpll1: dpll::DpllToken<dpll::DpllId1>,
    /// Construction tokens for [`gclkio::GclkIo`]
    pub gclk_io: gclkio::Tokens,
    /// Construction tokens for [`gclk::Gclk`]
    pub gclks: Preset3GclkTokens,
    /// Construction tokens for [`pclk::Pclk`]
    pub pclks: Preset3PclkTokens,
    /// Construction token for [`rtc::RtcOsc`]
    pub rtcosc: rtcosc::RtcOscToken,
    /// Construction token for [`xosc::Xosc0`]
    pub xosc0: xosc::XoscToken<xosc::XoscId0>,
    /// Construction token for [`xosc::Xosc1`]
    pub xosc1: xosc::XoscToken<xosc::XoscId1>,
    /// Construction tokens for [`osculp32k::OscUlp1k`]
    pub osculp1k: osculp32k::OscUlp1kToken,
}

//==============================================================================
// Setup
//==============================================================================

/// Consume the [PAC](crate::pac) clock structs and return configured clocks and
/// clock tokens
///
/// The clocks are configured according to the [module-level](self)
/// documentation.
pub fn preset3_clocks_tokens(
    oscctrl: OSCCTRL,
    osc32kctrl: OSC32KCTRL,
    gclk: GCLK,
    mclk: MCLK,
    nvmctrl: &mut NVMCTRL,
) -> (Preset3Clocks, Preset3Tokens) {
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
    let gclks = Preset3GclkTokens {
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
    let (osculp32k, osculp_base) =
        osculp32k::OscUlp32k::enable(tokens.osculp.osculp32k, osculp_base);
    let (gclk1, osculp32k) = gclk::Gclk::new(gclk1, osculp32k);
    let gclk1 = gclk1.enable();
    let clocks = Preset3Clocks {
        gclk0,
        gclk1,
        gclk5,
        dpll0,
        dfll,
        osculp_base,
        osculp32k,
    };
    let tokens = Preset3Tokens {
        pac: tokens.pac,
        ahbs: tokens.ahbs,
        apbs: tokens.apbs,
        dpll1: tokens.dpll1,
        gclk_io: tokens.gclk_io,
        gclks,
        // SAFETY: Only the DPLL0 token was used, the rest will be dropped and
        // recreated here
        pclks: unsafe { Preset3PclkTokens::new() },
        rtcosc: tokens.rtcosc,
        xosc0: tokens.xosc0,
        xosc1: tokens.xosc1,
        osculp1k: tokens.osculp.osculp1k,
    };
    (clocks, tokens)
}
