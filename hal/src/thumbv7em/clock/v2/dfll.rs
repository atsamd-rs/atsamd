//#![warn(missing_docs)]
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

use crate::time::Hertz;
use crate::typelevel::{NoneT, PrivateIncrement, Sealed};

use super::gclk::{EnabledGclk0, GclkId};
use super::pclk::{Pclk, PclkToken};
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
        // SAFETY: The `Token` types in the `clock` module only ever use shared
        // references when dealing with MMIO registers wrapped in `UnsafeCell`s,
        // so it is safe to conjure a new one.
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

    #[inline]
    fn wait_sync_dfllctrlb(&self) {
        while self.dfllsync().read().dfllctrlb().bit() {}
    }

    #[inline]
    fn configure(&mut self, settings: settings::All) {
        self.dfllctrlb().modify(|_, w| {
            w.mode().bit(settings.closed_loop);
            w.usbcrm().bit(settings.usb_recovery);
            w.ccdis().bit(!settings.chill_cycle);
            w.qldis().bit(!settings.quick_lock)
        });
        self.wait_sync_dfllctrlb();
        if settings.closed_loop {
            self.dfllmul().modify(|_, w| unsafe {
                w.mul().bits(settings.mult_factor);
                w.cstep().bits(settings.coarse_max_step);
                w.fstep().bits(settings.fine_max_step)
            });
            self.wait_sync_dfllmul();
        }
        self.dfllctrla().modify(|_, w| {
            w.runstdby().bit(settings.run_standby);
            w.ondemand().bit(settings.on_demand)
        });
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
}

//==============================================================================
// Aliases
//==============================================================================

type MultFactor = u16;
type CoarseMaxStep = u8;
type FineMaxStep = u8;

//==============================================================================
// UsbSofId
//==============================================================================

pub enum UsbSofId {}

//==============================================================================
// DynDfllSource
//==============================================================================

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum DynDfllSourceId {
    Pclk,
    UsbSof,
}

//==============================================================================
// DfllSourceId
//==============================================================================

pub trait DfllSourceId {
    /// Corresponding variant of [`DynDfllSourceId`]
    const DYN: DynDfllSourceId;

    /// [`settings`] type for the reference clock source
    #[doc(hidden)]
    type Settings: Settings;
}

impl<G: GclkId> DfllSourceId for G {
    const DYN: DynDfllSourceId = DynDfllSourceId::Pclk;
    type Settings = settings::Pclk;
}

impl DfllSourceId for UsbSofId {
    const DYN: DynDfllSourceId = DynDfllSourceId::UsbSof;
    type Settings = settings::Usb;
}

//==============================================================================
// OptionalDfllSourceId
//==============================================================================

pub trait OptionalDfllSourceId {
    /// Optional variant of [`DynDfllSourceId`]
    ///
    /// When there is [`Some`] [`DynDfllSourceId`], it specifies the [`Dfll`]'s
    /// reference clock source in closed-loop mode. Otherwise, when there is no
    /// reference clock, the [`Dfll`] is in open-loop mode.
    const DYN: Option<DynDfllSourceId>;

    /// [`settings`] type for the operating mode
    #[doc(hidden)]
    type Settings: Settings;
}

impl OptionalDfllSourceId for NoneT {
    const DYN: Option<DynDfllSourceId> = None;
    type Settings = settings::OpenLoop;
}

impl<I: SomeDfllSourceId> OptionalDfllSourceId for I {
    const DYN: Option<DynDfllSourceId> = Some(I::DYN);
    type Settings = settings::ClosedLoop<I::Settings>;
}

//==============================================================================
// SomeDfllSourceId
//==============================================================================

pub trait SomeDfllSourceId: DfllSourceId {}

impl<I: DfllSourceId> SomeDfllSourceId for I {}

//==============================================================================
// Settings
//==============================================================================

/// Store and retrieve [`Dfll`] settings in different modes
///
/// Many of the [`Dfll`] settings are not valid or required in every operating
/// mode. This module provides a framework to store only the minimum required
/// settings for each mode in a generic way. Specifically, the [`Minimum`]
/// struct stores the few settings relevant in all modes, along with a generic,
/// mode-specific type. The [`Settings`] trait unifies all concrete instances
/// of [`Minimum`] by providing a function to return a collection of [`All`]
/// settings. Each sub-struct within [`Minimum`] implements [`Settings`] and is
/// responsible for filling the relevent fields of [`All`].
///
/// [`Dfll`]: super::Dfll
mod settings {
    use super::{CoarseMaxStep, FineMaxStep, Hertz, MultFactor};

    /// Collection of all possible [`Dfll`] settings
    ///
    /// This struct is returned by the [`Settings`] trait.
    ///
    /// [`Dfll`]: super::Dfll
    pub struct All {
        pub src_freq: Hertz,
        pub closed_loop: bool,
        pub usb_recovery: bool,
        pub mult_factor: MultFactor,
        pub chill_cycle: bool,
        pub quick_lock: bool,
        pub coarse_max_step: CoarseMaxStep,
        pub fine_max_step: FineMaxStep,
        pub run_standby: bool,
        pub on_demand: bool,
    }

    impl Default for All {
        #[inline]
        fn default() -> Self {
            All {
                src_freq: Hertz(48_000_000),
                closed_loop: false,
                usb_recovery: false,
                mult_factor: 1,
                chill_cycle: true,
                quick_lock: true,
                coarse_max_step: 1,
                fine_max_step: 1,
                run_standby: false,
                on_demand: true,
            }
        }
    }

    /// Collection of [`Dfll`] settings containing only the minimum required
    /// for the specific mode
    ///
    /// Many [`Dfll`] settings are not valid or required in every operating
    /// mode. This struct provides a framework to store and retrieve only the
    /// minimum settings for each mode in a generic way.
    ///
    /// Specifically, it stores flags for the `RUNSTDBY` and `ONDEMAND` fields,
    /// which are relevant in every mode, and it stores a mode-specific type,
    /// `T`. This can be either [`OpenLoop`] or [`ClosedLoop`], which both
    /// implement the [`Settings`] trait.
    ///
    /// [`Dfll`]: super::Dfll
    pub struct Minimum<T: Settings> {
        pub mode: T,
        pub run_standby: bool,
        pub on_demand: bool,
    }

    impl<T: Settings> Minimum<T> {
        /// Create a new instance of [`Minimum`] with default settings
        #[inline]
        pub fn new(mode: T) -> Self {
            Self {
                mode,
                run_standby: false,
                on_demand: true,
            }
        }
    }

    /// Collection of settings specific to open-loop [`Dfll`] operation
    ///
    /// Right now, this struct is empty, as none of the settings are relevant to
    /// open-loop operation.
    ///
    /// [`Dfll`]: super::Dfll
    pub struct OpenLoop;

    /// Collection of settings specific to closed-loop [`Dfll`] operation
    ///
    /// This struct stores the maximum step size for the coarse and fine
    /// adjustments in closed-loop mode. It also stores an additional type, `T`,
    /// containing settings specific to the reference clock source, which can be
    /// either [`Pclk`] or [`Usb`]. Both implement the [`Settings`] trait.
    ///
    /// [`Dfll`]: super::Dfll
    pub struct ClosedLoop<T: Settings> {
        pub source: T,
        pub coarse_max_step: CoarseMaxStep,
        pub fine_max_step: FineMaxStep,
    }

    impl<T: Settings> ClosedLoop<T> {
        /// Create a new instance of [`ClosedLoop`] with default settings
        #[inline]
        pub fn new(source: T) -> Self {
            Self {
                source,
                coarse_max_step: 1,
                fine_max_step: 1,
            }
        }
    }

    /// Collection of settings specific to [`Dfll`] USB recovery mode
    ///
    /// Right now, this struct is empty, but its implementation of [`Settings`]
    /// fills several fields of [`All`] with known, constant values for USB
    /// recovery mode.
    ///
    /// [`Dfll`]: super::Dfll
    pub struct Usb;

    /// Collection of [`Dfll`] settings when used in closed-loop mode with a
    /// [`Pclk`] reference
    ///
    /// This struct stores the [`Pclk`] frequency and multiplication factor,
    /// which determine the precise [`Dfll`] frequency, as well as flags to
    /// control the chill-cycle and quick-lock features. Note that these flags
    /// indicates whether the feature is *enabled*, while the corresponding
    /// register bits indicate whether the feature is *disabled*.
    ///
    /// [`Dfll`]: super::Dfll
    /// [`Pclk`]: super::Pclk
    pub struct Pclk {
        pub pclk_freq: Hertz,
        pub mult_factor: MultFactor,
        pub chill_cycle: bool,
        pub quick_lock: bool,
    }

    impl Pclk {
        /// Create a new instance of [`Pclk`] with default settings
        #[inline]
        pub fn new(pclk_freq: Hertz, mult_factor: MultFactor) -> Self {
            Self {
                pclk_freq,
                mult_factor,
                chill_cycle: true,
                quick_lock: true,
            }
        }
    }

    /// Generic interface to convert the [`Minimum`] settings into a collection
    /// of [`All`] settings
    ///
    /// Because many of the [`Dfll`] settings are not valid or relevant in every
    /// operating mode, we only want to store the [`Minimum`] required settings
    /// for each. To do so, we must have a generic interface to retrieve
    /// settings in every mode.
    ///
    /// This trait provides a recursive interface to yield a collection of
    /// [`All`] [`Dfll`] settings. Each implementer of [`Settings`] is required
    /// to fill its respective fields of [`All`] and recursively defer other
    /// fields to any sub-structs. At the bottom of the stack, structs can defer
    /// to the [`Default`] settings for [`All`].
    ///
    /// [`Dfll`]: super::Dfll
    pub trait Settings {
        /// Fill the respective fields of [`All`] and recursively defer any
        /// remaining fields to sub-structs or the [`Default`] settings
        fn all(&self) -> All;
    }

    impl<T: Settings> Settings for Minimum<T> {
        #[inline]
        fn all(&self) -> All {
            All {
                run_standby: self.run_standby,
                on_demand: self.on_demand,
                ..self.mode.all()
            }
        }
    }

    impl Settings for OpenLoop {
        #[inline]
        fn all(&self) -> All {
            All::default()
        }
    }

    impl<T: Settings> Settings for ClosedLoop<T> {
        #[inline]
        fn all(&self) -> All {
            All {
                closed_loop: true,
                coarse_max_step: self.coarse_max_step,
                fine_max_step: self.fine_max_step,
                ..self.source.all()
            }
        }
    }

    impl Settings for Usb {
        #[inline]
        fn all(&self) -> All {
            All {
                usb_recovery: true,
                src_freq: Hertz(1_000),
                mult_factor: 48_000,
                ..All::default()
            }
        }
    }

    impl Settings for Pclk {
        #[inline]
        fn all(&self) -> All {
            All {
                src_freq: self.pclk_freq,
                mult_factor: self.mult_factor,
                chill_cycle: self.chill_cycle,
                quick_lock: self.quick_lock,
                ..All::default()
            }
        }
    }
}

use settings::Settings;

//==============================================================================
// Dfll
//==============================================================================

/// Struct representing a [`Dfll`] abstraction
///
/// It is generic over the supported modes of operation
pub struct Dfll<I: OptionalDfllSourceId = NoneT> {
    token: DfllToken,
    settings: settings::Minimum<I::Settings>,
}

impl<I: OptionalDfllSourceId> Dfll<I> {
    #[inline]
    fn new(token: DfllToken, mode: I::Settings) -> Self {
        let settings = settings::Minimum::new(mode);
        Self { token, settings }
    }
}

impl Dfll {
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
    pub fn open_loop(token: DfllToken) -> Self {
        Self::new(token, settings::OpenLoop)
    }

    /// Release the resources
    #[inline]
    pub fn free(self) -> DfllToken {
        self.token
    }
}

impl Dfll<UsbSofId> {
    #[inline]
    pub fn from_usb(token: DfllToken) -> Self {
        let settings = settings::ClosedLoop::new(settings::Usb);
        Self::new(token, settings)
    }

    #[inline]
    pub fn free(self) -> DfllToken {
        self.token
    }
}

impl<G: GclkId> Dfll<G> {
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
    #[inline]
    pub fn from_pclk(token: DfllToken, pclk: Pclk<DfllId, G>) -> Self {
        const MIN: u32 = 48_000_000 / MultFactor::MAX as u32;
        const MAX: u32 = 33_000;
        let freq = pclk.freq().0;
        if freq < MIN || freq > MAX {
            panic!("Invalid Pclk<DfllId, _> input frequency");
        }
        // Cast is fine because division result cannot be greater than u16::MAX
        let mult_factor = (48_000_000 / freq) as u16;
        // SAFETY: The multiplication factor is guaranteed to be valid
        unsafe { Self::from_pclk_unchecked(token, pclk, mult_factor) }
    }

    /// Constructor for Pclk based closed loop submode.
    ///
    /// # Safety
    /// Correctness of input parameters is assumed:
    /// - `pclk` frequency in range `[732, 33_000`] Hz
    /// - `mult_factor` cannot yield out-of-spec output frequency for Dfll
    #[inline]
    pub unsafe fn from_pclk_unchecked(
        token: DfllToken,
        pclk: Pclk<DfllId, G>,
        mult_factor: MultFactor,
    ) -> Self {
        let source = settings::Pclk::new(pclk.freq(), mult_factor);
        let mode = settings::ClosedLoop::new(source);
        Self::new(token, mode)
    }

    #[inline]
    pub fn free(self) -> (DfllToken, Pclk<DfllId, G>) {
        let pclk = unsafe { Pclk::new(PclkToken::new(), self.settings.mode.source.pclk_freq) };
        (self.token, pclk)
    }

    /// Controls the chill cycle functionality. Default value is `true`
    #[inline]
    pub fn chill_cycle(mut self, value: bool) -> Self {
        self.settings.mode.source.chill_cycle = value;
        self
    }

    /// Controls quick lock functionality. Default value is `true`
    #[inline]
    pub fn quick_lock(mut self, value: bool) -> Self {
        self.settings.mode.source.quick_lock = value;
        self
    }
}

impl<I: SomeDfllSourceId> Dfll<I> {
    /// Set a maximum step size allowed during a process a frequency tuning for
    /// a coarse parameter.
    ///
    /// Consult datasheet regarding what kind of set of parameters is acceptable
    /// (c. 28.6.4.1, Closed-Loop Operation). Otherwise, [`Dfll`] behavior might
    /// be incorrect
    #[inline]
    pub fn coarse_max_step(mut self, coarse_max_step: CoarseMaxStep) -> Self {
        self.settings.mode.coarse_max_step = coarse_max_step;
        self
    }

    /// Set a maximum step size allowed during a process a frequency tuning for
    /// a fine parameter.
    ///
    /// Consult datasheet regarding what kind of set of parameters is acceptable
    /// (c. 28.6.4.1, Closed-Loop Operation). Otherwise, [`Dfll`] behavior might
    /// be incorrect
    #[inline]
    pub fn fine_max_step(mut self, fine_max_step: FineMaxStep) -> Self {
        self.settings.mode.fine_max_step = fine_max_step;
        self
    }
}

impl<I: OptionalDfllSourceId> Dfll<I> {
    #[inline]
    pub fn freq(&self) -> Hertz {
        // Valid for all modes based on default values
        let settings = self.settings.all();
        Hertz(settings.src_freq.0 * settings.mult_factor as u32)
    }

    /// Controls the clock source behaviour during standby
    ///
    /// See Datasheet c. 28.6.4.1
    #[inline]
    pub fn run_standby(mut self, value: bool) -> Self {
        self.settings.run_standby = value;
        self
    }

    /// Controls the on demand functionality of the clock source
    ///
    /// Only starts the clock source when a peripheral uses it. If cleared the
    /// clock will be always active
    ///
    /// See Datasheet c. 13.5 for general information; 28.6.4.1 for [`Dfll`]
    /// specific details
    #[inline]
    pub fn on_demand(mut self, value: bool) -> Self {
        self.settings.on_demand = value;
        self
    }

    /// Enables [`Dfll`]
    ///
    /// This method modifies hardware to match the configuration stored within
    #[inline]
    pub fn enable(mut self) -> EnabledDfll<I> {
        self.token.configure(self.settings.all());
        self.token.enable();
        Enabled::new(self)
    }
}

//==============================================================================
// EnabledDfll
//==============================================================================

/// An [`Enabled`] [`Dfll`]
///
/// As described in the [`clock` module documentation](super), the [`Enabled`]
/// wrapper implements compile-time clock tree safety by tracking the number of
/// consumer clocks and restricting access to the underlying [`Dfll`] to prevent
/// modification while in use.
///
/// As with [`Enabled`], the default value for `N` is `U0`; if left unspecified,
/// the counter is assumed to be zero.
pub type EnabledDfll<I = NoneT, N = U0> = Enabled<Dfll<I>, N>;

impl<I: OptionalDfllSourceId> EnabledDfll<I> {
    /// Disable the [`Dfll`]
    #[inline]
    pub fn disable(mut self) -> Dfll<I> {
        self.0.token.disable();
        self.0
    }
}

impl<Old: OptionalDfllSourceId> EnabledDfll<Old, U1> {
    #[inline]
    pub fn into_mode<New, F, R>(
        self,
        _gclk0: &mut EnabledGclk0<DfllId, U1>,
        f: F,
    ) -> (EnabledDfll<New, U1>, R)
    where
        New: OptionalDfllSourceId,
        F: FnOnce(Dfll<Old>) -> (Dfll<New>, R),
    {
        let (dfll, r) = f(self.0);
        (dfll.enable().inc(), r)
    }
}

//==============================================================================
// Source
//==============================================================================

impl<I: OptionalDfllSourceId, N> Source for EnabledDfll<I, N> {
    type Id = DfllId;

    #[inline]
    fn freq(&self) -> Hertz {
        self.0.freq()
    }
}
