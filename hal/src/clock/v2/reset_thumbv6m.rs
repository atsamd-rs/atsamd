//! This module is intentionally private. Its contents are publicly exported
//! from the `v2` module, which is where the corresponding documentation will
//! appear.

use typenum::U1;

use crate::pac::{Gclk, Pm, Sysctrl};

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
    gclk: Gclk,
    pm: Pm,
    sysctrl: Syctrl,
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
    pub unsafe fn steal(self) -> (Gclk, Pm, Syctrl) {
        (self.gclk, self.pm, self.sysctrl)
    }
}

/// Bus clock objects
///
/// This type is constructed using the [`clock_system_at_reset`] function, which
/// consumes the PAC-level clocking structs and returns the HAL-level clocking
/// structs in their reset state.
///
/// This type contains the [bus clocks](super#bus-clocks), which are a necessary
/// to implement memory safety for the [`AhbClk`]s and [`ApbClk`]s.
///
/// [`AhbClk`]: super::ahb::AhbClk
/// [`ApbClk`]: super::apb::ApbClk
pub struct Buses {
    pub ahb: ahb::Ahb,
    pub apb: apb::Apb,
}

pub struct OscUlp32kClocks {
    pub base: osculp32k::EnabledOscUlp32kBase,
    pub osculp1k: osculp32k::EnabledOscUlp1k,
    pub osculp32k: osculp32k::EnabledOscUlp32k<U1>,
}

/// Enabled clocks at power-on reset
///
/// This type is constructed using the [`clock_system_at_reset`] function, which
/// consumes the PAC-level clocking structs and returns the HAL-level clocking
/// structs in their reset state.
///
/// This type represents the clocks as they are configured at power-on reset.
/// The main clock, [`Gclk0`](gclk::Gclk0), runs at 48 MHz using the
/// [`Dfll`](dfll::Dfll) in open-loop mode. The ultra-low power
/// [base oscillator](osculp32k::OscUlp32kBase) is also enabled and running, as
/// it can never be disabled.
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
    /// Main system clock, driven at 1 MHz by the OSC8M divided by 8
    pub gclk0: Enabled<gclk::Gclk0<osc8m::Osc8mId>, U1>,
    /// GCLK2, driven at 32 kHz by the OSCULP
    pub gclk2: Enabled<gclk::Gclk2<osculp32k::OscUlp32kId>, U1>,
    /// [`Pclk`](pclk::Pclk) for the watchdog timer, sourced from [`Gclk2`](gclk::Gclk2)
    pub wdt: pclk::Pclk<types::Wdt, gclk::Gclk2Id>,
    /// Always-enabled OSCULP oscillators
    pub osculp32k: OscUlp32kClocks,
}

/// Type-level tokens for unused clocks at power-on reset
///
/// This type is constructed using the [`clock_system_at_reset`] function, which
/// consumes the PAC-level clocking structs and returns the HAL-level clocking
/// structs in their reset state.
///
/// As described in the [top-level](super::super) documentation for the `clock`
/// module, token types are used to guanrantee the uniqueness of each clock. To
/// configure or enable a clock, you must provide the corresponding token.
pub struct Tokens {
    /// Tokens to create [`apb::ApbClk`]s
    pub apbs: apb::ApbTokens,
    /// Token to create [`dfll::Dfll`]
    pub dfll: dfll::DfllToken,
    /// Token to create [`dpll::Dpll0`]
    pub dpll: dpll::DpllToken<dpll::Dpll0Id>,
    /// Tokens to create [`gclk::Gclk`]
    pub gclks: gclk::GclkTokens,
    /// Tokens to create [`pclk::Pclk`]s
    pub pclks: pclk::PclkTokens,
    /// Tokens [`xosc::Xosc0`]
    pub xosc: xosc::XoscToken<xosc::Xosc0Id>,
    /// Tokens to create [`xosc32k::Xosc32kBase`], [`xosc32k::Xosc1k`] and
    /// [`xosc32k::Xosc32k`]
    pub xosc32k: xosc32k::Xosc32kTokens,
}

/// Consume the PAC clocking structs and return a HAL-level
/// representation of the clocks at power-on reset
///
/// This function consumes the [`OSCCTRL`], [`OSC32KCTRL`], [`Gclk`] and
/// [`MCLK`] PAC structs and returns the [`Buses`], [`Clocks`] and [`Tokens`].
///
/// See the [module-level documentation](super) for more details.
#[inline]
pub fn clock_system_at_reset(gclk: Gclk, pm: Pm, sysctrl: Syctrl) -> (Buses, Clocks, Tokens) {
    // Safety: No bus, clock or token is instantiated more than once
    unsafe {
        let buses = Buses {
            ahb: ahb::Ahb::new(),
            apb: apb::Apb::new(),
        };
        let pac = Pac { gclk, pm, sysctrl };
        let osc8m = Enabled::new(osc8m::Osc8m::new(osc8m::Osc8mToken::new()));
        let (gclk0, osc8m) = gclk::Gclk0::from_source(gclk::GclkToken::new(), osc8m);
        let gclk0 = Enabled::new(gclk0);
        let base = osculp32k::OscUlp32kBase::new();
        let osculp1k = Enabled::new(osculp32k::OscUlp1k::new());
        let osculp32k = Enabled::new(osculp32k::OscUlp32k::new());
        let (gclk2, osculp32k) = gclk::Gclk2::from_source(gclk::GclkToken::new(), osculp32k);
        let gclk2 = Enabled::new(gclk2);
        let wdt = pclk::Pclk::new(pclk::PclkToken::new(), gclk2.freq());
        let osculp32k = OscUlp32kClocks {
            base,
            osculp1k,
            osculp32k,
        };
        let clocks = Clocks {
            pac,
            ahbs: ahb::AhbClks::new(),
            apbs: apb::ApbClks::new(),
            gclk0,
            gclk2,
            wdt,
            osculp32k,
        };
        let tokens = Tokens {
            apbs: apb::ApbTokens::new(),
            dfll: dfll::DfllToken::new(),
            dpll: dpll::DpllToken::new(),
            gclks: gclk::GclkTokens::new(),
            pclks: pclk::PclkTokens::new(),
            xosc: xosc::XoscToken::new(),
            xosc32k: xosc32k::Xosc32kTokens::new(),
        };
        (buses, clocks, tokens)
    }
}
