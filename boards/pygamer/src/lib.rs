#![no_std]
#![recursion_limit = "1024"]

pub mod pins;
use atsamd_hal as hal;

#[cfg(feature = "rt")]
pub use cortex_m_rt::entry;

pub use pins::Pins;

use hal::*;

pub use hal::common::*;
pub use hal::samd51::*;
pub use hal::target_device as pac;

use gpio::{Floating, Input, Output, PfC, Port, PushPull};
use hal::clock::GenericClockController;
use hal::prelude::*;
use hal::pwm::Pwm2;
use hal::sercom::{
    I2CMaster2, PadPin, SPIMaster1, SPIMaster4, Sercom4Pad1, Sercom4Pad2, Sercom4Pad3, UART5,
};
use hal::time::Hertz;

use st7735_lcd::{Orientation, ST7735};

/// This powers up SERCOM1 and configures it for use as an
/// SPI Master in SPI Mode 0.
/// This function does not accept a CS pin; configuring a pin for CS
/// is the responsibility of the caller, because it could be
/// any OutputPin, or even a pulled up line on the slave.
pub fn spi_master<F: Into<Hertz>>(
    clocks: &mut GenericClockController,
    bus_speed: F,
    sercom1: pac::SERCOM1,
    mclk: &mut pac::MCLK,
    miso: gpio::Pb22<Input<Floating>>,
    mosi: gpio::Pb23<Input<Floating>>,
    sck: gpio::Pa17<Input<Floating>>,
    port: &mut Port,
) -> SPIMaster1<
    hal::sercom::Sercom1Pad2<gpio::Pb22<gpio::PfC>>,
    hal::sercom::Sercom1Pad3<gpio::Pb23<gpio::PfC>>,
    hal::sercom::Sercom1Pad1<gpio::Pa17<gpio::PfC>>,
> {
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
        (miso.into_pad(port), mosi.into_pad(port), sck.into_pad(port)),
    )
}

/// Convenience for accessing the on-board TFT LCD.
pub fn display(
    clocks: &mut GenericClockController,
    sercom4: pac::SERCOM4,
    mclk: &mut pac::MCLK,
    accel_irq: gpio::Pb14<Input<Floating>>, // TODO remove once we make miso optional
    tft_mosi: gpio::Pb15<Input<Floating>>,
    tft_sck: gpio::Pb13<Input<Floating>>,
    tft_reset: gpio::Pa0<Input<Floating>>,
    tft_cs: gpio::Pb12<Input<Floating>>,
    tft_dc: gpio::Pb5<Input<Floating>>,
    tft_backlight: gpio::Pa1<Input<Floating>>,
    timer2: pac::TC2,
    delay: &mut hal::delay::Delay,
    port: &mut Port,
) -> Result<
    (
        ST7735<
            SPIMaster4<
                Sercom4Pad2<gpio::Pb14<gpio::PfC>>,
                Sercom4Pad3<gpio::Pb15<gpio::PfC>>,
                Sercom4Pad1<gpio::Pb13<gpio::PfC>>,
            >,
            gpio::Pb5<Output<PushPull>>,
            gpio::Pa0<Output<PushPull>>,
        >,
        atsamd_hal::samd51::pwm::Pwm2,
    ),
    (),
> {
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
        (
            accel_irq.into_pad(port),
            tft_mosi.into_pad(port),
            tft_sck.into_pad(port),
        ),
    );

    let mut tft_cs = tft_cs.into_push_pull_output(port);
    tft_cs.set_low()?;

    let tft_dc = tft_dc.into_push_pull_output(port);
    let tft_reset = tft_reset.into_push_pull_output(port);
    let gclk0 = clocks.gclk0();

    let mut display = st7735_lcd::ST7735::new(tft_spi, tft_dc, tft_reset, true, false);
    display.init(delay)?;
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
    sercom2: pac::SERCOM2,
    mclk: &mut pac::MCLK,
    sda: gpio::Pa12<Input<Floating>>,
    scl: gpio::Pa13<Input<Floating>>,
    port: &mut Port,
) -> I2CMaster2<
    hal::sercom::Sercom2Pad0<gpio::Pa12<gpio::PfC>>,
    hal::sercom::Sercom2Pad1<gpio::Pa13<gpio::PfC>>,
> {
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

/// Convenience for setting up the labelled TX, RX pins to
/// operate as a UART running at the specified frequency.
pub fn uart<F: Into<Hertz>>(
    pins: pins::UART,
    clocks: &mut GenericClockController,
    baud: F,
    sercom5: pac::SERCOM5,
    mclk: &mut pac::MCLK,
    port: &mut Port,
) -> UART5<
    hal::sercom::Sercom5Pad1<gpio::Pb17<PfC>>,
    hal::sercom::Sercom5Pad0<gpio::Pb16<PfC>>,
    (),
    (),
> {
    pins.uart(clocks, baud, sercom5, mclk, port)
}
