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

use core::convert::Infallible;
use core::marker::PhantomData;

use typenum::U0;

use crate::pac::oscctrl::xoscctrl::{CFDPRESC_A, STARTUP_A};
use crate::pac::oscctrl::{RegisterBlock, XOSCCTRL};

use crate::gpio::v2::{AnyPin, FloatingDisabled, Pin, PinId, PA14, PA15, PB22, PB23};
use crate::time::Hertz;
use crate::typelevel::{Counter, Sealed};

use super::{Driver, Enabled};

//==============================================================================
// XoscId
//==============================================================================

/// Type-level `enum` for XOSC identifiers
///
/// See the documentation / on [type-level enums] for more details on the
/// pattern.
///
/// [type-level enums]: crate::typelevel#type-level-enum
pub trait XoscId: Sealed {
    /// Corresponding numeric index
    const NUM: usize;
    /// Corresponding XIN [`PinId`]
    type XIn: PinId;
    /// Corresponding XOUT [`PinId`]
    type XOut: PinId;
}

/// Type-level variant representing the identity of XOSC0
///
/// This type is a member of several [type-level enums]. See the documentation
/// on [type-level enums] for more details on the pattern.
///
/// [type-level enums]: crate::typelevel#type-level-enum
pub enum XoscId0 {}

impl Sealed for XoscId0 {}

/// Type which serves as a source marker for the [`super::Xosc0`] and
/// provides numerical identity for it
impl XoscId for XoscId0 {
    const NUM: usize = 0;
    type XIn = PA14;
    type XOut = PA15;
}

/// Type-level variant representing the identity of XOSC1
///
/// This type is a member of several [type-level enums]. See the documentation
/// on [type-level enums] for more details on the pattern.
///
/// [type-level enums]: crate::typelevel#type-level-enum
pub enum XoscId1 {}

impl Sealed for XoscId1 {}

impl XoscId for XoscId1 {
    const NUM: usize = 1;
    type XIn = PB22;
    type XOut = PB23;
}

//==============================================================================
// CrystalCurrent
//==============================================================================

/// Crystal current settings
///
/// The crystal current fields (`IMULT` and `IPTAT`) are usually set by the
/// crystal frequency range. Normally:
/// - 8 MHz -> `Low`      
/// - 8 MHz to 16 MHz -> `Medium`   
/// - 16 MHz to 24 MHz -> `High`     
/// - 24 MHz to 48 MHz -> `ExtraHigh`
///
/// However, the datasheet notes that the next lower setting can be used if the
/// capacitive load is particularly small, which would save power.
#[derive(Clone, Copy, PartialEq, Eq)]
#[allow(missing_docs)]
pub enum CrystalCurrent {
    Low,
    Medium,
    High,
    ExtraHigh,
}

impl CrystalCurrent {
    /// Get the current multiplier
    #[inline]
    fn imult(&self) -> u8 {
        match self {
            Self::Low => 3,
            Self::Medium => 4,
            Self::High => 5,
            Self::ExtraHigh => 6,
        }
    }

    /// Get the current reference
    #[inline]
    fn iptat(&self) -> u8 {
        match self {
            Self::Low => 2,
            Self::Medium => 3,
            Self::High => 3,
            Self::ExtraHigh => 3,
        }
    }
}

impl From<Hertz> for CrystalCurrent {
    fn from(freq: Hertz) -> Self {
        use CrystalCurrent::*;
        match freq.0 {
            _freq @ 8_000_000 => Low,
            _freq @ 8_000_001..=16_000_000 => Medium,
            _freq @ 16_000_001..=24_000_000 => High,
            _freq @ 24_000_001..=48_000_000 => ExtraHigh,
            // The crystal frequency should be known statically.
            // I see no reason to return a `Result`, because the user would
            // have no meaningful way to handle it.
            _ => panic!("Invalid crystal frequency"),
        }
    }
}

//==============================================================================
// XoscToken
//==============================================================================

/// Token struct that is essential in order to construct an instance of an
/// [`Xosc`].
pub struct XoscToken<X: XoscId> {
    id: PhantomData<X>,
}

impl<X: XoscId> XoscToken<X> {
    /// Create a new instance of [`XoscToken`]
    #[inline]
    pub(super) unsafe fn new() -> Self {
        Self { id: PhantomData }
    }

    #[inline]
    fn oscctrl(&self) -> &RegisterBlock {
        unsafe { &*crate::pac::OSCCTRL::ptr() }
    }

    #[inline]
    fn xoscctrl(&self) -> &XOSCCTRL {
        &self.oscctrl().xoscctrl[X::NUM]
    }

    #[inline]
    fn reset(&self) {
        self.xoscctrl().reset();
    }

    #[inline]
    fn set_start_up(&mut self, start_up: StartUp) {
        self.xoscctrl().modify(|_, w| w.startup().variant(start_up));
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
    fn set_source(&mut self, from_crystal: bool) {
        self.xoscctrl().modify(|_, w| w.xtalen().bit(from_crystal));
    }

    #[inline]
    fn enable(&mut self) {
        self.xoscctrl().modify(|_, w| w.enable().bit(true));
    }

    #[inline]
    fn disable(&mut self) {
        self.xoscctrl().modify(|_, w| w.enable().bit(false));
    }

    #[inline]
    fn wait_ready(&self) -> nb::Result<(), Infallible> {
        let mask = 1 << X::NUM;
        if self.oscctrl().status.read().bits() & mask == 0 {
            Err(nb::Error::WouldBlock)
        } else {
            Ok(())
        }
    }

    #[inline]
    #[allow(dead_code)]
    fn set_clock_failure_detection(&mut self, cfden: bool) {
        self.xoscctrl().modify(|_, w| w.cfden().bit(cfden));
    }

    #[inline]
    #[allow(dead_code)]
    fn set_clock_failure_detection_prescaler(&mut self, prescale: CFDPRESC_A) {
        self.xoscctrl()
            .modify(|_, w| w.cfdpresc().variant(prescale));
    }
    #[inline]
    fn set_current(&mut self, current: CrystalCurrent) {
        self.xoscctrl().modify(|_, w| unsafe {
            w.imult().bits(current.imult());
            w.iptat().bits(current.iptat())
        });
    }

    #[inline]
    fn set_amplitude_loop_control(&mut self, enalc: bool) {
        self.xoscctrl().modify(|_, w| w.enalc().bit(enalc));
    }

    #[inline]
    fn set_clock_switch(&mut self, swben: bool) {
        self.xoscctrl().modify(|_, w| w.swben().bit(swben));
    }

    #[inline]
    fn set_low_buf_gain(&mut self, lowbufgain: bool) {
        self.xoscctrl()
            .modify(|_, w| w.lowbufgain().bit(lowbufgain));
    }
}

//==============================================================================
// Aliases
//==============================================================================

/// When Clock Failure Detection (CFD) is active, sets how many clock cycles
/// before monitoring of the clock begins when starting the oscillator.
pub type StartUp = STARTUP_A;

/// Type alias for Xosc Input pin
pub type XIn<X> = Pin<<X as XoscId>::XIn, FloatingDisabled>;

/// Type alias for Xosc Output pin
pub type XOut<X> = Pin<<X as XoscId>::XOut, FloatingDisabled>;

//==============================================================================
// Mode structure for Xosc
//==============================================================================

mod private {
    use super::{CrystalCurrent, Sealed, XOut, XoscId, XoscToken};

    /// Private trait to restrict access to the `configure` function
    pub trait Mode<X: XoscId>: Sealed {
        const FROM_CRYSTAL: bool;
        type Config;
        fn configure(token: &mut XoscToken<X>, config: &Self::Config);
    }

    /// Implementation detail of [`CrystalMode`]
    pub struct CrystalConfig<X: XoscId> {
        pub xout: XOut<X>,
        pub current: CrystalCurrent,
        pub amplitude_loop_control: bool,
        pub low_buf_gain: bool,
    }
}

use private::CrystalConfig;
use private::Mode as PrivateMode;

/// Type-level `enum` for the [`Xosc`] operation mode
///
/// An [`Xosc`] can be sourced from either an external clock or a cyrstal
/// oscillator. This type-level `enum` provides the type-level variants
/// [`ClockMode`] and [`CrystalMode`].
///
/// See the [type-level enum] documentation for more details on the pattern.
///
/// [type-level enum]: crate::typelevel#type-level-enum
pub trait Mode<X: XoscId>: PrivateMode<X> {}

impl<X: XoscId, M: PrivateMode<X>> Mode<X> for M {}

/// Type-level variant of the [`Xosc`] operation [`Mode`]
///
/// Represents the [`Xosc`] configured to use an externally provided clock.
///
/// See the [type-level enum] documentation for more details on the pattern.
///
/// [type-level enum]: crate::typelevel#type-level-enum
pub enum ClockMode {}
impl Sealed for ClockMode {}
impl<X: XoscId> PrivateMode<X> for ClockMode {
    const FROM_CRYSTAL: bool = false;
    type Config = ();
    fn configure(_: &mut XoscToken<X>, _: &Self::Config) {}
}

/// Type-level variant of the [`Xosc`] operation [`Mode`]
///
/// Represents the [`Xosc`] configured to use an external crystal oscillator.
///
/// See the [type-level enum] documentation for more details on the pattern.
///
/// [type-level enum]: crate::typelevel#type-level-enum
pub enum CrystalMode {}
impl Sealed for CrystalMode {}
impl<X: XoscId> PrivateMode<X> for CrystalMode {
    const FROM_CRYSTAL: bool = true;
    type Config = CrystalConfig<X>;
    fn configure(token: &mut XoscToken<X>, config: &Self::Config) {
        token.set_current(config.current);
        token.set_amplitude_loop_control(config.amplitude_loop_control);
        token.set_low_buf_gain(config.low_buf_gain);
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
    M: Mode<X>,
{
    token: XoscToken<X>,
    config: M::Config,
    xin: XIn<X>,
    src_freq: Hertz,
    start_up_cycles: StartUp,
    on_demand: bool,
    run_standby: bool,
    clock_switch: bool,
}

/// Alias for the corresponding [`Xosc`]
pub type Xosc0<M> = Xosc<XoscId0, M>;

/// Alias for the corresponding [`Xosc`]
pub type Xosc1<M> = Xosc<XoscId1, M>;

impl<X, M> Xosc<X, M>
where
    X: XoscId,
    M: Mode<X>,
{
    /// Returns the frequency of the oscillator
    #[inline]
    pub fn freq(&self) -> Hertz {
        self.src_freq
    }

    /// Sets the number of cycles allowed to pass before Clock Failure Detection
    /// (CFD) starts monitoring the external oscillator.
    #[inline]
    pub fn set_start_up(mut self, start_up: StartUp) -> Self {
        self.start_up_cycles = start_up;
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

    /// Xosc lock Switch Enable
    ///
    /// Controls if XOSCn switches back to external clock or crystal in case of
    /// clock recovery
    #[inline]
    pub fn set_clock_switch(mut self, swben: bool) -> Self {
        self.clock_switch = swben;
        self
    }

    /// Modify hardware to realise the desired state
    /// stored within the [`Xosc`]
    ///
    /// Returns the enabled Xosc
    #[inline]
    pub fn enable(mut self) -> Enabled<Xosc<X, M>, U0> {
        self.token.reset();
        self.token.set_source(M::FROM_CRYSTAL);
        self.token.set_start_up(self.start_up_cycles);
        self.token.set_on_demand(self.on_demand);
        self.token.set_run_standby(self.run_standby);
        self.token.set_clock_switch(self.clock_switch);
        M::configure(&mut self.token, &self.config);
        self.token.enable();
        Enabled::new(self)
    }
}

impl<X: XoscId> Xosc<X, ClockMode> {
    /// Construct a [`Xosc`] from a single pin oscillator clock signal
    #[inline]
    pub fn from_clock(
        token: XoscToken<X>,
        xin: impl AnyPin<Id = X::XIn>,
        src_freq: impl Into<Hertz>,
    ) -> Self {
        let xin = xin.into().into_floating_disabled();
        let start_up_cycles = StartUp::CYCLE1;
        // Mimic default reset state
        let on_demand = true;
        let run_standby = false;
        let clock_switch = false;
        Self {
            token,
            config: (),
            xin,
            src_freq: src_freq.into(),
            start_up_cycles,
            on_demand,
            run_standby,
            clock_switch,
        }
    }

    /// Deconstruct the Xosc and return the inner XoscToken
    #[inline]
    pub fn free(self) -> (XoscToken<X>, XIn<X>) {
        (self.token, self.xin)
    }
}

impl<X: XoscId> Xosc<X, CrystalMode> {
    /// Construct a [`Xosc`] from a two pin crystal oscillator signal
    ///
    /// The crystal oscillator frequency must be supported, for valid
    /// frequencies see [`CrystalCurrent`].
    ///
    /// By default `Amplitude Loop Control` is set, see
    /// [`Xosc::set_amplitude_loop_control`]
    #[inline]
    pub fn from_crystal(
        token: XoscToken<X>,
        xin: impl AnyPin<Id = X::XIn>,
        xout: impl AnyPin<Id = X::XOut>,
        src_freq: impl Into<Hertz>,
    ) -> Self {
        let xin = xin.into().into_floating_disabled();
        let xout = xout.into().into_floating_disabled();

        let src_freq = src_freq.into();
        let current = src_freq.into();

        // Lowers power usage and protects the crystal
        let amplitude_loop_control = true;

        let low_buf_gain = false;

        let start_up_cycles = StartUp::CYCLE1;
        let on_demand = true;
        let run_standby = false;
        let clock_switch = false;
        let config = CrystalConfig {
            xout,
            current,
            amplitude_loop_control,
            low_buf_gain,
        };
        Self {
            token,
            config,
            xin,
            src_freq,
            start_up_cycles,
            on_demand,
            run_standby,
            clock_switch,
        }
    }

    /// Sets the current drive strength for the crystal
    ///
    /// See [CrystalCurrent] for possible values
    #[inline]
    pub fn set_current(mut self, current: CrystalCurrent) -> Self {
        self.config.current = current;
        self
    }

    /// Controls the automatic amplitude loop control
    ///
    /// Recommended option, ensures the crystal is not overdriven,
    /// and lowers power consumption. See datasheet c. 54.13 p. 1811
    #[inline]
    pub fn set_amplitude_loop_control(mut self, enalc: bool) -> Self {
        self.config.amplitude_loop_control = enalc;
        self
    }

    /// If `LOWBUFGAIN`is set when `ENALC` is enabled,
    /// the oscillators amplitude is increased by approximately a factor 2.
    ///
    /// Default value (0) should be used together with low amplitude
    /// oscillators. Can be used to solve stability issues.
    #[inline]
    pub fn set_low_buf_gain(mut self, lowbufgain: bool) -> Self {
        self.config.low_buf_gain = lowbufgain;
        self
    }

    /// Deconstruct the Xosc and return the inner XoscToken
    #[inline]
    pub fn free(self) -> (XoscToken<X>, XIn<X>, XOut<X>) {
        (self.token, self.xin, self.config.xout)
    }
}

impl<X, M> Enabled<Xosc<X, M>, U0>
where
    X: XoscId,
    M: Mode<X>,
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

impl<X, M, N> Enabled<Xosc<X, M>, N>
where
    X: XoscId,
    M: Mode<X>,
    N: Counter,
{
    /// Busy-wait until ready
    #[inline]
    pub fn wait_ready(&self) -> nb::Result<(), Infallible> {
        self.0.token.wait_ready()
    }
}

//==============================================================================
// Driver
//==============================================================================

impl<X, M, N> Driver for Enabled<Xosc<X, M>, N>
where
    X: XoscId,
    M: Mode<X>,
    N: Counter,
{
    type Source = X;

    #[inline]
    fn freq(&self) -> Hertz {
        self.0.freq()
    }
}
