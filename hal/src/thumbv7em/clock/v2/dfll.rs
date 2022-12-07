//! # Digital Frequency Locked Loop
//!
//! The `dfll` module provides access to the 48 MHz digital frequency locked
//! loop (DFLL) within the `OSCCTRL` peripheral.
//!
//! ## Operation modes
//!
//! The DFLL can operate in both open-loop and closed-loop modes. In open-loop
//! mode, it uses an internal oscillator to produce an unreferenced, 48 MHz
//! output clock. While in closed-loop mode, the DFLL multiplies a low-frequency
//! input clock to yield a 48 MHz output clock. The reference clock can be
//! provided by a GCLK, through the DFLL peripheral channel clock, or it can be
//! provided by the USB start-of-frame signal.
//!
//! The DFLL is represented by the [`Dfll`] type. When the DFLL is in
//! closed-loop mode, it looks like many of the other clocks in the `clock`
//! module; it takes an input clock and produces an output clock. And like those
//! other clocks, [`Dfll<I>`] takes a type parameter to represent the
//! [`Id` type](super#id-types) of its clock source. However, when the DFLL is
//! in open-loop mode, it instead looks more like the [`OscUlp32k`] clock, which
//! doesn't require a type parameter to track its configuration or source.
//!
//! To handle both of these configurations simultaneously, we leverage the
//! [`OptionalKind`] pattern to express the notion of an optional type
//! parameter. When the DFLL is in open-loop mode, we can set the [`Dfll<I>`]
//! type parameter to [`NoneT`]. In fact, this is the default type for `I`,
//! so an unqualified [`Dfll`] is in open-loop mode. Otherwise, when the DFLL is
//! in closed-loop mode, `I` represents one of the [`DfllSourceId`] types. See
//! the documentation of [`OptionalDfllSourceId`] for more details.
//!
//! ## The DFLL at power-on reset
//!
//! Because the DFLL can produce a 48 MHz clock from an internal oscillator, it
//! is used as the default master clock for the system at power-on reset. While
//! most clocks are disabled at reset and represented by items in the [`Tokens`]
//! struct, the [`Dfll`] is [`Enabled`] at reset, so it is found in the
//! [`Clocks`] struct.
//!
//! At reset, the [`EnabledDfll`] is in open-loop mode and has one consumer
//! clock, so its complete type is `EnabledDfll<U1>`. The corresponding consumer
//! is [`Gclk0`], which is represented as `EnabledGclk0<DfllId, U1>`. The
//! [`EnabledGclk0`] has its own consumer as well, which is the system master
//! clock.
//!
//! ## Example
//!
//! Configuring the [`Dfll`] proceeds according to the principles outlined in
//! the [`clock` module documentation]. Suppose we start with the default clock
//! tree after power-on reset.
//!
//! ```text
//! DFLL (48 MHz; open-loop mode)
//! └── GCLK0 (48 MHz)
//!     └── Master clock (48 MHz)
//! ```
//!
//! We would like to transform it to a clock tree like this:
//!
//! ```text
//! XOSC0 (24 MHz; external clock)
//! └── GCLK0 (24 MHz)
//!     ├── Master clock (24 MHz)
//!     └── DFLL (48 MHz; closed-loop mode)
//! ```
//!
//! Let's start by using [`clock_system_at_reset`] to access the HAL clocking
//! structs. We'll also need access to the GPIO [`Pins`].
//!
//! ```no_run
//! use atsamd_hal::{
//!     clock::v2::{clock_system_at_reset, dfll::Dfll, xosc::Xosc},
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
//! Next, we create a 24 MHz [`Xosc`] clock from one of the [`Pins`] and enable
//! it.
//!
//! ```no_run
//! # use atsamd_hal::{
//! #     clock::v2::{clock_system_at_reset, dfll::Dfll, xosc::Xosc},
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
//! let xosc0 = Xosc::from_clock(tokens.xosc0, pins.pa14, 24.mhz()).enable();
//! ```
//!
//! We can then swap [`Gclk0`] from the [`EnabledDfll`] to the [`EnabledXosc`].
//! This releases the [`EnabledDfll`] and [`Decrement`]s its consumer count,
//! which allows us to disable it and retrieve the underlying [`DfllToken`].
//!
//! ```no_run
//! # use atsamd_hal::{
//! #     clock::v2::{clock_system_at_reset, dfll::Dfll, xosc::Xosc},
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
//! # let xosc0 = Xosc::from_clock(tokens.xosc0, pins.pa14, 24.mhz()).enable();
//! let (gclk0, dfll, _xosc0) = clocks.gclk0.swap_sources(clocks.dfll, xosc0);
//! let token_dfll = dfll.disable().free();
//! ```
//!
//! Next, we can enable the peripheral channel clock, or [`Pclk`], for the
//! [`Dfll`], sourcing it from [`Gclk0`]. With the `Pclk`, we can then recreate
//! the `Dfll` in closed-loop mode. Finally, we can adjust some of the
//! closed-loop parameters before we enable it. The returned [`EnabledDfll`] can
//! be used as a clock [`Source`] elsewhere in the clock tree.
//!
//! ```no_run
//! # use atsamd_hal::{
//! #     clock::v2::{clock_system_at_reset, dfll::Dfll, pclk::Pclk, xosc::Xosc},
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
//! # let xosc0 = Xosc::from_clock(tokens.xosc0, pins.pa14, 24.mhz()).enable();
//! # let (gclk0, dfll, _xosc0) = clocks.gclk0.swap_sources(clocks.dfll, xosc0);
//! # let token_dfll = dfll.disable().free();
//! let (pclk_dfll, _gclk0) = Pclk::enable(tokens.pclks.dfll, gclk0);
//! let dfll = Dfll::from_pclk(token_dfll, pclk_dfll)
//!     .coarse_max_step(1)
//!     .fine_max_step(10)
//!     .quick_lock(false)
//!     .enable();
//! ```
//!
//! The entire example is provided below.
//!
//! ```no_run
//! use atsamd_hal::{
//!     clock::v2::{clock_system_at_reset, dfll::Dfll, pclk::Pclk, xosc::Xosc},
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
//! let xosc0 = Xosc::from_clock(tokens.xosc0, pins.pa14, 24.mhz()).enable();
//! let (gclk0, dfll, _xosc0) = clocks.gclk0.swap_sources(clocks.dfll, xosc0);
//! let token_dfll = dfll.disable().free();
//! let (pclk_dfll, _gclk0) = Pclk::enable(tokens.pclks.dfll, gclk0);
//! let dfll = Dfll::from_pclk(token_dfll, pclk_dfll)
//!     .coarse_max_step(1)
//!     .fine_max_step(10)
//!     .quick_lock(false)
//!     .enable();
//! ```
//!
//! # Reconfiguring an [`EnabledDfll`]
//!
//! In some cases, users may want to reconfigure the DFLL while it remains
//! enabled. For instance, a user may want to place the DFLL in its closed-loop,
//! USB recovery mode while in use by the system's master clock. It would
//! normally be impossible to do so with other clocks in the `clock` module,
//! because changing the clock source would break an invariant of the clock
//! tree. However, the DFLL is special, because its output frequency is always
//! 48 MHz. Moreover, by design, consumers of the DFLL aren't affected by its
//! configuration (see the discussion on [`Id` types]).
//!
//! For this reason, we define a special [`change_source`] function on
//! [`EnabledDfll`]. It will consume the `EnabledDfll` and transform it to use
//! a different [`OptionalDfllSourceId`]. Note that [`change_source`] takes and
//! returns a [`Resource`] type for the corresponding `Id` type. The `Resource`
//! type for [`GclkId`]s is a [`Pclk`], because it needs to be stored by the
//! [`Dfll`]. The `Resource` type for both [`NoneT`] and [`UsbSofId`] is `()`,
//! because they don't need to store anything.
//!
//! Consider the following example. As above, we start with the clocks in their
//! default configuration at power-on reset. Remember that the [`Dfll`] is used
//! by the system master clock. At this point, we would like to reconfigure it
//! to use USB recovery mode. We call the `change_source` function, which takes
//! the [`Resource`] type for [`UsbSofId`], which is `()` and a closure to
//! modify the [`Dfll`] before the new settings are applied. The `change_source`
//! function is responsible for applying this closure without disabling the
//! DFLL. The return value is a tuple containing the reconfigured DFLL and the
//! old [`Resource`] type, which is again `()` for [`NoneT`].
//!
//! ```no_run
//! use atsamd_hal::{
//!     clock::v2::{
//!         clock_system_at_reset,
//!         dfll::{Dfll, UsbSofId},
//!     },
//!     pac::Peripherals,
//! };
//! let mut pac = Peripherals::take().unwrap();
//! let (buses, mut clocks, tokens) = clock_system_at_reset(
//!     pac.OSCCTRL,
//!     pac.OSC32KCTRL,
//!     pac.GCLK,
//!     pac.MCLK,
//!     &mut pac.NVMCTRL,
//! );
//! let (dfll, _) = clocks.dfll.change_source::<UsbSofId, _>((), |dfll| {
//!     dfll.set_coarse_max_step(1);
//!     dfll.set_fine_max_step(8);
//!     dfll.set_run_standby(true);
//! });
//! ```
//!
//! Note that the user may also wish to return some other object from the
//! closure. To do so, the expected return type for the closure is actually
//! `(Dfll<J>, R)`, where `R` is an arbitrary return type set by the user. In
//! the example above, we don't make use of this feature, so `R = ()`. However,
//! if the DFLL had been in closed-loop mode and sourced from a [`Pclk`], `R`
//! could have been used to return the [`Pclk`] to the user.
//!
//! [`clock_system_at_reset`]: super::clock_system_at_reset
//! [`clock` module documentation]: super
//! [`Id` types]: super#id-types
//! [`Clocks`]: super::Clocks
//! [`Tokens`]: super::Tokens
//! [`Pins`]: crate::gpio::Pins
//! [`Xosc`]: super::xosc::Xosc
//! [`EnabledXosc`]: super::xosc::EnabledXosc
//! [`Gclk0`]: super::gclk::Gclk0
//! [`EnabledGclk0`]: super::gclk::EnabledGclk0
//! [`Decrement`]: crate::typelevel::Decrement
//! [`OscUlp32k`]: super::osculp32k::OscUlp32k
//! [`OptionalKind`]: crate::typelevel#optionalkind-trait-pattern
//! [`change_source`]: EnabledDfll::change_source

use crate::time::Hertz;
use crate::typelevel::{NoneT, Sealed};
use typenum::U0;

use super::gclk::GclkId;
use super::pclk::Pclk;
use super::{Enabled, Source};

//==============================================================================
// DfllToken
//==============================================================================

/// Singleton token that can be exchanged for the [`Dfll`]
///
/// As explained in the [`clock` module documentation](super), instances of
/// various `Token` types represent disabled clocks and can be exchanged for
/// actual clock types. However, unlike most other clocks in the module, the
/// [`Dfll`] is [`Enabled`] at power-on reset. Thus, users will never deal with
/// the `DfllToken` unless they first disable the [`EnabledDfll`].
pub struct DfllToken(());

impl DfllToken {
    /// Create a new [`DfllToken`]
    ///
    /// # Safety
    ///
    /// The `DfllToken`s is a singleton. There must never be two simulatenous
    /// instances of it. See the notes on `Token` types and memory safety in the
    /// root of the `clock` module for more details.
    #[inline]
    pub(crate) unsafe fn new() -> Self {
        Self(())
    }

    #[inline]
    fn oscctrl(&self) -> &crate::pac::oscctrl::RegisterBlock {
        // Safety: The `DfllToken` only has access to a mutually exclusive set
        // of registers for the DFLL, and we use a shared reference to the
        // register block. See the notes on `Token` types and memory safety in
        // the root of the `clock` module for more details.
        unsafe { &*crate::pac::OSCCTRL::PTR }
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
            self.dfllmul().modify(|_, w|
            // Safety: All bit patterns are valid for these fields
            unsafe {
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
// DfllId
//==============================================================================

/// [`Id` type](super#id-types) representing the identity of the DFLL clock
pub enum DfllId {}

impl Sealed for DfllId {}

//==============================================================================
// UsbSofId
//==============================================================================

/// [`Id` type](super#id-types) representing the identity of the USB
/// start-of-frame clock
pub enum UsbSofId {}

//==============================================================================
// DynDfllSource
//==============================================================================

/// Value-level enum of possible clock sources for the [`Dfll`]
///
/// The variants of this enum identify one of two possible clock sources for
/// the [`Dfll`] when operating in closed-loop mode.
///
/// `DynDfllSourceId` is the value-level equivalent of [`DfllSourceId`].
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum DynDfllSourceId {
    /// The DFLL is driven by a [`Pclk`]
    Pclk,
    /// The DFLL is driven by the USB start-of-frame signal
    UsbSof,
}

//==============================================================================
// DfllSourceId
//==============================================================================

/// Type-level enum of possible reference clock sources for the [`Dfll`]
///
/// The types implementing this trait are type-level variants of `DfllSourceId`,
/// and they identify one of two possible reference clocks for the [`Dfll`]. The
/// implementers of this trait are `Id` types, which are described in more
/// detail in the [`clock` module documentation](super).
///
/// `DfllSourceId` is the type-level equivalent of [`DynDfllSourceId`]. See the
/// documentation on [type-level programming] and specifically
/// [type-level enums] for more details.
///
/// [type-level programming]: crate::typelevel
/// [type-level enums]: crate::typelevel#type-level-enums
pub trait DfllSourceId {
    /// Corresponding variant of [`DynDfllSourceId`]
    const DYN: DynDfllSourceId;

    /// [`settings`] type for the reference clock source
    #[doc(hidden)]
    type Settings: Settings;
}

impl<G: GclkId> DfllSourceId for G {
    const DYN: DynDfllSourceId = DynDfllSourceId::Pclk;
    type Settings = settings::Pclk<G>;
}

impl DfllSourceId for UsbSofId {
    const DYN: DynDfllSourceId = DynDfllSourceId::UsbSof;
    type Settings = settings::Usb;
}

//==============================================================================
// OptionalDfllSourceId
//==============================================================================

/// Type-level equivalent of `Option<DfllSourceId>`
///
/// The [`Dfll`] only has a reference clock source when it is in closed-loop
/// mode. When it is in open-loop mode, there is no reference clock. This trait
/// serves as a way to represent an optional [`DfllSourceId`] at the type level.
///
/// At the value level, this would be represented by the type
/// `Option<DynDfllSourceId>`. At the type level, we can use the
/// [`OptionalKind`] pattern to represent the same concept. We implement this
/// trait for both [`NoneT`], to represent open-loop mode, and all
/// [`DfllSourceId`] types for closed-loop mode.
///
/// [`OptionalKind`]: crate::typelevel#optionalkind-trait-pattern
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

/// Type-level equivalent of `Some(DfllSourceId)`
///
/// There is no practical difference between this trait and [`DfllSourceId`]. It
/// exists only to emphasize when an [`OptionalDfllSourceId`] type is
/// constrained to non-[`NoneT`] types. See documentation of the
/// [`OptionalKind`] pattern for more details.
///
/// [`OptionalKind`]: crate::typelevel#optionalkind-trait-pattern
pub trait SomeDfllSourceId: DfllSourceId {}

impl<I: DfllSourceId> SomeDfllSourceId for I {}

//==============================================================================
// Settings
//==============================================================================

mod settings {
    //! Store and retrieve [`Dfll`] settings in different modes
    //!
    //! Many of the [`Dfll`] settings are not valid or required in every
    //! operating mode. This module provides a framework to store only the
    //! minimum required settings for each mode in a generic way. Specifically,
    //! the [`Minimum`] struct stores the few settings relevant in all modes,
    //! along with a generic, mode-specific type. The [`Settings`] trait unifies
    //! all concrete instances of [`Minimum`] by providing a function to return
    //! a collection of [`All`] settings. Each sub-struct within [`Minimum`]
    //! implements [`Settings`] and is responsible for filling the relevent
    //! fields of [`All`].
    //!
    //! [`Dfll`]: super::Dfll

    use super::super::pclk;
    use super::{CoarseMaxStep, DfllId, FineMaxStep, GclkId, Hertz, MultFactor};

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
    /// This struct stores the [`Pclk`] and multiplication factor, which
    /// determine the precise [`Dfll`] frequency, as well as flags to control
    /// the chill-cycle and quick-lock features. Note that these flags indicate
    /// whether the feature is *enabled*, while the corresponding register bits
    /// indicate whether the feature is *disabled*.
    ///
    /// [`Dfll`]: super::Dfll
    /// [`Pclk`]: pclk::Pclk
    pub struct Pclk<G: GclkId> {
        pub pclk: pclk::Pclk<DfllId, G>,
        pub mult_factor: MultFactor,
        pub chill_cycle: bool,
        pub quick_lock: bool,
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
        /// Resource stored by the `Settings` struct
        type Resource;

        /// Construct the `Settings` struct from its resource
        ///
        /// Use default values for all other fields of the struct.
        fn from_resource(resource: Self::Resource) -> Self;

        /// Consume the `Settings` struct and return its resource
        fn into_resource(self) -> Self::Resource;

        /// Fill the respective fields of [`All`] and recursively defer any
        /// remaining fields to sub-structs or the [`Default`] settings
        fn all(&self) -> All;
    }

    impl<T: Settings> Settings for Minimum<T> {
        type Resource = T::Resource;
        #[inline]
        fn from_resource(resource: Self::Resource) -> Self {
            let mode = T::from_resource(resource);
            Self {
                mode,
                run_standby: false,
                on_demand: true,
            }
        }
        #[inline]
        fn into_resource(self) -> Self::Resource {
            self.mode.into_resource()
        }
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
        type Resource = ();
        #[inline]
        fn from_resource(_: ()) -> Self {
            OpenLoop
        }
        #[inline]
        fn into_resource(self) -> Self::Resource {}
        #[inline]
        fn all(&self) -> All {
            All::default()
        }
    }

    impl<T: Settings> Settings for ClosedLoop<T> {
        type Resource = T::Resource;
        #[inline]
        fn from_resource(resource: T::Resource) -> Self {
            let source = T::from_resource(resource);
            Self {
                source,
                coarse_max_step: 1,
                fine_max_step: 1,
            }
        }
        #[inline]
        fn into_resource(self) -> Self::Resource {
            self.source.into_resource()
        }
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
        type Resource = ();
        #[inline]
        fn from_resource(_: ()) -> Self {
            Usb
        }
        #[inline]
        fn into_resource(self) -> Self::Resource {}
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

    impl<G: GclkId> Settings for Pclk<G> {
        type Resource = pclk::Pclk<DfllId, G>;
        #[inline]
        fn all(&self) -> All {
            All {
                src_freq: self.pclk.freq(),
                mult_factor: self.mult_factor,
                chill_cycle: self.chill_cycle,
                quick_lock: self.quick_lock,
                ..All::default()
            }
        }
        #[inline]
        fn from_resource(pclk: Self::Resource) -> Self {
            // Cast is fine because division result cannot be greater than u16::MAX
            let mult_factor = (48_000_000 / pclk.freq().0) as u16;
            Self {
                pclk,
                mult_factor,
                chill_cycle: true,
                quick_lock: true,
            }
        }
        #[inline]
        fn into_resource(self) -> Self::Resource {
            self.pclk
        }
    }
}

use settings::Settings;

/// [`Dfll`] resource type for the corresponding [`OptionalDfllSourceId`]
///
/// This type alias maps from [`OptionalDfllSourceId`] types to a corresponding
/// resource type that the [`Dfll`] stores for its existence.
///
/// For [`NoneT`], when the DFLL is in open-loop mode, the resource type is
/// merely `()`, as there is no resource to store. At the moment, [`UsbSofId`]
/// also maps to `()`, but that may change in the future. [`GclkId`] types map
/// to the corresponding [`Pclk`] type for the [`Dfll`].
pub type Resource<I> = <<I as OptionalDfllSourceId>::Settings as Settings>::Resource;

//==============================================================================
// Dfll
//==============================================================================

/// Digital frequency-locked loop used to generate a 48 MHz clock
///
/// The DFLL generates a 48 MHz clock in two different possible modes. In
/// open-loop mode, it generates the output clock from an internal oscillator,
/// while in closed-loop mode, it multiplies a low-frequency reference clock.
///
/// The type parameter `I` represents an optional [`Id` type](super#id-types)
/// for the reference clock. When the DFLL is in open-loop mode, there is no
/// reference clock, so `I` is [`NoneT`]. This is the default value for `I`.
/// Alternatively, when the DFLL is in closed-loop mode, `I` is one of the
/// [`DfllSourceId`] types. The [`OptionalDfllSourceId`] trait unifies these two
/// possibilities and is an expression of the [`OptionalKind`] pattern.
///
/// On its own, the `Dfll` type does not represent the enabled DFLL. Instead, it
/// must first be wrapped with [`Enabled`], which implements compile-time safety
/// of the clock tree.
///
/// Because the terminal call to [`enable`] consumes the `Dfll` and returns an
/// [`EnabledDfll`], the remaining API uses the builder pattern, where each
/// method takes and returns `self` by value, allowing them to be easily
/// chained.
///
/// See the [module-level documentation](self) for an example of creating,
/// configuring and using the `Dfll`.
///
/// [`enable`]: Dfll::enable
/// [`OptionalKind`]: crate::typelevel#optionalkind-trait-pattern
pub struct Dfll<I: OptionalDfllSourceId = NoneT> {
    token: DfllToken,
    settings: settings::Minimum<I::Settings>,
}

impl<I: OptionalDfllSourceId> Dfll<I> {
    #[inline]
    fn new(token: DfllToken, resource: Resource<I>) -> Self {
        let settings = settings::Minimum::from_resource(resource);
        Self { token, settings }
    }
}

impl Dfll {
    /// Create the [`Dfll`] in open-loop mode
    ///
    /// Creating a [`Dfll`] does not modify any of the hardware registers. It
    /// only creates a struct to track the `Dfll` configuration.
    ///
    /// The configuration data is stored until the user calls [`enable`].
    /// At that point, all of the registers are written according to the
    /// initialization procedures specified in the datasheet, and an
    /// [`EnabledDfll`] is returned. The `Dfll` is not active or useful until
    /// that point.
    ///
    /// [`enable`]: Dfll::enable
    #[inline]
    pub fn open_loop(token: DfllToken) -> Self {
        Self::new(token, ())
    }

    /// Consume the [`Dfll`] and release the [`DfllToken`]
    #[inline]
    pub fn free(self) -> DfllToken {
        self.token
    }
}

impl Dfll<UsbSofId> {
    /// Create the [`Dfll`] in USB recovery mode
    ///
    /// This creates the `Dfll` in closed-loop mode referenced to the USB
    /// start-of-frame signal. For now, this function does not require any proof
    /// of a functioning USB interface. Future versions of this function may
    /// take ownership of some resource both to prove USB has been setup
    /// correctly and to prevent modification while in use.
    ///
    /// Creating a [`Dfll`] does not modify any of the hardware registers. It
    /// only creates a struct to track the `Dfll` configuration.
    ///
    /// The configuration data is stored until the user calls [`enable`].
    /// At that point, all of the registers are written according to the
    /// initialization procedures specified in the datasheet, and an
    /// [`EnabledDfll`] is returned. The `Dfll` is not active or useful until
    /// that point.
    ///
    /// [`enable`]: Dfll::enable
    #[inline]
    pub fn from_usb(token: DfllToken) -> Self {
        Self::new(token, ())
    }

    /// Consume the [`Dfll`] and release the [`DfllToken`]
    #[inline]
    pub fn free(self) -> DfllToken {
        self.token
    }
}

impl<G: GclkId> Dfll<G> {
    /// Create the [`Dfll`] in closed-loop mode
    ///
    /// This creates the `Dfll` in closed-loop mode referenced to a [`Gclk`]
    /// through a [`Pclk`]. It will also auto-calculate the correct
    /// multiplication factor to best yield 48 MHz at the output.
    ///
    /// Creating a [`Dfll`] does not modify any of the hardware registers. It
    /// only creates a struct to track the `Dfll` configuration.
    ///
    /// The configuration data is stored until the user calls [`enable`].
    /// At that point, all of the registers are written according to the
    /// initialization procedures specified in the datasheet, and an
    /// [`EnabledDfll`] is returned. The `Dfll` is not active or useful until
    /// that point.
    ///
    /// # Panics
    ///
    /// According to the datasheet, the [`Pclk`] frequency must be between
    /// 732 Hz and 33 kHz. This function will perform a run-time check of the
    /// input frequency and panic if it is out of range. To use a `Pclk`
    /// frequency outside this range or to force a particular multiplication
    /// factor, use [`Dfll::from_pclk_unchecked`].
    ///
    /// [`Gclk`]: super::gclk::Gclk
    /// [`enable`]: Dfll::enable
    #[inline]
    pub fn from_pclk(token: DfllToken, pclk: Pclk<DfllId, G>) -> Self {
        const MIN: u32 = 48_000_000 / MultFactor::MAX as u32;
        const MAX: u32 = 33_000;
        let freq = pclk.freq().0;
        if freq < MIN || freq > MAX {
            panic!("Invalid Pclk<DfllId, _> input frequency");
        }
        Self::new(token, pclk)
    }

    /// Create the [`Dfll`] in closed-loop mode
    ///
    /// This constructor behaves identically to [`Dfll::from_pclk`], but it
    /// skips the run-time check of the [`Pclk`] frequency and does not
    /// auto-calculate the multiplication factor.
    #[inline]
    pub fn from_pclk_unchecked(
        token: DfllToken,
        pclk: Pclk<DfllId, G>,
        mult_factor: MultFactor,
    ) -> Self {
        let mut dfll = Self::new(token, pclk);
        dfll.settings.mode.source.mult_factor = mult_factor;
        dfll
    }

    /// Consume the [`Dfll`], release the [`DfllToken`], and return the [`Pclk`]
    #[inline]
    pub fn free(self) -> (DfllToken, Pclk<DfllId, G>) {
        (self.token, self.settings.mode.source.pclk)
    }

    /// Enable or disable the [`Dfll`] chill cycle
    ///
    /// See the documentation of [`chill_cycle`] for more details.
    ///
    /// [`chill_cycle`]: Dfll::chill_cycle
    #[inline]
    pub fn set_chill_cycle(&mut self, value: bool) {
        self.settings.mode.source.chill_cycle = value;
    }

    /// Enable or disable the [`Dfll`] chill cycle
    ///
    /// When operating in closed-loop mode with small multiplication factors,
    /// the DFLL can sometimes have trouble locking. To avoid this, the hardware
    /// normally implements a chill cycle, during which the output frequency is
    /// not measured. The chill cycle is enabled by default, but it can be
    /// disabled to reduce the duration before lock. See the datasheet for more
    /// details.
    #[inline]
    pub fn chill_cycle(mut self, value: bool) -> Self {
        self.set_chill_cycle(value);
        self
    }

    /// Enable or disable the [`Dfll`] quick lock
    ///
    /// See the documentation of [`quick_lock`] for more details.
    ///
    /// [`quick_lock`]: Dfll::quick_lock
    #[inline]
    pub fn set_quick_lock(&mut self, value: bool) {
        self.settings.mode.source.quick_lock = value;
    }

    /// Enable or disable the [`Dfll`] quick lock
    ///
    /// By default, the DFLL locking requirements are somewhat loose. Users can
    /// tighten these requirements by disabling the quick lock feature, which is
    /// enabled by default. See the datasheet for more details.
    #[inline]
    pub fn quick_lock(mut self, value: bool) -> Self {
        self.set_quick_lock(value);
        self
    }
}

impl<I: SomeDfllSourceId> Dfll<I> {
    /// Set the maximum coarse step size during closed-loop frequency tuning
    ///
    /// See the documentation of [`coarse_max_step`] for more details.
    ///
    /// [`coarse_max_step`]: Dfll::coarse_max_step
    #[inline]
    pub fn set_coarse_max_step(&mut self, coarse_max_step: CoarseMaxStep) {
        self.settings.mode.coarse_max_step = coarse_max_step;
    }

    /// Set the maximum coarse step size during closed-loop frequency tuning
    ///
    /// In closed-loop operation, the DFLL output frequency is continuously
    /// regulated against the reference clock by adjusting the coarse and fine
    /// tuning parameters. This function sets a maximum step size for the coarse
    /// tuning parameter.
    ///
    /// In general, a small step size will ensure low overshoot in the output
    /// frequency, but it will lengthen the time to lock. A larger step size
    /// will produce more overshoot but will be quicker to lock. See the
    /// datasheet for more details.
    #[inline]
    pub fn coarse_max_step(mut self, coarse_max_step: CoarseMaxStep) -> Self {
        self.set_coarse_max_step(coarse_max_step);
        self
    }

    /// Set the maximum fine step size during closed-loop frequency tuning
    ///
    /// See the documentation of [`fine_max_step`] for more details.
    ///
    /// [`fine_max_step`]: Dfll::fine_max_step
    #[inline]
    pub fn set_fine_max_step(&mut self, fine_max_step: FineMaxStep) {
        self.settings.mode.fine_max_step = fine_max_step;
    }

    /// Set the maximum fine step size during closed-loop frequency tuning
    ///
    /// In closed-loop operation, the DFLL output frequency is continuously
    /// regulated against the reference clock by adjusting the coarse and fine
    /// tuning parameters. This function sets a maximum step size for the fine
    /// tuning parameter.
    ///
    /// In general, a small step size will ensure low overshoot in the output
    /// frequency, but it will lengthen the time to lock. A larger step size
    /// will produce more overshoot but will be quicker to lock. See the
    /// datasheet for more details.
    #[inline]
    pub fn fine_max_step(mut self, fine_max_step: FineMaxStep) -> Self {
        self.set_fine_max_step(fine_max_step);
        self
    }
}

impl<I: OptionalDfllSourceId> Dfll<I> {
    /// Return the [`Dfll`] output frequency
    ///
    /// The output frequency will always be close to, if not exactly, 48 MHz.
    #[inline]
    pub fn freq(&self) -> Hertz {
        // Valid for all modes based on default values
        let settings = self.settings.all();
        Hertz(settings.src_freq.0 * settings.mult_factor as u32)
    }

    /// Control the [`Dfll`] behavior during idle or standby sleep modes
    ///
    /// See the documentation of [`run_standby`] for more details.
    ///
    /// [`run_standby`]: Dfll::run_standby
    #[inline]
    pub fn set_run_standby(&mut self, value: bool) {
        self.settings.run_standby = value;
    }

    /// Control the [`Dfll`] behavior during idle or standby sleep modes
    ///
    /// When `true`, the `Dfll` will run in standby sleep mode, but its behavior
    /// can still be modified by the on-demand setting. See the datasheet for
    /// more details.
    #[inline]
    pub fn run_standby(mut self, value: bool) -> Self {
        self.set_run_standby(value);
        self
    }

    /// Control the [`Dfll`] on-demand functionality
    ///
    /// See the documentation of [`on_demand`] for more details.
    ///
    /// [`on_demand`]: Dfll::on_demand
    #[inline]
    pub fn set_on_demand(&mut self, value: bool) {
        self.settings.on_demand = value;
    }

    /// Control the [`Dfll`] on-demand functionality
    ///
    /// When `true`, only run the clock when requested by peripheral. If `false`
    /// the clock will be always active. This setting will also modify the
    /// behavior in standby sleep modes. See the datasheet for more details.
    #[inline]
    pub fn on_demand(mut self, value: bool) -> Self {
        self.set_on_demand(value);
        self
    }

    /// Enable the [`Dfll`], so that it can be used as a clock [`Source`]
    ///
    /// As mentioned when creating a new `Dfll`, no hardware registers are
    /// actually modified until this call. Rather, the desired configuration is
    /// stored internally, and the `Dfll` is initialized and configured here
    /// according to the datasheet.
    ///
    /// The returned value is an [`EnabledDfll`] that can be used as a clock
    /// [`Source`] for other clocks.
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

impl<I, N> EnabledDfll<I, N>
where
    I: OptionalDfllSourceId,
    N: Default,
{
    /// Change the [`OptionalDfllSourceId`] of the [`Dfll`] while it remains enabled
    ///
    /// Take ownership of an [`EnabledDfll`] and convert it to use a new
    /// [`OptionalDfllSourceId`]. This both requires the new [`Resource`] type
    /// and returns the old `Resource`. Users can also supply a closure to alter
    /// the [`Dfll`] settings before they are applied. The closure takes
    /// `&mut Dfll<J>` as its input, so it can only modify those settings with a
    /// `set_` method.
    ///
    /// See the [`dfll` module documentation] for more details on why and how
    /// this function would be used.
    ///
    /// [`dfll` module documentation]: super::dfll#reconfiguring-an-enableddfll
    pub fn change_source<J, F>(
        self,
        resource: Resource<J>,
        f: F,
    ) -> (EnabledDfll<J, N>, Resource<I>)
    where
        J: OptionalDfllSourceId,
        F: FnOnce(&mut Dfll<J>),
    {
        let old = self.0.settings.into_resource();
        let mut dfll = Dfll::new(self.0.token, resource);
        f(&mut dfll);
        let dfll = dfll.enable().0;
        (Enabled::new(dfll), old)
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
