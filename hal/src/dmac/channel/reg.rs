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

macro_rules! reg_proxy {
    (@new $reg:ident) => {
        paste! {
            /// Register proxy tied to a specific channel
            pub(super) struct [< $reg:camel Proxy >]<Id: ChId, REG> {
                #[allow(ununsed)]
                dmac: DMAC,
                _id: PhantomData<Id>,
                _reg: PhantomData<REG>,
            }

            impl<Id: ChId> [< $reg:camel Proxy >]<Id, [< $reg:upper >]> {
                /// Create a new register proxy
                #[inline]
                pub fn new() -> Self {
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
        }
    };

    // Internal rule for a Read-enabled register
    (@read_reg $reg:ident) => {
        paste! {
            impl<Id: ChId> Register<Id> for [< $reg:camel Proxy >]<Id, [< $reg:upper >]> {}

            impl<Id> [< $reg:camel Proxy >]<Id, [< $reg:upper >]> where Id: ChId, [< $reg:upper >]: target_device::generic::Readable {
                #[inline]
                #[allow(dead_code)]
                pub fn read(&self) -> channel_regs::[< $reg:lower >]::R {
                    self.with_chid(&self.dmac, |d| d.[< $reg:lower >].read())
                }
            }
        }
    };

    // Read-only register
    ($reg:ident, register, r) => {
        paste! {

            reg_proxy!(@new $reg);
            reg_proxy!(@read_reg $reg);
        }
    };

    // Read-write register
    ($reg:ident, register, rw) => {
        paste! {
            reg_proxy!(@new $reg);
            reg_proxy!(@read_reg $reg);

            impl<Id> [< $reg:camel Proxy >]<Id, [< $reg:upper >]> where Id: ChId, [< $reg:upper >]: target_device::generic::Writable {
                #[inline]
                #[allow(dead_code)]
                pub fn write<F>(&mut self, func: F)
                where
                    for<'w> F: FnOnce(&'w mut channel_regs::[< $reg:lower >]::W) -> &'w mut channel_regs::[< $reg:lower >]::W,
                {
                    self.with_chid(&self.dmac, |d| d.[< $reg:lower >].write(|w| func(w)));
                }
            }

            impl<Id>[< $reg:camel Proxy >]<Id, [< $reg:upper >]> where
                Id: ChId,
                [< $reg:upper >]: target_device::generic::Writable + target_device::generic::Readable
            {
                #[inline]
                #[allow(dead_code)]
                pub fn modify<F>(&mut self, func: F)
                where
                    for<'w> F: FnOnce(
                        &channel_regs::[< $reg:lower >]::R,
                        &'w mut channel_regs::[< $reg:lower >]::W
                    ) -> &'w mut channel_regs::[< $reg:lower >]::W,
                {
                    self.with_chid(&self.dmac, |d| d.[< $reg:lower >].modify(|r, w| func(r, w)));
                }
            }
        }
    };

    // Internal rule for read-enabled bit
    (@read_bit $reg:ident) => {
        paste! {
            impl<Id: ChId> Register<Id> for [< $reg:camel Proxy >]<Id, [< $reg:upper >]> {}

            impl<Id> [< $reg:camel Proxy >]<Id, [< $reg:upper >]> where Id: ChId, [< $reg:upper >]: target_device::generic::Readable {
                #[inline]
                #[allow(dead_code)]
                pub fn read_bit(&self) -> bool {
                    self.dmac.[< $reg:lower >].read().bits() & (1 << Id::U8) != 0
                }
            }
        }
    };

    // Read-only bit
    ($reg:ident, bit, r) => {
        paste! {
            reg_proxy!(@new $reg);
            reg_proxy!(@read_bit $reg);
        }
    };

    // Read-write bit
    ($reg:ident, bit, rw) => {
        paste! {
            reg_proxy!(@new $reg);
            reg_proxy!(@read_bit $reg);

            impl<Id> [< $reg:camel Proxy >]<Id, [< $reg:upper>]> where
                Id: ChId,
                [< $reg:upper >]: target_device::generic::Readable + target_device::generic::Writable
            {
                #[inline]
                #[allow(dead_code)]
                pub fn write_bit(&self, bit: bool) {
                    // SAFETY: This is safe because we are only writing
                    // to the bit controlled by the channel.
                    self.dmac
                        .[< $reg:lower >]
                        .modify(|r, w| unsafe { w.bits(r.bits() & ((bit as u32) << Id::U8)) });
                }
            }
        }
    };
}

reg_proxy!(chctrla, register, rw);
reg_proxy!(chctrlb, register, rw);
reg_proxy!(chintenclr, register, rw);
reg_proxy!(chintenset, register, rw);
reg_proxy!(chintflag, register, rw);
reg_proxy!(chstatus, register, r);
#[cfg(feature = "min-samd51g")]
reg_proxy!(chprilvl, register, rw);

reg_proxy!(intstatus, bit, r);
reg_proxy!(busych, bit, r);
reg_proxy!(pendch, bit, r);
reg_proxy!(swtrigctrl, bit, rw);

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
