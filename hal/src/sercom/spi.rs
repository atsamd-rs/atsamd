use atsamd21g18a::sercom0::SPI;
use atsamd21g18a::{SERCOM0, SERCOM1, SERCOM2, SERCOM3, SERCOM4, SERCOM5, PM};
use clock;
use gpio::{self, IntoFunction, Port};
use hal::spi::{FullDuplex, Mode, Phase, Polarity};
use sercom::pads::*;
use time::Hertz;

/// This is a convenience trait for setting up the SPI pins.
/// You should not implement this trait yourself.
pub trait IntoPinout<T> {
    fn into_pinout(self, port: &mut Port) -> T;
}

macro_rules! spi_pinout {
    ([$($Type:ident:
        ($pad0:ident, $pad1:ident, $pad2:ident, $pad3:ident),)+
    ]) => {
$(
/// The pad ordering below corresponds to these pinouts:
/// DI, DO, SCK, SS
/// For slave: MOSI, MISO, SCK, SS
/// For master: MISO, MOSI, SCK
/// The variant labeling refers to the value of the DIPO
/// and DOPO configuration bits.  More combinations than
/// these are possible, but result in conflicting pin
/// assignments so are not represented in this interface.
pub enum $Type {
    /*
    Dipo0Dopo1SS($pad0, $pad2, $pad3, $pad1),
    Dipo0Dopo2SS($pad0, $pad3, $pad1, $pad2),
    Dipo2Dopo3SS($pad2, $pad0, $pad3, $pad1),
    Dipo3Dopo0SS($pad3, $pad0, $pad1, $pad2),
    */

    Dipo0Dopo1($pad0, $pad2, $pad3),
    /*
    Dipo0Dopo2($pad0, $pad3, $pad1),

    Dipo1Dopo1($pad1, $pad2, $pad3),
    Dipo1Dopo3($pad1, $pad0, $pad3),

    Dipo2Dopo0($pad2, $pad0, $pad1),
    Dipo2Dopo2($pad2, $pad3, $pad1),
    Dipo2Dopo3($pad2, $pad0, $pad3),

    Dipo3Dopo0($pad3, $pad0, $pad1),
    */
}

// TODO: this is sufficient for the Metro M0 express Sercom4
// which uses (pa12, pb10, pb11), but we should flesh this
// out for the other pins, or find a way to do that bit
// via a macro too.
impl<P0, P2, P3> IntoPinout<$Type> for (P0, P2, P3)
where
    P0: PadPin<$pad0>,
    P2: PadPin<$pad2>,
    P3: PadPin<$pad3>
{
    fn into_pinout(self, port: &mut Port) -> $Type {
        $Type::Dipo0Dopo1(
            self.0.into_pad(port),
            self.1.into_pad(port),
            self.2.into_pad(port))
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

pub struct $Type {
    pinout: $PinOut,
    sercom: $SERCOM,
}

impl $Type {
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
                    Phase::CaptureOnFirstTransition=> w.cpha().clear_bit(),
                    Phase::CaptureOnSecondTransition=>w.cpha().set_bit(),
                };

                match &pinout {
                    $PinOut::Dipo0Dopo1(_,_,_) => {
                        w.dipo().bits(0);
                        w.dopo().bits(1);
                    }
                }

                // MSB first
                w.dord().clear_bit()
            });


            sercom.spi.ctrla.modify(|_, w| w.enable().set_bit());
            // wait for configuration to take effect
            while sercom.spi.syncbusy.read().enable().bit_is_set() {}

        }

        Self { pinout, sercom }
    }

    pub fn free(self) -> ($PinOut, $SERCOM) {
        (self.pinout, self.sercom)
    }

    fn spi(&mut self) -> &SPI {
        unsafe { &self.sercom.spi }
    }
}


)+
    };
}

spi!([
    SPIMaster0: (SPI0Pinout, SERCOM0, sercom0_, Sercom0CoreClock),
    SPIMaster1: (SPI1Pinout, SERCOM1, sercom1_, Sercom1CoreClock),
    SPIMaster2: (SPI2Pinout, SERCOM2, sercom2_, Sercom2CoreClock),
    SPIMaster3: (SPI3Pinout, SERCOM3, sercom3_, Sercom3CoreClock),
    SPIMaster4: (SPI4Pinout, SERCOM4, sercom4_, Sercom4CoreClock),
    SPIMaster5: (SPI5Pinout, SERCOM5, sercom5_, Sercom5CoreClock),
]);
