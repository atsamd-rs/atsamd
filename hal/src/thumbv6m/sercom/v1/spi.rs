#![deprecated(
    since = "0.13.0",
    note = "The `sercom::v1::spi` module is deprecated, and will be removed in a subsequent release.
    Please use the `sercom::v2::spi` module instead."
)]

use core::marker::PhantomData;

use crate::clock;
use crate::hal::spi::{FullDuplex, Mode, Phase, Polarity};
use crate::pac::sercom0::SPI;
use crate::pac::{PM, SERCOM0, SERCOM1};
#[cfg(feature = "samd21")]
use crate::pac::{SERCOM2, SERCOM3};
#[cfg(feature = "min-samd21g")]
use crate::pac::{SERCOM4, SERCOM5};
use crate::sercom::v1::pads::CompatiblePad;
use crate::sercom::v2::*;
use crate::spi_common::CommonSpi;
use crate::time::Hertz;

#[derive(Debug)]
pub enum Error {
    Overrun,
}

/// The DipoDopo trait defines a way to get the data in and data out pin out
/// values for a given SPIMasterXPadout configuration. You should not implement
/// this trait for yourself; only the implementations in the sercom module make
/// sense.
pub trait DipoDopo {
    const DIPO: u8;
    const DOPO: u8;
    fn dipo_dopo(&self) -> (u8, u8) {
        (Self::DIPO, Self::DOPO)
    }
}

macro_rules! padout {
    ( ($dipo:literal, $dopo:literal) => $pad0:ident, $pad1:ident, $pad2:ident) => {
        impl DipoDopo for ($pad0, $pad1, $pad2) {
            const DIPO: u8 = $dipo;
            const DOPO: u8 = $dopo;
        }
    };
}

padout!((0, 1) => Pad0, Pad2, Pad3);
padout!((0, 2) => Pad0, Pad3, Pad1);

padout!((1, 1) => Pad1, Pad2, Pad3);
padout!((1, 3) => Pad1, Pad0, Pad3);

padout!((2, 0) => Pad2, Pad0, Pad1);
padout!((2, 2) => Pad2, Pad3, Pad1);
padout!((2, 3) => Pad2, Pad0, Pad3);

padout!((3, 0) => Pad3, Pad0, Pad1);

/// A pad mapping configuration for the SERCOM in SPI master mode.
///
/// This type can only be constructed using the From implementations
/// in this module, which are restricted to valid configurations.
///
/// Defines which sercom pad is mapped to which SPI function.
pub struct Padout<S, MISO, MOSI, SCLK>
where
    S: Sercom,
{
    sercom: PhantomData<S>,
    _miso: MISO,
    _mosi: MOSI,
    _sclk: SCLK,
}

/// Convert from a tuple of (MISO, MOSI, SCK) to SPIMasterXPadout
impl<S, PAD0, PAD1, PAD2> From<(PAD0, PAD1, PAD2)> for Padout<S, PAD0, PAD1, PAD2>
where
    S: Sercom,
    PAD0: CompatiblePad<Sercom = S>,
    PAD1: CompatiblePad<Sercom = S>,
    PAD2: CompatiblePad<Sercom = S>,
    (PAD0::PadNum, PAD1::PadNum, PAD2::PadNum): DipoDopo,
{
    fn from(pads: (PAD0, PAD1, PAD2)) -> Padout<S, PAD0, PAD1, PAD2> {
        Padout {
            sercom: PhantomData,
            _miso: pads.0,
            _mosi: pads.1,
            _sclk: pads.2,
        }
    }
}

impl<S, PAD0, PAD1, PAD2> DipoDopo for Padout<S, PAD0, PAD1, PAD2>
where
    S: Sercom,
    PAD0: CompatiblePad<Sercom = S>,
    PAD1: CompatiblePad<Sercom = S>,
    PAD2: CompatiblePad<Sercom = S>,
    (PAD0::PadNum, PAD1::PadNum, PAD2::PadNum): DipoDopo,
{
    const DIPO: u8 = <(PAD0::PadNum, PAD1::PadNum, PAD2::PadNum)>::DIPO;
    const DOPO: u8 = <(PAD0::PadNum, PAD1::PadNum, PAD2::PadNum)>::DOPO;
}

/// Define an SPIMasterX type for the given Sercom number.
///
/// Also defines the valid "pad to spi function" mappings for this instance so
/// that construction is restricted to correct configurations.
macro_rules! spi_master {
    ($Type:ident: ($Sercom:ident, $SERCOM:ident, $powermask:ident, $clock:ident)) => {
        $crate::paste::item! {
            pub type [<$Type Padout>]<MISO, MOSI, SCLK> = Padout<$Sercom, MISO, MOSI, SCLK>;
        }

        /// SPIMasterX represents the corresponding SERCOMX instance
        /// configured to act in the role of an SPI Master.
        /// Objects of this type implement the HAL `FullDuplex` and blocking
        /// SPI traits.
        ///
        /// This type is generic over any valid pad mapping where there is
        /// a defined "data in pin out data out pin out" implementation.
        pub struct $Type<MISO, MOSI, SCK> {
            padout: Padout<$Sercom, MISO, MOSI, SCK>,
            sercom: $SERCOM,
        }

        impl<MISO, MOSI, SCK> CommonSpi for $Type<MISO, MOSI, SCK> {
            /// Helper for accessing the spi member of the sercom instance
            fn spi(&self) -> &SPI {
                &self.sercom.spi()
            }

            /// Helper for accessing the spi member of the sercom instance
            fn spi_mut(&mut self) -> &SPI {
                &self.sercom.spi()
            }
        }

        impl<MISO, MOSI, SCK> $Type<MISO, MOSI, SCK> {
            /// Power on and configure SERCOMX to work as an SPI Master operating
            /// with the specified frequency and SPI Mode. The padout specifies
            /// which pins are bound to the MISO, MOSI, SCK functions.
            ///
            /// You can use a tuple of three SercomXPadY instances for which
            /// there exists a From implementation for SPIMasterXPadout.
            pub fn new<F: Into<Hertz>, T: Into<Padout<$Sercom, MISO, MOSI, SCK>>>(
                clock: &clock::$clock,
                freq: F,
                mode: Mode,
                sercom: $SERCOM,
                pm: &mut PM,
                padout: T,
            ) -> Self
            where
                Padout<$Sercom, MISO, MOSI, SCK>: DipoDopo,
            {
                let padout = padout.into();

                // Power up the peripheral bus clock.
                // safe because we're exclusively owning SERCOM
                pm.apbcmask.modify(|_, w| w.$powermask().set_bit());

                // reset the sercom instance
                sercom.spi().ctrla.modify(|_, w| w.swrst().set_bit());
                // wait for reset to complete
                while sercom.spi().syncbusy.read().swrst().bit_is_set()
                    || sercom.spi().ctrla.read().swrst().bit_is_set()
                {}

                // Put the hardware into spi master mode
                sercom.spi().ctrla.modify(|_, w| w.mode().spi_master());
                // wait for configuration to take effect
                while sercom.spi().syncbusy.read().enable().bit_is_set() {}

                // 8 bit data size and enable the receiver
                unsafe {
                    sercom.spi().ctrlb.modify(|_, w| {
                        w.chsize().bits(0);
                        w.rxen().set_bit()
                    });
                }

                // set the baud rate
                let baud = Self::calculate_baud(freq, clock.freq());

                unsafe {
                    sercom.spi().baud.modify(|_, w| w.baud().bits(baud));

                    sercom.spi().ctrla.modify(|_, w| {
                        match mode.polarity {
                            Polarity::IdleLow => w.cpol().clear_bit(),
                            Polarity::IdleHigh => w.cpol().set_bit(),
                        };

                        match mode.phase {
                            Phase::CaptureOnFirstTransition => w.cpha().clear_bit(),
                            Phase::CaptureOnSecondTransition => w.cpha().set_bit(),
                        };

                        let (dipo, dopo) = padout.dipo_dopo();
                        w.dipo().bits(dipo);
                        w.dopo().bits(dopo);

                        // MSB first
                        w.dord().clear_bit()
                    });
                }

                sercom.spi().ctrla.modify(|_, w| w.enable().set_bit());
                // wait for configuration to take effect
                while sercom.spi().syncbusy.read().enable().bit_is_set() {}

                Self { padout, sercom }
            }

            /// Set the baud rate
            pub fn set_baud<F: Into<Hertz>>(&mut self, freq: F, clock: &clock::$clock) {
                self.disable();
                let baud = Self::calculate_baud(freq, clock.freq());
                unsafe {
                    self.spi_mut().baud.modify(|_, w| w.baud().bits(baud));
                }
                self.enable();
            }

            /// Tear down the SPI instance and yield the constituent pins and
            /// SERCOM instance.  No explicit de-initialization is performed.
            pub fn free(self) -> (Padout<$Sercom, MISO, MOSI, SCK>, $SERCOM) {
                (self.padout, self.sercom)
            }
        }

        impl<MISO, MOSI, SCK> FullDuplex<u8> for $Type<MISO, MOSI, SCK> {
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
                    self.spi_mut()
                        .data
                        .write(|w| unsafe { w.data().bits(byte as u16) });
                    Ok(())
                } else {
                    Err(nb::Error::WouldBlock)
                }
            }
        }

        impl<MISO, MOSI, SCK> ::hal::blocking::spi::transfer::Default<u8>
            for $Type<MISO, MOSI, SCK>
        {
        }
        impl<MISO, MOSI, SCK> ::hal::blocking::spi::write::Default<u8> for $Type<MISO, MOSI, SCK> {}
        #[cfg(feature = "unproven")]
        impl<MISO, MOSI, SCK> ::hal::blocking::spi::write_iter::Default<u8>
            for $Type<MISO, MOSI, SCK>
        {
        }
    };
}

spi_master!(SPIMaster0: (Sercom0, SERCOM0, sercom0_, Sercom0CoreClock));
spi_master!(SPIMaster1: (Sercom1, SERCOM1, sercom1_, Sercom1CoreClock));
#[cfg(feature = "samd21")]
spi_master!(SPIMaster2: (Sercom2, SERCOM2, sercom2_, Sercom2CoreClock));
#[cfg(feature = "samd21")]
spi_master!(SPIMaster3: (Sercom3, SERCOM3, sercom3_, Sercom3CoreClock));
#[cfg(feature = "min-samd21g")]
spi_master!(SPIMaster4: (Sercom4, SERCOM4, sercom4_, Sercom4CoreClock));
#[cfg(feature = "min-samd21g")]
spi_master!(SPIMaster5: (Sercom5, SERCOM5, sercom5_, Sercom5CoreClock));
