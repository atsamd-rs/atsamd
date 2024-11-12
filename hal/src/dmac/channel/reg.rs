//! # Channel registers
//!
//! This module adds a [`RegisterBlock`] struct, which acts as a proxy for the
//! registers a single DMAC [`Channel`](super::Channel) can read/write. Its
//! purpose is to remediate the inadequacies of the PAC. In particular, for
//! SAMD11/SAMD21, the CHID register must be written with the correct channel ID
//! before accessing the channel specific registers. There is a provided
//! `with_chid` method that takes a closure with the register read/write proxies
//! to ensure any read/write to these registers are done in an interrupt-safe
//! way. For SAMD51+, `with_chid` returns the register block which contains the
//! registers owned by a specific channel.

use atsamd_hal_macros::{hal_cfg, hal_macro_helper};

use super::super::dma_controller::ChId;
use core::marker::PhantomData;
use paste::paste;

use crate::pac::{
    self,
    dmac::{
        busych::BusychSpec, intstatus::IntstatusSpec, pendch::PendchSpec,
        swtrigctrl::SwtrigctrlSpec,
    },
    dmac::{Busych, Intstatus, Pendch, Swtrigctrl},
    Dmac, Peripherals,
};

#[hal_cfg(any("dmac-d11", "dmac-d21"))]
use pac::dmac as channel_regs;

#[hal_cfg("dmac-d5x")]
use pac::dmac::channel as channel_regs;

use channel_regs::{
    chctrla::ChctrlaSpec, chctrlb::ChctrlbSpec, chintenclr::ChintenclrSpec,
    chintenset::ChintensetSpec, chintflag::ChintflagSpec, chstatus::ChstatusSpec,
};
use channel_regs::{Chctrla, Chctrlb, Chintenclr, Chintenset, Chintflag, Chstatus};

#[hal_cfg("dmac-d5x")]
use pac::dmac::channel::{chprilvl::ChprilvlSpec, Chprilvl};

//==============================================================================
// RegisterBlock
//==============================================================================
/// Read/write proxy for DMAC registers accessible to individual channels.
pub(super) trait Register<Id: ChId> {
    /// Get a shared reference to the underlying PAC object
    fn dmac(&self) -> &Dmac;

    /// Set channel ID and run the closure. A closure is needed to ensure
    /// the registers are accessed in an interrupt-safe way, as the SAMD21
    /// DMAC is a little funky - It requires setting the channel number in
    /// the CHID register, then access the channel control registers.
    /// If an interrupt were to change the CHID register and not reset it
    /// to the expected value, we would be faced with undefined behaviour.
    #[hal_cfg(any("dmac-d11", "dmac-d21"))]
    #[inline]
    fn with_chid<F: FnOnce(&Dmac) -> R, R>(&mut self, fun: F) -> R {
        // SAFETY: This method is ONLY safe if the individual channels are GUARANTEED
        // not to mess with either:
        // - The global DMAC configuration
        // - The configuration of other channels.
        //
        // In practice, this means that the channel-specific registers should only be
        // accessed through the `with_chid` method.

        let dmac = self.dmac();

        let mut old_id = 0;

        dmac.chid().modify(|r, w| {
            // Get the CHID contents before changing channel
            old_id = r.id().bits();
            // Change channels
            unsafe { w.id().bits(Id::U8) }
        });

        // Run the provided closure on the channel we own
        let ret = fun(dmac);
        // Restore the old CHID value. This way, if we're running `with_chid` from an
        // ISR, the CHID value will still be what the preempted context expects
        // when the method returns.
        unsafe { dmac.chid().write(|w| w.id().bits(old_id)) };

        ret
    }

    /// Set channel ID and run the closure. A closure is needed to ensure
    /// the registers are accessed in an interrupt-safe way, as the SAMD21
    /// DMAC is a little funky. For the SAMD51/SAMEx, we simply take a reference
    /// to the correct channel number and run the closure on that.
    #[hal_cfg("dmac-d5x")]
    #[inline]
    fn with_chid<F: FnOnce(&pac::dmac::Channel) -> R, R>(&mut self, fun: F) -> R {
        // SAFETY: This method is ONLY safe if the individual channels are GUARANTEED
        // not to mess with either:
        // - The global DMAC configuration
        // - The configuration of other channels.
        //
        // In practice, this means that the channel-specific registers should only be
        // accessed through the `with_chid` method.
        let ch = &self.dmac().channel(Id::USIZE);
        fun(ch)
    }
}

macro_rules! reg_proxy {
    (@new $reg:ident) => {
        paste! {
            /// Register proxy tied to a specific channel
            pub(super) struct [< $reg:camel Proxy >]<Id: ChId, REG> {
                #[allow(unused)]
                dmac: Dmac,
                _id: PhantomData<Id>,
                _reg: PhantomData<REG>,
            }

            impl<Id: ChId> [< $reg:camel Proxy >]<Id, [< $reg:camel >]> {
                /// Create a new register proxy
                #[inline]
                pub fn new() -> Self {
                    Self {
                        // SAFETY: This is safe as long as the register
                        // only reads/writes registers through
                        // the `with_chid` method.
                        dmac: unsafe { Peripherals::steal().dmac },
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
            impl<Id: ChId> Register<Id> for [< $reg:camel Proxy >]<Id, [< $reg:camel >]> {
                fn dmac(&self) -> &Dmac {
                    &self.dmac
                }
            }

            impl<Id> [< $reg:camel Proxy >]<Id, [< $reg:camel >]> where Id: ChId, [< $reg:camel Spec>]: pac::generic::Readable {
                #[inline]
                #[allow(dead_code)]
                pub fn read(&mut self) -> channel_regs::[< $reg:lower >]::R {
                    self.with_chid(|d| d.[< $reg:lower >]().read())
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

            impl<Id> [< $reg:camel Proxy >]<Id, [< $reg:camel >]> where Id: ChId, [< $reg:camel Spec >]: pac::generic::Writable {
                #[inline]
                #[allow(dead_code)]
                pub fn write<F>(&mut self, func: F)
                where
                    for<'w> F: FnOnce(&'w mut channel_regs::[< $reg:lower >]::W) -> &'w mut channel_regs::[< $reg:lower >]::W,
                {
                    self.with_chid(|d| d.[< $reg:lower >]().write(|w| func(w)));
                }
            }

            impl<Id>[< $reg:camel Proxy >]<Id, [< $reg:camel >]> where
                Id: ChId,
                [< $reg:camel Spec >]: pac::generic::Writable + pac::generic::Readable
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
                    self.with_chid(|d| d.[< $reg:lower >]().modify(|r, w| func(r, w)));
                }
            }
        }
    };

    // Internal rule for read-enabled bit
    (@read_bit $reg:ident) => {
        paste! {
            impl<Id: ChId> Register<Id> for [< $reg:camel Proxy >]<Id, [< $reg:camel >]> {
                fn dmac(&self) -> &Dmac {
                    &self.dmac
                }
            }

            impl<Id> [< $reg:camel Proxy >]<Id, [< $reg:camel >]> where Id: ChId, [< $reg:camel Spec >]: pac::generic::Readable {
                #[inline]
                #[allow(dead_code)]
                pub fn read_bit(&self) -> bool {
                    self.dmac.[< $reg:lower >]().read().bits() & (1 << Id::U8) != 0
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

            impl<Id> [< $reg:camel Proxy >]<Id, [< $reg:camel >]> where
                Id: ChId,
                [< $reg:camel Spec >]: pac::generic::Readable + pac::generic::Writable
            {
                #[inline]
                #[allow(dead_code)]
                pub fn set_bit(&mut self) {
                    // SAFETY: This is safe because we are only writing
                    // to the bit controlled by the channel.
                    unsafe {
                        self.dmac.[< $reg:lower >]().modify(|r, w| w.bits(r.bits() | (1 << Id::U8)));
                    }
                }

                #[inline]
                #[allow(dead_code)]
                pub fn clear_bit(&mut self) {
                    // SAFETY: This is safe because we are only writing
                    // to the bit controlled by the channel.
                    unsafe {
                        self.dmac.[< $reg:lower >]().modify(|r, w| w.bits(r.bits() & !(1 << Id::U8)));
                    }
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
#[hal_cfg("dmac-d5x")]
reg_proxy!(chprilvl, register, rw);

reg_proxy!(intstatus, bit, r);
reg_proxy!(busych, bit, r);
reg_proxy!(pendch, bit, r);
reg_proxy!(swtrigctrl, bit, rw);

/// Acts as a proxy to the PAC DMAC object. Only registers and bits
/// within registers that should be readable/writable by specific
/// [`Channel`]s are exposed.
///
/// This struct implements [`Drop`]. Dropping this struct will stop
/// any ongoing transfers for the channel which it represents.
#[allow(dead_code)]
#[hal_macro_helper]
pub(super) struct RegisterBlock<Id: ChId> {
    pub chctrla: ChctrlaProxy<Id, Chctrla>,
    pub chctrlb: ChctrlbProxy<Id, Chctrlb>,
    pub chintenclr: ChintenclrProxy<Id, Chintenclr>,
    pub chintenset: ChintensetProxy<Id, Chintenset>,
    pub chintflag: ChintflagProxy<Id, Chintflag>,
    pub chstatus: ChstatusProxy<Id, Chstatus>,
    pub intstatus: IntstatusProxy<Id, Intstatus>,
    pub busych: BusychProxy<Id, Busych>,
    pub pendch: PendchProxy<Id, Pendch>,
    pub swtrigctrl: SwtrigctrlProxy<Id, Swtrigctrl>,
    #[hal_cfg("dmac-d5x")]
    pub chprilvl: ChprilvlProxy<Id, Chprilvl>,
}

impl<Id: ChId> RegisterBlock<Id> {
    #[hal_macro_helper]
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
            #[hal_cfg("dmac-d5x")]
            chprilvl: ChprilvlProxy::new(),
        }
    }
}

impl<Id: ChId> Drop for RegisterBlock<Id> {
    fn drop(&mut self) {
        // Stop any potentially ongoing transfers
        self.chctrla.modify(|_, w| w.enable().clear_bit());

        while self.chctrla.read().enable().bit_is_set() {
            core::hint::spin_loop();
        }

        // Prevent the compiler from re-ordering read/write
        // operations beyond this fence.
        // (see https://docs.rust-embedded.org/embedonomicon/dma.html#compiler-misoptimizations)
        core::sync::atomic::fence(core::sync::atomic::Ordering::Acquire); // ▼
    }
}
