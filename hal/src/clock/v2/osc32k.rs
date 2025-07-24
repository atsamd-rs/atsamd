//! # Tunable, low-speed and low-power clock source
//!
//! ## Overview
//!
//! TODO this documentation is for the SAMD51 version, not the SYSCTRL sub-peripheral
//!
//! The `osc32k` module provides access to the 32 kHz ultra low power
//! internal oscillator (OSC32K) within the `OSC32KCTRL` peripheral.
//!
//! The `OSC32K` clock is unlike most other clocks. First, it is an internal
//! clock that is always enabled and can't be disabled. And second, it has two
//! separate outputs, one at 32 kHz and another divided down to 1 kHz. Moreover,
//! none, either or both of these outputs can be enabled at any given time.
//!
//! We can see, then, that the `OSC32K` peripheral forms its own, miniature
//! clock tree. There is a 1:N producer clock that is always enabled; and there
//! are two possible consumer clocks that can be independently and optionally
//! enabled. In fact, this structure is illustrated by the `OSC32K`
//! register, which has no regular `ENABLE` bit and two different enable bits
//! for clock output, `EN32K` and `EN1K`.
//!
//! To represent this structure in the type system, we divide the `OSC32K`
//! peripheral into these three clocks. Users get access to the 1:N
//! [`EnabledOsc32kBase`] clock [`Source`] at power-on reset, which can be
//! consumed by both the [`Osc32k`] and [`Osc1k`] clocks. Note that
//! `Osc32k` and `Osc1k` are themselves 1:N clocks as well.
//!
//! ## Write lock
//!
//! Rhe `OSC32K` register has a dedicated write lock bit that will freeze its
//! configuration until the next power-on reset. We implement this by simply
//! dropping the [`Osc32kBase`] clock, which prevents any further access to
//! the `OSC32K` register.
//!
//! ## Example
//!
//! Creating and configuring the OSC32K clocks proceeds according to the
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
//!         osc32k::{Osc1k, Osc32k},
//!     },
//!     pac::Peripherals,
//! };
//! let mut pac = Peripherals::take().unwrap();
//! let (buses, clocks, tokens) = clock_system_at_reset(
//!     pac.OSCCTRL,
//!     pac.OSC32KCTRL,
//!     pac.GCLK,
//!     pac.MCLK,
//!     &mut pac.NVMCTRL,
//! );
//! ```
//!
//! Next, we can extract the [`EnabledOsc32kBase`] clock from the [`Clocks`]
//! struct and use it to enable the [`Osc1k`] and [`Osc32k`] clocks.
//!
//! ```no_run
//! # use atsamd_hal::{
//! #     clock::v2::{
//! #         clock_system_at_reset,
//! #         osc32k::{Osc1k, Osc32k},
//! #     },
//! #     pac::Peripherals,
//! # };
//! # let mut pac = Peripherals::take().unwrap();
//! # let (buses, clocks, tokens) = clock_system_at_reset(
//! #     pac.OSCCTRL,
//! #     pac.OSC32KCTRL,
//! #     pac.GCLK,
//! #     pac.MCLK,
//! #     &mut pac.NVMCTRL,
//! # );
//! let base = clocks.osc32k_base;
//! let (osc1k, base) = Osc1k::enable(tokens.osc32k.osc1k, base);
//! let (osc32k, base) = Osc32k::enable(tokens.osc32k.osc32k, base);
//! ```
//!
//! We can then override the calibration value read from flash at start up.
//!
//! ```no_run
//! # use atsamd_hal::{
//! #     clock::v2::{
//! #         clock_system_at_reset,
//! #         osc32k::{Osc1k, Osc32k},
//! #     },
//! #     pac::Peripherals,
//! # };
//! # let mut pac = Peripherals::take().unwrap();
//! # let (buses, clocks, tokens) = clock_system_at_reset(
//! #     pac.OSCCTRL,
//! #     pac.OSC32KCTRL,
//! #     pac.GCLK,
//! #     pac.MCLK,
//! #     &mut pac.NVMCTRL,
//! # );
//! # let base = clocks.osc32k_base;
//! # let (osc1k, base) = Osc1k::enable(tokens.osc32k.osc1k, base);
//! # let (osc32k, mut base) = Osc32k::enable(tokens.osc32k.osc32k, base);
//! base.set_calibration(128);
//! ```
//!
//! And finally, we can set the write lock bit to freeze the configuation until
//! the next power-on reset. Doing so also drops the `EnabledOsc32kBase`
//! clock.
//!
//! ```no_run
//! # use atsamd_hal::{
//! #     clock::v2::{
//! #         clock_system_at_reset,
//! #         osc32k::{Osc1k, Osc32k},
//! #     },
//! #     pac::Peripherals,
//! # };
//! # let mut pac = Peripherals::take().unwrap();
//! # let (buses, clocks, tokens) = clock_system_at_reset(
//! #     pac.OSCCTRL,
//! #     pac.OSC32KCTRL,
//! #     pac.GCLK,
//! #     pac.MCLK,
//! #     &mut pac.NVMCTRL,
//! # );
//! # let base = clocks.osc32k_base;
//! # let (osc1k, base) = Osc1k::enable(tokens.osc32k.osc1k, base);
//! # let (osc32k, mut base) = Osc32k::enable(tokens.osc32k.osc32k, base);
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
//!         osc32k::{Osc1k, Osc32k},
//!     },
//!     pac::Peripherals,
//! };
//! let mut pac = Peripherals::take().unwrap();
//! let (buses, clocks, tokens) = clock_system_at_reset(
//!     pac.OSCCTRL,
//!     pac.OSC32KCTRL,
//!     pac.GCLK,
//!     pac.MCLK,
//!     &mut pac.NVMCTRL,
//! );
//! let base = clocks.osc32k_base;
//! let (osc1k, base) = Osc1k::enable(tokens.osc32k.osc1k, base);
//! let (osc32k, mut base) = Osc32k::enable(tokens.osc32k.osc32k, base);
//! base.set_calibration(128);
//! base.write_lock();
//! ```
//!
//! [`clock` module documentation]: super
//! [`clock_system_at_reset`]: super::clock_system_at_reset
//! [`Clocks`]: super::Clocks

use atsamd_hal_macros::{hal_cfg, hal_macro_helper};

use typenum::U0;

use crate::pac::sysctrl::Osc32k as OSC32K;

use crate::typelevel::{Decrement, Increment, PrivateDecrement, PrivateIncrement};

use fugit::RateExtU32;
use crate::time::Hertz;
use crate::typelevel::Sealed;

use super::{Enabled, Source};

//==============================================================================
// Tokens
//==============================================================================

/// Singleton token for the [`Osc32kBase`] clock
//
// There should never be more than one instance of `Osc32kBaseToken`, because
// it relies on that fact for memory safety.
//
// Users never see `Osc32kBaseToken`, because the OSC32K base oscillator
// is always enabled. Internally, however, it is used as a register interface.
// The token is zero-sized, so it can be carried by clock types without
// introducing any memory bloat.
//
// As part of that register interface, the `Osc32kBaseToken` can access the
// `OSC32K` register. That the token is a singleton guarantees the register
// is written from only one location. This allows the token to be `Sync`, even
// though the PAC `OSC32KCTRL` struct is not.
pub struct Osc32kBaseToken(());

/// Singleton token that can be exchanged for [`Osc1k`]
///
/// As explained in the [`clock` module documentation](super), instances of
/// various `Token` types can be exchanged for actual clock types. They
/// typically represent clocks that are disabled at power-on reset.
///
/// The [`Osc1k`] clock is disabled at power-on reset. To use it, you must
/// first exchange the token for an actual clock with [`Osc1k::enable`].

#[hal_cfg("sysctrl-d11")]
pub struct Osc1kToken(());

/// Singleton token that can be exchanged for [`Osc32k`]
///
/// As explained in the [`clock` module documentation](super), instances of
/// various `Token` types can be exchanged for actual clock types. They
/// typically represent clocks that are disabled at power-on reset.
///
/// The [`Osc32k`] clock is disabled at power-on reset. To use it, you must
/// first exchange the token for an actual clock with [`Osc32k::enable`].
pub struct Osc32kToken(());

/// Set of tokens representing the disabled OSC32K clocks power-on reset
#[hal_macro_helper]
pub struct Osc32kTokens {
    pub base: Osc32kBaseToken,
    #[hal_cfg("sysctrl-d11")]
    pub osc1k: Osc1kToken,
    pub osc32k: Osc32kToken,
}

impl Osc32kTokens {
    /// Create the set of tokens
    ///
    /// # Safety
    ///
    /// There must never be more than one instance of each token at any given
    /// time. See the notes on `Token` types and memory safety in the root of
    /// the `clock` module for more details.
    #[allow(unused)]
    #[hal_macro_helper]
    pub(super) unsafe fn new() -> Self {
        Self {
            base: Osc32kBaseToken(()),
            #[hal_cfg("sysctrl-d11")]
            osc1k: Osc1kToken(()),
            osc32k: Osc32kToken(()),
        }
    }
}

impl Osc32kBaseToken {
    #[inline]
    fn osc32k(&self) -> &OSC32K {
        // Safety: The `Osc32kBaseToken` has exclusive access to the
        // `OSC32K` register. See the notes on `Token` types and memory
        // safety in the root of the `clock` module for more details.
        unsafe { &(*crate::pac::Sysctrl::PTR).osc32k() }
    }

    /// Set the calibration
    #[inline]
    fn set_calibration(&mut self, calib: u8) {
        // Safety: All bit patterns are valid for this field
        self.osc32k()
            .modify(|_, w| unsafe { w.calib().bits(calib) });
    }

    #[inline]
    fn enable(&mut self, settings: Settings) {
        self.osc32k().modify(|_, w| {
            // Safety: The value comes from the `StartUpDelay` enum,
            // so the value is guaranteed to be valid
            unsafe { w.startup().bits(settings.start_up as u8) };
            w.ondemand().bit(settings.on_demand);
            w.runstdby().bit(settings.run_standby);
            w.enable().set_bit()
        });
    }

    #[inline]
    fn disable(&mut self) {
        self.osc32k().modify(|_, w| w.enable().clear_bit());
    }

    /// Enable the 1 kHz output
    #[hal_cfg("sysctrl-d11")]
    #[inline]
    fn enable_1k(&mut self) {
        self.osc32k().modify(|_, w| w.en1k().set_bit());
    }

    /// Disable the 1 kHz output
    #[hal_cfg("sysctrl-d11")]
    #[inline]
    fn disable_1k(&mut self) {
        self.osc32k().modify(|_, w| w.en1k().clear_bit());
    }

    /// Enable the 32 kHz output
    #[inline]
    fn enable_32k(&mut self) {
        self.osc32k().modify(|_, w| w.en32k().set_bit());
    }

    /// Disable the 32 kHz output
    #[inline]
    fn disable_32k(&mut self) {
        self.osc32k().modify(|_, w| w.en32k().clear_bit());
    }

    /// Enable the write lock
    #[inline]
    fn write_lock(&mut self) {
        self.osc32k().modify(|_, w| w.wrtlock().set_bit());
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct Settings {
    start_up: StartUpDelay,
    on_demand: bool,
    run_standby: bool,
}

//==============================================================================
// StartUpDelay
//==============================================================================

#[repr(u8)]
#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub enum StartUpDelay {
    #[default]
    Delay92us,
    Delay122us,
    Delay183us,
    Delay305us,
    Delay549us,
    Delay1ms,
    Delay2ms,
    Delay4ms,
}

//==============================================================================
// OscBase
//==============================================================================

/// OSC3ULP2K base clock, which feeds the [`Osc1k`] and [`Osc32k`] clocks
///
/// The OSC32K peripheral has two possible clock outputs, one at 32 kHz and
/// another at 1 kHz. This structure is represented in the type system as a set
/// of three clocks forming a small clock tree. The [`Osc32kBase`] clock
/// represents the base oscillator that feeds the optional [`Osc1k`] and
/// [`Osc32k`] output clocks. See the [module-level documentation](super) for
/// details and examples.
pub struct Osc32kBase {
    token: Osc32kBaseToken,
    settings: Settings,
}

/// The [`Enabled`] [`Osc32kBase`] clock
///
/// As described in the [`clock` module documentation](super), the [`Enabled`]
/// wrapper implements compile-time clock tree safety by tracking the number of
/// clocks consuming the [`Osc32kBase`] clock and restricts access to the
/// underlying type to prevent misuse.
///
/// **NOTE:** The `Osc32kBase` clock is internal and can never be disabled,
/// so we do not provide a `disable` method.
///
/// As with [`Enabled`], the default value for `N` is `U0`; if left unspecified,
/// the counter is assumed to be zero.
pub type EnabledOsc32kBase<N = U0> = Enabled<Osc32kBase, N>;

impl Osc32kBase {
    /// Create the ultra-low power base oscillator
    ///
    /// # Safety
    ///
    /// Because an `Osc32kBase` contains an `Osc32kBaseToken`, there must
    /// never be more than one instance of this struct at any given time. See
    /// the notes on `Token` types and memory safety in the root of the `clock`
    /// module for more details.
    #[inline]
    pub fn new(token: Osc32kBaseToken) -> Self {
        let settings = Settings {
            start_up: StartUpDelay::Delay92us,
            on_demand: true,
            run_standby: false,
        };
        Self { token, settings }
    }

    pub fn start_up_delay(mut self, start_up: StartUpDelay) -> Self {
        self.settings.start_up = start_up;
        self
    }

    pub fn on_demand(mut self, on_demand: bool) -> Self {
        self.settings.on_demand = on_demand;
        self
    }

    pub fn run_standby(mut self, run_standby: bool) -> Self {
        self.settings.run_standby = run_standby;
        self
    }

    #[inline]
    pub fn enable(mut self) -> EnabledOsc32kBase {
        self.token.enable(self.settings);
        Enabled::new(self)
    }
}

impl<N> EnabledOsc32kBase<N> {
    #[inline]
    pub fn disable(mut self) -> Osc32kBase {
        self.0.token.disable();
        self.0
    }

    /// Override the factory-default calibration value
    #[inline]
    pub fn set_calibration(&mut self, calib: u8) {
        self.0.token.set_calibration(calib);
    }

    /// Freeze the OSC32K configuration until power-on reset
    ///
    /// This function sets the write-lock bit, which freezes the OSC32K
    /// configuration at the hardware level until power-on reset. At the API
    /// level, it also consumes and drops the [`Osc32kBase`] clock, which
    /// prevents any further modifications.
    #[inline]
    pub fn write_lock(mut self) {
        self.0.token.write_lock();
    }
}

//==============================================================================
// Ids
//==============================================================================

/// Type representing the identity of the [`Osc1k`] clock
///
/// See the discussion on [`Id` types](super#id-types) for more information.
#[hal_cfg("sysctrl-d11")]
pub enum Osc1kId {}

#[hal_cfg("sysctrl-d11")]
impl Sealed for Osc1kId {}

/// Type representing the identity of the [`Osc32k`] clock
///
/// See the discussion on [`Id` types](super#id-types) for more information.
pub enum Osc32kId {}

impl Sealed for Osc32kId {}

//==============================================================================
// Osc1k
//==============================================================================

/// Clock representing the 1 kHz output of the [`Osc32kBase`] clock
///
/// The OSC32K peripheral has two possible clock outputs, one at 32 kHz and
/// another at 1 kHz. This structure is represented in the type system as a set
/// of three clocks forming a small clock tree. The [`Osc1k`] clock is
/// derived from the [`Osc32kBase`] clock. See the
/// [module-level documentation](super) for details and examples.
#[hal_cfg("sysctrl-d11")]
pub struct Osc1k {
    #[allow(unused)]
    token: Osc1kToken,
}

/// The [`Enabled`] [`Osc1k`] clock
///
/// As described in the [`clock` module documentation](super), the [`Enabled`]
/// wrapper implements compile-time clock tree safety by tracking the number of
/// clocks consuming the [`Osc1k`] clock and restricts access to the
/// underlying type to prevent misuse.
///
/// As with [`Enabled`], the default value for `N` is `U0`; if left unspecified,
/// the counter is assumed to be zero.
#[hal_cfg("sysctrl-d11")]
pub type EnabledOsc1k<N = U0> = Enabled<Osc1k, N>;

#[hal_cfg("sysctrl-d11")]
impl Osc1k {
    /// Enable 1 kHz output from the [`Osc32kBase`] clock
    ///
    /// This will [`Increment`] the [`EnabledOsc32kBase`] counter.
    #[inline]
    pub fn enable<N: Increment>(
        token: Osc1kToken,
        mut base: EnabledOsc32kBase<N>,
    ) -> (EnabledOsc1k, EnabledOsc32kBase<N::Inc>) {
        base.0.token.enable_1k();
        (Enabled::new(Self { token }), base.inc())
    }
}

#[hal_cfg("sysctrl-d11")]
impl EnabledOsc1k {
    /// Disable 1 kHz output from the [`Osc32kBase`] clock
    ///
    /// This will [`Decrement`] the [`EnabledOsc32kBase`] counter.
    #[inline]
    pub fn disable<N: Decrement>(
        self,
        mut base: EnabledOsc32kBase<N>,
    ) -> (Osc1kToken, EnabledOsc32kBase<N::Dec>) {
        base.0.token.disable_1k();
        (self.0.token, base.dec())
    }
}

#[hal_cfg("sysctrl-d11")]
impl<N> Source for EnabledOsc1k<N> {
    type Id = Osc1kId;

    #[inline]
    fn freq(&self) -> Hertz {
        Hertz(1024)
    }
}

//==============================================================================
// Osc32k
//==============================================================================

/// Clock representing the 32 kHz output of the [`Osc32kBase`] clock
///
/// The OSC32K peripheral has two possible clock outputs, one at 32 kHz and
/// another at 1 kHz. This structure is represented in the type system as a set
/// of three clocks forming a small clock tree. The [`Osc32k`] clock is
/// derived from the [`Osc32kBase`] clock. See the
/// [module-level documentation](super) for details and examples.
pub struct Osc32k {
    #[allow(unused)]
    token: Osc32kToken,
}

/// The [`Enabled`] [`Osc32k`] clock
///
/// As described in the [`clock` module documentation](super), the [`Enabled`]
/// wrapper implements compile-time clock tree safety by tracking the number of
/// clocks consuming the [`Osc32k`] clock and restricts access to the
/// underlying type to prevent misuse.
///
/// As with [`Enabled`], the default value for `N` is `U0`; if left unspecified,
/// the counter is assumed to be zero.
pub type EnabledOsc32k<N = U0> = Enabled<Osc32k, N>;

impl Osc32k {
    /// Enable 32 kHz output from the [`Osc32kBase`] clock
    ///
    /// This will [`Increment`] the [`EnabledOsc32kBase`] counter.
    #[inline]
    pub fn enable<N: Increment>(
        token: Osc32kToken,
        mut base: EnabledOsc32kBase<N>,
    ) -> (EnabledOsc32k, EnabledOsc32kBase<N::Inc>) {
        base.0.token.enable_32k();
        (Enabled::new(Self { token }), base.inc())
    }
}

impl EnabledOsc32k {
    /// Disable 32 kHz output from the [`Osc32kBase`] clock
    ///
    /// This will [`Decrement`] the [`EnabledOsc32kBase`] counter.
    #[inline]
    pub fn disable<N: Decrement>(
        self,
        mut base: EnabledOsc32kBase<N>,
    ) -> (Osc32kToken, EnabledOsc32kBase<N::Dec>) {
        base.0.token.disable_32k();
        (self.0.token, base.dec())
    }
}

impl<N> Source for EnabledOsc32k<N> {
    type Id = Osc32kId;

    #[inline]
    fn freq(&self) -> Hertz {
        32_768u32.Hz()
    }
}
