//! # Xosc32k - External 32 kHz oscillator
//!
//! Provides 32 kHz outputs for [`Gclks`][super::gclk], [`Rtc`][super::rtc]
//! and [`Dplls`][super::dpll].
//! Additionally provides 1 kHz output for the [`rtc`][super::rtc] module.
//!
//! There are two modes of operation that are available:
//! - [`Enabled`]`<`[`Xosc32k`]`<`[`CrystalMode`]`, _, _>>`: Xosc32k is being
//!   powered by an external crystal (2 pins)
//! - [`Enabled`]`<`[`Xosc32k`]`<`[`ClockMode`]`, _, _>>`: Xosc32k is being
//!   powered by an external signal (1 pin)
//!
//! Signal outputs are independently controllable and also expressed as
//! typestates
//! - [`Enabled`]`<`[`Xosc32k`]`<_, `[`Active32k`]`, _>>`: Xosc32k 32 kHz signal
//!   output is active
//! - [`Enabled`]`<`[`Xosc32k`]`<_, `[`Inactive32k`]`, _>>`: Xosc32k 32 kHz
//!   signal output is inactive
//! - [`Enabled`]`<`[`Xosc32k`]`<_, _, `[`Active1k`]`>>`: Xosc32k 1 kHz signal
//!   output is active
//! - [`Enabled`]`<`[`Xosc32k`]`<_, _, `[`Inactive1k`]`>>`: Xosc32k 1 kHz signal
//!   output is inactive
//!
//! To activate outputs, see:
//! - [`Enabled<Xosc32k>::activate_32k`]
//! - [`Enabled<Xosc32k>::activate_1k`]
//!
//! To construct a Xosc32k in a proper mode use an appropriate construction
//! function:
//! - [`Xosc32k::from_clock`]
//! - [`Xosc32k::from_crystal`]
//! Then, enable it with a [`Xosc32k::enable`] function call

use typenum::U0;

use crate::pac::osc32kctrl::xosc32k::{CGM_A, STARTUP_A};
use crate::pac::osc32kctrl::{RegisterBlock, STATUS, XOSC32K};

use crate::gpio::v2::{AnyPin, FloatingDisabled, Pin, PA00, PA01};
use crate::time::{Hertz, U32Ext};
use crate::typelevel::{Counter, Sealed};

use super::{Driver, Enabled};

//==============================================================================
// Xosc32kToken
//==============================================================================

/// Token struct that is essential in order to construct an instance of an
/// [`Xosc32k`].
pub struct Xosc32kToken {
    __: (),
}

impl Xosc32kToken {
    /// Create a new instance of [`Xosc32kToken`]
    #[inline]
    pub(super) unsafe fn new() -> Self {
        Self { __: () }
    }

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
    fn set_gain_mode(&mut self, high_speed: bool) {
        let variant = match high_speed {
            false => CGM_A::XT,
            true => CGM_A::HS,
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
    pub(super) fn activate_1k(&mut self, enabled: bool) {
        self.xosc32k().modify(|_, w| w.en1k().bit(enabled));
    }

    #[inline]
    pub(super) fn activate_32k(&mut self, enabled: bool) {
        self.xosc32k().modify(|_, w| w.en32k().bit(enabled));
    }

    #[inline]
    fn from_clock(&mut self) {
        self.xosc32k().modify(|_, w| w.xtalen().bit(false));
    }

    #[inline]
    fn from_crystal(&mut self) {
        self.xosc32k().modify(|_, w| w.xtalen().bit(true));
    }

    #[inline]
    fn enable(&mut self) {
        self.xosc32k().modify(|_, w| w.enable().bit(true));
    }

    #[allow(dead_code)]
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

/// For how long should the clock output be masked to prevent
/// unstable clocking output
pub type StartUp32k = STARTUP_A;

/// Pin alias type to ensure proper GPIO is used
pub type XIn32 = Pin<PA00, FloatingDisabled>;

/// Pin alias type to ensure proper GPIO is used
pub type XOut32 = Pin<PA01, FloatingDisabled>;

//==============================================================================
// Mode structure for Xosc32k
//==============================================================================

/// Trait that defines a mode [`Xosc32k`] is operating in
pub trait Mode: Sealed {}

/// Struct representing a clock mode for [`Xosc32k`]
///
/// In that mode [`Xosc32k`] requires a single clocking signal
pub struct ClockMode {}
impl Mode for ClockMode {}
impl Sealed for ClockMode {}

/// Struct representing a crystal mode for [`Xosc32k`]
///
/// In that mode [`Xosc32k`] requires two signals coming from an external
/// crystal
pub struct CrystalMode {
    xout32: XOut32,
    /// Control external crystal tuning
    control_gain_mode_high_speed: bool,
}
impl Mode for CrystalMode {}
impl Sealed for CrystalMode {}

//==============================================================================
// Xosc32k
//==============================================================================

/// Struct representing a disabled external oscillator
///
/// It is generic over:
/// - a mode of operation (available modes: [`ClockMode`], [`CrystalMode`])
/// - An output state of a 32 kHz signal ([`Active32k`]/[`Inactive32k`])
/// - An output state of a 1 kHz signal ([`Active1k`]/[`Inactive1k`])
pub struct Xosc32k<M>
where
    M: Mode,
{
    pub(super) token: Xosc32kToken,
    xin32: XIn32,
    mode: M,
    run_standby: bool,
    on_demand_mode: bool,
    start_up_masking: StartUp32k,
}

impl<M: Mode> Xosc32k<M> {
    /// Set for how long the clock output should be masked during startup
    #[inline]
    pub fn set_start_up(mut self, start_up: StartUp32k) -> Self {
        self.start_up_masking = start_up;
        self
    }

    /// Controls how [`Xosc32k`] behaves when a peripheral clock request is
    /// detected
    #[inline]
    pub fn set_on_demand(mut self, on_demand: bool) -> Self {
        self.on_demand_mode = on_demand;
        self
    }

    /// Controls how [`Xosc32k`] should behave during standby
    #[inline]
    pub fn set_run_standby(mut self, run_standby: bool) -> Self {
        self.run_standby = run_standby;
        self
    }
    /// Busy wait until the clock source is ready
    #[inline]
    pub fn wait_ready(&self) {
        self.token.wait_ready();
    }
}

impl Xosc32k<ClockMode> {
    /// Construct a [`Xosc32k`] from a single pin oscillator clock signal
    #[inline]
    pub fn from_clock(token: Xosc32kToken, xin32: impl AnyPin<Id = PA00>) -> Self {
        // Configure input pin
        let xin32 = xin32.into().into_floating_disabled();
        Self {
            token,
            xin32,
            mode: ClockMode {},
            run_standby: false,
            on_demand_mode: true,
            start_up_masking: StartUp32k::CYCLE2048,
        }
    }

    /// Enable the [`Xosc32k`], allowing it to be used by other peripherals
    ///
    /// To output a 32 kHz clock signal the output must be activated with
    /// the method: [`Enabled<Xosc32k>::activate_32k`]
    #[inline]
    pub fn enable(mut self) -> Enabled<Self, U0> {
        self.token.from_clock();
        self.token.set_on_demand(self.on_demand_mode);
        self.token.set_run_standby(self.run_standby);
        self.token.set_start_up(self.start_up_masking);
        self.token.enable();
        Enabled::new(self)
    }

    /// Deconstruct the [`Xosc32k`] into a Xosc32kToken and the associated GPIO
    /// pin
    #[inline]
    pub fn free(self) -> (Xosc32kToken, XIn32) {
        (self.token, self.xin32)
    }
}

impl Xosc32k<CrystalMode> {
    /// Construct a [`Xosc32k`] from a two pin crystal oscillator signal
    #[inline]
    pub fn from_crystal(
        token: Xosc32kToken,
        xin32: impl AnyPin<Id = PA00>,
        xout32: impl AnyPin<Id = PA01>,
    ) -> Self {
        // Configure input pins
        let xin32 = xin32.into().into_floating_disabled();
        let xout32 = xout32.into().into_floating_disabled();
        // Set to default reset value
        let control_gain_mode_high_speed = false;
        Self {
            token,
            xin32,
            mode: CrystalMode {
                xout32,
                control_gain_mode_high_speed,
            },
            run_standby: false,
            on_demand_mode: true,
            start_up_masking: StartUp32k::CYCLE2048,
        }
    }

    /// Crystal oscillator gain
    ///
    /// Pick between high speed or low power consumption
    #[inline]
    pub fn set_gain_mode(mut self, high_speed: bool) -> Self {
        self.mode.control_gain_mode_high_speed = high_speed;
        self
    }

    /// Enable the [`Xosc32k`], allowing it to be used by other peripherals
    ///
    /// To output a 32 kHz clock signal the output must be activated with
    /// the method: [`Enabled<Xosc32k>::activate_32k`]
    #[inline]
    pub fn enable(mut self) -> Enabled<Self, U0> {
        self.token.from_crystal();
        self.token.set_on_demand(self.on_demand_mode);
        self.token.set_run_standby(self.run_standby);
        self.token.set_start_up(self.start_up_masking);

        self.token
            .set_gain_mode(self.mode.control_gain_mode_high_speed);
        self.token.enable();
        Enabled::new(self)
    }

    /// Deconstruct the [`Xosc32k`] into a Xosc32kToken and the two associated
    /// GPIO pins
    #[inline]
    pub fn free(self) -> (Xosc32kToken, XIn32, XOut32) {
        (self.token, self.xin32, self.mode.xout32)
    }
}

impl<M: Mode> Enabled<Xosc32k<M>, U0> {
    /// Disable the enabled [`Xosc32k`]
    #[inline]
    pub fn disable(mut self) -> Xosc32k<M> {
        self.0.token.activate_32k(false);
        self.0.token.activate_1k(false);
        self.0.token.disable();
        self.0
    }
}

//==============================================================================
// Xosc32kId
//==============================================================================

/// Type-level variant representing the identity of the XOSC32K clock
///
/// This type is a member of several [type-level enums]. See the documentation
/// on [type-level enums] for more details on the pattern.
///
/// [type-level enums]: crate::typelevel#type-level-enum
pub enum Xosc32kId {}

impl Sealed for Xosc32kId {}

//==============================================================================
// Driver
//==============================================================================

impl<M, N> Driver for Enabled<Xosc32k<M>, N>
where
    M: Mode,
    N: Counter,
{
    type Source = Xosc32kId;

    #[inline]
    fn freq(&self) -> Hertz {
        32768.hz()
    }
}
