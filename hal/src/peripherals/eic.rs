//! # External Interrupt Controller
//!
//! This module provides typesafe APIs for interacting with the EIC peripheral,
//! which is used to generate interrupts based on the state of a GPIO.
//!
//! Each chip has a number of EXTINT channels:
//!
//! * SAMD11: 8 channels
//! * SAMD21/SAMx5x: 16 channels
//!
//! Each channel can operate independently, and sense state changes for a single
//! GPIO pin at a time. Refer to the datasheet for GPIO pin/EXTINT channel
//! compatibility. In this module, an [`ExtInt`] represents an EXTINT channel
//! which is tied to a GPIO [`Pin`], and is capable of sensing state changes.
//!
//! ## Steps to create an [`ExtInt`]
//!
//! 1. Start by creating an [`Eic`] struct, by calling [`Eic::new`]. This
//!    initializes the EIC peripheral and sets up the correct clocking.
//!
//! 1. Turn the [`Eic`] into a tuple of [`Channel`]s by calling [`Eic::split`].
//!    Each channel represents a single EXTINT channel.
//!
//! 1. Assign a pin to a channel by calling [`Channel::with_pin`]. This returns
//!    a fully configured and ready to use [`ExtInt`]. A [`Pin`] can also be
//!    directly converted into an [`ExtInt`] by calling one of the methods
//!    provided by the [`EicPin`] trait.
//!
//! ### Example setup
//!
//! ```no_run
//! let eic_clock = clocks.eic(&gclk0).unwrap();
//! // Initialize the EIC peripheral
//! let eic = Eic::new(&mut peripherals.pm, eic_clock, peripherals.eic);
//! // Split into channels
//! let eic_channels = eic.split();
//!
//! // Take the pin that we want to use
//! let button: Pin<_, PullUpInterrupt> = pins.d10.into();
//!
//! // Turn the EXTINT[2] channel into an ExtInt struct
//! let mut extint = eic_channels.2.with_pin(button);
//! ```
//!
//! ## `async` operation <span class="stab portability" title="Available on crate feature `async` only"><code>async</code></span>
//!
//! [`ExtInt`]s can be used for async operations. Configuring the [`Eic`] in
//! async mode is relatively simple:
//!
//! * Bind the corresponding `EIC` interrupt source to the SPI
//!   [`InterruptHandler`] (refer to the module-level
//!   [`async_hal`](crate::async_hal) documentation for more information).
//!
//! * SAMD11/SAMD21: Turn an [`Eic`] into an async-enabled [`Eic`] by calling
//!   [`Eic::into_future`]. Since there is only a single interrupt handler for
//!   the EIC peripheral, all EXTINT channels must be turned into async channels
//!   at once.
//! * SAMx5x: Turn an individuel [`ExtInt`] into an async-enabled [`ExtInt`] by
//!   calling [`ExtInt::into_future`]. Each channel has a dedicated interrupt
//!   source, therefore you must individually choose which channels to turn into
//!   async channels.
//! * Use the provided [`wait`](ExtInt::wait) method. async-enabled [`ExtInt`]s
//!   also implement [`embedded_hal_async::digital::Wait`].

use core::marker::PhantomData;

use atsamd_hal_macros::{hal_cfg, hal_module};
use seq_macro::seq;

use crate::{
    clock::EicClock,
    gpio::{AnyPin, Pin},
    pac,
    typelevel::{NoneT, Sealed},
};

#[hal_module(
    any("eic-d11", "eic-d21") => "eic/d11/mod.rs",
    "eic-d5x" => "eic/d5x/mod.rs",
)]
mod impls {}
#[cfg(feature = "async")]
pub use impls::async_api::*;

pub type Sense = pac::eic::config::Sense0select;

/// Trait representing an EXTINT channel ID.
pub trait ChId {
    const ID: usize;
}

/// Marker type that represents an EXTINT channel capable of doing async
/// operations.
#[cfg(feature = "async")]
pub enum EicFuture {}

/// Trait representing a GPIO pin which can be used as an external interrupt.
pub trait EicPin: AnyPin + Sealed {
    type Floating;
    type PullUp;
    type PullDown;

    type ChId: ChId;

    #[hal_cfg("eic-d5x")]
    #[cfg(feature = "async")]
    type InterruptSource: crate::async_hal::interrupts::InterruptSource;

    /// Configure a pin as a floating external interrupt
    fn into_floating_ei(self, chan: Channel<Self::ChId>) -> Self::Floating;

    /// Configure a pin as pulled-up external interrupt
    fn into_pull_up_ei(self, chan: Channel<Self::ChId>) -> Self::PullUp;

    /// Configure a pin as pulled-down external interrupt
    fn into_pull_down_ei(self, chan: Channel<Self::ChId>) -> Self::PullDown;
}

/// A numbered external interrupt, which can be used to sense state changes on
/// its pin.
pub struct ExtInt<P, Id, F = NoneT>
where
    P: EicPin,
    Id: ChId,
{
    chan: Channel<Id, F>,
    pin: Pin<P::Id, P::Mode>,
}

impl<P, Id, F> ExtInt<P, Id, F>
where
    P: EicPin,
    Id: ChId,
{
    /// Release the underlying resources: [`Pin`] and [`Channel`].
    pub fn free(self) -> (Pin<P::Id, P::Mode>, Channel<Id, F>) {
        (self.pin, self.chan)
    }

    /// Construct pad from the appropriate pin in any mode.
    /// You may find it more convenient to use the `into_pad` trait
    /// and avoid referencing the pad type.
    fn new(pin: P, chan: Channel<Id, F>) -> Self {
        ExtInt {
            pin: pin.into(),
            chan,
        }
    }
}

/// EIC channel.
///
/// Use this struct to create an [`ExtInt`](pins::ExtInt) by calling
/// [`with_pin`](Self::with_pin).
pub struct Channel<Id: ChId, F = NoneT> {
    eic: core::mem::ManuallyDrop<pac::Eic>,
    _id: PhantomData<Id>,
    _irqs: PhantomData<F>,
}

impl<Id: ChId, F> Channel<Id, F> {
    /// Assign a pin to this [`Channel`], and turn it into an [`ExtInt`], which
    /// is capable of sensing state changes on the pin.
    pub fn with_pin<P: EicPin<ChId = Id>>(self, pin: P) -> ExtInt<P, Id, F> {
        ExtInt::new(pin, self)
    }

    fn new(eic: pac::Eic) -> Self {
        Self {
            eic: core::mem::ManuallyDrop::new(eic),
            _id: PhantomData,
            _irqs: PhantomData,
        }
    }

    #[hal_cfg("eic-d5x")]
    #[cfg(feature = "async")]
    fn change_mode<N>(self) -> Channel<Id, N> {
        Channel {
            eic: self.eic,
            _id: self._id,
            _irqs: PhantomData,
        }
    }
}

/// External Interrupt Controller.
///
/// Use [`split`](Self::split) to split the struct into individual channels,
/// which can then be used to create [`ExtInt`]s, by calling
/// [`Channel::with_pin`].
pub struct Eic<I = NoneT> {
    eic: pac::Eic,
    _irqs: PhantomData<I>,
}

impl Eic {
    /// Create a new [`Eic`] and initialize it.
    #[hal_cfg(any("eic-d11", "eic-d21"))]
    pub fn new(pm: &mut pac::Pm, _clock: EicClock, eic: pac::Eic) -> Self {
        pm.apbamask().modify(|_, w| w.eic_().set_bit());

        // Reset the EIC
        eic.ctrl().modify(|_, w| w.swrst().set_bit());
        while eic.ctrl().read().swrst().bit_is_set() {
            core::hint::spin_loop();
        }

        eic.ctrl().modify(|_, w| w.enable().set_bit());
        while eic.status().read().syncbusy().bit_is_set() {
            cortex_m::asm::nop();
        }
        Self {
            eic,
            _irqs: PhantomData,
        }
    }

    /// Create and initialize a new [`Eic`], and wire it up to the
    /// ultra-low-power 32kHz clock source.
    #[hal_cfg("eic-d5x")]
    pub fn new(mclk: &mut pac::Mclk, _clock: EicClock, eic: pac::Eic) -> Self {
        mclk.apbamask().modify(|_, w| w.eic_().set_bit());

        let mut eic = Self {
            eic,
            _irqs: PhantomData,
        };

        // Reset the EIC
        eic.swreset();

        // Use the low-power 32k clock and enable.
        eic.eic.ctrla().modify(|_, w| {
            w.cksel().set_bit();
            w.enable().set_bit()
        });

        while eic.eic.syncbusy().read().enable().bit_is_set() {
            core::hint::spin_loop();
        }

        eic
    }

    /// Release the EIC and return the register block.
    ///
    /// **Note**: The [`Channels`] struct is consumed by this method. This means
    /// that any [`Channel`] obtained by [`split`](Eic::split) must be
    /// moved back into the [`Channels`] struct before being able to pass it
    /// into [`free`](Eic::free).
    pub fn free(mut self, _channels: Channels) -> pac::Eic {
        self.swreset();

        self.eic
    }
}

impl<F> Eic<F> {
    /// Reset the EIC
    #[atsamd_hal_macros::hal_macro_helper]
    fn swreset(&mut self) {
        #[hal_cfg(any("eic-d11", "eic-d21"))]
        let ctrl = self.eic.ctrl();

        #[hal_cfg("eic-d5x")]
        let ctrl = self.eic.ctrla();

        ctrl.modify(|_, w| w.swrst().set_bit());
        while ctrl.read().swrst().bit_is_set() {
            core::hint::spin_loop();
        }
    }
}

#[cfg(feature = "async")]
impl Eic<EicFuture> {
    /// Release the EIC and return the register block.
    ///
    /// **Note**: The [`Channels`] struct is consumed by this method. This means
    /// that any [`Channel`] obtained by [`split`](Eic::split) must be
    /// moved back into the [`Channels`] struct before being able to pass it
    /// into [`free`](Eic::free).
    pub fn free(mut self, _channels: FutureChannels) -> pac::Eic {
        self.swreset();
        self.eic
    }
}

#[hal_cfg("eic-d11")]
macro_rules! with_num_channels {
    ($some_macro:ident) => {
        $some_macro! {8}
    };
}

#[hal_cfg(any("eic-d5x", "eic-d21"))]
macro_rules! with_num_channels {
    ($some_macro:ident) => {
        $some_macro! {16}
    };
}

macro_rules! get {
    ($literal:literal) => {
        $literal
    };
}

/// The number of EXTINT channels on this chip.
pub const NUM_CHANNELS: usize = with_num_channels!(get);

macro_rules! define_channels_struct {
    ($num_channels:literal) => {
        seq!(N in 0..$num_channels {
            #(
                /// Type alias for a channel number
                pub enum Ch~N {}

                impl ChId for Ch~N {
                    const ID: usize = N;
                }
            )*

            /// Struct generating individual handles to each EXTINT channel
            pub struct Channels(
                #(
                    pub Channel<Ch~N>,
                )*
            );
        });
    };
}

with_num_channels!(define_channels_struct);

#[cfg(feature = "async")]
macro_rules! define_channels_struct_future {
    ($num_channels:literal) => {
        seq!(N in 0..$num_channels {
            /// Struct generating individual handles to each EXTINT channel for `async` operation
            pub struct FutureChannels(
                #(
                    pub Channel<Ch~N, EicFuture>,
                )*
            );
        });
    };
}

#[cfg(feature = "async")]
with_num_channels!(define_channels_struct_future);

macro_rules! define_split {
    ($num_channels:literal) => {
        seq!(N in 0..$num_channels {
            /// Split the EIC into individual channels.
            #[inline]
            pub fn split(self) -> Channels {
                Channels(
                    #(
                       unsafe { Channel::new(core::ptr::read(&self.eic as *const _)) },
                    )*
                )
            }

        });
    };
}

impl Eic {
    with_num_channels!(define_split);
}

#[cfg(feature = "async")]
macro_rules! define_split_future {
    ($num_channels:literal) => {
        seq!(N in 0..$num_channels {
            /// Split the EIC into individual channels
            #[inline]
            pub fn split(self) -> FutureChannels {
                FutureChannels(
                    #(
                        unsafe { Channel::new(core::ptr::read(&self.eic as *const _)) },
                    )*
                )
            }
        });
    };
}

#[cfg(feature = "async")]
impl Eic<EicFuture> {
    with_num_channels!(define_split_future);
}
