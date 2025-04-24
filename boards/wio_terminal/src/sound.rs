use atsamd_hal::adc::Adc;
use atsamd_hal::clock::GenericClockController;
use atsamd_hal::pac::gclk::pchctrl::Genselect::Gclk11;
use atsamd_hal::pac::{Adc1, Mclk, Tcc0};
use atsamd_hal::prelude::*;
use atsamd_hal::pwm::{TCC0Pinout, Tcc0Pwm};

use super::pins::aliases::*;

/// Buzzer pins
pub struct Buzzer {
    /// Buzzer control pin
    pub ctr: BuzzerCtrlReset,
}

impl Buzzer {
    /// Initialize the pin connected to the piezo buzzer. Configure the pin as a
    /// PWM output using TCC0, and return the Pwm instance.
    pub fn init(
        self,
        clocks: &mut GenericClockController,
        tcc0: Tcc0,
        mclk: &mut Mclk,
    ) -> Tcc0Pwm<BuzzerCtrlId, BuzzerCtrlMode> {
        let pinout = TCC0Pinout::Pd11(self.ctr);

        let gclk0 = clocks.gclk0();
        let pwm0 = Tcc0Pwm::new(
            &clocks.tcc0_tcc1(&gclk0).unwrap(),
            1.kHz(),
            tcc0,
            pinout,
            mclk,
        );

        pwm0
    }
}

/// Microphone pins
pub struct Microphone {
    /// Microphone output (analog input) pin
    pub mic: MicOutputReset,
}

impl Microphone {
    /// Initialize Pd1 as an ADC input, and return a Tuple containing the ADC
    /// peripheral and the configured pin.
    pub fn init(
        self,
        adc: Adc1,
        clocks: &mut GenericClockController,
        mclk: &mut Mclk,
    ) -> (Adc<Adc1>, MicOutput) {
        let adc1 = Adc::adc1(adc, mclk, clocks, Gclk11);

        (adc1, self.mic.into())
    }
}
