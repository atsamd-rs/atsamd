//! # Async interrupts
//!
//! This module provides APIs for working with interrupts, tailored towards
//! async peripherals.
//!
//! Asynchronous programming relies on tasks that can be paused and resumed
//! without blocking the entire program. When an async task is waiting for a
//! particular event, such as data from a peripheral, it enters a suspended
//! state. It is crucial that the task is properly woken up when the expected
//! event occurs to resume its execution.
//!
//! By having peripherals take interrupts, they can signal the occurrence of
//! relevant events, effectively waking up the associated async tasks. This
//! ensures that the async runtime can schedule and resume tasks in a timely
//! manner, providing the responsiveness required in embedded systems.
//!
//! ## Typelevel and enum-level interrupts
//!
//! There are two main ways of representing interrupts in the HAL: either by
//! using [`pac::Interrupt`], where each interrupt is represented as an enum
//! variant, or by using the typelevel interrupts defined in this module. Each
//! interrupt source *that is usable with async peripherals* is declared as a
//! struct with the same name of the corresponsing [`pac::Interrupt`] variant.
//! Therefore, two distinct traits are needed to perform basic tasks on
//! interrupt types:
//!
//! * Use [`Interrupt`] when dealing with the typelevel interrupt types defined
//!   in this module;
//! * Use [`InterruptExt`] when dealing with enum-level interrupt types defined
//!   in [`pac`].
//!
//! [`pac::Interrupt`]: crate::pac::Interrupt
//! [`Interrupt`]: crate::async_hal::interrupts::Interrupt
//! [`InterruptExt`]: crate::async_hal::interrupts::InterruptExt
//! [`pac`]: crate::pac

use crate::typelevel::Sealed;
use atsamd_hal_macros::{hal_cfg, hal_macro_helper};
use core::{
    mem,
    sync::atomic::{compiler_fence, Ordering},
};
use cortex_m::{interrupt::InterruptNumber, peripheral::NVIC};
use critical_section::CriticalSection;

/// Marker trait indicating that an interrupt source has one binding and
/// one handler.
///
/// May not be implemented outside of this HAL.
pub trait SingleInterruptSource: Sealed {}

/// Marker trait indicating that an interrupt source has multiple bindings and
/// handlers.
///
/// May not be implemented outside of this HAL.
pub trait MultipleInterruptSources: Sealed {}

macro_rules! declare_interrupts {
    ($($(#[$cfg:meta])* $irqs:ident),* $(,)?) => {
        $(
            $(#[$cfg])*
            #[allow(non_camel_case_types)]
            #[doc=stringify!($irqs)]
            #[doc=" typelevel interrupt."]
            pub enum $irqs {}

            $(#[$cfg])*
            impl $crate::typelevel::Sealed for $irqs{}

            $(#[$cfg])*
            impl $crate::async_hal::interrupts::Interrupt for $irqs {
                const IRQ: crate::pac::Interrupt = crate::pac::Interrupt::$irqs;
            }

            $(#[$cfg])*
            impl $crate::async_hal::interrupts::SingleInterruptSource for $irqs {}
        )*
    }
}

/// Useful when we need to bind multiple interrupt sources to the same handler.
/// Calling the `InterruptSource` methods on the created struct will act on all
/// interrupt sources at once.
// Lint allowed because the macro is unused for thumbv6 devices.
#[allow(unused_macros)]
macro_rules! declare_multiple_interrupts {
    ($(#[$cfg:meta])* $name:ident: [ $($irq:ident),+ $(,)? ]) => {
        ::paste::paste! {
            $(#[$cfg])*
            pub enum $name {}

            $(#[$cfg])*
            impl $crate::typelevel::Sealed for $name {}

            $(#[$cfg])*
            impl $crate::async_hal::interrupts::InterruptSource for $name {
                unsafe fn enable() {
                    $($crate::pac::Interrupt::$irq.enable();)+
                }

                fn disable() {
                    $($crate::pac::Interrupt::$irq.disable();)+
                }

                fn unpend() {
                    $($crate::pac::Interrupt::$irq.unpend();)+
                }

                fn set_priority(prio: $crate::async_hal::interrupts::Priority){
                    $($crate::pac::Interrupt::$irq.set_priority(prio);)+
                }
            }

            $(#[$cfg])*
            impl $crate::async_hal::interrupts::MultipleInterruptSources for $name {}
        }
    };
}

// ---------- DMAC Interrupts ---------- //
#[cfg(feature = "dma")]
#[hal_cfg("dmac-d5x")]
declare_multiple_interrupts!(DMAC: [DMAC_0, DMAC_1, DMAC_2, DMAC_OTHER]);

#[cfg(feature = "dma")]
#[hal_cfg(any("dmac-d11", "dmac-d21"))]
declare_interrupts!(DMAC);

// ----------  SERCOM Interrupts ---------- //
#[hal_cfg(any("sercom0-d11", "sercom0-d21"))]
declare_interrupts!(SERCOM0);

#[hal_cfg(any("sercom1-d11", "sercom1-d21"))]
declare_interrupts!(SERCOM1);

#[hal_cfg(any("sercom2-d11", "sercom2-d21"))]
declare_interrupts!(SERCOM2);

#[hal_cfg("sercom3-d21")]
declare_interrupts!(SERCOM3);

#[hal_cfg("sercom4-d21")]
declare_interrupts!(SERCOM4);

#[hal_cfg("sercom5-d21")]
declare_interrupts!(SERCOM5);

#[hal_cfg("sercom0-d5x")]
declare_multiple_interrupts!(SERCOM0: [SERCOM0_0, SERCOM0_1, SERCOM0_2, SERCOM0_OTHER ]);

#[hal_cfg("sercom1-d5x")]
declare_multiple_interrupts!(SERCOM1: [SERCOM1_0, SERCOM1_1, SERCOM1_2, SERCOM1_OTHER ]);

#[hal_cfg("sercom2-d5x")]
declare_multiple_interrupts!(SERCOM2: [SERCOM0_2, SERCOM2_1, SERCOM2_2, SERCOM2_OTHER ]);

#[hal_cfg("sercom3-d5x")]
declare_multiple_interrupts!(SERCOM3: [SERCOM3_0, SERCOM3_1, SERCOM3_2, SERCOM3_OTHER ]);

#[hal_cfg("sercom4-d5x")]
declare_multiple_interrupts!(SERCOM4: [SERCOM4_0, SERCOM4_1, SERCOM4_2, SERCOM4_OTHER ]);

#[hal_cfg("sercom5-d5x")]
declare_multiple_interrupts!(SERCOM5: [SERCOM5_0, SERCOM5_1, SERCOM5_2, SERCOM5_OTHER ]);

#[hal_cfg("sercom6-d5x")]
declare_multiple_interrupts!(SERCOM6: [SERCOM6_0, SERCOM6_1, SERCOM6_2, SERCOM6_OTHER ]);

#[hal_cfg("sercom7-d5x")]
declare_multiple_interrupts!(SERCOM7: [SERCOM7_0, SERCOM7_1, SERCOM7_2, SERCOM7_OTHER ]);

// ----------  TC Interrupts ---------- //

#[hal_cfg("tc0")]
declare_interrupts!(TC0);

#[hal_cfg("tc1")]
declare_interrupts!(TC1);

#[hal_cfg("tc2")]
declare_interrupts!(TC2);

#[hal_cfg("tc3")]
declare_interrupts!(TC3);

#[hal_cfg("tc4")]
declare_interrupts!(TC4);

#[hal_cfg("tc5")]
declare_interrupts!(TC5);

// ----------  EIC Interrupt ---------- //
#[hal_cfg(any("eic-d11", "eic-d21"))]
declare_interrupts!(EIC);

#[hal_cfg("eic-d5x")]
seq_macro::seq!(N in 0..= 15 {
    paste::paste! {
        declare_interrupts! {
            EIC_EXTINT_~N
        }

    }
});

/// An interrupt source that may have one or many interrupt bindings.
///
/// This trait may implemented directly when multiple interrupt sources are
/// needed to operate a single peripheral (eg, SERCOM and DMAC for thumbv7
/// devices). If using one interrupt source per peripheral,
/// implement [`Interrupt`] instead. When implemented on a type that handles
/// multiple interrupt sources, the methods will act on all interrupt sources at
/// once.
///
/// May not be implemented outside of this HAL.
pub trait InterruptSource: crate::typelevel::Sealed {
    /// Enable the interrupt.
    ///
    /// # Safety
    ///
    /// Do not enable any interrupt inside a critical section.
    unsafe fn enable();

    /// Disable the interrupt.
    fn disable();

    /// Unset interrupt pending.
    fn unpend();

    /// Set the interrupt priority.
    fn set_priority(prio: Priority);
}

impl<T: Interrupt> InterruptSource for T {
    unsafe fn enable() {
        Self::enable();
    }

    fn disable() {
        Self::disable();
    }

    fn unpend() {
        Self::unpend();
    }

    fn set_priority(prio: Priority) {
        Self::set_priority(prio);
    }
}

/// Type-level interrupt.
///
/// This trait is implemented for all typelevel single interrupt types defined
/// in this module. May not be implemented outside of this HAL.
pub trait Interrupt: crate::typelevel::Sealed {
    /// Interrupt enum variant.
    ///
    /// This allows going from typelevel interrupts (one type per interrupt,
    /// defined in [`this module`](self)) to non-typelevel interrupts (a
    /// single [`Interrupt`](crate::pac::Interrupt) enum type, with one
    /// variant per interrupt).
    const IRQ: crate::pac::Interrupt;

    /// Enable the interrupt.
    ///
    /// # Safety
    ///
    /// Do not enable any interrupt inside a critical section.
    #[inline]
    unsafe fn enable() {
        Self::IRQ.enable()
    }

    /// Disable the interrupt.
    #[inline]
    fn disable() {
        Self::IRQ.disable()
    }

    /// Check if interrupt is enabled.
    #[inline]
    fn is_enabled() -> bool {
        Self::IRQ.is_enabled()
    }

    /// Check if interrupt is pending.
    #[inline]
    fn is_pending() -> bool {
        Self::IRQ.is_pending()
    }

    /// Set interrupt pending.
    #[inline]
    fn pend() {
        Self::IRQ.pend()
    }

    /// Unset interrupt pending.
    #[inline]
    fn unpend() {
        Self::IRQ.unpend()
    }

    /// Get the priority of the interrupt.
    #[inline]
    fn get_priority() -> Priority {
        Self::IRQ.get_priority()
    }

    /// Set the interrupt priority.
    #[inline]
    fn set_priority(prio: Priority) {
        Self::IRQ.set_priority(prio)
    }

    /// Set the interrupt priority with an already-acquired critical section.
    ///
    /// Equivalent to [`set_priority`](Self::set_priority), except you pass a
    /// [`CriticalSection`] to prove you've already acquired a critical
    /// section. This prevents acquiring another one, which saves code size.
    #[inline]
    fn set_priority_with_cs(cs: critical_section::CriticalSection, prio: Priority) {
        Self::IRQ.set_priority_with_cs(cs, prio)
    }
}

/// Interrupt handler.
///
/// Drivers that need to handle interrupts implement this trait.
/// The user must ensure `on_interrupt()` is called every time the interrupt
/// fires. Drivers must use use [`Binding`] to assert at compile time that the
/// user has done so.
pub trait Handler<I: InterruptSource>: Sealed {
    /// Interrupt handler function.
    ///
    /// Must be called every time the `I` interrupt fires, synchronously from
    /// the interrupt handler context.
    ///
    /// # Safety
    ///
    /// This function must ONLY be called from the interrupt handler for `I`.
    unsafe fn on_interrupt();
}

/// Compile-time assertion that an interrupt has been bound to a handler.
///
/// For the vast majority of cases, you should use the `bind_interrupts!`
/// macro instead of writing `unsafe impl`s of this trait.
///
/// # Safety
///
/// By implementing this trait, you are asserting that you have arranged for
/// `H::on_interrupt()` to be called every time the `I` interrupt fires.
///
/// This allows drivers to check bindings at compile-time.
pub unsafe trait Binding<I: InterruptSource, H: Handler<I>> {}

/// An interrupt type that can be configured by the HAL to handle
/// interrupts.
///
/// The PAC defined enum-level interrupts implement this trait.
pub trait InterruptExt: InterruptNumber + Copy {
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
    #[inline]
    fn set_priority_with_cs(self, _cs: CriticalSection, prio: Priority) {
        unsafe {
            let mut nvic = steal_nvic();
            nvic.set_priority(self, prio.logical2hw());
        }
    }
}

impl<T: InterruptNumber + Copy> InterruptExt for T {}

#[hal_cfg(any("nvic-d11", "nvic-d21"))]
const NVIC_PRIO_BITS: u8 = 2;
#[hal_cfg("nvic-d5x")]
const NVIC_PRIO_BITS: u8 = 3;

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
        unsafe { mem::transmute((1 << NVIC_PRIO_BITS) - (prio >> (8 - NVIC_PRIO_BITS))) }
    }
}

unsafe fn steal_nvic() -> NVIC {
    cortex_m::peripheral::Peripherals::steal().NVIC
}
