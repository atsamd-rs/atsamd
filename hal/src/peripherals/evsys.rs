//! # Event system controller
//!
//! This peripheral allows for the chips peripherals to exchange events with eachother
//! without needing the CPU.
//!
//! The event system contains a number of channels, that can be wired between peripherals.
//! The number of channels available varies per chip:
//! * SAM5x: 32 channels
//! * SAMD21: 12 channels
//! * SAMD11: 6 channels
//!
//! The channels destinations are configured using a multiplexor register, which directs events to
//! a destination peripherals. Destination peripherals can support 1 or more supported path modes:
//! * Asynchronous
//! * Synchronous
//! * Resynchronized
//!
//! A peripheral can only be the receiver of 1 channel, however, a peripheral can generate
//! events on multiple channels at the same time.
//!
//! ## Usage
//!
//! 1. Create the [`EvSysController`]
//! 2. Split the controller into a tuple of its channels, using [`EvSysController::split`]
//! 3. Pass a channel into a generator peripheral to enable it to output to the provided channel
//! 4. Pass the channel into the receiving peripheral to complete wiring the peripheral up.
//!
//! ## Notes
//! At the moment, the event system channels will only run using Asynchronous paths,
//! which is not supported by all receiving peripherals. Consult the chip datasheet
use atsamd_hal_macros::hal_cfg;
use core::marker::PhantomData;

use crate::pac::Evsys;
use crate::typelevel::Sealed;
use seq_macro::seq;

#[hal_cfg("evsys-d5x")]
use crate::pac::Mclk;
#[hal_cfg(any("evsys-d21", "evsys-d11"))]
use crate::pac::Pm;

pub trait Status: Sealed {}

pub enum Uninitialized {}
impl Sealed for Uninitialized {}
impl Status for Uninitialized {}

pub enum GenReady {}
impl Sealed for GenReady {}
impl Status for GenReady {}

pub enum Ready {}
impl Sealed for Ready {}
impl Status for Ready {}

pub trait ChId {
    const ID: usize;
}

pub struct EvSysChannel<Id: ChId, S: Status> {
    evsys: core::mem::ManuallyDrop<Evsys>,
    _id: PhantomData<Id>,
    _state: PhantomData<S>,
    generator_id: u8,
}

// Create a new Uninitialized channel
fn new_evsys_channel<Id: ChId>(evsys: Evsys) -> EvSysChannel<Id, Uninitialized> {
    EvSysChannel {
        evsys: core::mem::ManuallyDrop::new(evsys),
        _id: PhantomData,
        _state: PhantomData,
        generator_id: 0,
    }
}

// Impl for channels of any state
impl<Id: ChId, S: Status> EvSysChannel<Id, S> {
    fn change_status<N: Status>(self) -> EvSysChannel<Id, N> {
        EvSysChannel {
            evsys: self.evsys,
            _id: PhantomData,
            _state: PhantomData,
            generator_id: self.generator_id,
        }
    }
}

// Methods that can only be used on a channel that is uninitialized
impl<Id: ChId> EvSysChannel<Id, Uninitialized> {
    pub fn register_generator(mut self, generator_id: u8) -> EvSysChannel<Id, GenReady> {
        self.generator_id = generator_id;
        self.change_status()
    }
}

// Methods that can only be used on a channel with just a connected generator
impl<Id: ChId> EvSysChannel<Id, GenReady> {
    pub fn remove_generator(mut self) -> EvSysChannel<Id, Uninitialized> {
        self.generator_id = 0;
        self.change_status()
    }

    #[hal_cfg("evsys-d5x")]
    pub fn register_user(self, user_id: usize) -> EvSysChannel<Id, Ready> {
        // Now wire up the generator
        // Multiplexor MUST be wired before the channel
        self.evsys
            .user(user_id)
            .write(|w| unsafe { w.channel().bits(Id::ID as u8 + 1) }); // +1 since 0 means no channel

        self.evsys.channels(Id::ID as usize).channel().write(|w| {
            w.path().asynchronous();
            w.edgsel().no_evt_output();
            unsafe { w.evgen().bits(self.generator_id) }
        });
        self.change_status()
    }

    #[hal_cfg(any("evsys-d21", "evsys-d11"))]
    pub fn register_user(self, user_id: usize) -> EvSysChannel<Id, Ready> {
        // Multiplexor MUST be wired before the channel

        // Select our user
        self.evsys
            .user()
            .write(|w| unsafe { w.user().bits(user_id as u8) });

        // Write the channel we want to connect to it
        self.evsys
            .user()
            .write(|w| unsafe { w.channel().bits(Id::ID as u8 + 1) });

        // Select our channel
        self.evsys
            .channel()
            .write(|w| unsafe { w.channel().bits(Id::ID as u8) });

        // Configure the channel
        self.evsys.channel().write(|w| {
            w.path().asynchronous();
            w.edgsel().no_evt_output();
            unsafe { w.evgen().bits(self.generator_id) }
        });
        self.change_status()
    }
}

// Methods that can only be used on a channel that has both ends connected
impl<Id: ChId> EvSysChannel<Id, Ready> {
    #[hal_cfg("evsys-d5x")]
    pub fn deregister_user(self, user_id: usize) -> EvSysChannel<Id, GenReady> {
        // Unhook the channel generator
        let reg = self.evsys.channels(Id::ID as usize);
        reg.channel().reset();
        // Then unhook the user
        self.evsys
            .user(user_id)
            .write(|w| unsafe { w.channel().bits(0) });
        self.change_status()
    }

    #[hal_cfg(any("evsys-d21", "evsys-d11"))]
    pub fn deregister_user(self, user_id: usize) -> EvSysChannel<Id, GenReady> {
        // Select our user
        self.evsys
            .user()
            .write(|w| unsafe { w.user().bits(user_id as u8) });
        self.evsys.user().write(|w| unsafe { w.channel().bits(0) }); // Disconnect channel
        self.change_status()
    }
}

/// Event system controller peripheral
pub struct EvSysController {
    evsys: crate::pac::Evsys,
}

#[hal_cfg("evsys-d5x")]
impl EvSysController {
    pub fn new(mclk: &mut Mclk, evsys: crate::pac::Evsys) -> Self {
        mclk.apbbmask().write(|w| w.evsys_().set_bit()); // Enable EVSYS clock
        evsys.ctrla().write(|w| w.swrst().set_bit());
        Self { evsys }
    }

    pub fn free(self, _channels: Channels) -> Evsys {
        self.evsys.ctrla().write(|w| w.swrst().set_bit());
        self.evsys
    }
}

#[hal_cfg(any("evsys-d21", "evsys-d11"))]
impl EvSysController {
    pub fn new(pm: &mut Pm, evsys: crate::pac::Evsys) -> Self {
        pm.apbcmask().write(|w| w.evsys_().set_bit()); // Enable EVSYS clock
        evsys.ctrl().write(|w| w.swrst().set_bit());
        Self { evsys }
    }

    pub fn free(self, _channels: Channels) -> Evsys {
        self.evsys.ctrl().write(|w| w.swrst().set_bit());
        self.evsys
    }
}

#[hal_cfg("evsys-d11")]
macro_rules! with_num_channels {
    ($some_macro:ident) => {
        $some_macro! {6}
    };
}

#[hal_cfg(any("evsys-d21"))]
macro_rules! with_num_channels {
    ($some_macro:ident) => {
        $some_macro! {12}
    };
}

#[hal_cfg(any("evsys-d5x"))]
macro_rules! with_num_channels {
    ($some_macro:ident) => {
        $some_macro! {32}
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
                pub enum Ch~N {}

                impl ChId for Ch~N {
                    const ID: usize = N;
                }
            )*

            /// Struct generating individual handles to each EVSYS channel
            pub struct Channels(
                #(
                    pub EvSysChannel<Ch~N, Uninitialized>,
                )*
            );
        });
    };
}
with_num_channels!(define_channels_struct);

macro_rules! define_split {
    ($num_channels:literal) => {
        seq!(N in 0..$num_channels {
            /// Split the EVSYS into individual channels.
            #[inline]
            pub fn split(self) -> Channels {
                Channels(
                    #(
                       unsafe { new_evsys_channel(core::ptr::read(&self.evsys as *const _)) },
                    )*
                )
            }

        });
    };
}

impl EvSysController {
    with_num_channels!(define_split);
}
