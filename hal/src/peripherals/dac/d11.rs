//! Digital-to-Analog Converter

use pac::Pm;
use core::mem::ManuallyDrop;
use num_traits::clamp;
use pac::dac;

use crate::{
    clock::DacClock,
    gpio::{self, AlternateB, Pin, PA02, PA05},
    pac,
};

#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Error {
    /// The DAC clock exceeds the maximum
    /// of 350Khz
    ClockTooFast,
}

pub struct Dac {
    inner: pac::Dac,
}

pub struct DacWriteHandle {
    pin: Pin<PA02, AlternateB>,
    reg: ManuallyDrop<pac::Dac>,
}

impl DacWriteHandle {
    pub fn new(dac: pac::Dac, pin: Pin<PA02, AlternateB>) -> Self {
        Self {
            reg: ManuallyDrop::new(dac),
            pin,
        }
    }
}

impl Dac {
    /// Construct a new DAC. The DAC VREF is set to 3.3V
    /// (VDDANA) to allow for maximum output voltage of 3.3V
    ///
    /// This function also checks the clock frequency provided
    /// to the DAC, erroring out if the Clock exceeds the maximum
    /// DAC clock frequency:
    ///
    /// * **SAMD/E5x** - 12Mhz
    /// * **SAMC/D21** - 350Khz
    /// * **SAMD11** - 350Khz
    pub fn new(dac: pac::Dac, clk: DacClock, pm: &mut Pm) -> Result<Self, Error> {
        if clk.freq().to_Hz() > 350_000 {
            return Err(Error::ClockTooFast);
        }
        dac.ctrla().write(|w| w.swrst().set_bit());
        let s = Self { inner: dac };
        s.sync();
        s.with_disable(|dac| {
            // Set VREF
            dac.ctrlb().modify(|_, w| w.refsel().avcc());
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
        while self.inner.status().read().syncbusy().bit_is_set() {
            core::hint::spin_loop();
        }
    }

    /// Converts input voltage in millivolts to DAC output value (RAW)
    /// The input voltage is clamped between 0 and 3300mv,
    /// resulting in an output between 0 and 1024
    ///
    /// Use this function for single channel mode DAC
    pub fn voltage_to_raw(mv: u16) -> u16 {
        // Up to 4096 output
        const RATIO: f32 = 1024.0 / 3300.0;
        let targ = core::cmp::min(3300, mv) as f32;
        (targ * RATIO) as u16
    }

    /// Get a handle to DAC 0 output. This consumes PA02, since
    /// the DAC is now enabled and using this pin
    pub fn dac0(&self, pin: Pin<PA02, AlternateB>) -> DacWriteHandle {
        self.inner.ctrla().modify(|_, w| w.enable().set_bit());
        self.sync();
        DacWriteHandle::new(unsafe { core::ptr::read(&self.inner as *const _) }, pin)
    }
}

impl DacWriteHandle {
    pub fn sync(&self) {
        while self.reg.status().read().syncbusy().bit_is_set() {
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

    pub fn write_val(&mut self, val: u16) {
        unsafe {
            self.reg.data().write(|w| w.bits(val));
        }
    }

    /// Writes a voltage to the DAC.
    #[inline]
    pub fn write_voltage(&mut self, mv: u16) {
        let raw = Dac::voltage_to_raw(mv);
        self.write_val(raw);
    }

    /// Stop the DAC write handle, releasing the pin
    pub fn stop(self) -> Pin<PA02, AlternateB> {
        self.with_dac_disable(|dac| {
            dac.ctrla().modify(|_, w| w.enable().clear_bit());
        });
        self.pin
    }
}

#[cfg(feature = "dma")]
mod dma {
    use pac::dmac::chctrlb::Trigactselect as TriggerAction;
    use pac::dmac::chctrlb::Trigsrcselect as TriggerSource;

    #[cfg(feature = "async")]
    use crate::dmac::ReadyFuture;
    use crate::{
        dmac::{self, AnyChannel, Buffer, Ready},
        sercom::dma::SharedSliceBuffer,
    };

    use super::*;

    struct DacDmaPtr(pub *mut u16);

    impl DacDmaPtr {
        pub fn new(reg: &pac::Dac) -> Self {
            Self(reg.data().as_ptr())
        }
    }

    unsafe impl Buffer for DacDmaPtr {
        type Beat = u16;

        fn dma_ptr(&mut self) -> *mut Self::Beat {
            self.0
        }

        fn incrementing(&self) -> bool {
            false
        }

        fn buffer_len(&self) -> usize {
            1
        }
    }

    impl DacWriteHandle {
        /// Writes a buffer to the DAC using DMA. Each buffer value is DAC
        /// RAW output (0-1024). Use [Dac::voltage_to_raw] to convert
        /// between target voltage output of the DAC and the value to write
        /// to the DAC
        ///
        /// The samples are consumed at the sample rate of the DAC
        pub fn write_buffer_blocking<CH>(
            &mut self,
            buf: &[u16],
            channel: &mut CH,
        ) -> Result<(), dmac::Error>
        where
            CH: AnyChannel<Status = Ready>,
        {
            let mut bytes = SharedSliceBuffer::from_slice(buf);
            let trigger_action = TriggerAction::Beat;

            let mut dest = DacDmaPtr::new(&self.reg);

            unsafe {
                channel.as_mut().transfer(
                    &mut bytes,
                    &mut dest,
                    TriggerSource::DacEmpty,
                    trigger_action,
                    None,
                )?;
            }
            while channel.as_mut().xfer_complete() {
                core::hint::spin_loop();
            }
            channel.as_mut().xfer_success()
        }

        /// Writes a buffer to the DAC using DMA (Async version).
        /// Each buffer value is DAC RAW output (0-4096).
        /// Use [Dac::voltage_to_raw] to convert between target
        /// voltage output of the DAC and the value to write
        /// to the DAC
        ///
        /// The samples are consumed at the sample rate of the DAC
        #[cfg(feature = "async")]
        pub async fn write_buffer<CH>(
            &mut self,
            buf: &[u16],
            channel: &mut CH,
        ) -> Result<(), dmac::Error>
        where
            CH: AnyChannel<Status = ReadyFuture>,
        {
            let bytes = SharedSliceBuffer::from_slice(buf);
            let trigger_action = TriggerAction::Beat;
            let dest = DacDmaPtr::new(&self.reg);

            channel
                .as_mut()
                .transfer_future(bytes, dest, TriggerSource::DacEmpty, trigger_action)
                .await
        }
    }
}
