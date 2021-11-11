use crate::pins::MicOutput;
use atsamd_hal::adc::Adc;
use atsamd_hal::clock::GenericClockController;
use atsamd_hal::gpio::v2::*;
use atsamd_hal::pac::gclk::pchctrl::GEN_A::GCLK11;
use atsamd_hal::pac::{ADC1, MCLK, TCC0};
use atsamd_hal::prelude::*;
use atsamd_hal::pwm::{TCC0Pinout, Tcc0Pwm};

/// Buzzer pins
pub struct Buzzer<C>
where
    C: AnyPin<Id = PD11>,
{
    /// Buzzer control pin
    pub ctr: C,
}

impl<C> Buzzer<C>
where
    C: AnyPin<Id = PD11>,
{
    /// Initialize the pin connected to the piezo buzzer. Configure the pin as a
    /// PWM output using TCC0, and return the Pwm instance.
    pub fn init(
        self,
        clocks: &mut GenericClockController,
        tcc0: TCC0,
        mclk: &mut MCLK,
    ) -> Tcc0Pwm<PD11, AlternateF> {
        let pinout = TCC0Pinout::Pd11(self.ctr.into());

        let gclk0 = clocks.gclk0();
        let pwm0 = Tcc0Pwm::new(
            &clocks.tcc0_tcc1(&gclk0).unwrap(),
            1.khz(),
            tcc0,
            pinout,
            mclk,
        );

        pwm0
    }
}

/// Microphone pins
pub struct Microphone<M>
where
    M: Into<MicOutput>,
{
    /// Microphone output (analog input) pin
    pub mic: M,
}

impl<M> Microphone<M>
where
    M: Into<MicOutput>,
{
    /// Initialize Pd1 as an ADC input, and return a Tuple containing the ADC
    /// peripheral and the configured pin.
    pub fn init(
        self,
        adc: ADC1,
        clocks: &mut GenericClockController,
        mclk: &mut MCLK,
    ) -> (Adc<ADC1>, MicOutput) {
        let adc1 = Adc::adc1(adc, mclk, clocks, GCLK11);

        (adc1, self.mic.into())
    }
}
