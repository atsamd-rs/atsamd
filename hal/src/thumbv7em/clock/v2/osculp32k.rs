//! # Osculp32k - Ultra Low power 32 kHz oscillator
//!
//! Always-on internal oscillator capable of producing
//! 32 kHz and 1 kHz output.
//!
//! * 1 kHz signal is only accessible by specific peripherals (datasheet c.
//!   13.1)
//! * 32 kHz signal is a general purpose source that can be used by any
//! [`Gclk`](crate::clock::v2::gclk::Gclk).
//!
//! Independently controllable default-on outputs for 32 kHz and 1 kHz
//!
//! See:
//! * [`Enabled<Osculp32k>::activate_32k`]
//! * [`Enabled<Osculp32k>::activate_1k`]

use typenum::U0;

use crate::pac::osc32kctrl::OSCULP32K;

use crate::time::{Hertz, U32Ext};
use crate::typelevel::{Counter, Sealed};

use super::{Driver, Enabled};

//==============================================================================
// OscUlp32kToken
//==============================================================================

/// Token struct that is essential in order to construct an instance of a
/// [`OscUlp32k`].
///
/// Irrelevant to the user as
/// * [`Enabled`]`<`[`OscUlp32k`]`>` is undisableable and undeconstructable
/// * [`Enabled`]`<`[`OscUlp32k`]`>` is already provided within a tuple as
///   return value from [`retrieve_clocks`](crate::clock::v2::retrieve_clocks)
///   and one does not have to create it
pub struct OscUlp32kToken {
    __: (),
}

impl OscUlp32kToken {
    /// Create a new instance of [`OscUlp32kToken`]
    #[inline]
    pub(crate) unsafe fn new() -> Self {
        Self { __: () }
    }

    #[inline]
    fn osculp32k(&self) -> &OSCULP32K {
        unsafe { &(*crate::pac::OSC32KCTRL::ptr()).osculp32k }
    }

    #[inline]
    fn set_calib(&mut self, calib: u8) {
        self.osculp32k()
            .modify(|_, w| unsafe { w.calib().bits(calib & 0x3F) });
    }

    #[inline]
    pub(super) fn activate_1k(&mut self, enabled: bool) {
        self.osculp32k().modify(|_, w| w.en1k().bit(enabled));
    }

    #[inline]
    pub(super) fn activate_32k(&mut self, enabled: bool) {
        self.osculp32k().modify(|_, w| w.en32k().bit(enabled));
    }

    #[allow(dead_code)]
    #[inline]
    fn wrtlock(&mut self) {
        self.osculp32k().modify(|_, w| w.wrtlock().bit(true));
    }
}

//==============================================================================
// OscUlp32k
//==============================================================================

/// Struct representing a disabled ultra low power oscillator
///
/// In reality, this oscillator is always enabled and
/// [`Enabled`]`<`[`OscUlp32k`]`>` does not have a `disable` method that would
/// allow to retreive underlying [`OscUlp32k`] struct.
///
/// User can only activate/deactivate outgoing signals (as in making them
/// visible outside of an oscillator).
/// Therefore [`Enabled`]`<`[`OscUlp32k`]`<`[`Inactive32k`]`, `[`Inactive1k`]`>,
/// _>` represents an **always** enabled oscillator with disabled outgoing
/// signals.
///
/// It is generic over:
/// - An output state of a 32 kHz signal (active/inactive)
/// - An output state of a 1 kHz signal (active/inactive)
pub struct OscUlp32k {
    pub(super) token: OscUlp32kToken,
}

impl OscUlp32k {
    /// Create a new instance of [`OscUlp32k`]
    #[inline]
    pub(crate) fn new(token: OscUlp32kToken) -> Self {
        Self { token }
    }
}

impl Enabled<OscUlp32k, U0> {
    /// Set calibration parameters
    ///
    /// This data is used to compensate for process variations
    ///
    /// This method performs a HW register write. At startup, this register is
    /// populated with a default parametrization from Flash Calibration at
    /// startup
    #[inline]
    pub fn set_calibration(mut self, calib: u8) -> Self {
        self.0.token.set_calib(calib);
        self
    }
}

//==============================================================================
// OscUlp32kId
//==============================================================================

/// Type-level variant representing the identity of the OSCULP32K clock
///
/// This type is a member of several [type-level enums]. See the documentation
/// on [type-level enums] for more details on the pattern.
///
/// [type-level enums]: crate::typelevel#type-level-enum
pub enum OscUlp32kId {}

impl Sealed for OscUlp32kId {}

//==============================================================================
// Driver
//==============================================================================

impl<N: Counter> Driver for Enabled<OscUlp32k, N> {
    type Source = OscUlp32kId;

    #[inline]
    fn freq(&self) -> Hertz {
        32768.hz()
    }
}
