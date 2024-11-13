//! # Type-level module for GPIO pins
//!
//! This module provides a type-level API for GPIO pins. It uses the type system
//! to track the state of pins at compile-time. Representing GPIO pins in this
//! manner incurs no run-time overhead. Each [`Pin`] struct is zero-sized, so
//! there is no data to copy around. Instead, real code is generated as a side
//! effect of type transformations, and the resulting assembly is nearly
//! identical to the equivalent, hand-written C.
//!
//! To track the state of pins at compile-time, this module uses traits to
//! represent [type classes] and types as instances of those type classes. For
//! example, the trait [`InputConfig`] acts as a [type-level enum] of the
//! available input configurations, and the types [`Floating`], [`PullDown`] and
//! [`PullUp`] are its type-level variants.
//!
//! Type-level [`Pin`]s are parameterized by two type-level enums, [`PinId`] and
//! [`PinMode`].
//!
//! ```
//! pub struct Pin<I, M>
//! where
//!     I: PinId,
//!     M: PinMode,
//! {
//!     // ...
//! }
//! ```
//!
//! A `PinId` identifies a pin by it's group (A, B, C or D) and pin number. Each
//! `PinId` instance is named according to its datasheet identifier, e.g.
//! [`PA02`].
//!
//! A `PinMode` represents the various pin modes. The available `PinMode`
//! variants are [`Disabled`], [`Input`], [`Interrupt`], [`Output`] and
//! [`Alternate`], each with its own corresponding configurations.
//!
//! It is not possible for users to create new instances of a [`Pin`]. Singleton
//! instances of each pin are made available to users through the [`Pins`]
//! struct.
//!
//! To create the [`Pins`] struct, users must supply the PAC
//! [`Port`] peripheral. The [`Pins`] struct takes
//! ownership of the [`Port`] and provides the corresponding pins. Each [`Pin`]
//! within the [`Pins`] struct can be moved out and used individually.
//!
//!
//! ```
//! let mut peripherals = Peripherals::take().unwrap();
//! let pins = Pins::new(peripherals.Port);
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
//! For example, you can control the logic level of an `OutputPin` like so
//!
//! ```
//! use atsamd_hal::pac::Peripherals;
//! use atsamd_hal::gpio::Pins;
//! use crate::ehal_02::digital::v2::OutputPin;
//!
//! let mut peripherals = Peripherals::take().unwrap();
//! let mut pins = Pins::new(peripherals.Port);
//! pins.pa27.set_high();
//! ```
//!
//! # Type-level features
//!
//! This module also provides additional, type-level tools to work with GPIO
//! pins.
//!
//! The [`OptionalPinId`] and [`OptionalPin`] traits use the [`OptionalKind`]
//! pattern to act as type-level versions of [`Option`] for `PinId` and `Pin`
//! respectively. And the [`AnyPin`] trait defines an [`AnyKind`] type class
//! for all `Pin` types.
//!
//! [type classes]: crate::typelevel#type-classes
//! [type-level enum]: crate::typelevel#type-level-enum
//! [`OptionalKind`]: crate::typelevel#optionalkind-trait-pattern
//! [`AnyKind`]: crate::typelevel#anykind-trait-pattern

#![allow(clippy::zero_prefixed_literal)]
#![allow(clippy::bool_comparison)]

use atsamd_hal_macros::{hal_cfg, hal_macro_helper};

use core::convert::Infallible;
use core::marker::PhantomData;
use core::mem::transmute;

use crate::ehal::digital::{ErrorType, InputPin, OutputPin, StatefulOutputPin};
use paste::paste;

use crate::pac::Port;

use crate::typelevel::{NoneT, Sealed};

use super::dynpin::*;
use super::reg::RegisterInterface;

//==============================================================================
//  Disabled configurations
//==============================================================================

/// Type-level enum for disabled configurations
///
/// The valid options are [`Floating`], [`PullDown`] and [`PullUp`]. See the
/// [type-level enum] documentation for more details on the pattern.
///
/// [type-level enum]: crate::typelevel#type-level-enum
pub trait DisabledConfig: Sealed {
    /// Corresponding [`DynDisabled`]
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

/// Type-level enum for input configurations
///
/// The valid options are [`Floating`], [`PullDown`] and [`PullUp`]. See the
/// [type-level enum] documentation for more details on the pattern.
///
/// [type-level enum]: crate::typelevel#type-level-enum
pub trait InputConfig: Sealed {
    /// Corresponding [`DynInput`]
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
//  Interrupt configurations
//==============================================================================

/// Type-level `enum` for Interrupt configurations
pub trait InterruptConfig: Sealed {
    /// Corresponding [`DynInterrupt`]
    const DYN: DynInterrupt;
}

impl InterruptConfig for Floating {
    const DYN: DynInterrupt = DynInterrupt::Floating;
}
impl InterruptConfig for PullDown {
    const DYN: DynInterrupt = DynInterrupt::PullDown;
}
impl InterruptConfig for PullUp {
    const DYN: DynInterrupt = DynInterrupt::PullUp;
}

/// Type-level variant of [`PinMode`] for Interrupt modes
///
/// Type `C` is one of three Interrupt configurations: [`Floating`],
/// [`PullDown`] or [`PullUp`]
pub struct Interrupt<C: InterruptConfig> {
    cfg: PhantomData<C>,
}

impl<C: InterruptConfig> Sealed for Interrupt<C> {}

/// Type-level variant of [`PinMode`] for floating Interrupt mode
pub type FloatingInterrupt = Interrupt<Floating>;

/// Type-level variant of [`PinMode`] for pull-down Interrupt mode
pub type PullDownInterrupt = Interrupt<PullDown>;

/// Type-level variant of [`PinMode`] for pull-up Interrupt mode
pub type PullUpInterrupt = Interrupt<PullUp>;

//==============================================================================
//  Output configurations
//==============================================================================

/// Type-level enum for output configurations
///
/// The valid options are [`PushPull`] and [`Readable`]. See the [type-level
/// enum] documentation for more details on the pattern.
///
/// [type-level enum]: crate::typelevel#type-level-enum
pub trait OutputConfig: Sealed {
    /// Corresponding [`DynOutput`]
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

/// Type-level enum for alternate peripheral function configurations
///
/// See the [type-level enum] documentation for more details on the pattern.
///
/// [type-level enum]: crate::typelevel#type-level-enum
pub trait AlternateConfig: Sealed {
    /// Corresponding [`DynAlternate`]
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

alternate!(B, C, D, E, F, G);

#[hal_cfg(any("port-d21", "port-d5x"))]
alternate!(H);

#[hal_cfg("port-d5x")]
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

/// Type-level enum representing pin modes
///
/// The valid options are [`Disabled`], [`Input`], [`Output`] and [`Alternate`].
/// See the [type-level enum] documentation for more details on the pattern.
///
/// [type-level enum]: crate::typelevel#type-level-enum
pub trait PinMode: Sealed {
    /// Corresponding [`DynPinMode`]
    const DYN: DynPinMode;
}

impl<C: DisabledConfig> PinMode for Disabled<C> {
    const DYN: DynPinMode = DynPinMode::Disabled(C::DYN);
}

impl<C: InputConfig> PinMode for Input<C> {
    const DYN: DynPinMode = DynPinMode::Input(C::DYN);
}

impl<C: InterruptConfig> PinMode for Interrupt<C> {
    const DYN: DynPinMode = DynPinMode::Interrupt(C::DYN);
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

/// Type-level enum for pin IDs
///
/// Valid options take the form `PXYY`, where `X` is a letter in `A`-`D` and
/// `YY` is a number between 00-31. See the [type-level enum] documentation for
/// more details on the pattern.
///
/// [type-level enum]: crate::typelevel#type-level-enum
pub trait PinId: Sealed {
    /// Corresponding [`DynPinId`]
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
//  OptionalPinId
//==============================================================================

/// Type-level equivalent of `Option<PinId>`
///
/// See the [`OptionalKind`] documentation for more details on the pattern.
///
/// [`OptionalKind`]: crate::typelevel#optionalkind-trait-pattern
pub trait OptionalPinId {}

impl OptionalPinId for NoneT {}

impl<I: PinId> OptionalPinId for I {}

/// Type-level equivalent of `Some(PinId)`
///
/// See the [`OptionalKind`] documentation for more details on the pattern.
///
/// [`OptionalKind`]: crate::typelevel#optionalkind-trait-pattern
pub trait SomePinId: OptionalPinId + PinId {}

impl<I: PinId> SomePinId for I {}

//==============================================================================
//  Registers
//==============================================================================

/// Provide a safe register interface for [`Pin`]s
///
/// This `struct` takes ownership of a [`PinId`] and provides an API to
/// access the corresponding regsiters.
pub(in crate::gpio) struct Registers<I: PinId> {
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
    pub(in crate::gpio) fn change_mode<M: PinMode>(&mut self) {
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
    pub(in crate::gpio) regs: Registers<I>,
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
        // Only modify registers if we are actually changing pin mode
        // This check should compile away
        if N::DYN != M::DYN {
            self.regs.change_mode::<N>();
        }
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

    /// Configure the pin to operate as a floating interrupt
    #[inline]
    pub fn into_floating_interrupt(self) -> Pin<I, FloatingInterrupt> {
        self.into_mode()
    }

    /// Configure the pin to operate as a pulled down interrupt
    #[inline]
    pub fn into_pull_down_interrupt(self) -> Pin<I, PullDownInterrupt> {
        self.into_mode()
    }

    /// Configure the pin to operate as a pulled up interrupt
    #[inline]
    pub fn into_pull_up_interrupt(self) -> Pin<I, PullUpInterrupt> {
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

#[hal_macro_helper]
impl_core_convert_from!(
    FloatingDisabled,
    PullDownDisabled,
    PullUpDisabled,
    FloatingInput,
    PullDownInput,
    PullUpInput,
    PushPullOutput,
    ReadableOutput,
    FloatingInterrupt,
    PullUpInterrupt,
    PullDownInterrupt,
    AlternateB,
    AlternateC,
    AlternateD,
    AlternateE,
    AlternateF,
    AlternateG,
    #[hal_cfg(any("port-d21", "port-d5x"))]
    AlternateH,
    #[hal_cfg("port-d5x")]
    AlternateI,
    #[hal_cfg("port-d5x")]
    AlternateJ,
    #[hal_cfg("port-d5x")]
    AlternateK,
    #[hal_cfg("port-d5x")]
    AlternateL,
    #[hal_cfg("port-d5x")]
    AlternateM,
    #[hal_cfg("port-d5x")]
    AlternateN,
);

//==============================================================================
//  AnyPin
//==============================================================================

/// Type class for [`Pin`] types
///
/// This trait uses the [`AnyKind`] trait pattern to create a [type class] for
/// [`Pin`] types. See the `AnyKind` documentation for more details on the
/// pattern.
///
/// ## `v1` Compatibility
///
/// Normally, this trait would use `Is<Type = SpecificPin<Self>>` as a super
/// trait. But doing so would restrict implementations to only the `v2` `Pin`
/// type in this module. To aid in backwards compatibility, we want to implement
/// `AnyPin` for the `v1` `Pin` type as well. This is possible for a few
/// reasons. First, both structs are zero-sized, so there is no meaningful
/// memory layout to begin with. And even if there were, the `v1` `Pin` type is
/// a newtype wrapper around a `v2` `Pin`, and single-field structs are
/// guaranteed to have the same layout as the field, even for `repr(Rust)`.
///
/// [`AnyKind`]: crate::typelevel#anykind-trait-pattern
/// [type class]: crate::typelevel#type-classes
pub trait AnyPin
where
    Self: Sealed,
    Self: From<SpecificPin<Self>>,
    Self: Into<SpecificPin<Self>>,
    Self: AsRef<SpecificPin<Self>>,
    Self: AsMut<SpecificPin<Self>>,
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
/// See the [`AnyKind`] documentation for more details on the pattern.
///
/// [`AnyKind`]: crate::typelevel#anykind-trait-pattern
pub type SpecificPin<P> = Pin<<P as AnyPin>::Id, <P as AnyPin>::Mode>;

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

/// Type-level equivalent of `Option<PinId>`
///
/// See the [`OptionalKind`] documentation for more details on the pattern.
///
/// [`OptionalKind`]: crate::typelevel#optionalkind-trait-pattern
pub trait OptionalPin: Sealed {
    type Id: OptionalPinId;
}

impl OptionalPin for NoneT {
    type Id = NoneT;
}

impl<P: AnyPin> OptionalPin for P {
    type Id = P::Id;
}

/// Type-level equivalent of `Some(PinId)`
///
/// See the [`OptionalKind`] documentation for more details on the pattern.
///
/// [`OptionalKind`]: crate::typelevel#optionalkind-trait-pattern
pub trait SomePin: AnyPin {}
impl<P: AnyPin> SomePin for P {}

//==============================================================================
//  Embedded HAL v1 traits
//==============================================================================

impl<I, M> ErrorType for Pin<I, M>
where
    I: PinId,
    M: PinMode,
{
    type Error = Infallible;
}

impl<I, C> OutputPin for Pin<I, Output<C>>
where
    I: PinId,
    C: OutputConfig,
{
    #[inline]
    fn set_low(&mut self) -> Result<(), Self::Error> {
        self._set_low();
        Ok(())
    }

    #[inline]
    fn set_high(&mut self) -> Result<(), Self::Error> {
        self._set_high();
        Ok(())
    }
}

impl<I> InputPin for Pin<I, ReadableOutput>
where
    I: PinId,
{
    #[inline]
    fn is_high(&mut self) -> Result<bool, Self::Error> {
        Ok(self._is_high())
    }
    #[inline]
    fn is_low(&mut self) -> Result<bool, Self::Error> {
        Ok(self._is_low())
    }
}

impl<I, C> InputPin for Pin<I, Input<C>>
where
    I: PinId,
    C: InputConfig,
{
    #[inline]
    fn is_high(&mut self) -> Result<bool, Self::Error> {
        Ok(self._is_high())
    }
    #[inline]
    fn is_low(&mut self) -> Result<bool, Self::Error> {
        Ok(self._is_low())
    }
}

impl<I, C> InputPin for Pin<I, Interrupt<C>>
where
    I: PinId,
    C: InterruptConfig,
{
    #[inline]
    fn is_high(&mut self) -> Result<bool, Self::Error> {
        Ok(self._is_high())
    }
    #[inline]
    fn is_low(&mut self) -> Result<bool, Self::Error> {
        Ok(self._is_low())
    }
}

impl<I, C> StatefulOutputPin for Pin<I, Output<C>>
where
    I: PinId,
    C: OutputConfig,
{
    #[inline]
    fn is_set_high(&mut self) -> Result<bool, Self::Error> {
        Ok(self._is_set_high())
    }
    #[inline]
    fn is_set_low(&mut self) -> Result<bool, Self::Error> {
        Ok(self._is_set_low())
    }
}

//==============================================================================
//  Embedded HAL v0.2 traits
//==============================================================================

impl<I, C> crate::ehal_02::digital::v2::OutputPin for Pin<I, Output<C>>
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

impl<I> crate::ehal_02::digital::v2::InputPin for Pin<I, ReadableOutput>
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

impl<I, C> crate::ehal_02::digital::v2::InputPin for Pin<I, Input<C>>
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

impl<I, C> crate::ehal_02::digital::v2::InputPin for Pin<I, Interrupt<C>>
where
    I: PinId,
    C: InterruptConfig,
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

impl<I, C> crate::ehal_02::digital::v2::ToggleableOutputPin for Pin<I, Output<C>>
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

impl<I, C> crate::ehal_02::digital::v2::StatefulOutputPin for Pin<I, Output<C>>
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
            #[$cfg:meta]
            $Id:ident,
        )+
    ) => {
        paste! {
            /// Collection of all the individual [`Pin`]s
            pub struct Pins {
                port: Option<Port>,
                $(
                    #[doc = "Pin " $Id]
                    #[$cfg]
                    pub [<$Id:lower>]: Pin<$Id, Reset>,
                )+
            }
            impl Pins {
                /// Take ownership of the PAC
                /// [`Port`](crate::pac::Port) and split it into
                /// discrete [`Pin`]s
                #[inline]
                pub fn new(port: Port) -> Pins {
                    Pins {
                        port: Some(port),
                        // Safe because we only create one `Pin` per `PinId`
                        $(
                            #[$cfg]
                            [<$Id:lower>]: unsafe { Pin::new() },
                        )+
                    }
                }
                /// Take the PAC [`Port`]
                ///
                /// The [`Port`] can only be taken once. Subsequent calls to
                /// this function will panic.
                ///
                /// # Safety
                ///
                /// Direct access to the [`Port`] could allow you to invalidate
                /// the compiler's type-level tracking, so it is unsafe.
                ///
                /// [`Port`](crate::pac::Port)
                #[inline]
                pub unsafe fn port(&mut self) -> Port {
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
                    #[$cfg:meta]
                    ($Id:ident, $NUM:literal),
                )+
            }
        )+
    ) => {
        $(
            $(
                #[$cfg]
                pin_id!($Group, $Id, $NUM);
            )+
        )+
        pins!(
            $(
                $(
                    #[$cfg]
                    $Id,
                )+
            )+
        );
    };
}

#[hal_macro_helper]
declare_pins!(
    A {
        #[hal_cfg("pa00")]
        (PA00, 00),
        #[hal_cfg("pa01")]
        (PA01, 01),
        #[hal_cfg("pa02")]
        (PA02, 02),
        #[hal_cfg("pa03")]
        (PA03, 03),
        #[hal_cfg("pa04")]
        (PA04, 04),
        #[hal_cfg("pa05")]
        (PA05, 05),
        #[hal_cfg("pa06")]
        (PA06, 06),
        #[hal_cfg("pa07")]
        (PA07, 07),
        #[hal_cfg("pa08")]
        (PA08, 08),
        #[hal_cfg("pa09")]
        (PA09, 09),
        #[hal_cfg("pa10")]
        (PA10, 10),
        #[hal_cfg("pa11")]
        (PA11, 11),
        #[hal_cfg("pa12")]
        (PA12, 12),
        #[hal_cfg("pa13")]
        (PA13, 13),
        #[hal_cfg("pa14")]
        (PA14, 14),
        #[hal_cfg("pa15")]
        (PA15, 15),
        #[hal_cfg("pa16")]
        (PA16, 16),
        #[hal_cfg("pa17")]
        (PA17, 17),
        #[hal_cfg("pa18")]
        (PA18, 18),
        #[hal_cfg("pa19")]
        (PA19, 19),
        #[hal_cfg("pa20")]
        (PA20, 20),
        #[hal_cfg("pa21")]
        (PA21, 21),
        #[hal_cfg("pa22")]
        (PA22, 22),
        #[hal_cfg("pa23")]
        (PA23, 23),
        #[hal_cfg("pa24")]
        (PA24, 24),
        #[hal_cfg("pa25")]
        (PA25, 25),
        #[hal_cfg("pa27")]
        (PA27, 27),
        #[hal_cfg("pa28")]
        (PA28, 28),
        #[hal_cfg("pa30")]
        (PA30, 30),
        #[hal_cfg("pa31")]
        (PA31, 31),
    }
    B {
        #[hal_cfg("pb00")]
        (PB00, 00),
        #[hal_cfg("pb01")]
        (PB01, 01),
        #[hal_cfg("pb02")]
        (PB02, 02),
        #[hal_cfg("pb03")]
        (PB03, 03),
        #[hal_cfg("pb04")]
        (PB04, 04),
        #[hal_cfg("pb05")]
        (PB05, 05),
        #[hal_cfg("pb06")]
        (PB06, 06),
        #[hal_cfg("pb07")]
        (PB07, 07),
        #[hal_cfg("pb08")]
        (PB08, 08),
        #[hal_cfg("pb09")]
        (PB09, 09),
        #[hal_cfg("pb10")]
        (PB10, 10),
        #[hal_cfg("pb11")]
        (PB11, 11),
        #[hal_cfg("pb12")]
        (PB12, 12),
        #[hal_cfg("pb13")]
        (PB13, 13),
        #[hal_cfg("pb14")]
        (PB14, 14),
        #[hal_cfg("pb15")]
        (PB15, 15),
        #[hal_cfg("pb16")]
        (PB16, 16),
        #[hal_cfg("pb17")]
        (PB17, 17),
        #[hal_cfg("pb18")]
        (PB18, 18),
        #[hal_cfg("pb19")]
        (PB19, 19),
        #[hal_cfg("pb20")]
        (PB20, 20),
        #[hal_cfg("pb21")]
        (PB21, 21),
        #[hal_cfg("pb22")]
        (PB22, 22),
        #[hal_cfg("pb23")]
        (PB23, 23),
        #[hal_cfg("pb24")]
        (PB24, 24),
        #[hal_cfg("pb25")]
        (PB25, 25),
        #[hal_cfg("pb26")]
        (PB26, 26),
        #[hal_cfg("pb27")]
        (PB27, 27),
        #[hal_cfg("pb28")]
        (PB28, 28),
        #[hal_cfg("pb29")]
        (PB29, 29),
        #[hal_cfg("pb30")]
        (PB30, 30),
        #[hal_cfg("pb31")]
        (PB31, 31),
    }
    C {
        #[hal_cfg("pc00")]
        (PC00, 00),
        #[hal_cfg("pc01")]
        (PC01, 01),
        #[hal_cfg("pc02")]
        (PC02, 02),
        #[hal_cfg("pc03")]
        (PC03, 03),
        #[hal_cfg("pc04")]
        (PC04, 04),
        #[hal_cfg("pc05")]
        (PC05, 05),
        #[hal_cfg("pc06")]
        (PC06, 06),
        #[hal_cfg("pc07")]
        (PC07, 07),
        #[hal_cfg("pc10")]
        (PC10, 10),
        #[hal_cfg("pc11")]
        (PC11, 11),
        #[hal_cfg("pc12")]
        (PC12, 12),
        #[hal_cfg("pc13")]
        (PC13, 13),
        #[hal_cfg("pc14")]
        (PC14, 14),
        #[hal_cfg("pc15")]
        (PC15, 15),
        #[hal_cfg("pc16")]
        (PC16, 16),
        #[hal_cfg("pc17")]
        (PC17, 17),
        #[hal_cfg("pc18")]
        (PC18, 18),
        #[hal_cfg("pc19")]
        (PC19, 19),
        #[hal_cfg("pc20")]
        (PC20, 20),
        #[hal_cfg("pc21")]
        (PC21, 21),
        #[hal_cfg("pc22")]
        (PC22, 22),
        #[hal_cfg("pc23")]
        (PC23, 23),
        #[hal_cfg("pc24")]
        (PC24, 24),
        #[hal_cfg("pc25")]
        (PC25, 25),
        #[hal_cfg("pc26")]
        (PC26, 26),
        #[hal_cfg("pc27")]
        (PC27, 27),
        #[hal_cfg("pc28")]
        (PC28, 28),
        #[hal_cfg("pc30")]
        (PC30, 30),
        #[hal_cfg("pc31")]
        (PC31, 31),
    }
    D {
        #[hal_cfg("pd00")]
        (PD00, 00),
        #[hal_cfg("pd01")]
        (PD01, 01),
        #[hal_cfg("pd08")]
        (PD08, 08),
        #[hal_cfg("pd09")]
        (PD09, 09),
        #[hal_cfg("pd10")]
        (PD10, 10),
        #[hal_cfg("pd11")]
        (PD11, 11),
        #[hal_cfg("pd12")]
        (PD12, 12),
        #[hal_cfg("pd20")]
        (PD20, 20),
        #[hal_cfg("pd21")]
        (PD21, 21),
    }
);

//==============================================================================
//  bsp_pins
//==============================================================================

/// # Helper macro to give meaningful names to GPIO pins
///
/// The [`atsamd_hal::gpio`](self) module generally refers to each [`Pin`] by
/// its [`PinId`]. However, in the context of a BSP, pins can often be given
/// more meaningful names. This macro gives BSP authors a convenient way to
/// provide custom names for each pin.
///
/// ## Calling the macro
///
/// The `bsp_pins!` macro takes a series of `PinId` blocks. Each block starts
/// with a `PinId` and is delimited by curly brackets. Within each block, there
/// are two optional fields, `name` and `aliases`. The `name` field represents
/// the *principal* name or function assigned to the pin and is given in
/// `snake_case`. If the `name` field is absent, the pin name will default to
/// its `PinId` (converted to `snake_case`). The `aliases` field represents any
/// number of alternative names for the pin, where each name corresponds to the
/// pin in a particular [`PinMode`]. Note that each alias is given in
/// `PascalCase`.
///
/// The example below defines a `name` and two `aliases` for pin `PA24`. In
/// `PinMode` [`AlternateC`], the pin is used as an SPI MOSI pin. In `PinMode`
/// [`AlternateD`], it is used as a UART TX pin. In both cases, it is a serial
/// output, so its `name` is `serial_out`.
///
/// ```
/// atsamd_hal::bsp_pins!(
///     PA24 {
///         name: serial_out,
///         aliases: {
///             AlternateC: SpiMosi,
///             AlternateD: UartTx,
///         }
///     }
/// );
/// ```
///
/// ## Expanding the macro
///
/// When expanded, the `bsp_pins!` macro will define a number of structs, type
/// aliases, constants and macros.
///
/// ### A new `Pins` struct
///
/// First, it will define a new, more-useful `Pins` struct. The [`Pins`] struct
/// defined in the `gpio` module is intended for general use. It contains *all*
/// the pins for a given chip, and each pin is named according to its `PinId`.
/// The `Pins` struct defined by this macro, on the other hand, contains only
/// the declared pins, and each pin is named appropriately.
///
/// The field name for each pin within the `Pins` struct is based on the macro
/// `name` field. For example, the `serial_out` pin from the example above could
/// be accessed like this:
///
/// ```
/// let mut peripherals = pac::Peripherals::take().unwrap();
/// let pins = bsp::Pins::new(peripherals.Port);
/// let out = pins.serial_out;
/// ```
///
/// However, that is not the only way to access each pin. While the `name` field
/// represents the principal name, each pin can also be accessed using its
/// corresponding `aliases`.
///
/// In Rust, each struct field can only have one name. To provide access to the
/// same struct field using several *different* names, the `bsp_pins!` macro
/// defines another macro, `pin_alias!`. Based on the example above, we could
/// use the `pin_alias!` macro to access pin `PA24` without ever referring to
/// the `serial_out` field.
///
/// ```
/// let mut peripherals = pac::Peripherals::take().unwrap();
/// let pins = bsp::Pins::new(peripherals.Port);
/// let mosi = pin_alias!(pins.spi_mosi);
/// ```
///
/// Note that the `SpiMosi` alias was translated to `snake_case` when accessing
/// the `Pins` field. The same is true for the `UartTx` alias.
///
/// ```
/// let mut peripherals = pac::Peripherals::take().unwrap();
/// let pins = bsp::Pins::new(peripherals.Port);
/// let tx = pin_alias!(pins.uart_tx);
/// ```
///
/// ### Type aliases
///
/// Next, the macro defines several useful type aliases for each pin. It
/// provides aliases for the corresponding `PinId`, `PinMode` and fully
/// specified `Pin` type of each alternate name.
///
/// The example above would exand to
///
/// ```
/// pub type SpiMosi = Pin<PA24, AlternateC>;
/// pub type SpiMosiId = PA24;
/// pub type SpiMosiMode = AlternateC;
///
/// pub type UartTx = Pin<PA24, AlternateD>;
/// pub type UartTxId = PA24;
/// pub type UartTxMode = AlternateD;
/// ```
///
/// Each `PascalCase` alias provided in the macro is used for the `Pin` type,
/// and the suffixes `Id` and `Mode` are appended to for the corresponding
/// `PinId` and `PinMode` types.
///
/// ### `DYN` constants
///
/// Although the [`pin`](self) API is more common, there are use cases for the
/// type-erased, [`dyn_pin`](super::dynpin) API as well. The `bsp_pins!` macro
/// also defines some useful constants for these cases. In particular, it
/// defines [`DynPinId`] and [`DynPinMode`] constants for each alias.
///
/// The example above would effectively expand to
///
/// ```
/// pub const SPI_MOSI_ID: DynPinId = DynPinId { group: DynGroup::A, num: 24 };
/// pub const SPI_MOSI_MODE: DynPinMode = DYN_ALTERNATE_C;
///
/// pub const UART_TX_ID: DynPinId = DynPinId { group: DynGroup::A, num: 24 };
/// pub const UART_TX_MODE: DynPinMode = DYN_ALTERNATE_D;
/// ```
///
/// The `PascalCase` alias provided in the macro is converted to
/// `SCREAMING_CASE`, and the suffixes `_ID` and `_MODE` are appended for the
/// corresponding `DynPinId` and `DynPinMode` constants.
///
/// ## Attributes and documentation
///
/// BSP authors can also add attributes and documentation to various parts of
/// the macro declaration. Attributes can be added to the entire `PinId` block.
/// These attributes will be propagated to every use of the corresponding
/// `PinId`. Attributes applied to each alias, on the other hand, will only be
/// propagated to items specific to that alias, like the corresponding `DYN`
/// constants. Finally, any documentation (or other attributes) provided for the
/// `name` field will be propagated to the corresponding field of the
/// `bsp::Pins` struct defined by this macro.
///
/// ```
/// atsamd_hal::bsp_pins!(
///     #[cfg(feature = "has_pin_PA24")]
///     PA24 {
///         /// Documentation that will appear on the corresponding field in the
///         /// `bsp::Pins` struct
///         name: serial_out,
///         aliases: {
///             #[cfg(feature = "uses_SPI")]
///             AlternateC: SpiMosi,
///             #[cfg(feature = "uses_UART")]
///             AlternateD: UartTx,
///         }
///     }
/// );
/// ```
#[macro_export]
macro_rules! bsp_pins {
    (
        $(
            $( #[$id_cfg:meta] )*
            $Id:ident {
                $( #[$name_doc:meta] )*
                $( name: $name:ident $(,)? )?
                $(
                    aliases: {
                        $(
                            $( #[$alias_cfg:meta] )*
                            $Mode:ident: $Alias:ident $(,)?
                        )+
                    }
                )?
            } $(,)?
        )+
    ) => {
        $crate::paste::paste! {

            $crate::__declare_pins_type!(
                $(
                    {
                        $( #[$id_cfg] )*
                        ( $Id, [<$Id:lower>] )
                        $( #[$name_doc] )*
                        $(
                            #[
                                doc = "\nThis field can also be accessed using the [`pin_alias!`] \
                                macro with the following alternate names:\n    "
                            ]
                            $(
                                #[doc = $Alias:snake ", "]
                            )+
                        )?
                        ( $( $name )? [<$Id:lower>] )
                    }
                )+
            );

            $(
                $( #[$id_cfg] )*
                $crate::__create_pin_aliases!(
                    $Id
                    ( $( $name )? [<$Id:lower>] )
                    $(
                        $(
                            $( #[$alias_cfg] )*
                            { $Mode, $Alias }
                        )+
                    )?
                );
            )+

            $crate::__define_pin_alias_macro!(
                $(
                    {
                        ( $( $name )? [<$Id:lower>] )
                        $(
                            $(
                                $( #[$alias_cfg] )*
                                [<$Alias:snake>]
                            )+
                        )?
                    }
                )+
            );

        }
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! __declare_pins_type {
    (
        $(
            {
                $( #[$id_cfg:meta] )*
                ( $Id:ident, $id:ident )
                $( #[$name_doc:meta] )*
                ( $name:ident $( $others:ident )* )
            }
        )+
    ) => {
        /// BSP replacement for the HAL
        /// [`Pins`](atsamd_hal::gpio::Pins) type
        ///
        /// This type is intended to provide more meaningful names for the
        /// given pins.
        pub struct Pins {
            port: Option<$crate::pac::Port>,
            $(
                $( #[$id_cfg] )*
                $( #[$name_doc] )*
                pub $name: $crate::gpio::Pin<
                    $crate::gpio::$Id,
                    $crate::gpio::Reset
                >,
            )+
        }

        impl Pins {

            /// Take ownership of the PAC [`Port`] and split it into
            /// discrete [`Pin`]s.
            ///
            /// This struct serves as a replacement for the HAL [`Pins`]
            /// struct. It is intended to provide more meaningful names for
            /// each [`Pin`] in a BSP. Any [`Pin`] not defined by the BSP is
            /// dropped.
            ///
            /// [`Port`](atsamd_hal::pac::Port)
            /// [`Pin`](atsamd_hal::gpio::Pin)
            /// [`Pins`](atsamd_hal::gpio::Pins)
            #[inline]
            pub fn new(port: $crate::pac::Port) -> Self {
                let mut pins = $crate::gpio::Pins::new(port);
                Self {
                    port: Some(unsafe{ pins.port() }),
                    $(
                        $( #[$id_cfg] )*
                        $name: pins.$id,
                    )+
                }
            }

            /// Take the PAC [`Port`]
            ///
            /// The [`Port`] can only be taken once. Subsequent calls to
            /// this function will panic.
            ///
            /// # Safety
            ///
            /// Direct access to the [`Port`] could allow you to invalidate
            /// the compiler's type-level tracking, so it is unsafe.
            ///
            /// [`Port`](atsamd_hal::pac::Port)
            #[inline]
            pub unsafe fn port(&mut self) -> $crate::pac::Port {
                self.port.take().unwrap()
            }
        }
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! __create_pin_aliases {
    (
        $Id:ident
        ( $name:ident $( $others:ident )* )
        $(
            $( #[$attr:meta] )*
            { $Mode:ident, $Alias:ident }
        )*
    ) => {
        $crate::paste::paste! {
            $(
                $( #[$attr] )*
                /// Alias for a configured [`Pin`](atsamd_hal::gpio::Pin)
                pub type $Alias = $crate::gpio::Pin<
                    $crate::gpio::$Id,
                    $crate::gpio::$Mode
                >;

                $( #[$attr] )*
                #[doc = "[`PinId`](atsamd_hal::gpio::PinId) for the [`"]
                #[doc = $Alias "`] alias"]
                pub type [<$Alias Id>] = $crate::gpio::$Id;

                $( #[$attr] )*
                #[doc = "[`PinMode`](atsamd_hal::gpio::PinMode) for the [`"]
                #[doc = $Alias "`] alias"]
                pub type [<$Alias Mode>] = $crate::gpio::$Mode;

                $( #[$attr] )*
                #[doc = "[DynPinId](atsamd_hal::gpio::DynPinId) "]
                #[doc = "for the `" $Alias "` alias."]
                pub const [<$Alias:snake:upper _ID>]: $crate::gpio::DynPinId =
                <$crate::gpio::$Id as $crate::gpio::PinId>::DYN;

                $( #[$attr] )*
                #[doc = "[DynPinMode](atsamd_hal::gpio::DynPinMode) "]
                #[doc = "for the `" $Alias "` alias."]
                pub const [<$Alias:snake:upper _MODE>]: $crate::gpio::DynPinMode =
                <$crate::gpio::$Mode as $crate::gpio::PinMode>::DYN;
            )*
        }
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! __define_pin_alias_macro {
    (
        $(
            {
                ( $name:ident $( $others:ident )* )
                $(
                    $( #[$attr:meta] )*
                    $alias:ident
                )*
            }
        )+
    ) => {
        $crate::paste::paste! {
            /// Refer to fields of the [`Pins`] struct by alternate names
            ///
            /// This macro can be used to access fields of the [`Pins`] struct
            /// by alternate names. See the `Pins` documentation for a list of
            /// the availabe pin aliases.
            ///
            /// For example. suppose `spi_mosi` were an alternate name for the
            /// `serial_out` pin of the `Pins` struct. You could use the
            /// `pin_alias!` macro to access it like this:
            ///
            /// ```
            /// let mut peripherals = pac::Peripherals::take().unwrap();
            /// let pins = bsp::Pins::new(peripherals.Port);
            /// // Replace this
            /// let mosi = pins.serial_out;
            /// // With this
            /// let mosi = pin_alias!(pins.spi_mosi);
            /// ```
            #[macro_export]
            macro_rules! pin_alias {
                $(
                    // Always provide an identity "alias"
                    ( $pins:ident . $name ) => { $pins.$name };
                )+
                $(
                    $(
                        ( $pins:ident . $alias ) => {
                            {
                                // Since attributes can't apply to expressions, only
                                // items, apply any attributes to a dummy macro. This
                                // lets us ensure the alias is only valid when the
                                // corresponding attributes are valid.
                                $( #[$attr] )*
                                macro_rules! [<pin_alias_ $alias>] {
                                    () => { $pins.$name };
                                }
                                [<pin_alias_ $alias>]!()
                            }
                        };
                    )*
                )+
            }
        }
    }
}
