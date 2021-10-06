#![warn(missing_docs)]
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

use typenum::{U0, U1};

use crate::pac::{GCLK, MCLK, NVMCTRL, OSC32KCTRL, OSCCTRL};
use crate::time::Hertz;
use types::Enabled;

use rtc::{Active1k, Active32k};

mod presets;

pub mod ahb;
pub mod apb;
pub mod dfll;
pub mod dpll;
pub mod gclk;
pub mod gclkio;
pub mod osculp32k;
pub mod pclk;
pub mod rtc;
pub mod types;
pub mod xosc;
pub mod xosc32k;

/// Collection of low-level PAC structs.
///
/// Gathers all clocking related peripherals consumed by [`retrieve_clocks`]
/// function that are then being contained within [`Tokens::pac`] field. PAC
/// structs can be accessed using unsafe [`PacClocks::steal`] function.
pub struct PacClocks {
    oscctrl: OSCCTRL,
    osc32kctrl: OSC32KCTRL,
    gclk: GCLK,
    mclk: MCLK,
}

impl PacClocks {
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

/// This struct contains instantiated `Token` structs that can be used to
/// construct clocking components.
///
/// E.g. to call [`gclk::Gclk<gclk::Gen5, _>::new`] one needs
/// [`gclk::Tokens::gclk5`] from within [`Tokens::gclks`]. Thus, it is
/// impossible to create multiple instances of [`gclk::Gclk<gclk::Gen5, _>`].
///
/// These do not expose any API externally. Inside of the HAL crate, these
/// expose low-level API to HW register of finer granularity than regular PAC
/// structs.
pub struct Tokens {
    /// Wrapper for low level PAC -- can be unsafely stolen if needed
    pub pac: PacClocks,
    /// Synchronous clocking domain clocks -- AHB bus
    pub ahbs: ahb::AhbClks,
    /// Synchronous clocking domain clocks -- APB buses
    pub apbs: apb::ApbClks,
    /// Construction token for [`dpll::Dpll0`]
    pub dpll0: dpll::DpllToken<dpll::marker::Dpll0>,
    /// Construction token for [`dpll::Dpll1`]
    pub dpll1: dpll::DpllToken<dpll::marker::Dpll1>,
    /// Construction tokens for [`gclkio::GclkIo`]
    pub gclk_io: gclkio::Tokens,
    /// Construction tokens for [`gclk::Gclk`]
    pub gclks: gclk::Tokens,
    /// Construction tokens for [`pclk::Pclk`]
    pub pclks: pclk::Tokens,
    /// Construction token for [`xosc::Xosc0`]
    pub xosc0: xosc::XoscToken<xosc::Osc0>,
    /// Construction token for [`xosc::Xosc1`]
    pub xosc1: xosc::XoscToken<xosc::Osc1>,
    /// Construction token for [`xosc32k::Xosc32k`]
    pub xosc32k: xosc32k::Xosc32kToken,
}

/// Standalone function returning a set of instantiated clocking abstractions
/// representing a default state of a clocking system. For `thumbv7em` based
/// devices it is a chain of:
/// - [`dfll::Dfll<OpenLoop>`] (`48 MHz`)
/// - [`gclk::Gclk0<Dfll>`] (`48 MHz`)
///
/// And also ultra low power internal 32k oscillator:
///
/// - [`osculp32k::OscUlp32k`] (`32 kHz`)
pub fn retrieve_clocks(
    oscctrl: OSCCTRL,
    osc32kctrl: OSC32KCTRL,
    gclk: GCLK,
    mclk: MCLK,
    nvmctrl: &mut NVMCTRL,
) -> (
    Enabled<gclk::Gclk0<dfll::marker::Dfll>, U1>,
    Enabled<dfll::Dfll<dfll::OpenLoop>, U1>,
    Enabled<osculp32k::OscUlp32k<Active32k, Active1k>, U0>,
    Tokens,
) {
    // Safe because registers are instantiated only once
    unsafe {
        let tokens = Tokens {
            pac: PacClocks {
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
            xosc0: xosc::XoscToken::new(),
            xosc1: xosc::XoscToken::new(),
            xosc32k: xosc32k::Xosc32kToken::new(),
        };
        let dfll = Enabled::<_, U0>::new(dfll::Dfll::in_open_mode(dfll::DfllToken::new()));
        let (gclk0, dfll) = gclk::Gclk0::new(gclk::GclkToken::new(), dfll);
        let gclk0 = Enabled::new(gclk0);
        let osculp32k = Enabled::new(osculp32k::OscUlp32k::new(osculp32k::OscUlp32kToken::new()));
        (gclk0, dfll, osculp32k, tokens)
    }
}

/// Marker supertrait unifying family of more specific source marker traits.
///
/// These ones are essential during a construction (`fn ::{new, enable}`) and
///
/// deconstruction (`fn ::{free, disable}`) of clocking components as they
/// provide information to the constructed/deconstructed type what its source is
/// and which variant of source (associated constant) is applicable while
/// performing a HW register write.
pub trait SourceMarker: crate::typelevel::Sealed {}

/// Supertrait unifying family of more specific source traits.
///
/// These are implemented by specific specialized forms of [`types::Enabled`].
/// They are used during a construction (`fn ::{new, enable}`) and
/// deconstruction (`fn ::{free, disable}`) of clocking components and they
/// express the type of dependency needed by a dependee. For examples,
/// [`gclk::Gclk::new`] will only consume source implementing
/// [`gclk::GclkSource`] trait.
pub trait Source: crate::typelevel::Sealed {
    fn freq(&self) -> Hertz;
}
