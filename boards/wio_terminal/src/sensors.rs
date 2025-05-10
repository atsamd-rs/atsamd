use atsamd_hal::adc::Adc;
use atsamd_hal::clock::GenericClockController;
use atsamd_hal::pac::gclk::pchctrl::Genselect::Gclk11;
use atsamd_hal::pac::{Adc1, Mclk};
use atsamd_hal::prelude::*;
use atsamd_hal::sercom::{i2c, Sercom4};

use lis3dh::{Lis3dh, SlaveAddr};

use super::pins::aliases::*;

/// I2C Accelerometer pins (uses `SERCOM4`)
pub struct Accelerometer {
    /// `I2C0` bus clock pin
    pub scl: I2c0SclReset,

    /// `I2C0` bus data pin
    pub sda: I2c0SdaReset,
}

/// I2C pads for the labelled I2C peripheral
///
/// You can use these pads with other, user-defined [`i2c::Config`]urations.
pub type I2cPads = i2c::Pads<Sercom4, I2c0Sda, I2c0Scl>;

impl Accelerometer {
    /// Initialize the LIS3DH accelerometer using the correct pins and
    // peripherals. Use the driver's default settings.
    pub fn init(
        self,
        clocks: &mut GenericClockController,
        sercom4: Sercom4,
        mclk: &mut Mclk,
    ) -> Lis3dh<i2c::I2c<i2c::Config<I2cPads>>> {
        // The accelerometer is connected to the Wio Terminal's `I2C0` bus, so
        // based on the possible padouts listed in the datasheet it must use
        // `SERCOM4`.
        let gclk0 = clocks.gclk0();
        let clock = &clocks.sercom4_core(&gclk0).unwrap();
        let freq = clock.freq();
        let (sda, scl): (I2c0Sda, I2c0Scl) = (self.sda.into(), self.scl.into());
        let pads: I2cPads = i2c::Pads::new(sda, scl);
        let i2c = i2c::Config::new(mclk, sercom4, pads, freq)
            .baud(400.kHz())
            .enable();

        // The schematic states that the alternate I2C address `0x19` is used,
        // but that doesn't appear to work!
        Lis3dh::new(i2c, SlaveAddr::Default).unwrap()
    }
}

/// Analog Light Sensor
pub struct LightSensor {
    /// Analog Light Sensor input pin
    pub pd1: LightSensorAdcReset,
}

//  impl LightSensor {
//      /// Initialize Pd1 as an ADC input, and return a Tuple containing the ADC
//      /// peripheral and the configured pin.
//      pub fn init(
//          self,
//          adc: Adc1,
//          clocks: &mut GenericClockController,
//          mclk: &mut Mclk,
//      ) -> (Adc<Adc1>, LightSensorAdc) {
//          todo!()
//          // let adc1 = Adc::adc1(adc, mclk, clocks, Gclk11);
//  
//          // (adc1, self.pd1.into())
//      }
//  }
