use crate::clock::GenericClockController;
use crate::gpio::*;
use crate::pac::{dac, DAC, PM};

/// Reference voltage
pub use dac::ctrlb::REFSEL_A as Reference;

/// `Dac` encapsulates the DAC module
pub struct Dac {
    dac: DAC,
    pin: Pin<PA02, AlternateB>,
    v_ref: f32,
}

impl Dac {
    /// Create a new [`Dac`] instance. The default configuration is:
    ///
    /// * Reference voltage = 1.0 V, Internal voltage reference is used (INT1V)
    ///
    /// **Note**:  INT1V is the buffered internal reference of 1.0V. The voltage
    /// reference configuration can be modified with
    /// [`set_reference`](Self::set_reference).
    pub fn new(
        dac: DAC,
        pin: impl AnyPin<Id = PA02>,
        pm: &mut PM,
        clocks: &mut GenericClockController,
    ) -> Self {
        pm.apbcmask.modify(|_, w| w.dac_().set_bit());

        let gclk0 = clocks.gclk0();
        clocks.dac(&gclk0).expect("DAC clock setup failed");

        let mut newdac = Self {
            dac,
            pin: pin.into().into_alternate(),
            v_ref: 1.0,
        };

        newdac.syncbusy();

        newdac.dac.ctrla.modify(|_, w| w.swrst().set_bit());
        newdac.syncbusy();

        // the dac output is enable by writting a one to CTRLB.EOEN
        newdac.dac.ctrlb.modify(|_, w| w.eoen().set_bit());
        newdac.power_up();
        newdac
    }

    /// Sets the DAC's voltage reference. The conversion range is between GND
    /// and the selected DAC voltage reference.
    pub fn set_reference(&mut self, reference: Reference, mut v_ref: f32) {
        self.power_down();
        if let Reference::INT1V = reference {
            v_ref = 1.0;
        }
        self.v_ref = v_ref;
        self.dac.ctrlb.modify(|_, w| w.refsel().variant(reference));
        self.power_up();
    }

    /// Sets the voltage output by the DAC. The resolution is 10 bits
    /// internally.
    pub fn set_voltage(&mut self, v_out: f32) {
        let value = self.voltage_as_u16(v_out);
        self.dac.data.modify(|_, w| unsafe { w.data().bits(value) });
        self.syncbusy();
    }

    pub fn set_voltage_raw(&mut self, v_out: u16) {
        self.dac.data.modify(|_, w| unsafe { w.data().bits(v_out) });
        self.syncbusy();
    }

    /// Release the resources owned by the [`Dac`]. The DAC peripheral is reset
    /// before being released.
    pub fn free(mut self) -> (DAC, Pin<PA02, AlternateB>) {
        self.dac.ctrla.modify(|_, w| w.swrst().set_bit());
        self.syncbusy();
        (self.dac, self.pin)
    }

    /// The DAC Controller is enabled by writing a '1' to the Enable bit in the
    /// Control A register (CTRLA.ENABLE)
    fn power_up(&mut self) {
        self.syncbusy();
        self.dac.ctrla.modify(|_, w| w.enable().set_bit());
        self.syncbusy();
    }

    /// The DAC Controller is disabled by writing a '0' to CTRLA.ENABLE.
    fn power_down(&mut self) {
        self.syncbusy();
        self.dac.ctrla.modify(|_, w| w.enable().clear_bit());
        self.syncbusy();
    }

    /// some registers must be synchronized when written or read
    fn syncbusy(&mut self) {
        while self.dac.status.read().syncbusy().bit_is_set() {
            core::hint::spin_loop();
        }
    }

    /// The output voltage from the DAC can be calculated using the function
    /// [`voltage_as_u16`](Self::voltage_as_u16)
    fn voltage_as_u16(&self, v_out: f32) -> u16 {
        // 0x3FF is the maximum for a 10 bit number
        ((v_out / self.v_ref) * 1024.0) as u16
    }
}

#[cfg(feature = "dma")]
mod dma {
    use super::Dac;
    use crate::dmac::{
        AnyChannel, Buffer, BufferPair, Busy, Channel, Ready, Transfer, TriggerAction,
        TriggerSource,
    };

    unsafe impl Buffer for Dac {
        type Beat = u16;

        #[inline]
        fn dma_ptr(&mut self) -> *mut Self::Beat {
            self.dac.data.as_ptr()
        }

        #[inline]
        fn incrementing(&self) -> bool {
            false
        }

        #[inline]
        fn buffer_len(&self) -> usize {
            1
        }
    }

    impl Dac {
        /// This function is used for transferring values with DMA.
        /// 'self' represents the DAC, 'buffer' is the values to be returned, and 'channel' is chosen according to the needs from 0 to 11.
        /// Trigger source must be determined in order to control the triggering of the data transfer.
        /// 'circular', true or false, the function will be executed in a loop.
        pub fn transfer_with_dma<B, Ch>(
            self,
            buffer: B,
            channel: Ch,
            trigger_source: TriggerSource,
            circular: bool,
        ) -> Transfer<Channel<Ch::Id, Busy>, BufferPair<Self, B>, ()>
        where
            // Allows the user to choose the type without restrictions
            Ch: AnyChannel<Status = Ready> + 'static,
            B: Buffer<Beat = <Self as Buffer>::Beat> + 'static,
        {
            // Whenever triggered, the DMA will transfer a BEAT containing a u16 value.
            // It's important to remember that the DAC has a 10-bit limit.

            let trigger_action = TriggerAction::BEAT;

            // SAFETY: This is safe because the of the `'static` bound check
            // for `B`, and the fact that the buffer length of an `Dac` is always 1.
            let xfer = unsafe { Transfer::new_unchecked(channel, self, buffer, circular) };
            xfer.begin(trigger_source, trigger_action)
        }
    }
}
