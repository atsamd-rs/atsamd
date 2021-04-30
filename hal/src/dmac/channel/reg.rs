//! # Channel registers
//!
//! This module adds a [`RegisterBlock`] struct, which acts as a proxy for the
//! registers a single DMAC [`Channel`] can read/write. Its purpose is to
//! remediate the inadequacies of the PAC. In particular, for SAMD11/SAMD21, the
//! CHID register must be written with the correct channel ID before accessing
//! the channel specific registers. There is a provided `with_chid` method that
//! takes a closure with the register read/write proxies to ensure any
//! read/write to these registers are done in an interrupt-safe way. For
//! SAMD51+, `with_chid` returns the register block which contains the registers
//! owned by a specific channel.

use super::super::dma_controller::ChId;
use core::marker::PhantomData;
use paste::paste;

use crate::target_device::{
    self,
    dmac::{BUSYCH, INTSTATUS, PENDCH, SWTRIGCTRL},
    Peripherals, DMAC,
};

#[cfg(any(feature = "samd11", feature = "samd21"))]
use target_device::dmac as channel_regs;

#[cfg(feature = "min-samd51g")]
use target_device::dmac::channel as channel_regs;

use channel_regs::{CHCTRLA, CHCTRLB, CHINTENCLR, CHINTENSET, CHINTFLAG, CHSTATUS};

#[cfg(feature = "min-samd51g")]
use target_device::dmac::{channel::CHPRILVL, CHANNEL};

//==============================================================================
// RegisterBlock
//==============================================================================
/// Read/write proxy for DMAC registers accessible to individual channels.
pub(super) trait Register<Id: ChId> {
    /// Set channel ID and run the closure. A closure is needed to ensure
    /// the registers are accessed in an interrupt-safe way, as the SAMD21
    /// DMAC is a little funky - It requires setting the channel number in
    /// the CHID register, then access the channel control registers.
    /// If an interrupt were to change the CHID register, we would be faced
    /// with undefined behaviour.
    #[cfg(any(feature = "samd11", feature = "samd21"))]
    #[inline]
    fn with_chid<F: FnOnce(&DMAC) -> R, R>(&self, dmac: &DMAC, fun: F) -> R {
        cortex_m::interrupt::free(|_| {
            // SAFETY: This is ONLY safe if the individual channels are GUARANTEED not to
            // mess with either:
            // - The global DMAC configuration
            // - The configuration of other channels.
            //
            // In practice, this means that the DMAC registers should only be accessed
            // through the `with_chid` method.
            unsafe {
                dmac.chid.modify(|_, w| w.id().bits(Id::U8));
            };

            fun(dmac)
        })
    }

    /// Set channel ID and run the closure. A closure is needed to ensure
    /// the registers are accessed in an interrupt-safe way, as the SAMD21
    /// DMAC is a little funky. For the SAMD51/SAMEx, we simply take a reference
    /// to the correct channel number and run the closure on that.
    #[cfg(feature = "min-samd51g")]
    #[inline]
    fn with_chid<F: FnOnce(&CHANNEL) -> R, R>(&self, dmac: &DMAC, fun: F) -> R {
        // SAFETY: This is ONLY safe if the individual channels are GUARANTEED not to
        // mess with either:
        // - The global DMAC configuration
        // - The configuration of other channels.
        //
        // In practice, this means that the DMAC registers should only be accessed
        // through the `with_chid` method.
        let mut ch = &dmac.channel[Id::USIZE];
        fun(&mut ch)
    }
}

/// Readable register which belongs to a specific channel in its entirety
pub(super) trait ReadRegister<Id: ChId, REG>: Register<Id>
where
    REG: target_device::generic::Readable,
{
    /// PAC read proxy
    type R;
    /// Read the register in a safe way.
    fn read(&self) -> Self::R;
}
/// Writable register which belongs to a specific channel in its entirety
pub(super) trait WriteRegister<Id: ChId, REG>: Register<Id>
where
    REG: target_device::generic::Writable,
{
    /// PAC write proxy
    type W;
    /// Write to the register in a safe way
    fn write<F>(&mut self, func: F)
    where
        for<'w> F: FnOnce(&'w mut Self::W) -> &'w mut Self::W;
}

/// Readable/writable register which belongs to a specific channel in its
/// entirety
pub(super) trait ModifyRegister<Id: ChId, REG>: Register<Id>
where
    REG: target_device::generic::Readable + target_device::generic::Writable,
{
    /// PAC read proxy
    type R;
    /// PAC write proxy
    type W;

    /// Modify the register in a safe way
    fn modify<F>(&mut self, func: F)
    where
        for<'w> F: FnOnce(&Self::R, &'w mut Self::W) -> &'w mut Self::W;
}

macro_rules! channel_reg_read {
    ($reg:ident, $obj:ty) => {
        paste! {
            /// Register proxy tied to a specific channel
            pub(super) struct [< $obj:camel Proxy >]<Id: ChId, REG> {
                dmac: DMAC,
                _id: PhantomData<Id>,
                _reg: PhantomData<REG>,
            }

            impl<Id: ChId> [< $obj:camel Proxy >]<Id, $obj> {
                /// Create a new register proxy
                #[inline]
                pub(super) fn new() -> Self {
                    Self {
                        // SAFETY: This is safe as long as the register
                        // only reads/writes registers through
                        // the `with_chid` method.
                        dmac: unsafe { Peripherals::steal().DMAC },
                        _id: PhantomData,
                        _reg: PhantomData,
                    }
                }
            }

            impl<Id: ChId> Register<Id> for [< $obj:camel Proxy >]<Id, $obj> {}

            impl<Id: ChId> ReadRegister<Id, $obj> for [<$obj:camel Proxy>]<Id, $obj> {
                type R = channel_regs::$reg::R;

                #[inline]
                fn read(&self) -> Self::R {
                    self.with_chid(&self.dmac, |d| d.$reg.read())
                }
            }
        }
    };
}
macro_rules! channel_reg_write {
    ($reg:ident, $obj:ty) => {
        paste! {
        impl<Id: ChId> WriteRegister<Id, $obj> for [< $obj:camel Proxy >]<Id, $obj> {
            type W = channel_regs::$reg::W;

            #[inline]
            fn write<F>(&mut self, func: F)
            where
                for<'w> F: FnOnce(&'w mut Self::W) -> &'w mut Self::W,
            {
                self.with_chid(&self.dmac, |d| d.$reg.write(|w| func(w)));
            }
        }

        impl<Id: ChId> ModifyRegister<Id, $obj> for [< $obj:camel Proxy >]<Id, $obj> {
            type R = channel_regs::$reg::R;
            type W = channel_regs::$reg::W;

            #[inline]
            fn modify<F>(&mut self, func: F)
            where
                for<'w> F: FnOnce(&Self::R, &'w mut Self::W) -> &'w mut Self::W,
            {
                self.with_chid(&self.dmac, |d| d.$reg.modify(|r, w| func(r, w)));
            }
        }}
    };
}

channel_reg_read!(chctrla, CHCTRLA);
channel_reg_read!(chctrlb, CHCTRLB);
channel_reg_read!(chintenclr, CHINTENCLR);
channel_reg_read!(chintenset, CHINTENSET);
channel_reg_read!(chintflag, CHINTFLAG);
channel_reg_read!(chstatus, CHSTATUS);
#[cfg(feature = "min-samd51g")]
channel_reg_read!(chprilvl, CHPRILVL);

channel_reg_write!(chctrla, CHCTRLA);
channel_reg_write!(chctrlb, CHCTRLB);
channel_reg_write!(chintenclr, CHINTENCLR);
channel_reg_write!(chintenset, CHINTENSET);
channel_reg_write!(chintflag, CHINTFLAG);
#[cfg(feature = "min-samd51g")]
channel_reg_write!(chprilvl, CHPRILVL);
pub(super) trait ReadBit<Id: ChId, REG>: Register<Id>
where
    REG: target_device::generic::Readable,
{
    fn read(&self) -> bool;
}

macro_rules! channel_bit_read {
    ($reg:ident, $obj:ty) => {
        paste! {
            /// Register proxy tied to a specific channel
            pub(super) struct [< $obj:camel Proxy >]<Id: ChId, REG> {
                dmac: DMAC,
                _id: PhantomData<Id>,
                _reg: PhantomData<REG>,
            }

            impl<Id: ChId> [< $obj:camel Proxy >]<Id, $obj> {
                /// Create a new register proxy
                #[inline]
                pub(super) fn new() -> Self {
                    Self {
                        // SAFETY: This is safe as long as the register
                        // only reads/writes the bits it controls
                        // within registers.
                        dmac: unsafe { Peripherals::steal().DMAC },
                        _id: PhantomData,
                        _reg: PhantomData,
                    }
                }
            }

            impl<Id: ChId> Register<Id> for [< $obj:camel Proxy >]<Id, $obj> {}

            impl<Id: ChId> ReadBit<Id, $obj> for [< $obj:camel Proxy >]<Id, $obj> {
                #[inline]
                fn read(&self) -> bool {
                    self.dmac.$reg.read().bits() & (1 << Id::U8) != 0
                }
            }
        }
    };
}

channel_bit_read!(intstatus, INTSTATUS);
channel_bit_read!(busych, BUSYCH);
channel_bit_read!(pendch, PENDCH);
channel_bit_read!(swtrigctrl, SWTRIGCTRL);

pub(super) trait WriteBit<Id: ChId, REG>: Register<Id>
where
    REG: target_device::generic::Writable,
{
    fn write(&self, bit: bool);
}

macro_rules! channel_bit_write {
    ($reg: ident, $obj:ty) => {
        paste! {
            impl<Id: ChId> WriteBit<Id, $obj> for [< $obj:camel Proxy >]<Id, $obj> {
                #[inline]
                fn write(&self, bit: bool) {
                    // SAFETY: This is safe because we are only writing
                    // to the bit controlled by the channel.
                    self.dmac
                        .$reg
                        .modify(|r, w| unsafe { w.bits(r.bits() & ((bit as u32) << Id::U8)) });
                }
            }
        }
    };
}

channel_bit_write!(swtrigctrl, SWTRIGCTRL);

/// Acts as a proxy to the PAC DMAC object. Only registers and bits
/// within registers that should be readable/writable by specific
/// [`Channel`]s are exposed.
pub(super) struct RegisterBlock<Id: ChId> {
    pub chctrla: ChctrlaProxy<Id, CHCTRLA>,
    pub chctrlb: ChctrlbProxy<Id, CHCTRLB>,
    pub chintenclr: ChintenclrProxy<Id, CHINTENCLR>,
    pub chintenset: ChintensetProxy<Id, CHINTENSET>,
    pub chintflag: ChintflagProxy<Id, CHINTFLAG>,
    pub chstatus: ChstatusProxy<Id, CHSTATUS>,
    pub intstatus: IntstatusProxy<Id, INTSTATUS>,
    pub busych: BusychProxy<Id, BUSYCH>,
    pub pendch: PendchProxy<Id, PENDCH>,
    pub swtrigctrl: SwtrigctrlProxy<Id, SWTRIGCTRL>,
    #[cfg(feature = "min-samd51g")]
    pub chprilvl: ChprilvlProxy<Id, CHPRILVL>,
}

impl<Id: ChId> RegisterBlock<Id> {
    pub(super) fn new(_id: PhantomData<Id>) -> Self {
        Self {
            chctrla: ChctrlaProxy::new(),
            chctrlb: ChctrlbProxy::new(),
            chintenclr: ChintenclrProxy::new(),
            chintenset: ChintensetProxy::new(),
            chintflag: ChintflagProxy::new(),
            chstatus: ChstatusProxy::new(),
            intstatus: IntstatusProxy::new(),
            busych: BusychProxy::new(),
            pendch: PendchProxy::new(),
            swtrigctrl: SwtrigctrlProxy::new(),
            #[cfg(feature = "min-samd51g")]
            chprilvl: ChprilvlProxy::new(),
        }
    }
}
