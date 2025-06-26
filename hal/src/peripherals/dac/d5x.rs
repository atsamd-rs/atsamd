//! Digital-to-Analog Converter

use crate::clock::v2::types;
use num_traits::clamp;
use pac::dac;

#[cfg(feature = "dma")]
use pac::dmac::channel::chctrla::Trigsrcselect as TriggerSource;

use crate::{
    gpio::{self, AlternateB, Pin, PA02, PA05},
    pac,
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
    clk: crate::clock::v2::apb::ApbClk<types::Dac>,
}

pub trait DacInstance {
    type PinId: gpio::AnyPin;
    const IDX: usize;

    #[cfg(feature = "dma")]
    const DMA_TX_TRIGGER: TriggerSource;

    fn data_ready(reg: &pac::Dac) -> bool;
    fn ready(reg: &pac::Dac) -> bool;
    fn eoc(reg: &pac::Dac) -> bool;

    #[cfg(feature = "dma")]
    fn dma_data_ptr(reg: &pac::Dac) -> *mut u16;
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

pub struct DacWriteHandle<'a, M: DacMode> {
    reg: &'a pac::Dac,
    mode: M,
}

impl<'a, M: DacMode> DacWriteHandle<'a, M> {
    pub fn new(dac: &'a pac::Dac, mode: M) -> Self {
        Self { reg: dac, mode }
    }
}

pub struct Dac0;

pub struct Dac1;

impl DacInstance for Dac0 {
    type PinId = Pin<PA02, AlternateB>;
    const IDX: usize = 0;

    #[cfg(feature = "dma")]
    const DMA_TX_TRIGGER: TriggerSource = TriggerSource::DacEmpty0;

    fn ready(reg: &pac::Dac) -> bool {
        reg.status().read().ready0().bit_is_set()
    }

    fn data_ready(reg: &pac::Dac) -> bool {
        reg.syncbusy().read().data0().bit_is_set()
    }

    fn eoc(reg: &pac::Dac) -> bool {
        reg.status().read().eoc0().bit_is_set()
    }

    #[cfg(feature = "dma")]
    fn dma_data_ptr(reg: &pac::Dac) -> *mut u16 {
        reg.data(0).as_ptr()
    }
}

impl DacInstance for Dac1 {
    type PinId = Pin<PA05, AlternateB>;
    const IDX: usize = 1;

    #[cfg(feature = "dma")]
    const DMA_TX_TRIGGER: TriggerSource = TriggerSource::DacEmpty1;

    fn ready(reg: &pac::Dac) -> bool {
        reg.status().read().ready1().bit_is_set()
    }

    fn data_ready(reg: &pac::Dac) -> bool {
        reg.syncbusy().read().data1().bit_is_set()
    }

    fn eoc(reg: &pac::Dac) -> bool {
        reg.status().read().eoc1().bit_is_set()
    }

    #[cfg(feature = "dma")]
    fn dma_data_ptr(reg: &pac::Dac) -> *mut u16 {
        reg.data(1).as_ptr()
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
    pub fn new<PS: crate::clock::v2::pclk::PclkSourceId>(
        dac: pac::Dac,
        clk: crate::clock::v2::apb::ApbClk<types::Dac>,
        pclk: &crate::clock::v2::pclk::Pclk<types::Dac, PS>,
    ) -> Result<Self, Error> {
        // TODO: Ideally, the DAC struct would take ownership of the Pclk type here.
        // However, since clock::v2 is not implemented for all chips yet, the
        // generics for the Adc type would be different between chip families,
        // leading to massive and unnecessary code duplication. In the meantime,
        // we use a "lite" variation of the typelevel guarantees laid out by the
        // clock::v2 module, meaning that we can guarantee that the clocks are enabled
        // at the time of creation of the Adc struct; however we can't guarantee
        // that the clock will stay enabled for the duration of its lifetime.
        let pclk_freq = pclk.freq().to_Hz();
        if pclk_freq > 12_000_000 {
            return Err(Error::ClockTooFast);
        }
        dac.ctrla().write(|w| w.swrst().set_bit());

        let s = Self {
            inner: dac,
            clk,
        };
        s.sync();
        s.with_disable(|dac| {
            // Set VREF
            dac.ctrlb().modify(|_, w| w.refsel().vddana());

            // Setup CC size based on GCLK Freq
            let cc = match pclk_freq {
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

    /// Converts input voltage in millivolts to DAC output value (RAW)
    /// The input voltage is clamped between 0 and 3300mv,
    /// resulting in an output between 0 and 4096
    ///
    /// Use this function for single channel mode DAC
    pub fn voltage_to_raw(mv: u16) -> u16 {
        // Up to 4096 output
        const RATIO: f32 = 4096.0 / 3300.0;
        let targ = core::cmp::min(3300, mv) as f32;
        (targ * RATIO) as u16
    }

    /// Converts input voltage in millivolts to DAC output value (RAW)
    /// The input voltage is clamped between -1650mv and +1650mv,
    /// resulting in an output between -2048 and +2048
    ///
    /// Use this function for differential mode DAC
    pub fn voltage_to_raw_signed(mv: i16) -> i16 {
        // Up to +/-2048 output
        const RATIO: f32 = 4096.0 / 3300.0;
        let targ = clamp(mv, -1650, 1650) as f32;
        (targ * RATIO) as i16
    }

    /// Sets the DAC up in differential mode.
    ///
    /// In this mode, Dac0 is output D0, and Dac1 is output D1.
    ///
    /// When writing to the Differential pair, The baseline voltage
    /// is 1.65V, with D0 writing `1.65+x`, and D1 writing `1.65-x`.
    /// This is why in differential mode, the write value is signed.
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
            &self.inner,
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
        DacWriteHandle::new(&self.inner, Single { pin })
    }

    pub fn dac1(&self, pin: Pin<PA05, AlternateB>) -> DacWriteHandle<Single<Dac1>> {
        self.with_disable(|dac| {
            dac.ctrlb().modify(|_, w| w.diff().clear_bit());
            dac.dacctrl(Dac1::IDX).modify(|_, w| w.enable().set_bit())
        });
        while !Dac1::ready(&self.inner) {
            core::hint::spin_loop();
        }
        DacWriteHandle::new(&self.inner, Single { pin })
    }

    /// Destroy the DAC peripheral, returning resources.
    /// The DAC will be disabled and reset before being released
    pub fn release(
        self,
    ) -> (
        pac::Dac,
        crate::clock::v2::apb::ApbClk<types::Dac>
    ) {
        self.inner.ctrla().modify(|_, w| w.enable().clear_bit());
        self.sync();
        self.inner.ctrla().write(|w| w.swrst().set_bit());
        self.sync();
        (self.inner, self.clk)
    }
}

// For all modes
impl<M: DacMode> DacWriteHandle<'_, M> {
    pub fn sync(&self) {
        while self.reg.syncbusy().read().bits() != 0 {
            core::hint::spin_loop();
        }
    }

    pub(crate) fn with_dac_disable<R, F: FnOnce(&pac::Dac) -> R>(&self, f: F) -> R {
        self.reg.ctrla().write(|w| w.enable().clear_bit());
        self.sync();
        let ret = f(self.reg);
        self.reg.ctrla().write(|w| w.enable().set_bit());
        self.sync();
        ret
    }
}

// For single DAC modes (Dac0/Dac1)
impl<DAC: DacInstance> DacWriteHandle<'_, Single<DAC>> {
    /// Writes a raw value to the DAC. Input value is between
    /// 0 and 4096. If you want the DAC to output a specific
    /// voltage, use [Self::write_voltage] instead.
    pub fn write_val(&mut self, val: u16) {
        unsafe {
            self.reg.data(DAC::IDX).write(|w| w.bits(val));
        }
        while DAC::data_ready(self.reg) {
            core::hint::spin_loop();
        }
        while !DAC::eoc(self.reg) {
            core::hint::spin_loop();
        }
    }

    /// Writes a voltage to the DAC.
    #[inline]
    pub fn write_voltage(&mut self, mv: u16) {
        let raw = Dac::voltage_to_raw(mv);
        self.write_val(raw);
    }

    /// Stop the DAC in single mode, releasing the pin
    pub fn stop(self) -> DAC::PinId {
        self.with_dac_disable(|dac| {
            dac.dacctrl(DAC::IDX).modify(|_, w| w.enable().clear_bit());
        });
        self.mode.pin
    }
}

// For differential mode
impl<D0: DacInstance, D1: DacInstance> DacWriteHandle<'_, Differential<D0, D1>> {
    /// Writes a raw value to the DAC. Input value is between
    /// -2048 and +2048. If you want the DAC to output a specific
    /// voltage, use [Self::write_voltage] instead.
    pub fn write_val(&mut self, val: i16) {
        // Manipulation for differential mode is done via DAC0 channel info
        unsafe {
            self.reg.data(0).write(|w| w.bits(val as u16));
        }
        while Dac0::data_ready(self.reg) {
            core::hint::spin_loop();
        }
        while !Dac0::eoc(self.reg) {
            core::hint::spin_loop();
        }
    }

    /// Writes a voltage to the DAC.
    #[inline]
    pub fn write_voltage(&mut self, mv: i16) {
        let raw = Dac::voltage_to_raw_signed(mv);
        self.write_val(raw);
    }

    /// Stop the DAC in differential mode, releasing both pins
    pub fn stop(self) -> (D0::PinId, D1::PinId) {
        self.with_dac_disable(|dac| {
            dac.dacctrl(0).modify(|_, w| w.enable().clear_bit());
        });
        (self.mode.pin_0, self.mode.pin_1)
    }
}

#[cfg(feature = "dma")]
mod dma {
    use pac::dmac::channel::chctrla::Trigactselect as TriggerAction;

    #[cfg(feature = "async")]
    use crate::dmac::ReadyFuture;
    use crate::{
        dmac::{self, AnyChannel, Buffer, Ready},
        sercom::dma::SharedSliceBuffer,
    };

    use super::*;

    struct DacDmaPtr(pub *mut u16);

    impl DacDmaPtr {
        pub fn new<D: DacInstance>(reg: &pac::Dac) -> Self {
            Self(D::dma_data_ptr(reg))
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

    impl<DAC: DacInstance> DacWriteHandle<'_, Single<DAC>> {
        /// Writes a buffer to the DAC using DMA. Each buffer value is DAC
        /// RAW output (0-4096). Use [Dac::voltage_to_raw] to convert
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
            let trigger_action = TriggerAction::Burst;
            let mut dest = DacDmaPtr::new::<DAC>(self.reg);

            unsafe {
                channel.as_mut().transfer(
                    &mut bytes,
                    &mut dest,
                    DAC::DMA_TX_TRIGGER,
                    trigger_action,
                    None,
                )
            }
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
            let trigger_action = TriggerAction::Burst;
            let dest = DacDmaPtr::new::<DAC>(self.reg);

            channel
                .as_mut()
                .transfer_future(bytes, dest, DAC::DMA_TX_TRIGGER, trigger_action)
                .await
        }
    }

    impl<D0: DacInstance, D1: DacInstance> DacWriteHandle<'_, Differential<D0, D1>> {
        /// Writes a buffer to the DAC using DMA
        /// Each buffer value is DAC Differential output (-2048-+2048).
        /// Use `[Dac::voltage_to_raw_signed]` to convert between target
        /// voltage output of the DAC and the value to write
        /// to the DAC.
        ///
        /// The samples are consumed at the sample rate of the DAC
        pub fn write_buffer_blocking<CH>(
            &mut self,
            buf: &[i16],
            channel: &mut CH,
        ) -> Result<(), dmac::Error>
        where
            CH: AnyChannel<Status = Ready>,
        {
            // SAFETY - This case is safe, since it just allows
            // us to not have a new DacDmaPtr type with i16.
            // The DAC interprets the data as i16 for differential mode
            let mut bytes = SharedSliceBuffer::from_slice(unsafe {
                core::mem::transmute::<&[i16], &[u16]>(buf)
            });

            let trigger_action = TriggerAction::Burst;

            let mut dest = DacDmaPtr::new::<D0>(self.reg);

            unsafe {
                channel.as_mut().transfer(
                    &mut bytes,
                    &mut dest,
                    D0::DMA_TX_TRIGGER,
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
        /// Each buffer value is DAC Differential output (-2048-+2048).
        /// Use `[Dac::voltage_to_raw_signed]` to convert between target
        /// voltage output of the DAC and the value to write
        /// to the DAC.
        ///
        /// The samples are consumed at the sample rate of the DAC
        #[cfg(feature = "async")]
        pub async fn write_buffer<CH>(
            &mut self,
            buf: &[i16],
            channel: &mut CH,
        ) -> Result<(), dmac::Error>
        where
            CH: AnyChannel<Status = ReadyFuture>,
        {
            // SAFETY - This case is safe, since it just allows
            // us to not have a new DacDmaPtr type with i16.
            // The DAC interprets the data as i16 for differential mode
            let bytes = SharedSliceBuffer::from_slice(unsafe {
                core::mem::transmute::<&[i16], &[u16]>(buf)
            });

            let trigger_action = TriggerAction::Burst;

            let dest = DacDmaPtr::new::<D0>(self.reg);

            channel
                .as_mut()
                .transfer_future(bytes, dest, D0::DMA_TX_TRIGGER, trigger_action)
                .await
        }
    }
}
