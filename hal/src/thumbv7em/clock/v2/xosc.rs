//! # Xosc - External oscillator
//!
//! A signal source for [`Gclks`][super::gclk] and [`Dplls`][super::dpll].
//!
//! There are two external oscillators that are available:
//! - [`Enabled`]`<`[`Xosc`]`<`[`marker::Xosc0`]`, _>>`: [`Xosc0`]
//! - [`Enabled`]`<`[`Xosc`]`<`[`marker::Xosc1`]`, _>>`: [`Xosc1`]
//!
//! There are two modes of operation that are available:
//! - [`Enabled`]`<`[`Xosc`]`<_, `[`CrystalMode`]`>>`: Xosc is being powered by
//!   an external crystal (2 pins)
//! - [`Enabled`]`<`[`Xosc`]`<_, `[`ClockMode`]`>>`: Xosc is being powered by an
//!   external signal (1 pin)
//!
//! To construct a Xosc in a proper mode use an appropriate construction
//! function:
//! - [`Xosc::from_clock`]
//! - [`Xosc::from_crystal`]
//! Then, enable it with a [`Xosc::enable`] function call

use core::marker::PhantomData;

use typenum::U0;

use crate::pac::oscctrl::{self, XOSCCTRL};

use crate::gpio::{FloatingDisabled, Pin, PinId, PA14, PA15, PB22, PB23};
use crate::time::Hertz;
use crate::typelevel::{Counter, Decrement, Increment, PrivateDecrement, PrivateIncrement, Sealed};

use super::dfll::{self, EnabledDfll};
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
//
// # Internal notes
//
// `XoscToken` is generic over the `XoscId`, and each corresponding instance is
// a singleton. There should never be more than one instance of `XoscToken` with
// a given `XoscId`, because `XoscToken` relies on this fact for memory safety.
//
// Users see `XoscToken` as merely an opaque token. but internally, `XoscToken`
// is also used as a register interface. The tokens are zero-sized, so they can
// be carried by all clock types without introducing any memory bloat.
//
// As part of that register interface, each `GclkToken` can access its
// corresponding `XOSCCTRL` register. That each `XoscToken` is a singleton
// guarantees each corresponding register is written from only one location.
// This allows `XoscToken` to be `Sync`, even though the PAC `OSCCTRL` struct is
// not.
pub struct XoscToken<X: XoscId> {
    id: PhantomData<X>,
}

impl<X: XoscId> XoscToken<X> {
    /// Create a new instance of [`XoscToken`]
    ///
    /// # Safety
    ///
    /// Each `XoscToken`s is a singleton. There must never be two simulatenous
    /// instances with the same [`XoscId`].
    #[inline]
    pub(super) unsafe fn new() -> Self {
        Self { id: PhantomData }
    }

    #[inline]
    fn xoscctrl(&self) -> &XOSCCTRL {
        // Safety: `XOSCCTRL` is not `Sync`, because it has interior mutability.
        // However, each `XoscToken` represents only one of two XOSCs, and
        // this function only ever returns a reference to the corresponding
        // `XOSCCTRL`, so there is no risk of accessing the same register from
        // multiple execution contexts. Division of the PAC `OSCCTRL` struct
        // into individual `XoscTokens`s based on `XoscId` is what lets us make
        // each `XoscToken` `Sync`.
        let oscctrl = unsafe { &*crate::pac::OSCCTRL::ptr() };
        &oscctrl.xoscctrl[X::NUM]
    }

    #[inline]
    fn status(&self) -> oscctrl::status::R {
        // Safety: We are only reading from the `STATUS` register, so there is
        // no risk of memory corruption.
        let oscctrl = unsafe { &*crate::pac::OSCCTRL::ptr() };
        oscctrl.status.read()
    }

    #[inline]
    fn is_ready(&self) -> bool {
        let mask = 1 << X::NUM;
        self.status().bits() & mask != 0
    }

    #[inline]
    fn has_failed(&self) -> bool {
        let mask = 1 << (X::NUM + 2);
        self.status().bits() & mask != 0
    }

    #[inline]
    fn is_switched(&self) -> bool {
        let mask = 1 << (X::NUM + 4);
        self.status().bits() & mask != 0
    }

    #[inline]
    fn reset(&self) {
        self.xoscctrl().reset();
    }

    /// Set the safe clock prescaler divider to 2^N
    ///
    /// To monitor the XOSC clock/oscillator, the safe clock frequency must be
    /// less than 4x the XOSC frequency. The safe clock is derived from the DFLL
    /// and is normally 48 MHz. This prescaler can be used to lower the safe
    /// clock frequency.
    ///
    /// The actual prescaler is 2^N, where N is in the range `0..16`. If
    /// `N >= 16`, it will be set to 15.
    #[inline]
    fn set_safe_clock_divider(&mut self, n: u8) {
        let n = if n < 16 { n } else { 15 };
        // Safety: The PAC enum is incomplete. The prescaler field has 4 bits,
        // but the enum only allows setting a value of N up to 7. It is safe to
        // set N up to 15.
        self.xoscctrl()
            .modify(|_, w| unsafe { w.cfdpresc().bits(n) });
    }

    /// Set the start up delay before the XOSC is continuously monitored
    ///
    /// During the start up period, the XOSC is masked to prevent clock
    /// instability from propagating to the digital logic. During this time,
    /// clock failure detection is disabled.
    ///
    /// The startup delay is set to 2^N OSCULP32K clock cyces, where N is in the
    /// range `0..16`. If `N >= 16`, it will be set to 15.
    #[inline]
    fn set_start_up_delay(&mut self, n: u8) {
        let n = if n < 16 { n } else { 15 };
        self.xoscctrl().modify(|_, w| w.startup().bits(n));
    }

    /// Switch from the safe clock back to the XOSC clock/oscillator
    ///
    /// This bit is cleared by the hardware after successfully switching back
    #[inline]
    fn switch_back(&mut self) {
        self.xoscctrl().modify(|_, w| w.swben().set_bit());
    }

    /// Enable clock failure detection
    #[inline]
    fn enable_failure_detection(&mut self) {
        self.xoscctrl().modify(|_, w| w.cfden().set_bit());
    }

    /// Disable clock failure detection
    #[inline]
    fn disable_failure_detection(&mut self) {
        self.xoscctrl().modify(|_, w| w.cfden().clear_bit());
    }

    #[inline]
    fn set_loop_control(&mut self, loop_control: bool) {
        self.xoscctrl().modify(|_, w| w.enalc().bit(loop_control));
    }

    #[inline]
    fn set_current(&mut self, current: CrystalCurrent) {
        // Safety: The `IMULT` and `IPTAT` values come from the
        // `CrystalCurrent`, so they are guaranteed to be valid.
        self.xoscctrl().modify(|_, w| unsafe {
            w.imult().bits(current.imult());
            w.iptat().bits(current.iptat())
        });
    }

    #[inline]
    fn set_low_buf_gain(&mut self, low_buf_gain: bool) {
        self.xoscctrl()
            .modify(|_, w| w.lowbufgain().bit(low_buf_gain));
    }

    #[inline]
    fn set_on_demand(&mut self, on_demand: bool) {
        self.xoscctrl().modify(|_, w| w.ondemand().bit(on_demand));
    }

    #[inline]
    fn set_run_standby(&mut self, run_standby: bool) {
        self.xoscctrl().modify(|_, w| w.runstdby().bit(run_standby));
    }

    #[inline]
    fn set_xtalen(&mut self, xtalen: bool) {
        self.xoscctrl().modify(|_, w| w.xtalen().bit(xtalen));
    }

    #[inline]
    fn enable(&mut self) {
        self.xoscctrl().modify(|_, w| w.enable().bit(true));
    }

    #[inline]
    fn disable(&mut self) {
        self.xoscctrl().modify(|_, w| w.enable().bit(false));
    }
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
/// `XoscId` is the type-level equivalent of [`DynXoscId`]. See the
/// documentation on [type-level programming] and specifically
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

/// Type alias for Xosc Input pin
pub type XIn<X> = Pin<<X as XoscId>::XIn, FloatingDisabled>;

/// Type alias for Xosc Output pin
pub type XOut<X> = Pin<<X as XoscId>::XOut, FloatingDisabled>;

//==============================================================================
// CrystalCurrent
//==============================================================================

/// Crystal current settings
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
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum CrystalCurrent {
    /// Used only in [`ClockMode`] to represent the default register values
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
// Mode
//==============================================================================

/// Type-level `enum` for the [`Xosc`] operation mode
///
/// An [`Xosc`] can be sourced from either an external clock or a cyrstal
/// oscillator. This type-level `enum` provides the type-level variants
/// [`ClockMode`] and [`CrystalMode`].
///
/// See the [type-level enum] documentation for more details on the pattern.
///
/// [type-level enum]: crate::typelevel#type-level-enum
pub trait Mode: Sealed {
    /// `XTALEN` field for the corresponding mode
    const XTALEN: bool;
    /// Get the [`CrystalCurrent`]
    fn current(&self) -> CrystalCurrent;
    /// Get the loop control bit
    fn loop_control(&self) -> bool;
    /// Get the low buf gain bit
    fn low_buf_gain(&self) -> bool;
}

//==============================================================================
// ClockMode
//==============================================================================

/// Type-level variant of the [`Xosc`] operation [`Mode`]
///
/// Represents the [`Xosc`] configured to use an externally provided clock.
///
/// See the [type-level enum] documentation for more details on the pattern.
///
/// [type-level enum]: crate::typelevel#type-level-enum
pub struct ClockMode;

impl Sealed for ClockMode {}

impl Mode for ClockMode {
    const XTALEN: bool = false;
    #[inline]
    fn current(&self) -> CrystalCurrent {
        CrystalCurrent::Zero
    }
    #[inline]
    fn loop_control(&self) -> bool {
        false
    }
    #[inline]
    fn low_buf_gain(&self) -> bool {
        false
    }
}

//==============================================================================
// CrystalMode
//==============================================================================

/// Type-level variant of the [`Xosc`] operation [`Mode`]
///
/// Represents the [`Xosc`] configured to use an external crystal oscillator.
///
/// See the [type-level enum] documentation for more details on the pattern.
///
/// [type-level enum]: crate::typelevel#type-level-enum
pub struct CrystalMode {
    current: CrystalCurrent,
    loop_control: bool,
    low_buf_gain: bool,
}

impl Sealed for CrystalMode {}

impl Mode for CrystalMode {
    const XTALEN: bool = true;
    #[inline]
    fn current(&self) -> CrystalCurrent {
        self.current
    }
    #[inline]
    fn loop_control(&self) -> bool {
        self.loop_control
    }
    #[inline]
    fn low_buf_gain(&self) -> bool {
        self.low_buf_gain
    }
}

//==============================================================================
// Xosc
//==============================================================================

/// Struct representing a disabled external oscillator
///
/// It is generic over:
/// - a numeric variant (available variants: [`marker::Xosc0`],
///   [`marker::Xosc1`])
/// - a mode of operation (available modes: [`ClockMode`], [`CrystalMode`])
pub struct Xosc<X, M>
where
    X: XoscId,
    M: Mode,
{
    token: XoscToken<X>,
    mode: M,
    freq: Hertz,
    start_up: u8,
    on_demand: bool,
    run_standby: bool,
}

/// Alias for the corresponding [`Xosc`]
pub type Xosc0<M> = Xosc<Xosc0Id, M>;

/// Alias for the corresponding [`Xosc`]
pub type Xosc1<M> = Xosc<Xosc1Id, M>;

pub type EnabledXosc<X, M, N = U0> = Enabled<Xosc<X, M>, N>;

pub type EnabledXosc0<M, N = U0> = EnabledXosc<Xosc0Id, M, N>;

pub type EnabledXosc1<M, N = U0> = EnabledXosc<Xosc1Id, M, N>;

impl<X: XoscId> Xosc<X, ClockMode> {
    #[inline]
    pub fn from_clock(token: XoscToken<X>, xin: impl Into<XIn<X>>, freq: impl Into<Hertz>) -> Self {
        // Convert `XIn` to the correct `PinMode` and then drop it.
        // We can recreate it when freeing the `Xosc`
        let _xin: XIn<X> = xin.into();
        Xosc::new(token, ClockMode, freq.into())
    }

    #[inline]
    pub fn free_clock(self) -> (XoscToken<X>, XIn<X>) {
        // Safety: We dropped the `Pin` on construction of the `Xosc`,
        // so we can safely recreate it here.
        let xin = unsafe { Pin::new() };
        (self.token, xin)
    }
}

impl<X: XoscId> Xosc<X, CrystalMode> {
    /// Construct a [`Xosc`] from a two pin crystal oscillator signal
    ///
    /// The crystal oscillator frequency must be supported, for valid
    /// frequencies see [`CrystalCurrent`].
    ///
    /// By default `Amplitude Loop Control` is set, see
    /// [`Xosc::set_loop_control`]
    #[inline]
    pub fn from_crystal(
        token: XoscToken<X>,
        xin: impl Into<XIn<X>>,
        xout: impl Into<XOut<X>>,
        freq: impl Into<Hertz>,
        current: CrystalCurrent,
    ) -> Self {
        // Convert `XIn` and `Xout` to the correct `PinMode` and then drop them.
        // We can recreate them when freeing the `Xosc`
        let _xin: XIn<X> = xin.into();
        let _xout: XOut<X> = xout.into();
        let mode = CrystalMode {
            current,
            loop_control: false,
            low_buf_gain: false,
        };
        Xosc::new(token, mode, freq.into())
    }

    /// Deconstruct the Xosc and return the inner XoscToken
    #[inline]
    pub fn free_crystal(self) -> (XoscToken<X>, XIn<X>, XOut<X>) {
        // Safety: We dropped the `Pin`s on construction of the `Xosc`,
        // so we can safely recreate them here.
        let xin = unsafe { Pin::new() };
        let xout = unsafe { Pin::new() };
        (self.token, xin, xout)
    }

    /// Sets the current drive strength for the crystal
    ///
    /// See [CrystalCurrent] for possible values
    #[inline]
    pub fn set_current(mut self, current: CrystalCurrent) -> Self {
        self.mode.current = current;
        self
    }

    /// Controls the automatic loop control
    ///
    /// Recommended option, ensures the crystal is not overdriven,
    /// and lowers power consumption. See datasheet c. 54.13 p. 1811
    #[inline]
    pub fn set_loop_control(mut self, enalc: bool) -> Self {
        self.mode.loop_control = enalc;
        self
    }

    /// If `LOWBUFGAIN`is set when `ENALC` is enabled,
    /// the oscillators amplitude is increased by approximately a factor 2.
    ///
    /// Default value (0) should be used together with low amplitude
    /// oscillators. Can be used to solve stability issues.
    #[inline]
    pub fn set_low_buf_gain(mut self, lowbufgain: bool) -> Self {
        self.mode.low_buf_gain = lowbufgain;
        self
    }
}

impl<X, M> Xosc<X, M>
where
    X: XoscId,
    M: Mode,
{
    #[inline]
    fn new(token: XoscToken<X>, mode: M, freq: Hertz) -> Self {
        let start_up = 0;
        let on_demand = true;
        let run_standby = false;
        Self {
            token,
            mode,
            freq,
            start_up,
            on_demand,
            run_standby,
        }
    }

    /// Returns the frequency of the oscillator
    #[inline]
    pub fn freq(&self) -> Hertz {
        self.freq
    }

    /// Set the start up delay before the [`Xosc`] is continuously monitored
    ///
    /// During the start up period, the [`Xosc`] is masked to prevent clock
    /// instability from propagating to the digital logic. During this time,
    /// clock failure detection is disabled.
    ///
    /// The startup delay is set to 2^N OSCULP32K clock cyces, where N is in the
    /// range `0..16`. If `N >= 16`, it will be set to 15.
    #[inline]
    pub fn set_start_up_delay(mut self, n: u8) -> Self {
        self.start_up = n;
        self
    }
    /// Controls the on demand functionality of the clock source
    ///
    /// Only starts the clock source when a peripheral uses it
    ///
    /// If cleared the clock will be always active
    /// See Datasheet c. 13.5 for details
    #[inline]
    pub fn set_on_demand(mut self, on_demand: bool) -> Self {
        self.on_demand = on_demand;
        self
    }

    /// Controls the clock source behaviour during standby
    ///
    /// See Datasheet c. 28.6.2
    #[inline]
    pub fn set_run_standby(mut self, run_standby: bool) -> Self {
        self.run_standby = run_standby;
        self
    }

    /// Modify hardware to realise the desired state
    /// stored within the [`Xosc`]
    ///
    /// Returns the enabled Xosc
    #[inline]
    pub fn enable(mut self) -> EnabledXosc<X, M> {
        self.token.reset();
        self.token.set_xtalen(M::XTALEN);
        self.token.set_start_up_delay(self.start_up);
        self.token.set_on_demand(self.on_demand);
        self.token.set_run_standby(self.run_standby);
        self.token.set_loop_control(self.mode.loop_control());
        self.token.set_low_buf_gain(self.mode.low_buf_gain());
        self.token.set_current(self.mode.current());
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
    /// Only possible when nothing uses the `Xosc`
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
    N: Counter,
{
    /// Check whether the [`Xosc`] is stable and ready to be used as a clock
    /// source
    pub fn is_ready(&self) -> bool {
        self.0.token.is_ready()
    }

    /// Check whether the [`Xosc`] has triggered clock failure detection
    ///
    /// Failure detection must be enabled for this to return `true`. Failure is
    /// triggered when four safe clock periods pass without seeing a rising &
    /// falling edge pair on the XOSC clock.
    ///
    /// See [`EnabledXosc::enable_failure_detection`] for more details.
    pub fn has_failed(&self) -> bool {
        self.0.token.has_failed()
    }

    /// Check whether the [`Xosc`] has been switched to the safe clock
    ///
    /// Returns `true` if the [`Xosc`] has been switched to the safe clock. This
    /// will only occur if clock failure detection is enabled.
    pub fn is_switched(&self) -> bool {
        self.0.token.is_switched()
    }

    /// Enable continuous monitoring of the [`Xosc`] for clock failure
    ///
    /// Failure detection will continuously monitor the [`Xosc`] to verify it is
    /// still running. In the event of a failure, the [`Xosc`] output will be
    /// switched to the "safe clock".
    ///
    /// The safe clock is derived from the DFLL, which runs at 48 MHz. The XOSC
    /// peripheral provides a prescaler to divide down the 48 MHz DFLL to better
    /// match the clock it replaces. The prescaler division factor can be any
    /// power of two, `2^N`, with `N` in the range `0..16`. If `N >= 16`, it
    /// will be set to `15`.
    ///
    /// For example, if the [`Xosc`] input frequency is 8 MHz, a reasonable
    /// choice of `N` would be `N = 3`, becuase the safe clock frequency would
    /// be 6 MHz, which is closest to 8 MHz.
    ///
    /// Note that clock failure is triggered when four safe clock periods pass
    /// without seeing a rising & falling edge pair on the XOSC clock. Once
    /// failure is detected, the corresponding bit in the `STATUS` register will
    /// go high and an interrupt will be triggered.
    ///
    /// If the external clock can be fixed, the [`Xosc`] can be switched
    /// back to it using [`EnabledXosc::switch_back`].
    ///
    /// Because the safe clock makes use of the DFLL, users must register as a
    /// consumer of the [`EnabledDfll`] and [`Increment`] its [`Counter`].
    pub fn enable_failure_detection<DM, DN>(
        &mut self,
        dfll: EnabledDfll<DM, DN>,
        n: u8,
    ) -> EnabledDfll<DM, DN::Inc>
    where
        DM: dfll::Mode,
        DN: Counter + Increment,
    {
        self.0.token.set_safe_clock_divider(n);
        self.0.token.enable_failure_detection();
        return dfll.inc();
    }

    /// Attempt to switch from the safe clock back to the external clock
    ///
    /// This function will set the switch back bit (`SWBEN`) in the `XOSCCTRL`
    /// register. Once the hardware has successfully switched back, this bit
    /// will be automatically cleared.
    ///
    /// Users can check whether switching back was successful by checking the
    /// `STATUS` register with [`EnabledXosc::is_switched`].
    pub fn switch_back(&mut self) {
        self.0.token.switch_back()
    }

    /// Disable continuous monitoring of the [`Xosc`] for clock failure
    ///
    /// Once failure monitoring is disabled, the DFLL is no longer used as the
    /// safe clock, so the [`EnabledDfll`] [`Counter`] can be [`Decrement`]ed.
    pub fn disable_failure_detection<DM, DN>(
        &mut self,
        dfll: EnabledDfll<DM, DN>,
    ) -> EnabledDfll<DM, DN::Dec>
    where
        DM: dfll::Mode,
        DN: Counter + Decrement,
    {
        self.0.token.disable_failure_detection();
        return dfll.dec();
    }
}

//==============================================================================
// Source
//==============================================================================

impl<X, M, N> Source for EnabledXosc<X, M, N>
where
    X: XoscId,
    M: Mode,
    N: Counter,
{
    type Id = X;

    #[inline]
    fn freq(&self) -> Hertz {
        self.0.freq()
    }
}
