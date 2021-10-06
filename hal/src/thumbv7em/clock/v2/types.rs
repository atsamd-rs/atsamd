#![warn(missing_docs)]
//! # Clocking API v2 type infrastructure
//!
//! In order to model _one-to-many_ type of relationship between dependencies in
//! a Clocking API v2 a few additional concepts/abstractions were introduced.
//!
//! ## [`Enabled`] type and its helper types/traits
//!
//! [`Enabled`] type wrapper represents a clocking component in its enabled
//! state while also holding an information about current amount of dependencies
//! (usually [`zero`](typenum::U0) upon construction). This amount of
//! dependencies is embedded into the type via second generic parameter
//! leveraging [`typenum::UInt`] [`typenum::UTerm`] types.
//!
//! ```no_run
//! # use core::marker::PhantomData;
//! pub trait Counter {} /* implemented for every `typenum::Unsigned` */
//!
//! pub trait Increment: Counter {
//!     /* implemented for every `typenum::Unsigned` and `Enabled` */
//!     type Inc: Counter;
//!     fn inc(self) -> Self::Inc;
//! }
//!
//! pub trait Decrement: Counter {
//!     /* implemented for every `typenum::Unsigned` and `Enabled` */
//!     type Dec: Counter;
//!     fn dec(self) -> Self::Dec;
//! }
//!
//! pub struct Enabled<T, N: Counter>(pub(crate) T, PhantomData<N>);
//! ```
//!
//! Via specialized implementation blocks for this type (like for
//! `Enabled<Dfll<TMode>, U0>`) it is possible to introduce special behaviour;
//! e.g. [`Enabled::disable`] will only exist for clocking component having
//! [`zero`](typenum::U0) current users.
//!
//! ## `SourceMarker` trait and its subtraits
//!
//! This marker trait unifies family of more specific traits. These ones are
//! essential during a construction `fn ::{new, enable}` and deconstruction `fn
//! ::{free, disable}` of clocking components as they provide information to the
//! constructed/deconstructed type what its source is (shown in the example
//! later) and which variant of source (associated constant) is applicable while
//! performing a HW register write.
//!
//! ```no_run
//! # use atsamd_hal::clock::v2::gclk::GenNum;
//! pub trait SourceMarker {}
//!
//! pub trait GclkSourceMarker: SourceMarker {
//!     const GCLK_SRC: atsamd_hal::pac::gclk::genctrl::SRC_A /* GclkSourceEnum */;
//! }
//!
//! pub trait PclkSourceMarker: GenNum + SourceMarker {
//!     const PCLK_SRC: atsamd_hal::pac::gclk::pchctrl::GEN_A /* PclkSourceEnum */;
//! }
//!
//! pub trait DpllSourceMarker: SourceMarker {
//!     const DPLL_SRC: atsamd_hal::pac::oscctrl::dpll::dpllctrlb::REFCLK_A /* DpllSrc */;
//! }
//!
//! pub trait GclkOutSourceMarker: GenNum + SourceMarker {}
//! ```
//!
//! These are implemented by marker types corresponding to existing clocking
//! abstractions e.g.:
//!
//! ```no_run
//! # use atsamd_hal::clock::v2::gclk::GclkSourceEnum;
//! # pub trait SourceMarker {}
//! # pub trait GclkSourceMarker /*: SourceMarker */ {
//! #     const GCLK_SRC: atsamd_hal::pac::gclk::genctrl::SRC_A /* GclkSourceEnum */;
//! # }
//! pub enum Pll1 {}
//! impl GclkSourceMarker for Pll1 {
//!     const GCLK_SRC: GclkSourceEnum = GclkSourceEnum::DPLL1;
//! }
//! // or
//! pub enum Dfll {}
//! impl GclkSourceMarker for Dfll {
//!     const GCLK_SRC: GclkSourceEnum = GclkSourceEnum::DFLL;
//! }
//! ```
//!
//! ## `Source` trait and its subtraits
//!
//! This trait represents a source of clocking signal while subtraits its more
//! specialized flavours (source of signal for [`Dpll`](super::v2::dpll::Dpll),
//! [`Pclk`](super::v2::pclk::Pclk), [`Gclk`](super::v2::gclk::Gclk), etc.).
//! ```no_run
//! # use atsamd_hal::time::Hertz;
//! # use atsamd_hal::clock::v2::gclk::{GenNum, GclkSourceMarker};
//! # use atsamd_hal::clock::v2::pclk::PclkSourceMarker;
//! # use atsamd_hal::clock::v2::dpll::DpllSourceMarker;
//! # use atsamd_hal::clock::v2::gclkio::GclkOutSourceMarker;
//! pub trait Source {
//!     fn freq(&self) -> Hertz;
//! }
//!
//! pub trait GclkSource<G: GenNum>: Source {
//!     type Type: GclkSourceMarker;
//! }
//!
//! pub trait PclkSource: Source {
//!     type Type: PclkSourceMarker;
//! }
//!
//! pub trait DpllSource: Source {
//!     type Type: DpllSourceMarker;
//! }
//!
//! pub trait PrivateGclkOutSource: Source {
//!     fn enable_gclk_out(&mut self, polarity: bool);
//!     fn disable_gclk_out(&mut self);
//! }
//!
//! pub trait GclkOutSource: PrivateGclkOutSource {
//!     type Type: GclkOutSourceMarker;
//! }
//! ```
//!
//! These are implemented by corresponding specialized types of [`Enabled`]
//! e.g.:
//!
//! ```no_run
//! # use atsamd_hal::clock::types::{Counter, Enabled};
//! # use atsamd_hal::clock::v2::Source;
//! # use atsamd_hal::clock::v2::dpll::{Dpll, DpllNum, SrcMode};
//! # use atsamd_hal::clock::v2::gclk::{GclkSourceMarker, GenNum};
//! # pub trait GclkSource<G: GenNum>: Source {
//! #     type Type: GclkSourceMarker;
//! # }
//! impl<G, D, M, N> GclkSource<G> for Enabled<Dpll<D, M>, N>
//! where
//!     G: GenNum,
//!     D: DpllNum + GclkSourceMarker,
//!     M: SrcMode<D>,
//!     N: Counter,
//! {
//!     type Type = D;
//! }
//! ```
//!
//! Regardless of how complicated it might seem to look, it can be roughly
//! understood as:
//! - Enabled [`Dpll`](super::v2::dpll::Dpll) peripheral can be a source of
//!   signal for any [`Gclk`](super::v2::gclk::Gclk).
//!
//! ## `*Token` types
//! Unfortunately, [`Peripherals`](crate::pac::Peripherals) granularity is too
//! low for them to be useful when spliting clocking system into such small,
//! semi-independent pieces. In order to solve this problem, we consume PAC
//! in [`retrieve_clocks`](super::v2::retrieve_clocks) and return a set of
//! tokens that internally use appropriate `RegisterBlock` directly, retrieved
//! from a raw pointer . It is safe because register regions managed by
//! different tokens do not overlap. Tokens cannot be created by a user; they
//! are provided during initialization and do not expose any public API. Memory
//! accesses are read/write-synchronized.
//!
//! ## `Source/SourceMarker` traits usage in an API
//!
//! This is a slightly simplified example of how more less every clocking
//! component that relies on _one-to-many_ depenedency relationships is
//! implemented
//!
//! ```no_run
//! # use core::marker::PhantomData;
//! # use typenum::U0;
//! # use atsamd_hal::clock::types::{Counter, Decrement, Increment};
//! # use atsamd_hal::clock::v2::gclk::{GenNum, GclkSourceMarker, GclkToken, GclkSource};
//! # pub struct Enabled<T, N: Counter>(pub(crate) T, PhantomData<N>);
//! # impl<T, N: Counter> Enabled<T, N> {
//! #     pub(crate) fn new(t: T) -> Self {
//! #         Enabled(t, PhantomData)
//! #     }
//! # }
//! # struct Gclk<G: GenNum, T: GclkSourceMarker> {
//! #    token: GclkToken<G>,
//! #    src: PhantomData<T>,
//! # }
//! impl<G, T> Gclk<G, T>
//! where
//!     // `GenNum` is a generalization of a Gclk compile time parameters
//!     // (e.g. ordering number via associated constant)
//!     G: GenNum,
//!     // Practical application of `SourceMarker`; it makes a connection between
//!     // `Source` and a `Gclk` used by it
//!     T: GclkSourceMarker,
//! {
//!     pub fn new<S>(token: GclkToken<G>, source: S) -> (Self, S::Inc)
//!     where
//!         S: GclkSource<G, Type = T> + Increment,
//!     {
//!         // .. implementation details ..
//!         let gclk = Gclk {
//!             token,
//! #           src: PhantomData,
//!             /* ... */
//!         };
//!         (gclk, source.inc())
//!     }
//!
//!     pub fn free<S>(self, source: S) -> (GclkToken<G>, S::Dec)
//!     where
//!         S: GclkSource<G, Type = T> + Decrement,
//!     {
//!         (self.token, source.dec())
//!     }
//!
//!     pub fn enable(mut self) -> Enabled<Self, U0> {
//!         // HW register writes
//!         Enabled::new(self)
//!     }
//!     // Other functions operating on a disabled `Gclk<G, T>`
//! }
//! impl<G, T> Enabled<Gclk<G, T>, U0>
//! where
//!     G: GenNum,
//!     T: GclkSourceMarker,
//! {
//!     fn disable(mut self) -> Gclk<G, T> {
//!         // HW register writes
//!         self.0
//!     }
//! }
//! ```
//!
//! [`Gclk::new`](super::v2::gclk::Gclk::new) consumes, upon construction, a
//! [`GclkSource`](super::v2::gclk::GclkSource) provided to it and returns it
//! with a type of `Enabled<_, N++>` (as mentioned previously, specializations
//! of [`Enabled`] implement `Source` based traits). Analogically,
//! [`Gclk::free`](super::v2::gclk::Gclk::free) consumes a
//! [`GclkSource`](super::v2::gclk::GclkSource) passed in and returns it with a
//! new type of `Enabled<_, N-->`. By design it is impossible to go below
//! [`zero`](typenum::U0), because there is always less or equal amount of users
//! than a counter value.
//!
//! ## `Gclk0` case
//!
//! Amount of users might be less than a value of a counter in case of special
//! types like [`Gclk0`](super::v2::gclk::Gclk0) which always has an implicit
//! single user -- synchronous clocking domain. Minimal amount of users for it
//! is [`one`](typenum::U1), making it impossible to disable and therefore
//! consistent with its documented HW characteristics.
//!
//! It also makes it impossible to change a configuration of a
//! [`Gclk0`](super::v2::gclk::Gclk0) as a `Enabled<Gclk0, _>` cannot be
//! deconstructed. Therefore, `Enabled<Gclk0, U1>` exposes additional methods
//! that are usually available only for disabled
//! [`Gclks`](super::v2::gclk::Gclk).
use core::marker::PhantomData;
use core::ops::{Add, Sub};
use typenum::{Add1, Sub1, Unsigned, B1};

use crate::typelevel::Sealed;

mod private {
    use super::*;
    pub trait Increment: Counter {
        type Inc: Counter;
        fn inc(self) -> Self::Inc;
    }
    pub trait Decrement: Counter {
        type Dec: Counter;
        fn dec(self) -> Self::Dec;
    }
}

pub(crate) use private::Decrement as PrivateDecrement;
pub(crate) use private::Increment as PrivateIncrement;

/// Trait indicating that type can be _incremented_.
///
/// In practice, it is used as a type constraint for a generic source during a
/// construction phase (`fn ::{new, enable}`) and is implemented by [`Enabled`]
/// types of `N >= `[`0`](typenum::U0)
pub trait Increment: PrivateIncrement {}

/// Trait indicating that type can be _decremented_.
///
/// In practice, it is used as a type constraint for a generic source during a
/// destruction phase (`fn ::{free, disable}`) and is implemented by [`Enabled`]
/// types of `N > `[`0`](typenum::U0)
pub trait Decrement: PrivateDecrement {}

impl<N> PrivateIncrement for N
where
    N: Sealed + Unsigned + Add<B1>,
    Add1<N>: Sealed + Unsigned,
{
    type Inc = Add1<N>;
    fn inc(self) -> Self::Inc {
        Self::Inc::default()
    }
}

impl<N> PrivateDecrement for N
where
    N: Sealed + Unsigned + Sub<B1>,
    Sub1<N>: Sealed + Unsigned,
{
    type Dec = Sub1<N>;
    fn dec(self) -> Self::Dec {
        Self::Dec::default()
    }
}

/// Trait generalizing over _countable_ types.
pub trait Counter: Sealed {}

impl<N: Unsigned + Sealed> Counter for N {}

/// Wrapper type representing enabled multiuser clocking abstraction
///
/// Specialized implementations for this type usually allow for a specific
/// behaviour dependent on a user count. For instance, clocking components are
/// disableable only when no one is using them.
pub struct Enabled<T, N: Counter>(pub(crate) T, PhantomData<N>);

impl<T, N: Counter> Enabled<T, N> {
    pub(crate) fn new(t: T) -> Self {
        Enabled(t, PhantomData)
    }
}

impl<T, N: Counter> Counter for Enabled<T, N> {}

impl<T, N: Counter> Sealed for Enabled<T, N> {}

impl<T, N: PrivateIncrement> PrivateIncrement for Enabled<T, N> {
    type Inc = Enabled<T, N::Inc>;

    fn inc(self) -> Self::Inc {
        Enabled(self.0, PhantomData)
    }
}

impl<T, N: PrivateIncrement> Increment for Enabled<T, N> {}

impl<T, N: PrivateDecrement> PrivateDecrement for Enabled<T, N> {
    type Dec = Enabled<T, N::Dec>;

    fn dec(self) -> Self::Dec {
        Enabled(self.0, PhantomData)
    }
}

impl<T, N: PrivateDecrement> Decrement for Enabled<T, N> {}
