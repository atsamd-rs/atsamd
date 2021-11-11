use crate::pins::{I2c0Scl, I2c0Sda, LightSensorAdc};
use atsamd_hal::adc::Adc;
use atsamd_hal::clock::GenericClockController;
use atsamd_hal::gpio::v2::*;
use atsamd_hal::pac::gclk::pchctrl::GEN_A::GCLK11;
use atsamd_hal::pac::{ADC1, MCLK, SERCOM4};
use atsamd_hal::prelude::*;
use atsamd_hal::sercom::{I2CMaster4, PadPin, Sercom4Pad0, Sercom4Pad1};

use lis3dh::{Lis3dh, SlaveAddr};

/// I2C Accelerometer pins (uses `SERCOM4`)
pub struct Accelerometer<C, D>
where
    C: Into<I2c0Scl>,
    D: Into<I2c0Sda>,
{
    /// `I2C0` bus clock pin
    pub scl: C,

    /// `I2C0` bus data pin
    pub sda: D,
}

impl<C, D> Accelerometer<C, D>
where
    C: Into<I2c0Scl>,
    D: Into<I2c0Sda>,
{
    /// Initialize the LIS3DH accelerometer using the correct pins and
    // peripherals. Use the driver's default settings.
    pub fn init(
        self,
        clocks: &mut GenericClockController,
        sercom4: SERCOM4,
        mclk: &mut MCLK,
    ) -> Lis3dh<I2CMaster4<I2c0Sda, I2c0Scl>> {
        // The accelerometer is connected to the Wio Terminal's `I2C0` bus, so
        // based on the possible padouts listed in the datasheet it must use
        // `SERCOM4` and in turn `I2CMaster4`.
        let gclk0 = clocks.gclk0();
        let i2c = I2CMaster4::new(
            &clocks.sercom4_core(&gclk0).unwrap(),
            400.khz(),
            sercom4,
            mclk,
            self.sda.into(),
            self.scl.into(),
        );

        // The schematic states that the alternate I2C address `0x19` is used,
        // but that doesn't appear to work!
        Lis3dh::new(i2c, SlaveAddr::Default).unwrap()
    }
}

/// Analog Light Sensor
pub struct LightSensor<P>
where
    P: Into<LightSensorAdc>,
{
    /// Analog Light Sensor input pin
    pub pd1: P,
}

impl<P> LightSensor<P>
where
    P: Into<LightSensorAdc>,
{
    /// Initialize Pd1 as an ADC input, and return a Tuple containing the ADC
    /// peripheral and the configured pin.
    pub fn init(
        self,
        adc: ADC1,
        clocks: &mut GenericClockController,
        mclk: &mut MCLK,
    ) -> (Adc<ADC1>, Pin<PD01, Alternate<B>>) {
        let adc1 = Adc::adc1(adc, mclk, clocks, GCLK11);

        (adc1, self.pd1.into().into())
    }
}
