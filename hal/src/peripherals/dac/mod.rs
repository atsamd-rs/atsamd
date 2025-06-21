//! Digital-to-Analog Converter

use core::{marker::PhantomData, mem::ManuallyDrop, ops::Deref};

use atsamd_hal_macros::{hal_cfg, hal_module};
use pac::{dac, Peripherals};

use crate::{
    clock::v2::{pclk::Pclk, types},
    gpio::{self, AlternateB, AnyPin, Output, Pin, PushPullOutput, PA02, PA05},
    pac,
    typelevel::{NoneT, Sealed},
};

#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Error {
    /// The DAC clock exceeds the maximum
    /// of 12Mhz
    ClockTooFast,
}

pub struct Dac {
    inner: pac::Dac,
}

pub trait DacInstance {
    type PinId: gpio::AnyPin;
    const IDX: usize;

    fn data(reg: &pac::Dac) -> bool;
    fn ready(reg: &pac::Dac) -> bool;
    fn eoc(reg: &pac::Dac) -> bool;
}

pub struct Single<D: DacInstance> {
    pin: D::PinId,
}
pub struct Differential<D0: DacInstance, D1: DacInstance> {
    pin_0: D0::PinId,
    pin_1: D1::PinId,
}

pub trait DacMode {}

impl<D: DacInstance> DacMode for Single<D> {}
impl<D0: DacInstance, D1: DacInstance> DacMode for Differential<D0, D1> {}

pub struct DacWriteHandle<M: DacMode> {
    reg: ManuallyDrop<pac::Dac>,
    mode: M,
}

impl<M: DacMode> DacWriteHandle<M> {
    pub fn new(dac: pac::Dac, mode: M) -> Self {
        Self {
            reg: ManuallyDrop::new(dac),
            mode,
        }
    }
}

pub struct Dac0;
pub struct Dac1;

impl DacInstance for Dac0 {
    type PinId = Pin<PA02, AlternateB>;
    const IDX: usize = 0;
    fn ready(reg: &pac::Dac) -> bool {
        reg.status().read().ready0().bit_is_set()
    }

    fn data(reg: &pac::Dac) -> bool {
        reg.syncbusy().read().data0().bit_is_set()
    }

    fn eoc(reg: &pac::Dac) -> bool {
        reg.status().read().eoc0().bit_is_set()
    }
}

impl DacInstance for Dac1 {
    type PinId = Pin<PA05, AlternateB>;
    const IDX: usize = 1;
    fn ready(reg: &pac::Dac) -> bool {
        reg.status().read().ready1().bit_is_set()
    }

    fn data(reg: &pac::Dac) -> bool {
        reg.syncbusy().read().data1().bit_is_set()
    }

    fn eoc(reg: &pac::Dac) -> bool {
        reg.status().read().eoc1().bit_is_set()
    }
}

impl Dac {
    pub fn new<PS: crate::clock::v2::pclk::PclkSourceId>(
        dac: pac::Dac,
        clk: crate::clock::v2::apb::ApbClk<types::Dac>,
        pclk: &crate::clock::v2::pclk::Pclk<types::Dac, PS>,
    ) -> Result<Self, Error> {
        if pclk.freq().to_Hz() > 12_000_000 {
            return Err(Error::ClockTooFast);
        }
        dac.ctrla().write(|w| w.swrst().set_bit());
        let mut s = Self { inner: dac };
        s.sync();
        s.with_disable(|dac| {
            // Set VREF
            dac.ctrlb().modify(|_, w| w.refsel().vddana());

            // Setup CC size based on GCLK Freq
            let cc = match pclk.freq().to_Hz() {
                ..1_200_000 => dac::dacctrl::Cctrlselect::Cc100k,
                1_200_000..=6_000_000 => dac::dacctrl::Cctrlselect::Cc1m,
                _ => dac::dacctrl::Cctrlselect::Cc12m,
            };
            dac.dacctrl(0).modify(|_, w| w.cctrl().variant(cc));
            dac.dacctrl(1).modify(|_, w| w.cctrl().variant(cc));
        });
        Ok(s)
    }

    pub(crate) fn with_disable<R, F: FnOnce(&pac::Dac) -> R>(&self, f: F) -> R {
        self.inner.ctrla().write(|w| w.enable().clear_bit());
        self.sync();
        let ret = f(&self.inner);
        self.inner.ctrla().write(|w| w.enable().set_bit());
        self.sync();
        ret
    }

    pub fn sync(&self) {
        while self.inner.syncbusy().read().bits() != 0 {
            core::hint::spin_loop();
        }
    }

    pub fn differential(
        &self,
        d0: Pin<PA02, AlternateB>,
        d1: Pin<PA05, AlternateB>,
    ) -> DacWriteHandle<Differential<Dac0, Dac1>> {
        self.with_disable(|dac| {
            dac.ctrlb().modify(|_, w| w.diff().set_bit());
            dac.dacctrl(0).modify(|_, w| w.enable().set_bit());
        });
        while !Dac0::ready(&self.inner) {
            core::hint::spin_loop();
        }
        DacWriteHandle::new(
            unsafe { core::ptr::read(&self.inner as *const _) },
            Differential {
                pin_0: d0,
                pin_1: d1,
            },
        )
    }

    pub fn dac0(&self, pin: Pin<PA02, AlternateB>) -> DacWriteHandle<Single<Dac0>> {
        self.with_disable(|dac| {
            dac.ctrlb().modify(|_, w| w.diff().clear_bit());
            dac.dacctrl(Dac0::IDX).modify(|_, w| w.enable().set_bit())
        });
        while !Dac0::ready(&self.inner) {
            core::hint::spin_loop();
        }
        DacWriteHandle::new(
            unsafe { core::ptr::read(&self.inner as *const _) },
            Single { pin },
        )
    }

    pub fn dac1(&self, pin: Pin<PA05, AlternateB>) -> DacWriteHandle<Single<Dac1>> {
        self.with_disable(|dac| {
            dac.ctrlb().modify(|_, w| w.diff().clear_bit());
            dac.dacctrl(Dac1::IDX).modify(|_, w| w.enable().set_bit())
        });
        while !Dac1::ready(&self.inner) {
            core::hint::spin_loop();
        }
        DacWriteHandle::new(
            unsafe { core::ptr::read(&self.inner as *const _) },
            Single { pin },
        )
    }
}

// For all modes
impl<M: DacMode> DacWriteHandle<M> {
    pub fn sync(&self) {
        while self.reg.syncbusy().read().bits() != 0 {
            core::hint::spin_loop();
        }
    }

    pub(crate) fn with_dac_disable<R, F: FnOnce(&pac::Dac) -> R>(&self, f: F) -> R {
        self.reg.ctrla().write(|w| w.enable().clear_bit());
        self.sync();
        let ret = f(&self.reg);
        self.reg.ctrla().write(|w| w.enable().set_bit());
        self.sync();
        ret
    }
}

// For single DAC modes (Dac0/Dac1)
impl<DAC: DacInstance> DacWriteHandle<Single<DAC>> {
    pub fn write(&mut self, val: u16) {
        unsafe {
            self.reg.data(DAC::IDX).write(|w| w.bits(val));
        }
        while DAC::data(&self.reg) {
            core::hint::spin_loop();
        }
        while !DAC::eoc(&self.reg) {
            core::hint::spin_loop();
        }
    }

    pub fn stop(self) -> DAC::PinId {
        self.with_dac_disable(|dac| {
            dac.dacctrl(DAC::IDX).modify(|_, w| w.enable().clear_bit());
        });
        self.mode.pin
    }
}

// For differential mode
impl<D0: DacInstance, D1: DacInstance> DacWriteHandle<Differential<D0, D1>> {
    pub fn write(&mut self, val: i16) {
        // Manipulation for differential mode is done via DAC0 channel info
        unsafe {
            self.reg.data(0).write(|w| w.bits(val as u16));
        }
        while Dac0::data(&self.reg) {
            core::hint::spin_loop();
        }
        while !Dac0::eoc(&self.reg) {
            core::hint::spin_loop();
        }
    }

    pub fn stop(self) -> (D0::PinId, D1::PinId) {
        self.with_dac_disable(|dac| {
            dac.dacctrl(0).modify(|_, w| w.enable().clear_bit());
        });
        (self.mode.pin_0, self.mode.pin_1)
    }
}

#[cfg(feature = "dma")]
mod dma {
    // TODO - DMA
}
