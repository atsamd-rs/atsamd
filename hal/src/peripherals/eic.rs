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
pub use impls::*;

pub type Sense = pac::eic::config::Sense0select;

/// Trait representing a EIC EXTINT channel ID
pub trait ChId {
    const ID: usize;
}

/// Marker struct that represents an EXTINT channel capable of doing async operations.
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

/// Represents a numbered external interrupt. The external interrupt is generic
/// over any pin, only the EicPin implementations in this module make sense.
pub struct ExtInt<P, Id, F = NoneT>
where
    P: EicPin,
    Id: ChId,
{
    chan: Channel<Id, F>,
    _pin: Pin<P::Id, P::Mode>,
}

// impl !Send for [<$PadType $num>]<GPIO> {}
// impl !Sync for [<$PadType $num>]<GPIO> {}

impl<P, Id, F> ExtInt<P, Id, F>
where
    P: EicPin,
    Id: ChId,
{
    /// Construct pad from the appropriate pin in any mode.
    /// You may find it more convenient to use the `into_pad` trait
    /// and avoid referencing the pad type.
    fn new(pin: P, chan: Channel<Id, F>) -> Self {
        ExtInt {
            _pin: pin.into(),
            chan,
        }
    }
}

/// EIC channel.
///
/// Use this struct to create an [`ExtInt`](pins::ExtInt) by calling [`with_pin`](Self::with_pin).
pub struct Channel<Id: ChId, F = NoneT> {
    eic: core::mem::ManuallyDrop<pac::Eic>,
    _id: PhantomData<Id>,
    _irqs: PhantomData<F>,
}

impl<Id: ChId, F> Channel<Id, F> {
    /// Assign a pin to this [`Channel`], and turn it into an [`ExtInt`], which is capable of sensing
    /// state changes on the pin.
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

/// External Interrupt Controller. Use [`split`](Self::split) to split the struct into individual channels, whcih can be used to create [`ExtInt`]s.
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

        // Reset the EIC
        eic.ctrla().modify(|_, w| w.swrst().set_bit());
        while eic.syncbusy().read().swrst().bit_is_set() {
            core::hint::spin_loop();
        }

        // Use the low-power 32k clock and enable.
        eic.ctrla().modify(|_, w| {
            w.cksel().set_bit();
            w.enable().set_bit()
        });

        while eic.syncbusy().read().enable().bit_is_set() {
            core::hint::spin_loop();
        }

        Self {
            eic,
            _irqs: PhantomData,
        }
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

pub const NUM_CHANNELS: usize = with_num_channels!(get);

macro_rules! define_channels_struct {
    ($num_channels:literal) => {
        seq!(N in 0..$num_channels {
            #(
                /// Type alias for a channel number
                pub struct Ch~N;

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
