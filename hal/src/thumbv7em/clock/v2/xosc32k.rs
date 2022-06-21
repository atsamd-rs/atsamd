//! # Xosc32k - External 32 kHz oscillator

#![allow(missing_docs)]

use typenum::U0;

use crate::pac::osc32kctrl::xosc32k::{CGM_A, STARTUP_A};
use crate::pac::osc32kctrl::{RegisterBlock, STATUS, XOSC32K};

use crate::gpio::{FloatingDisabled, Pin, PA00, PA01};
use crate::time::Hertz;
use crate::typelevel::{Counter, Decrement, Increment, PrivateDecrement, PrivateIncrement, Sealed};

use super::{Enabled, Source};

//==============================================================================
// Tokens
//==============================================================================

pub struct XoscBaseToken(());

pub struct Xosc1kToken(());

pub struct Xosc32kToken(());

pub struct Tokens {
    pub base: XoscBaseToken,
    pub xosc1k: Xosc1kToken,
    pub xosc32k: Xosc32kToken,
}

impl Tokens {
    /// Create a new set of tokens
    ///
    /// Safety: There must never be more than one instance of a token at any
    /// given time.
    pub(super) unsafe fn new() -> Self {
        Self {
            base: XoscBaseToken(()),
            xosc1k: Xosc1kToken(()),
            xosc32k: Xosc32kToken(()),
        }
    }
}

impl XoscBaseToken {
    #[inline]
    fn osc32kctrl(&self) -> &RegisterBlock {
        unsafe { &*crate::pac::OSC32KCTRL::ptr() }
    }

    #[inline]
    fn status(&self) -> &STATUS {
        &self.osc32kctrl().status
    }

    #[inline]
    fn xosc32k(&self) -> &XOSC32K {
        &self.osc32kctrl().xosc32k
    }

    #[inline]
    fn set_control_gain_mode(&mut self, cgm: ControlGainMode) {
        let variant = match cgm {
            ControlGainMode::Standard => CGM_A::XT,
            ControlGainMode::HighSpeed => CGM_A::HS,
        };
        self.xosc32k().modify(|_, w| w.cgm().variant(variant));
    }

    #[inline]
    fn set_start_up(&mut self, start_up: StartUp32k) {
        self.xosc32k().modify(|_, w| w.startup().variant(start_up));
    }

    #[inline]
    fn set_on_demand(&mut self, on_demand: bool) {
        self.xosc32k().modify(|_, w| w.ondemand().bit(on_demand));
    }

    #[inline]
    fn set_run_standby(&mut self, run_standby: bool) {
        self.xosc32k().modify(|_, w| w.runstdby().bit(run_standby));
    }

    #[inline]
    fn enable_1k(&mut self, enabled: bool) {
        self.xosc32k().modify(|_, w| w.en1k().bit(enabled));
    }

    #[inline]
    fn enable_32k(&mut self, enabled: bool) {
        self.xosc32k().modify(|_, w| w.en32k().bit(enabled));
    }

    #[inline]
    fn set_xtalen(&mut self, xtalen: bool) {
        self.xosc32k().modify(|_, w| w.xtalen().bit(xtalen));
    }

    #[inline]
    fn enable(&mut self) {
        self.xosc32k().modify(|_, w| w.enable().bit(true));
    }

    #[inline]
    fn wrtlock(&mut self) {
        self.xosc32k().modify(|_, w| w.wrtlock().bit(true));
    }

    #[inline]
    fn disable(&mut self) {
        self.xosc32k().modify(|_, w| w.enable().bit(false));
    }

    #[inline]
    fn wait_ready(&self) {
        while self.status().read().xosc32krdy().bit_is_clear() {}
    }
}

//==============================================================================
// Aliases
//==============================================================================

/// Clock start-up time, in cycles
///
/// At start-up, the XOSC32K clock output is masked to guard against clock
/// instability. This `enum` represents the duration of that start-up time, in
/// clock cycles.
pub type StartUp32k = STARTUP_A;

/// [`Pin`] alias for the XOSC32K input pin
///
/// This pin is required in both [`ClockMode`] and [`CrystalMode`]
pub type XIn32 = Pin<PA00, FloatingDisabled>;

/// [`Pin`] alias for the XOSC32K output pin
///
/// This pin is only required in [`CrystalMode`]
pub type XOut32 = Pin<PA01, FloatingDisabled>;

//==============================================================================
// ControlGainMode
//==============================================================================

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ControlGainMode {
    Standard,
    HighSpeed,
}

//==============================================================================
// Mode
//==============================================================================

/// Type-level enum for the XOSC32K operation mode
///
/// The XOSC32K clock can be sourced from an external clock or a crystal
/// oscillator. The [`ClockMode`] and [`CrystalMode`] types act as type-level
/// variants. See the documentation on [type-level enums] for more details on
/// the pattern.
///
/// [type-level enums]: crate::typelevel#type-level-enum
pub trait Mode: Sealed {
    const XTALEN: bool;
    fn control_gain_mode(&self) -> ControlGainMode;
}

/// Type-level variant of the XOSC32K operating [`Mode`]
///
/// In this `Mode`, the XOSC32K clock will be sourced from an external clock.
/// See the documentation on [type-level enums] for more details on the pattern.
///
/// [type-level enums]: crate::typelevel#type-level-enum
pub struct ClockMode;
impl Sealed for ClockMode {}
impl Mode for ClockMode {
    const XTALEN: bool = false;
    fn control_gain_mode(&self) -> ControlGainMode {
        ControlGainMode::Standard
    }
}

/// Type-level variant of the XOSC32K operating [`Mode`]
///
/// In this `Mode`, the XOSC32K clock will be sourced from a crystal oscillator.
/// See the documentation on [type-level enums] for more details on the pattern.
///
/// [type-level enums]: crate::typelevel#type-level-enum
pub struct CrystalMode {
    xout32: XOut32,
    cgm: ControlGainMode,
}
impl Sealed for CrystalMode {}
impl Mode for CrystalMode {
    const XTALEN: bool = true;
    fn control_gain_mode(&self) -> ControlGainMode {
        self.cgm
    }
}

//==============================================================================
// XoscBase
//==============================================================================

pub struct XoscBase<M: Mode> {
    token: XoscBaseToken,
    xin32: XIn32,
    mode: M,
    run_standby: bool,
    on_demand_mode: bool,
    start_up_masking: StartUp32k,
}

pub type EnabledXoscBase<M, N = U0> = Enabled<XoscBase<M>, N>;

impl XoscBase<ClockMode> {
    /// Initialize the XOSC32K from an external clock signal
    ///
    /// This mode only uses the [`XIn32`] [`Pin`]. The [`XOut32`] `Pin` is still
    /// available to be used as GPIO.
    #[inline]
    pub fn from_clock(token: XoscBaseToken, xin32: impl Into<XIn32>) -> Self {
        let xin32 = xin32.into().into_floating_disabled();
        Self::new(token, xin32, ClockMode)
    }

    /// Consume the [`XoscBase`] and return its token and the [`XIn32`] [`Pin`]
    #[inline]
    pub fn free(self) -> (XoscBaseToken, XIn32) {
        (self.token, self.xin32)
    }
}

impl XoscBase<CrystalMode> {
    /// Initialize the XOSC32K from a crystal oscillator
    ///
    /// This mode only uses both the [`XIn32`] [`Pin`] and the [`XOut32`] `Pin`.
    #[inline]
    pub fn from_crystal(
        token: XoscBaseToken,
        xin32: impl Into<XIn32>,
        xout32: impl Into<XOut32>,
    ) -> Self {
        let xin32 = xin32.into().into_floating_disabled();
        let xout32 = xout32.into().into_floating_disabled();
        let cgm = ControlGainMode::Standard;
        let mode = CrystalMode { xout32, cgm };
        Self::new(token, xin32, mode)
    }

    /// Set the crystal oscillator control gain mode
    ///
    /// Pick between high speed or low power consumption
    #[inline]
    pub fn control_gain_mode(mut self, cgm: ControlGainMode) -> Self {
        self.mode.cgm = cgm;
        self
    }

    /// Consume the [`XoscBase`] and return its token and both GPIO [`Pin`]s
    #[inline]
    pub fn free(self) -> (XoscBaseToken, XIn32, XOut32) {
        (self.token, self.xin32, self.mode.xout32)
    }
}

impl<M: Mode> XoscBase<M> {
    #[inline]
    fn new(token: XoscBaseToken, xin32: XIn32, mode: M) -> Self {
        Self {
            token,
            xin32,
            mode,
            run_standby: false,
            on_demand_mode: true,
            start_up_masking: StartUp32k::CYCLE2048,
        }
    }

    /// Set for how long the clock output should be masked during startup
    #[inline]
    pub fn start_up(mut self, start_up: StartUp32k) -> Self {
        self.start_up_masking = start_up;
        self
    }

    /// Controls how [`Xosc32k`] behaves when a peripheral clock request is
    /// detected
    #[inline]
    pub fn on_demand(mut self, on_demand: bool) -> Self {
        self.on_demand_mode = on_demand;
        self
    }

    /// Controls how [`Xosc32k`] should behave during standby
    #[inline]
    pub fn run_standby(mut self, run_standby: bool) -> Self {
        self.run_standby = run_standby;
        self
    }

    /// Wait until the clock source is ready
    #[inline]
    pub fn wait_ready(&self) {
        self.token.wait_ready();
    }

    /// Set the write-lock, which will last until POR
    ///
    /// This function sets the write-lock bit, which lasts until power-on reset.
    /// It also consumes and drops the [`XoscBase`], which destroys API access
    /// to the registers.
    ///
    /// **NOTE:** Because the `XoscBase` is not enabled, calling `write_lock`
    /// will lock both the 1 kHz and 32 kHz clocks in their disabled state.
    #[inline]
    pub fn write_lock(mut self) {
        self.token.wrtlock();
    }

    #[inline]
    pub fn enable(mut self) -> EnabledXoscBase<M> {
        self.token.set_xtalen(M::XTALEN);
        self.token.set_on_demand(self.on_demand_mode);
        self.token.set_run_standby(self.run_standby);
        self.token.set_start_up(self.start_up_masking);
        self.token
            .set_control_gain_mode(self.mode.control_gain_mode());
        self.token.enable();
        Enabled::new(self)
    }
}

impl<M: Mode, N: Counter> EnabledXoscBase<M, N> {
    /// Set the write-lock, which will last until POR
    ///
    /// This function sets the write-lock bit, which lasts until power-on reset.
    /// It also consumes and drops the [`XoscBase`], which destroys API access
    /// to the registers.
    #[inline]
    pub fn write_lock(mut self) {
        self.0.token.wrtlock();
    }

    pub fn disable(mut self) -> XoscBase<M> {
        self.0.token.disable();
        self.0
    }
}

//==============================================================================
// Ids
//==============================================================================

/// Type-level variant representing the identity of the XOSC1K clock
///
/// This type is a member of several [type-level enums]. See the documentation
/// on [type-level enums] for more details on the pattern.
///
/// [type-level enums]: crate::typelevel#type-level-enum
pub enum Xosc1kId {}

impl Sealed for Xosc1kId {}

/// Type-level variant representing the identity of the XOSC32K clock
///
/// This type is a member of several [type-level enums]. See the documentation
/// on [type-level enums] for more details on the pattern.
///
/// [type-level enums]: crate::typelevel#type-level-enum
pub enum Xosc32kId {}

impl Sealed for Xosc32kId {}

//==============================================================================
// Xosc1k
//==============================================================================

pub struct Xosc1k {
    token: Xosc1kToken,
}

pub type EnabledXosc1k<N = U0> = Enabled<Xosc1k, N>;

impl Xosc1k {
    /// Enable the 1 kHz output from XOSC32K
    ///
    /// This clock is derived from the [`Enabled`] [`XoscBase`] clock.
    ///
    /// ```no_run
    /// # use atsamd_hal::clock::v2::clock_system_at_reset;
    /// # use atsamd_hal::clock::v2::xosc32k::{XoscBase, Xosc1k};
    /// # use atsamd_hal::gpio::Pins;
    /// # use atsamd_hal::pac::Peripherals;
    /// # let mut pac = Peripherals::take().unwrap();
    /// let (buses, clocks, tokens) = clock_system_at_reset(
    ///     pac.OSCCTRL,
    ///     pac.OSC32KCTRL,
    ///     pac.GCLK,
    ///     pac.MCLK,
    ///     &mut pac.NVMCTRL,
    /// );
    /// let pins = Pins::new(pac.PORT);
    /// let xosc_base = XoscBase::from_clock(
    ///     tokens.xosc32k.base,
    ///     pins.pa00,
    /// ).enable();
    /// let (xosc1k, base) = Xosc1k::enable(tokens.xosc32k.xosc1k, xosc_base);
    /// ```
    #[inline]
    pub fn enable<M, N>(
        token: Xosc1kToken,
        mut base: EnabledXoscBase<M, N>,
    ) -> (EnabledXosc1k, EnabledXoscBase<M, N::Inc>)
    where
        M: Mode,
        N: Increment,
    {
        base.0.token.enable_1k(true);
        (Enabled::new(Self { token }), base.inc())
    }
}

impl EnabledXosc1k {
    /// Disable the 1 kHz output from XOSC32K
    ///
    /// Doing so will clear one usage of the [`Enabled`] [`XoscBase`] clock
    #[inline]
    pub fn disable<M, N>(
        self,
        mut base: EnabledXoscBase<M, N>,
    ) -> (Xosc1kToken, EnabledXoscBase<M, N::Dec>)
    where
        M: Mode,
        N: Decrement,
    {
        base.0.token.enable_1k(false);
        (self.0.token, base.dec())
    }
}

impl<N: Counter> Source for EnabledXosc1k<N> {
    type Id = Xosc1kId;

    fn freq(&self) -> Hertz {
        Hertz(1024)
    }
}

//==============================================================================
// Xosc32k
//==============================================================================

pub struct Xosc32k {
    token: Xosc32kToken,
}

pub type EnabledXosc32k<N = U0> = Enabled<Xosc32k, N>;

impl Xosc32k {
    /// Enable the 32 kHz output from XOSC32K
    ///
    /// This clock is derived from the [`Enabled`] [`XoscBase`] clock.
    ///
    /// ```no_run
    /// # use atsamd_hal::clock::v2::clock_system_at_reset;
    /// # use atsamd_hal::clock::v2::xosc32k::{XoscBase, Xosc32k};
    /// # use atsamd_hal::gpio::Pins;
    /// # use atsamd_hal::pac::Peripherals;
    /// # let mut pac = Peripherals::take().unwrap();
    /// let (buses, clocks, tokens) = clock_system_at_reset(
    ///     pac.OSCCTRL,
    ///     pac.OSC32KCTRL,
    ///     pac.GCLK,
    ///     pac.MCLK,
    ///     &mut pac.NVMCTRL,
    /// );
    /// let pins = Pins::new(pac.PORT);
    /// let xosc_base = XoscBase::from_clock(
    ///     tokens.xosc32k.base,
    ///     pins.pa00,
    /// ).enable();
    /// let (xosc32k, base) = Xosc32k::enable(tokens.xosc32k.xosc32k, xosc_base);
    /// ```
    #[inline]
    pub fn enable<M, N>(
        token: Xosc32kToken,
        mut base: EnabledXoscBase<M, N>,
    ) -> (EnabledXosc32k, EnabledXoscBase<M, N::Inc>)
    where
        M: Mode,
        N: Increment,
    {
        base.0.token.enable_32k(true);
        (Enabled::new(Self { token }), base.inc())
    }
}

impl EnabledXosc32k {
    /// Disable the 32 kHz output from XOSC32K
    ///
    /// Doing so will clear one usage of the [`Enabled`] [`XoscBase`] clock
    #[inline]
    pub fn disable<M, N>(
        self,
        mut base: EnabledXoscBase<M, N>,
    ) -> (Xosc32kToken, EnabledXoscBase<M, N::Dec>)
    where
        M: Mode,
        N: Decrement,
    {
        base.0.token.enable_32k(false);
        (self.0.token, base.dec())
    }
}

impl<N: Counter> Source for EnabledXosc32k<N> {
    type Id = Xosc32kId;

    fn freq(&self) -> Hertz {
        Hertz(32_768)
    }
}
