use crate::clock;
use crate::gpio::v2::pin::AnyPin;
use crate::hal::blocking::serial::{write::Default, Write};
use crate::hal::serial;
use crate::sercom::pads::*;
use crate::sercom::v2;
use crate::target_device::sercom0::USART;
use crate::target_device::{PM, SERCOM0, SERCOM1};
#[cfg(feature = "samd21")]
use crate::target_device::{SERCOM2, SERCOM3};
#[cfg(feature = "min-samd21g")]
use crate::target_device::{SERCOM4, SERCOM5};
use crate::time::Hertz;
use core::fmt;
use core::marker::PhantomData;

/// The RxpoTxpo trait defines a way to get the data in and data out pin out
/// values for a given UARTXPadout configuration. You should not implement
/// this trait for yourself; only the implementations in the sercom module make
/// sense.
pub trait RxpoTxpo {
    fn rxpo_txpo(&self) -> (u8, u8);
}

/// Define a UARTX type for the given Sercom.
///
/// Also defines the valid "pad to uart function" mappings for this instance so
/// that construction is restricted to valid configurations.
macro_rules! uart {
    ($Type:ident: ($Sercom:ident, $SERCOM:ident, $powermask:ident, $clock:ident)) => {
        $crate::paste::item! {
            /// A pad mapping configuration for the SERCOM in UART mode.
            ///
            /// This type can only be constructed using the From implementations
            /// in this module, which are restricted to valid configurations.
            ///
            /// Defines which sercom pad is mapped to which UART function.
            pub struct [<$Type Padout>]<RX, TX, RTS, CTS> {
                rx: RX,
                tx: TX,
                rts: RTS,
                cts: CTS,
            }

            /// A pad mapping configuration for the receiving half of the SERCOM in UART mode.
            pub struct [<$Type RxPadout>]<RX, CTS> {
                rx: RX,
                cts: CTS,
            }

            /// A pad mapping configuration for the transmitting half of the SERCOM in UART mode.
            pub struct [<$Type TxPadout>]<TX, RTS> {
                tx: TX,
                rts: RTS,
            }

            impl<RX, TX, RTS, CTS> [<$Type Padout>]<RX, TX, RTS, CTS> {
                /// Splits the padout into transmit and receive halves
                pub fn split(self) -> ([<$Type TxPadout>]<TX, RTS>, [<$Type RxPadout>]<RX, CTS>) {
                    (
                        [<$Type TxPadout>] {
                            tx: self.tx,
                            rts: self.rts,
                        },
                        [<$Type RxPadout>] {
                            rx: self.rx,
                            cts: self.cts,
                        },
                    )
                }

                /// Combines transmit and receive halves back into a duplex padout
                pub fn join(tx: [<$Type TxPadout>]<TX, RTS>, rx: [<$Type RxPadout>]<RX, CTS>) -> Self {
                    Self {
                        rx: rx.rx,
                        tx: tx.tx,
                        rts: tx.rts,
                        cts: rx.cts,
                    }
                }
            }
        }

        /// Define a From instance for either a tuple of two SercomXPadX
        /// instances, or a tuple of four SercomXPadX instances that converts
        /// them into an UARTXPadout instance.
        ///
        /// Also defines a RxpoTxpo instance for the constructed padout instance
        /// that returns the values used to configure the sercom pads for the
        /// appropriate function in the sercom register file.
        macro_rules! padout {
            ($rxpo_txpo:expr => $pad0:ident, $pad1:ident) => {
                $crate::paste::item! {
                    /// Convert from a tuple of (RX, TX) to UARTXPadout
                    impl<PIN0, PIN1> From<([<$Sercom $pad0>]<PIN0>, [<$Sercom $pad1>]<PIN1>)> for [<$Type Padout>]<[<$Sercom $pad0>]<PIN0>, [<$Sercom $pad1>]<PIN1>, (), ()>
                    where
                        PIN0: AnyPin,
                        PIN1: AnyPin,
                        PIN0::Id: GetPadMode<$Sercom, $pad0>,
                        PIN1::Id: GetPadMode<$Sercom, $pad1>,
                    {
                        fn from(pads: ([<$Sercom $pad0>]<PIN0>, [<$Sercom $pad1>]<PIN1>)) -> [<$Type Padout>]<[<$Sercom $pad0>]<PIN0>, [<$Sercom $pad1>]<PIN1>, (), ()> {
                            [<$Type Padout>] { rx: pads.0, tx: pads.1, rts: (), cts: () }
                        }
                    }

                    impl<PIN0, PIN1> RxpoTxpo for [<$Type Padout>]<[<$Sercom $pad0>]<PIN0>, [<$Sercom $pad1>]<PIN1>, (), ()>
                    where
                        PIN0: AnyPin,
                        PIN1: AnyPin,
                        PIN0::Id: GetPadMode<$Sercom, $pad0>,
                        PIN1::Id: GetPadMode<$Sercom, $pad1>,
                    {
                        fn rxpo_txpo(&self) -> (u8, u8) {
                            $rxpo_txpo
                        }
                    }

                    /// Convert from a tuple of (RX, TX, RTS, CTS) to UARTXPadout
                    impl<Id0, Id1> From<(v2::Pad<$Sercom, $pad0, Id0>, v2::Pad<$Sercom, $pad1, Id1>)> for [<$Type Padout>]<v2::Pad<$Sercom, $pad0, Id0>, v2::Pad<$Sercom, $pad1, Id1>, (), ()>
                    where
                        Id0: v2::GetPadMode<$Sercom, $pad0>,
                        Id1: v2::GetPadMode<$Sercom, $pad1>,
                    {
                        fn from(pads: (v2::Pad<$Sercom, $pad0, Id0>, v2::Pad<$Sercom, $pad1, Id1>)) -> Self {
                            [<$Type Padout>] { rx: pads.0, tx: pads.1, rts: (), cts: () }
                        }
                    }

                    impl<Id0, Id1> RxpoTxpo for [<$Type Padout>]<v2::Pad<$Sercom, $pad0, Id0>, v2::Pad<$Sercom, $pad1, Id1>, (), ()>
                    where
                        Id0: v2::GetPadMode<$Sercom, $pad0>,
                        Id1: v2::GetPadMode<$Sercom, $pad1>,
                    {
                        fn rxpo_txpo(&self) -> (u8, u8) {
                            $rxpo_txpo
                        }
                    }
                }
            };
            ($rxpo_txpo:expr => $pad0:ident, $pad1:ident, $pad2:ident, $pad3:ident) => {
                $crate::paste::item! {
                    /// Convert from a tuple of (RX, TX, RTS, CTS) to UARTXPadout
                    impl<PIN0, PIN1, PIN2, PIN3> From<([<$Sercom $pad0>]<PIN0>, [<$Sercom $pad1>]<PIN1>, [<$Sercom $pad2>]<PIN2>, [<$Sercom $pad3>]<PIN3>)> for [<$Type Padout>]<[<$Sercom $pad0>]<PIN0>, [<$Sercom $pad1>]<PIN1>, [<$Sercom $pad2>]<PIN2>, [<$Sercom $pad3>]<PIN3>>
                    where
                        PIN0: AnyPin,
                        PIN1: AnyPin,
                        PIN2: AnyPin,
                        PIN3: AnyPin,
                        PIN0::Id: GetPadMode<$Sercom, $pad0>,
                        PIN1::Id: GetPadMode<$Sercom, $pad1>,
                        PIN2::Id: GetPadMode<$Sercom, $pad2>,
                        PIN3::Id: GetPadMode<$Sercom, $pad3>,
                    {
                        fn from(pads: ([<$Sercom $pad0>]<PIN0>, [<$Sercom $pad1>]<PIN1>, [<$Sercom $pad2>]<PIN2>, [<$Sercom $pad3>]<PIN3>)) -> [<$Type Padout>]<[<$Sercom $pad0>]<PIN0>, [<$Sercom $pad1>]<PIN1>, [<$Sercom $pad2>]<PIN2>, [<$Sercom $pad3>]<PIN3>> {
                            [<$Type Padout>] { rx: pads.0, tx: pads.1, rts: pads.2, cts: pads.3 }
                        }
                    }

                    impl<PIN0, PIN1, PIN2, PIN3> RxpoTxpo for [<$Type Padout>]<[<$Sercom $pad0>]<PIN0>, [<$Sercom $pad1>]<PIN1>, [<$Sercom $pad2>]<PIN2>, [<$Sercom $pad3>]<PIN3>>
                    where
                        PIN0: AnyPin,
                        PIN1: AnyPin,
                        PIN2: AnyPin,
                        PIN3: AnyPin,
                        PIN0::Id: GetPadMode<$Sercom, $pad0>,
                        PIN1::Id: GetPadMode<$Sercom, $pad1>,
                        PIN2::Id: GetPadMode<$Sercom, $pad2>,
                        PIN3::Id: GetPadMode<$Sercom, $pad3>,
                    {
                        fn rxpo_txpo(&self) -> (u8, u8) {
                            $rxpo_txpo
                        }
                    }

                    /// Convert from a tuple of (RX, TX, RTS, CTS) to UARTXPadout
                    impl<Id0, Id1, Id2, Id3> From<(v2::Pad<$Sercom, $pad0, Id0>, v2::Pad<$Sercom, $pad1, Id1>, v2::Pad<$Sercom, $pad2, Id2>, v2::Pad<$Sercom, $pad3, Id3>)> for [<$Type Padout>]<v2::Pad<$Sercom, $pad0, Id0>, v2::Pad<$Sercom, $pad1, Id1>, v2::Pad<$Sercom, $pad2, Id2>, v2::Pad<$Sercom, $pad3, Id3>>
                    where
                        Id0: v2::GetPadMode<$Sercom, $pad0>,
                        Id1: v2::GetPadMode<$Sercom, $pad1>,
                        Id2: v2::GetPadMode<$Sercom, $pad2>,
                        Id3: v2::GetPadMode<$Sercom, $pad3>,
                    {
                        fn from(pads: (v2::Pad<$Sercom, $pad0, Id0>, v2::Pad<$Sercom, $pad1, Id1>, v2::Pad<$Sercom, $pad2, Id2>, v2::Pad<$Sercom, $pad3, Id3>)) -> Self {
                            [<$Type Padout>] { rx: pads.0, tx: pads.1, rts: pads.2, cts: pads.3 }
                        }
                    }

                    impl<Id0, Id1, Id2, Id3> RxpoTxpo for [<$Type Padout>]<v2::Pad<$Sercom, $pad0, Id0>, v2::Pad<$Sercom, $pad1, Id1>, v2::Pad<$Sercom, $pad2, Id2>, v2::Pad<$Sercom, $pad3, Id3>>
                    where
                        Id0: v2::GetPadMode<$Sercom, $pad0>,
                        Id1: v2::GetPadMode<$Sercom, $pad1>,
                        Id2: v2::GetPadMode<$Sercom, $pad2>,
                        Id3: v2::GetPadMode<$Sercom, $pad3>,
                    {
                        fn rxpo_txpo(&self) -> (u8, u8) {
                            $rxpo_txpo
                        }
                    }
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

        $crate::paste::item! {
            /// UARTX represents the corresponding SERCOMX instance
            /// configured to act in the role of a UART Master.
            /// Objects of this type implement the HAL `serial::Read`,
            /// `serial::Write` traits.
            ///
            /// This type is generic over any valid pad mapping where there is
            /// a defined "receive pin out transmit pin out" implementation.
            pub struct $Type<RX, TX, RTS, CTS> {
                padout: [<$Type Padout>]<RX, TX, RTS, CTS>,
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
                pub fn new<F: Into<Hertz>, T: Into<[<$Type Padout>]<RX, TX, RTS, CTS>>>(
                    clock: &clock::$clock,
                    freq: F,
                    sercom: $SERCOM,
                    pm: &mut PM,
                    padout: T
                ) -> $Type<RX, TX, RTS, CTS> where
                    [<$Type Padout>]<RX, TX, RTS, CTS>: RxpoTxpo {
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

                pub fn free(self) -> ([<$Type Padout>]<RX, TX, RTS, CTS>, $SERCOM) {
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
                where F: FnOnce(&mut crate::target_device::sercom0::usart::intenset::W)
                {
                    unsafe {
                        self.usart().intenset.write(|w| {
                            f(w);
                            w
                        });
                    }
                }

                pub fn intenclr<F>(&mut self, f: F)
                where F: FnOnce(&mut crate::target_device::sercom0::usart::intenclr::W)
                {
                    unsafe {
                        self.usart().intenclr.write(|w| {
                            f(w);
                            w
                        });
                    }
                }

                pub fn flags(&self) -> crate::target_device::sercom0::usart::status::R {
                    unsafe {
                        self.usart().status.read()
                    }
                }
            }

            /// The transmitting half of the corresponding UARTX instance (as returned by `UARTX::split`)
            pub struct [<$Type Tx>]<TX, RTS> {
                padout: [<$Type TxPadout>]<TX, RTS>,
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
                padout: [<$Type RxPadout>]<RX, CTS>,
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
    }
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
