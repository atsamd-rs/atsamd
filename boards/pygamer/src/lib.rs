#![no_std]
#![recursion_limit = "1024"]

pub mod pins;
use atsamd_hal as hal;

#[cfg(feature = "rt")]
pub use cortex_m_rt::entry;

pub use crate::pins::Pins;
pub use hal::target_device::*;
pub use hal::*;

use hal::prelude::*;
use gpio::{Floating, Input, Port};
use hal::clock::GenericClockController;
use hal::sercom::{I2CMaster2, PadPin, SPIMaster1, SPIMaster4};
use hal::time::Hertz;
use hal::timer::TimerCounter;
use hal::pwm::Pwm2;
use cortex_m::peripheral::SYST;

use st7735_lcd::{Orientation, ST7735};

#[cfg(feature = "usb")]
pub use hal::usb::UsbBus;
#[cfg(feature = "usb")]
use usb_device::bus::UsbBusWrapper;

/// This powers up SERCOM1 and configures it for use as an
/// SPI Master in SPI Mode 0.
/// This function does not accept a CS pin; configuring a pin for CS
/// is the responsibility of the caller, because it could be
/// any OutputPin, or even a pulled up line on the slave.
pub fn spi_master<F: Into<Hertz>>(
    clocks: &mut GenericClockController,
    bus_speed: F,
    sercom1: SERCOM1,
    mclk: &mut MCLK,
    miso: gpio::Pb22<Input<Floating>>,
    mosi: gpio::Pb23<Input<Floating>>,
    sck: gpio::Pa17<Input<Floating>>,
    port: &mut Port,
) -> SPIMaster1 {
    let gclk0 = clocks.gclk0();
    SPIMaster1::new(
        &clocks.sercom1_core(&gclk0).unwrap(),
        bus_speed.into(),
        hal::hal::spi::Mode {
            phase: hal::hal::spi::Phase::CaptureOnFirstTransition,
            polarity: hal::hal::spi::Polarity::IdleLow,
        },
        sercom1,
        mclk,
        hal::sercom::SPI1Pinout::Dipo2Dopo2 {
            miso: miso.into_pad(port),
            mosi: mosi.into_pad(port),
            sck: sck.into_pad(port),
        },
    )
}

/// Convenience for accessing the on-board TFT LCD.
pub fn display(
    mut clocks: &mut GenericClockController,
    sercom4: SERCOM4,
    mclk: &mut MCLK,
    accel_irq: gpio::Pb14<Input<Floating>>, // TODO remove once we make miso optional
    tft_mosi: gpio::Pb15<Input<Floating>>,
    tft_sck: gpio::Pb13<Input<Floating>>,
    tft_reset: gpio::Pa0<Input<Floating>>,
    tft_cs: gpio::Pb12<Input<Floating>>,
    tft_dc: gpio::Pb5<Input<Floating>>,
    tft_backlight: gpio::Pa1<Input<Floating>>,
    timer2: TC2,
    syst: SYST,
    port: &mut Port,
) -> Result<(ST7735<SPIMaster4, gpio::Pb5<gpio::Output<gpio::PushPull>>, gpio::Pa0<gpio::Output<gpio::PushPull>>>, Pwm2), ()> { 
    let gclk0 = clocks.gclk0();
    let tft_spi = SPIMaster4::new(
        &clocks.sercom4_core(&gclk0).ok_or(())?,
        16.mhz(),
        hal::hal::spi::Mode {
            phase: hal::hal::spi::Phase::CaptureOnFirstTransition,
            polarity: hal::hal::spi::Polarity::IdleLow,
        },
        sercom4,
        mclk,
        hal::sercom::SPI4Pinout::Dipo2Dopo2 {
            miso: accel_irq.into_pad(port),
            mosi: tft_mosi.into_pad(port),
            sck: tft_sck.into_pad(port),
        },
    );

    let mut tft_cs = tft_cs.into_push_pull_output(port);
    tft_cs.set_low();

    let tft_dc = tft_dc.into_push_pull_output(port);
    let tft_reset = tft_reset.into_push_pull_output(port);
    let gclk0 = clocks.gclk0();

    let mut display = st7735_lcd::ST7735::new(tft_spi, tft_dc, tft_reset, true, false);
    let mut delay = hal::delay::Delay::new(syst, &mut clocks);
    display.init(&mut delay)?;
    display.set_orientation(&Orientation::LandscapeSwapped)?;

    let tft_backlight = tft_backlight.into_function_e(port);
    let mut pwm2 = Pwm2::new(
        &clocks.tc2_tc3(&gclk0).ok_or(())?,
        1.khz(),
        timer2,
        hal::pwm::TC2Pinout::Pa1(tft_backlight),
        mclk,
    );

    pwm2.set_duty(pwm2.get_max_duty());

    Ok((display, pwm2))
}

/// Convenience for setting up the labelled SDA, SCL pins to
/// operate as an I2C master running at the specified frequency.
pub fn i2c_master<F: Into<Hertz>>(
    clocks: &mut GenericClockController,
    bus_speed: F,
    sercom2: SERCOM2,
    mclk: &mut MCLK,
    sda: gpio::Pa12<Input<Floating>>,
    scl: gpio::Pa13<Input<Floating>>,
    port: &mut Port,
) -> I2CMaster2 {
    let gclk0 = clocks.gclk0();
    I2CMaster2::new(
        &clocks.sercom2_core(&gclk0).unwrap(),
        bus_speed.into(),
        sercom2,
        mclk,
        sda.into_pad(port),
        scl.into_pad(port),
    )
}
