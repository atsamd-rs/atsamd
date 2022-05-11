#![no_std]
#![recursion_limit = "1024"]

pub use atsamd_hal as hal;
pub use hal::pac;

#[cfg(feature = "rt")]
pub use cortex_m_rt::entry;

use hal::clock::GenericClockController;
use hal::sercom::uart;
use hal::sercom::uart::BaudMode;
use hal::sercom::uart::Oversampling;
use hal::sercom::{i2c, spi, IoSet1, IoSet6};
use hal::time::Hertz;

pub mod pins;
pub use pins::*;

#[cfg(feature = "display")]
mod display;
#[cfg(feature = "display")]
pub use display::*;

#[cfg(feature = "usb")]
use hal::usb::{usb_device::bus::UsbBusAllocator, UsbBus};

hal::bsp_peripherals!(
    SERCOM2 { SpiSercom }
    SERCOM4 { EspUartSercom }
    SERCOM5 { I2cSercom }
);

pub type SpiPads = spi::Pads<SpiSercom, IoSet1, Miso, Mosi, Sck>;

pub type Spi = spi::Spi<spi::Config<SpiPads>, spi::Duplex>;

/// This powers up SERCOM2 and configures it for use as an
/// SPI Master in SPI Mode 0.
/// Unlike the `flash_spi_master` function, this
/// one does not accept a CS pin; configuring a pin for CS
/// is the responsibility of the caller, because it could be
/// any OutputPin, or even a pulled up line on the slave.
pub fn spi_master(
    clocks: &mut GenericClockController,
    baud: impl Into<Hertz>,
    sercom: SpiSercom,
    mclk: &mut pac::MCLK,
    sclk: impl Into<Sck>,
    mosi: impl Into<Mosi>,
    miso: impl Into<Miso>,
) -> Spi {
    let gclk0 = clocks.gclk0();
    let clock = clocks.sercom2_core(&gclk0).unwrap();
    let freq = clock.freq();
    let (miso, mosi, sclk) = (miso.into(), mosi.into(), sclk.into());
    let pads = spi::Pads::default().data_in(miso).data_out(mosi).sclk(sclk);
    spi::Config::new(mclk, sercom, pads, freq)
        .baud(baud)
        .spi_mode(spi::MODE_0)
        .enable()
}

/// I2C pads for the labelled I2C peripheral
///
/// You can use these pads with other, user-defined [`i2c::Config`]urations.
pub type I2cPads = i2c::Pads<I2cSercom, IoSet6, Sda, Scl>;

/// I2C master for the labelled I2C peripheral
///
/// This type implements [`Read`](hal::ehal::blocking::i2c::Read),
/// [`Write`](hal::ehal::blocking::i2c::Write) and
/// [`WriteRead`](hal::ehal::blocking::i2c::WriteRead).
pub type I2c = i2c::I2c<i2c::Config<I2cPads>>;

/// Convenience for setting up the labelled SDA, SCL pins to
/// operate as an I2C master running at the specified frequency.
pub fn i2c_master(
    clocks: &mut GenericClockController,
    baud: impl Into<Hertz>,
    sercom: I2cSercom,
    mclk: &mut pac::MCLK,
    sda: impl Into<Sda>,
    scl: impl Into<Scl>,
) -> I2c {
    let gclk0 = clocks.gclk0();
    let clock = &clocks.sercom5_core(&gclk0).unwrap();
    let freq = clock.freq();
    let baud = baud.into();
    let pads = i2c::Pads::new(sda.into(), scl.into());
    i2c::Config::new(mclk, sercom, pads, freq)
        .baud(baud)
        .enable()
}

/// UART Pads for the ESP32 Wi-Fi co-processor
pub type EspUartPads = uart::Pads<EspUartSercom, IoSet1, EspUartRx, EspUartTx>;

/// UART device for the ESP32 Wi-Fi co-processor
pub type EspUart = uart::Uart<uart::Config<EspUartPads>, uart::Duplex>;

/// UART is connected to the ESP32 Wi-Fi co-processor
pub fn esp_uart(
    clocks: &mut GenericClockController,
    baud: impl Into<Hertz>,
    sercom: EspUartSercom,
    mclk: &mut pac::MCLK,
    esp_rx: impl Into<EspUartRx>,
    esp_tx: impl Into<EspUartTx>,
) -> EspUart {
    let gclk0 = clocks.gclk0();
    let clock = &clocks.sercom4_core(&gclk0).unwrap();
    let baud = baud.into();
    let pads = uart::Pads::default().rx(esp_rx.into()).tx(esp_tx.into());
    uart::Config::new(mclk, sercom, pads, clock.freq())
        .baud(baud, BaudMode::Fractional(Oversampling::Bits16))
        .enable()
}

#[cfg(feature = "usb")]
/// Convenience function for setting up USB
pub fn usb_allocator(
    dm: impl Into<UsbDm>,
    dp: impl Into<UsbDp>,
    usb: pac::USB,
    clocks: &mut GenericClockController,
    mclk: &mut pac::MCLK,
) -> UsbBusAllocator<UsbBus> {
    use pac::gclk::{genctrl::SRC_A, pchctrl::GEN_A};

    clocks.configure_gclk_divider_and_source(GEN_A::GCLK2, 1, SRC_A::DFLL, false);
    let usb_gclk = clocks.get_gclk(GEN_A::GCLK2).unwrap();
    let usb_clock = &clocks.usb(&usb_gclk).unwrap();
    let (dm, dp) = (dm.into(), dp.into());
    UsbBusAllocator::new(UsbBus::new(usb_clock, mclk, dm, dp, usb))
}
