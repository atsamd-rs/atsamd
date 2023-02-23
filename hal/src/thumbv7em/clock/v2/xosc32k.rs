//! # External, 32 kHz crystal oscillator controller
//!
//! ## Overview
//!
//! The `xosc32k` module provides access to the 32 kHz external crystal
//! oscillator controller (XOSC32K) within the `OSC32KCTRL` peripheral.
//!
//! The peripheral can operate in two [`Mode`]s. It can accept an external
//! clock, or it can interface with an crystal oscillator. In both cases, the
//! clock must be 32,768 Hz.
//!
//! When used with an external clock, only one GPIO [`Pin`] is required, but
//! when used with a crystal oscillator, two GPIO `Pin`s are required. The
//! [`XIn32`] `Pin` is used in both `Mode`s, while the [`XOut32`] `Pin` is only
//! used in [`CrystalMode`].
//!
//! ## Clock tree structure
//!
//! The `XOSC32K` clock is unlike most other clocks, because it has two separate
//! outputs, one at 32 kHz and another divided down to 1 kHz. Moreover, none,
//! either or both of these outputs can be enabled at any given time.
//!
//! We can see, then, that the `XOSC32K` peripheral forms its own, miniature
//! clock tree. There is a 1:N producer clock that must be enabled first; and
//! there are two possible consumer clocks that can be independently and
//! optionally enabled. In fact, this structure is illustrated by the `XOSC32K`
//! register, which has three different enable bits: `ENABLE`, `EN32K` and
//! `EN1K`.
//!
//! To represent this structure in the type system, we divide the `XOSC32K`
//! peripheral into these three clocks. Users start by enabling the
//! [`Xosc32kBase`] clock, which corresponds to setting the `XOSC32K` register
//! `ENABLE` bit. The call to [`Xosc32kBase::enable`] returns a 1:N [`Enabled`]
//! clock [`Source`], which can be consumed by both the [`Xosc32k`] and
//! [`Xosc1k`] clocks. Enabling either of these two clocks will [`Increment`]
//! the [`EnabledXosc32kBase`] counter, preventing it from being disabled.
//! Note that `Xosc32k` and `Xosc1k` are themselves 1:N clocks as well.
//!
//! ## Clock failure detection and write lock
//!
//! Like the [`Xosc`] clocks, the XOSC32K peripheral also has clock failure
//! detection. However, unlike the `XOSCCTRL` registers, the `XOSC32K` register
//! has a dedicated write lock bit that will freeze its configuration until the
//! next power-on reset.
//!
//! While `Xosc` clock failure detection is configured directly in the
//! `XOSCCTRL` register, the XOSC32K peripheral has a separate, dedicated
//! clock failure detection register (`CFDCTRL`). This difference likely exists
//! to provide control of clock failure detection *after* write lock has been
//! enabled.
//!
//! In this module, write lock is implemented by simply dropping the
//! [`Xosc32kBase`] clock, which prevents any further access to the `XOSC32K`
//! register. Thus, to allow control of clock failure detection in the presence
//! of write lock, we provide a dedicated [`Xosc32kCfd`] interface, which has
//! exclusive control over the `CFDCTRL` register.
//!
//! ## Example
//!
//! Creating and configuring the XOSC32K clocks proceeds according to the
//! principles outlined in the [`clock` module documentation]. It is best shown
//! with an example.
//!
//! Let's start by using [`clock_system_at_reset`] to access the HAL clocking
//! structs. We'll also need access to the GPIO [`Pins`].
//!
//! ```no_run
//! use atsamd_hal::{
//!     clock::v2::{
//!         clock_system_at_reset,
//!         osculp32k::OscUlp32k,
//!         xosc32k::{
//!             ControlGainMode, SafeClockDiv, StartUpDelay, Xosc1k, Xosc32k, Xosc32kBase,
//!             Xosc32kCfd,
//!         },
//!     },
//!     gpio::Pins,
//!     pac::Peripherals,
//! };
//! let mut pac = Peripherals::take().unwrap();
//! let pins = Pins::new(pac.PORT);
//! let (buses, clocks, tokens) = clock_system_at_reset(
//!     pac.OSCCTRL,
//!     pac.OSC32KCTRL,
//!     pac.GCLK,
//!     pac.MCLK,
//!     &mut pac.NVMCTRL,
//! );
//! ```
//!
//! Next, we create the [`Xosc32kBase`] clock from a 32 kHz oscillator using its
//! corresponding [`Xosc32kBaseToken`] and the [`XIn32`] and [`XOut32`] `Pin`s.
//! We then set the delay before the clock is unmasked by providing a desired
//! [`StartUpDelay`]. Finally, we select a [`ControlGainMode`] for the crystal
//! before enabling it.
//!
//! ```no_run
//! # use atsamd_hal::{
//! #     clock::v2::{
//! #         clock_system_at_reset,
//! #         osculp32k::OscUlp32k,
//! #         xosc32k::{
//! #             ControlGainMode, SafeClockDiv, StartUpDelay, Xosc1k, Xosc32k, Xosc32kBase,
//! #             Xosc32kCfd,
//! #         },
//! #     },
//! #     gpio::Pins,
//! #     pac::Peripherals,
//! # };
//! # let mut pac = Peripherals::take().unwrap();
//! # let pins = Pins::new(pac.PORT);
//! # let (buses, clocks, tokens) = clock_system_at_reset(
//! #     pac.OSCCTRL,
//! #     pac.OSC32KCTRL,
//! #     pac.GCLK,
//! #     pac.MCLK,
//! #     &mut pac.NVMCTRL,
//! # );
//! let xosc32k_base = Xosc32kBase::from_crystal(tokens.xosc32k.base, pins.pa00, pins.pa01)
//!     .start_up_delay(StartUpDelay::Delay1s)
//!     .control_gain_mode(ControlGainMode::HighSpeed)
//!     .enable();
//! ```
//!
//! At this point, we opt to wait until the `Xosc32kBase` oscillator `is_ready`
//! and stable.
//!
//! ```no_run
//! # use atsamd_hal::{
//! #     clock::v2::{
//! #         clock_system_at_reset,
//! #         osculp32k::OscUlp32k,
//! #         xosc32k::{
//! #             ControlGainMode, SafeClockDiv, StartUpDelay, Xosc1k, Xosc32k, Xosc32kBase,
//! #             Xosc32kCfd,
//! #         },
//! #     },
//! #     gpio::Pins,
//! #     pac::Peripherals,
//! # };
//! # let mut pac = Peripherals::take().unwrap();
//! # let pins = Pins::new(pac.PORT);
//! # let (buses, clocks, tokens) = clock_system_at_reset(
//! #     pac.OSCCTRL,
//! #     pac.OSC32KCTRL,
//! #     pac.GCLK,
//! #     pac.MCLK,
//! #     &mut pac.NVMCTRL,
//! # );
//! # let xosc32k_base = Xosc32kBase::from_crystal(tokens.xosc32k.base, pins.pa00, pins.pa01)
//! #     .start_up_delay(StartUpDelay::Delay1s)
//! #     .control_gain_mode(ControlGainMode::HighSpeed)
//! #     .enable();
//! while !xosc32k_base.is_ready() {}
//! ```
//!
//! With the [`EnabledXosc32kBase`] clock in hand, we can enable the [`Xosc1k`]
//! and [`Xosc32k`], each of which [`Increment`]s the [`Enabled`] counter.
//! Once we are satisfied with the configuration, we can call `write_lock` to
//! lock the XOSC32K configuration at the hardware level. Doing so also consumes
//! the `EnabledXosc32kBase` clock, which eliminates any ability to change the
//! configuration at the API level.
//!
//! ```no_run
//! # use atsamd_hal::{
//! #     clock::v2::{
//! #         clock_system_at_reset,
//! #         osculp32k::OscUlp32k,
//! #         xosc32k::{
//! #             ControlGainMode, SafeClockDiv, StartUpDelay, Xosc1k, Xosc32k, Xosc32kBase,
//! #             Xosc32kCfd,
//! #         },
//! #     },
//! #     gpio::Pins,
//! #     pac::Peripherals,
//! # };
//! # let mut pac = Peripherals::take().unwrap();
//! # let pins = Pins::new(pac.PORT);
//! # let (buses, clocks, tokens) = clock_system_at_reset(
//! #     pac.OSCCTRL,
//! #     pac.OSC32KCTRL,
//! #     pac.GCLK,
//! #     pac.MCLK,
//! #     &mut pac.NVMCTRL,
//! # );
//! # let xosc32k_base = Xosc32kBase::from_crystal(tokens.xosc32k.base, pins.pa00, pins.pa01)
//! #     .start_up_delay(StartUpDelay::Delay1s)
//! #     .control_gain_mode(ControlGainMode::HighSpeed)
//! #     .enable();
//! # while !xosc32k_base.is_ready() {}
//! let (xosc1k, xosc32k_base) = Xosc1k::enable(tokens.xosc32k.xosc1k, xosc32k_base);
//! let (xosc32k, xosc32k_base) = Xosc32k::enable(tokens.xosc32k.xosc32k, xosc32k_base);
//! xosc32k_base.write_lock();
//! ```
//!
//! However, while we have locked the XOSC32K configuration, we still want to
//! enable clock failure detection, which will continuously monitor the clock
//! and switch to a safe, backup clock if necessary.
//!
//! To do so, we must first enable the backup clock, which, for the XOSC32K, is
//! the [`OscUlp32k`]. The OSCULP32K peripheral has a nearly identical structure
//! to the XOSC32K; we create an [`EnabledOscUlp32k`] from the
//! [`EnabledOscUlp32kBase`] clock and the corresponding [`OscUlp32kToken`].
//!
//! Upon creation of the [`Xosc32kCfd`] struct, we register it as a consumer of
//! the `EnabledOscUlp32k`, which will `Increment` its `Counter` as well. When
//! creating the safe clock, the `OscUlp32k` can be optionally divided by two,
//! which is selected with [`SafeClockDiv`].
//!
//! ```no_run
//! # use atsamd_hal::{
//! #     clock::v2::{
//! #         clock_system_at_reset,
//! #         osculp32k::OscUlp32k,
//! #         xosc32k::{
//! #             ControlGainMode, SafeClockDiv, StartUpDelay, Xosc1k, Xosc32k, Xosc32kBase,
//! #             Xosc32kCfd,
//! #         },
//! #     },
//! #     gpio::Pins,
//! #     pac::Peripherals,
//! # };
//! # let mut pac = Peripherals::take().unwrap();
//! # let pins = Pins::new(pac.PORT);
//! # let (buses, clocks, tokens) = clock_system_at_reset(
//! #     pac.OSCCTRL,
//! #     pac.OSC32KCTRL,
//! #     pac.GCLK,
//! #     pac.MCLK,
//! #     &mut pac.NVMCTRL,
//! # );
//! # let xosc32k_base = Xosc32kBase::from_crystal(tokens.xosc32k.base, pins.pa00, pins.pa01)
//! #     .start_up_delay(StartUpDelay::Delay1s)
//! #     .control_gain_mode(ControlGainMode::HighSpeed)
//! #     .enable();
//! # while !xosc32k_base.is_ready() {}
//! # let (xosc1k, xosc32k_base) = Xosc1k::enable(tokens.xosc32k.xosc1k, xosc32k_base);
//! # let (xosc32k, xosc32k_base) = Xosc32k::enable(tokens.xosc32k.xosc32k, xosc32k_base);
//! # xosc32k_base.write_lock();
//! let (osculp32k, osculp32k_base) =
//!     OscUlp32k::enable(tokens.osculp32k.osculp32k, clocks.osculp32k_base);
//! let (mut cfd, osculp32k) =
//!     Xosc32kCfd::enable(tokens.xosc32k.cfd, osculp32k, SafeClockDiv::Div1);
//! ```
//!
//! Finally, with the clock failure detection interface in hand, we can do
//! things like check if the XOSC32K [`has_failed`] or if it [`is_switched`] to
//! the safe clock. If we are able to recover from a clock failure, we can even
//! [`switch_back`] to the crystal oscillator.
//!
//! ```no_run
//! # use atsamd_hal::{
//! #     clock::v2::{
//! #         clock_system_at_reset,
//! #         osculp32k::OscUlp32k,
//! #         xosc32k::{
//! #             ControlGainMode, SafeClockDiv, StartUpDelay, Xosc1k, Xosc32k, Xosc32kBase,
//! #             Xosc32kCfd,
//! #         },
//! #     },
//! #     gpio::Pins,
//! #     pac::Peripherals,
//! # };
//! # let mut pac = Peripherals::take().unwrap();
//! # let pins = Pins::new(pac.PORT);
//! # let (buses, clocks, tokens) = clock_system_at_reset(
//! #     pac.OSCCTRL,
//! #     pac.OSC32KCTRL,
//! #     pac.GCLK,
//! #     pac.MCLK,
//! #     &mut pac.NVMCTRL,
//! # );
//! # let xosc32k_base = Xosc32kBase::from_crystal(tokens.xosc32k.base, pins.pa00, pins.pa01)
//! #     .start_up_delay(StartUpDelay::Delay1s)
//! #     .control_gain_mode(ControlGainMode::HighSpeed)
//! #     .enable();
//! # while !xosc32k_base.is_ready() {}
//! # let (xosc1k, xosc32k_base) = Xosc1k::enable(tokens.xosc32k.xosc1k, xosc32k_base);
//! # let (xosc32k, xosc32k_base) = Xosc32k::enable(tokens.xosc32k.xosc32k, xosc32k_base);
//! # xosc32k_base.write_lock();
//! # let (osculp32k, osculp32k_base) =
//! #     OscUlp32k::enable(tokens.osculp32k.osculp32k, clocks.osculp32k_base);
//! # let (mut cfd, osculp32k) =
//! #     Xosc32kCfd::enable(tokens.xosc32k.cfd, osculp32k, SafeClockDiv::Div1);
//! if cfd.has_failed() && cfd.is_switched() {
//!     cfd.switch_back();
//! }
//! ```
//!
//! The complete example is provided below.
//!
//! ```no_run
//! use atsamd_hal::{
//!     clock::v2::{
//!         clock_system_at_reset,
//!         osculp32k::OscUlp32k,
//!         xosc32k::{
//!             ControlGainMode, SafeClockDiv, StartUpDelay, Xosc1k, Xosc32k, Xosc32kBase,
//!             Xosc32kCfd,
//!         },
//!     },
//!     gpio::Pins,
//!     pac::Peripherals,
//! };
//! let mut pac = Peripherals::take().unwrap();
//! let pins = Pins::new(pac.PORT);
//! let (buses, clocks, tokens) = clock_system_at_reset(
//!     pac.OSCCTRL,
//!     pac.OSC32KCTRL,
//!     pac.GCLK,
//!     pac.MCLK,
//!     &mut pac.NVMCTRL,
//! );
//! let xosc32k_base = Xosc32kBase::from_crystal(tokens.xosc32k.base, pins.pa00, pins.pa01)
//!     .start_up_delay(StartUpDelay::Delay1s)
//!     .control_gain_mode(ControlGainMode::HighSpeed)
//!     .enable();
//! while !xosc32k_base.is_ready() {}
//! let (xosc1k, xosc32k_base) = Xosc1k::enable(tokens.xosc32k.xosc1k, xosc32k_base);
//! let (xosc32k, xosc32k_base) = Xosc32k::enable(tokens.xosc32k.xosc32k, xosc32k_base);
//! xosc32k_base.write_lock();
//! let (osculp32k, osculp32k_base) =
//!     OscUlp32k::enable(tokens.osculp32k.osculp32k, clocks.osculp32k_base);
//! let (mut cfd, osculp32k) =
//!     Xosc32kCfd::enable(tokens.xosc32k.cfd, osculp32k, SafeClockDiv::Div1);
//! if cfd.has_failed() && cfd.is_switched() {
//!     cfd.switch_back();
//! }
//! ```
//!
//! [`clock` module documentation]: super
//! [`Pins`]: crate::gpio::Pins
//! [`clock_system_at_reset`]: super::clock_system_at_reset
//! [`Xosc`]: super::xosc::Xosc
//! [`OscUlp32k`]: super::osculp32k::OscUlp32k
//! [`EnabledOscUlp32k`]: super::osculp32k::EnabledOscUlp32k
//! [`OscUlp32kToken`]: super::osculp32k::OscUlp32kToken
//! [`EnabledOscUlp32kBase`]: super::osculp32k::EnabledOscUlp32kBase
//! [`OscUlp32k`]: super::osculp32k::OscUlp32k
//! [`has_failed`]: Xosc32kCfd::has_failed
//! [`is_switched`]: Xosc32kCfd::is_switched
//! [`switch_back`]: Xosc32kCfd::switch_back

use fugit::RateExtU32;
use typenum::U0;

use crate::pac::osc32kctrl::xosc32k::{CGM_A, STARTUP_A};
use crate::pac::osc32kctrl::{status, CFDCTRL, XOSC32K};

use crate::gpio::{FloatingDisabled, Pin, PA00, PA01};
use crate::time::Hertz;
use crate::typelevel::{Decrement, Increment, PrivateDecrement, PrivateIncrement, Sealed};

use super::osculp32k::OscUlp32kId;
use super::{Enabled, Source};

//==============================================================================
// Tokens
//==============================================================================

/// Singleton token that can be exchanged for [`Xosc32kBase`]
///
/// As explained in the [`clock` module documentation](super), instances of
/// various `Token` types can be exchanged for actual clock types. They
/// typically represent clocks that are disabled at power-on reset.
///
/// The [`Xosc32kBase`] clock is disabled at power-on reset. To use it, you must
/// first exchange the token for an actual clock with
/// [`Xosc32kBase::from_clock`] or [`Xosc32kBase::from_crystal`].
pub struct Xosc32kBaseToken(());

/// Singleton token that can be exchanged for [`Xosc1k`]
///
/// As explained in the [`clock` module documentation](super), instances of
/// various `Token` types can be exchanged for actual clock types. They
/// typically represent clocks that are disabled at power-on reset.
///
/// The [`Xosc1k`] clock is disabled at power-on reset. To use it, you must
/// first exchange the token for an actual clock with [`Xosc1k::enable`].
pub struct Xosc1kToken(());

/// Singleton token that can be exchanged for [`Xosc32k`]
///
/// As explained in the [`clock` module documentation](super), instances of
/// various `Token` types can be exchanged for actual clock types. They
/// typically represent clocks that are disabled at power-on reset.
///
/// The [`Xosc32k`] clock is disabled at power-on reset. To use it, you must
/// first exchange the token for an actual clock with [`Xosc32k::enable`].
pub struct Xosc32kToken(());

/// Singleton token that can be exchanged for [`Xosc32kCfd`]
///
/// As explained in the [module-level documentation](self), clock failure
/// detection can be used even after the `XOSC32K` register has been write
/// locked. For that reason, users control clock failure detection through the
/// dedicated [`Xosc32kCfd`] type.
///
/// Clock failure detection is disabled at power-on reset. To use it, you must
/// first enable it by exchanging the token with [`Xosc32kCfd::enable`].
pub struct Xosc32kCfdToken(());

/// Set of tokens representing the disabled XOSC32K clocks power-on reset
pub struct Xosc32kTokens {
    pub base: Xosc32kBaseToken,
    pub xosc1k: Xosc1kToken,
    pub xosc32k: Xosc32kToken,
    pub cfd: Xosc32kCfdToken,
}

impl Xosc32kTokens {
    /// Create the set of tokens
    ///
    /// # Safety
    ///
    /// There must never be more than one instance of these tokens at any given
    /// time. See the notes on `Token` types and memory safety in the root of
    /// the `clock` module for more details.
    pub(super) unsafe fn new() -> Self {
        Self {
            base: Xosc32kBaseToken(()),
            xosc1k: Xosc1kToken(()),
            xosc32k: Xosc32kToken(()),
            cfd: Xosc32kCfdToken(()),
        }
    }
}

impl Xosc32kBaseToken {
    #[inline]
    fn status(&self) -> status::R {
        // Safety: We are only reading from the `STATUS` register, so there is
        // no risk of memory corruption.
        unsafe { (*crate::pac::OSC32KCTRL::PTR).status.read() }
    }

    /// Check whether the XOSC32K is stable and ready
    #[inline]
    fn is_ready(&self) -> bool {
        self.status().xosc32krdy().bit()
    }

    #[inline]
    fn xosc32k(&self) -> &XOSC32K {
        // Safety: The `Xosc32kBaseToken` has exclusive access to the `XOSC32K`
        // register. See the notes on `Token` types and memory safety in the
        // root of the `clock` module for more details.
        unsafe { &(*crate::pac::OSC32KCTRL::PTR).xosc32k }
    }

    /// Reset the XOSC32K register
    #[inline]
    fn reset(&mut self) {
        self.xosc32k().reset();
    }

    /// Set most of the fields in the XOSC32K register
    #[inline]
    fn set_xosc32k(&mut self, settings: Settings) {
        let xtalen = settings.mode == DynMode::CrystalMode;
        self.xosc32k().modify(|_, w| {
            w.cgm().variant(settings.cgm.into());
            w.startup().variant(settings.start_up.into());
            w.ondemand().bit(settings.on_demand);
            w.runstdby().bit(settings.run_standby);
            w.xtalen().bit(xtalen)
        });
    }

    /// Disable the XOSC32K
    #[inline]
    fn enable(&mut self) {
        self.xosc32k().modify(|_, w| w.enable().set_bit());
    }

    /// Disable the XOSC32K
    #[inline]
    fn disable(&mut self) {
        self.xosc32k().modify(|_, w| w.enable().clear_bit());
    }

    /// Enable the 1 kHz output
    #[inline]
    fn enable_1k(&mut self) {
        self.xosc32k().modify(|_, w| w.en1k().set_bit());
    }

    /// Disable the 1 kHz output
    #[inline]
    fn disable_1k(&mut self) {
        self.xosc32k().modify(|_, w| w.en1k().clear_bit());
    }

    /// Enable the 32 kHz output
    #[inline]
    fn enable_32k(&mut self) {
        self.xosc32k().modify(|_, w| w.en32k().set_bit());
    }

    /// Disable the 32 kHz output
    #[inline]
    fn disable_32k(&mut self) {
        self.xosc32k().modify(|_, w| w.en32k().clear_bit());
    }

    /// Enable the write lock
    #[inline]
    fn write_lock(&mut self) {
        self.xosc32k().modify(|_, w| w.wrtlock().set_bit());
    }
}

impl Xosc32kCfdToken {
    #[inline]
    fn status(&self) -> status::R {
        // Safety: We are only reading from the `STATUS` register, so there is
        // no risk of memory corruption.
        unsafe { (*crate::pac::OSC32KCTRL::PTR).status.read() }
    }

    /// Check whether the XOSC32K has triggered failure detection
    #[inline]
    fn has_failed(&self) -> bool {
        self.status().xosc32kfail().bit()
    }

    /// Check whether the XOSC32K has been switched to the safe clock
    #[inline]
    fn is_switched(&self) -> bool {
        self.status().xosc32ksw().bit()
    }

    #[inline]
    fn cfdctrl(&self) -> &CFDCTRL {
        // Safety: The `Xosc32kCfdToken` has exclusive access to the `CFDCTRL`
        // register. See the notes on `Token` types and memory safety in the
        // root of the `clock` module for more details.
        unsafe { &(*crate::pac::OSC32KCTRL::PTR).cfdctrl }
    }

    /// Enable clock failure detection and set the safe clock divider
    #[inline]
    fn enable(&mut self, div: SafeClockDiv) {
        self.cfdctrl().modify(|_, w| {
            w.cfdpresc().bit(div.into());
            w.cfden().set_bit()
        });
    }

    /// Disable clock failure detection
    #[inline]
    fn disable(&mut self) {
        self.cfdctrl().modify(|_, w| w.cfden().clear_bit());
    }

    /// Switch from the safe clock back to the XOSC32K clock/oscillator
    ///
    /// This bit is cleared by the hardware after successfully switching back
    #[inline]
    fn switch_back(&mut self) {
        self.cfdctrl().modify(|_, w| w.swback().set_bit());
    }
}

//==============================================================================
// Settings
//==============================================================================

// Collection of XOSC32K register fields
//
// All of these fields are set in a single write to XOSC32K during the call to
// [`Xosc32kBase::enable`]. The remaining fields are only modified after it has
// been enabled.
#[derive(Clone, Copy)]
struct Settings {
    start_up: StartUpDelay,
    cgm: ControlGainMode,
    on_demand: bool,
    run_standby: bool,
    mode: DynMode,
}

//==============================================================================
// XIn32 & XOut32
//==============================================================================

/// Type alias for the XOSC32K input [`Pin`]
pub type XIn32 = Pin<PA00, FloatingDisabled>;

/// Type alias for the XOSC32K output [`Pin`]
pub type XOut32 = Pin<PA01, FloatingDisabled>;

//==============================================================================
// SafeClockDiv
//==============================================================================

/// Division factor for the safe clock prescaler
///
/// If an XOSC32K clock failure is detected, the hardware will switch to a safe
/// clock derived from the [`OscUlp32k`]. This enum sets the divider between it
/// and the safe clock frequency. The divider can be 1 or 2.
///
///[`OscUlp32k`]: super::osculp32k::OscUlp32k
#[repr(u8)]
#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub enum SafeClockDiv {
    #[default]
    Div1,
    Div2,
}

impl From<SafeClockDiv> for bool {
    fn from(div: SafeClockDiv) -> Self {
        match div {
            SafeClockDiv::Div1 => false,
            SafeClockDiv::Div2 => true,
        }
    }
}

//==============================================================================
// StartUpDelay
//==============================================================================

/// Start up delay before continuous monitoring takes effect
///
/// After a hard reset or waking from sleep, the XOSC32K output will remained
/// masked for the start up period, to ensure an unstable clock is not
/// propagated into the digital logic.
///
/// The start up delay is counted using the [`OscUlp32k`] clock.
///
/// [`OscUlp32k`]: super::osculp32k::OscUlp32k
#[repr(u8)]
#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub enum StartUpDelay {
    #[default]
    Delay63ms,
    Delay125ms,
    Delay500ms,
    Delay1s,
    Delay2s,
    Delay4s,
    Delay8s,
}

impl From<StartUpDelay> for STARTUP_A {
    fn from(delay: StartUpDelay) -> Self {
        match delay {
            StartUpDelay::Delay63ms => STARTUP_A::CYCLE2048,
            StartUpDelay::Delay125ms => STARTUP_A::CYCLE4096,
            StartUpDelay::Delay500ms => STARTUP_A::CYCLE16384,
            StartUpDelay::Delay1s => STARTUP_A::CYCLE32768,
            StartUpDelay::Delay2s => STARTUP_A::CYCLE65536,
            StartUpDelay::Delay4s => STARTUP_A::CYCLE131072,
            StartUpDelay::Delay8s => STARTUP_A::CYCLE262144,
        }
    }
}

//==============================================================================
// ControlGainMode
//==============================================================================

/// Gain mode for the XOSC32K control loop
///
/// The XOSC32K crystal oscillator control loop has a configurable gain to allow
/// users to trade power for speed and stability.
#[derive(Copy, Clone, Default, PartialEq, Eq)]
pub enum ControlGainMode {
    #[default]
    Standard,
    HighSpeed,
}

impl From<ControlGainMode> for CGM_A {
    fn from(cgm: ControlGainMode) -> Self {
        match cgm {
            ControlGainMode::Standard => CGM_A::XT,
            ControlGainMode::HighSpeed => CGM_A::HS,
        }
    }
}

//==============================================================================
// DynMode
//==============================================================================

/// Value-level enum identifying one of two possible XOSC32K operating modes
///
/// The XOSC32K clock can be sourced from either an external clock or crystal
/// oscillator. The variants of this enum identify one of these two possible
/// operating modes.
///
/// `DynMode` is the value-level equivalent of [`Mode`].
#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub enum DynMode {
    #[default]
    ClockMode,
    CrystalMode,
}

//==============================================================================
// Mode
//==============================================================================

/// Type-level enum for the XOSC32K operation mode
///
/// The XOSC32K clock can be sourced from either an external clock or a crystal
/// oscillator. This type-level `enum` provides two type-level variants,
/// [`ClockMode`] and [`CrystalMode`], representing these operating modes.
///
/// `Mode` is the type-level equivalent of [`DynMode`]. See the documentation on
/// [type-level programming] and specifically [type-level enums] for more
/// details.
///
/// [type-level programming]: crate::typelevel
/// [type-level enums]: crate::typelevel#type-level-enums
pub trait Mode: Sealed {
    const DYN: DynMode;
    #[doc(hidden)]
    type Pins;
}

/// Type-level variant of the XOSC32K operating [`Mode`]
///
/// In this `Mode`, the XOSC32K clock will be sourced from an external clock.
///
/// See the documentation on [type-level programming] and specifically
/// [type-level enums] for more details.
///
/// [type-level programming]: crate::typelevel
/// [type-level enums]: crate::typelevel#type-level-enums
pub enum ClockMode {}
impl Sealed for ClockMode {}
impl Mode for ClockMode {
    const DYN: DynMode = DynMode::ClockMode;
    type Pins = XIn32;
}

/// Type-level variant of the XOSC32K operating [`Mode`]
///
/// In this `Mode`, the XOSC32K clock will be sourced from a crystal oscillator.
///
/// See the documentation on [type-level programming] and specifically
/// [type-level enums] for more details.
///
/// [type-level programming]: crate::typelevel
/// [type-level enums]: crate::typelevel#type-level-enums
pub enum CrystalMode {}
impl Sealed for CrystalMode {}
impl Mode for CrystalMode {
    const DYN: DynMode = DynMode::CrystalMode;
    type Pins = (XIn32, XOut32);
}

//==============================================================================
// Xosc32kBase
//==============================================================================

/// XOSC32K base clock, which feeds the [`Xosc1k`] and [`Xosc32k`] clocks
///
/// The XOSC32K peripheral has two possible clock outputs, one at 32 kHz and
/// another at 1 kHz. This structure is represented in the type system as a set
/// of three clocks forming a small clock tree. The [`Xosc32kBase`] clock
/// represents the configurable base oscillator that feeds the optional
/// [`Xosc1k`] and [`Xosc32k`] output clocks. See the
/// [module-level documentation](super) for details and examples.
pub struct Xosc32kBase<M: Mode> {
    token: Xosc32kBaseToken,
    pins: M::Pins,
    settings: Settings,
}

/// The [`Enabled`] [`Xosc32kBase`] clock
///
/// As described in the [`clock` module documentation](super), the [`Enabled`]
/// wrapper implements compile-time clock tree safety by tracking the number of
/// clocks consuming the [`Xosc32kBase`] clock and restricts access to the
/// underlying type to prevent misuse.
///
/// As with [`Enabled`], the default value for `N` is `U0`; if left unspecified,
/// the counter is assumed to be zero.
pub type EnabledXosc32kBase<M, N = U0> = Enabled<Xosc32kBase<M>, N>;

impl Xosc32kBase<ClockMode> {
    /// Create the [`Xosc32kBase`] clock from an external clock, taking
    /// ownership of the [`XIn32`] [`Pin`]
    ///
    /// Creating an [`Xosc32kBase`] clock does not modify any of the hardware
    /// registers. It only creates a struct to track the configuration. The
    /// configuration data is stored until the user calls [`enable`]. At that
    /// point, all of the registers are written according to the initialization
    /// procedures specified in the datasheet, and an [`EnabledXosc32kBase`]
    /// clock is returned. The `Xosc32kBase` clock is not active or useful until
    /// that point.
    ///
    /// [`enable`]: Xosc32kBase::enable
    #[inline]
    pub fn from_clock(token: Xosc32kBaseToken, xin32: impl Into<XIn32>) -> Self {
        let pins = xin32.into();
        Self::new(token, pins)
    }

    /// Consume the [`Xosc32kBase`] and release the [`Xosc32kBaseToken`] and
    /// [`XIn32`] [`Pin`]
    #[inline]
    pub fn free(self) -> (Xosc32kBaseToken, XIn32) {
        (self.token, self.pins)
    }
}

impl Xosc32kBase<CrystalMode> {
    /// Create the [`Xosc32kBase`] clock from an external crystal oscillator,
    /// taking ownership of the [`XIn32`] and [`XOut32`] [`Pin`]s.
    ///
    /// Creating an [`Xosc32kBase`] clock does not modify any of the hardware
    /// registers. It only creates a struct to track the configuration. The
    /// configuration data is stored until the user calls [`enable`]. At that
    /// point, all of the registers are written according to the initialization
    /// procedures specified in the datasheet, and an [`EnabledXosc32kBase`]
    /// clock is returned. The `Xosc32kBase` is not active or useful until that
    /// point.
    ///
    /// [`enable`]: Xosc32kBase::enable
    #[inline]
    pub fn from_crystal(
        token: Xosc32kBaseToken,
        xin32: impl Into<XIn32>,
        xout32: impl Into<XOut32>,
    ) -> Self {
        let pins = (xin32.into(), xout32.into());
        Self::new(token, pins)
    }

    /// Consume the [`Xosc32kBase`] and release the [`Xosc32kBaseToken`],
    /// [`XIn32`] and [`XOut32`] [`Pin`]s
    #[inline]
    pub fn free(self) -> (Xosc32kBaseToken, XIn32, XOut32) {
        let (xin32, xout32) = self.pins;
        (self.token, xin32, xout32)
    }

    /// Set the crystal oscillator [`ControlGainMode`]
    #[inline]
    pub fn control_gain_mode(mut self, cgm: ControlGainMode) -> Self {
        self.settings.cgm = cgm;
        self
    }
}

impl<M: Mode> Xosc32kBase<M> {
    #[inline]
    fn new(token: Xosc32kBaseToken, pins: M::Pins) -> Self {
        let settings = Settings {
            start_up: StartUpDelay::Delay63ms,
            cgm: ControlGainMode::Standard,
            on_demand: true,
            run_standby: false,
            mode: M::DYN,
        };
        Self {
            token,
            pins,
            settings,
        }
    }

    /// Set the start up delay before the [`Xosc32kBase`] clock is unmasked and
    /// continuously monitored
    ///
    /// During the start up period, the [`Xosc32kBase`] clock is masked to
    /// prevent clock instability from propagating to the digital logic. During
    /// this time, clock failure detection is disabled.
    #[inline]
    pub fn start_up_delay(mut self, delay: StartUpDelay) -> Self {
        self.settings.start_up = delay;
        self
    }

    /// Control the XOSC32K on-demand behavior
    ///
    /// When the on-demand is enabled, the XOSC32K clocks will only run in Idle
    /// or Standby sleep modes if it is requested by a peripheral. Otherwise,
    /// its behavior is dependent on the run-standby setting.
    #[inline]
    pub fn on_demand(mut self, on_demand: bool) -> Self {
        self.settings.on_demand = on_demand;
        self
    }

    /// Control the XOSC32K behavior in Standby sleep mode
    ///
    /// When `RUNSTDBY` is disabled, the XOSC32K clocks will never run in
    /// Standby sleep mode unless `ONDEMAND` is enabled and a clock is requested
    /// by a peripheral.
    ///
    /// When `RUNSTDBY` is enabled, the `Xosc` will run in Standby sleep mode,
    /// but it can still be disabled if `ONDEMAND` is enabled and a clock is not
    /// requested.
    #[inline]
    pub fn run_standby(mut self, run_standby: bool) -> Self {
        self.settings.run_standby = run_standby;
        self
    }

    /// Freeze the XOSC32K configuration until power-on reset
    ///
    /// This function sets the write-lock bit, which freezes the XOSC32K
    /// configuration at the hardware level until power-on reset. At the API
    /// level, it also consumes and drops the [`Xosc32kBase`], which prevents
    /// any further modifications.
    ///
    /// **NOTE:** Because the `Xosc32kBase` is not yet enabled, calling this
    /// method will lock both the [`Xosc1k`] and [`Xosc32k`] in their disabled
    /// state.
    #[inline]
    pub fn write_lock(mut self) {
        self.token.write_lock();
    }

    /// Enable the [`Xosc32kBase`] clock, so that it can be used as a clock
    /// [`Source`] for the [`Xosc1k`] and [`Xosc32k`] clocks
    ///
    /// As mentioned when creating a new `Xosc32kBase`, no hardware registers
    /// are actually modified until this call. Rather, the desired configuration
    /// is stored internally, and the `Xosc32kBase` is initialized and
    /// configured here according to the datasheet.
    ///
    /// The returned value is an [`EnabledXosc32kBase`] that can be used as a
    /// clock [`Source`] for the [`Xosc1k`] and [`Xosc32k`] clocks.
    #[inline]
    pub fn enable(mut self) -> EnabledXosc32kBase<M> {
        self.token.reset();
        self.token.set_xosc32k(self.settings);
        self.token.enable();
        Enabled::new(self)
    }
}

impl<M: Mode> EnabledXosc32kBase<M> {
    /// Disable the [`Xosc32kBase`] clock
    ///
    /// This method is only implemented for `N = U0`, which means the clock can
    /// only be disabled when no other clocks consume this [`Xosc32kBase`]
    /// clock.
    #[inline]
    pub fn disable(mut self) -> Xosc32kBase<M> {
        self.0.token.disable();
        self.0
    }
}

impl<M: Mode, N> EnabledXosc32kBase<M, N> {
    /// Check whether the XOSC32K is stable and ready to be used as a clock
    /// [`Source`]
    #[inline]
    pub fn is_ready(&self) -> bool {
        self.0.token.is_ready()
    }

    /// Freeze the XOSC32K configuration until power-on reset
    ///
    /// This function sets the write-lock bit, which freezes the XOSC32K
    /// configuration at the hardware level until power-on reset. At the API
    /// level, it also consumes and drops the [`Xosc32kBase`] clock, which
    /// prevents any further modifications.
    #[inline]
    pub fn write_lock(mut self) {
        self.0.token.write_lock();
    }
}

//==============================================================================
// Xosc32kCfd
//==============================================================================

/// Clock failure detection interface for the XOSC32K peripheral
///
/// The XOSC32K peripheral provides a hardware method to continuously monitor
/// the clock to verify it is still running. In the event of a failure, the
/// output will be switched to a "safe clock" derived from the [`OscUlp32k`].
/// The XOSC32K peripheral provides a prescaler to optionally divide the
/// `OscUlp32k` by two.
///
/// Note that clock failure is triggered when four safe clock periods pass
/// without seeing a rising & falling edge pair on the XOSC32K clock. Once
/// failure is detected, the corresponding bit in the `STATUS` register will
/// go high and an interrupt will be triggered.
///
/// If the external clock can be fixed, the XOSC32K clock can be switched back
/// using [`Xosc32kCfd::switch_back`].
///
/// Because the safe clock makes use of the `OscUlp32k`, the `Xosc32kCfd` must
/// register as a consumer of the [`EnabledOscUlp32k`] and [`Increment`] its
/// counter.
///
/// [`OscUlp32k`]: super::osculp32k::OscUlp32k
/// [`EnabledOscUlp32k`]: super::osculp32k::EnabledOscUlp32k
pub struct Xosc32kCfd {
    token: Xosc32kCfdToken,
}

impl Xosc32kCfd {
    /// Enable continuous monitoring of the XOSC32K for clock failure
    ///
    /// Because the safe clock makes use of the [`OscUlp32k`], the `Xosc32kCfd`
    /// must register as a consumer of the [`EnabledOscUlp32k`] and
    /// [`Increment`] its counter.
    ///
    /// [`OscUlp32k`]: super::osculp32k::OscUlp32k
    /// [`EnabledOscUlp32k`]: super::osculp32k::EnabledOscUlp32k
    #[inline]
    pub fn enable<S>(
        mut token: Xosc32kCfdToken,
        osc_ulp_32k: S,
        div: SafeClockDiv,
    ) -> (Xosc32kCfd, S::Inc)
    where
        S: Source<Id = OscUlp32kId> + Increment,
    {
        token.enable(div);
        (Self { token }, osc_ulp_32k.inc())
    }

    /// Check whether the XOSC32K has triggered clock failure detection
    ///
    /// Failure is triggered when four safe clock periods pass without seeing a
    /// rising & falling edge pair on the XOSC32K clock.
    #[inline]
    pub fn has_failed(&self) -> bool {
        self.token.has_failed()
    }

    /// Check whether the XOSC32K has been switched to the safe clock
    ///
    /// Returns `true` if the XOSC32K has been switched to the safe clock.
    #[inline]
    pub fn is_switched(&self) -> bool {
        self.token.is_switched()
    }

    /// Attempt to switch from the safe clock back to the external clock
    ///
    /// This function will set the switch back bit (`SWBACK`) in the `CFDCTRL`
    /// register. Once the hardware has successfully switched back, this bit
    /// will be automatically cleared.
    ///
    /// Users can check whether switching back was successful by checking the
    /// `STATUS` register with [`Xosc32kCfd::is_switched`].
    #[inline]
    pub fn switch_back(&mut self) {
        self.token.switch_back()
    }

    /// Disable continuous monitoring of the XOSC32K for clock failure
    ///
    /// Once failure monitoring is disabled, the [`OscUlp32k`] is no longer used
    /// as the safe clock, so the [`EnabledOscUlp32k`] counter can be
    /// [`Decrement`]ed.
    ///
    /// [`OscUlp32k`]: super::osculp32k::OscUlp32k
    /// [`EnabledOscUlp32k`]: super::osculp32k::EnabledOscUlp32k
    #[inline]
    pub fn disable<S>(mut self, osc_ulp_32k: S) -> (Xosc32kCfdToken, S::Dec)
    where
        S: Source<Id = OscUlp32kId> + Decrement,
    {
        self.token.disable();
        (self.token, osc_ulp_32k.dec())
    }
}

//==============================================================================
// Ids
//==============================================================================

/// Type representing the identity of the [`Xosc1k`] clock
///
/// See the discussion on [`Id` types](super#id-types) for more information.
pub enum Xosc1kId {}

impl Sealed for Xosc1kId {}

/// Type representing the identity of the [`Xosc32k`] clock
///
/// See the discussion on [`Id` types](super#id-types) for more information.
pub enum Xosc32kId {}

impl Sealed for Xosc32kId {}

//==============================================================================
// Xosc1k
//==============================================================================

/// Clock representing the 1 kHz output of the [`Xosc32kBase`] clock
///
/// The XOSC32K peripheral has two possible clock outputs, one at 32 kHz and
/// another at 1 kHz. This structure is represented in the type system as a set
/// of three clocks forming a small clock tree. The [`Xosc1k`] clock is derived
/// from the [`Xosc32kBase`] clock. See the [module-level documentation](super)
/// for details and examples.
pub struct Xosc1k {
    token: Xosc1kToken,
}

/// The [`Enabled`] [`Xosc1k`] clock
///
/// As described in the [`clock` module documentation](super), the [`Enabled`]
/// wrapper implements compile-time clock tree safety by tracking the number of
/// clocks consuming the [`Xosc1k`] clock and restricts access to the underlying
/// type to prevent misuse.
///
/// As with [`Enabled`], the default value for `N` is `U0`; if left unspecified,
/// the counter is assumed to be zero.
pub type EnabledXosc1k<N = U0> = Enabled<Xosc1k, N>;

impl Xosc1k {
    /// Enable 1 kHz output from the [`Xosc32kBase`] clock
    ///
    /// This will [`Increment`] the [`EnabledXosc32kBase`] counter.
    #[inline]
    pub fn enable<M, N>(
        token: Xosc1kToken,
        mut base: EnabledXosc32kBase<M, N>,
    ) -> (EnabledXosc1k, EnabledXosc32kBase<M, N::Inc>)
    where
        M: Mode,
        N: Increment,
    {
        base.0.token.enable_1k();
        (Enabled::new(Self { token }), base.inc())
    }
}

impl EnabledXosc1k {
    /// Disable 1 kHz output from the [`Xosc32kBase`] clock
    ///
    /// This will [`Decrement`] the [`EnabledXosc32kBase`] counter.
    #[inline]
    pub fn disable<M, N>(
        self,
        mut base: EnabledXosc32kBase<M, N>,
    ) -> (Xosc1kToken, EnabledXosc32kBase<M, N::Dec>)
    where
        M: Mode,
        N: Decrement,
    {
        base.0.token.disable_1k();
        (self.0.token, base.dec())
    }
}

impl<N> Source for EnabledXosc1k<N> {
    type Id = Xosc1kId;

    #[inline]
    fn freq(&self) -> Hertz {
        1024.Hz()
    }
}

//==============================================================================
// Xosc32k
//==============================================================================

/// Clock representing the 32 kHz output of the [`Xosc32kBase`] clock
///
/// The XOSC32K peripheral has two possible clock outputs, one at 32 kHz and
/// another at 1 kHz. This structure is represented in the type system as a set
/// of three clocks forming a small clock tree. The [`Xosc32k`] clock is derived
/// from the [`Xosc32kBase`] clock. See the [module-level documentation](super)
/// for details and examples.
pub struct Xosc32k {
    token: Xosc32kToken,
}

/// The [`Enabled`] [`Xosc32k`] clock
///
/// As described in the [`clock` module documentation](super), the [`Enabled`]
/// wrapper implements compile-time clock tree safety by tracking the number of
/// clocks consuming the [`Xosc32k`] clock and restricts access to the
/// underlying type to prevent misuse.
///
/// As with [`Enabled`], the default value for `N` is `U0`; if left unspecified,
/// the counter is assumed to be zero.
pub type EnabledXosc32k<N = U0> = Enabled<Xosc32k, N>;

impl Xosc32k {
    /// Enable 32 kHz output from the [`Xosc32kBase`] clock
    ///
    /// This will [`Increment`] the [`EnabledXosc32kBase`] counter.
    #[inline]
    pub fn enable<M, N>(
        token: Xosc32kToken,
        mut base: EnabledXosc32kBase<M, N>,
    ) -> (EnabledXosc32k, EnabledXosc32kBase<M, N::Inc>)
    where
        M: Mode,
        N: Increment,
    {
        base.0.token.enable_32k();
        (Enabled::new(Self { token }), base.inc())
    }
}

impl EnabledXosc32k {
    /// Disable 1 kHz output from the [`Xosc32kBase`] clock
    ///
    /// This will [`Decrement`] the [`EnabledXosc32kBase`] counter.
    #[inline]
    pub fn disable<M, N>(
        self,
        mut base: EnabledXosc32kBase<M, N>,
    ) -> (Xosc32kToken, EnabledXosc32kBase<M, N::Dec>)
    where
        M: Mode,
        N: Decrement,
    {
        base.0.token.disable_32k();
        (self.0.token, base.dec())
    }
}

impl<N> Source for EnabledXosc32k<N> {
    type Id = Xosc32kId;

    #[inline]
    fn freq(&self) -> Hertz {
        32_768.Hz()
    }
}
