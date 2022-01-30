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
use crate::typelevel::{Counter, PrivateIncrement, Sealed};

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
    /// Corresponding [`DynMode`]
    const DYN: DynMode;
    /// Get the coarse maximum step (only valid in [`ClosedLoop`] mode)
    fn coarse_max_step(&self) -> CoarseMaxStep;
    /// Get the fine maximum step (only valid in [`ClosedLoop`] mode)
    fn fine_max_step(&self) -> FineMaxStep;
}

/// Type-level variant of [`Mode`] representing open loop operation of the DFLL
///
/// See the [type-level enum] documentation for more details on the pattern.
///
/// [type-level enum]: crate::typelevel#type-level-enum
pub struct OpenLoop(());

impl Sealed for OpenLoop {}

impl Mode for OpenLoop {
    const DYN: DynMode = DynMode::OpenLoop;
    #[inline]
    fn coarse_max_step(&self) -> CoarseMaxStep {
        0
    }
    #[inline]
    fn fine_max_step(&self) -> FineMaxStep {
        0
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
pub struct ClosedLoop<G: GclkId> {
    pclk: Pclk<DfllId, G>,
    coarse_max_step: CoarseMaxStep,
    fine_max_step: FineMaxStep,
}

impl<G: GclkId> Sealed for ClosedLoop<G> {}

impl<G: GclkId> Mode for ClosedLoop<G> {
    const DYN: DynMode = DynMode::ClosedLoop;
    #[inline]
    fn coarse_max_step(&self) -> CoarseMaxStep {
        self.coarse_max_step
    }
    #[inline]
    fn fine_max_step(&self) -> FineMaxStep {
        self.fine_max_step
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
    freq: Hertz,
    mode: M,
    mult_factor: MultFactor,
    run_standby: bool,
    on_demand_mode: bool,
}

impl<M: Mode> Dfll<M> {
    /// Returns the frequency of the [`Dfll`]
    #[inline]
    pub fn freq(&self) -> Hertz {
        Hertz(self.freq.0 * self.mult_factor as u32)
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
        if M::DYN == DynMode::ClosedLoop {
            self.token.set_coarse_max_step(self.mode.coarse_max_step());
            self.token.set_fine_max_step(self.mode.fine_max_step());
        }
        self.token.set_mode(M::DYN);
        self.token.set_on_demand_mode(self.on_demand_mode);
        self.token.set_run_standby(self.run_standby);
        self.token.set_mult_factor(self.mult_factor);
        self.token.enable();
        Enabled::new(self)
    }
}

impl Dfll<OpenLoop> {
    /// Constructs a builder of [`Dfll`] in an [`OpenLoop`]. To affect the
    /// hardware, it requires an additional call to [`Dfll::enable`]
    #[inline]
    pub fn in_open_mode(token: DfllToken) -> Dfll<OpenLoop> {
        Self {
            token,
            freq: 48.mhz().into(),
            mode: OpenLoop(()),
            mult_factor: 1,
            run_standby: false,
            on_demand_mode: true,
        }
    }

    /// Release the resources
    #[inline]
    pub fn free(self) -> DfllToken {
        self.token
    }
}

impl<G: GclkId> Dfll<ClosedLoop<G>> {
    /// Constructs a builder of [`Dfll`] in a [`ClosedLoop`]. To affect the
    /// hardware, it requires an additional call to [`Dfll::enable`]
    #[inline]
    pub fn in_closed_mode(
        token: DfllToken,
        pclk: Pclk<DfllId, G>,
        mult_factor: MultFactor,
        coarse_max_step: CoarseMaxStep,
        fine_max_step: FineMaxStep,
    ) -> Dfll<ClosedLoop<G>> {
        let freq = pclk.freq();
        let mode = ClosedLoop {
            pclk,
            coarse_max_step,
            fine_max_step,
        };
        Self {
            token,
            freq,
            mode,
            mult_factor,
            run_standby: false,
            on_demand_mode: true,
        }
    }
    /// Set a multiplication factor for an input frequency
    ///
    /// Consult datasheet regarding what kind of set of parameters is acceptable
    /// (c. 28.6.4.1, Closed-Loop Operation). Otherwise, [`Dfll`] behavior might
    /// be incorrect
    #[inline]
    pub fn set_mult_factor(&mut self, mult_factor: MultFactor) {
        self.mult_factor = mult_factor;
    }

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

    /// Release the resources
    #[inline]
    pub fn free(self) -> (DfllToken, Pclk<DfllId, G>) {
        (self.token, self.mode.pclk)
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

impl EnabledDfll<OpenLoop, U1> {
    /// Special, helper method allowing to change a mode of [`Dfll`] operation
    /// in place. It is implemented only for an enabled [`Dfll`] having a single
    /// user (which is a [`Gclk0`]). Without it, it becomes unwieldy to change a
    /// mode of a [`Dfll`] actively used by [`Gclk0`], which is a very common
    /// scenario
    #[inline]
    pub fn to_closed_mode<G: GclkId>(
        self,
        gclk0: EnabledGclk0<DfllId, U1>,
        reference_clk: Pclk<DfllId, G>,
        multiplication_factor: MultFactor,
        coarse_maximum_step: CoarseMaxStep,
        fine_maximum_step: FineMaxStep,
    ) -> (EnabledDfll<ClosedLoop<G>, U1>, EnabledGclk0<DfllId, U1>) {
        let token = self.0.free();
        let dfll = Dfll::in_closed_mode(
            token,
            reference_clk,
            multiplication_factor,
            coarse_maximum_step,
            fine_maximum_step,
        );
        (dfll.enable().inc(), gclk0)
    }
}

impl<G: GclkId> EnabledDfll<ClosedLoop<G>, U1> {
    /// Special, helper method allowing to change a mode of [`Dfll`] operation
    /// in place. It is implemented only for an enabled [`Dfll`] having a single
    /// user (which is a [`Gclk0`]). Without it, it becomes unwieldy to change a
    /// mode of a [`Dfll`] actively used by [`Gclk0`], which is a very common
    /// scenario
    #[inline]
    pub fn to_open_mode(
        self,
        gclk0: EnabledGclk0<DfllId, U1>,
    ) -> (
        EnabledDfll<OpenLoop, U1>,
        EnabledGclk0<DfllId, U1>,
        Pclk<DfllId, G>,
    ) {
        let (token, pclk) = self.0.free();
        let dfll = Dfll::in_open_mode(token);
        (dfll.enable().inc(), gclk0, pclk)
    }
}

//==============================================================================
// Source
//==============================================================================

impl<M: Mode, N: Counter> Source for EnabledDfll<M, N> {
    type Id = DfllId;

    #[inline]
    fn freq(&self) -> Hertz {
        self.0.freq()
    }
}
