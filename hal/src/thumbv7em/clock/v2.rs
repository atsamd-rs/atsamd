//#![warn(missing_docs)]

//! # Clocking API v2
//!
//! This module provides the set of abstractions allowing a user to manipulate
//! an ATSAMD's clocking system in a safe and expressive manner.
//!
//! Foundation of an API is a [`retrieve_clocks`] function that returns a tuple
//! of instantiated clocking abstractions that are enabled on reset.
//!
//! To reconfigure a clocking tree, user can instantiate additional clocking
//! components or reconfigure existing ones and connect them. Type-safe nature
//! of the API prevents user from building an application with an unsound clock
//! tree setup.
//!
//! Example:
//! ```no_run
//! # use atsamd_hal::{
//! #     clock::v1,
//! #     clock::v2::{
//! #         retrieve_clocks,
//! #         apb::{ApbClk, ApbToken, Trng},
//! #         gclkio::{GclkIn, GclkOut},
//! #         xosc::Xosc,
//! #         gclk::{Gclk, Gclk1Div},
//! #         dpll::Dpll,
//! #         pclk::Pclk
//! #     },
//! #     gpio::v2::Pins,
//! #     pac::Peripherals,
//! #     time::U32Ext,
//! # };
//! let mut pac = Peripherals::take().unwrap();
//! let (gclk0, dfll, osculp32k, tokens) = retrieve_clocks(
//!     pac.OSCCTRL,
//!     pac.OSC32KCTRL,
//!     pac.GCLK,
//!     pac.MCLK,
//!     &mut pac.NVMCTRL,
//! );
//!
//! let pins = Pins::new(pac.PORT);
//!
//! // Asynchronous clocking domain
//! // Xosc0 (8 MHz) set up using pins PA14 and PA15
//! let xosc0 = Xosc::from_crystal(tokens.xosc0, pins.pa14, pins.pa15, 8.mhz()).enable();
//!
//! // Dfll (48 MHz) -> Gclk1 (48 MHz / 24) -> 2 MHz
//! let (gclk1, dfll) = Gclk::new(tokens.gclks.gclk1, dfll);
//! let _gclk1 = gclk1.div(Gclk1Div::Div(24)).enable();
//!
//! // Xosc based Dpll OutFreq: InFreq * (int + frac / 32) / (2 * (1 + predivider))
//! // Xosc (8 MHz) -> Dpll0 (8 MHz * (50 + 0 / 32) / (2 * (1 + 1)) -> 100 MHz
//! let (dpll0, _xosc0) = Dpll::from_xosc(tokens.dpll0, xosc0, 1);
//! let dpll0 = dpll0.set_loop_div(50, 0).enable();
//!
//! // Swap Dfll (48 MHz) for Dpll0 (100 MHz) in Gclk0
//! // Gclk0 drives MCLK and CPU, it can be neither disabled nor deconstructed
//! let (gclk0, _dfll, _dpll0) = gclk0.swap(dfll, dpll0);
//!
//! //// Output Gclk0 on a pin PB14
//! let (_gclk_out0, gclk0) =
//!     GclkOut::enable(tokens.gclk_io.gclk_out0, pins.pb14, gclk0, false);
//!
//! // Pclk to be consumed by an adequate peripheral abstraction
//! let (sercom0_pclk, gclk0) = Pclk::enable(tokens.pclks.sercom0, gclk0);
//!
//! // Clocking API V1 compatibility layer:
//! // Access to pac::MCLK
//! let (_, _, _, mclk) = unsafe { tokens.pac.steal() };
//! // v2's Pclks are convertible to v1's CoreClocks
//! let sercom0_core_clock: v1::Sercom0CoreClock = sercom0_pclk.into();
//!
//! // Synchronous clocking domain (v1's MCLK)
//! // Synchronous clocks are also expressed by typestates
//! let trng_apb: ApbToken<Trng> = tokens.apbs.trng;
//! let trng_apb: ApbClk<Trng> = trng_apb.enable();
//! ```
//!
//! More information on technicalities regarding implementation and principles
//! of operations can be found in a [`types`] module documentation.
//!
//! HAL also provides macros with ready-to-use presets. These presets correspond
//! to opinionated clock setup from API v1:
//! [`atsamd_hal::clocking_preset_*`](crate#macros)

use typenum::{Unsigned, U0};

use crate::pac::{GCLK, MCLK, OSC32KCTRL, OSCCTRL};

use crate::time::Hertz;
use crate::typelevel::{Counter, PrivateDecrement, PrivateIncrement, Sealed};

pub mod ahb;
pub mod apb;
pub mod dfll;
pub mod dpll;
pub mod gclk;
pub mod gclkio;
pub mod osculp32k;
pub mod pclk;
pub mod rtcosc;
pub mod types;
pub mod xosc;
pub mod xosc32k;

mod reset;
pub use reset::*;

/// Collection of low-level PAC structs.
///
/// Gathers all clocking related peripherals consumed by [`retrieve_clocks`]
/// function that are then being contained within [`Tokens::pac`] field. PAC
/// structs can be accessed using unsafe [`PacClocks::steal`] function.
pub struct PacStructs {
    oscctrl: OSCCTRL,
    osc32kctrl: OSC32KCTRL,
    gclk: GCLK,
    mclk: MCLK,
}

impl PacStructs {
    /// Escape hatch allowing to access low-level PAC structs.
    /// This is especially useful when V2 clocking API must interact with
    /// legacy V1 clocking API based peripherals; E.g. access to [`MCLK`] is
    /// necessary in most circumstances.
    ///
    /// # Safety
    ///
    /// Stealing the PAC resources allows for full control of
    /// clocking, something clocking v2 cannot observe or detect.
    ///
    /// Thus changing clocking "behind the back" of v2 clocking might invalidate
    /// typestates representing the current configuration as seen by v2.
    pub unsafe fn steal(self) -> (OSCCTRL, OSC32KCTRL, GCLK, MCLK) {
        (self.oscctrl, self.osc32kctrl, self.gclk, self.mclk)
    }
}

/// Marks clock types that can act as a source for downstream clocks
///
/// Implementers of this type can drive downstream clocks in the clock tree.
/// Typically, implementors are [`Enabled`] clocks. The `Id` associated type
/// maps to the corresponding `*Id` type of the implementer.
///
/// For example, `Enabled<Gclk5<DfllId>>` would implement
/// `Source<Id = GclkId5>`.
pub trait Source: Sealed {
    /// `Id` type of the implementer, e.g. [`GclkId5`](gclk::GclkId5) for
    /// `Enabled<Gclk5<DfllId>>`
    type Id;

    /// Returns a clock signal frequency produced by a source
    fn freq(&self) -> Hertz;
}

/// An enabled clock with compile-time counting of downstream users
///
/// This struct is a wrapper around other clock types from this module. It
/// represents a clock that has been enabled, and it maintains a compile-time
/// count of its uses by downstream clocks in the clock tree.
///
/// Compile-time counting allows the API to enforce when clocks may be enabled
/// or disabled. In particular, most clocks can only be disabled when their
/// counter is zero. However, there are exceptions, most notably [`Gclk0`],
/// which can never be disabled, because it is the main clock.
///
/// The count is maintained using the [`Counter`] trait, along with type-level,
/// [`Unsigned`] integers from the [`typenum`] crate.
///
/// [`Gclk0`]: gclk::Gclk0
pub struct Enabled<T, N: Counter = U0>(pub(crate) T, N);

impl<T, N: Counter> Sealed for Enabled<T, N> {}

impl<T, N: Unsigned + Counter> Enabled<T, N> {
    #[inline]
    pub(crate) fn new(t: T) -> Self {
        Enabled(t, N::default())
    }
}

impl<T, N: PrivateIncrement> PrivateIncrement for Enabled<T, N> {
    type Inc = Enabled<T, N::Inc>;

    #[inline]
    fn inc(self) -> Self::Inc {
        Enabled(self.0, self.1.inc())
    }
}

impl<T, N: PrivateDecrement> PrivateDecrement for Enabled<T, N> {
    type Dec = Enabled<T, N::Dec>;

    #[inline]
    fn dec(self) -> Self::Dec {
        Enabled(self.0, self.1.dec())
    }
}

impl<T, N: Counter> Counter for Enabled<T, N> {}

/// Test
pub fn test() {
    use crate::{
        clock::v2::{
            dpll::Dpll,
            gclk::{Gclk, Gclk1Div},
            gclkio::GclkOut,
            pclk::Pclk,
            xosc::{CrystalCurrent, Xosc},
        },
        gpio::v2::Pins,
        pac::Peripherals,
        time::U32Ext,
    };
    let mut pac = Peripherals::take().unwrap();
    let (clocks, tokens) = reset_clocks_tokens(
        pac.OSCCTRL,
        pac.OSC32KCTRL,
        pac.GCLK,
        pac.MCLK,
        &mut pac.NVMCTRL,
    );

    let pins = Pins::new(pac.PORT);

    // Asynchronous clocking domain
    // Xosc0 (8 MHz) set up using pins PA14 and PA15
    let xosc0 = Xosc::from_crystal(
        tokens.xosc0,
        pins.pa14,
        pins.pa15,
        8.mhz(),
        CrystalCurrent::Low,
    )
    .enable();

    // Dfll (48 MHz) -> Gclk1 (48 MHz / 24) -> 2 MHz
    let (gclk1, dfll) = Gclk::new(tokens.gclks.gclk1, clocks.dfll);
    let _gclk1 = gclk1.div(Gclk1Div::Div(24)).enable();

    // Xosc based Dpll OutFreq: InFreq * (int + frac / 32) / (2 * (1 + predivider))
    // Xosc (8 MHz) -> Dpll0 (8 MHz * (50 + 0 / 32) / (2 * (1 + 1)) -> 100 MHz
    let (dpll0, _xosc0) = Dpll::from_xosc0(tokens.dpll0, xosc0, 1);
    let dpll0 = dpll0.set_loop_div(50, 0).enable().ok().unwrap();

    // Swap Dfll (48 MHz) for Dpll0 (100 MHz) in Gclk0
    // Gclk0 drives MCLK and CPU, it can be neither disabled nor deconstructed
    let (gclk0, _dfll, _dpll0) = clocks.gclk0.swap(dfll, dpll0);

    //// Output Gclk0 on a pin PB14
    let (_gclk_out0, gclk0) = GclkOut::enable(tokens.gclk_io.gclk_out0, pins.pb14, gclk0, false);

    // Pclk to be consumed by an adequate peripheral abstraction
    let (_sercom0_pclk, _gclk0) = Pclk::enable(tokens.pclks.sercom0, gclk0);
}
