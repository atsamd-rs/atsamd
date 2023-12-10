use core::mem;
use core::sync::atomic::{compiler_fence, Ordering};
use cortex_m::interrupt::InterruptNumber;
use cortex_m::peripheral::NVIC;
use critical_section::CriticalSection;
use paste::paste;
use seq_macro::seq;

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
        )*
    }
}

// Useful when we need to bind multiple interrupt sources to the same handler.
// Calling the `InterruptSource` methods on the created struct will act on all
// interrupt sources at once.
#[allow(unused_macros)]
macro_rules! declare_multiple_interrupts {
    ($(#[$cfg:meta])* $name:ident: [ $($irq:ident),+ $(,)? ]) => {
        paste! {
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
        }
    };
}

// ---------- DMAC Interrupts ---------- //
#[cfg(all(feature = "dma", feature = "thumbv7"))]
declare_multiple_interrupts!(DMAC: [DMAC_0, DMAC_1, DMAC_2, DMAC_OTHER]);

#[cfg(all(feature = "dma", feature = "thumbv7"))]
declare_interrupts!(DMAC_OTHER);

#[cfg(all(feature = "dma", feature = "thumbv6"))]
declare_interrupts!(DMAC);

// ----------  SERCOM Interrupts ---------- //
seq!(N in 0..=7 {
    paste! {
        #[cfg(all(feature = "has-" sercom~N, feature = "thumbv6"))]
        declare_interrupts!(SERCOM~N);
        #[cfg(all(feature = "has-" sercom~N, feature = "thumbv7"))]
        declare_multiple_interrupts!([<SERCOM ~N>]: [ [<SERCOM ~N _0>], [<SERCOM ~N _1>], [<SERCOM ~N _2>], [<SERCOM ~N _OTHER>] ]);
    }
});

// ----------  TC Interrupts ---------- //
seq!(N in 0..=5{
    paste! {
        declare_interrupts! {
            #[cfg(feature = "has-" tc~N)]
            TC~N
        }
    }
});

// ----------  EIC Interrupt ---------- //
#[cfg(feature = "thumbv6")]
declare_interrupts!(EIC);

#[cfg(feature = "thumbv7")]
seq!(N in 0..= 15 {
    paste! {
        declare_interrupts! {
            EIC_EXTINT_~N
        }

    }
});

/// Interrupt source. This trait may implemented directly when multiple
/// interrupt sources are needed to operate a single peripheral (eg, SERCOM and
/// DMAC for thumbv7 devices). If using one interrupt source per peripheral,
/// implement [`Interrupt`] instead. When implemented on a type that handles
/// multiple interrupt sources, the methods will act on all interrupt sources at
/// once.
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
/// This trait is implemented for all typelevel single interrupt types in this
/// module.
pub trait Interrupt: crate::typelevel::Sealed {
    /// Interrupt enum variant.
    ///
    /// This allows going from typelevel interrupts (one type per interrupt) to
    /// non-typelevel interrupts (a single `Interrupt` enum type, with one
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

    /// Set the interrupt priority with an already-acquired critical section
    #[inline]
    fn set_priority_with_cs(cs: critical_section::CriticalSection, prio: Priority) {
        Self::IRQ.set_priority_with_cs(cs, prio)
    }
}

/// Interrupt handler trait.
///
/// Drivers that need to handle interrupts implement this trait.
/// The user must ensure `on_interrupt()` is called every time the interrupt
/// fires. Drivers must use use [`Binding`] to assert at compile time that the
/// user has done so.
pub trait Handler<I: InterruptSource> {
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

/// Represents an interrupt type that can be configured by the HAL to handle
/// interrupts.
pub unsafe trait InterruptExt: InterruptNumber + Copy {
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

    /// Check if interrupt is being handled.
    #[inline]
    #[cfg(not(feature = "thumbv6"))]
    fn is_active(self) -> bool {
        NVIC::is_active(self)
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
        Priority::from(NVIC::get_priority(self))
    }

    /// Set the interrupt priority.
    #[inline]
    fn set_priority(self, prio: Priority) {
        unsafe {
            let mut nvic = steal_nvic();

            // On thumbv6, set_priority must do a RMW to change 8bit in a 32bit reg.
            #[cfg(feature = "thumbv6")]
            critical_section::with(|_| nvic.set_priority(self, prio.into()));
            // On thumbv7+, set_priority does an atomic 8bit write, so no CS needed.
            #[cfg(not(feature = "thumbv6"))]
            nvic.set_priority(self, prio.into());
        }
    }

    /// Set the interrupt priority with an already-acquired critical section
    ///
    /// Equivalent to `set_priority`, except you pass a `CriticalSection` to
    /// prove you've already acquired a critical section. This prevents
    /// acquiring another one, which saves code size.
    #[inline]
    fn set_priority_with_cs(self, _cs: CriticalSection, prio: Priority) {
        unsafe {
            let mut nvic = steal_nvic();
            nvic.set_priority(self, prio.into());
        }
    }
}

unsafe impl<T: InterruptNumber + Copy> InterruptExt for T {}

impl From<u8> for Priority {
    fn from(priority: u8) -> Self {
        unsafe { mem::transmute(priority & PRIO_MASK) }
    }
}

impl From<Priority> for u8 {
    fn from(p: Priority) -> Self {
        p as u8
    }
}

#[cfg(feature = "thumbv6")]
const PRIO_MASK: u8 = 0x60;
#[cfg(feature = "thumbv7")]
const PRIO_MASK: u8 = 0xe0;

/// The interrupt priority level.
///
/// P0 represents the most urgent prioriry, whereas P7 represents the least
/// urgent.
#[cfg(feature = "thumbv6")]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u8)]
#[allow(missing_docs)]
pub enum Priority {
    P0 = 0x0,
    P1 = 0x20,
    P2 = 0x40,
    P3 = 0x60,
}

/// The interrupt priority level.
///
/// P0 represents the most urgent prioriry, whereas P7 represents the least
/// urgent.
#[cfg(feature = "thumbv7")]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u8)]
#[allow(missing_docs)]
pub enum Priority {
    P0 = 0x0,
    P1 = 0x20,
    P2 = 0x40,
    P3 = 0x60,
    P4 = 0x80,
    P5 = 0xa0,
    P6 = 0xc0,
    P7 = 0xe0,
}

unsafe fn steal_nvic() -> NVIC {
    cortex_m::peripheral::Peripherals::steal().NVIC
}
