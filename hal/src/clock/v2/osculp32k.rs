//! # Internal, ultra low power, 32 kHz oscillator
//!
//! ## Overview
//!
//! The `osculp32k` module provides access to the 32 kHz ultra low power
//! internal oscillator (OSCULP32K) within the `OSC32KCTRL` or `SYSCTRL`
//! peripheral.
//!
//! The `OSCULP32K` clock is unlike most other clocks. First, it is an internal
//! clock that is always enabled and can't be disabled. And second, it has two
//! separate outputs, one at 32 kHz and another divided down to 1 kHz. Moreover,
//! none, either or both of these outputs can be enabled at any given time.
//!
//! We can see, then, that the `OSCULP32K` peripheral forms its own, miniature
//! clock tree. There is a 1:N producer clock that is always enabled; and there
//! are two possible consumer clocks that can be independently and optionally
//! enabled. In fact, this structure is illustrated by the `OSCULP32K` register,
//! which has no regular `ENABLE` bit and two different enable bits for clock
//! output, `EN32K` and `EN1K`.
//!
//! To represent this structure in the type system, we divide the `OSCULP32K`
//! peripheral into these three clocks. Users get access to the 1:N
//! [`EnabledOscUlp32kBase`] clock [`Source`] at power-on reset, which can be
//! consumed by both the [`OscUlp32k`] and [`OscUlp1k`] clocks. Note that
//! `OscUlp32k` and `OscUlp1k` are themselves 1:N clocks as well.
//!
//! ## Write lock
//!
//! Rhe `OSCULP32K` register has a dedicated write lock bit that will freeze its
//! configuration until the next power-on reset. We implement this by simply
//! dropping the [`OscUlp32kBase`] clock, which prevents any further access to
//! the `OSCULP32K` register.
//!
//! ## Example
//!
//! Creating and configuring the OSCULP32K clocks proceeds according to the
//! principles outlined in the [`clock` module documentation]. It is best shown
//! with an example.
//!
//! Let's start by using [`clock_system_at_reset`] to access the HAL clocking
//! structs.
//!
//! ```no_run
//! use atsamd_hal::{
//!     clock::v2::{
//!         clock_system_at_reset,
//!         osculp32k::{OscUlp1k, OscUlp32k},
//!     },
//!     pac::Peripherals,
//! };
//! let mut pac = Peripherals::take().unwrap();
//! let (buses, clocks, tokens) = clock_system_at_reset(
//!     pac.oscctrl,
//!     pac.osc32kctrl,
//!     pac.gclk,
//!     pac.mclk,
//!     &mut pac.nvmctrl,
//! );
//! ```
//!
//! Next, we can extract the [`EnabledOscUlp32kBase`] clock from the [`Clocks`]
//! struct and use it to enable the [`OscUlp1k`] and [`OscUlp32k`] clocks.
//!
//! ```no_run
//! # use atsamd_hal::{
//! #     clock::v2::{
//! #         clock_system_at_reset,
//! #         osculp32k::{OscUlp1k, OscUlp32k},
//! #     },
//! #     pac::Peripherals,
//! # };
//! # let mut pac = Peripherals::take().unwrap();
//! # let (buses, clocks, tokens) = clock_system_at_reset(
//! #     pac.oscctrl,
//! #     pac.osc32kctrl,
//! #     pac.gclk,
//! #     pac.mclk,
//! #     &mut pac.nvmctrl,
//! # );
//! let base = clocks.osculp32k_base;
//! let (osculp1k, base) = OscUlp1k::enable(tokens.osculp32k.osculp1k, base);
//! let (osculp32k, base) = OscUlp32k::enable(tokens.osculp32k.osculp32k, base);
//! ```
//!
//! We can then override the calibration value read from flash at start up.
//!
//! ```no_run
//! # use atsamd_hal::{
//! #     clock::v2::{
//! #         clock_system_at_reset,
//! #         osculp32k::{OscUlp1k, OscUlp32k},
//! #     },
//! #     pac::Peripherals,
//! # };
//! # let mut pac = Peripherals::take().unwrap();
//! # let (buses, clocks, tokens) = clock_system_at_reset(
//! #     pac.oscctrl,
//! #     pac.osc32kctrl,
//! #     pac.gclk,
//! #     pac.mclk,
//! #     &mut pac.nvmctrl,
//! # );
//! # let base = clocks.osculp32k_base;
//! # let (osculp1k, base) = OscUlp1k::enable(tokens.osculp32k.osculp1k, base);
//! # let (osculp32k, mut base) = OscUlp32k::enable(tokens.osculp32k.osculp32k, base);
//! base.set_calibration(128);
//! ```
//!
//! And finally, we can set the write lock bit to freeze the configuation until
//! the next power-on reset. Doing so also drops the `EnabledOscUlp32kBase`
//! clock.
//!
//! ```no_run
//! # use atsamd_hal::{
//! #     clock::v2::{
//! #         clock_system_at_reset,
//! #         osculp32k::{OscUlp1k, OscUlp32k},
//! #     },
//! #     pac::Peripherals,
//! # };
//! # let mut pac = Peripherals::take().unwrap();
//! # let (buses, clocks, tokens) = clock_system_at_reset(
//! #     pac.oscctrl,
//! #     pac.osc32kctrl,
//! #     pac.gclk,
//! #     pac.mclk,
//! #     &mut pac.nvmctrl,
//! # );
//! # let base = clocks.osculp32k_base;
//! # let (osculp1k, base) = OscUlp1k::enable(tokens.osculp32k.osculp1k, base);
//! # let (osculp32k, mut base) = OscUlp32k::enable(tokens.osculp32k.osculp32k, base);
//! # base.set_calibration(128);
//! base.write_lock();
//! ```
//!
//! The complete example is shown below.
//!
//! ```no_run
//! use atsamd_hal::{
//!     clock::v2::{
//!         clock_system_at_reset,
//!         osculp32k::{OscUlp1k, OscUlp32k},
//!     },
//!     pac::Peripherals,
//! };
//! let mut pac = Peripherals::take().unwrap();
//! let (buses, clocks, tokens) = clock_system_at_reset(
//!     pac.oscctrl,
//!     pac.osc32kctrl,
//!     pac.gclk,
//!     pac.mclk,
//!     &mut pac.nvmctrl,
//! );
//! let base = clocks.osculp32k_base;
//! let (osculp1k, base) = OscUlp1k::enable(tokens.osculp32k.osculp1k, base);
//! let (osculp32k, mut base) = OscUlp32k::enable(tokens.osculp32k.osculp32k, base);
//! base.set_calibration(128);
//! base.write_lock();
//! ```
//!
//! [`clock` module documentation]: super
//! [`clock_system_at_reset`]: super::clock_system_at_reset
//! [`Clocks`]: super::Clocks

use atsamd_hal_macros::{hal_cfg, hal_macro_helper};
use fugit::RateExtU32;
use typenum::U0;

#[hal_cfg("osc32kctrl")]
use crate::pac::osc32kctrl::Osculp32k;
#[hal_cfg("osc32kctrl")]
use crate::typelevel::{Decrement, Increment, PrivateDecrement, PrivateIncrement};

#[hal_cfg("sysctrl")]
use crate::pac::sysctrl::Osculp32k;

use crate::time::Hertz;
use crate::typelevel::Sealed;

use super::{Enabled, Source};

//==============================================================================
// Tokens
//==============================================================================

/// Singleton token for the [`OscUlp32kBase`] clock
//
// There should never be more than one instance of `OscUlp32kBaseToken`, because
// it relies on that fact for memory safety.
//
// Users never see `OscUlp32kBaseToken`, because the OSCULP32K base oscillator
// is always enabled. Internally, however, it is used as a register interface.
// The token is zero-sized, so it can be carried by clock types without
// introducing any memory bloat.
//
// As part of that register interface, the `OscUlp32kBaseToken` can access the
// `OSCULP32K` register. That the token is a singleton guarantees the register
// is written from only one location. This allows the token to be `Sync`, even
// though the PAC `OSC32KCTRL` struct is not.
struct OscUlp32kBaseToken(());

/// Singleton token that can be exchanged for [`OscUlp1k`]
///
/// As explained in the [`clock` module documentation](super), instances of
/// various `Token` types can be exchanged for actual clock types. They
/// typically represent clocks that are disabled at power-on reset.
///
/// On some targets, the [`OscUlp1k`] clock is disabled at power-on reset, and
/// to use it the token must first be exchanged for an actual clock with
/// [`OscUlp1k::enable`].
pub struct OscUlp1kToken(());

/// Singleton token that can be exchanged for [`OscUlp32k`]
///
/// As explained in the [`clock` module documentation](super), instances of
/// various `Token` types can be exchanged for actual clock types. They
/// typically represent clocks that are disabled at power-on reset.
///
/// On some targets, the [`OscUlp32k`] clock is disabled at power-on reset, and
/// to use it the token must first be exchanged for an actual clock with
/// [`OscUlp32k::enable`].
pub struct OscUlp32kToken(());

impl OscUlp32kBaseToken {
    #[inline]
    #[hal_macro_helper]
    fn osculp32k(&self) -> &Osculp32k {
        // Safety: The `OscUlp32kBaseToken` has exclusive access to the
        // `OSCULP32K` register. See the notes on `Token` types and memory
        // safety in the root of the `clock` module for more details.
        #[hal_cfg("osc32kctrl")]
        unsafe {
            (*crate::pac::Osc32kctrl::PTR).osculp32k()
        }
        #[hal_cfg("sysctrl")]
        unsafe {
            (*crate::pac::Sysctrl::PTR).osculp32k()
        }
    }

    #[hal_cfg("clock-d5x")]
    /// Enable the 1 kHz output
    #[inline]
    fn enable_1k(&mut self) {
        self.osculp32k().modify(|_, w| w.en1k().set_bit());
    }

    #[hal_cfg("clock-d5x")]
    /// Disable the 1 kHz output
    #[inline]
    fn disable_1k(&mut self) {
        self.osculp32k().modify(|_, w| w.en1k().clear_bit());
    }

    #[hal_cfg("clock-d5x")]
    /// Enable the 32 kHz output
    #[inline]
    fn enable_32k(&mut self) {
        self.osculp32k().modify(|_, w| w.en32k().set_bit());
    }

    #[hal_cfg("clock-d5x")]
    /// Disable the 32 kHz output
    #[inline]
    fn disable_32k(&mut self) {
        self.osculp32k().modify(|_, w| w.en32k().clear_bit());
    }

    /// Enable the write lock
    #[inline]
    fn write_lock(&mut self) {
        self.osculp32k().modify(|_, w| w.wrtlock().set_bit());
    }
}

//==============================================================================
// OscUlpBase
//==============================================================================

/// OSC3ULP2K base clock, which feeds the [`OscUlp1k`] and [`OscUlp32k`] clocks
///
/// The OSCULP32K peripheral has two possible clock outputs, one at 32 kHz and
/// another at 1 kHz. This structure is represented in the type system as a set
/// of three clocks forming a small clock tree. The [`OscUlp32kBase`] clock
/// represents the base oscillator that feeds the optional [`OscUlp1k`] and
/// [`OscUlp32k`] output clocks. See the [module-level documentation](super) for
/// details and examples.
pub struct OscUlp32kBase {
    token: OscUlp32kBaseToken,
}

/// The [`Enabled`] [`OscUlp32kBase`] clock
///
/// As described in the [`clock` module documentation](super), the [`Enabled`]
/// wrapper implements compile-time clock tree safety by tracking the number of
/// clocks consuming the [`OscUlp32kBase`] clock and restricts access to the
/// underlying type to prevent misuse.
///
/// **NOTE:** The `OscUlp32kBase` clock is internal and can never be disabled,
/// so we do not provide a `disable` method.
///
/// As with [`Enabled`], the default value for `N` is `U0`; if left unspecified,
/// the counter is assumed to be zero.
pub type EnabledOscUlp32kBase<N = U0> = Enabled<OscUlp32kBase, N>;

impl OscUlp32kBase {
    /// Create the ultra-low power base oscillator
    ///
    /// # Safety
    ///
    /// Because an `OscUlp32kBase` contains an `OscUlp32kBaseToken`, there must
    /// never be more than one instance of this struct at any given time. See
    /// the notes on `Token` types and memory safety in the root of the `clock`
    /// module for more details.
    #[inline]
    pub(super) unsafe fn new() -> Self {
        let token = OscUlp32kBaseToken(());
        Self { token }
    }
}

impl<N> EnabledOscUlp32kBase<N> {
    /// Freeze the OSCULP32K configuration until power-on reset
    ///
    /// This function sets the write-lock bit, which freezes the OSCULP32K
    /// configuration at the hardware level until power-on reset. At the API
    /// level, it also consumes and drops the [`OscUlp32kBase`] clock, which
    /// prevents any further modifications.
    #[inline]
    pub fn write_lock(mut self) {
        self.0.token.write_lock();
    }
}

//==============================================================================
// Ids
//==============================================================================

/// Type representing the identity of the [`OscUlp1k`] clock
///
/// See the discussion on [`Id` types](super#id-types) for more information.
pub enum OscUlp1kId {}

impl Sealed for OscUlp1kId {}

/// Type representing the identity of the [`OscUlp32k`] clock
///
/// See the discussion on [`Id` types](super#id-types) for more information.
pub enum OscUlp32kId {}

impl Sealed for OscUlp32kId {}

//==============================================================================
// OscUlp1k
//==============================================================================

/// Clock representing the 1 kHz output of the [`OscUlp32kBase`] clock
///
/// The OSCULP32K peripheral has two possible clock outputs, one at 32 kHz and
/// another at 1 kHz. This structure is represented in the type system as a set
/// of three clocks forming a small clock tree. The [`OscUlp1k`] clock is
/// derived from the [`OscUlp32kBase`] clock. See the
/// [module-level documentation](super) for details and examples.
pub struct OscUlp1k {
    #[allow(unused)]
    token: OscUlp1kToken,
}

/// The [`Enabled`] [`OscUlp1k`] clock
///
/// As described in the [`clock` module documentation](super), the [`Enabled`]
/// wrapper implements compile-time clock tree safety by tracking the number of
/// clocks consuming the [`OscUlp1k`] clock and restricts access to the
/// underlying type to prevent misuse.
///
/// As with [`Enabled`], the default value for `N` is `U0`; if left unspecified,
/// the counter is assumed to be zero.
pub type EnabledOscUlp1k<N = U0> = Enabled<OscUlp1k, N>;

impl OscUlp1k {
    /// Create the ultra-low power base oscillator
    ///
    /// # Safety
    ///
    /// Because an `OscUlp1k` contains an `OscUlp1kToken`, there must never be
    /// more than one instance of this struct at any given time. See the notes
    /// on `Token` types and memory safety in the root of the `clock` module for
    /// more details.
    #[inline]
    pub(super) unsafe fn new() -> Self {
        let token = OscUlp1kToken(());
        Self { token }
    }

    /// Enable 1 kHz output from the [`OscUlp32kBase`] clock
    ///
    /// This will [`Increment`] the [`EnabledOscUlp32kBase`] counter.
    #[hal_cfg("clock-d5x")]
    #[inline]
    pub fn enable<N: Increment>(
        token: OscUlp1kToken,
        mut base: EnabledOscUlp32kBase<N>,
    ) -> (EnabledOscUlp1k, EnabledOscUlp32kBase<N::Inc>) {
        base.0.token.enable_1k();
        (Enabled::new(Self { token }), base.inc())
    }

    /// The OSCULP32K 1.024kHz output is always enabled on the documented target
    #[cfg(doc)]
    #[hal_cfg(not("clock-d5x"))]
    pub fn enable(_token: OscUlp1kToken) {}
}

impl EnabledOscUlp1k {
    /// Disable 1 kHz output from the [`OscUlp32kBase`] clock
    ///
    /// This will [`Decrement`] the [`EnabledOscUlp32kBase`] counter.
    #[hal_cfg("clock-d5x")]
    #[inline]
    pub fn disable<N: Decrement>(
        self,
        mut base: EnabledOscUlp32kBase<N>,
    ) -> (OscUlp1kToken, EnabledOscUlp32kBase<N::Dec>) {
        base.0.token.disable_1k();
        (self.0.token, base.dec())
    }
}

impl<N> Source for EnabledOscUlp1k<N> {
    type Id = OscUlp1kId;

    #[inline]
    fn freq(&self) -> Hertz {
        1024.Hz()
    }
}

//==============================================================================
// OscUlp32k
//==============================================================================

/// Clock representing the 32 kHz output of the [`OscUlp32kBase`] clock
///
/// The OSCULP32K peripheral has two possible clock outputs, one at 32 kHz and
/// another at 1 kHz. This structure is represented in the type system as a set
/// of three clocks forming a small clock tree. The [`OscUlp32k`] clock is
/// derived from the [`OscUlp32kBase`] clock. See the
/// [module-level documentation](super) for details and examples.
pub struct OscUlp32k {
    #[allow(unused)]
    token: OscUlp32kToken,
}

/// The [`Enabled`] [`OscUlp32k`] clock
///
/// As described in the [`clock` module documentation](super), the [`Enabled`]
/// wrapper implements compile-time clock tree safety by tracking the number of
/// clocks consuming the [`OscUlp32k`] clock and restricts access to the
/// underlying type to prevent misuse.
///
/// As with [`Enabled`], the default value for `N` is `U0`; if left unspecified,
/// the counter is assumed to be zero.
pub type EnabledOscUlp32k<N = U0> = Enabled<OscUlp32k, N>;

impl OscUlp32k {
    /// Create the ultra-low power base oscillator
    ///
    /// # Safety
    ///
    /// Because an `OscUlp32k` contains an `OscUlp32kToken`, there must never be
    /// more than one instance of this struct at any given time. See the notes
    /// on `Token` types and memory safety in the root of the `clock` module for
    /// more details.
    #[inline]
    pub(super) unsafe fn new() -> Self {
        let token = OscUlp32kToken(());
        Self { token }
    }

    /// Enable 32 kHz output from the [`OscUlp32kBase`] clock
    ///
    /// This will [`Increment`] the [`EnabledOscUlp32kBase`] counter.
    #[hal_cfg("clock-d5x")]
    #[inline]
    pub fn enable<N: Increment>(
        token: OscUlp32kToken,
        mut base: EnabledOscUlp32kBase<N>,
    ) -> (EnabledOscUlp32k, EnabledOscUlp32kBase<N::Inc>) {
        base.0.token.enable_32k();
        (Enabled::new(Self { token }), base.inc())
    }

    /// The OSCULP32K 32.768kHz output is always enabled on the documented target
    #[cfg(doc)]
    #[hal_cfg(not("clock-d5x"))]
    pub fn enable(_token: OscUlp32kToken) {}
}

impl EnabledOscUlp32k {
    /// Disable 32 kHz output from the [`OscUlp32kBase`] clock
    ///
    /// This will [`Decrement`] the [`EnabledOscUlp32kBase`] counter.
    #[hal_cfg("clock-d5x")]
    #[inline]
    pub fn disable<N: Decrement>(
        self,
        mut base: EnabledOscUlp32kBase<N>,
    ) -> (OscUlp32kToken, EnabledOscUlp32kBase<N::Dec>) {
        base.0.token.disable_32k();
        (self.0.token, base.dec())
    }
}

impl<N> Source for EnabledOscUlp32k<N> {
    type Id = OscUlp32kId;

    #[inline]
    fn freq(&self) -> Hertz {
        32_768.Hz()
    }
}
