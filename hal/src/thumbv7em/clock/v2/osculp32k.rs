//! # Osculp32k - Ultra Low power 32 kHz oscillator

#![allow(missing_docs)]

use typenum::U0;

use crate::pac::osc32kctrl::OSCULP32K;

use crate::time::Hertz;
use crate::typelevel::{Counter, Decrement, Increment, PrivateDecrement, PrivateIncrement, Sealed};

use super::{Enabled, Source};

//==============================================================================
// Tokens
//==============================================================================

pub struct OscUlpBaseToken(());

pub struct OscUlp1kToken(());

pub struct OscUlp32kToken(());

pub struct Tokens {
    pub osculp1k: OscUlp1kToken,
    pub osculp32k: OscUlp32kToken,
}

impl Tokens {
    /// Create a new set of tokens
    ///
    /// Safety: There must never be more than one instance of a token at any
    /// given time.
    pub(super) unsafe fn new() -> Self {
        Self {
            osculp1k: OscUlp1kToken(()),
            osculp32k: OscUlp32kToken(()),
        }
    }
}

impl OscUlpBaseToken {
    #[inline]
    fn osculp32k(&self) -> &OSCULP32K {
        unsafe { &(*crate::pac::OSC32KCTRL::ptr()).osculp32k }
    }

    #[inline]
    fn set_calibration(&mut self, calib: u8) {
        self.osculp32k()
            .modify(|_, w| unsafe { w.calib().bits(calib) });
    }

    #[inline]
    fn enable_1k(&mut self, enabled: bool) {
        self.osculp32k().modify(|_, w| w.en1k().bit(enabled));
    }

    #[inline]
    fn enable_32k(&mut self, enabled: bool) {
        self.osculp32k().modify(|_, w| w.en32k().bit(enabled));
    }

    #[inline]
    fn wrtlock(&mut self) {
        self.osculp32k().modify(|_, w| w.wrtlock().bit(true));
    }
}

//==============================================================================
// OscUlpBase
//==============================================================================

pub struct OscUlpBase {
    token: OscUlpBaseToken,
}

pub type EnabledOscUlpBase<N = U0> = Enabled<OscUlpBase, N>;

impl OscUlpBase {
    /// Create the ultra-low power base oscillator
    ///
    /// Safety: There must never be more than one instance of this struct at any
    /// given time.
    #[inline]
    pub(super) unsafe fn new() -> EnabledOscUlpBase {
        let token = OscUlpBaseToken(());
        Enabled::new(Self { token })
    }
}

impl<N: Counter> EnabledOscUlpBase<N> {
    /// Override the factory-default calibration value
    #[inline]
    pub fn set_calibration(&mut self, calib: u8) {
        self.0.token.set_calibration(calib);
    }

    /// Set the write-lock, which will last until POR
    ///
    /// This function sets the write-lock bit, which lasts until power-on reset.
    /// It also consumes and drops the [`XoscBase`], which destroys API access
    /// to the registers.
    #[inline]
    pub fn write_lock(mut self) {
        self.0.token.wrtlock();
    }
}

//==============================================================================
// Ids
//==============================================================================

/// Type-level variant representing the identity of the OSCULP1K clock
///
/// This type is a member of several [type-level enums]. See the documentation
/// on [type-level enums] for more details on the pattern.
///
/// [type-level enums]: crate::typelevel#type-level-enum
pub enum OscUlp1kId {}

impl Sealed for OscUlp1kId {}

/// Type-level variant representing the identity of the OSCULP32K clock
///
/// This type is a member of several [type-level enums]. See the documentation
/// on [type-level enums] for more details on the pattern.
///
/// [type-level enums]: crate::typelevel#type-level-enum
pub enum OscUlp32kId {}

impl Sealed for OscUlp32kId {}

//==============================================================================
// OscUlp1k
//==============================================================================

pub struct OscUlp1k {
    token: OscUlp1kToken,
}

pub type EnabledOscUlp1k<N = U0> = Enabled<OscUlp1k, N>;

impl OscUlp1k {
    /// Enable the 1 kHz output from OSCULP32K
    ///
    /// This clock is derived from the [`Enabled`] [`OscUlpBase`] clock.
    ///
    /// ```
    /// let token = tokens.osculp.osculp1k;
    /// let (osculp1k, osculp) = OscUlp1k::enable(token, osculp);
    /// ```
    #[inline]
    pub fn enable<N: Increment>(
        token: OscUlp1kToken,
        mut base: EnabledOscUlpBase<N>,
    ) -> (EnabledOscUlp1k, EnabledOscUlpBase<N::Inc>) {
        base.0.token.enable_1k(true);
        (Enabled::new(Self { token }), base.inc())
    }
}

impl EnabledOscUlp1k {
    /// Disable the 1 kHz output from OSCULP32K
    ///
    /// Doing so will clear one usage of the [`Enabled`] [`OscUlpBase`] clock
    #[inline]
    pub fn disable<N: Decrement>(
        self,
        mut base: EnabledOscUlpBase<N>,
    ) -> (OscUlp1kToken, EnabledOscUlpBase<N::Dec>) {
        base.0.token.enable_1k(false);
        (self.0.token, base.dec())
    }
}

impl<N: Counter> Source for EnabledOscUlp1k<N> {
    type Id = OscUlp1kId;

    fn freq(&self) -> Hertz {
        Hertz(1024)
    }
}

//==============================================================================
// OscUlp32k
//==============================================================================

pub struct OscUlp32k {
    token: OscUlp32kToken,
}

pub type EnabledOscUlp32k<N = U0> = Enabled<OscUlp32k, N>;

impl OscUlp32k {
    /// Enable the 32 kHz output from OSCULP32K
    ///
    /// This clock is derived from the [`Enabled`] [`OscUlpBase`] clock.
    ///
    /// ```
    /// let token = tokens.osculp.osculp32k;
    /// let (osculp1k, osculp) = OscUlp1k::enable(token, osculp);
    /// ```
    #[inline]
    pub fn enable<N: Increment>(
        token: OscUlp32kToken,
        mut base: EnabledOscUlpBase<N>,
    ) -> (EnabledOscUlp32k, EnabledOscUlpBase<N::Inc>) {
        base.0.token.enable_32k(true);
        (Enabled::new(Self { token }), base.inc())
    }
}

impl EnabledOscUlp32k {
    /// Disable the 32 kHz output from OSCULP32K
    ///
    /// Doing so will clear one usage of the [`Enabled`] [`OscUlpBase`] clock
    #[inline]
    pub fn disable<N: Decrement>(
        self,
        mut base: EnabledOscUlpBase<N>,
    ) -> (OscUlp32kToken, EnabledOscUlpBase<N::Dec>) {
        base.0.token.enable_32k(false);
        (self.0.token, base.dec())
    }
}

impl<N: Counter> Source for EnabledOscUlp32k<N> {
    type Id = OscUlp32kId;

    fn freq(&self) -> Hertz {
        Hertz(32_768)
    }
}
