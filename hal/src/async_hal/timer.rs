use crate::{
    ehal::timer::CountDown, time::Nanoseconds, timer_traits::InterruptDrivenTimer,
    typelevel::Sealed,
};
use atomic_polyfill::AtomicBool;
use core::{
    sync::atomic::Ordering,
    task::{Poll, Waker},
};
use embassy::{interrupt::InterruptExt, waitqueue::AtomicWaker};
use futures::future::poll_fn;

#[cfg(any(feature = "samd11", feature = "samd21"))]
use crate::thumbv6m::timer;

#[cfg(feature = "min-samd51g")]
use crate::thumbv7em::timer;

#[cfg(feature = "samd11")]
use crate::pac::TC1;
#[cfg(feature = "samd21")]
use crate::pac::{TC3, TC4, TC5};

#[cfg(feature = "min-samd51g")]
use crate::pac::{TC2, TC3};
// Only the G variants are missing these timers
#[cfg(feature = "min-samd51j")]
use crate::pac::{TC4, TC5};

use timer::{Count16, TimerCounter};

#[doc(hidden)]
pub trait AsyncCount16: Count16 + Sealed {
    /// Interrupt handler for this TC
    type Interrupt: ::embassy::interrupt::InterruptExt;

    /// Index of this TC in the `STATE` tracker
    const STATE_ID: usize;

    /// Callback function when the corresponding TC interrupt is fired
    ///
    /// # Safety
    ///
    /// This method may [`steal`](crate::pac::Peripherals::steal) the `TC`
    /// peripheral instance to check the interrupt flags. The only
    /// modifications it is allowed to apply to the peripheral is to clear
    /// the interrupt flag (to prevent re-firing). This method should ONLY be
    /// able to be called while an [`AsyncTimer`] holds an unique reference
    /// to the underlying `TC` peripheral.
    unsafe fn on_interrupt(_: *mut ());
}

macro_rules! impl_async_count16 {
        ($(($TC: ident, $id: expr)),+) => {
            $(
                impl AsyncCount16 for $TC {
                    type Interrupt = crate::interrupt::$TC;
                    const STATE_ID: usize = $id;

                    unsafe fn on_interrupt(_: *mut ()) {
                        let tc = crate::pac::Peripherals::steal().$TC;
                        let intflag = &tc.count_16().intflag;

                        if intflag.read().ovf().bit_is_set() {
                            // Clear the flag
                            intflag.modify(|_, w| w.ovf().set_bit());
                            STATE[Self::STATE_ID].wake();
                        }
                    }
                }

                impl Sealed for $TC {}
            )+
        };
    }

#[cfg(feature = "samd11")]
impl_async_count16!((TC1, 0));
#[cfg(feature = "samd11")]
const NUM_TIMERS: usize = 1;

#[cfg(feature = "samd21")]
impl_async_count16!((TC3, 0), (TC4, 1), (TC5, 2));
#[cfg(feature = "samd21")]
const NUM_TIMERS: usize = 3;

#[cfg(feature = "samd51g")]
impl_async_count16!((TC2, 0), (TC3, 1));
#[cfg(feature = "samd51g")]
const NUM_TIMERS: usize = 2;

#[cfg(feature = "min-samd51j")]
impl_async_count16!((TC2, 0), (TC3, 1), (TC4, 2), (TC5, 3));
#[cfg(feature = "min-samd51j")]
const NUM_TIMERS: usize = 4;

impl<T: AsyncCount16> TimerCounter<T> {
    /// Transform a [`TimerCounter`] into an [`AsyncTimer`]
    #[inline]
    pub fn as_async<'a, 'self_mut: 'a>(
        &'self_mut mut self,
        irq: &'a mut T::Interrupt,
    ) -> AsyncTimer<'a, T> {
        irq.set_handler(T::on_interrupt);
        irq.enable();
        self.enable_interrupt();

        AsyncTimer { timer: self, irq }
    }
}

/// Wrapper around a [`TimerCounter`] with an `async` interface
pub struct AsyncTimer<'a, T: AsyncCount16> {
    timer: &'a mut TimerCounter<T>,
    irq: &'a mut T::Interrupt,
}

impl<'a, T: AsyncCount16> AsyncTimer<'a, T> {
    /// Delay asynchronously
    #[inline]
    pub async fn delay(&mut self, count: impl Into<Nanoseconds>) {
        self.timer.start(count);
        self.timer.enable_interrupt();

        poll_fn(|cx| {
            STATE[T::STATE_ID].register(cx.waker());
            if STATE[T::STATE_ID].ready() {
                return Poll::Ready(());
            }

            Poll::Pending
        })
        .await;
    }
}

impl<'a, T: AsyncCount16> Drop for AsyncTimer<'a, T> {
    #[inline]
    fn drop(&mut self) {
        self.irq.remove_handler();
        self.irq.disable();
    }
}

// TODO instead of tracking the state manually, we could use ONESHOT
// mode and check the STATUS.STOP bit
struct State {
    waker: AtomicWaker,
    ready: AtomicBool,
}

impl State {
    #[inline]
    const fn new() -> Self {
        Self {
            waker: AtomicWaker::new(),
            ready: AtomicBool::new(false),
        }
    }

    #[inline]
    fn register(&self, waker: &Waker) {
        self.waker.register(waker)
    }

    #[inline]
    fn wake(&self) {
        self.ready.store(true, Ordering::SeqCst);
        self.waker.wake()
    }

    #[inline]
    fn ready(&self) -> bool {
        self.ready.swap(false, Ordering::SeqCst)
    }
}

#[allow(clippy::declare_interior_mutable_const)]
const STATE_NEW: State = State::new();
static STATE: [State; NUM_TIMERS] = [STATE_NEW; NUM_TIMERS];
