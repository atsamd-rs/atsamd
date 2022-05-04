#![no_std]

#[cfg(feature = "rt")]
pub use cortex_m_rt::entry;

pub use atsamd_hal as hal;
use atsamd_hal::sercom::I2CMaster2;
pub use hal::ehal;
pub use hal::pac;

use hal::clock::GenericClockController;
use hal::sercom::v2::{spi, uart, Sercom1, Sercom2, Sercom5};
use hal::time::Hertz;

#[cfg(feature = "usb")]
use hal::usb::{usb_device::bus::UsbBusAllocator, UsbBus};

// The docs could be further improved with details of the specific channels etc
// Maps the pins to their arduino names and the numbers printed on the board.
// Information from: <https://github.com/arduino/ArduinoCore-samd/blob/master/variants/mkrwifi1010/variant.cpp>
hal::bsp_pins!(
    PB23 {
        /// RX
        name: rx
        aliases: {
            AlternateD: Rx
        }
    }
    PB22 {
        /// TX
        name: tx
        aliases: {
            AlternateD: Tx
        }
    }
    PA22 {
        /// Digital 0: PWM, TC
        name: d0
    }
    PA23 {
        /// Digital 1: PWM, TC
        name: d1
    }
    PA10 {
        /// Digital 2: ADC, PWM, TCC
        name: d2
    }
    PA11 {
        /// Digital 3: ADC, PWM, TCC
        name: d3
    }
    PB10 {
        /// Digital 4: PWM, TC
        name: d4
    }
    PB11 {
        /// Digital 5: PWM, TC
        name: d5
    }
    PA20 {
        /// Digital 6: PWM, TCC, LED_BUILTIN
        name: led
        aliases: {
            PushPullOutput: Led,
        }
    }
    PA21 {
        /// Digital 7: PWM, TC
        name: d7
    }
    PA16 {
        /// Digital 8/SPI MOSI: PWM, TCC
        name: mosi
        aliases: {
            AlternateC: Mosi
        }
    }
    PA17 {
        /// Digital 9/SPI SCK
        name: sck,
        aliases: {
            AlternateC: Sck
        }
    }
    PA19 {
        /// Digital 10/SPI MISO: PWM, TC
        name: miso
        aliases: {
            AlternateC: Miso
        }
    }
    PA08 {
        /// Digital 11/SC2 SDA: ADC
        name: sda
        aliases: {
            AlternateD: Sda
        }
    }
    PA09 {
        /// Digital 12/SC2 SCL: ADC
        name: scl
        aliases: {
            AlternateD: Scl
        }
    }
    PA02 {
        /// Analog 0: DAC0, ADC
        name: a0
    }
    PB02 {
        /// Analog 1: ADC
        name: a1
    }
    PB03 {
        /// Analog 2: ADC
        name: a2
    }
    PA04 {
        /// Analog 3: ADC, PWM, TCC
        name: a3
    }
    PA05 {
        /// Analog 4: ADC, PWM, TCC
        name: a4
    }
    PA06 {
        /// Analog 5: ADC
        name: a5
    }
    PA07 {
        /// Analog 5: ADC
        name: a6
    }
    PA24 {
        /// USB D- Pad
        name: usb_dm
        aliases: {
            AlternateG: UsbDm
        }
    }
    PA25 {
        /// USB D+ Pad
        name: usb_dp
        aliases: {
            AlternateG: UsbDp
        }
    }
    PA18 {
        /// USB ID Pad
        name: usb_id
    }
    PA03 {
        /// AREF
        name: aref
    }
    PA12 {
        /// NINA MOSI
        name: nina_mosi
        aliases: {
            AlternateC: NinaMosi
        }
    }
    PA13 {
        /// NINA MISO
        name: nina_miso
        aliases: {
            AlternateC: NinaMiso
        }
    }
    PA14 {
        /// NINA CS
        name: nina_cs
    }
    PA15 {
        /// NINA SCK
        name: nina_sck
        aliases: {
            AlternateC: NinaSck
        }
    }
    PA27 {
        /// NINA GPIO0
        name: nina_gpio0
    }
    PB08 {
        /// NINA RESETN: ADC
        name: nina_resetn
    }
    PB09 {
        /// ADC VBAT: ADC, PWM, TC
        name: adc_vbat
    }
    PA00 {
        /// 32768Hz Crystal XIN32
        name: xin32
    }
    PA01 {
        /// 32768Hz Crystal XOUT32
        name: xout32
    }
    PA28 {
        /// NINA ACK
        name: nina_ack
    }
);

#[cfg(feature = "usb")]
pub fn usb_allocator(
    usb: pac::USB,
    clocks: &mut GenericClockController,
    pm: &mut pac::PM,
    dm: impl Into<UsbDm>,
    dp: impl Into<UsbDp>,
) -> UsbBusAllocator<UsbBus> {
    let gclk0 = clocks.gclk0();
    let usb_clock = &clocks.usb(&gclk0).unwrap();
    let (dm, dp) = (dm.into(), dp.into());
    UsbBusAllocator::new(UsbBus::new(usb_clock, pm, dm, dp, usb))
}

/// Convenience for setting up the labelled SDA, SCL pins to
/// operate as an I2C master running at the specified frequency.
pub fn i2c_master(
    clocks: &mut GenericClockController,
    bus_speed: impl Into<Hertz>,
    sercom2: pac::SERCOM2,
    pm: &mut pac::PM,
    sda: impl Into<Sda>,
    scl: impl Into<Scl>,
) -> I2CMaster2<Sda, Scl> {
    let gclk0 = &clocks.gclk0();
    let clock = &clocks.sercom2_core(gclk0).unwrap();
    let (bus_speed, sda, scl) = (bus_speed.into(), sda.into(), scl.into());

    I2CMaster2::new(clock, bus_speed, sercom2, pm, sda, scl)
}

/// UART pads
pub type UartPads = uart::Pads<Sercom5, Rx, Tx>;
/// UART device for the labelled RX & TX pins
pub type Uart = uart::Uart<uart::Config<UartPads>, uart::Duplex>;

/// Convenience for setting up the labelled RX, TX pins to
/// operate as a UART device running at the specified baud.
pub fn uart(
    clocks: &mut GenericClockController,
    baud: impl Into<Hertz>,
    sercom5: pac::SERCOM5,
    pm: &mut pac::PM,
    rx: impl Into<Rx>,
    tx: impl Into<Tx>,
) -> Uart {
    let gclk0 = clocks.gclk0();
    let clock = &clocks.sercom5_core(&gclk0).unwrap();
    let baud = baud.into();
    let pads = uart::Pads::default().rx(rx.into()).tx(tx.into());

    uart::Config::new(pm, sercom5, pads, clock.freq())
        .baud(baud, uart::BaudMode::Fractional(uart::Oversampling::Bits16))
        .enable()
}

// SPI pads for the labelled SPI peripheral
///
/// You can use these pads with other, user-defined [`spi::Config`]urations.
pub type SpiPads = spi::Pads<Sercom1, Miso, Mosi, Sck>;

/// SPI master for the labelled SPI peripheral
///
/// This type implements [`FullDuplex<u8>`](ehal::spi::FullDuplex).
pub type Spi = spi::Spi<spi::Config<SpiPads>, spi::Duplex>;

/// Convenience for setting up the labelled SPI peripheral.
/// This powers up SERCOM1 and configures it for use as an
/// SPI Master in SPI Mode 0.
pub fn spi_master(
    clocks: &mut GenericClockController,
    baud: impl Into<Hertz>,
    sercom1: pac::SERCOM1,
    pm: &mut pac::PM,
    sck: impl Into<Sck>,
    mosi: impl Into<Mosi>,
    miso: impl Into<Miso>,
) -> Spi {
    let gclk0 = clocks.gclk0();
    let clock = clocks.sercom1_core(&gclk0).unwrap();
    let (miso, mosi, sclk) = (miso.into(), mosi.into(), sck.into());
    let pads = spi::Pads::default().data_in(miso).data_out(mosi).sclk(sclk);

    spi::Config::new(pm, sercom1, pads, clock.freq())
        .baud(baud)
        .spi_mode(spi::MODE_0)
        .enable()
}

pub type NinaSpiPads = spi::Pads<Sercom2, NinaMiso, NinaMosi, NinaSck>;
pub type NinaSpi = spi::Spi<spi::Config<NinaSpiPads>, spi::Duplex>;

pub fn nina_spi_master(
    clocks: &mut GenericClockController,
    baud: impl Into<Hertz>,
    sercom2: pac::SERCOM2,
    pm: &mut pac::PM,
    sck: impl Into<NinaSck>,
    mosi: impl Into<NinaMosi>,
    miso: impl Into<NinaMiso>,
) -> NinaSpi {
    let gclk0 = clocks.gclk0();
    let clock = clocks.sercom2_core(&gclk0).unwrap();
    let (miso, mosi, sclk) = (miso.into(), mosi.into(), sck.into());
    let pads = spi::Pads::default().data_in(miso).data_out(mosi).sclk(sclk);

    spi::Config::new(pm, sercom2, pads, clock.freq())
        .baud(baud)
        .spi_mode(spi::MODE_0)
        .enable()
}
