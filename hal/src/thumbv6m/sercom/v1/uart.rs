#![deprecated(
    since = "0.13.0",
    note = "The `sercom::v1::uart` module is deprecated, and will be removed in a subsequent release.
    Please use the `sercom::v2::uart` module instead."
)]

use crate::clock;
use crate::hal::blocking::serial::{write::Default, Write};
use crate::hal::serial;
use crate::pac::sercom0::USART;
use crate::pac::{PM, SERCOM0, SERCOM1};
#[cfg(feature = "samd21")]
use crate::pac::{SERCOM2, SERCOM3};
#[cfg(feature = "min-samd21g")]
use crate::pac::{SERCOM4, SERCOM5};
use crate::sercom::v1::pads::CompatiblePad;
use crate::sercom::v2::*;
use crate::time::Hertz;
use core::fmt;
use core::marker::PhantomData;

/// The RxpoTxpo trait defines a way to get the data in and data out pin out
/// values for a given UARTXPadout configuration. You should not implement
/// this trait for yourself; only the implementations in the sercom module make
/// sense.
pub trait RxpoTxpo {
    const RXPO: u8;
    const TXPO: u8;
    fn rxpo_txpo(&self) -> (u8, u8) {
        (Self::RXPO, Self::TXPO)
    }
}

macro_rules! padout {
    ( ($rxpo:literal, $txpo:literal) => $pad0:ident, $pad1:ident) => {
        impl RxpoTxpo for ($pad0, $pad1) {
            const RXPO: u8 = $rxpo;
            const TXPO: u8 = $txpo;
        }
    };
    ( ($rxpo:literal, $txpo:literal) => $pad0:ident, $pad1:ident, $pad2:ident, $pad3:ident) => {
        impl RxpoTxpo for ($pad0, $pad1, $pad2, $pad3) {
            const RXPO: u8 = $rxpo;
            const TXPO: u8 = $txpo;
        }
    };
}

padout!((0, 1) => Pad0, Pad2);

padout!((1, 0) => Pad1, Pad0);
padout!((1, 2) => Pad1, Pad0, Pad2, Pad3);
padout!((1, 1) => Pad1, Pad2);

padout!((2, 0) => Pad2, Pad0);

padout!((3, 0) => Pad3, Pad0);
padout!((3, 1) => Pad3, Pad2);

/// A pad mapping configuration for the SERCOM in UART mode.
///
/// This type can only be constructed using the From implementations
/// in this module, which are restricted to valid configurations.
///
/// Defines which sercom pad is mapped to which UART function.
pub struct Padout<S, RX, TX, RTS, CTS>
where
    S: Sercom,
{
    sercom: PhantomData<S>,
    rx: RX,
    tx: TX,
    rts: RTS,
    cts: CTS,
}

/// A pad mapping configuration for the receiving half of the SERCOM in UART
/// mode.
pub struct RxPadout<S, RX, CTS>
where
    S: Sercom,
{
    sercom: PhantomData<S>,
    rx: RX,
    cts: CTS,
}

/// A pad mapping configuration for the transmitting half of the SERCOM in UART
/// mode.
pub struct TxPadout<S, TX, RTS>
where
    S: Sercom,
{
    sercom: PhantomData<S>,
    tx: TX,
    rts: RTS,
}

impl<S, RX, TX, RTS, CTS> Padout<S, RX, TX, RTS, CTS>
where
    S: Sercom,
{
    /// Splits the padout into transmit and receive halves
    pub fn split(self) -> (TxPadout<S, TX, RTS>, RxPadout<S, RX, CTS>) {
        (
            TxPadout {
                sercom: PhantomData,
                tx: self.tx,
                rts: self.rts,
            },
            RxPadout {
                sercom: PhantomData,
                rx: self.rx,
                cts: self.cts,
            },
        )
    }

    /// Combines transmit and receive halves back into a duplex padout
    pub fn join(tx: TxPadout<S, TX, RTS>, rx: RxPadout<S, RX, CTS>) -> Self {
        Self {
            sercom: PhantomData,
            rx: rx.rx,
            tx: tx.tx,
            rts: tx.rts,
            cts: rx.cts,
        }
    }
}

/// Convert from a tuple of (RX, TX) to UARTXPadout
impl<S, PAD0, PAD1> From<(PAD0, PAD1)> for Padout<S, PAD0, PAD1, (), ()>
where
    S: Sercom,
    PAD0: CompatiblePad<Sercom = S>,
    PAD1: CompatiblePad<Sercom = S>,
    (PAD0::PadNum, PAD1::PadNum): RxpoTxpo,
{
    fn from(pads: (PAD0, PAD1)) -> Padout<S, PAD0, PAD1, (), ()> {
        Padout {
            sercom: PhantomData,
            rx: pads.0,
            tx: pads.1,
            rts: (),
            cts: (),
        }
    }
}

impl<S, PAD0, PAD1> RxpoTxpo for Padout<S, PAD0, PAD1, (), ()>
where
    S: Sercom,
    PAD0: CompatiblePad<Sercom = S>,
    PAD1: CompatiblePad<Sercom = S>,
    (PAD0::PadNum, PAD1::PadNum): RxpoTxpo,
{
    const RXPO: u8 = <(PAD0::PadNum, PAD1::PadNum)>::RXPO;
    const TXPO: u8 = <(PAD0::PadNum, PAD1::PadNum)>::TXPO;
}

/// Convert from a tuple of (RX, TX, RTS, CTS) to UARTXPadout
impl<S, PAD0, PAD1, PAD2, PAD3> From<(PAD0, PAD1, PAD2, PAD3)> for Padout<S, PAD0, PAD1, PAD2, PAD3>
where
    S: Sercom,
    PAD0: CompatiblePad<Sercom = S>,
    PAD1: CompatiblePad<Sercom = S>,
    PAD2: CompatiblePad<Sercom = S>,
    PAD3: CompatiblePad<Sercom = S>,
    (PAD0::PadNum, PAD1::PadNum, PAD2::PadNum, PAD3::PadNum): RxpoTxpo,
{
    fn from(pads: (PAD0, PAD1, PAD2, PAD3)) -> Padout<S, PAD0, PAD1, PAD2, PAD3> {
        Padout {
            sercom: PhantomData,
            rx: pads.0,
            tx: pads.1,
            rts: pads.2,
            cts: pads.3,
        }
    }
}

impl<S, PAD0, PAD1, PAD2, PAD3> RxpoTxpo for Padout<S, PAD0, PAD1, PAD2, PAD3>
where
    S: Sercom,
    PAD0: CompatiblePad<Sercom = S>,
    PAD1: CompatiblePad<Sercom = S>,
    PAD2: CompatiblePad<Sercom = S>,
    PAD3: CompatiblePad<Sercom = S>,
    (PAD0::PadNum, PAD1::PadNum, PAD2::PadNum, PAD3::PadNum): RxpoTxpo,
{
    const RXPO: u8 = <(PAD0::PadNum, PAD1::PadNum, PAD2::PadNum, PAD3::PadNum)>::RXPO;
    const TXPO: u8 = <(PAD0::PadNum, PAD1::PadNum, PAD2::PadNum, PAD3::PadNum)>::TXPO;
}

/// Define a UARTX type for the given Sercom.
///
/// Also defines the valid "pad to uart function" mappings for this instance so
/// that construction is restricted to valid configurations.
macro_rules! uart {
    ($Type:ident: ($Sercom:ident, $SERCOM:ident, $powermask:ident, $clock:ident)) => {
        $crate::paste::item! {
            pub type [<$Type Padout>]<RX, TX, RTS, CTS> = Padout<$Sercom, RX, TX, RTS, CTS>;
            pub type [<$Type TxPadout>]<TX, RTS> = TxPadout<$Sercom, TX, RTS>;
            pub type [<$Type RxPadout>]<RX, CTS> = RxPadout<$Sercom, RX, CTS>;
        }

        $crate::paste::item! {
            /// UARTX represents the corresponding SERCOMX instance
            /// configured to act in the role of a UART Master.
            /// Objects of this type implement the HAL `serial::Read`,
            /// `serial::Write` traits.
            ///
            /// This type is generic over any valid pad mapping where there is
            /// a defined "receive pin out transmit pin out" implementation.
            pub struct $Type<RX, TX, RTS, CTS> {
                padout: Padout<$Sercom, RX, TX, RTS, CTS>,
                sercom: $SERCOM,
            }

            impl<RX, TX, RTS, CTS> $Type<RX, TX, RTS, CTS> {
                /// Power on and configure SERCOMX to work as a UART Master operating
                /// with the specified frequency. The padout specifies
                /// which pins are bound to the RX, TX and optionally RTS and CTS
                /// functions.
                ///
                /// You can use any tuple of two or four SercomXPadY instances
                /// for which there exists a From implementation for
                /// UARTXPadout.
                pub fn new<F: Into<Hertz>, T: Into<Padout<$Sercom, RX, TX, RTS, CTS>>>(
                    clock: &clock::$clock,
                    freq: F,
                    sercom: $SERCOM,
                    pm: &mut PM,
                    padout: T
                ) -> $Type<RX, TX, RTS, CTS> where
                    Padout<$Sercom, RX, TX, RTS, CTS>: RxpoTxpo {
                    let padout = padout.into();

                    pm.apbcmask.modify(|_, w| w.$powermask().set_bit());

                    // Lots of union fields which require unsafe access
                    unsafe {
                        // Reset
                        sercom.usart().ctrla.modify(|_, w| w.swrst().set_bit());
                        while sercom.usart().syncbusy.read().swrst().bit_is_set()
                            || sercom.usart().ctrla.read().swrst().bit_is_set() {
                            // wait for sync of CTRLA.SWRST
                        }

                        // Unsafe b/c of direct call to bits on rxpo/txpo
                        sercom.usart().ctrla.modify(|_, w| {
                            w.dord().set_bit();

                            let (rxpo, txpo) = padout.rxpo_txpo();
                            w.rxpo().bits(rxpo);
                            w.txpo().bits(txpo);

                            w.form().bits(0x00);
                            w.sampr().bits(0x00); // 16x oversample fractional
                            w.runstdby().set_bit(); // Run in standby
                            w.form().bits(0); // 0 is no parity bits

                            w.mode().usart_int_clk() // Internal clock mode
                        });

                        // Calculate value for BAUD register
                        let sample_rate: u8 = 16;
                        let fref = clock.freq().0;

            //          TODO: Support fractional BAUD mode
            //            let mul_ratio = (fref.0 * 1000) / (freq.into().0 * 16);
            //
            //            let baud = mul_ratio / 1000;
            //            let fp = ((mul_ratio - (baud*1000))*8)/1000;
            //
            //            sercom.usart().baud()_frac_mode.modify(|_, w| {
            //                w.baud().bits(baud as u16);
            //                w.fp().bits(fp as u8)
            //            });

                        // Asynchronous arithmetic mode (Table 24-2 in datasheet)
                        let baud = calculate_baud_value(freq.into().0, fref, sample_rate);

                        sercom.usart().baud().modify(|_, w| {
                            w.baud().bits(baud)
                        });

                        sercom.usart().ctrlb.modify(|_, w| {
                            w.sbmode().clear_bit(); // 0 is one stop bit see sec 25.8.2
                            w.chsize().bits(0x0);
                            w.txen().set_bit();
                            w.rxen().set_bit()
                        });

                        while sercom.usart().syncbusy.read().ctrlb().bit_is_set() {}

                        sercom.usart().ctrla.modify(|_, w| w.enable().set_bit());
                        // wait for sync of ENABLE
                        while sercom.usart().syncbusy.read().enable().bit_is_set() {}
                    }

                    Self {
                        padout,
                        sercom,
                    }
                }

                pub fn free(self) -> (Padout<$Sercom, RX, TX, RTS, CTS>, $SERCOM) {
                    (self.padout, self.sercom)
                }

                /// Splits the UART into transmit and receive halves
                pub fn split(self) -> ([<$Type Tx>]<TX, RTS>, [<$Type Rx>]<RX, CTS>) {
                    let (tx_pads, rx_pads) = self.padout.split();
                    (
                        [<$Type Tx>] {
                            padout: tx_pads,
                            sercom: self.sercom,
                        },
                        [<$Type Rx>] {
                            padout: rx_pads,
                            sercom: PhantomData,
                        },
                    )
                }

                /// Combines transmit and receive halves back into a duplex UART
                pub fn join(tx: [<$Type Tx>]<TX, RTS>, rx: [<$Type Rx>]<RX, CTS>) -> Self {
                    Self {
                        padout: [<$Type Padout>]::join(tx.padout, rx.padout),
                        sercom: tx.sercom,
                    }
                }

                /// # Safety
                ///
                /// Only this struct instance should be able to access TX-related fields on this SERCOM.
                unsafe fn usart(&self) -> &USART {
                    return &self.sercom.usart();
                }

                pub fn intenset<F>(&mut self, f: F)
                where F: FnOnce(&mut crate::pac::sercom0::usart::intenset::W)
                {
                    unsafe {
                        self.usart().intenset.write(|w| {
                            f(w);
                            w
                        });
                    }
                }

                pub fn intenclr<F>(&mut self, f: F)
                where F: FnOnce(&mut crate::pac::sercom0::usart::intenclr::W)
                {
                    unsafe {
                        self.usart().intenclr.write(|w| {
                            f(w);
                            w
                        });
                    }
                }

                pub fn flags(&self) -> crate::pac::sercom0::usart::status::R {
                    unsafe {
                        self.usart().status.read()
                    }
                }
            }

            /// The transmitting half of the corresponding UARTX instance (as returned by `UARTX::split`)
            pub struct [<$Type Tx>]<TX, RTS> {
                padout: TxPadout<$Sercom, TX, RTS>,
                /// We store the SERCOM object here so we can retrieve it later,
                /// but conceptually, ownership is shared between the Rx and Tx halves.
                sercom: $SERCOM,
            }

            impl<TX, RTS> [<$Type Tx>]<TX, RTS> {
                /// # Safety
                ///
                /// Only this struct instance should be able to access TX-related fields on this SERCOM.
                unsafe fn usart(&self) -> &USART {
                    return &self.sercom.usart();
                }

                fn do_write(usart: &USART, word: u8) -> nb::Result<(), ()> {
                    unsafe {
                        if !usart.intflag.read().dre().bit_is_set() {
                            return Err(nb::Error::WouldBlock);
                        }

                        usart.data.write(|w| {
                            w.bits(word as u16)
                        });
                    }

                    Ok(())
                }

                fn do_flush(usart: &USART) -> nb::Result<(), ()> {
                    // simply await DRE empty
                    if !usart.intflag.read().dre().bit_is_set() {
                        return Err(nb::Error::WouldBlock);
                    }

                    Ok(())
                }
            }

            impl<TX, RTS> serial::Write<u8> for [<$Type Tx>]<TX, RTS> {
                type Error = ();

                fn write(&mut self, word: u8) -> nb::Result<(), Self::Error> {
                    Self::do_write(unsafe { self.usart() }, word)
                }

                fn flush(&mut self) -> nb::Result<(), Self::Error> {
                    Self::do_flush(unsafe { self.usart() })
                }
            }

            impl<RX, TX, RTS, CTS> serial::Write<u8> for $Type<RX, TX, RTS, CTS> {
                type Error = ();

                fn write(&mut self, word: u8) -> nb::Result<(), Self::Error> {
                    [<$Type Tx>]::<TX, RTS>::do_write(unsafe { self.usart() }, word)
                }

                fn flush(&mut self) -> nb::Result<(), Self::Error> {
                    [<$Type Tx>]::<TX, RTS>::do_flush(unsafe { self.usart() })
                }
            }

            /// The receiving half of the corresponding UARTX instance (as returned by `UARTX::split`)
            pub struct [<$Type Rx>]<RX, CTS> {
                padout: RxPadout<$Sercom, RX, CTS>,
                sercom: PhantomData<$SERCOM>,
            }

            impl<RX, CTS> [<$Type Rx>]<RX, CTS> {
                /// # Safety
                ///
                /// Only this struct instance should be able to access RX-related fields on this SERCOM.
                unsafe fn usart(&self) -> &USART {
                    (*$SERCOM::ptr()).usart()
                }

                fn do_read(usart: &USART) -> nb::Result<u8, ()> {
                    let has_data = usart.intflag.read().rxc().bit_is_set();

                    if !has_data {
                        return Err(nb::Error::WouldBlock);
                    }

                    let data = usart.data.read().bits();

                    Ok(data as u8)
                }
            }

            impl<RX, CTS> serial::Read<u8> for [<$Type Rx>]<RX, CTS> {
                type Error = ();

                fn read(&mut self) -> nb::Result<u8, Self::Error> {
                    Self::do_read(unsafe { self.usart() })
                }
            }

            impl<RX, TX, RTS, CTS> serial::Read<u8> for $Type<RX, TX, RTS, CTS> {
                type Error = ();

                fn read(&mut self) -> nb::Result<u8, Self::Error> {
                    [<$Type Rx>]::<RX, CTS>::do_read(self.sercom.usart())
                }
            }

            impl<TX, RTS> Default<u8> for [<$Type Tx>]<TX, RTS> {}

            impl<RX, TX, RTS, CTS> Default<u8> for $Type<RX, TX, RTS, CTS> {}

            impl<TX, RTS> fmt::Write for [<$Type Tx>]<TX, RTS> {
                fn write_str(&mut self, s: &str) -> fmt::Result {
                    self.bwrite_all(s.as_bytes()).map_err(|_| fmt::Error)
                }
            }

            impl<RX, TX, RTS, CTS> fmt::Write for $Type<RX, TX, RTS, CTS> {
                fn write_str(&mut self, s: &str) -> fmt::Result {
                    self.bwrite_all(s.as_bytes()).map_err(|_| fmt::Error)
                }
            }
        }
    };
}

uart!(UART0: (Sercom0, SERCOM0, sercom0_, Sercom0CoreClock));
uart!(UART1: (Sercom1, SERCOM1, sercom1_, Sercom1CoreClock));
#[cfg(feature = "samd21")]
uart!(UART2: (Sercom2, SERCOM2, sercom2_, Sercom2CoreClock));
#[cfg(feature = "samd21")]
uart!(UART3: (Sercom3, SERCOM3, sercom3_, Sercom3CoreClock));
#[cfg(feature = "min-samd21g")]
uart!(UART4: (Sercom4, SERCOM4, sercom4_, Sercom4CoreClock));
#[cfg(feature = "min-samd21g")]
uart!(UART5: (Sercom5, SERCOM5, sercom5_, Sercom5CoreClock));

const SHIFT: u8 = 32;

fn calculate_baud_value(baudrate: u32, clk_freq: u32, n_samples: u8) -> u16 {
    let sample_rate = (n_samples as u64 * baudrate as u64) << 32;
    let ratio = sample_rate / clk_freq as u64;
    let scale = (1u64 << SHIFT) - ratio;
    let baud_calculated = (65536u64 * scale) >> SHIFT;

    baud_calculated as u16
}
