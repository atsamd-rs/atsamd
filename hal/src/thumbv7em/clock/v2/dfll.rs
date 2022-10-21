//! # DFLL48M - Digital Frequency Locked Loop
//!
//! Dfll is an internal 48 MHz oscillator that provides two different modes of
//! operation
//!
//! - [`Enabled`]`<`[`Dfll`]`<`[`OpenLoop`]`>>`: Dfll operates as a stand-alone,
//!   high-frequency oscillator (default)
//! - [`Enabled`]`<`[`Dfll`]`<`[`ClosedLoop`]`>>`: Dfll engages internal
//!   frequency tuner operating against the external reference clock signal to
//!   tune internally produced signal (e.g. drifting)
//!
//! Dfll in a default state is provided
//! - in a return value of [`crate::clock::v2::retrieve_clocks`]
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
    fn enable(&self, token: &mut DfllToken);
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

pub trait ClosedLoopSubmode {
    fn enable(&self, token: &mut DfllToken);
    fn freq(&self) -> Hertz;
}

pub struct FromPclk<G: GclkId> {
    pclk: Pclk<DfllId, G>,
    mult_factor: MultFactor,
    chill_cycle: bool,
    quick_lock: bool,
}

impl<G: GclkId> FromPclk<G> {
    // 54.13.4 Digital Frequency Locked Loop (DFLL48M) Characteristics
    //
    // Table 54-51. DFLL48M Characteristics - Closed Loop Mode
    //
    // Input reference frequency:
    // - Min: 732 Hz
    // - Typical: 32_768 Hz
    // - Max: 33 kHz
    //
    // Notes:
    //
    // 1. These values are based on simulation. They are not covered by production
    // test limits or characterization.
    //
    // 2. To ensure that the device stays
    // within the maximum allowed clock frequency, any reference clock for the DFLL
    // in close loop must be within 2% error accuracy.
    pub fn new(pclk: Pclk<DfllId, G>, mult_factor: MultFactor) -> Self {
        let pclk_freq = pclk.freq().0;
        if pclk_freq < 700 || pclk_freq > 33_000 {
            panic!("Invalid Pclk<DfllId, _> input frequency");
        }
        unsafe { Self::new_unchecked(pclk, mult_factor) }
    }

    pub unsafe fn new_unchecked(pclk: Pclk<DfllId, G>, mult_factor: MultFactor) -> Self {
        Self {
            pclk,
            mult_factor,
            chill_cycle: true,
            quick_lock: true,
        }
    }
    pub fn chill_cycle(mut self, value: bool) -> Self {
        self.chill_cycle = value;
        self
    }
    pub fn quick_lock(mut self, value: bool) -> Self {
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

struct FromUsb;

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
/// In closed loop operation, the DFLL output is referenced to the its [`Pclk`].
/// Consequently, this type takes a [`GclkId`] representing the
/// [`PclkSourceId`](super::pclk::PclkSourceId) of the corresponding `Pclk`.
///
/// See the [type-level enum] documentation for more details on the pattern.
///
/// [type-level enum]: crate::typelevel#type-level-enum
pub struct ClosedLoop<M> {
    pub mode: M,
    pub coarse_max_step: CoarseMaxStep,
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

    /// Enabling a [`Dfll`] modifies hardware to match the configuration stored
    /// within
    #[inline]
    pub fn enable(mut self) -> EnabledDfll<M> {
        self.mode.enable(&mut self.token);
        self.token.set_on_demand_mode(self.on_demand_mode);
        self.token.set_run_standby(self.run_standby);
        self.token.enable();
        Enabled::new(self)
    }

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

impl<G: GclkId> Dfll<ClosedLoop<FromPclk<G>>> {
    /// Set a multiplication factor for an input frequency
    ///
    /// Consult datasheet regarding what kind of set of parameters is acceptable
    /// (c. 28.6.4.1, Closed-Loop Operation). Otherwise, [`Dfll`] behavior might
    /// be incorrect
    #[inline]
    pub fn set_mult_factor(&mut self, mult_factor: MultFactor) {
        self.mode.mode.mult_factor = mult_factor;
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

pub type EnabledDfll<M, N = U0> = Enabled<Dfll<M>, N>;

impl<OldM: Mode> EnabledDfll<OldM, U1> {
    /// Special, helper method allowing to change a mode of [`Dfll`] operation
    /// in place. It is implemented only for an enabled [`Dfll`] having a single
    /// user (which is a [`Gclk0`]). Without it, it becomes unwieldy to change a
    /// mode of a [`Dfll`] actively used by [`Gclk0`], which is a very common
    /// scenario
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
