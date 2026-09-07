#![no_std]
#![recursion_limit = "1024"]

pub use atsamd_hal as hal;
pub use hal::pac;

#[cfg(feature = "rt")]
pub use cortex_m_rt::entry;

use hal::clock::GenericClockController;
use hal::sercom::{
    i2c, spi,
    uart::{self, BaudMode, Oversampling},
};
use hal::time::Hertz;

pub mod pins;
pub use pins::*;

#[cfg(feature = "display")]
mod display;
#[cfg(feature = "display")]
pub use display::*;

#[cfg(feature = "usb")]
use hal::usb::{usb_device::bus::UsbBusAllocator, UsbBus};

#[cfg(feature = "wifi")]
use hal::{
    eic,
    sercom::{spi::Duplex, Sercom2},
};

hal::bsp_peripherals!(
    Sercom2 { SpiSercom }
    Sercom4 { EspUartSercom }
    Sercom5 { I2cSercom }
);

pub type SpiPads = spi::Pads<SpiSercom, Miso, Mosi, Sck>;

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
    mclk: &mut pac::Mclk,
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
        .baud(baud.into())
        .spi_mode(spi::MODE_0)
        .enable()
}

#[cfg(feature = "wifi")]
/// Initialize embassy-nina to talk to the ESP32 coprocessor
/// see examples/embassy_wifi.rs
pub async fn wifi<SI, BI>(
    spi: Spi,
    spi_interrupt: SI,
    ch: hal::eic::Channel<hal::eic::Ch0>,
    cs: impl Into<EspCs>,
    busy: impl Into<EspBusy>,
    busy_interrupt: BI,
    reset: impl Into<EspReset>,
    gpio: impl Into<EspGpio>,
) -> Result<
    embassy_nina::Nina<
        spi::SpiFuture<spi::Config<SpiPads>, Duplex>,
        EspCs,
        eic::ExtInt<EspBusy, hal::eic::Ch0, eic::EicFuture>,
        EspReset,
        EspGpio,
    >,
    embassy_nina::Error<spi::Error>,
>
where
    SI: hal::async_hal::interrupts::Binding<
        hal::async_hal::interrupts::SERCOM2,
        spi::InterruptHandler<Sercom2>,
    >,
    BI: hal::async_hal::interrupts::Binding<
        hal::async_hal::interrupts::EIC_EXTINT_0,
        eic::InterruptHandler,
    >,
{
    let spi = spi.into_future(spi_interrupt);
    let busy2 = ch.with_pin(busy.into()).into_future(busy_interrupt);
    let mut nina = embassy_nina::Nina::new(spi, cs.into(), busy2, reset.into(), gpio.into());
    nina.init().await?;
    Ok(nina)
}

/// I2C pads for the labelled I2C peripheral
///
/// You can use these pads with other, user-defined Durations.
pub type I2cPads = i2c::Pads<I2cSercom, Sda, Scl>;

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
    mclk: &mut pac::Mclk,
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
pub type EspUartPads = uart::Pads<EspUartSercom, EspUartRx, EspUartTx>;

/// UART device for the ESP32 Wi-Fi co-processor
pub type EspUart = uart::Uart<uart::Config<EspUartPads>, uart::Duplex>;

/// UART is connected to the ESP32 Wi-Fi co-processor
pub fn esp_uart(
    clocks: &mut GenericClockController,
    baud: impl Into<Hertz>,
    sercom: EspUartSercom,
    mclk: &mut pac::Mclk,
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
    usb: pac::Usb,
    clocks: &mut GenericClockController,
    mclk: &mut pac::Mclk,
) -> UsbBusAllocator<UsbBus> {
    use pac::gclk::{genctrl::Srcselect, pchctrl::Genselect};

    clocks.configure_gclk_divider_and_source(Genselect::Gclk2, 1, Srcselect::Dfll, false);
    let usb_gclk = clocks.get_gclk(Genselect::Gclk2).unwrap();
    let usb_clock = &clocks.usb(&usb_gclk).unwrap();
    let (dm, dp) = (dm.into(), dp.into());
    UsbBusAllocator::new(UsbBus::new(usb_clock, mclk, dm, dp, usb))
}
