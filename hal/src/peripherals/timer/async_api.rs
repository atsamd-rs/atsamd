//! Async APIs for timers.
//!
//! Use [`TimerCounter::into_future`] to convert a regular [`TimerCounter`] into
//! an asynchronous [`TimerFuture`].

use crate::{
    async_hal::interrupts::{Binding, Handler, Interrupt},
    pac,
    time::Nanoseconds,
    timer_traits::InterruptDrivenTimer,
    typelevel::Sealed,
};
use atsamd_hal_macros::hal_cfg;
use core::{
    future::poll_fn,
    marker::PhantomData,
    sync::atomic::Ordering,
    task::{Poll, Waker},
};
use embassy_sync::waitqueue::AtomicWaker;
use portable_atomic::AtomicBool;

use crate::peripherals::timer;

#[hal_cfg("tc1")]
#[allow(unused_imports)]
use crate::pac::Tc1;

#[hal_cfg("tc2")]
#[allow(unused_imports)]
use crate::pac::Tc2;

#[hal_cfg("tc3")]
#[allow(unused_imports)]
use crate::pac::Tc3;

#[hal_cfg("tc4")]
#[allow(unused_imports)]
use crate::pac::Tc4;

#[hal_cfg("tc5")]
#[allow(unused_imports)]
use crate::pac::Tc5;

use timer::{Count16, TimerCounter};

#[hal_cfg("tc1-d11")]
type RegBlock = pac::tc1::RegisterBlock;

#[hal_cfg("tc3-d21")]
type RegBlock = pac::tc3::RegisterBlock;

#[hal_cfg("tc1-d5x")]
type RegBlock = pac::tc0::RegisterBlock;

/// Trait enabling the use of a Timer/Counter in async mode. Specifically, this
/// trait enables us to register a `TC*` interrupt as a waker for timer futures.
///
/// **⚠️ Warning** This trait should not be implemented outside of this crate!
pub trait AsyncCount16: Count16 + Sealed {
    /// Index of this TC in the `STATE` tracker
    const STATE_ID: usize;

    /// Get a reference to the timer's register block
    fn reg_block(peripherals: &pac::Peripherals) -> &RegBlock;

    /// Interrupt type for this timer
    type Interrupt: Interrupt;
}

/// Interrupt handler for async timer operarions
pub struct InterruptHandler<T: AsyncCount16> {
    _private: (),
    _tc: PhantomData<T>,
}

impl<T: AsyncCount16> crate::typelevel::Sealed for InterruptHandler<T> {}

impl<A: AsyncCount16> Handler<A::Interrupt> for InterruptHandler<A> {
    /// Callback function when the corresponding TC interrupt is fired
    ///
    /// # Safety
    ///
    /// This method may [`steal`](crate::pac::Peripherals::steal) the `TC`
    /// peripheral instance to check the interrupt flags. The only
    /// modifications it is allowed to apply to the peripheral is to clear
    /// the interrupt flag (to prevent re-firing). This method should ONLY be
    /// able to be called while a [`TimerFuture`] holds an unique reference
    /// to the underlying `TC` peripheral.
    unsafe fn on_interrupt() {
        let periph = unsafe { crate::pac::Peripherals::steal() };
        let tc = A::reg_block(&periph);
        let intflag = &tc.count16().intflag();

        if intflag.read().ovf().bit_is_set() {
            // Clear the flag
            intflag.modify(|_, w| w.ovf().set_bit());
            STATE[A::STATE_ID].wake();
        }
    }
}

macro_rules! impl_async_count16 {
    ($TC: ident, $id: expr) => {
        paste::paste! {
            impl AsyncCount16 for $TC {
                const STATE_ID: usize = $id;

                type Interrupt = crate::async_hal::interrupts::[< $TC:upper >];

                fn reg_block(peripherals: &pac::Peripherals) -> &RegBlock {
                    &*peripherals.[< $TC:lower >]
                }
            }

            impl Sealed for $TC {}
        }
    };
}

#[hal_cfg("tc1-d11")]
impl_async_count16!(Tc1, 0);

#[hal_cfg("tc3-d21")]
impl_async_count16!(Tc3, 0);

#[hal_cfg("tc4-d21")]
impl_async_count16!(Tc4, 1);

#[hal_cfg("tc5-d21")]
impl_async_count16!(Tc5, 2);

#[hal_cfg("tc2-d5x")]
impl_async_count16!(Tc2, 0);

#[hal_cfg("tc3-d5x")]
impl_async_count16!(Tc3, 1);

#[hal_cfg("tc4-d5x")]
impl_async_count16!(Tc4, 2);

#[hal_cfg("tc5-d5x")]
impl_async_count16!(Tc5, 3);

// Reserve space for the max number of timer peripherals based on chip type,
// even though some wakers may not be used on some chips if they actually don't
// exist on variant's hardware
#[hal_cfg("tc1-d11")]
const NUM_TIMERS: usize = 1;

#[hal_cfg("tc3-d21")]
const NUM_TIMERS: usize = 3;

#[hal_cfg("tc3-d5x")]
const NUM_TIMERS: usize = 4;

impl<T> TimerCounter<T>
where
    T: AsyncCount16,
{
    /// Transform a [`TimerCounter`] into an [`TimerFuture`]
    #[inline]
    pub fn into_future<I>(mut self, _irq: I) -> TimerFuture<T>
    where
        I: Binding<T::Interrupt, InterruptHandler<T>>,
    {
        T::Interrupt::unpend();
        unsafe { T::Interrupt::enable() };
        self.enable_interrupt();

        TimerFuture { timer: self }
    }
}

/// Wrapper around a [`TimerCounter`] with an `async` interface
pub struct TimerFuture<T>
where
    T: AsyncCount16,
{
    timer: TimerCounter<T>,
}

impl<T> TimerFuture<T>
where
    T: AsyncCount16,
{
    /// Delay asynchronously
    #[inline]
    pub async fn delay(&mut self, count: Nanoseconds) {
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

impl<T> Drop for TimerFuture<T>
where
    T: AsyncCount16,
{
    #[inline]
    fn drop(&mut self) {
        T::Interrupt::disable();
    }
}

impl<T> embedded_hal_async::delay::DelayNs for TimerFuture<T>
where
    T: AsyncCount16,
{
    async fn delay_ns(&mut self, ns: u32) {
        self.delay(Nanoseconds::from_ticks(ns.into()).convert())
            .await;
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
