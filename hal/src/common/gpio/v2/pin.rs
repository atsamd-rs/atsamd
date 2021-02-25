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
//! ownership of the [`PORT`] and provides the corresponding pins. Each [`Pin`]
//! within the [`Pins`] struct can be moved out and used individually.
//!
//!
//! ```
//! let mut peripherals = Peripherals::take().unwrap();
//! let pins = Pins::new(peripherals.PORT);
//! ```
//!
//! Pins can be converted between modes using several different methods.
//!
//! ```
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
//! in the corresponding [`PinMode`]s, namely: [`InputPin`], [`OutputPin`],
//! [`ToggleableOutputPin`] and [`StatefulOutputPin`].
//!
//! # Type-level encapsulation
//!
//! Normally, storing a generic pin within some data structure requires two type
//! parameters.
//!
//! ```
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
//! ```
//! struct UserStruct<P: AnyPin> {
//!     pin: P
//! }
//! ```
//!
//! Moreover, no information is lost with this approach, because the [`AnyPin`]
//! trait has associated types for each type parameter of [`Pin`]. Use these
//! associated types to apply trait bounds or restrict the pin in some way.
//!
//! ```
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
//! a bit different than working directly with a [`Pin`]. See the [`AnyPin`]
//! documentation for more details.
//!
//! # Optional pins
//!
//! Finally, this module provides an easy way to implement optional pins. The
//! trait [`OptionalPin`] is implemented for each [`Pin`] as well as the
//! [`NoneT`] struct. [`NoneT`] acts as a type-level version of the [`None`]
//! variant. The [`SomePin`] trait has both [`OptionalPin`] and [`AnyPin`] as
//! super traits, so it can be used as a bound to guarantee a valid pin and
//! provide access to the [`AnyPin`] associated types.
//!
//! ```
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

#![allow(clippy::zero_prefixed_literal)]

use core::convert::Infallible;
use core::marker::PhantomData;
use core::mem::transmute;

use hal::digital::v2::OutputPin;
#[cfg(feature = "unproven")]
use hal::digital::v2::{InputPin, StatefulOutputPin, ToggleableOutputPin};
use paste::paste;

use crate::target_device::PORT;

use crate::typelevel::{NoneT, Sealed};

use super::dynpin::*;
use super::reg::RegisterInterface;

//==============================================================================
//  Disabled configurations
//==============================================================================

/// Type-level `enum` for disabled configurations
pub trait DisabledConfig: Sealed {
    /// Corresponding [`DynDisabled`](super::DynDisabled)
    const DYN: DynDisabled;
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
}
impl DisabledConfig for PullDown {
    const DYN: DynDisabled = DynDisabled::PullDown;
}
impl DisabledConfig for PullUp {
    const DYN: DynDisabled = DynDisabled::PullUp;
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
}

impl InputConfig for Floating {
    const DYN: DynInput = DynInput::Floating;
}
impl InputConfig for PullDown {
    const DYN: DynInput = DynInput::PullDown;
}
impl InputConfig for PullUp {
    const DYN: DynInput = DynInput::PullUp;
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
}
impl OutputConfig for Readable {
    const DYN: DynOutput = DynOutput::Readable;
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
}

macro_rules! alternate {
    (
        $(
            $Letter:ident
        ),+
    ) => {
        paste! {
            $(
                #[
                    doc = "Type-level variant of [`AlternateConfig`] for \
                    alternate peripheral function " $Letter
                ]
                pub enum $Letter {}
                impl Sealed for $Letter {}
                impl AlternateConfig for $Letter {
                    const DYN: DynAlternate = DynAlternate::$Letter;
                }
                #[
                    doc = "Type-level variant of [`PinMode`] for alternate \
                    peripheral function [`" $Letter "`]"
                ]
                pub type [<Alternate $Letter>] = Alternate<$Letter>;
            )+
        }
    };
}

alternate!(A, B, C, D, E, F, G);

#[cfg(any(feature = "samd21", feature = "min-samd51g"))]
alternate!(H);

#[cfg(feature = "min-samd51g")]
alternate!(I, J, K, L, M, N);

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
}

impl<C: DisabledConfig> PinMode for Disabled<C> {
    const DYN: DynPinMode = DynPinMode::Disabled(C::DYN);
}

impl<C: InputConfig> PinMode for Input<C> {
    const DYN: DynPinMode = DynPinMode::Input(C::DYN);
}

impl<C: OutputConfig> PinMode for Output<C> {
    const DYN: DynPinMode = DynPinMode::Output(C::DYN);
}

impl<C: AlternateConfig> PinMode for Alternate<C> {
    const DYN: DynPinMode = DynPinMode::Alternate(C::DYN);
}

//==============================================================================
//  Pin IDs
//==============================================================================

/// Type-level `enum` for pin IDs
pub trait PinId: Sealed {
    /// Corresponding [`DynPinId`](super::DynPinId)
    const DYN: DynPinId;
}

macro_rules! pin_id {
    ($Group:ident, $Id:ident, $NUM:literal) => {
        paste! {
            #[doc = "Pin ID representing pin " $Id]
            pub enum $Id {}
            impl Sealed for $Id {}
            impl PinId for $Id {
                const DYN: DynPinId = DynPinId {
                    group: DynGroup::$Group,
                    num: $NUM,
                };
            }
        }
    };
}

//==============================================================================
//  Registers
//==============================================================================

/// Provide a safe register interface for [`Pin`]s
///
/// This `struct` takes ownership of a [`PinId`] and provides an API to
/// access the corresponding regsiters.
struct Registers<I: PinId> {
    id: PhantomData<I>,
}

// [`Registers`] takes ownership of the [`PinId`], and [`Pin`] guarantees that
// each pin is a singleton, so this implementation is safe.
unsafe impl<I: PinId> RegisterInterface for Registers<I> {
    #[inline]
    fn id(&self) -> DynPinId {
        I::DYN
    }
}

impl<I: PinId> Registers<I> {
    /// Create a new instance of [`Registers`]
    ///
    /// # Safety
    ///
    /// Users must never create two simultaneous instances of this `struct` with
    /// the same [`PinId`]
    #[inline]
    unsafe fn new() -> Self {
        Registers { id: PhantomData }
    }

    /// Provide a type-level equivalent for the
    /// [`RegisterInterface::change_mode`] method.
    #[inline]
    fn change_mode<M: PinMode>(&mut self) {
        RegisterInterface::change_mode(self, M::DYN);
    }
}

//==============================================================================
//  Pin
//==============================================================================

/// A type-level GPIO pin, parameterized by [`PinId`] and [`PinMode`] types
pub struct Pin<I, M>
where
    I: PinId,
    M: PinMode,
{
    regs: Registers<I>,
    mode: PhantomData<M>,
}

impl<I, M> Pin<I, M>
where
    I: PinId,
    M: PinMode,
{
    /// Create a new [`Pin`]
    ///
    /// # Safety
    ///
    /// Each [`Pin`] must be a singleton. For a given [`PinId`], there must be
    /// at most one corresponding [`Pin`] in existence at any given time.
    /// Violating this requirement is `unsafe`.
    #[inline]
    pub(crate) unsafe fn new() -> Pin<I, M> {
        Pin {
            regs: Registers::new(),
            mode: PhantomData,
        }
    }

    /// Convert the pin to the requested [`PinMode`]
    #[inline]
    pub fn into_mode<N: PinMode>(mut self) -> Pin<I, N> {
        self.regs.change_mode::<N>();
        // Safe because we drop the existing Pin
        unsafe { Pin::new() }
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
    pub fn get_drive_strength(&self) -> bool {
        self.regs.read_drive_strength()
    }

    /// Set the drive strength for the pin.
    ///
    /// The drive strength is reset to normal on every change in pin mode.
    #[inline]
    pub fn set_drive_strength(&mut self, stronger: bool) {
        self.regs.write_drive_strength(stronger);
    }

    #[inline]
    pub(crate) fn _is_low(&self) -> bool {
        self.regs.read_pin() == false
    }

    #[inline]
    pub(crate) fn _is_high(&self) -> bool {
        self.regs.read_pin() == true
    }

    #[inline]
    pub(crate) fn _set_low(&mut self) {
        self.regs.write_pin(false);
    }

    #[inline]
    pub(crate) fn _set_high(&mut self) {
        self.regs.write_pin(true);
    }

    #[inline]
    pub(crate) fn _toggle(&mut self) {
        self.regs.toggle_pin();
    }

    #[inline]
    pub(crate) fn _is_set_low(&self) -> bool {
        self.regs.read_out_pin() == false
    }

    #[inline]
    pub(crate) fn _is_set_high(&self) -> bool {
        self.regs.read_out_pin() == true
    }
}

//==============================================================================
//  PinMode conversions
//==============================================================================

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
//  AnyPin
//==============================================================================

/// Meta-type representing any [`Pin`]
///
/// All instances of [`Pin`] implement this trait. When used as a trait bound,
/// it acts to encapsulate a [`Pin`]. Without this trait, a completely generic
/// [`Pin`] requires two type parameters, i.e. `Pin<I, M>`. But when using this
/// trait, only one type parameter is required, i.e. `P: AnyPin`. However, even
/// though we have dropped a type parameter, no information is lost, because the
/// [`PinId`] and [`PinMode`] type parameters are stored as associated types in
/// the trait. The implementation of [`AnyPin`] looks like this:
///
/// ```
/// impl<I: PinId, M: PinMode> AnyPin for Pin<I, M> {
///     type Id = I;
///     type Mode = M;
/// }
/// ```
///
/// Thus, there is a one-to-one mapping between `Pin<I, M>` and
/// `AnyPin<Id = I, Mode = M>`, so you can always recover the specific type from
/// an implementation of [`AnyPin`]. The type alias [`SpecificPin`] is provided
/// for this purpose. You can convert between [`AnyPin`] and its corresponding
/// [`SpecificPin`] using the [`Into`], [`AsRef`] and [`AsMut`] traits.
///
/// ```
/// fn example<P: AnyPin>(mut any_pin: P) {
///     let pin_mut: &mut SpecificPin<P> = any_pin.as_mut();
///     let pin_ref: &SpecificPin<P> = any_pin.as_ref();
///     let pin: SpecificPin<P> = any_pin.into();
/// }
/// ```
///
/// ## [`AnyPin`] as a trait bound
///
/// When using [`AnyPin`] as a trait bound, you can constrain the associated
/// types to restrict the acceptable [`Pin`]s. For example, you could restrict
/// a function to accept a particular pin in any mode.
///
/// ```
/// fn example<P>(pin: P)
/// where
///     P: AnyPin<Id = PA27>
/// {
/// }
/// ```
///
/// Or you could accept any pin, as long as it's in the desired mode.
///
/// ```
/// fn example<P>(pin: P)
/// where
///     P: AnyPin<Mode = PullDownInput>
/// {
/// }
/// ```
///
/// You can also apply more complex bounds. In the following example, `P` must
/// be an output pin, and its [`PinId`] must satisfy some `UserTrait`.
///
/// ```
/// fn example<P, C>(pin: P)
/// where
///     P: AnyPin<Mode = Output<C>>,
///     C: OutputConfig,
///     P::Id: UserTrait,
/// {
/// }
/// ```
///
/// ## Generic [`AnyPin`]s
///
/// Working with a generic type constrained by [`AnyPin`] is slightly different
/// than working with a [`Pin`] directly. When compiling a generic function, the
/// compiler cannot assume anything about the specific type. It can only use
/// what it knows about the [`AnyPin`] trait. To use a generic [`AnyPin`], you
/// must first convert it to its corresponding [`SpecificPin`] using the
/// [`Into`], [`AsRef`] or [`AsMut`] trait. In some instances, you may also need
/// to convert back.
///
/// The following example walks through a few different ways to interact with a
/// generic type constrained by [`AnyPin`]. Suppose you wanted to store a
/// completely generic [`Pin`] within a struct. You can do so using only one
/// type parameter and the [`AnyPin`] trait.
///
/// ```
/// pub struct Example<P: AnyPin> {
///     pin: P,
/// }
/// ```
///
/// Next, suppose you want to create a method that will take the [`Pin`] out of
/// the struct, perform some operations in different [`PinMode`]s, and put it
/// back into the struct before returning. The `elided` method below shows such
/// an example. However, it can be a bit tricky to follow all of the type
/// conversions here. For clarity, the `expanded` method shows the same behavior
/// with each transformation given its proper type annotation.
///
/// Notice that it is not enough to simply put back the correct [`SpecificPin`].
/// Even though the [`SpecificPin`] implements
/// `AnyPin<Id = P::Id, Mode = P::Mode>` the compiler doesn't understand that
/// `SpecificPin<P> == P` for all `P`. As far as the compiler is concerned,
/// there could be several different types that implement
/// `AnyPin<Id = P::Id, Mode = P::Mode>`. Instead, the compiler requires that
/// you put back an instance of `P` exactly. The final use of [`Into`] is key
/// here. It transforms the [`SpecificPin`] back into `P` itself.
///
/// ```
/// impl<P: AnyPin> Example<P> {
///     pub fn elided(mut self) -> Self {
///         let pin = self.pin.into();
///         let mut pin = pin.into_push_pull_output();
///         pin.set_high().ok();
///         let pin = pin.into_floating_input();
///         let _bit = pin.is_low().unwrap();
///         let pin = pin.into_mode();
///         self.pin = pin.into();
///         self
///     }
///     pub fn expanded(mut self) -> Self {
///         let pin: SpecificPin<P> = self.pin.into();
///         let mut pin: Pin<P::Id, PushPullOutput> = pin.into_push_pull_output();
///         pin.set_high().ok();
///         let pin: Pin<P::Id, FloatingInput> = pin.into_floating_input();
///         let _bit = pin.is_low().unwrap();
///         let pin: SpecificPin<P> = pin.into_mode::<P::Mode>();
///         self.pin = pin.into();
///         self
///     }
/// }
/// ```
///
/// ## `v1` Compatibility
///
/// Normally, this trait would use Is<Type = SpecificPin<Self>> as a super
/// trait. But doing so would restrict implementations to only the `v2` `Pin`
/// type in this module. To aid in backwards compatibility, we want to implement
/// `AnyPin` for the `v1` `Pin` type as well. This is possible for a few
/// reasons. First, both structs are zero-sized, so there is no meaningful
/// memory layout to begin with. And even if there were, the `v1` `Pin` type is
/// a newtype wrapper around a `v2` `Pin`, and single-field structs are
/// guaranteed to have the same layout as the field, even for `repr(Rust)`.
pub trait AnyPin
where
    Self: Sealed,
    Self: From<SpecificPin<Self>>,
    Self: Into<SpecificPin<Self>>,
    Self: AsRef<SpecificPin<Self>>,
    Self: AsMut<SpecificPin<Self>>,
    SpecificPin<Self>: AsRef<Self>,
    SpecificPin<Self>: AsMut<Self>,
{
    /// [`PinId`] of the corresponding [`Pin`]
    type Id: PinId;
    /// [`PinMode`] of the corresponding [`Pin`]
    type Mode: PinMode;
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
}

/// Type alias to recover the specific [`Pin`] type from an implementation of
/// [`AnyPin`]
///
/// By definition, `P == SpecificPin<P>` for all `P: AnyPin`.
pub type SpecificPin<P> = Pin<<P as AnyPin>::Id, <P as AnyPin>::Mode>;

/// Implementation required to satisfy the `Is<Type = SpecificPin<Self>>` bound
/// on [`AnyPin`]
impl<P: AnyPin> AsRef<P> for SpecificPin<P> {
    #[inline]
    fn as_ref(&self) -> &P {
        // SAFETY: This is guaranteed to be safe, because P == SpecificPin<P>
        // Transmuting between `v1` and `v2` `Pin` types is also safe, because
        // both are zero-sized, and single-field, newtype structs are guaranteed
        // to have the same layout as the field anyway, even for repr(Rust).
        unsafe { transmute(self) }
    }
}

/// Implementation required to satisfy the `Is<Type = SpecificPin<Self>>` bound
/// on [`AnyPin`]
impl<P: AnyPin> AsMut<P> for SpecificPin<P> {
    #[inline]
    fn as_mut(&mut self) -> &mut P {
        // SAFETY: This is guaranteed to be safe, because P == SpecificPin<P>
        // Transmuting between `v1` and `v2` `Pin` types is also safe, because
        // both are zero-sized, and single-field, newtype structs are guaranteed
        // to have the same layout as the field anyway, even for repr(Rust).
        unsafe { transmute(self) }
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
                #[inline]
                pub fn new(port: PORT) -> Pins {
                    Pins {
                        port: Some(port),
                        // Safe because we only create one `Pin` per `PinId`
                        $(
                            $( #[$cfg] )?
                            [<$Id:lower>]: unsafe { Pin::new() },
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

macro_rules! declare_pins {
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

declare_pins!(
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

//==============================================================================
//  bsp_pins
//==============================================================================

/// Helper macro to give meaningful names to GPIO pins
///
/// The normal [`Pins`] struct names each [`Pin`] according to its [`PinId`].
/// However, BSP authors would prefer to name each [`Pin`] according to its
/// function. This macro defines a new `Pins` struct with custom field names
/// for each [`Pin`], and it defines type aliases and constants to make it
/// easier to work with the [`Pin`]s and [`DynPin`](super::DynPin)s.
///
/// When specifying pin aliases, be sure to use a [`PinMode`]. See
/// [here](self#types) for a list of the available [`PinMode`] type aliases.
///
/// # Example
///
/// The following example macro call
///
/// ```
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
///     PA25 {
///         name: led_fail
///     }
/// );
/// ```
///
/// would expand to something like this
///
/// ```
/// pub struct Pins {
///     port: Option<PORT>,
///     #[cfg(feature = "unproven")]
///     pub led_pass: Pin<PA24, Reset>,
///     pub led_fail: Pin<PA25, Reset>,
/// }
///
/// impl Pins {
///
///     pub fn new(port: PORT) -> Self {
///         let pins = gpio::Pins::new(port);
///         Self {
///             port: Some(unsafe { pins.port() }),
///             #[cfg(feature = "unproven")]
///             led_pass: pins.pa24,
///             led_fail: pins.pa25,
///         }
///     }
///
///     #[inline]
///     pub unsafe fn port(&mut self) -> PORT {
///         self.port.take().unwrap()
///     }
/// }
///
/// #[cfg(feature = "unproven")]
/// pub type LedPass = Pin<PA24, AlternateH>;
///
/// #[cfg(feature = "unproven")]
/// pub const LED_PASS_ID: DynPinId = <PA24 as PinId>::DYN;
///
/// #[cfg(feature = "unproven")]
/// pub const LED_PASS_MODE: DynPinMode = <AlternateH as PinMode>::DYN;
///
/// #[cfg(feature = "unproven")]
/// #[cfg(feature = "usb")]
/// pub type UsbPin = Pin<PA24, AlternateM>;
///
/// #[cfg(feature = "unproven")]
/// #[cfg(feature = "usb")]
/// pub const USB_PIN_ID: DynPinId = <PA24 as PinId>::DYN;
///
/// #[cfg(feature = "unproven")]
/// #[cfg(feature = "usb")]
/// pub const USB_PIN_MODE: DynPinMode = <AlternateM as PinMode>::DYN;
/// ```
#[macro_export]
macro_rules! bsp_pins {
    (
        $(
            $( #[$name_cfg:meta] )*
            $Id:ty {
                name: $name:ident $(,)?
                $(
                    aliases: {
                        $(
                            $( #[$alias_cfg:meta] )*
                            $Mode:ty: $Alias:ident
                        ),+
                    }
                )?
            } $(,)?
        )+
    ) => {
        $crate::paste::paste! {

            /// BSP replacement for the HAL
            /// [`Pins`](atsamd_hal::gpio::v2::Pins) type
            ///
            /// This type is intended to provide more meaningful names for the
            /// given pins.
            pub struct Pins {
                port: Option<$crate::target_device::PORT>,
                $(
                    $( #[$name_cfg] )*
                    pub $name: $crate::gpio::v2::Pin<
                        $crate::gpio::v2::$Id,
                        $crate::gpio::v2::Reset
                    >,
                )+
            }

            impl Pins {

                /// Take ownership of the PAC [`PORT`] and split it into
                /// discrete [`Pin`]s.
                ///
                /// This struct serves as a replacement for the HAL [`Pins`]
                /// struct. It is intended to provide more meaningful names for
                /// each [`Pin`] in a BSP. Any [`Pin`] not defined by the BSP is
                /// dropped.
                ///
                /// [`PORT`](atsamd_hal::target_device::PORT)
                /// [`Pin`](atsamd_hal::gpio::v2::Pin)
                /// [`Pins`](atsamd_hal::gpio::v2::Pins)
                #[inline]
                pub fn new(port: $crate::target_device::PORT) -> Self {
                    let mut pins = $crate::gpio::v2::Pins::new(port);
                    Self {
                        port: Some(unsafe{ pins.port() }),
                        $(
                            $( #[$name_cfg] )*
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
                pub unsafe fn port(&mut self) -> $crate::target_device::PORT {
                    self.port.take().unwrap()
                }
            }

            $(
                $( #[$name_cfg] )*
                $(
                    $(
                        $( #[$alias_cfg] )*
                        /// Alias for a configured [`Pin`](atsamd_hal::gpio::v2::Pin)
                        pub type $Alias = $crate::gpio::v2::Pin<
                            $crate::gpio::v2::$Id,
                            $crate::gpio::v2::$Mode
                        >;
                    )+
                )?
                $( #[$name_cfg] )*
                $(
                    $(
                        $( #[$alias_cfg] )*
                        #[doc = "[DynPinId](atsamd_hal::gpio::v2::DynPinId) "]
                        #[doc = "for the `" $Alias "` alias."]
                        pub const [<$Alias:snake:upper _ID>]: $crate::gpio::v2::DynPinId =
                        <$crate::gpio::v2::$Id as $crate::gpio::v2::PinId>::DYN;
                    )+
                )?
                $( #[$name_cfg] )*
                $(
                    $(
                        $( #[$alias_cfg] )*
                        #[doc = "[DynPinMode](atsamd_hal::gpio::v2::DynPinMode) "]
                        #[doc = "for the `" $Alias "` alias."]
                        pub const [<$Alias:snake:upper _MODE>]: $crate::gpio::v2::DynPinMode =
                        <$crate::gpio::v2::$Mode as $crate::gpio::v2::PinMode>::DYN;
                    )+
                )?
            )+
        }
    };
}
