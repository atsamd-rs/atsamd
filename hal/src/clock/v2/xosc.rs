//! # External multipurpose crystal oscillator controller
//!
//! ## Overview
//!
//! The `xosc` module provides access to the two external crystal oscillator
//! controllers (XOSCs) within the `OSCCTRL` peripheral.
//!
//! Each XOSC peripheral can operate in two [`Mode`]s. It can accept an external
//! clock or can interface with an crystal oscillator. In both cases, the clock
//! must be in the 8-48 MHz range.
//!
//! When used with an external clock, only one GPIO [`Pin`] is required, but
//! when used with a crystal oscillator, two GPIO `Pin`s are required. The
//! [`XIn`] `Pin` is used in both `Mode`s, while the [`XOut`] `Pin` is only
//! used in [`CrystalMode`].
//!
//! When operating in [`CrystalMode`], the XOSC peripheral provides several
//! configuration options to increase stability or reduce power consumption of
//! the crystal.
//!
//! The XOSC peripheral can also detect failure of the clock or crystal; and if
//! failure occurs, it can automatically switch to a safe, backup clock derived
//! from the [DFLL].
//!
//! Creating and configuring an [`Xosc`] proceeds according to the principles
//! outlined in the [`clock` module documentation]. It is best shown with an
//! example.
//!
//! ## Example
//!
//! Let's start by using [`clock_system_at_reset`] to access the HAL clocking
//! structs. We'll also need access to the GPIO [`Pins`].
//!
//! ```no_run
//! use atsamd_hal::{
//!     clock::v2::{
//!         clock_system_at_reset,
//!         xosc::{CrystalCurrent, SafeClockDiv, StartUpDelay, Xosc},
//!     },
//!     gpio::Pins,
//!     pac::Peripherals,
//!     time::U32Ext,
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
//! Next, we can create and configure the [`Xosc`] in one long chain of methods,
//! using the provided builder API. The final call to [`Xosc::enable`] yields an
//! [`EnabledXosc`] that can act as a clock [`Source`] for other clocks in the
//! tree.
//!
//! ```no_run
//! # use atsamd_hal::{
//! #     clock::v2::{
//! #         clock_system_at_reset,
//! #         xosc::{CrystalCurrent, SafeClockDiv, StartUpDelay, Xosc},
//! #     },
//! #     gpio::Pins,
//! #     pac::Peripherals,
//! #     time::U32Ext,
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
//! let mut xosc = Xosc::from_crystal(tokens.xosc0, pins.pa14, pins.pa15, 20.mhz())
//!     .current(CrystalCurrent::Medium)
//!     .loop_control(true)
//!     .low_buf_gain(true)
//!     .start_up_delay(StartUpDelay::Delay488us)
//!     .enable();
//! ```
//!
//! We start by calling [`Xosc::from_crystal`], and we provide the corresponding
//! [`XIn`] and [`XOut`] [`Pin`]s, as well as the nominal crystal frequency. We
//! then set the [`CrystalCurrent`] level to `Medium`. The default current level
//! for a 20 MHz signal is actually `High`, but we opt for a lower current under
//! the assumption that our crystal's capacitive load is small. Next, we turn on
//! automatic loop control, which should save power, but we also set
//! `LOWBUFGAIN` to `1`. Counterintuitively, this actually _increases_ the
//! crystal amplitude, which increases power consumption, but it also improves
//! stability. We then apply a 488 μs start up delay, to allow the clock to
//! stabilize before it is applied to any logic. Finally, we enable the `Xosc`.
//!
//! Next, we wait until the `Xosc` is stable and ready to be used as a clock
//! [`Source`].
//!
//! ```no_run
//! # use atsamd_hal::{
//! #     clock::v2::{
//! #         clock_system_at_reset,
//! #         xosc::{CrystalCurrent, SafeClockDiv, StartUpDelay, Xosc},
//! #     },
//! #     gpio::Pins,
//! #     pac::Peripherals,
//! #     time::U32Ext,
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
//! # let mut xosc = Xosc::from_crystal(tokens.xosc0, pins.pa14, pins.pa15, 20.mhz())
//! #     .current(CrystalCurrent::Medium)
//! #     .loop_control(true)
//! #     .low_buf_gain(true)
//! #     .start_up_delay(StartUpDelay::Delay488us)
//! #     .enable();
//! while !xosc.is_ready() {}
//! ```
//!
//! Once the clock is stable, we can also enable failure detection. To do so, we
//! must provide the [`EnabledDfll`] to act as the backup safe clock. We can
//! also select a divider for the safe clock, so that it loosely matches the
//! `Xosc` frequency. In thise case, we divide the 48 MHz [`Dfll`] down to
//! 24 MHz, which is the closest option to 20 MHz.
//!
//! ```no_run
//! # use atsamd_hal::{
//! #     clock::v2::{
//! #         clock_system_at_reset,
//! #         xosc::{CrystalCurrent, SafeClockDiv, StartUpDelay, Xosc},
//! #     },
//! #     gpio::Pins,
//! #     pac::Peripherals,
//! #     time::U32Ext,
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
//! # let mut xosc = Xosc::from_crystal(tokens.xosc0, pins.pa14, pins.pa15, 20.mhz())
//! #     .current(CrystalCurrent::Medium)
//! #     .loop_control(true)
//! #     .low_buf_gain(true)
//! #     .start_up_delay(StartUpDelay::Delay488us)
//! #     .enable();
//! # while !xosc.is_ready() {}
//! xosc.enable_failure_detection(clocks.dfll, SafeClockDiv::Div2);
//! ```
//!
//! In the event of a clock failure, the [`Xosc`] would be automatically
//! switched to the safe clock, and [`EnabledXosc::has_failed`] would return
//! true. If the problem were later resolved, the `Xosc` could be switched back
//! to the crystal with [`EnabledXosc::switch_back`].
//!
//! The complete example is provided below.
//!
//! ```no_run
//! use atsamd_hal::{
//!     clock::v2::{
//!         clock_system_at_reset,
//!         xosc::{CrystalCurrent, SafeClockDiv, StartUpDelay, Xosc},
//!     },
//!     gpio::Pins,
//!     pac::Peripherals,
//!     time::U32Ext,
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
//! let mut xosc = Xosc::from_crystal(tokens.xosc0, pins.pa14, pins.pa15, 20.mhz())
//!     .current(CrystalCurrent::Medium)
//!     .loop_control(true)
//!     .low_buf_gain(true)
//!     .start_up_delay(StartUpDelay::Delay488us)
//!     .enable();
//! while !xosc.is_ready() {}
//! xosc.enable_failure_detection(clocks.dfll, SafeClockDiv::Div2);
//! ```
//!
//! [`Pins`]: crate::gpio::Pins
//! [`clock` module documentation]: super
//! [`clock_system_at_reset`]: super::clock_system_at_reset
//! [DFLL]: super::dfll
//! [`Dfll`]: super::dfll::Dfll
//! [`EnabledDfll`]: super::dfll::EnabledDfll

use core::marker::PhantomData;

use typenum::U0;

use crate::pac::oscctrl::{self, XOSCCTRL};

use crate::gpio::{FloatingDisabled, Pin, PinId, PA14, PA15, PB22, PB23};
use crate::time::Hertz;
use crate::typelevel::{Decrement, Increment, Sealed};

use super::dfll::DfllId;
use super::{Enabled, Source};

//==============================================================================
// XoscToken
//==============================================================================

/// Singleton token that can be exchanged for an [`Xosc`]
///
/// As explained in the [`clock` module documentation](super), instances of
/// various `Token` types can be exchanged for actual clock types. They
/// typically represent clocks that are disabled at power-on reset.
///
/// [`XoscToken`]s are no different. Both [`Xosc`]s are disabled at power-on
/// reset. To use an [`Xosc`], you must first exchange the token for an actual
/// clock with [`Xosc::from_clock`] or [`Xosc::from_crystal`].
///
/// [`XoscToken`] is generic over the [`XoscId`], where each corresponding token
/// represents one of the two respective [`Xosc`]s.
pub struct XoscToken<X: XoscId> {
    id: PhantomData<X>,
}

impl<X: XoscId> XoscToken<X> {
    /// Create a new instance of [`XoscToken`]
    ///
    /// # Safety
    ///
    /// Each `XoscToken`s is a singleton. There must never be two simulatenous
    /// instances with the same [`XoscId`]. See the notes on `Token` types and
    /// memory safety in the root of the `clock` module for more details.
    #[inline]
    pub(super) unsafe fn new() -> Self {
        Self { id: PhantomData }
    }

    /// Return a reference to the corresponding XOSCCTRL register
    #[inline]
    fn xoscctrl(&self) -> &XOSCCTRL {
        // Safety: Each `XoscToken` only has access to a mutually exclusive set
        // of registers for the corresponding `XoscId`, and we use a shared
        // reference to the register block. See the notes on `Token` types and
        // memory safety in the root of the `clock` module for more details.
        unsafe { &(*crate::pac::OSCCTRL::PTR).xoscctrl[X::NUM] }
    }

    /// Read the STATUS register
    #[inline]
    fn status(&self) -> oscctrl::status::R {
        // Safety: We are only reading from the `STATUS` register, so there is
        // no risk of memory corruption.
        unsafe { (*crate::pac::OSCCTRL::PTR).status.read() }
    }

    /// Check whether the XOSC is stable and ready
    #[inline]
    fn is_ready(&self) -> bool {
        let mask = 1 << X::NUM;
        self.status().bits() & mask != 0
    }

    /// Check whether the XOSC has triggered failure detection
    #[inline]
    fn has_failed(&self) -> bool {
        let mask = 1 << (X::NUM + 2);
        self.status().bits() & mask != 0
    }

    /// Check whether the XOSC has been switched to the safe clock
    #[inline]
    fn is_switched(&self) -> bool {
        let mask = 1 << (X::NUM + 4);
        self.status().bits() & mask != 0
    }

    /// Reset the XOSCCTRL register
    #[inline]
    fn reset(&self) {
        self.xoscctrl().reset();
    }

    /// Switch from the safe clock back to the XOSC clock/oscillator
    ///
    /// This bit is cleared by the hardware after successfully switching back
    #[inline]
    fn switch_back(&mut self) {
        self.xoscctrl().modify(|_, w| w.swben().set_bit());
    }

    /// Enable clock failure detection and set the safe clock divider
    #[inline]
    fn enable_failure_detection(&mut self, div: SafeClockDiv) {
        // Safety: The divider is guaranteed to be in the valid range 0..16.
        // The PAC is wrong here. It seems to think the field is 4 bits wide and
        // the set of valid values is only 0..8. The `bits` method should really
        // be safe here, just like it is for the `STARTUP` field.
        self.xoscctrl().modify(|_, w| unsafe {
            w.cfdpresc().bits(div as u8);
            w.cfden().set_bit()
        });
    }

    /// Disable clock failure detection
    #[inline]
    fn disable_failure_detection(&mut self) {
        self.xoscctrl().modify(|_, w| w.cfden().clear_bit());
    }

    /// Set most of the fields in the XOSCCTRL register
    #[inline]
    fn set_xoscctrl(&mut self, settings: Settings) {
        let xtalen = settings.mode == DynMode::CrystalMode;
        // Safety: The `IMULT` and `IPTAT` values come from the
        // `CrystalCurrent`, so they are guaranteed to be valid.
        self.xoscctrl().modify(|_, w| unsafe {
            w.startup().bits(settings.start_up as u8);
            w.enalc().bit(settings.loop_control);
            w.imult().bits(settings.current.imult());
            w.iptat().bits(settings.current.iptat());
            w.lowbufgain().bit(settings.low_buf_gain);
            w.ondemand().bit(settings.on_demand);
            w.runstdby().bit(settings.run_standby);
            w.xtalen().bit(xtalen)
        });
    }

    /// Enable the XOSC
    #[inline]
    fn enable(&mut self) {
        self.xoscctrl().modify(|_, w| w.enable().set_bit());
    }

    /// Disable the XOSC
    #[inline]
    fn disable(&mut self) {
        self.xoscctrl().modify(|_, w| w.enable().clear_bit());
    }
}

//==============================================================================
// Settings
//==============================================================================

// Collection of XOSCCTRL register fields
//
// All of these fields are set in a single write to XOSCCTRL during the call to
// [`Xosc::enable`]. The remaining fields are only modified after it has been
// enabled.
#[derive(Clone, Copy)]
struct Settings {
    start_up: StartUpDelay,
    loop_control: bool,
    current: CrystalCurrent,
    low_buf_gain: bool,
    on_demand: bool,
    run_standby: bool,
    mode: DynMode,
}

//==============================================================================
// XoscId
//==============================================================================

/// Type-level enum identifying one of two possible [`Xosc`]s
///
/// The types implementing this trait, i.e. [`Xosc0Id`] and [`Xosc1Id`], are
/// type-level variants of `XoscId`, and they identify one of two possible
/// external crystal oscillators.
///
/// See the documentation on [type-level programming] and specifically
/// [type-level enums] for more details.
///
/// [type-level programming]: crate::typelevel
/// [type-level enums]: crate::typelevel#type-level-enums
pub trait XoscId: Sealed {
    /// Corresponding numeric index
    const NUM: usize;
    /// Corresponding XIN [`PinId`]
    type XIn: PinId;
    /// Corresponding XOUT [`PinId`]
    type XOut: PinId;
}

/// Type-level variant of [`XoscId`] representing the identity of XOSC0
///
/// See the documentation on [type-level programming] and specifically
/// [type-level enums] for more details.
///
/// [type-level programming]: crate::typelevel
/// [type-level enums]: crate::typelevel#type-level-enums
pub enum Xosc0Id {}

impl Sealed for Xosc0Id {}

impl XoscId for Xosc0Id {
    const NUM: usize = 0;
    type XIn = PA14;
    type XOut = PA15;
}

/// Type-level variant of [`XoscId`] representing the identity of XOSC1
///
/// See the documentation on [type-level programming] and specifically
/// [type-level enums] for more details.
///
/// [type-level programming]: crate::typelevel
/// [type-level enums]: crate::typelevel#type-level-enums
pub enum Xosc1Id {}

impl Sealed for Xosc1Id {}

impl XoscId for Xosc1Id {
    const NUM: usize = 1;
    type XIn = PB22;
    type XOut = PB23;
}

//==============================================================================
// XIn & XOut
//==============================================================================

/// Type alias for the [`Xosc`] input [`Pin`]
pub type XIn<X> = Pin<<X as XoscId>::XIn, FloatingDisabled>;

/// Type alias for the [`Xosc`] output [`Pin`]
pub type XOut<X> = Pin<<X as XoscId>::XOut, FloatingDisabled>;

//==============================================================================
// SafeClockDiv
//==============================================================================

/// Division factor for the safe clock prescaler
///
/// If an [`Xosc`] clock failure is detected, the hardware will switch to a safe
/// clock derived from the [`Dfll`]. This enum sets the divider between the
/// 48 MHz DFLL and the safe clock frequency. The divider can be any value of
/// 2^N, with N in the range `0..16`.
///
///[`Dfll`]: super::dfll::Dfll
#[repr(u8)]
#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub enum SafeClockDiv {
    #[default]
    Div1,
    Div2,
    Div4,
    Div8,
    Div16,
    Div32,
    Div64,
    Div128,
    Div256,
    Div512,
    Div1024,
    Div2048,
    Div4096,
    Div8192,
    Div16384,
    Div32768,
}

//==============================================================================
// StartUpDelay
//==============================================================================

/// Start up delay before continuous [`Xosc`] monitoring takes effect
///
/// After a hard reset or waking from sleep, the [`Xosc`] output will remained
/// masked for the start up period, to ensure an unstable clock is not
/// propagated into the digital logic.
///
/// The start up delay is counted using the [`OscUlp32k`] clock, and the delay
/// is equal to 2^N clock cycles, where N is selectable in the range `0..16`.
///
/// [`OscUlp32k`]: super::osculp32k::OscUlp32k
#[repr(u8)]
#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub enum StartUpDelay {
    #[default]
    Delay31us,
    Delay62us,
    Delay122us,
    Delay244us,
    Delay488us,
    Delay977us,
    Delay2ms,
    Delay4ms,
    Delay8ms,
    Delay16ms,
    Delay31ms,
    Delay63ms,
    Delay125ms,
    Delay250ms,
    Delay500ms,
    Delay1s,
}

//==============================================================================
// CrystalCurrent
//==============================================================================

/// Crystal current level
///
/// This struct represents an abstraction over the datasheet table for the
/// `IMULT` and `IPTAT` register fields, which control the current used when an
/// [`Xosc`] is in [`CrystalMode`]
///
/// The variants of this enum are not named according to the explicit frequency
/// range provided in the datasheet. While the datasheet recommends settings for
/// each frequency range, it also acknowledges some flexibility in that choice.
/// Specifically, it notes that users can save power by selecting the next-lower
/// frequency range if the capacitive load is small.
#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub enum CrystalCurrent {
    /// Used only in [`ClockMode`] to represent the default register values
    #[default]
    Zero,
    /// Typically used for 8 MHz oscillators
    Low,
    /// Typically used for 8-16 MHz oscillators
    Medium,
    /// Typically used for 16-24 MHz oscillators
    High,
    /// Typically used for 24-48 MHz oscillators
    ExtraHigh,
}

impl CrystalCurrent {
    #[inline]
    fn imult(&self) -> u8 {
        match self {
            Self::Zero => 0,
            Self::Low => 3,
            Self::Medium => 4,
            Self::High => 5,
            Self::ExtraHigh => 6,
        }
    }

    #[inline]
    fn iptat(&self) -> u8 {
        match self {
            Self::Zero => 0,
            Self::Low => 2,
            Self::Medium => 3,
            Self::High => 3,
            Self::ExtraHigh => 3,
        }
    }
}

//==============================================================================
// DynMode
//==============================================================================

/// Value-level enum identifying one of two possible [`Xosc`] operating modes
///
/// An [`Xosc`] can be sourced from either an external clock or crystal
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

/// Type-level `enum` for the [`Xosc`] operating mode
///
/// An [`Xosc`] can be sourced from either an external clock or a cyrstal
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
    /// Corresponding variant of [`DynMode`]
    const DYN: DynMode;
    #[doc(hidden)]
    type Pins<X: XoscId>;
}

//==============================================================================
// ClockMode
//==============================================================================

/// Type-level variant of the [`Xosc`] operating [`Mode`]
///
/// Represents the [`Xosc`] configured to use an externally provided clock.
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
    type Pins<X: XoscId> = XIn<X>;
}

//==============================================================================
// CrystalMode
//==============================================================================

/// Type-level variant of the [`Xosc`] operating [`Mode`]
///
/// Represents the [`Xosc`] configured to use an external crystal oscillator.
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
    type Pins<X: XoscId> = (XIn<X>, XOut<X>);
}

//==============================================================================
// Xosc
//==============================================================================

/// An external multipurpose crystal oscillator controller
///
/// An `Xosc` interfaces with either an external clock or external crystal
/// oscillator and delivers the resulting clock to the rest of the clock system.
///
/// The type parameter `X` is a [`XoscId`] that determines which of the two
/// instances this `Xosc` represents ([`Xosc0`] or [`Xosc1`]). The type
/// parameter `M` represents the operating [`Mode`], either [`ClockMode`] or
/// [`CrystalMode`].
///
/// On its own, an instance of `Xosc` does not represent an enabled XOSC.
/// Instead, it must first be wrapped with [`Enabled`], which implements
/// compile-time safety of the clock tree.
///
/// Because the terminal call to [`enable`] consumes the `Xosc` and returns an
/// [`EnabledXosc`], the remaining API uses the builder pattern, where each
/// method takes and returns `self` by value, allowing them to be easily
/// chained.
///
/// See the [module-level documentation](self) for an example of creating,
/// configuring and using an `Xosc`.
///
/// [`enable`]: Xosc::enable
pub struct Xosc<X, M>
where
    X: XoscId,
    M: Mode,
{
    token: XoscToken<X>,
    pins: M::Pins<X>,
    freq: Hertz,
    settings: Settings,
}

/// Type alias for the corresponding [`Xosc`]
pub type Xosc0<M> = Xosc<Xosc0Id, M>;

/// Type alias for the corresponding [`Xosc`]
pub type Xosc1<M> = Xosc<Xosc1Id, M>;

/// An [`Enabled`] [`Xosc`]
///
/// As described in the [`clock` module documentation](super), the [`Enabled`]
/// wrapper implements compile-time clock tree safety by tracking the number of
/// consumer clocks and restricting access to the underlying [`Xosc`] to prevent
/// modification while in use.
///
/// As with [`Enabled`], the default value for `N` is `U0`; if left unspecified,
/// the counter is assumed to be zero.
pub type EnabledXosc<X, M, N = U0> = Enabled<Xosc<X, M>, N>;

/// Type alias for the corresponding [`EnabledXosc`]
pub type EnabledXosc0<M, N = U0> = EnabledXosc<Xosc0Id, M, N>;

/// Type alias for the corresponding [`EnabledXosc`]
pub type EnabledXosc1<M, N = U0> = EnabledXosc<Xosc1Id, M, N>;

impl<X: XoscId> Xosc<X, ClockMode> {
    /// Create an [`Xosc`] from an external clock, taking ownership of the
    /// [`XIn`] [`Pin`]
    ///
    /// Creating a [`Xosc`] does not modify any of the hardware registers. It
    /// only creates a struct to track the configuration. The configuration data
    /// is stored until the user calls [`enable`]. At that point, all of the
    /// registers are written according to the initialization procedures
    /// specified in the datasheet, and an [`EnabledXosc`] is returned. The
    /// `Xosc` is not active or useful until that point.
    ///
    /// [`enable`]: Xosc::enable
    #[inline]
    pub fn from_clock(token: XoscToken<X>, xin: impl Into<XIn<X>>, freq: impl Into<Hertz>) -> Self {
        let pins = xin.into();
        Xosc::new(token, pins, freq.into())
    }

    /// Consume the [`Xosc`] and release the [`XoscToken`] and [`XIn`] [`Pin`]
    #[inline]
    pub fn free(self) -> (XoscToken<X>, XIn<X>) {
        (self.token, self.pins)
    }
}

impl<X: XoscId> Xosc<X, CrystalMode> {
    /// Create an [`Xosc`] from an external crystal oscillator, taking ownership
    /// of the [`XIn`] and [`XOut`] [`Pin`]s.
    ///
    /// Creating a [`Xosc`] does not modify any of the hardware registers. It
    /// only creates a struct to track the configuration. The configuration data
    /// is stored until the user calls [`enable`]. At that point, all of the
    /// registers are written according to the initialization procedures
    /// specified in the datasheet, and an [`EnabledXosc`] is returned. The
    /// `Xosc` is not active or useful until that point.
    ///
    /// [`enable`]: Xosc::enable
    #[inline]
    pub fn from_crystal(
        token: XoscToken<X>,
        xin: impl Into<XIn<X>>,
        xout: impl Into<XOut<X>>,
        freq: impl Into<Hertz>,
    ) -> Self {
        let pins = (xin.into(), xout.into());
        Xosc::new(token, pins, freq.into())
    }

    /// Consume the [`Xosc`] and release the [`XoscToken`], [`XIn`] and [`XOut`]
    /// [`Pin`]s
    #[inline]
    pub fn free(self) -> (XoscToken<X>, XIn<X>, XOut<X>) {
        let (xin, xout) = self.pins;
        (self.token, xin, xout)
    }

    /// Set the [`CrystalCurrent`] drive strength
    #[inline]
    pub fn current(mut self, current: CrystalCurrent) -> Self {
        self.settings.current = current;
        self
    }

    /// Toggle automatic loop control
    ///
    /// If enabled, the hardware will automatically adjust the oscillator
    /// amplitude. In most cases, this will lower power consumption.
    #[inline]
    pub fn loop_control(mut self, loop_control: bool) -> Self {
        self.settings.loop_control = loop_control;
        self
    }

    /// Modify the oscillator amplitude when automatic loop control is enabled
    ///
    /// The datasheet name for this setting is very misleading. When automatic
    /// loop control is enabled, setting the `LOWBUFGAIN` field to `1` will
    /// _increase_ the oscillator amplitude by a factor of appoximately 2. This
    /// can help solve stability issues.
    #[inline]
    pub fn low_buf_gain(mut self, low_buf_gain: bool) -> Self {
        self.settings.low_buf_gain = low_buf_gain;
        self
    }
}

impl<X, M> Xosc<X, M>
where
    X: XoscId,
    M: Mode,
{
    #[inline]
    fn new(token: XoscToken<X>, pins: M::Pins<X>, freq: Hertz) -> Self {
        let current = match freq.to_Hz() {
            8_000_000 => CrystalCurrent::Low,
            8_000_001..=16_000_000 => CrystalCurrent::Medium,
            16_000_001..=24_000_000 => CrystalCurrent::High,
            24_000_001..=48_000_000 => CrystalCurrent::ExtraHigh,
            _ => panic!("The XOSC input frequency must be 8-48 MHz"),
        };
        let current = match M::DYN {
            DynMode::ClockMode => CrystalCurrent::Zero,
            DynMode::CrystalMode => current,
        };
        let settings = Settings {
            start_up: StartUpDelay::Delay31us,
            loop_control: false,
            current,
            low_buf_gain: false,
            on_demand: true,
            run_standby: false,
            mode: M::DYN,
        };
        Self {
            token,
            pins,
            freq,
            settings,
        }
    }

    /// Return the clock or crystal frequency
    #[inline]
    pub fn freq(&self) -> Hertz {
        self.freq
    }

    /// Set the start up delay before the [`Xosc`] is unmasked and continuously
    /// monitored
    ///
    /// During the start up period, the [`Xosc`] is masked to prevent clock
    /// instability from propagating to the digital logic. During this time,
    /// clock failure detection is disabled.
    #[inline]
    pub fn start_up_delay(mut self, delay: StartUpDelay) -> Self {
        self.settings.start_up = delay;
        self
    }

    /// Control the [`Xosc`] on-demand behavior
    ///
    /// When the on-demand is enabled, the [`Xosc`] will only run in Idle or
    /// Standby sleep modes if it is requested by a peripheral. Otherwise, its
    /// behavior is dependent on the run-standby setting.
    #[inline]
    pub fn on_demand(mut self, on_demand: bool) -> Self {
        self.settings.on_demand = on_demand;
        self
    }

    /// Control the [`Xosc`] behavior in Standby sleep mode
    ///
    /// When `RUNSTDBY` is disabled, the [`Xosc`] will never run in Standby
    /// sleep mode unless `ONDEMAND` is enabled and the `Xosc` is requested by a
    /// peripheral. When `RUNSTDBY` is enabled, the `Xosc` will run in Standby
    /// sleep mode, but it can still be disabled if `ONDEMAND` is enabled and
    /// the `Xosc` is not requested.
    #[inline]
    pub fn run_standby(mut self, run_standby: bool) -> Self {
        self.settings.run_standby = run_standby;
        self
    }

    /// Enable the [`Xosc`], so that it can be used as a clock [`Source`]
    ///
    /// As mentioned when creating a new `Xosc`, no hardware registers are
    /// actually modified until this call. Rather, the desired configuration is
    /// stored internally, and the `Xosc` is initialized and configured here
    /// according to the datasheet.
    ///
    /// The returned value is an [`EnabledXosc`] that can be used as a clock
    /// [`Source`] for other clocks.
    #[inline]
    pub fn enable(mut self) -> EnabledXosc<X, M> {
        self.token.reset();
        self.token.set_xoscctrl(self.settings);
        self.token.enable();
        Enabled::new(self)
    }
}

impl<X, M> EnabledXosc<X, M>
where
    X: XoscId,
    M: Mode,
{
    /// Disable the [`Xosc`]
    ///
    /// This method is only implemented for `N = U0`, which means the clock can
    /// only be disabled when no other clocks consume this [`Xosc`].
    #[inline]
    pub fn disable(mut self) -> Xosc<X, M> {
        self.0.token.disable();
        self.0
    }
}

impl<X, M, N> EnabledXosc<X, M, N>
where
    X: XoscId,
    M: Mode,
{
    /// Check whether the [`Xosc`] is stable and ready to be used as a clock
    /// [`Source`]
    #[inline]
    pub fn is_ready(&self) -> bool {
        self.0.token.is_ready()
    }

    /// Enable continuous monitoring of the [`Xosc`] for clock failure
    ///
    /// Failure detection will continuously monitor the [`Xosc`] to verify it is
    /// still running. In the event of a failure, the `Xosc` output will be
    /// switched to the "safe clock".
    ///
    /// The safe clock is derived from the DFLL, which runs at 48 MHz. The XOSC
    /// peripheral provides a prescaler to divide down the 48 MHz DFLL to better
    /// match the clock it replaces. The prescaler division factor can be any
    /// power of two, `2^N`, with `N` in the range `0..16`.
    ///
    /// For example, if the [`Xosc`] input frequency is 16 MHz, a reasonable
    /// divider would be 4, becuase the safe clock frequency would be 12 MHz,
    /// which is closest to 16 MHz.
    ///
    /// Note that clock failure is triggered when four safe clock periods pass
    /// without seeing a rising & falling edge pair on the XOSC clock. Once
    /// failure is detected, the corresponding bit in the `STATUS` register will
    /// go high and an interrupt will be triggered.
    ///
    /// If the external clock can be fixed, the `Xosc` can be switched back to
    /// it using [`EnabledXosc::switch_back`].
    ///
    /// Because the safe clock makes use of the DFLL, the `Xosc` must register
    /// as a consumer of the [`EnabledDfll`] and [`Increment`] its counter.
    ///
    /// [`EnabledDfll`]: super::dfll::EnabledDfll
    #[inline]
    pub fn enable_failure_detection<S>(&mut self, dfll: S, div: SafeClockDiv) -> S::Inc
    where
        S: Source<Id = DfllId> + Increment,
    {
        self.0.token.enable_failure_detection(div);
        dfll.inc()
    }

    /// Check whether the [`Xosc`] has triggered clock failure detection
    ///
    /// Failure detection must be enabled for this to return `true`. Failure is
    /// triggered when four safe clock periods pass without seeing a rising &
    /// falling edge pair on the XOSC clock.
    ///
    /// See [`EnabledXosc::enable_failure_detection`] for more details.
    #[inline]
    pub fn has_failed(&self) -> bool {
        self.0.token.has_failed()
    }

    /// Check whether the [`Xosc`] has been switched to the safe clock
    ///
    /// Returns `true` if the [`Xosc`] has been switched to the safe clock. This
    /// will only occur if clock failure detection is enabled.
    #[inline]
    pub fn is_switched(&self) -> bool {
        self.0.token.is_switched()
    }

    /// Attempt to switch from the safe clock back to the external clock
    ///
    /// This function will set the switch back bit (`SWBEN`) in the `XOSCCTRL`
    /// register. Once the hardware has successfully switched back, this bit
    /// will be automatically cleared.
    ///
    /// Users can check whether switching back was successful by checking the
    /// `STATUS` register with [`EnabledXosc::is_switched`].
    #[inline]
    pub fn switch_back(&mut self) {
        self.0.token.switch_back()
    }

    /// Disable continuous monitoring of the [`Xosc`] for clock failure
    ///
    /// Once failure monitoring is disabled, the DFLL is no longer used as the
    /// safe clock, so the [`EnabledDfll`] counter can be [`Decrement`]ed.
    ///
    /// [`EnabledDfll`]: super::dfll::EnabledDfll
    #[inline]
    pub fn disable_failure_detection<S>(&mut self, dfll: S) -> S::Dec
    where
        S: Source<Id = DfllId> + Decrement,
    {
        self.0.token.disable_failure_detection();
        dfll.dec()
    }
}

//==============================================================================
// Source
//==============================================================================

impl<X, M, N> Source for EnabledXosc<X, M, N>
where
    X: XoscId,
    M: Mode,
{
    type Id = X;

    #[inline]
    fn freq(&self) -> Hertz {
        self.0.freq()
    }
}
