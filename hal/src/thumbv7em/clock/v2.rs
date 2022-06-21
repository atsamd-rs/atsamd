//! # Version 2 of the `clock` module
//!
//! ## Overview
//!
//! This module provides a simple, ergonomic, and most of all **safe** API to
//! create and manage the clock tree in ATSAMD5x and E5x devices. It uses
//! [type-level programming techniques](crate::typelevel) to prevent users from
//! creating invalid or unsound clocking configurations.
//!
//! <p style="background:rgba(255,181,77,0.16);padding:0.75em;">
//! <strong> Note: </strong> Using a type-level API does place some limitations
//! on how the clocks can be configured. The types must be checked at
//! compile-time, which means the state of a given clock must also be known at
//! compile-time. This is exceedingly reasonable for most clocking
//! configurations, because most users set up their clocks once and never change
//! them again. However, if you need to dynamically change the clocking
//! configuration at run-time, you may find this API less ergonomic. A future,
//! fully dynamic API has been discussed, but nothing has been developed so far.
//! </p>
//!
//! To safely create and use a clock tree, it is critical that root and branch
//! clocks not be disabled while leaf clocks are still active. Stated
//! differently, if clock `B` is derived from, and dependent on, clock `A`, then
//! clock `A` **must not** be disabled while clock `B` is still in use.
//!
//! This module accomplishes clock-tree safety by tracking the number of
//! dependents for a given clock at compile-time. This is made possible using
//! type-level integers from the [`typenum`] crate.
//!
//! ## `Enabled` wrapper
//!
//! Specifically, once a clock has been enabled, it is wrapped in a dedicated
//! [`Enabled<T, N>`] struct, which provides two useful features. First, it
//! restricts access to the underlying clock type, `T`, so that it cannot be
//! placed in an invalid state. And second, it provides a compile-time integer,
//! `N`, to track the number of clocks that are derived from, and therefore
//! dependent on, this clock. If a given `Enabled` clock has `N > `[`U0`], then
//! it cannot be disabled.
//!
//! Management of the dependent clock count, `N`, is done using the
//! [`Increment`](crate::typelevel::Increment) and
//! [`Decrement`](crate::typelevel::Decrement) traits, which map the type-level
//! integers from [`typenum`] to their respective successors and predecessors.
//!
//! ## `Source` trait
//!
//! The [`Source`] trait also plays a critical role in this module. It has two
//! main purposes. First, it marks types that can act as a source to downstream
//! clocks in the tree. And second, it maps from each clock type to its
//! corresponding `Id` type.
//!
//! Each implementer of [`Source`] has a corresponding `Id` type which serves
//! to represent the *identity* of a particular clock, not the clock itself.
//! This distinction is important, because the actual clock type frequently has
//! additional configuration type parameters that are not important when
//! identifying a clock [`Source`].
//!
//! For example, the type `Enabled<Dfll<OpenLoop>, U0>` implements [`Source`]
//! with an associated `Id` type of [`DfllId`](dfll::DfllId). While the
//! [`Dfll<M>`](dfll::Dfll) type itself needs the type parameter `M` to track
//! its loop [`Mode`](dfll::Mode), downstream clocks don't need to know or care
//! which mode the DFLL is using. The `Id` types serve to erase such
//! configuration, representing only the clock's identity.
//!
//! ## Getting started
//!
//! To set up a clock tree, start by trading the [PAC](crate::pac)-level
//! clocking structs for their HAL equivalents. Right now, the only way to do so
//! safely is using the [`clock_system_at_reset`] function, which assumes all
//! clocks are in their default state at power-on reset. If this is not the
//! case, because, for example, a bootloader has modified the clocks, then you
//! may need to manually create the matching configuration using `unsafe` code.
//!
//! ```no_run
//! use atsamd_hal::clock::v2::clock_system_at_reset;
//! use atsamd_hal::pac::Peripherals;
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
//! At this point, you may notice that the function returned three different
//! objects, the [`Buses`], [`Clocks`] and [`Tokens`]. We will describe each of
//! these in turn.
//!
//! ### Buses
//!
//! The [`Buses`] struct contains the [`Ahb`](ahb::Ahb) and [`Apb`](apb::Apb)
//! objects, which represent the corresponding AHB and APB buses. Because clocks
//! on these buses are configured using sets of shared registers, the associated
//! [`AhbClk`](ahb::AhbClk) and [`ApbClk`](apb::ApbClk) types cannot be fully
//! standalone. To change their configuration, you must have exclusive, mutable
//! access to the respective bus type, in the form of `&mut Ahb` or `&mut Apb`.
//!
//! See the [`ahb`] and [`apb`] module documentation for more details.
//!
//! ### Clocks
//!
//! The [`Clocks`] struct contains all of the clocks that are enabled and
//! running at power-on reset, namely:
//! - All of the [`AhbClks`](ahb::AhbClks)
//! - Some of the [`ApbClks`](apb::ApbClks)
//! - The 48 MHz [`Dfll`](`dfll::Dfll`), running in [`OpenLoop`](dfll::OpenLoop)
//!   mode
//! - [`Gclk0`](gclk::Gclk0), which is based on the `Dfll` and feeds the
//!   processor's master clock
//! - The [`OscUlpBase`](osculp32k::OscUlpBase) clock, which is always running
//!   and cannot be disabled. It can feed the derived
//!   [`OscUlp1k`](osculp32k::OscUlp1k) and [`OscUlp32k`](osculp32k::OscUlp32k)
//!   clocks.
//!
//! Note that bus clocks are not wrapped in `Enabled`, because they can never be
//! used to derive further clocks. They are always leaf clocks, never root or
//! branch clocks. The other three clocks, on the other hand, are all wrapped in
//! [`Enabled`].
//!
//! The `Dfll` is configured as `Enabled<Dfll<OpenLoop>, U1>`, which represents
//! fact that `Gclk0` is dependent on it. `Gclk0` is configured as
//! `Enabled<Gclk0<DfllId>, U1>`, which indicates that it is derived from the
//! `Dfll` (note the use of `DfllId` as an `Id` type) and has one dependent
//! clock of its own. The dependent clock is, of course, the processor master
//! clock.
//!
//! `Gclk0` can be configured to use a different base clock, which would allow
//! you to reduce the `Dfll` dependent clock count from `U1` to `U0` and disable
//! it. However, the same is not true for `Gclk0`. Because the master clock has
//! no representation in this module, the `Enabled` `Gclk0` can *never* be
//! reduced to zero depdendent clocks, which neatly models that the master clock
//! can never be disabled.
//!
//! Finally, the `OscUlpBase` clock is modelled as `Enabled<OscUlpBase, U0>`,
//! because it has no dependent clocks at power-on reset. However, it still
//! cannot be disabled, because no such `.disable()` method is provided.
//!
//! ### Tokens
//!
//! All remaining clocks in the chip are disabled at power-on reset. These
//! clocks are represented with corresponding `Token` structs, which are
//! singleton objects that can be traded for actual clocks.
//!
//! ## Example clock tree
//!
//! Finally, we will walk through the creation of a simple clock tree to
//! illustrate some of the remaining concepts inherent to this module.
//!
//! Starting from the previous snippet, we have the [`Buses`], [`Clocks`] and
//! [`Tokens`] to work with, and our clock tree at power-on reset looks like
//! this.
//!
//! ```text
//! DFLL (48 MHz)
//! └── GCLK0 (48 MHz)
//!     └── Master clock (48 MHz)
//!
//! OSCULP base clock
//! ```
//!
//! Our goal will be a clock tree that looks like this:
//!
//! ```text
//! XOSC0 (8 MHz)
//! └── DPLL0 (100 MHz)
//!     └── GCLK0 (100 MHz)
//!         ├── Master clock (100 MHz)
//!         ├── SERCOM0 peripheral clock
//!         └── Ouput to GPIO pin
//!
//! OSCULP base clock
//! ```
//!
//! We will use an external crystal oscillator running at 8 MHz to feed a DPLL,
//! which will increase the clock frequency to 100 MHz. Then, we will
//! reconfigure GCLK0 to use the 100 MHz DPLL clock instead of the 48 MHz DFLL
//! clock.
//!
//! First, let's import some of the necessary types. We will see what each type
//! represents in turn.
//!
//! ```
//! use atsamd_hal::{
//!     clock::v2::{
//!         clock_system_at_reset,
//!         dpll::Dpll,
//!         gclkio::GclkOut,
//!         pclk::Pclk,
//!         xosc::{Xosc, CrystalCurrent},
//!     },
//!     gpio::Pins,
//!     pac::Peripherals,
//!     time::U32Ext,
//! };
//! ```
//!
//! To create an instance of [`Xosc`](xosc::Xosc), we will first need to
//! identify which of the two XOSC clocks we will use. Suppose an external
//! crystal is attached to pins `PA14` and `PA15`. These pins feed the XOSC0
//! clock, so we will want to create an instance of [`Xosc0`](xosc::Xosc0). Note
//! that `Xosc0<M>` is merely an alias for `Xosc<Xosc0Id, M>`. Here,
//! [`Xosc0Id`](xosc::Xosc0Id) represents the *identity* of the XOSC0 clock,
//! rather than the clock itself, and `M` represents the XOSC
//! [`Mode`](xosc::Mode).
//!
//! Next, we access the [`Tokens`] struct to extract the corresponding
//! [`XoscToken`](xosc::XoscToken) for XOSC0, and we trade the PAC `PORT` struct
//! for the [`gpio::Pins`](crate::gpio::Pins) struct to access the GPIO
//! pins. We can then call `Xosc::from_crystal` to trade the token and
//! [`Pin`](crate::gpio::Pin)s to yield an instance of
//! [`Xosc0`](xosc::Xosc0). In doing so, we also provide the external oscillator
//! frequency and the desired level of current to use with the crystal. In this
//! case, we will use a low [`CrystalCurrent`](xosc::CrystalCurrent).
//!
//! Finally, we can chain a call to the `Xosc::enable` method to enable the XOSC
//! and return an instance of [`EnabledXosc0<M, N>`](xosc::EnabledXosc0), which
//! is simply an alias for `Enabled<Xosc0<M>, N>`. In this case, we get
//! `EnabledXosc0<CrystalMode, U0>`.
//!
//! ```ignore
//! let pins = Pins::new(pac.PORT);
//!
//! let xosc0 = Xosc::from_crystal(
//!     tokens.xosc0,
//!     pins.pa14,
//!     pins.pa15,
//!     8.mhz(),
//!     CrystalCurrent::Low,
//! ).enable();
//! ```
//!
//! Next, we want to use a DPLL to multiply the 8 MHz crystal clock up to 100
//! MHz. Once again, we need to decide between two instances of a clock, because
//! each chip has two [`Dpll`](dpll::Dpll)s. This time, however, our decision
//! between [`Dpll0`](dpll::Dpll0) and [`Dpll1`](dpll::Dpll1) is arbitrary.
//!
//! Also note that, like before, `Dpll0<I>` and `Dpll1<I>` are aliases for
//! `Dpll<Dpll0Id, I>` and `Dpll<Dpll1Id, I>`. [`Dpll0Id`](dpll::Dpll0Id)
//! and [`Dpll1Id`](dpll::Dpll1Id) represent the *identity* of the respective
//! DPLL, while `I` represents the `Id` type for the [`Source`] driving the
//! DPLL. In this particular case, we aim to create an instance of
//! `Dpll0<Xosc0Id>`.
//!
//! Only certain clocks can drive the DPLL, so `I` is constrained by the
//! [`DpllSourceId`](dpll::DpllSourceId) trait. Specifically, only the
//! [`Xosc0Id`](xosc::Xosc0Id), [`Xosc1Id`](xosc::Xosc1Id),
//! [`Xosc32kId`](xosc32k::Xosc32kId) and [`GclkId`](gclk::GclkId) types
//! implement this trait.
//!
//! As before, we access the [`Tokens`] struct and use the corresponding
//! [`DpllToken`](dpll::DpllToken) when creating an instance of `Dpll`. However,
//! unlike before, we are creating a new clock-tree relationship that must be
//! tracked by the type system. Because DPLL0 will now depend on XOSC0, we
//! must increment the count of dependent clocks in the
//! [`EnabledXosc0`](xosc::EnabledXosc0) struct.
//!
//! Thus, to create an instance of `Dpll0<XoscId0>`, we must provide the
//! `EnabledXosc0`, so that its `U0` type parameter can be incremented to `U1`.
//! The `Dpll::from_xosc0` method takes ownership of the `EnabledXosc0` and
//! returns it with this modified type parameter.
//!
//! ```ignore
//! let (dpll0, xosc0) = Dpll::from_xosc0(tokens.dpll0, xosc0);
//! ```
//! Next, we set the DPLL predivider and loop parameters. We must first divide
//! the XOSC clock down from 8 MHz to 2 MHz, so that it is within the valid
//! input frequency range for the DPLL. Then, we multiple the 2 MHz clock by 50
//! for a 100 MHz output. We do not need fractional mutiplication here, so the
//! fractional multiplier is zero. Finally, we can enable the `Dpll`, yielding
//! an instance of `EnabledDpll0<XoscId0>`.
//!
//! ```ignore
//! let dpll0 = dpll0.set_prediv(4).set_loop_div(50, 0).enable();
//! ```
//!
//! So far, our clock tree looks like this
//!
//! ```text
//! DFLL (48 MHz)
//! └── GCLK0 (48 MHz)
//!     └── Master clock (48 MHz)
//!
//! XOSC0 (8 MHz)
//! └── DPLL0 (100 MHz)
//!
//! OSCULP base clock
//! ```
//!
//! Our next task will be to swap GCLK0 from the 48 MHz DFLL to the 100 MHz
//! DPLL. To do that, we will use the special `swap` method on
//! [`EnabledGclk0`](gclk::EnabledGclk0) to change the base clock without
//! disabling GCLK0 or the master clock.
//!
//! This time we will be performing two simultaneous modifications of dependent
//! clock counts. We will be decreasing the [`EnabledDfll`](dfll::EnabledDfll)
//! count from [`U1`](typenum::U1) to [`U0`], and we will be increasing the
//! [`EnabledDpll0`](dpll::EnabledDpll0) count from `U0` to `U1`. Again, we need
//! to provide both the DFLL and DPLL clocks, so that their type parameters can
//! be changed.
//!
//! ```ignore
//! let (gclk0, dfll, dpll0) = clocks.gclk0.swap(clocks.dfll, dpll0);
//! ```
//!
//! At this point, the DFLL is completely unused, so it can be disbled and
//! deconstructed, leaving only the [`DfllToken`](dfll::DfllToken).
//!
//! ```ignore
//! let dfll_token = dfll.disable().free();
//! ```
//!
//! Our clock tree now looks like this:
//!
//! ```text
//! XOSC0 (8 MHz)
//! └── DPLL0 (100 MHz)
//!     └── GCLK0 (100 MHz)
//!         └── Master clock (100 MHz)
//!
//! OSCULP base clock
//! ```
//!
//! We have the clocks set up, but we're not using them for anything other than
//! the master clock. Our final steps will create SERCOM APB and peripheral
//! clocks and will output the raw GCLK0 to a GPIO pin.
//!
//! To enable the APB clock for SERCOM0, we must access the [`Apb`](apb::Apb)
//! bus struct. We provide an [`ApbToken`](apb::ApbToken) to the `Apb::enable`
//! method and receive an [`ApbClk`](apb::ApbClk) in return. APB clocks do not
//! depend on any other clocks, so there is no need to increment any existing
//! dependent clock count.
//!
//! ```ignore
//! let apb_sercom0 = buses.apb.enable(tokens.apbs.sercom0);
//! ```
//!
//! To enable a peripheral channel clock for SERCOM0, we must provide the
//! corresponding [`PclkToken`](pclk::PclkToken), as well as the instance of
//! `EnabledGclk0`, so that its counter can be incremented. The resulting clock
//! has the type `Pclk<Sercom0, Gclk0Id>`.
//!
//! ```ignore
//! let (pclk_sercom0, gclk0) = Pclk::enable(tokens.pclks.sercom0, gclk0);
//! ```
//!
//! Like [`Dpll<D, I>`](dpll::Dpll), [`Pclk<P, I>`](pclk::Pclk) also takes two
//! type parameters. The first represents the corresponding peripheral, while
//! the second is again an `Id` type representing the [`Source`] driving the
//! `Pclk`, which is restricted by the [`PclkSourceId`](pclk::PclkSourceId)
//! trait. Because peripheral channel clocks can only be driven by GCLKs,
//! `PclkSourceId` is effectively synonymous with the [`GclkId`](gclk::GclkId)
//! trait. Furthermore, like the [`AhbClk`](ahb::AhbClk) and
//! [`ApbClk`](apb::ApbClk) types, a `Pclk` is always a leaf in the clock tree.
//! In can never drive another clock, so it is never placed inside an
//! [`Enabled`] struct.
//!
//! Finally, we would like to output GCLK0 to a GPIO pin. Doing so takes a
//! similar approach to the `Pclk` above. But this time, we must also provide a
//! corresponding GPIO [`Pin`](crate::gpio::Pin), in this case `PB14`. And,
//! as with the `Pclk` above, creating a [`GclkOut`](gclkio::GclkOut) clock will
//! increment the dependent clock count of the `EnabledGclk0`.
//!
//! ```ignore
//! let (gclk_out0, gclk0) = GclkOut::enable(tokens.gclk_io.gclk_out0, pins.pb14, gclk0);
//! ```
//!
//! We have arrived at our final, desired clock tree. Putting the whole example
//! together, we get
//!
//! ```no_run
//! use atsamd_hal::{
//!     clock::v2::{
//!         clock_system_at_reset,
//!         dpll::Dpll,
//!         gclkio::GclkOut,
//!         pclk::Pclk,
//!         xosc::{CrystalCurrent, Xosc},
//!     },
//!     gpio::Pins,
//!     pac::Peripherals,
//!     time::U32Ext,
//! };
//!
//! let mut pac = Peripherals::take().unwrap();
//! let (mut buses, clocks, tokens) = clock_system_at_reset(
//!     pac.OSCCTRL,
//!     pac.OSC32KCTRL,
//!     pac.GCLK,
//!     pac.MCLK,
//!     &mut pac.NVMCTRL,
//! );
//! let pins = Pins::new(pac.PORT);
//! let xosc0 = Xosc::from_crystal(
//!     tokens.xosc0,
//!     pins.pa14,
//!     pins.pa15,
//!     8.mhz(),
//!     CrystalCurrent::Low,
//! )
//! .enable();
//! let (dpll0, xosc0) = Dpll::from_xosc0(tokens.dpll0, xosc0);
//! let dpll0 = dpll0.set_prediv(4).set_loop_div(50, 0).enable();
//! let (gclk0, dfll, dpll0) = clocks.gclk0.swap(clocks.dfll, dpll0);
//! let dfll_token = dfll.disable().free();
//! let apb_sercom0 = buses.apb.enable(tokens.apbs.sercom0);
//! let (pclk_sercom0, gclk0) = Pclk::enable(tokens.pclks.sercom0, gclk0);
//! let (gclk_out0, gclk0) = GclkOut::enable(tokens.gclk_io.gclk_out0, pins.pb14, gclk0);
//! ```

use typenum::{Unsigned, U0};

use crate::time::Hertz;
use crate::typelevel::{Counter, PrivateDecrement, PrivateIncrement, Sealed};

pub mod ahb;
pub mod apb;
pub mod dfll;
pub mod dpll;
pub mod gclk;
pub mod gclkio;
pub mod osculp32k;
pub mod pclk;
pub mod rtcosc;
pub mod types;
pub mod xosc;
pub mod xosc32k;

mod reset;
pub use reset::*;

/// Marks clock types that can act as a source for downstream clocks
///
/// Implementers of this type can drive downstream clocks in the clock tree.
/// Typically, implementors are [`Enabled`] clocks. The `Id` associated type
/// maps to the corresponding `Id` type of the implementer.
///
/// For example, `Enabled<Gclk5<DfllId>>` would implement
/// `Source<Id = Gclk5Id>`.
pub trait Source: Sealed {
    /// `Id` type of the implementer, e.g. [`Gclk5Id`](gclk::Gclk5Id) for
    /// `Enabled<Gclk5<DfllId>>`
    type Id;

    /// Returns the frequency of the clock source
    fn freq(&self) -> Hertz;
}

/// An enabled clock with compile-time counting of downstream users
///
/// This struct is a wrapper around other clock types from this module. It
/// represents a clock that has been enabled, and it maintains a compile-time
/// count of its uses by downstream clocks in the clock tree.
///
/// Compile-time counting allows the API to enforce when clocks may be enabled
/// or disabled. In particular, most clocks can only be disabled when their
/// counter is zero. However, there are exceptions, most notably [`Gclk0`],
/// which can never be disabled, because it is the main clock.
///
/// The count is maintained using the [`Counter`] trait, along with type-level,
/// [`Unsigned`] integers from the [`typenum`] crate.
///
/// [`Gclk0`]: gclk::Gclk0
pub struct Enabled<T, N: Counter = U0>(pub(crate) T, N);

impl<T, N: Counter> Sealed for Enabled<T, N> {}

impl<T, N: Unsigned + Counter> Enabled<T, N> {
    #[inline]
    pub(crate) fn new(t: T) -> Self {
        Enabled(t, N::default())
    }
}

impl<T, N: PrivateIncrement> PrivateIncrement for Enabled<T, N> {
    type Inc = Enabled<T, N::Inc>;

    #[inline]
    fn inc(self) -> Self::Inc {
        Enabled(self.0, self.1.inc())
    }
}

impl<T, N: PrivateDecrement> PrivateDecrement for Enabled<T, N> {
    type Dec = Enabled<T, N::Dec>;

    #[inline]
    fn dec(self) -> Self::Dec {
        Enabled(self.0, self.1.dec())
    }
}

impl<T, N: Counter> Counter for Enabled<T, N> {}

#[allow(dead_code)]
fn test() {
    use crate::{
        clock::v2::{
            dpll::Dpll,
            gclkio::GclkOut,
            pclk::Pclk,
            xosc::{CrystalCurrent, Xosc},
        },
        gpio::Pins,
        pac::Peripherals,
        time::U32Ext,
    };

    let mut pac = Peripherals::take().unwrap();
    let (mut buses, clocks, tokens) = clock_system_at_reset(
        pac.OSCCTRL,
        pac.OSC32KCTRL,
        pac.GCLK,
        pac.MCLK,
        &mut pac.NVMCTRL,
    );
    let pins = Pins::new(pac.PORT);
    let xosc0 = Xosc::from_crystal(
        tokens.xosc0,
        pins.pa14,
        pins.pa15,
        8.mhz(),
        CrystalCurrent::Low,
    )
    .enable();
    let (dpll0, _xosc0) = Dpll::from_xosc0(tokens.dpll0, xosc0);
    let dpll0 = dpll0.set_prediv(4).set_loop_div(50, 0).enable();
    let (gclk0, dfll, _dpll0) = clocks.gclk0.swap(clocks.dfll, dpll0);
    let _dfll_token = dfll.disable().free();
    let _apb_sercom0 = buses.apb.enable(tokens.apbs.sercom0);
    let (_pclk_sercom0, gclk0) = Pclk::enable(tokens.pclks.sercom0, gclk0);
    let (_gclk_out0, _gclk0) = GclkOut::enable(tokens.gclk_io.gclk_out0, pins.pb14, gclk0);
}
