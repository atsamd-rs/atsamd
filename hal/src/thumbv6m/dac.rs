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
        (v_out / self.v_ref) as u16 * 0x3FF
    }
}
