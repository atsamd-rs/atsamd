#![warn(missing_docs)]
//! # Digital Frequency Locked Loop
//!
//! Dfll is an internal 48 MHz oscillator that provides two different modes of
//! operation
//!
//! - [`Enabled`]`<`[`Dfll`]`<`[`OpenLoop`]`>, _>`: Dfll operates as a
//!   stand-alone, high-frequency oscillator (default)
//! - [`Enabled`]`<`[`Dfll`]`<`[`ClosedLoop`]`<_>>, _>`: Dfll engages internal
//!   frequency tuner operating against the external reference clock signal to
//!   tune internally produced signal (e.g. drifting)
//!
//!   While in [`ClosedLoop`] mode, two extra submodes can be derived
//!   - [`Enabled`]`<`[`Dfll`]`<`[`ClosedLoop`]`<`[`FromPclk`]`<_>>>>`:
//!     Designated [`Pclk`]`<`[`DfllId`]`, _>` serves as a reference clock
//!   - [`Enabled`]`<`[`Dfll`]`<`[`ClosedLoop`]`<`[`FromUsb`]`>>>`: Reference
//!     clock signal is derived from SOF bit showing up on the USB bus every 1ms
//!
//! `Dfll` in a default state is provided in a return value of
//! [`clock_system_at_reset`].
//!
//! Configuring a `Dfll` proceeds according to the principles outlined in the
//! [`clock` module documentation]. It is best shown with an example.
//!
//! ## Example
//!
//! Suppose we start with the default clock tree after power-on reset.
//!
//! ```text
//! DFLL (48 MHz; open loop mode)
//! └── GCLK0 (48 MHz)
//!     └── Master clock (48 MHz)
//! ```
//!
//! We would like to transform it to a clock tree like this:
//!
//! ```text
//! DFLL (48 MHz; USB based closed loop mode)
//! └── GCLK0 (48 MHz)
//!     └── Master clock (48 MHz)
//! ```
//!
//! Let's start by using [`clock_system_at_reset`] to access the HAL clocking
//! structs.
//!
//! ```no_run
//! use atsamd_hal::{
//!     clock::v2::{
//!         clock_system_at_reset,
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
//! We can use the helper method [`EnabledDfll::to_mode`] to switch
//! `Dfll` from one mode to another. This method is provided only for
//! `EnabledDfll<U1>` with `EnabledGclk0<DfllId, U1>` which is very common
//! clocking configuration. Without it, `EnabledGclk0` would have to be
//! temporarily switched to a different producer clock so `EnabledDfll` could be
//! disabled and deconstructed.
//!
//! ```no_run
//! # use atsamd_hal::{
//! #     time::U32Ext,
//! #     clock::v2::{
//! #         clock_system_at_reset,
//! #         dfll::*,
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
//! let (dfll, _, gclk0) = clocks.dfll.to_mode(
//!     clocks.gclk0,
//!     ClosedLoop {
//!         mode: FromUsb,
//!         coarse_max_step: 0xA,
//!         fine_max_step: 0xA,
//!     }
//! );
//! ```
//!
//! As mentioned before, in all other cases `EnabledDfll` has to be disabled and
//! reconstructed via [`Dfll::new`]
//!
//! ```no_run
//! # use atsamd_hal::{
//! #     time::U32Ext,
//! #     clock::v2::{
//! #         clock_system_at_reset,
//! #         dfll::*,
//! #         pclk::Pclk,
//! #         gclk::Gclk5,
//! #         osculp32k::OscUlp1k,
//! #         osculp32k::OscUlp32k,
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
//! let (osculp32k, base) = OscUlp32k::enable(tokens.osculp32k.osculp32k, clocks.osculp32k_base);
//! let (gclk0, dfll, osculp32k) = clocks.gclk0.swap_sources(clocks.dfll, osculp32k);
//! let (token, _) = dfll.disable().free();
//! let (pclk, _) = Pclk::enable(tokens.pclks.dfll, gclk0);
//! let dfll = Dfll::new(token, ClosedLoop {
//!         // Note: this configuration is just an example,
//!         // using internal oscillator as a reference clock
//!         // is probably not desirable; the same applies to
//!         // max step values
//!         mode: FromPclk::new(pclk),
//!         // 1464 * 32_768 Hz -> ~48 MHz
//!         coarse_max_step: 0x1,
//!         fine_max_step: 0xA,
//!     }
//! );
//! let dfll = dfll.enable();
//! let (gclk5, dfll) = Gclk5::from_source(tokens.gclks.gclk5, dfll);
//! ```
//!
//! [`clock_system_at_reset`]: super::clock_system_at_reset
//! [`clock` module documentation]: super
use typenum::{U0, U1};

use crate::time::{Hertz, U32Ext};
use crate::typelevel::{PrivateIncrement, Sealed};

use super::gclk::{EnabledGclk0, GclkId};
use super::pclk::Pclk;
use super::{Enabled, Source};

//==============================================================================
// DfllId
//==============================================================================

/// Type-level variant representing the identity of the DFLL clock
///
/// This type is a member of several [type-level enums]. See the documentation
/// on [type-level enums] for more details on the pattern.
///
/// [type-level enums]: crate::typelevel#type-level-enum
pub enum DfllId {}

impl Sealed for DfllId {}

//==============================================================================
// DfllToken
//==============================================================================

/// Token type required to construct a [`Dfll`] type instance.
///
/// From a [`atsamd_hal`][`crate`] external user perspective, it does not
/// contain any methods and serves only a token purpose.
///
/// Within a [`atsamd_hal`][`crate`], [`DfllToken`] struct is a low-level access
/// abstraction for HW register calls.
pub struct DfllToken(());

impl DfllToken {
    /// Constructor
    ///
    /// Unsafe: There should always be only a single instance thereof. It can be
    /// retrieved upon disabling and freeing an [`Enabled`]`<`[`Dfll`]`>`
    /// instance returned from `crate::clock::v2::retrieve_clocks` method
    #[inline]
    pub(crate) unsafe fn new() -> Self {
        Self(())
    }

    #[inline]
    fn oscctrl(&self) -> &crate::pac::oscctrl::RegisterBlock {
        unsafe { &*crate::pac::OSCCTRL::ptr() }
    }

    #[inline]
    fn dfllctrla(&self) -> &crate::pac::oscctrl::DFLLCTRLA {
        &self.oscctrl().dfllctrla
    }

    #[inline]
    fn dfllctrlb(&self) -> &crate::pac::oscctrl::DFLLCTRLB {
        &self.oscctrl().dfllctrlb
    }

    #[allow(dead_code)]
    #[inline]
    fn dfllval(&self) -> &crate::pac::oscctrl::DFLLVAL {
        &self.oscctrl().dfllval
    }

    #[inline]
    fn dfllmul(&self) -> &crate::pac::oscctrl::DFLLMUL {
        &self.oscctrl().dfllmul
    }

    #[inline]
    fn dfllsync(&self) -> &crate::pac::oscctrl::DFLLSYNC {
        &self.oscctrl().dfllsync
    }

    #[inline]
    fn wait_sync_enable(&self) {
        while self.dfllsync().read().enable().bit() {}
    }

    #[inline]
    fn wait_sync_dfllmul(&self) {
        while self.dfllsync().read().dfllmul().bit() {}
    }

    #[allow(dead_code)]
    #[inline]
    fn wait_sync_dfllval(&self) {
        while self.dfllsync().read().dfllval().bit() {}
    }

    #[inline]
    fn wait_sync_dfllctrlb(&self) {
        while self.dfllsync().read().dfllctrlb().bit() {}
    }

    #[inline]
    fn set_on_demand_mode(&mut self, value: bool) {
        self.dfllctrla().modify(|_, w| w.ondemand().bit(value));
    }

    #[inline]
    fn set_run_standby(&mut self, value: bool) {
        self.dfllctrla().modify(|_, w| w.runstdby().bit(value));
    }

    #[inline]
    fn enable(&mut self) {
        self.dfllctrla().modify(|_, w| w.enable().set_bit());
        self.wait_sync_enable();
    }

    #[inline]
    fn disable(&mut self) {
        self.dfllctrla().modify(|_, w| w.enable().clear_bit());
        self.wait_sync_enable();
    }

    #[inline]
    fn set_usb_clock_recovery_mode(&mut self, value: bool) {
        self.dfllctrlb().modify(|_, w| w.usbcrm().bit(value));
        self.wait_sync_dfllctrlb();
    }

    #[inline]
    fn set_quick_lock(&mut self, value: bool) {
        // QLDIS is Quick Lock Disable register; thus value has to be negated
        self.dfllctrlb().modify(|_, w| w.qldis().bit(!value));
        self.wait_sync_dfllctrlb();
    }

    #[inline]
    fn set_chill_cycle(&mut self, value: bool) {
        // CCDIS is Chill Cycle Disable register; thus value has to be negated
        self.dfllctrlb().modify(|_, w| w.ccdis().bit(!value));
        self.wait_sync_dfllctrlb();
    }

    #[inline]
    fn set_mode(&mut self, mode: DynMode) {
        let bit = match mode {
            DynMode::OpenLoop => false,
            DynMode::ClosedLoop => true,
        };
        self.dfllctrlb().modify(|_, w| w.mode().bit(bit));
        self.wait_sync_dfllctrlb();
    }

    #[inline]
    fn set_fine_max_step(&mut self, value: u8) {
        self.dfllmul()
            .modify(|_, w| unsafe { w.fstep().bits(value) });
        self.wait_sync_dfllmul();
    }

    #[inline]
    fn set_coarse_max_step(&mut self, value: u8) {
        self.dfllmul()
            .modify(|_, w| unsafe { w.cstep().bits(value) });
        self.wait_sync_dfllmul();
    }

    #[inline]
    fn set_mult_factor(&mut self, value: u16) {
        self.dfllmul().modify(|_, w| unsafe { w.mul().bits(value) });
        self.wait_sync_dfllmul();
    }
}

//==============================================================================
// Aliases
//==============================================================================

type MultFactor = u16;
type CoarseMaxStep = u8;
type FineMaxStep = u8;

//==============================================================================
// Mode
//==============================================================================

/// Value-level version of [`Mode`]
///
/// Represents the loop mode of the DFLL
#[allow(missing_docs)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum DynMode {
    OpenLoop,
    ClosedLoop,
}

/// Type-level `enum` for the [`Dfll`] loop mode
///
/// The DFLL can operate in either [`OpenLoop`] mode or [`ClosedLoop`] mode.
/// See the [type-level enum] documentation for more details on the pattern.
///
/// [type-level enum]: crate::typelevel#type-level-enum
pub trait Mode: Sealed {
    /// Performs a mode specific HW register writes. Should not be called by an
    /// end user.
    fn enable(&self, token: &mut DfllToken);
    /// Calculates the output frequency of Dfll for given mode
    fn freq(&self) -> Hertz;
}

/// Type-level variant of [`Mode`] representing open loop operation of the DFLL
///
/// See the [type-level enum] documentation for more details on the pattern.
///
/// [type-level enum]: crate::typelevel#type-level-enum
pub struct OpenLoop;

impl Sealed for OpenLoop {}

impl Mode for OpenLoop {
    fn enable(&self, token: &mut DfllToken) {
        token.set_mode(DynMode::OpenLoop);
    }

    fn freq(&self) -> Hertz {
        48.mhz().into()
    }
}

/// Type-level `enum` for the [`Dfll`] loop mode
///
/// The DFLL can operate in either [`OpenLoop`] mode or [`ClosedLoop`] mode.
/// See the [type-level enum] documentation for more details on the pattern.
///
/// [type-level enum]: crate::typelevel#type-level-enum
pub trait ClosedLoopSubmode {
    /// Performs a submode specific HW register writes. Should not be called by
    /// an end user.
    fn enable(&self, token: &mut DfllToken);
    /// Calculates the output frequency of Dfll for given submode
    fn freq(&self) -> Hertz;
}

/// Type-level variant of [`ClosedLoopSubmode`] representing a Dfll's closed
/// loop submode in which a dedicated [`Pclk`]`<`[`DfllId`]`, _>` is being used
/// as a source of reference clock.
///
/// More details regarding the usage together with example code can be found in
/// [module-level documentation](self).
pub struct FromPclk<G: GclkId> {
    pclk: Pclk<DfllId, G>,
    mult_factor: MultFactor,
    chill_cycle: bool,
    quick_lock: bool,
}

impl<G: GclkId> FromPclk<G> {
    /// Constructor for Pclk based closed loop submode. It derives
    /// multiplication factor from a frequency of provided [`Pclk`] so the
    /// output frequency is as close to 48 MHz as possible.
    ///
    /// Unsafe, non-panicking alternative is provided via
    /// [`FromPclk::new_unchecked`]
    ///
    /// # Panics
    /// Panics if provided [`Pclk`] frequency is not in range of `[732, 33_000]`
    /// Hz
    ///
    /// See the datasheet for more details (54.13.4 Digital Frequency Locked
    /// Loop (DFLL48M) Characteristics)
    pub fn new(pclk: Pclk<DfllId, G>) -> Self {
        let pclk_freq = pclk.freq().0;
        let min_pclk_freq = 48.mhz().0 / MultFactor::MAX as u32;

        if pclk_freq < min_pclk_freq || pclk_freq > 33_000 {
            panic!("Invalid Pclk<DfllId, _> input frequency");
        }

        // Cast is fine because division result cannot be greater than u16::MAX
        let mult_factor = (48.mhz().0 / pclk_freq) as u16;
        unsafe { Self::new_unchecked(pclk, mult_factor) }
    }

    /// Constructor for Pclk based closed loop submode.
    ///
    /// # Safety
    /// Correctness of input parameters is assumed:
    /// - `pclk` frequency in range `[732, 33_000`] Hz
    /// - `mult_factor` cannot yield out-of-spec output frequency for Dfll
    pub unsafe fn new_unchecked(pclk: Pclk<DfllId, G>, mult_factor: MultFactor) -> Self {
        Self {
            pclk,
            mult_factor,
            chill_cycle: true,
            quick_lock: true,
        }
    }

    /// Controls the chill cycle functionality. Default value is `true`
    pub fn set_chill_cycle(mut self, value: bool) -> Self {
        self.chill_cycle = value;
        self
    }

    /// Controls quick lock functionality. Default value is `true`
    pub fn set_quick_lock(mut self, value: bool) -> Self {
        self.quick_lock = value;
        self
    }
}

impl<G: GclkId> ClosedLoopSubmode for FromPclk<G> {
    fn enable(&self, token: &mut DfllToken) {
        token.set_usb_clock_recovery_mode(false);
        token.set_mult_factor(self.mult_factor);
        token.set_chill_cycle(self.chill_cycle);
        token.set_chill_cycle(self.quick_lock);
    }

    fn freq(&self) -> Hertz {
        Hertz(self.pclk.freq().0 * self.mult_factor as u32)
    }
}

/// Type-level variant of [`ClosedLoopSubmode`] representing a Dfll's closed
/// loop submode in which a reference clock is derived from SOF bits showing up
/// every 1ms on a (configured) USB bus
///
/// More details regarding the usage together with example code can be found in
/// [module-level documentation](self).
// TODO: 54.15; Table 54-68; allegedly fine step value must be 0xA for USB mode?
pub struct FromUsb;

impl ClosedLoopSubmode for FromUsb {
    fn enable(&self, token: &mut DfllToken) {
        token.set_usb_clock_recovery_mode(true);
        // 48 MHz / 1 kHz (1 ms SOF for USB)
        //
        // See Datasheet, `USB Clock Recovery Mode` in 28.6.4.2
        token.set_mult_factor(48_000);
        // Datasheet recommends using chill cycle and quick lock for USB based closed
        // loop mode
        token.set_chill_cycle(true);
        token.set_quick_lock(true);
    }

    fn freq(&self) -> Hertz {
        48.mhz().into()
    }
}

/// Type-level variant of [`Mode`] representing closed loop operation of the
/// DFLL
///
/// In closed loop operation, depending on the submode the DFLL output is
/// referenced either to the its [`Pclk`] ([`FromPclk`] submode) or derived from
/// USB bus ([`FromUsb`] submode).
///
/// See the [type-level enum] documentation for more details on the pattern.
///
/// More details regarding the usage together with example code can be found in
/// [module-level documentation](self).
///
/// [type-level enum]: crate::typelevel#type-level-enum
pub struct ClosedLoop<M> {
    /// Closed loop submode
    ///
    /// [`ClosedLoopSubmode`] implementing struct is expected
    pub mode: M,
    /// Maximum step size allowed during a process a frequency tuning for
    /// a coarse parameter.
    ///
    /// Consult datasheet regarding what kind of set of parameters is acceptable
    /// (c. 28.6.4.1, Closed-Loop Operation). Otherwise, [`Dfll`] behavior might
    /// be incorrect
    pub coarse_max_step: CoarseMaxStep,
    /// Maximum step size allowed during a process a frequency tuning for
    /// a fine parameter.
    ///
    /// Consult datasheet regarding what kind of set of parameters is acceptable
    /// (c. 28.6.4.1, Closed-Loop Operation). Otherwise, [`Dfll`] behavior might
    /// be incorrect
    pub fine_max_step: FineMaxStep,
}

impl<M> Sealed for ClosedLoop<M> {}

impl<M: ClosedLoopSubmode> Mode for ClosedLoop<M> {
    fn enable(&self, token: &mut DfllToken) {
        self.mode.enable(token);
        token.set_coarse_max_step(self.coarse_max_step);
        token.set_fine_max_step(self.fine_max_step);
        token.set_mode(DynMode::ClosedLoop);
    }

    fn freq(&self) -> Hertz {
        self.mode.freq()
    }
}

//==============================================================================
// Dfll
//==============================================================================

/// Struct representing a [`Dfll`] abstraction
///
/// It is generic over the supported modes of operation
pub struct Dfll<M: Mode> {
    token: DfllToken,
    mode: M,
    run_standby: bool,
    on_demand_mode: bool,
}

impl<M: Mode> Dfll<M> {
    /// Returns the frequency of the [`Dfll`]
    #[inline]
    pub fn freq(&self) -> Hertz {
        self.mode.freq()
    }

    /// Controls the clock source behaviour during standby
    ///
    /// See Datasheet c. 28.6.4.1
    #[inline]
    pub fn set_run_standby(&mut self, value: bool) {
        self.run_standby = value;
    }

    /// Controls the on demand functionality of the clock source
    ///
    /// Only starts the clock source when a peripheral uses it. If cleared the
    /// clock will be always active
    ///
    /// See Datasheet c. 13.5 for general information; 28.6.4.1 for [`Dfll`]
    /// specific details
    #[inline]
    pub fn set_on_demand_mode(&mut self, value: bool) {
        self.on_demand_mode = value;
    }

    /// Enables [`Dfll`]
    ///
    /// This method modifies hardware to match the configuration stored within
    #[inline]
    pub fn enable(mut self) -> EnabledDfll<M> {
        self.mode.enable(&mut self.token);
        self.token.set_on_demand_mode(self.on_demand_mode);
        self.token.set_run_standby(self.run_standby);
        self.token.enable();
        Enabled::new(self)
    }

    /// Create [`Dfll`] in a mode `M`
    ///
    /// Creating a [`Dfll`] does not modify any of the hardware registers. It
    /// only creates a struct to track the `Dfll` configuration.
    ///
    /// The configuration data is stored until the user calls [`Dfll::enable`].
    /// At that point, all of the registers are written according to the
    /// initialization procedures specified in the datasheet, and an
    /// [`EnabledDfll`] is returned. The `Dpll` is not active or useful until
    /// that point.
    #[inline]
    pub fn new(token: DfllToken, mode: M) -> Self {
        let dfll = Self {
            token,
            mode,
            run_standby: false,
            on_demand_mode: true,
        };

        dfll
    }

    /// Release the resources
    #[inline]
    pub fn free(self) -> (DfllToken, M) {
        (self.token, self.mode)
    }
}

impl<M: ClosedLoopSubmode> Dfll<ClosedLoop<M>> {
    /// Set a maximum step size allowed during a process a frequency tuning for
    /// a coarse parameter.
    ///
    /// Consult datasheet regarding what kind of set of parameters is acceptable
    /// (c. 28.6.4.1, Closed-Loop Operation). Otherwise, [`Dfll`] behavior might
    /// be incorrect
    #[inline]
    pub fn set_coarse_max_step(&mut self, coarse_max_step: CoarseMaxStep) {
        self.mode.coarse_max_step = coarse_max_step;
    }

    /// Set a maximum step size allowed during a process a frequency tuning for
    /// a fine parameter.
    ///
    /// Consult datasheet regarding what kind of set of parameters is acceptable
    /// (c. 28.6.4.1, Closed-Loop Operation). Otherwise, [`Dfll`] behavior might
    /// be incorrect
    #[inline]
    pub fn set_fine_max_step(&mut self, fine_max_step: FineMaxStep) {
        self.mode.fine_max_step = fine_max_step;
    }
}

impl<M: Mode> EnabledDfll<M> {
    /// Disable the [`Dfll`]
    #[inline]
    pub fn disable(mut self) -> Dfll<M> {
        self.0.token.disable();
        self.0
    }
}

/// An [`Enabled`] [`Dfll`]
///
/// As described in the [`clock` module documentation](super), the [`Enabled`]
/// wrapper implements compile-time clock tree safety by tracking the number of
/// consumer clocks and restricting access to the underlying [`Dfll`] to prevent
/// modification while in use.
///
/// As with [`Enabled`], the default value for `N` is `U0`; if left unspecified,
/// the counter is assumed to be zero.
pub type EnabledDfll<M, N = U0> = Enabled<Dfll<M>, N>;

impl<OldM: Mode> EnabledDfll<OldM, U1> {
    /// Helper method allowing to change a mode of [`Dfll`] operation in place.
    ///
    /// It is implemented only for an enabled [`Dfll`] having a single user
    /// (which is a [`EnabledGclk0`]). Without it, it becomes unwieldy to change
    /// a mode of a [`Dfll`], which is a very common use case.
    #[inline]
    pub fn to_mode<NewM: Mode>(
        self,
        gclk0: EnabledGclk0<DfllId, U1>,
        mode: NewM,
    ) -> (EnabledDfll<NewM, U1>, OldM, EnabledGclk0<DfllId, U1>) {
        let (token, old_mode) = self.0.free();
        let dfll = Dfll::new(token, mode);
        (dfll.enable().inc(), old_mode, gclk0)
    }
}

//==============================================================================
// Source
//==============================================================================

impl<M: Mode, N> Source for EnabledDfll<M, N> {
    type Id = DfllId;

    #[inline]
    fn freq(&self) -> Hertz {
        self.0.freq()
    }
}
