use clock;
use hal::spi::{FullDuplex, Mode, Phase, Polarity};
use nb;
use sercom::pads::*;
use target_device::sercom0::SPI;
use target_device::{PM, SERCOM0, SERCOM1, SERCOM2, SERCOM3};
#[cfg(feature = "samd21g18a")]
use target_device::{SERCOM4, SERCOM5};
use time::Hertz;

pub enum Error {
    Overrun,
}

macro_rules! spi_pinout {
    ([$($Type:ident:
        ($pad0:ident, $pad1:ident, $pad2:ident, $pad3:ident),)+
    ]) => {
$(
/// When configured for SPI, in addition to the normal
/// Sercom pad mapping, the SPI peripheral allows those
/// pads to be assigned to different combinations of
/// DI (data-in), DO (data-out), SCK (clock) and SS (slave select)
/// functions.
/// The SPIXPinOut types represent concrete pad mappings for a
/// given SPI Instance.
/// For slaves, DI is the MOSI function and DO is the MISO function.
/// For masters, DI is the MISO function and DO is the MOSI function.
/// The slave configurations all require an SS pin and are constructed
/// using the enum variants ending with SS.
/// The master confiugrations do not require an SS pin and are constructed
/// using the other variants.
/// The SPI master hardware has support for automatically managing the
/// SS line to enable a slave, but this interface does not expose that
/// functionality.
/// The variant names refer to the Data-in-Data-out configuration that
/// is used to configure the SPI peripheral.
pub enum $Type {
    /// Construct a slave pinout with mosi assigned to pad0,
    /// miso pad2, sck pad3 and ss to pad1.
    Dipo0Dopo1SS{mosi:$pad0, miso:$pad2, sck:$pad3, ss:$pad1},
    Dipo0Dopo2SS{mosi:$pad0, miso:$pad3, sck:$pad1, ss:$pad2},
    Dipo2Dopo3SS{mosi:$pad2, miso:$pad0, sck:$pad3, ss:$pad1},
    Dipo3Dopo0SS{mosi:$pad3, miso:$pad0, sck:$pad1, ss:$pad2},

    /// Construct a master pinout with miso assigned to pad0,
    /// mosi pad2 and sck to pad3
    Dipo0Dopo1{miso:$pad0, mosi:$pad2, sck:$pad3},
    Dipo1Dopo1{miso:$pad1, mosi:$pad2, sck:$pad3},
    Dipo0Dopo2{miso:$pad0, mosi:$pad3, sck:$pad1},

    Dipo1Dopo3{miso:$pad1, mosi:$pad0, sck:$pad3},

    Dipo2Dopo0{miso:$pad2, mosi:$pad0, sck:$pad1},
    Dipo2Dopo2{miso:$pad2, mosi:$pad3, sck:$pad1},
    Dipo2Dopo3{miso:$pad2, mosi:$pad0, sck:$pad3},

    Dipo3Dopo0{miso:$pad3, mosi:$pad0, sck:$pad1},
}

impl $Type {
    /// Return the data-in, data-out values for
    /// this pinout configuration
    fn dipo_dopo(&self) -> (u8, u8) {
        match self {
            &$Type::Dipo0Dopo1SS{..} => (0, 1),
            &$Type::Dipo0Dopo2SS{..} => (0, 2),
            &$Type::Dipo2Dopo3SS{..} => (2, 3),
            &$Type::Dipo3Dopo0SS{..} => (3, 0),

            &$Type::Dipo0Dopo1{..} => (0, 1),
            &$Type::Dipo1Dopo1{..} => (1, 1),
            &$Type::Dipo0Dopo2{..} => (0, 2),

            &$Type::Dipo1Dopo3{..} => (1, 3),

            &$Type::Dipo2Dopo0{..} => (2, 0),
            &$Type::Dipo2Dopo2{..} => (2, 2),
            &$Type::Dipo2Dopo3{..} => (2, 3),

            &$Type::Dipo3Dopo0{..} => (3, 0),
        }
    }
}

)+

}
}

spi_pinout!([
    SPI0Pinout: (Sercom0Pad0, Sercom0Pad1, Sercom0Pad2, Sercom0Pad3),
    SPI1Pinout: (Sercom1Pad0, Sercom1Pad1, Sercom1Pad2, Sercom1Pad3),
    SPI2Pinout: (Sercom2Pad0, Sercom2Pad1, Sercom2Pad2, Sercom2Pad3),
    SPI3Pinout: (Sercom3Pad0, Sercom3Pad1, Sercom3Pad2, Sercom3Pad3),
]);
#[cfg(feature = "samd21g18a")]
spi_pinout!([
    SPI4Pinout: (Sercom4Pad0, Sercom4Pad1, Sercom4Pad2, Sercom4Pad3),
    SPI5Pinout: (Sercom5Pad0, Sercom5Pad1, Sercom5Pad2, Sercom5Pad3),
]);

macro_rules! spi {
    ([
        $($Type:ident: (
                        $PinOut:ident,
                        $SERCOM:ident, $powermask:ident, $clock:ident),)+
    ]) => {
$(

/// SPIMasterX represents the corresponding SERCOMX instance configured to
/// act in the role of an SPI Master.
/// Objects of this type implement the HAL `FullDuplex` and blocking SPI
/// traits.
pub struct $Type {
    pinout: $PinOut,
    sercom: $SERCOM,
}

impl $Type {
    /// Power on and configure SERCOMX to work as an SPI Master operating
    /// with the specified frequency and SPI Mode.  The pinout specifies
    /// which pins are bound to the MISO, MOSI, SCK functions.
    pub fn new<F: Into<Hertz>>(
        clock:&clock::$clock,
        freq: F,
        mode: Mode,
        sercom: $SERCOM,
        pm: &mut PM,
        pinout: $PinOut,
    ) -> Self {
        // Power up the peripheral bus clock.
        // safe because we're exclusively owning SERCOM
        pm.apbcmask.modify(|_, w| w.$powermask().set_bit());

        unsafe {
            // reset the sercom instance
            sercom.spi.ctrla.modify(|_, w| w.swrst().set_bit());
            // wait for reset to complete
            while sercom.spi.syncbusy.read().swrst().bit_is_set()
                || sercom.spi.ctrla.read().swrst().bit_is_set()
            {}

            // Put the hardware into spi master mode
            sercom.spi.ctrla.modify(|_, w| w.mode().spi_master());
            // wait for configuration to take effect
            while sercom.spi.syncbusy.read().enable().bit_is_set() {}

            // 8 bit data size and enable the receiver
            sercom.spi.ctrlb.modify(|_, w|{
                w.chsize().bits(0);
                w.rxen().set_bit()
            });

            // set the baud rate
            let gclk = clock.freq();
            let baud = (gclk.0 / (2 * freq.into().0) - 1) as u8;
            sercom.spi.baud.modify(|_, w| w.baud().bits(baud));

            sercom.spi.ctrla.modify(|_, w| {
                match mode.polarity {
                    Polarity::IdleLow => w.cpol().clear_bit(),
                    Polarity::IdleHigh => w.cpol().set_bit(),
                };

                match mode.phase {
                    Phase::CaptureOnFirstTransition => w.cpha().clear_bit(),
                    Phase::CaptureOnSecondTransition => w.cpha().set_bit(),
                };

                let (dipo, dopo) = pinout.dipo_dopo();
                w.dipo().bits(dipo);
                w.dopo().bits(dopo);

                // MSB first
                w.dord().clear_bit()
            });


            sercom.spi.ctrla.modify(|_, w| w.enable().set_bit());
            // wait for configuration to take effect
            while sercom.spi.syncbusy.read().enable().bit_is_set() {}

        }

        Self {
            pinout,
            sercom,
        }
    }

    /// Tear down the SPI instance and yield the constituent pins and
    /// SERCOM instance.  No explicit de-initialization is performed.
    pub fn free(self) -> ($PinOut, $SERCOM) {
        (self.pinout, self.sercom)
    }

    /// Helper for accessing the spi member of the sercom instance
    fn spi(&mut self) -> &SPI {
        unsafe { &self.sercom.spi }
    }
}

impl FullDuplex<u8> for $Type {
    type Error = Error;

    fn read(&mut self) -> nb::Result<u8, Error> {
        let status = self.spi().status.read();
        if status.bufovf().bit_is_set() {
            return Err(nb::Error::Other(Error::Overrun));
        }

        let intflag = self.spi().intflag.read();
        // rxc is receive complete
        if intflag.rxc().bit_is_set() {
            Ok(self.spi().data.read().data().bits() as u8)
        } else {
            Err(nb::Error::WouldBlock)
        }
    }

    fn send(&mut self, byte: u8) -> nb::Result<(), Error> {
        let intflag = self.spi().intflag.read();
        // dre is data register empty
        if intflag.dre().bit_is_set() {
            self.spi().data.write(|w| unsafe{w.data().bits(byte as u16)});
            Ok(())
        } else {
            Err(nb::Error::WouldBlock)
        }
    }
}

impl ::hal::blocking::spi::transfer::Default<u8> for $Type {}
impl ::hal::blocking::spi::write::Default<u8> for $Type {}


)+
    };
}

spi!([
    SPIMaster0: (SPI0Pinout, SERCOM0, sercom0_, Sercom0CoreClock),
    SPIMaster1: (SPI1Pinout, SERCOM1, sercom1_, Sercom1CoreClock),
    SPIMaster2: (SPI2Pinout, SERCOM2, sercom2_, Sercom2CoreClock),
    SPIMaster3: (SPI3Pinout, SERCOM3, sercom3_, Sercom3CoreClock),
]);
#[cfg(feature = "samd21g18a")]
spi!([
    SPIMaster4: (SPI4Pinout, SERCOM4, sercom4_, Sercom4CoreClock),
    SPIMaster5: (SPI5Pinout, SERCOM5, sercom5_, Sercom5CoreClock),
]);
