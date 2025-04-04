//! Primitives for manipulating interrupts

use core::sync::atomic::{compiler_fence, Ordering};

use crate::pac::NVIC;
use atsamd_hal_macros::{hal_cfg, hal_macro_helper};

pub use crate::pac::NVIC_PRIO_BITS;

/// Logical interrupt priority level.
///
/// P4 is the most urgent, and P1 is the least urgent priority.
#[hal_cfg(any("nvic-d11", "nvic-d21"))]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u8)]
#[allow(missing_docs)]
pub enum Priority {
    P1 = 1,
    P2 = 2,
    P3 = 3,
    P4 = 4,
}

/// Logical interrupt priority level.
///
/// P8 is the most urgent, and P1 is the least urgent priority.
#[hal_cfg("nvic-d5x")]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u8)]
#[allow(missing_docs)]
pub enum Priority {
    P1 = 1,
    P2 = 2,
    P3 = 3,
    P4 = 4,
    P5 = 5,
    P6 = 6,
    P7 = 7,
    P8 = 8,
}

impl Priority {
    /// Creates the `Priority` from a numeric priority if possible.
    pub const fn from_numeric(prio: u8) -> Option<Self> {
        if prio >= 1 && prio <= 8 {
            Some(unsafe { core::mem::transmute::<u8, Self>(prio) })
        } else {
            None
        }
    }

    /// Convert a logical priority (where higher priority number = higher
    /// priority level) to a hardware priority level (where lower priority
    /// number = higher priority level).
    ///
    /// Taken from [`cortex-m-interrupt`]
    ///
    /// See LICENSE-MIT for the license.
    ///
    /// [`cortex-m-interrupt`]: https://github.com/datdenkikniet/cortex-m-interrupt
    #[inline]
    #[must_use]
    pub const fn logical2hw(self) -> u8 {
        ((1 << NVIC_PRIO_BITS) - self as u8) << (8 - NVIC_PRIO_BITS)
    }

    /// Convert a hardware priority level (where lower priority number = higher
    /// priority level) to a logical priority (where a higher priority number =
    /// higher priority level).
    ///
    /// # Panics
    ///
    /// This method may only be used with allowed hardware priority levels. Ie,
    /// * 0x00,
    /// * 0x20,
    /// * 0x40,
    /// * and so on.
    ///
    /// Any other value will cause a panic. To save yourself some
    /// trouble, use this method only with hardware priority values gotten
    /// directly from the NVIC.
    #[inline]
    #[must_use]
    pub const fn hw2logical(prio: u8) -> Self {
        assert!(prio % 0x20 == 0);
        unsafe { core::mem::transmute((1 << NVIC_PRIO_BITS) - (prio >> (8 - NVIC_PRIO_BITS))) }
    }
}

/// An interrupt type that can be configured by the HAL to handle
/// interrupts.
///
/// The PAC defined enum-level interrupts implement this trait.
pub trait InterruptExt: cortex_m::interrupt::InterruptNumber + Copy {
    /// Enable the interrupt.
    ///
    /// # Safety
    ///
    /// Do not enable any interrupt inside a critical section.
    #[inline]
    unsafe fn enable(self) {
        compiler_fence(Ordering::SeqCst);
        NVIC::unmask(self)
    }

    /// Disable the interrupt.
    #[inline]
    fn disable(self) {
        NVIC::mask(self);
        compiler_fence(Ordering::SeqCst);
    }

    /// Check if interrupt is enabled.
    #[inline]
    fn is_enabled(self) -> bool {
        NVIC::is_enabled(self)
    }

    /// Check if interrupt is pending.
    #[inline]
    fn is_pending(self) -> bool {
        NVIC::is_pending(self)
    }

    /// Set interrupt pending.
    #[inline]
    fn pend(self) {
        NVIC::pend(self)
    }

    /// Unset interrupt pending.
    #[inline]
    fn unpend(self) {
        NVIC::unpend(self)
    }

    /// Get the priority of the interrupt.
    #[inline]
    fn get_priority(self) -> Priority {
        Priority::hw2logical(NVIC::get_priority(self))
    }

    /// Set the interrupt priority.
    #[inline]
    #[hal_macro_helper]
    fn set_priority(self, prio: Priority) {
        unsafe {
            let mut nvic = steal_nvic();

            // On thumbv6, set_priority must do a RMW to change 8bit in a 32bit reg.
            #[hal_cfg(any("nvic-d11", "nvic-d21"))]
            critical_section::with(|_| nvic.set_priority(self, prio.logical2hw()));
            // On thumbv7+, set_priority does an atomic 8bit write, so no CS needed.
            #[hal_cfg("nvic-d5x")]
            nvic.set_priority(self, prio.logical2hw());
        }
    }

    /// Set the interrupt priority with an already-acquired critical section.
    ///
    /// Equivalent to [`set_priority`](Self::set_priority), except you pass a
    /// [`CriticalSection`] to prove you've already acquired a critical
    /// section. This prevents acquiring another one, which saves code size.
    ///
    /// [`CriticalSection`]: critical_section::CriticalSection
    #[inline]
    fn set_priority_with_cs(self, _cs: critical_section::CriticalSection, prio: Priority) {
        unsafe {
            let mut nvic = steal_nvic();
            nvic.set_priority(self, prio.logical2hw());
        }
    }
}

impl<T: cortex_m::interrupt::InterruptNumber + Copy> InterruptExt for T {}

unsafe fn steal_nvic() -> NVIC {
    cortex_m::peripheral::Peripherals::steal().NVIC
}
