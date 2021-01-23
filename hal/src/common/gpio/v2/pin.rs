//! # Type-level module for GPIO pins
//!
//! This module provides a type-level API for GPIO pins. It uses the type system
//! to track the state of pins at compile-time. To do so, it uses traits to
//! represent meta-types and types as instances of those meta-types. For
//! example, the trait [`InputConfig`] acts as a type-level `enum` of the
//! available input configurations, and the types [`Floating`], [`PullDown`] and
//! [`PullUp`] are the type-level variants of [`InputConfig`].
//!
//! When applied as a trait bound, meta-types restrict type parameters to the
//! corresponding variants. All of the traits in this module are closed, using
//! the `Sealed` trait pattern, so the type-level instances found in this module
//! are the only possible variants.
//!
//! Type-level pins are parameterized by two main meta-types, [`PinId`] and
//! [`PinMode`].
//!
//! The [`PinId`] meta-type identifies a pin by it's group (A, B, C or D) and
//! pin number. Each [`PinId`] instance is named according to its datasheet
//! identifier, e.g. [`PA27`].
//!
//! The [`PinMode`] meta-type represents the various pin modes. The available
//! [`PinMode`] variants are [`Disabled`], [`Input`], [`Output`] and
//! [`Alternate`], each with its own corresponding configurations.
//!
//! The [`Pin`] struct acts as a type-level instance of a pin. It is
//! parameterized by two type parameters, a [`PinId`] and a [`PinMode`].
//! [`Pin`]s with different [`PinId`]s or [`PinMode`]s are considered distinct
//! types by the compiler. As a consequence, converting from one [`PinMode`] to
//! another requires changing type. Functions that change [`PinMode`] must
//! consume the existing instance and return a new instance.
//!
//! It is not possible for users to create new instances of a [`Pin`]. Singleton
//! instances of each pin are made available to users through the [`Pins`]
//! struct.
//!
//! To create the [`Pins`] struct, users must supply the PAC
//! [`PORT`](crate::target_device::PORT) peripheral. The [`Pins`] struct takes
//! ownership of the [`PORT`](crate::target_device::PORT) and provides the
//! corresponding pins. Each [`Pin`] within the [`Pins`] struct can be moved out
//! and used individually.
//!
//!
//! ```rust
//! let mut peripherals = Peripherals::take().unwrap();
//! let pins = Pins::new(peripherals.PORT);
//! ```
//!
//! Pins can be converted between modes using several different methods.
//!
//! ```rust
//! // Use one of the literal function names
//! let pa27 = pins.pa27.into_floating_input();
//! // Use a generic method and one of the `PinMode` variant types
//! let pa27 = pins.pa27.into_mode::<FloatingInput>();
//! // Specify the target type and use `From`/`Into`
//! let pa27: Pin<PA27, FloatingInput> = pins.pa27.into();
//! ```
//!
//! # Embedded HAL traits
//!
//! This module implements all of the embedded HAL GPIO traits for each [`Pin`]
//! in the corresponding [`PinMode`]s, namely:
//! [`InputPin`](embedded_hal::digital::v2::InputPin),
//! [`OutputPin`](embedded_hal::digital::v2::OutputPin),
//! [`ToggleableOutputPin`](embedded_hal::digital::v2::ToggleableOutputPin) and
//! [`StatefulOutputPin`](embedded_hal::digital::v2::StatefulOutputPin).
//!
//! # Type-level encapsulation
//!
//! Normally, storing a generic pin within some data structure requires two type
//! parameters.
//!
//! ```rust
//! struct UserStruct<I: PinId, M: PinMode> {
//!     pin: Pin<I, M>
//! }
//! ```
//!
//! As an alternative, this module provides a trait to encapsulate a pin with a
//! single type-parameter, [`AnyPin`]. The [`AnyPin`] trait is implemented by
//! every possible variant of [`Pin`], so it can be used as a trait bound for
//! pins. With this approach, only one type parameter is required.
//!
//! ```rust
//! struct UserStruct<P: AnyPin> {
//!     pin: P
//! }
//! ```
//!
//! Moreover, no information is lost with this approach, because the [`AnyPin`]
//! trait has associated types for each type parameter of [`Pin`]. Use these
//! associated types to apply trait bounds or restrict the pin in some way.
//!
//! ```rust
//! struct UserStruct<P>
//! where
//!     P: AnyPin<Mode = AlternateE>,
//!     P::Id: SomeUserTrait,
//! {
//!     pin: P
//! }
//! ```
//!
//! However, note that working with a generic type constrained by [`AnyPin`] is
//! a bit different than working directly with a concrete type. See the
//! [`AnyPin`] documentation for more details.
//!
//! # Optional pins
//!
//! Finally, this module provides an easy way to implement optional pins. The
//! trait [`OptionalPin`] is implemented for each [`Pin`] as well as the
//! [`NoneT`](crate::typelevel::NoneT) struct.
//! [`NoneT`](crate::typelevel::NoneT) acts as a type-level version of the
//! [`None`](core::option::Option::None) variant. The [`SomePin`] trait has both
//! [`OptionalPin`] and [`AnyPin`] as super traits, so it can be used as a bound
//! to guarantee a valid pin and provide access to the [`AnyPin`] associated
//! types.
//!
//! ```rust
//! struct UserStruct<P: OptionalPin> {
//!     pin: P
//! }
//!
//! impl<P: OptionalPin> UserStruct<P> {
//!     pub fn new() -> UserStruct<NoneT> {
//!         UserStruct { pin: NoneT }
//!     }
//!     pub fn init<Q>(self, pin: Q) -> UserStruct<Q>
//!     where
//!         Q: SomePin<Mode = PushPullOutput>,
//!     {
//!         UserStruct { pin }
//!     }
//! }
//! ```

use core::convert::Infallible;
use core::marker::PhantomData;

use paste::paste;

use hal::digital::v2::OutputPin;
#[cfg(feature = "unproven")]
use hal::digital::v2::{InputPin, StatefulOutputPin, ToggleableOutputPin};

use crate::target_device::PORT;

use crate::typelevel::{NoneT, Sealed};

use super::dynpin::*;

//==============================================================================
//  Disabled configurations
//==============================================================================

/// Type-level `enum` for disabled configurations
pub trait DisabledConfig: Sealed {
    /// Corresponding [`DynDisabled`](super::DynDisabled)
    const DYN: DynDisabled;
    /// Value of the PULLEN field in this configuration
    const PULLEN: bool;
    /// Value of the OUT field in this configuration
    const OUT: bool = false;
}

/// Type-level variant of both [`DisabledConfig`] and [`InputConfig`]
pub enum Floating {}
/// Type-level variant of both [`DisabledConfig`] and [`InputConfig`]
pub enum PullDown {}
/// Type-level variant of both [`DisabledConfig`] and [`InputConfig`]
pub enum PullUp {}

impl Sealed for Floating {}
impl Sealed for PullDown {}
impl Sealed for PullUp {}

impl DisabledConfig for Floating {
    const DYN: DynDisabled = DynDisabled::Floating;
    const PULLEN: bool = false;
}
impl DisabledConfig for PullDown {
    const DYN: DynDisabled = DynDisabled::PullDown;
    const PULLEN: bool = true;
    const OUT: bool = false;
}
impl DisabledConfig for PullUp {
    const DYN: DynDisabled = DynDisabled::PullUp;
    const PULLEN: bool = true;
    const OUT: bool = true;
}

/// Type-level variant of [`PinMode`] for disabled modes
///
/// Type `C` is one of three configurations: [`Floating`], [`PullDown`] or
/// [`PullUp`]
pub struct Disabled<C: DisabledConfig> {
    cfg: PhantomData<C>,
}

impl<C: DisabledConfig> Sealed for Disabled<C> {}

/// Type-level variant of [`PinMode`] for floating disabled mode
pub type FloatingDisabled = Disabled<Floating>;

/// Type-level variant of [`PinMode`] for pull-down disabled mode
pub type PullDownDisabled = Disabled<PullDown>;

/// Type-level variant of [`PinMode`] for pull-up disabled mode
pub type PullUpDisabled = Disabled<PullUp>;

/// Type alias for the [`PinMode`] at reset
pub type Reset = FloatingDisabled;

//==============================================================================
//  Input configurations
//==============================================================================

/// Type-level `enum` for input configurations
pub trait InputConfig: Sealed {
    /// Corresponding [`DynInput`](super::DynInput)
    const DYN: DynInput;
    /// Value of the PULLEN field in this configuration
    const PULLEN: bool;
    /// Value of the OUT field in this configuration
    const OUT: bool = false;
}

impl InputConfig for Floating {
    const DYN: DynInput = DynInput::Floating;
    const PULLEN: bool = false;
}
impl InputConfig for PullDown {
    const DYN: DynInput = DynInput::PullDown;
    const PULLEN: bool = true;
    const OUT: bool = false;
}
impl InputConfig for PullUp {
    const DYN: DynInput = DynInput::PullUp;
    const PULLEN: bool = true;
    const OUT: bool = true;
}

/// Type-level variant of [`PinMode`] for input modes
///
/// Type `C` is one of three input configurations: [`Floating`], [`PullDown`] or
/// [`PullUp`]
pub struct Input<C: InputConfig> {
    cfg: PhantomData<C>,
}

impl<C: InputConfig> Sealed for Input<C> {}

/// Type-level variant of [`PinMode`] for floating input mode
pub type FloatingInput = Input<Floating>;

/// Type-level variant of [`PinMode`] for pull-down input mode
pub type PullDownInput = Input<PullDown>;

/// Type-level variant of [`PinMode`] for pull-up input mode
pub type PullUpInput = Input<PullUp>;

//==============================================================================
//  Output configurations
//==============================================================================

/// Type-level `enum` for output configurations
pub trait OutputConfig: Sealed {
    /// Corresponding [`DynOutput`](super::DynOutput)
    const DYN: DynOutput;
    /// Value of the INEN field in this configuration
    const INEN: bool;
}

/// Type-level variant of [`OutputConfig`] for a push-pull configuration
pub enum PushPull {}
/// Type-level variant of [`OutputConfig`] for a readable push-pull
/// configuration
pub enum Readable {}

impl Sealed for PushPull {}
impl Sealed for Readable {}

impl OutputConfig for PushPull {
    const DYN: DynOutput = DynOutput::PushPull;
    const INEN: bool = false;
}
impl OutputConfig for Readable {
    const DYN: DynOutput = DynOutput::Readable;
    const INEN: bool = true;
}

/// Type-level variant of [`PinMode`] for output modes
///
/// Type `C` is one of two output configurations: [`PushPull`] or [`Readable`]
pub struct Output<C: OutputConfig> {
    cfg: PhantomData<C>,
}

impl<C: OutputConfig> Sealed for Output<C> {}

/// Type-level variant of [`PinMode`] for push-pull output mode
pub type PushPullOutput = Output<PushPull>;

/// Type-level variant of [`PinMode`] for readable push-pull output mode
pub type ReadableOutput = Output<Readable>;

//==============================================================================
//  Alternate configurations
//==============================================================================

/// Type-level `enum` for alternate peripheral function configurations
pub trait AlternateConfig: Sealed {
    /// Corresponding [`DynAlternate`](super::DynAlternate)
    const DYN: DynAlternate;
    /// Value of the PMUXE/PMUXO field in this configuration
    const PMUX: u8;
}

macro_rules! alternate {
    ([
        $(
            $( #[$cfg:meta] )?
            ($Letter:ident, $NUM:literal),
        )+
    ]) => {
        paste! {
            $(
                $( #[$cfg] )?
                #[
                    doc = "Type-level variant of [`AlternateConfig`] for \
                    alternate peripheral function " $Letter
                ]
                pub enum $Letter {}
                $( #[$cfg] )?
                impl Sealed for $Letter {}
                $( #[$cfg] )?
                impl AlternateConfig for $Letter {
                    const DYN: DynAlternate = DynAlternate::$Letter;
                    const PMUX: u8 = $NUM;
                }
                $( #[$cfg] )?
                #[
                    doc = "Type-level variant of [`PinMode`] for alternate \
                    peripheral function [" $Letter "]"
                ]
                pub type [<Alternate $Letter>] = Alternate<$Letter>;
            )+
        }
    };
}

alternate!([
    (A, 00),
    (B, 01),
    (C, 02),
    (D, 03),
    (E, 04),
    (F, 05),
    (G, 06),
    #[cfg(any(feature = "samd21", feature = "min-samd51g"))]
    (H, 07),
    #[cfg(feature = "min-samd51g")]
    (I, 08),
    #[cfg(feature = "min-samd51g")]
    (J, 09),
    #[cfg(feature = "min-samd51g")]
    (K, 10),
    #[cfg(feature = "min-samd51g")]
    (L, 11),
    #[cfg(feature = "min-samd51g")]
    (M, 12),
    #[cfg(feature = "min-samd51g")]
    (N, 13),
]);

/// Type-level variant of [`PinMode`] for alternate peripheral functions
///
/// Type `C` is an [`AlternateConfig`]
pub struct Alternate<C: AlternateConfig> {
    cfg: PhantomData<C>,
}

impl<C: AlternateConfig> Sealed for Alternate<C> {}

//==============================================================================
//  Pin modes
//==============================================================================

/// Type-level `enum` representing pin modes
pub trait PinMode: Sealed + Sized {
    /// Corresponding [`DynPinMode`](super::DynPinMode)
    const DYN: DynPinMode;
    /// Value of the DIR field in this mode
    const DIR: bool = false;
    /// Value of the INEN field in this mode
    const INEN: bool = false;
    /// Value of the PULLEN field in this mode
    const PULLEN: bool = false;
    /// Value of the OUT field in this mode
    const OUT: bool = false;
    /// Value of the PMUXEN field in this mode
    const PMUXEN: bool = false;
    /// Value of the PMUXE/PMUXO field in this mode
    const PMUX: u8 = 0;
    /// Convert a [`Pin`] into this [`PinMode`]
    #[inline]
    fn into_mode<I, M>(_pin: Pin<I, M>) -> Pin<I, Self>
    where
        I: PinId,
        M: PinMode,
    {
        // SAFETY: We have exclusive control of the pin, so this is safe to use.
        unsafe { Self::convert(I::Group::group(), I::NUM) };
        Pin::new()
    }
    /// Set the hardware registers for a given [`PinMode`]
    ///
    /// # Safety
    /// This function is only safe to use with exclusive control of a pin,
    /// through ownership or a mutable reference. It uses the GROUP pointer
    /// safely, because it only writes bits of the corresponding pin and does so
    /// with atomic operations. Normally, it would not be possible to configure
    /// the PMUX value without writing bits for another pin, but the WRCONFIG
    /// register makes this possible.
    #[doc(hidden)]
    #[inline]
    unsafe fn convert(group: *const GROUP, num: u8) {
        let mask: u32 = 1 << num;
        (*group).wrconfig.write(|w| {
            if num & 0x10 != 0 {
                w.hwsel().set_bit();
            } else {
                w.hwsel().clear_bit();
            }
            w.wrpincfg().set_bit();
            w.wrpmux().set_bit();
            w.pmux().bits(Self::PMUX);
            w.pullen().bit(Self::PULLEN);
            w.inen().bit(Self::INEN);
            w.pmuxen().bit(Self::PMUXEN);
            w.pinmask().bits(1 << (num & 0xF))
        });
        if Self::DIR {
            (*group).dirset.write(|w| w.bits(mask));
        } else {
            (*group).dirclr.write(|w| w.bits(mask));
        }
        if Self::PULLEN {
            if Self::OUT {
                (*group).outset.write(|w| w.bits(mask));
            } else {
                (*group).outclr.write(|w| w.bits(mask));
            }
        }
    }
}

impl<C: DisabledConfig> PinMode for Disabled<C> {
    const DYN: DynPinMode = DynPinMode::Disabled(C::DYN);
    const DIR: bool = false;
    const INEN: bool = false;
    const PULLEN: bool = C::PULLEN;
    const OUT: bool = C::OUT;
}

impl<C: InputConfig> PinMode for Input<C> {
    const DYN: DynPinMode = DynPinMode::Input(C::DYN);
    const DIR: bool = false;
    const INEN: bool = true;
    const PULLEN: bool = C::PULLEN;
    const OUT: bool = C::OUT;
}

impl<C: OutputConfig> PinMode for Output<C> {
    const DYN: DynPinMode = DynPinMode::Output(C::DYN);
    const DIR: bool = true;
    const INEN: bool = C::INEN;
}

impl<C: AlternateConfig> PinMode for Alternate<C> {
    const DYN: DynPinMode = DynPinMode::Alternate(C::DYN);
    const PMUXEN: bool = true;
    const PMUX: u8 = C::PMUX;
}

/// Use a recursive macro to implement [`From`](core::convert::From) for each
/// pair of [`PinMode`]s. A macro is necessary to avoid conflicting with the
/// reflexive implementation in [`core::convert`], i.e. `impl<T> From<T> for T`.
macro_rules! impl_core_convert_from {
    (
        $( #[$cfg1:meta] )?
        $Mode1:ident,
    ) => {};
    (
        #[$cfg1:meta]
        $Mode1:ident,
        $(
            $( #[$cfg2:meta] )?
            $Mode2:ident,
        )*
    ) => {
        #[$cfg1]
        impl_core_convert_from!(
            $Mode1,
            $(
                $( #[$cfg2] )?
                $Mode2,
            )*
        );
    };
    (
        $Mode1:ident,
        $(
            $( #[$cfg2:meta] )?
            $Mode2:ident,
        )*
    ) => {
        paste! {
            $(
                $( #[$cfg2] )?
                impl<I> From<Pin<I, $Mode1>> for Pin<I, $Mode2>
                where
                    I: PinId,
                {
                    #[doc = "Convert from [`" $Mode1 "`] to [`" $Mode2 "`]"]
                    #[inline]
                    fn from(pin: Pin<I, $Mode1>) -> Self {
                        pin.into_mode()
                    }
                }
                $( #[$cfg2] )?
                impl<I> From<Pin<I, $Mode2>> for Pin<I, $Mode1>
                where
                    I: PinId,
                {
                    #[doc = "Convert from [`" $Mode2 "`] to [`" $Mode1 "`]"]
                    #[inline]
                    fn from(pin: Pin<I, $Mode2>) -> Self {
                        pin.into_mode()
                    }
                }
            )*
            impl_core_convert_from!(
                $(
                    $( #[$cfg2] )?
                    $Mode2,
                )*
            );
        }
    };
}

impl_core_convert_from!(
    FloatingDisabled,
    PullDownDisabled,
    PullUpDisabled,
    FloatingInput,
    PullDownInput,
    PullUpInput,
    PushPullOutput,
    ReadableOutput,
    AlternateA,
    AlternateB,
    AlternateC,
    AlternateD,
    AlternateE,
    AlternateF,
    AlternateG,
    #[cfg(any(feature = "samd21", feature = "min-samd51g"))]
    AlternateH,
    #[cfg(feature = "min-samd51g")]
    AlternateI,
    #[cfg(feature = "min-samd51g")]
    AlternateJ,
    #[cfg(feature = "min-samd51g")]
    AlternateK,
    #[cfg(feature = "min-samd51g")]
    AlternateL,
    #[cfg(feature = "min-samd51g")]
    AlternateM,
    #[cfg(feature = "min-samd51g")]
    AlternateN,
);

//==============================================================================
//  Pin groups
//==============================================================================

// Because the [`Group`] trait gives access to the raw registers, hide it in a
// private module
mod private {
    use super::{Sealed, PORT};

    #[cfg(any(feature = "samd11", feature = "samd21"))]
    use crate::target_device::port::{
        CTRL, DIR, DIRCLR, DIRSET, DIRTGL, IN, OUT, OUTCLR, OUTSET, OUTTGL, PINCFG0_ as PINCFG,
        PMUX0_ as PMUX, WRCONFIG,
    };

    #[cfg(feature = "min-samd51g")]
    use crate::target_device::port::group::{
        CTRL, DIR, DIRCLR, DIRSET, DIRTGL, IN, OUT, OUTCLR, OUTSET, OUTTGL, PINCFG, PMUX, WRCONFIG,
    };

    /// The SAMD11 and SAMD21 PACs don't have the GROUP type.
    /// Manually re-implement it here
    #[repr(C)]
    pub struct GROUP {
        pub dir: DIR,
        pub dirclr: DIRCLR,
        pub dirset: DIRSET,
        pub dirtgl: DIRTGL,
        pub out: OUT,
        pub outclr: OUTCLR,
        pub outset: OUTSET,
        pub outtgl: OUTTGL,
        pub in_: IN,
        pub ctrl: CTRL,
        pub wrconfig: WRCONFIG,
        _padding1: [u8; 4],
        pub pmux: [PMUX; 16],
        pub pincfg: [PINCFG; 32],
        _padding2: [u8; 32],
    }

    /// Trait used to implement zero-sized references to the GROUP registers
    pub trait Group: Sealed {
        /// Represents the group number
        const NUM: usize;
        /// Return a pointer to the corresponding GROUP register block
        ///
        /// On its own, the `PORT` is `Send` but not `Sync`. It is not safe to
        /// access the `PORT` from multiple execution contexts without atomic
        /// operations, because it would be possible to accidentally overwrite
        /// configuration from another context. Specifically, each PMUX register
        /// controls two pins. If one execution context is performing a
        /// read/modify/write operation on the PMUX register but is preempted by
        /// another context modifying the same PMUX register for a different
        /// pin, it could corrupt the configuration.
        ///
        /// In this module, we would like to create singleton `Pin`s that are
        /// both `Send` and `Sync`. To do so, we need to make three guarantees:
        /// - Each `Pin` can only modify its own configuration.
        /// - Each `Pin` can only modify shared `PORT` registers using atomic
        ///   operations.
        /// - A `Pin` should not have interior mutability. It should require
        ///   ownership or a mutable reference to change the pin configuration.
        ///
        /// It is not possible to fulfill the first two conditions by directly
        /// accessing the PMUX register. Instead, we can use the WRCONFIG
        /// register to change the PMUX value.
        ///
        /// This function will return a raw pointer to the GROUP register block
        /// for the corresponding `PinId`. It is unsafe to use this pointer
        /// unless the above conditions are met.
        #[inline]
        fn group() -> *const GROUP {
            #[cfg(feature = "samd11")]
            const GROUPS: *const [GROUP; 1] = PORT::ptr() as *const [GROUP; 1];
            #[cfg(any(
                feature = "samd21",
                feature = "samd51g",
                feature = "samd51j",
                feature = "same51j"
            ))]
            const GROUPS: *const [GROUP; 2] = PORT::ptr() as *const [GROUP; 2];
            #[cfg(any(feature = "samd51n", feature = "same51n"))]
            const GROUPS: *const [GROUP; 3] = PORT::ptr() as *const [GROUP; 3];
            #[cfg(feature = "min-samd51p")]
            const GROUPS: *const [GROUP; 4] = PORT::ptr() as *const [GROUP; 4];
            unsafe { &(*GROUPS)[Self::NUM] }
        }
    }
}

pub(super) use private::{Group, GROUP};

macro_rules! group {
    ($Group:ident, $NUM:literal) => {
        /// Type-level variant of `Group`
        pub enum $Group {}
        impl Sealed for $Group {}
        impl Group for $Group {
            const NUM: usize = $NUM;
        }
    };
}

group!(GroupA, 0);
#[cfg(any(feature = "samd21", feature = "min-samd51g"))]
group!(GroupB, 1);
#[cfg(feature = "min-samd51n")]
group!(GroupC, 2);
#[cfg(feature = "min-samd51p")]
group!(GroupD, 3);

//==============================================================================
//  Pin IDs
//==============================================================================

/// Type-level `enum` for pin IDs
pub trait PinId: Sealed {
    /// Corresponding [`DynPinId`](super::DynPinId)
    const DYN: DynPinId;
    /// Pin group; Also acts as zero-sized reference to the GROUP registers
    type Group: Group;
    /// Pin number
    const NUM: u8;
}

macro_rules! pin_id {
    ($Group:ident, $Id:ident, $NUM:literal) => {
        paste! {
            #[doc = "Pin ID representing pin " $Id]
            pub enum $Id {}
            impl Sealed for $Id {}
            impl PinId for $Id {
                type Group = [<Group $Group>];
                const NUM: u8 = $NUM;
                const DYN: DynPinId = DynPinId {
                    group: DynGroup::$Group,
                    num: $NUM,
                };
            }
        }
    };
}

//==============================================================================
//  Pin trait
//==============================================================================

/// Type alias to recover the corresponding concrete [`Pin`] type from a given
/// implementation of [`AnyPin`]
pub type ConcretePin<P> = Pin<<P as AnyPin>::Id, <P as AnyPin>::Mode>;

/// Meta-type representing any [`Pin`]
///
/// All instances of [`Pin`] implement this trait. When used as a trait bound,
/// it acts to encapsulate a [`Pin`]. Without this trait, a completely generic
/// [`Pin`] requires two type parameters, i.e. `Pin<I, M>`. But when using this
/// trait, only one type parameter is required, i.e. `P: AnyPin`. However, even
/// though we have dropped a type parameter, no information is lost, because the
/// [`PinId`] and [`PinMode`] type parameters are stored as associated types in
/// the trait. The implementation of [`AnyPin`] looks something like this:
///
/// ```rust
/// impl<I: PinId, M: PinMode> AnyPin for Pin<I, M> {
///     type Id = I;
///     type Mode = M;
///     // ...
/// }
/// ```
///
/// Thus, there is a one-to-one mapping between `Pin<I, M>` and
/// `AnyPin<Id = I, Mode = M>`, so you can always recover the full, concrete
/// type from an implementation of [`AnyPin`]. The type alias [`ConcretePin`] is
/// provided for just this purpose.
///
/// ## `AnyPin` as a trait bound
///
/// When using [`AnyPin`] as a trait bound, you can constrain the associated
/// types to restrict the acceptable [`Pin`]s. For example, you could restrict
/// a function to accept a particular pin in any mode.
///
/// ```rust
/// fn example<P>(pin: P)
/// where
///     P: AnyPin<Id = PA27>
/// {
/// }
/// ```
///
/// Or you could accept any pin, as long as it's in the desired mode.
///
/// ```rust
/// fn example<P>(pin: P)
/// where
///     P: AnyPin<Mode = PullDownInput>
/// {
/// }
/// ```
///
/// You can also apply more complex bounds.
///
/// ```rust
/// fn example<P, C>(pin: P)
/// where
///     P: AnyPin<Mode = Output<C>>,
///     C: OutputConfig,
///     P::Id: UserTrait,
/// {
/// }
/// ```
///
/// ## Generic `AnyPin`s
///
/// Working with a generic type constrained by [`AnyPin`] is slightly different
/// than working with a concrete [`Pin`]. When compiling a generic function, the
/// compiler cannot assume anything about the specific concrete type. It can
/// only use what it knows about the [`AnyPin`] trait.  Consequently, we repeat
/// all of [`Pin`]'s inherent methods on the trait, for convenience. To cast a
/// generic type to a concrete type, use the
/// [`as_concrete`](AnyPin::as_concrete) method. To cast back to the generic
/// type, use the [`Pin`] method [`as_any`](Pin::as_any).
///
/// The following example walks through a few different ways to interact with a
/// generic type constrained by `AnyPin`.
///
/// Suppose you wanted to store a completely generic [`Pin`] within a struct.
/// You can do so using only one type parameter and the [`AnyPin`] trait.
///
/// ```rust
/// pub struct Example<P: AnyPin> {
///     pin: P,
/// }
/// ```
///
/// Next, suppose you want to create a method that will take the [`Pin`] out of
/// the struct, perform some operations in several [`PinMode`]s, and put it back
/// into the struct before returning. The `elided` method below shows such an
/// example. However, it can be a bit tricky to follow all of the type
/// conversions here. For clarity, the `expanded` method shows the same behavior
/// with each transformation given its proper type annotation. Notice that it is
/// not enough to simply put back a type that implements
/// `AnyPin<Id = P::Id, Mode = P::Mode>`. As far as the compiler is concerned,
/// there could be many different types that satisfy that trait bound. Instead,
/// you must put back a `P` exactly. The [`as_any`](Pin::as_any) function is the
/// key here. It transforms some type that implements
/// `AnyPin<Id = P::Id, Mode = P::Mode>` into `P` itself.
///
/// ```rust
/// impl<P: AnyPin> Example<P> {
///     pub fn elided(mut self) -> Self {
///         let mut pin = self.pin.into_push_pull_output();
///         pin.set_high().unwrap();
///         let pin = pin.into_floating_input();
///         let _bit = pin.is_low().unwrap();
///         self.pin = pin.into_mode().as_any();
///         self
///     }
///     pub fn expanded(mut self) -> Self {
///         // This step is skipped above, because the `into_push_pull_output`
///         // trait method automatically calls `as_concrete`.
///         // By definition, ConcretePin<P> == Pin<P::Id, P::Mode>
///         let pin: ConcretePin<P> = self.pin.as_concrete();
///         let mut pin: Pin<P::Id, PushPullOutput> = pin.into_push_pull_output();
///         pin.set_high().unwrap();
///         let pin: Pin<P::Id, FloatingInput> = pin.into_floating_input();
///         let _bit = pin.is_low().unwrap();
///         let pin: ConcretePin<P> = pin.into_mode::<P::Mode>();
///         self.pin: P = pin.as_any::<P>();
///         self
///     }
/// }
/// ```
pub trait AnyPin: Sized + Sealed {
    /// [`PinId`] of the corresponding [`Pin`]
    type Id: PinId;
    /// [`PinMode`] of the corresponding [`Pin`]
    type Mode: PinMode;

    /// Convert a type that implements [`AnyPin`] to a concrete [`Pin`]
    ///
    /// Even though there is a one-to-one mapping between `Pin<I, M>` and
    /// `AnyPin<Id = I, Mode = M>`, the compiler doesn't know that. This method
    /// provides a way to convert from an [`AnyPin`] to a [`Pin`]
    fn as_concrete(self) -> ConcretePin<Self>;

    /// Implement [`as_concrete`](AnyPin::as_concrete) for references
    fn as_concrete_ref(&self) -> &ConcretePin<Self>;

    /// Implement [`as_concrete`](AnyPin::as_concrete) for mutable references
    fn as_concrete_mut(&mut self) -> &mut ConcretePin<Self>;

    /// Convert the pin to the requested [`PinMode`]
    #[inline]
    fn into_mode<N: PinMode>(self) -> Pin<Self::Id, N> {
        self.as_concrete().into_mode()
    }

    /// Disable the pin and set it to float
    #[inline]
    fn into_floating_disabled(self) -> Pin<Self::Id, FloatingDisabled> {
        self.as_concrete().into_mode()
    }

    /// Disable the pin and set it to pull down
    #[inline]
    fn into_pull_down_disabled(self) -> Pin<Self::Id, PullDownDisabled> {
        self.as_concrete().into_mode()
    }

    /// Disable the pin and set it to pull up
    #[inline]
    fn into_pull_up_disabled(self) -> Pin<Self::Id, PullUpDisabled> {
        self.as_concrete().into_mode()
    }

    /// Configure the pin to operate as a floating input
    #[inline]
    fn into_floating_input(self) -> Pin<Self::Id, FloatingInput> {
        self.as_concrete().into_mode()
    }

    /// Configure the pin to operate as a pulled down input
    #[inline]
    fn into_pull_down_input(self) -> Pin<Self::Id, PullDownInput> {
        self.as_concrete().into_mode()
    }

    /// Configure the pin to operate as a pulled up input
    #[inline]
    fn into_pull_up_input(self) -> Pin<Self::Id, PullUpInput> {
        self.as_concrete().into_mode()
    }

    /// Configure the pin to operate as a push-pull output
    #[inline]
    fn into_push_pull_output(self) -> Pin<Self::Id, PushPullOutput> {
        self.as_concrete().into_mode()
    }

    /// Configure the pin to operate as a readable push pull output
    #[inline]
    fn into_readable_output(self) -> Pin<Self::Id, ReadableOutput> {
        self.as_concrete().into_mode()
    }

    /// Configure the pin to operate as the corresponding peripheral function.
    ///
    /// The type `C` indicates the desired peripheral function.
    #[inline]
    fn into_alternate<C: AlternateConfig>(self) -> Pin<Self::Id, Alternate<C>> {
        self.as_concrete().into_mode()
    }

    /// Read the current drive strength of the pin.
    ///
    /// The drive strength is reset to normal on every change in pin mode.
    #[inline]
    fn get_drive_strength(&self) {
        self.as_concrete_ref().get_drive_strength()
    }

    /// Set the drive strength for the pin.
    ///
    /// The drive strength is reset to normal on every change in pin mode.
    #[inline]
    fn set_drive_strength(&mut self, stronger: bool) {
        self.as_concrete_mut().set_drive_strength(stronger)
    }
}

impl<I, M> Sealed for Pin<I, M>
where
    I: PinId,
    M: PinMode,
{
}

impl<I, M> AnyPin for Pin<I, M>
where
    I: PinId,
    M: PinMode,
{
    type Id = I;
    type Mode = M;
    #[inline]
    fn as_concrete(self) -> ConcretePin<Self> {
        self
    }
    #[inline]
    fn as_concrete_ref(&self) -> &ConcretePin<Self> {
        self
    }
    #[inline]
    fn as_concrete_mut(&mut self) -> &mut ConcretePin<Self> {
        self
    }
}

//==============================================================================
//  Optional pins
//==============================================================================

/// Meta-type representing an optional [`Pin`].
///
/// This trait is implemented for every [`Pin`], as well as for
/// [`NoneT`](crate::typelevel::NoneT).
pub trait OptionalPin: Sealed {}
impl OptionalPin for NoneT {}
impl<P: AnyPin> OptionalPin for P {}

/// Meta-type representing a valid [`Pin`].
///
/// When used as a bound, this trait allows you to exclude
/// [`NoneT`](crate::typelevel::NoneT) and limit the type to valid [`Pin`]s.
/// [`AnyPin`] is a super trait to [`SomePin`], so all of its functionality is
/// still available.
pub trait SomePin: OptionalPin + AnyPin {}
impl<P: OptionalPin + AnyPin> SomePin for P {}

//==============================================================================
//  Pin reading & writing
//==============================================================================

/// Read the logic level of an input put
///
/// This function is always safe to use, because it only reads from the GROUP
/// register block.
#[inline]
pub(super) fn read_pin(group: *const GROUP, num: u8) -> bool {
    let mask: u32 = 1 << num;
    unsafe { (*group).in_.read().bits() & mask != 0 }
}

/// Write the logic level of an output pin
///
/// # Safety
///
/// This function is only safe to use with exclusive control of a pin, through
/// ownership or a mutable reference. It uses the GROUP pointer safely, because
/// it writes to the OUTSET and OUTCLR registers. These register are designed to
/// allow changing the OUT setting atomically, without affecting other pins.
#[inline]
pub(super) unsafe fn write_pin(group: *const GROUP, num: u8, bit: bool) {
    let mask: u32 = 1 << num;
    if bit {
        (*group).outset.write(|w| w.bits(mask));
    } else {
        (*group).outclr.write(|w| w.bits(mask));
    }
}

/// Toggle the logic level of an output pin
///
/// # Safety
///
/// This function is only safe to use with exclusive control of a pin, through
/// ownership or a mutable reference. It uses the GROUP pointer safely, because
/// it writes to the OUTTGL register. This register is designed to allow
/// changing the OUT setting atomically, without affecting other pins.
#[inline]
pub(super) unsafe fn toggle_pin(group: *const GROUP, num: u8) {
    let mask: u32 = 1 << num;
    (*group).outtgl.write(|w| w.bits(mask));
}

/// Read back the logic level of an output pin
///
/// This function is always safe to use, because it only reads from the GROUP
/// register block.
#[inline]
pub(super) fn read_out_pin(group: *const GROUP, num: u8) -> bool {
    let mask: u32 = 1 << num;
    unsafe { (*group).out.read().bits() & mask != 0 }
}

/// Read the drive strength of a pin
///
/// This function is always safe to use, because it only reads from the GROUP
/// register block.
#[inline]
pub(super) fn read_drive_strength(group: *const GROUP, num: u8) -> bool {
    // The SAMD11 and SAMD21 PACs don't have read methods for DRVSTR, so do it
    // manually
    unsafe { (*group).pincfg[num as usize].read().bits() & 0x40 == 0 }
}

/// Write the drive strength of a pin
///
/// # Safety
///
/// This function is only safe to use with exclusive control of a pin, through
/// ownership or a mutable reference. It does not use atomic operations to
/// modify the pin configuration, but it only writes to the PINCFG register,
/// which is not shared with any other pin. As long as we have exclusive control
/// of the pin, we can safely perform the read/modify/write.
#[inline]
pub(super) unsafe fn write_drive_strength(group: *const GROUP, num: u8, bit: bool) {
    (*group).pincfg[num as usize].modify(|_, w| w.drvstr().bit(bit));
}

//==============================================================================
//  Pin struct
//==============================================================================

/// A type-level GPIO pin, parameterized by [`PinId`] and [`PinMode`] types
pub struct Pin<I, M>
where
    I: PinId,
    M: PinMode,
{
    id: PhantomData<I>,
    mode: PhantomData<M>,
}

impl<I, M> Pin<I, M>
where
    I: PinId,
    M: PinMode,
{
    #[inline]
    pub(crate) fn new() -> Pin<I, M> {
        Pin {
            id: PhantomData,
            mode: PhantomData,
        }
    }

    /// Convert a [`Pin`] to a type that implements [`AnyPin`]
    ///
    /// Even though there is a one-to-one mapping between `Pin<I, M>` and
    /// `AnyPin<Id = I, Mode = M>`, the compiler doesn't know that. This method
    /// provides a way to convert from a [`Pin`] to an [`AnyPin`]. See the
    /// [`AnyPin`] trait for more details.
    #[inline]
    pub fn as_any<P>(self) -> P
    where
        P: AnyPin<Id = I, Mode = M>,
    {
        // SAFETY:
        // core::ptr::read makes a bitwise copy, regardless of whether the type
        // implements Copy. Thus, the returned value is a copy, and we still
        // need to dispose of self. Because a `Pin` does not contain any
        // resources or allocations, we can safely drop it, rather than use
        // core::mem::forget
        unsafe { core::ptr::read(&self as *const _ as *const P) }
    }

    /// Implement [`as_any`](Pin::as_any) for references
    #[inline]
    pub fn as_any_ref<P>(&self) -> &P
    where
        P: AnyPin<Id = I, Mode = M>,
    {
        // SAFETY:
        // P is guaranteed to be Pin<I, M>, so there is no actual transmute
        // operation. The correct lifetime is preserved, albiet elided
        unsafe { &*(self as *const _ as *const P) }
    }

    /// Implement [`as_any`](Pin::as_any) for mutable references
    #[inline]
    pub fn as_any_mut<P>(&mut self) -> &mut P
    where
        P: AnyPin<Id = I, Mode = M>,
    {
        // SAFETY:
        // P is guaranteed to be Pin<I, M>, so there is no actual transmute
        // operation. The correct lifetime is preserved, albiet elided
        unsafe { &mut *(self as *mut _ as *mut P) }
    }

    /// Convert the pin to the requested [`PinMode`]
    #[inline]
    pub fn into_mode<N: PinMode>(self) -> Pin<I, N> {
        N::into_mode(self)
    }

    /// Disable the pin and set it to float
    #[inline]
    pub fn into_floating_disabled(self) -> Pin<I, FloatingDisabled> {
        self.into_mode()
    }

    /// Disable the pin and set it to pull down
    #[inline]
    pub fn into_pull_down_disabled(self) -> Pin<I, PullDownDisabled> {
        self.into_mode()
    }

    /// Disable the pin and set it to pull up
    #[inline]
    pub fn into_pull_up_disabled(self) -> Pin<I, PullUpDisabled> {
        self.into_mode()
    }

    /// Configure the pin to operate as a floating input
    #[inline]
    pub fn into_floating_input(self) -> Pin<I, FloatingInput> {
        self.into_mode()
    }

    /// Configure the pin to operate as a pulled down input
    #[inline]
    pub fn into_pull_down_input(self) -> Pin<I, PullDownInput> {
        self.into_mode()
    }

    /// Configure the pin to operate as a pulled up input
    #[inline]
    pub fn into_pull_up_input(self) -> Pin<I, PullUpInput> {
        self.into_mode()
    }

    /// Configure the pin to operate as a push-pull output
    #[inline]
    pub fn into_push_pull_output(self) -> Pin<I, PushPullOutput> {
        self.into_mode()
    }

    /// Configure the pin to operate as a readable push pull output
    #[inline]
    pub fn into_readable_output(self) -> Pin<I, ReadableOutput> {
        self.into_mode()
    }

    /// Configure the pin to operate as the corresponding peripheral function.
    ///
    /// The type `C` indicates the desired peripheral function.
    #[inline]
    pub fn into_alternate<C: AlternateConfig>(self) -> Pin<I, Alternate<C>> {
        self.into_mode()
    }

    /// Read the current drive strength of the pin.
    ///
    /// The drive strength is reset to normal on every change in pin mode.
    #[inline]
    pub fn get_drive_strength(&self) {
        read_drive_strength(I::Group::group(), I::NUM);
    }

    /// Set the drive strength for the pin.
    ///
    /// The drive strength is reset to normal on every change in pin mode.
    #[inline]
    pub fn set_drive_strength(&mut self, stronger: bool) {
        // SAFETY: We have exclusive control of the pin, so this is safe to use.
        unsafe { write_drive_strength(I::Group::group(), I::NUM, stronger) };
    }

    #[inline]
    fn _read(&self) -> bool {
        read_pin(I::Group::group(), I::NUM)
    }
    #[inline]
    fn _write(&mut self, bit: bool) {
        // SAFETY: We have exclusive control of the pin, so this is safe to use.
        unsafe { write_pin(I::Group::group(), I::NUM, bit) }
    }
    #[inline]
    pub(crate) fn _toggle(&mut self) {
        // SAFETY: We have exclusive control of the pin, so this is safe to use.
        unsafe { toggle_pin(I::Group::group(), I::NUM) }
    }
    #[inline]
    fn _read_out(&self) -> bool {
        read_out_pin(I::Group::group(), I::NUM)
    }
    #[inline]
    pub(crate) fn _is_low(&self) -> bool {
        self._read() == false
    }
    #[inline]
    pub(crate) fn _is_high(&self) -> bool {
        self._read() == true
    }
    #[inline]
    pub(crate) fn _set_low(&mut self) {
        self._write(false);
    }
    #[inline]
    pub(crate) fn _set_high(&mut self) {
        self._write(true);
    }
    #[inline]
    pub(crate) fn _is_set_low(&self) -> bool {
        self._read_out() == false
    }
    #[inline]
    pub(crate) fn _is_set_high(&self) -> bool {
        self._read_out() == true
    }
}

//==============================================================================
//  Embedded HAL traits
//==============================================================================

impl<I, C> OutputPin for Pin<I, Output<C>>
where
    I: PinId,
    C: OutputConfig,
{
    type Error = Infallible;
    #[inline]
    fn set_high(&mut self) -> Result<(), Self::Error> {
        self._set_high();
        Ok(())
    }
    #[inline]
    fn set_low(&mut self) -> Result<(), Self::Error> {
        self._set_low();
        Ok(())
    }
}

#[cfg(feature = "unproven")]
impl<I> InputPin for Pin<I, ReadableOutput>
where
    I: PinId,
{
    type Error = Infallible;
    #[inline]
    fn is_high(&self) -> Result<bool, Self::Error> {
        Ok(self._is_high())
    }
    #[inline]
    fn is_low(&self) -> Result<bool, Self::Error> {
        Ok(self._is_low())
    }
}

#[cfg(feature = "unproven")]
impl<I, C> InputPin for Pin<I, Input<C>>
where
    I: PinId,
    C: InputConfig,
{
    type Error = Infallible;
    #[inline]
    fn is_high(&self) -> Result<bool, Self::Error> {
        Ok(self._is_high())
    }
    #[inline]
    fn is_low(&self) -> Result<bool, Self::Error> {
        Ok(self._is_low())
    }
}

#[cfg(feature = "unproven")]
impl<I, C> ToggleableOutputPin for Pin<I, Output<C>>
where
    I: PinId,
    C: OutputConfig,
{
    type Error = Infallible;
    #[inline]
    fn toggle(&mut self) -> Result<(), Self::Error> {
        self._toggle();
        Ok(())
    }
}

#[cfg(feature = "unproven")]
impl<I, C> StatefulOutputPin for Pin<I, Output<C>>
where
    I: PinId,
    C: OutputConfig,
{
    #[inline]
    fn is_set_high(&self) -> Result<bool, Self::Error> {
        Ok(self._is_set_high())
    }
    #[inline]
    fn is_set_low(&self) -> Result<bool, Self::Error> {
        Ok(self._is_set_low())
    }
}

//==============================================================================
//  Pin definitions
//==============================================================================

macro_rules! pins{
    (
        $(
            $( #[$cfg:meta] )?
            $Id:ident,
        )+
    ) => {
        paste! {
            /// Collection of all the individual [`Pin`]s
            pub struct Pins {
                port: Option<PORT>,
                $(
                    #[doc = "Pin " $Id]
                    $( #[$cfg] )?
                    pub [<$Id:lower>]: Pin<$Id, Reset>,
                )+
            }
            impl Pins {
                /// Take ownership of the PAC
                /// [`PORT`](crate::target_device::PORT) and split it into
                /// discrete [`Pin`]s
                pub fn new(port: PORT) -> Pins {
                    Pins {
                        port: Some(port),
                        $(
                            $( #[$cfg] )?
                            [<$Id:lower>]: Pin::new(),
                        )+
                    }
                }
                /// Take the PAC [`PORT`]
                ///
                /// The [`PORT`] can only be taken once. Subsequent calls to
                /// this function will panic.
                ///
                /// # Safety
                ///
                /// Direct access to the [`PORT`] could allow you to invalidate
                /// the compiler's type-level tracking, so it is unsafe.
                ///
                /// [`PORT`](crate::target_device::PORT)
                #[inline]
                pub unsafe fn port(&mut self) -> PORT {
                    self.port.take().unwrap()
                }
            }
        }
    };
}

macro_rules! define_pins {
    (
        $(
            $Group:ident {
                $(
                    $( #[$cfg:meta] )?
                    ($Id:ident, $NUM:literal),
                )+
            }
        )+
    ) => {
        $(
            $(
                $( #[$cfg] )?
                pin_id!($Group, $Id, $NUM);
            )+
        )+
        pins!(
            $(
                $(
                    $( #[$cfg] )?
                    $Id,
                )+
            )+
        );
    };
}

define_pins!(
    A {
        #[cfg(not(feature = "samd11"))]
        (PA00, 00),
        #[cfg(not(feature = "samd11"))]
        (PA01, 01),
        (PA02, 02),
        #[cfg(not(feature = "samd11"))]
        (PA03, 03),
        (PA04, 04),
        (PA05, 05),
        #[cfg(not(feature = "samd11"))]
        (PA06, 06),
        #[cfg(not(feature = "samd11"))]
        (PA07, 07),
        (PA08, 08),
        (PA09, 09),
        #[cfg(not(feature = "samd11"))]
        (PA10, 10),
        #[cfg(not(feature = "samd11"))]
        (PA11, 11),
        #[cfg(any(feature = "min-samd21g", feature = "min-samd51g"))]
        (PA12, 12),
        #[cfg(any(feature = "min-samd21g", feature = "min-samd51g"))]
        (PA13, 13),
        (PA14, 14),
        (PA15, 15),
        #[cfg(not(feature = "samd11"))]
        (PA16, 16),
        #[cfg(not(feature = "samd11"))]
        (PA17, 17),
        #[cfg(not(feature = "samd11"))]
        (PA18, 18),
        #[cfg(not(feature = "samd11"))]
        (PA19, 19),
        #[cfg(any(feature = "min-samd21g", feature = "min-samd51g"))]
        (PA20, 20),
        #[cfg(any(feature = "min-samd21g", feature = "min-samd51g"))]
        (PA21, 21),
        #[cfg(not(feature = "samd11"))]
        (PA22, 22),
        #[cfg(not(feature = "samd11"))]
        (PA23, 23),
        (PA24, 24),
        (PA25, 25),
        #[cfg(not(feature = "samd11"))]
        (PA27, 27),
        #[cfg(any(feature = "samd11", feature = "samd21"))]
        (PA28, 28),
        (PA30, 30),
        (PA31, 31),
    }
    B {
        #[cfg(any(feature = "min-samd21j", feature = "min-samd51j"))]
        (PB00, 00),
        #[cfg(any(feature = "min-samd21j", feature = "min-samd51j"))]
        (PB01, 01),
        #[cfg(any(feature = "min-samd21g", feature = "min-samd51g"))]
        (PB02, 02),
        #[cfg(any(feature = "min-samd21g", feature = "min-samd51g"))]
        (PB03, 03),
        #[cfg(any(feature = "min-samd21j", feature = "min-samd51j"))]
        (PB04, 04),
        #[cfg(any(feature = "min-samd21j", feature = "min-samd51j"))]
        (PB05, 05),
        #[cfg(any(feature = "min-samd21j", feature = "min-samd51j"))]
        (PB06, 06),
        #[cfg(any(feature = "min-samd21j", feature = "min-samd51j"))]
        (PB07, 07),
        #[cfg(any(feature = "min-samd21g", feature = "min-samd51g"))]
        (PB08, 08),
        #[cfg(any(feature = "min-samd21g", feature = "min-samd51g"))]
        (PB09, 09),
        #[cfg(any(feature = "min-samd21g", feature = "min-samd51g"))]
        (PB10, 10),
        #[cfg(any(feature = "min-samd21g", feature = "min-samd51g"))]
        (PB11, 11),
        #[cfg(any(feature = "min-samd21j", feature = "min-samd51j"))]
        (PB12, 12),
        #[cfg(any(feature = "min-samd21j", feature = "min-samd51j"))]
        (PB13, 13),
        #[cfg(any(feature = "min-samd21j", feature = "min-samd51j"))]
        (PB14, 14),
        #[cfg(any(feature = "min-samd21j", feature = "min-samd51j"))]
        (PB15, 15),
        #[cfg(any(feature = "min-samd21j", feature = "min-samd51j"))]
        (PB16, 16),
        #[cfg(any(feature = "min-samd21j", feature = "min-samd51j"))]
        (PB17, 17),
        #[cfg(feature = "min-samd51n")]
        (PB18, 18),
        #[cfg(feature = "min-samd51n")]
        (PB19, 19),
        #[cfg(feature = "min-samd51n")]
        (PB20, 20),
        #[cfg(feature = "min-samd51n")]
        (PB21, 21),
        #[cfg(any(feature = "min-samd21g", feature = "min-samd51g"))]
        (PB22, 22),
        #[cfg(any(feature = "min-samd21g", feature = "min-samd51g"))]
        (PB23, 23),
        #[cfg(feature = "min-samd51n")]
        (PB24, 24),
        #[cfg(feature = "min-samd51n")]
        (PB25, 25),
        #[cfg(feature = "min-samd51p")]
        (PB26, 26),
        #[cfg(feature = "min-samd51p")]
        (PB27, 27),
        #[cfg(feature = "min-samd51p")]
        (PB28, 28),
        #[cfg(feature = "min-samd51p")]
        (PB29, 29),
        #[cfg(any(feature = "min-samd21j", feature = "min-samd51j"))]
        (PB30, 30),
        #[cfg(any(feature = "min-samd21j", feature = "min-samd51j"))]
        (PB31, 31),
    }
    C {
        #[cfg(feature = "min-samd51n")]
        (PC00, 00),
        #[cfg(feature = "min-samd51n")]
        (PC01, 01),
        #[cfg(feature = "min-samd51n")]
        (PC02, 02),
        #[cfg(feature = "min-samd51n")]
        (PC03, 03),
        #[cfg(feature = "min-samd51p")]
        (PC04, 04),
        #[cfg(feature = "min-samd51n")]
        (PC05, 05),
        #[cfg(feature = "min-samd51n")]
        (PC06, 06),
        #[cfg(feature = "min-samd51n")]
        (PC07, 07),
        #[cfg(feature = "min-samd51n")]
        (PC10, 10),
        #[cfg(feature = "min-samd51n")]
        (PC11, 11),
        #[cfg(feature = "min-samd51n")]
        (PC12, 12),
        #[cfg(feature = "min-samd51n")]
        (PC13, 13),
        #[cfg(feature = "min-samd51n")]
        (PC14, 14),
        #[cfg(feature = "min-samd51n")]
        (PC15, 15),
        #[cfg(feature = "min-samd51n")]
        (PC16, 16),
        #[cfg(feature = "min-samd51n")]
        (PC17, 17),
        #[cfg(feature = "min-samd51n")]
        (PC18, 18),
        #[cfg(feature = "min-samd51n")]
        (PC19, 19),
        #[cfg(feature = "min-samd51n")]
        (PC20, 20),
        #[cfg(feature = "min-samd51n")]
        (PC21, 21),
        #[cfg(feature = "min-samd51p")]
        (PC22, 22),
        #[cfg(feature = "min-samd51p")]
        (PC23, 23),
        #[cfg(feature = "min-samd51n")]
        (PC24, 24),
        #[cfg(feature = "min-samd51n")]
        (PC25, 25),
        #[cfg(feature = "min-samd51n")]
        (PC26, 26),
        #[cfg(feature = "min-samd51n")]
        (PC27, 27),
        #[cfg(feature = "min-samd51n")]
        (PC28, 28),
        #[cfg(feature = "min-samd51p")]
        (PC30, 30),
        #[cfg(feature = "min-samd51p")]
        (PC31, 31),
    }
    D {
        #[cfg(feature = "min-samd51p")]
        (PD00, 00),
        #[cfg(feature = "min-samd51p")]
        (PD01, 01),
        #[cfg(feature = "min-samd51p")]
        (PD08, 08),
        #[cfg(feature = "min-samd51p")]
        (PD09, 09),
        #[cfg(feature = "min-samd51p")]
        (PD10, 10),
        #[cfg(feature = "min-samd51p")]
        (PD11, 11),
        #[cfg(feature = "min-samd51p")]
        (PD12, 12),
        #[cfg(feature = "min-samd51p")]
        (PD20, 20),
        #[cfg(feature = "min-samd51p")]
        (PD21, 21),
    }
);

/// Helper macro to give meaningful names to GPIO pins
///
/// The normal [`Pins`] struct names each [`Pin`] according to its [`PinId`].
/// However, BSP authors would prefer to name each [`Pin`] according to its
/// function. This macro defines a new `Pins` struct with custom field names
/// for each [`Pin`], and it defines type aliases and constants to make it
/// easier to work with the [`Pin`]s and [`DynPin`](super::DynPin)s.
///
/// The types and constants are defined within a public module named
/// `bsp_pins_mod` and are then re-exported with `pub use bsp_pins_mod::*;`.
///
/// # Example
///
/// The following example macro call
///
/// ```rust
/// atsamd_hal::bsp_pins!(
///     #[cfg(feature = "unproven")]
///     PA24 {
///         name: led_pass,
///         aliases: {
///             AlternateH: LedPass,
///             #[cfg(feature = "usb")]
///             AlternateM: UsbPin
///         }
///     }
/// );
/// ```
///
/// would expand to this
///
/// ```rust
/// pub mod bsp_pins_mod {
///     use atsamd_hal::target_device::PORT;
///     use atsamd_hal::gpio::v2::{
///         self as gpio, Pin, PinId, PinMode, Reset, DynPinId, DynPinMode
///     };
///
///     pub struct Pins {
///         port: Option<PORT>,
///         #[cfg(feature = "unproven")]
///         pub led_pass: Pin<gpio::PA24, Reset>,
///     }
///
///     impl Pins {
///
///         pub fn new(port: PORT) -> Self {
///             let pins = gpio::Pins::new(port);
///             Self {
///                 port: Some(unsafe { pins.port() }),
///                 #[cfg(feature = "unproven")]
///                 led_pass: pins.pa24,
///             }
///         }
///
///         #[inline]
///         pub unsafe fn port(&mut self) -> PORT {
///             self.port.take().unwrap()
///         }
///     }
///
///     #[cfg(feature = "unproven")]
///     pub type LedPass = Pin<gpio::PA24, gpio::AlternateH>;
///     #[cfg(feature = "unproven")]
///     pub const LED_PASS_ID: DynPinId = <gpio::PA24 as PinId>::DYN;
///     #[cfg(feature = "unproven")]
///     pub const LED_PASS_MODE: DynPinMode = <gpio::AlternateH as PinMode>::DYN;
///
///     #[cfg(feature = "unproven")]
///     #[cfg(feature = "usb")]
///     pub type UsbPin = Pin<gpio::PA24, gpio::AlternateM>;
///     #[cfg(feature = "unproven")]
///     #[cfg(feature = "usb")]
///     pub const USB_PIN_ID: DynPinId = <gpio::PA24 as PinId>::DYN;
///     #[cfg(feature = "unproven")]
///     #[cfg(feature = "usb")]
///     pub const USB_PIN_MODE: DynPinMode = <gpio::AlternateM as PinMode>::DYN;
/// }
/// pub use bsp_pins_mod::*;
/// ```
#[macro_export]
macro_rules! bsp_pins {
    (
        $(
            $( #[$name_cfg:meta] )?
            $Id:ty {
                name: $name:ident $(,)?
                $(
                    aliases: {
                        $(
                            $( #[$alias_cfg:meta] )?
                            $Mode:ty: $Alias:ident
                        ),+
                    }
                )?
            }
        )+
    ) => {
        atsamd_hal::paste::paste! {
            pub mod bsp_pins_mod {
                use atsamd_hal::target_device::PORT;
                use atsamd_hal::gpio::v2::{
                    self as gpio, Pin, PinId, PinMode, Reset, DynPinId, DynPinMode
                };

                /// BSP replacement for the HAL
                /// [`Pins`](atsamd_hal::gpio::v2::Pins) type
                ///
                /// This type is intended to provide more meaningful names for
                /// the given pins.
                pub struct Pins {
                    port: Option<PORT>,
                    $(
                        $( #[$name_cfg] )?
                        pub $name: Pin<gpio::$Id, Reset>,
                    )+
                }

                impl Pins {

                    /// Take ownership of the PAC [`PORT`] and split it into
                    /// discrete [`Pin`]s.
                    ///
                    /// This struct serves as a replacement for the HAL [`Pins`]
                    /// struct. It is intended to provide more meaningful names
                    /// for each [`Pin`] in a BSP. Any [`Pin`] not defined by
                    /// the BSP is dropped.
                    ///
                    /// [`PORT`](atsamd_hal::target_device::PORT)
                    /// [`Pin`](atsamd_hal::gpio::v2::Pin)
                    /// [`Pins`](atsamd_hal::gpio::v2::Pins)
                    pub fn new(port: PORT) -> Self {
                        let mut pins = gpio::Pins::new(port);
                        Self {
                            port: Some(unsafe{ pins.port() }),
                            $(
                                $( #[$name_cfg] )?
                                $name: pins.[<$Id:lower>],
                            )+
                        }
                    }

                    /// Take the PAC [`PORT`]
                    ///
                    /// The [`PORT`] can only be taken once. Subsequent calls to
                    /// this function will panic.
                    ///
                    /// # Safety
                    ///
                    /// Direct access to the [`PORT`] could allow you to invalidate
                    /// the compiler's type-level tracking, so it is unsafe.
                    ///
                    /// [`PORT`](atsamd_hal::target_device::PORT)
                    #[inline]
                    pub unsafe fn port(&mut self) -> PORT {
                        self.port.take().unwrap()
                    }
                }

                $(
                    $( #[$name_cfg] )?
                    $(
                        $(
                            $( #[$alias_cfg] )?
                            /// Alias for a configured [`Pin`](atsamd_hal::gpio::v2::Pin)
                            pub type $Alias = Pin<gpio::$Id, gpio::$Mode>;
                        )+
                    )?
                    $( #[$name_cfg] )?
                    $(
                        $(
                            $( #[$alias_cfg] )?
                            #[doc = "[DynPinId](atsamd_hal::gpio::v2::DynPinId) for the `" $Alias "` alias."]
                            pub const [<$Alias:snake:upper _ID>]: DynPinId = <gpio::$Id as PinId>::DYN;
                        )+
                    )?
                    $( #[$name_cfg] )?
                    $(
                        $(
                            $( #[$alias_cfg] )?
                            #[doc = "[DynPinMode](atsamd_hal::gpio::v2::DynPinMode) for the `" $Alias "` alias."]
                            pub const [<$Alias:snake:upper _MODE>]: DynPinMode = <gpio::$Mode as PinMode>::DYN;
                        )+
                    )?
                )+
            }
            pub use bsp_pins_mod::*;
        }
    };
}
