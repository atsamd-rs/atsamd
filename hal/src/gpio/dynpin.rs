//! # Type-erased, value-level module for GPIO pins
//!
//! Although the type-level API is generally preferred, it is not suitable in
//! all cases. Because each pin is represented by a distinct type, it is not
//! possible to store multiple pins in a homogeneous data structure. The
//! value-level API solves this problem by erasing the type information and
//! tracking the pin at run-time.
//!
//! Value-level pins are represented by the [`DynPin`] type. [`DynPin`] has two
//! fields, `id` and `mode` with types [`DynPinId`] and [`DynPinMode`]
//! respectively. The implementation of these types closely mirrors the
//! type-level API.
//!
//! Instances of [`DynPin`] cannot be created directly. Rather, they must be
//! created from their type-level equivalents using [`From`]/[`Into`].
//!
//! ```
//! // Move a pin out of the Pins struct and convert to a DynPin
//! let pa27: DynPin = pins.pa27.into();
//! ```
//!
//! Conversions between pin modes use a value-level version of the type-level
//! API.
//!
//! ```
//! // Use one of the literal function names
//! pa27.into_floating_input();
//! // Use a method and a DynPinMode variant
//! pa27.into_mode(DYN_FLOATING_INPUT);
//! ```
//!
//! Because the pin state cannot be tracked at compile-time, many [`DynPin`]
//! operations become fallible. Run-time checks are inserted to ensure that
//! users don't try to, for example, set the output level of an input pin.
//!
//! Users may try to convert value-level pins back to their type-level
//! equivalents. However, this option is fallible, because the compiler cannot
//! guarantee the pin has the correct ID or is in the correct mode at
//! compile-time. Use [`TryFrom`](core::convert::TryFrom)/
//! [`TryInto`](core::convert::TryInto) for this conversion.
//!
//! ```
//! // Convert to a `DynPin`
//! let pa27: DynPin = pins.pa27.into();
//! // Change pin mode
//! pa27.into_floating_input();
//! // Convert back to a `Pin`
//! let pa27: Pin<PA27, FloatingInput> = pa27.try_into().unwrap();
//! ```
//!
//! # Embedded HAL traits
//!
//! This module implements all of the embedded HAL GPIO traits for [`DynPin`].
//! However, whereas the type-level API uses
//! `Error = core::convert::Infallible`, the value-level API can return a real
//! error. If the [`DynPin`] is not in the correct [`DynPinMode`] for the
//! operation, the trait functions will return
//! [`InvalidPinType`](Error::InvalidPinType).

use core::convert::TryFrom;

use paste::paste;

use crate::ehal::digital::v2::OutputPin;
#[cfg(feature = "unproven")]
use crate::ehal::digital::v2::{InputPin, StatefulOutputPin, ToggleableOutputPin};

use super::pin::*;
use super::reg::RegisterInterface;

//==============================================================================
//  DynPinMode configurations
//==============================================================================

/// Value-level `enum` for disabled configurations
#[derive(PartialEq, Eq, Clone, Copy)]
pub enum DynDisabled {
    Floating,
    PullDown,
    PullUp,
}

/// Value-level `enum` for input configurations
#[derive(PartialEq, Eq, Clone, Copy)]
pub enum DynInput {
    Floating,
    PullDown,
    PullUp,
}

/// Value-level `enum` for interrupt configurations
#[derive(PartialEq, Eq, Clone, Copy)]
pub enum DynInterrupt {
    Floating,
    PullDown,
    PullUp,
}

/// Value-level `enum` for output configurations
#[derive(PartialEq, Eq, Clone, Copy)]
pub enum DynOutput {
    PushPull,
    Readable,
}

/// Value-level `enum` for alternate peripheral function configurations
#[derive(PartialEq, Eq, Clone, Copy)]
pub enum DynAlternate {
    B,
    C,
    D,
    E,
    F,
    G,
    #[cfg(any(feature = "samd21", feature = "min-samd51g"))]
    H,
    #[cfg(feature = "min-samd51g")]
    I,
    #[cfg(feature = "min-samd51g")]
    J,
    #[cfg(feature = "min-samd51g")]
    K,
    #[cfg(feature = "min-samd51g")]
    L,
    #[cfg(feature = "min-samd51g")]
    M,
    #[cfg(feature = "min-samd51g")]
    N,
}

//==============================================================================
//  DynPinMode
//==============================================================================

/// Value-level `enum` representing pin modes
#[derive(PartialEq, Eq, Clone, Copy)]
pub enum DynPinMode {
    Disabled(DynDisabled),
    Input(DynInput),
    Interrupt(DynInterrupt),
    Output(DynOutput),
    Alternate(DynAlternate),
}

/// Value-level variant of [`DynPinMode`] for floating disabled mode
pub const DYN_FLOATING_DISABLED: DynPinMode = DynPinMode::Disabled(DynDisabled::Floating);
/// Value-level variant of [`DynPinMode`] for pull-down disabled mode
pub const DYN_PULL_DOWN_DISABLED: DynPinMode = DynPinMode::Disabled(DynDisabled::PullDown);
/// Value-level variant of [`DynPinMode`] for pull-up disabled mode
pub const DYN_PULL_UP_DISABLED: DynPinMode = DynPinMode::Disabled(DynDisabled::PullUp);

/// Value-level variant of [`DynPinMode`] for floating input mode
pub const DYN_FLOATING_INPUT: DynPinMode = DynPinMode::Input(DynInput::Floating);
/// Value-level variant of [`DynPinMode`] for pull-down input mode
pub const DYN_PULL_DOWN_INPUT: DynPinMode = DynPinMode::Input(DynInput::PullDown);
/// Value-level variant of [`DynPinMode`] for pull-up input mode
pub const DYN_PULL_UP_INPUT: DynPinMode = DynPinMode::Input(DynInput::PullUp);

/// Value-level variant of [`DynPinMode`] for floating interrupt mode
pub const DYN_FLOATING_INTERRUPT: DynPinMode = DynPinMode::Interrupt(DynInterrupt::Floating);
/// Value-level variant of [`DynPinMode`] for pull-down interrupt mode
pub const DYN_PULL_DOWN_INTERRUPT: DynPinMode = DynPinMode::Interrupt(DynInterrupt::PullDown);
/// Value-level variant of [`DynPinMode`] for pull-up interrupt mode
pub const DYN_PULL_UP_INTERRUPT: DynPinMode = DynPinMode::Interrupt(DynInterrupt::PullUp);

/// Value-level variant of [`DynPinMode`] for push-pull output mode
pub const DYN_PUSH_PULL_OUTPUT: DynPinMode = DynPinMode::Output(DynOutput::PushPull);
/// Value-level variant of [`DynPinMode`] for readable push-pull output mode
pub const DYN_READABLE_OUTPUT: DynPinMode = DynPinMode::Output(DynOutput::Readable);

macro_rules! dyn_alternate {
    ( $($Letter:ident),+ ) => {
        paste! {
            $(
                #[
                    doc = "Value-level variant of [`DynPinMode`] for alternate "
                    "peripheral function " $Letter
                ]
                pub const [<DYN_ALTERNATE_ $Letter>]: DynPinMode =
                DynPinMode::Alternate(DynAlternate::$Letter);
            )+
        }
    };
}

dyn_alternate!(B, C, D, E, F, G);
#[cfg(any(feature = "samd21", feature = "min-samd51g"))]
dyn_alternate!(H);
#[cfg(feature = "min-samd51g")]
dyn_alternate!(I, J, K, L, M, N);

//==============================================================================
//  DynGroup & DynPinId
//==============================================================================

/// Value-level `enum` for pin groups
#[derive(PartialEq, Clone, Copy)]
pub enum DynGroup {
    A,
    #[cfg(any(feature = "samd21", feature = "min-samd51g"))]
    B,
    #[cfg(feature = "min-samd51n")]
    C,
    #[cfg(feature = "min-samd51p")]
    D,
}

/// Value-level `struct` representing pin IDs
#[derive(PartialEq, Clone, Copy)]
pub struct DynPinId {
    pub group: DynGroup,
    pub num: u8,
}

//==============================================================================
//  DynRegisters
//==============================================================================

/// Provide a safe register interface for [`DynPin`]s
///
/// This `struct` takes ownership of a [`DynPinId`] and provides an API to
/// access the corresponding regsiters.
struct DynRegisters {
    id: DynPinId,
}

// [`DynRegisters`] takes ownership of the [`DynPinId`], and [`DynPin`]
// guarantees that each pin is a singleton, so this implementation is safe.
unsafe impl RegisterInterface for DynRegisters {
    #[inline]
    fn id(&self) -> DynPinId {
        self.id
    }
}

impl DynRegisters {
    /// Create a new instance of [`DynRegisters`]
    ///
    /// # Safety
    ///
    /// Users must never create two simultaneous instances of this `struct` with
    /// the same [`DynPinId`]
    #[inline]
    unsafe fn new(id: DynPinId) -> Self {
        DynRegisters { id }
    }
}

//==============================================================================
//  Error
//==============================================================================

/// GPIO error type
///
/// [`DynPin`]s are not tracked and verified at compile-time, so run-time
/// operations are fallible. This `enum` represents the corresponding errors.
pub enum Error {
    /// The pin did not have the correct ID or mode for the requested operation
    InvalidPinType,
}

//==============================================================================
//  DynPin
//==============================================================================

/// A value-level pin, parameterized by [`DynPinId`] and [`DynPinMode`]
///
/// This type acts as a type-erased version of [`Pin`]. Every pin is represented
/// by the same type, and pins are tracked and distinguished at run-time.
pub struct DynPin {
    regs: DynRegisters,
    mode: DynPinMode,
}

impl DynPin {
    /// Create a new [`DynPin`]
    ///
    /// # Safety
    ///
    /// Each [`DynPin`] must be a singleton. For a given [`DynPinId`], there
    /// must be at most one corresponding [`DynPin`] in existence at any given
    /// time.  Violating this requirement is `unsafe`.
    #[inline]
    unsafe fn new(id: DynPinId, mode: DynPinMode) -> Self {
        DynPin {
            regs: DynRegisters::new(id),
            mode,
        }
    }

    /// Return a copy of the pin ID
    #[inline]
    pub fn id(&self) -> DynPinId {
        self.regs.id
    }

    /// Return a copy of the pin mode
    #[inline]
    pub fn mode(&self) -> DynPinMode {
        self.mode
    }

    /// Convert the pin to the requested [`DynPinMode`]
    #[inline]
    pub fn into_mode(&mut self, mode: DynPinMode) {
        // Only modify registers if we are actually changing pin mode
        if mode != self.mode {
            self.regs.change_mode(mode);
            self.mode = mode;
        }
    }

    /// Disable the pin and set it to float
    #[inline]
    pub fn into_floating_disabled(&mut self) {
        self.into_mode(DYN_FLOATING_DISABLED);
    }

    /// Disable the pin and set it to pull down
    #[inline]
    pub fn into_pull_down_disabled(&mut self) {
        self.into_mode(DYN_PULL_DOWN_DISABLED);
    }

    /// Disable the pin and set it to pull up
    #[inline]
    pub fn into_pull_up_disabled(&mut self) {
        self.into_mode(DYN_PULL_UP_DISABLED);
    }

    /// Configure the pin to operate as a floating input
    #[inline]
    pub fn into_floating_input(&mut self) {
        self.into_mode(DYN_FLOATING_INPUT);
    }

    /// Configure the pin to operate as a pulled down input
    #[inline]
    pub fn into_pull_down_input(&mut self) {
        self.into_mode(DYN_PULL_DOWN_INPUT);
    }

    /// Configure the pin to operate as a pulled up input
    #[inline]
    pub fn into_pull_up_input(&mut self) {
        self.into_mode(DYN_PULL_UP_INPUT);
    }

    /// Configure the pin to operate as a floating interrupt
    #[inline]
    pub fn into_floating_interrupt(&mut self) {
        self.into_mode(DYN_FLOATING_INTERRUPT);
    }

    /// Configure the pin to operate as a pulled down interrupt
    #[inline]
    pub fn into_pull_down_interrupt(&mut self) {
        self.into_mode(DYN_PULL_DOWN_INTERRUPT);
    }

    /// Configure the pin to operate as a pulled up interrupt
    #[inline]
    pub fn into_pull_up_interrupt(&mut self) {
        self.into_mode(DYN_PULL_UP_INTERRUPT);
    }

    /// Configure the pin to operate as a push-pull output
    #[inline]
    pub fn into_push_pull_output(&mut self) {
        self.into_mode(DYN_PUSH_PULL_OUTPUT);
    }

    /// Configure the pin to operate as a readable push pull output
    #[inline]
    pub fn into_readable_output(&mut self) {
        self.into_mode(DYN_READABLE_OUTPUT);
    }

    /// Configure the pin to operate as the corresponding peripheral function.
    ///
    /// The `config` argument indicates the desired peripheral function.
    #[inline]
    pub fn into_alternate(&mut self, config: DynAlternate) {
        self.into_mode(DynPinMode::Alternate(config));
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
    fn _read(&self) -> Result<bool, Error> {
        match self.mode {
            DynPinMode::Input(_) | DYN_READABLE_OUTPUT => Ok(self.regs.read_pin()),
            _ => Err(Error::InvalidPinType),
        }
    }
    #[inline]
    fn _write(&mut self, bit: bool) -> Result<(), Error> {
        match self.mode {
            DynPinMode::Output(_) => {
                self.regs.write_pin(bit);
                Ok(())
            }
            _ => Err(Error::InvalidPinType),
        }
    }
    #[inline]
    fn _toggle(&mut self) -> Result<(), Error> {
        match self.mode {
            DynPinMode::Output(_) => {
                self.regs.toggle_pin();
                Ok(())
            }
            _ => Err(Error::InvalidPinType),
        }
    }
    #[inline]
    fn _read_out(&self) -> Result<bool, Error> {
        match self.mode {
            DYN_READABLE_OUTPUT => Ok(self.regs.read_out_pin()),
            _ => Err(Error::InvalidPinType),
        }
    }
    #[inline]
    fn _is_low(&self) -> Result<bool, Error> {
        Ok(self._read()? == false)
    }
    #[inline]
    fn _is_high(&self) -> Result<bool, Error> {
        Ok(self._read()? == true)
    }
    #[inline]
    fn _set_low(&mut self) -> Result<(), Error> {
        self._write(false)
    }
    #[inline]
    fn _set_high(&mut self) -> Result<(), Error> {
        self._write(true)
    }
    #[inline]
    fn _is_set_low(&self) -> Result<bool, Error> {
        Ok(self._read_out()? == false)
    }
    #[inline]
    fn _is_set_high(&self) -> Result<bool, Error> {
        Ok(self._read_out()? == true)
    }
}

//==============================================================================
//  Convert between Pin and DynPin
//==============================================================================

impl<I, M> From<Pin<I, M>> for DynPin
where
    I: PinId,
    M: PinMode,
{
    /// Erase the type-level information in a [`Pin`] and return a value-level
    /// [`DynPin`]
    #[inline]
    fn from(_pin: Pin<I, M>) -> Self {
        // The `Pin` is consumed, so it is safe to replace it with the
        // corresponding `DynPin`
        unsafe { DynPin::new(I::DYN, M::DYN) }
    }
}

impl<I, M> TryFrom<DynPin> for Pin<I, M>
where
    I: PinId,
    M: PinMode,
{
    type Error = Error;

    /// Try to recreate a type-level [`Pin`] from a value-level [`DynPin`]
    ///
    /// There is no way for the compiler to know if the conversion will be
    /// successful at compile-time. We must verify the conversion at run-time
    /// or refuse to perform it.
    #[inline]
    fn try_from(pin: DynPin) -> Result<Self, Error> {
        if pin.regs.id == I::DYN && pin.mode == M::DYN {
            // The `DynPin` is consumed, so it is safe to replace it with the
            // corresponding `Pin`
            Ok(unsafe { Self::new() })
        } else {
            Err(Error::InvalidPinType)
        }
    }
}

//==============================================================================
// Embedded HAL traits
//==============================================================================

impl OutputPin for DynPin {
    type Error = Error;
    #[inline]
    fn set_high(&mut self) -> Result<(), Self::Error> {
        self._set_high()
    }
    #[inline]
    fn set_low(&mut self) -> Result<(), Self::Error> {
        self._set_low()
    }
}

#[cfg(feature = "unproven")]
impl InputPin for DynPin {
    type Error = Error;
    #[inline]
    fn is_high(&self) -> Result<bool, Self::Error> {
        self._is_high()
    }
    #[inline]
    fn is_low(&self) -> Result<bool, Self::Error> {
        self._is_low()
    }
}

#[cfg(feature = "unproven")]
impl ToggleableOutputPin for DynPin {
    type Error = Error;
    #[inline]
    fn toggle(&mut self) -> Result<(), Self::Error> {
        self._toggle()
    }
}

#[cfg(feature = "unproven")]
impl StatefulOutputPin for DynPin {
    #[inline]
    fn is_set_high(&self) -> Result<bool, Self::Error> {
        self._is_set_high()
    }
    #[inline]
    fn is_set_low(&self) -> Result<bool, Self::Error> {
        self._is_set_low()
    }
}
