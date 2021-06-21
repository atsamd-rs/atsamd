//! Use the SERCOM peripheral for UART communications
//!
//! Configuring an UART peripheral occurs in three steps. First, you must create
//! a set of [`Pads`] for use by the peripheral. Next, you assemble pieces into
//! a [`Config`] struct. After configuring the peripheral, you then [`enable`]
//! it, yielding a pair of functional [`UartRx`] and/or [`UartTx`] structs.
//! Transactions are performed using the [`serial`](embedded_hal::serial) traits
//! from embedded HAL.
//!
//! # [`Pads`]
//!
//! A [`Sercom`] can use up to four [`Pin`]s as peripheral [`Pad`]s, but only
//! certain [`Pin`] combinations are acceptable. In particular, all [`Pin`]s
//! must be mapped to the same `Sercom` (see the datasheet). This HAL makes it
//! impossible to use invalid [`Pin`]/[`Pad`] combinations, and the [`Pads`]
//! struct is responsible for enforcing these constraints.
//!
//!
//! A `Pads` type takes five type parameters. The first specifies the `Sercom`,
//! while the remaining four, `DI`, `DO`, `CK` and `SS`, represent the Data In,
//! Data Out, Sclk and SS pads respectively. Each of the remaining type
//! parameters is an [`OptionalPad`] and defaults to [`NoneT`]. Aliases defining
//! the pad types can be provided by the [`bsp_pins!`] macro.
//!
//! ```
//! use atsamd_hal::gpio::v2::{PA08, PA09, AlternateC};
//! use atsamd_hal::sercom::v2::{Sercom0, uart};
//! use atsamd_hal::typelevel::NoneT;
//!
//! type Rx = Pin<PA08, AlternateC>;
//! type Tx = Pin<PA09, AlternateC>;
//! type Pads = uart::Pads<Sercom0, Rx, Tx>;
//! ```
#![cfg_attr(
    feature = "samd21",
    doc = "
Alternatively, you can use the [`PadsFromIds`] alias to define a set of
`Pads` in terms of [`PinId`]s instead of `Pin`s. This is useful when you
don't have `Pin` aliases pre-defined.

```
use atsamd_hal::gpio::v2::{PA08, PA09};
use atsamd_hal::sercom::v2::{Sercom0, uart};
use atsamd_hal::typelevel::NoneT;

type Pads = uart::PadsFromIds<Sercom0, PA09, PA09>;
```
"
)]
//! Instaces of `Pads` are created using the builder pattern. Start by creating
//! an empty set of `Pads` using [`Default`]. Then pass each respective `Pin`
//! using the corresponding methods. Both `v1::Pin` and `v2::Pin` types are
//! accepted here.
//!
//! On SAMD21 chips, the builder methods automatically convert each
//! pin to the correct [`PinMode`]. But for SAMD11 chips, users must manually
//! convert each pin before calling the builder methods. This is a consequence
//! of inherent ambiguities in the SAMD11 SERCOM pad definitions. Specifically,
//! the same `PinId` can correspond to two different `PadNum`s for the *same*
//! `Sercom`.
//!
//! ```
//! use atsamd_hal::target_device::Peripherals;
//! use atsamd_hal::gpio::v2::Pins;
//! use atsamd_hal::sercom::v2::{Sercom0, uart};
//!
//! let mut peripherals = Peripherals::take().unwrap();
//! let pins = Pins::new(peripherals.PORT);
//! let pads = uart::Pads::<Sercom0>::default()
//!     .rx(pins.pa09)
//!     .tx(pins.pa08);
//! ```
//!
//! To be accepted as [`ValidPads`], a set of `Pads` must do two things:
//! - Specify a type for `CK` and at least one of `DI` or `DO`
//! - Satisfy the [`RxpoTxpo`] trait
//!
//! # [`Config`]
//!
//! Next, create a [`Config`] struct, which represents the UART peripheral in
//! its disabled state. A `Config` is specified with two type parameters: the
//! [`Pads`] type; and a [`CharSize`], which defaults to [`EightBit`].
//!
//! ```
//! use atsamd_hal::gpio::v2::{PA08, PA09};
//! use atsamd_hal::sercom::v2::{Sercom0, uart};
//! use atsamd_hal::sercom::v2::uart::{NineBit};
//! use atsamd_hal::typelevel::NoneT;
//!
//! type Pads = uart::PadsFromIds<Sercom0, PA08, PA09>;
//! type Config = uart::Config<Pads, NineBit>;
//! ```
//!
//! Upon creation, the [`Config`] takes ownership of both the [`Pads`] struct
//! and the PAC [`Sercom`] struct. It takes a reference to the PM, so that it
//! can enable the APB clock, and it takes a frequency to indicate the GCLK
//! configuration. Users are responsible for correctly configuring the GCLK.
//!
//! ```
//! use atsamd_hal::time::U32Ext;
//!
//! let pm = peripherals.PM;
//! let sercom = peripherals.SERCOM0;
//! // Configure GCLK for 10 MHz
//! let freq = 10.mhz();
//! let config = uart::Config::new(&pm, sercom, pads, freq);
//! ```
//!
//! The [`Config`] struct uses the builder pattern to configure the peripheral,
//! ending with a call to [`enable`], which consumes the [`Config`] and returns
//! an enabled [`UartRx`] and/or [`UartTx`] peripheral(s). Note that the
//! [`Enable`] trait must be in scope to allow calling [`enable`] on a
//! [`Config`]. This limitation is necessary for [`enable`] to return different
//! types, depending on whether the [`Pads`] can transmit, receive, or both.
//!
//! ```
//! use atsamd_hal::sercom::v2::uart::{StopBits, NineBit, Enable};
//!
//! let (rx, tx) = uart::Config::new(&mclk, sercom, pads, freq)
//!     .baud(1.mhz())
//!     .char_size::<NineBit>()
//!     .msb_first(false)
//!     .stop_bits(StopBits::TwoBits)
//!     .enable();
//! ```
//!
//! To be accepted as a [`ValidConfig`], the [`Config`] must have at least one
//! of `Rx` or `Tx` pads.
//!
//! # [`UartRx`] and [`UartTx`]
//!
//! [`UartRx`] and [`UartTx`] structs can only be created from a [`Config`], and
//! have only one type parameter, the corresponding config. The nature of the
//! underlying [`Pads`] contained inside [`Config`] determines the type returned
//! by a call to [`enable`] If the pads only have a `TX` pin specified (ie, the
//! [`Pads`] struct is [`Tx`] + [`NotRx`]), then [`enable`] will return a
//! [`UartTx`]. Similarly, If the pads only have a `RX` pin specified (ie, the
//! [`Pads`] struct is [`Rx`] + [`NotTx`]), then [`enable`] will return a
//! [`UartRx`]. Finally, if both `RX` and `TX` pins are specified ([`Pads`] is
//! [`Tx`] + [`Rx`]), then [`enable`] will return a `(UartRx, UartTx)`
//! tuple.
//!
//! ```
//! use atsamd_hal::gpio::v2::{PA08, PA09};
//! use atsamd_hal::sercom::v2::{Sercom0, uart};
//! use atsamd_hal::sercom::v2::uart::{Master, NineBit};
//! use atsamd_hal::typelevel::NoneT;
//!
//! // Assuming SAMD21
//! type Pads = uart::PadsFromIds<Sercom0, PA08, NoneT, PA09>;
//! type Config = uart::Config<Pads, NineBit>;
//! type UartRx = uart::UartRx<Config>;
//! type UartTx = uart::UartTx<Config>;
//! ```
//!
//! Only the [`UartRx`] and [`UartTx`] structs can actually perform
//! transactions. To do so, use the embedded HAL traits, like
//! [`serial::Read`](Read) and [`serial::Write`](Write).
//!
//! ```
//! use nb::block;
//! use embedded_hal::serial::Write;
//!
//! block!(uart_tx.write(0x0fe));
//! ```
//!
//! # The [`Uart`] trait
//!
//! Since [`UartRx`] and [`UartTx`] operate independently, the [`Uart`] trait
//! was designed to provide some common methods, such as
//! reading/enabling/disabling interrupt or status flags, etc. These methods
//! only make use of the flags that are relevant to the struct's function. ie, a
//! [`UartRx`] can only access interrupt flags that are pertinent to RX
//! functionality, and cannot access the flags pertinent to TX functionality.
//! Bring the [`Uart`] trait in scope to make use of these methods.
//!
//! ```
//! use atsamd_hal::sercom::v2::uart::Uart;
//! ```
//!
//! # Disabling and reconfiguring
//!
//! Some methods, such as [`disable`] and [`reconfigure`], need to operate on
//! all parts of a UART at once. In practice, this means that these methods
//! operate on the type that was returned by [`enable`]. This can be [`UartRx`],
//! [`UartTx`], or a `([UartRx], [UartTx])` tuple, depending on how the
//! peripheral was configured. Note that like [`enable`], [`disable`] and
//! [`reconfigure`] need the [`Disable`] trait to be in scope.
//!
//! ```
//! use atsamd_hal::sercom::v2::uart::{Enable, Disable};
//! // Assume config is a valid UART Config struct
//! let (rx, tx) = config.enable();
//!
//! // Send/receive data with tx/rx...
//!
//! (rx, tx).reconfigure(|c| c.baud(1.mhz()));
//! ```
//!
//!
//! # Non-supported advanced features
//!
//! * Synchronous mode (USART) is not supported
//!
//! [`enable`]: Enable::enable
//! [`disable`]: Disable::disable
//! [`reconfigure`]: Disable::reconfigure
//! [`bsp_pins`]: crate::bsp_pins
//! [`Pin`]: crate::gpio::v2::pin::Pin
//! [`PinId`]: crate::gpio::v2::pin::PinId
//! [`PinMode`]: crate::gpio::v2::pin::PinMode
#![cfg_attr(
    feature = "dma",
    doc = "
# Using UART with DMA

This HAL includes support for DMA-enabled UART transfers. [`UartRx`] and
[`UartTx`] both implement the DMAC [`Buffer`]
trait. The provided [`send_with_dma`] and
[`receive_with_dma`] build and begin a
[`dmac::Transfer`], thus starting the UART
in a non-blocking way. Optionally, interrupts can be enabled on the provided
[`Channel`]. Note that the `dma` feature must
be enabled. Please refer to the [`dmac`](crate::dmac) module-level
documentation for more information.

```
/// Assume channel0 and channel1 are configured `dmac::Channel`s,
/// rx is a UartRx, and tx is a UartTx.

/// Create data to send
let tx_buffer: [u8; 50] = [0xff; 50];
let rx_buffer: [u8; 100] = [0xab; 100];

/// Launch transmit transfer
let tx_dma = tx.send_with_dma(&mut tx_buffer, channel0, ());

/// Launch receive transfer
let rx_dma = rx.receive_with_dma(&mut rx_buffer, channel1, ());

/// Wait for transfers to complete and reclaim resources
let (chan0, tx_buffer, tx) = tx_dma.wait();
let (chan1, rx, rx_buffer) = rx_dma.wait();
```

[`Buffer`]: crate::dmac::transfer::Buffer
[`send_with_dma`]: UartTx::send_with_dma
[`receive_with_dma`]: UartRx::receive_with_dma
[`dmac::Transfer`]: crate::dmac::Transfer
[`Channel`]: crate::dmac::channel::Channel
[`dmac`]: crate::dmac
"
)]

use core::convert::{TryFrom, TryInto};
use core::marker::PhantomData;

use crate::target_device as pac;
use crate::time::Hertz;
use pac::{
    sercom0::{usart::ctrla::MODE_A, RegisterBlock},
    PM,
};

#[cfg(not(feature = "samd11"))]
use crate::gpio::v2::AnyPin;
use crate::sercom::v2::*;
use crate::typelevel::{Is, NoneT, Sealed};

use bitflags::bitflags;
use embedded_hal::blocking;
use embedded_hal::serial::{Read, Write};
use nb::Error::WouldBlock;
use num_traits::{AsPrimitive, PrimInt};

//=============================================================================
// RxpoTxpo
//=============================================================================

/// Configure the `RXPO` and `TXPO` fields based on a set of [`Pads`]
///
/// According to the datasheet, the `RXPO` and `TXPO` values specify which
/// SERCOM pads are used for various functions. Moreover, depending on which
/// pads are actually in use, only certain combinations of these values make
/// sense and are valid.
///
/// This trait is implemented for valid, four-tuple combinations of
/// [`OptionalPadNum`]s. Those implementations are then lifted to the
/// corresponding [`Pads`] types.
///
/// To satisfy this trait, the combination of [`OptionalPadNum`]s must specify
/// [`PadNum`] for at least one of `RX` and `TX`. Furthermore, no
/// two [`PadNum`]s can conflict.
pub trait RxpoTxpo {
    /// `RXPO` field value
    const RXPO: u8;

    /// `RXPO` field value
    const TXPO: u8;

    /// Configure the pad according to [`Self::RXPO`] and [`Self::TXPO`]
    #[inline]
    fn configure(sercom: &RegisterBlock) {
        sercom.usart().ctrla.modify(|_, w| unsafe {
            w.rxpo().bits(Self::RXPO);
            w.txpo().bits(Self::TXPO)
        });
    }
}

/// Lift the implementations of [`RxpoTxpo`] from four-tuples of
/// [`OptionalPadNum`]s to the corresponding [`Pads`] types.
impl<S, RX, TX, RTS, CTS> RxpoTxpo for Pads<S, RX, TX, RTS, CTS>
where
    S: Sercom,
    RX: OptionalPad,
    TX: OptionalPad,
    RTS: OptionalPad,
    CTS: OptionalPad,
    (RX::PadNum, TX::PadNum, RTS::PadNum, CTS::PadNum): RxpoTxpo,
{
    const RXPO: u8 = <(RX::PadNum, TX::PadNum, RTS::PadNum, CTS::PadNum)>::RXPO;
    const TXPO: u8 = <(RX::PadNum, TX::PadNum, RTS::PadNum, CTS::PadNum)>::TXPO;
}

//=============================================================================
// Implement RxpoTxpo
//=============================================================================

/// Filter [`PadNum`] permutations and implement [`RxpoTxpo`]
#[rustfmt::skip]
macro_rules! impl_rxpotxpo {
    // This is the entry pattern. Start by checking RTS and CTS.
    ($RX:ident, $TX:ident, $RTS:ident, $CTS:ident) => { impl_rxpotxpo!(@check_rts_cts, $RX, $TX, $RTS, $CTS); };

    // Check whether RTS and CTS form a valid pair.
    // They both must be the correct pad or absent.
    (@check_rts_cts, $RX:ident, $TX:ident, NoneT, NoneT) => { impl_rxpotxpo!(@assign_rxpo, $RX, $TX, NoneT, NoneT); };
    (@check_rts_cts, $RX:ident, $TX:ident, Pad2, NoneT) => { impl_rxpotxpo!(@assign_rxpo, $RX, $TX, Pad2, NoneT); };
    (@check_rts_cts, $RX:ident, $TX:ident, NoneT, Pad3) => { impl_rxpotxpo!(@assign_rxpo, $RX, $TX, NoneT, Pad3); };
    (@check_rts_cts, $RX:ident, $TX:ident, Pad2, Pad3) => { impl_rxpotxpo!(@assign_rxpo, $RX, $TX, Pad2, Pad3); };

    // If RTS and CTS are not valid, fall through to this pattern.
    (@check_rts_cts, $RX:ident, $TX:ident, $RTS:ident, $CTS:ident) => { };

    // Assign RXPO based on RX.
    // Our options are exhaustive, so no fall through pattern is needed.
    (@assign_rxpo, NoneT, $TX:ident, $RTS:ident, $CTS:ident) => { impl_rxpotxpo!(@assign_txpo, NoneT, $TX, $RTS, $CTS, 0); };
    (@assign_rxpo, Pad0,  $TX:ident, $RTS:ident, $CTS:ident) => { impl_rxpotxpo!(@assign_txpo, Pad0,  $TX, $RTS, $CTS, 0); };
    (@assign_rxpo, Pad1,  $TX:ident, $RTS:ident, $CTS:ident) => { impl_rxpotxpo!(@assign_txpo, Pad1,  $TX, $RTS, $CTS, 1); };
    (@assign_rxpo, Pad2,  $TX:ident, $RTS:ident, $CTS:ident) => { impl_rxpotxpo!(@assign_txpo, Pad2,  $TX, $RTS, $CTS, 2); };
    (@assign_rxpo, Pad3,  $TX:ident, $RTS:ident, $CTS:ident) => { impl_rxpotxpo!(@assign_txpo, Pad3,  $TX, $RTS, $CTS, 3); };

    // Assign TXPO based on RX and RTS
    (@assign_txpo, $RX:ident, NoneT, NoneT, $CTS:ident, $RXPO:literal) => { impl_rxpotxpo!(@filter_conflicts, $RX, NoneT, NoneT, $CTS, $RXPO, 0); };
    (@assign_txpo, $RX:ident, NoneT, Pad2, $CTS:ident, $RXPO:literal) => { impl_rxpotxpo!(@filter_conflicts, $RX, NoneT, Pad2, $CTS, $RXPO, 2); };
    (@assign_txpo, $RX:ident, Pad0, NoneT, $CTS:ident, $RXPO:literal) => { impl_rxpotxpo!(@filter_conflicts, $RX, Pad0, NoneT, $CTS, $RXPO, 0); };
    (@assign_txpo, $RX:ident, Pad2, NoneT, $CTS:ident, $RXPO:literal) => { impl_rxpotxpo!(@filter_conflicts, $RX, Pad2, NoneT, $CTS, $RXPO, 1); };
    (@assign_txpo, $RX:ident, Pad0, Pad2, $CTS:ident, $RXPO:literal) => { impl_rxpotxpo!(@filter_conflicts, $RX, Pad0, Pad2, $CTS, $RXPO, 2); };

    // If TX is not valid, fall through to this pattern.
    (@assign_txpo, $RX:ident, $TX:ident, $RTS:ident, $CTS:ident, $RXPO:literal) => { };

    // Filter any remaining permutations that conflict.
    (@filter_conflicts, NoneT, NoneT, $RTS:ident, $CTS:ident, $RXPO:literal, $TXPO:literal) => { };
    (@filter_conflicts, Pad0, Pad0, $RTS:ident, $CTS:ident, $RXPO:literal, $TXPO:literal) => { };
    (@filter_conflicts, Pad2, Pad2, $RTS:ident, $CTS:ident, $RXPO:literal, $TXPO:literal) => { };
    (@filter_conflicts, Pad2, $TX:ident, Pad2, $CTS:ident, $RXPO:literal, $TXPO:literal) => { };
    (@filter_conflicts, Pad3, $TX:ident, $RTS:ident, Pad3, $RXPO:literal, $TXPO:literal) => { };
    (@filter_conflicts, $RX:ident, Pad2, Pad2, $CTS:ident, $RXPO:literal, $TXPO:literal) => { };

    // If there are no conflicts, fall through to this pattern and implement RxpoTxpo
    (@filter_conflicts, $RX:ident, $TX:ident, $RTS:ident, $CTS:ident, $RXPO:literal, $TXPO:literal) => {
        impl RxpoTxpo for ($RX, $TX, $RTS, $CTS) {
            const RXPO: u8 = $RXPO;
            const TXPO: u8 = $TXPO;
        }
    };
}

/// Try to implement [`RxpoTxpo`] on all possible 4-tuple permutations of
/// [`OptionalPadNum`]s.
///
/// The leading `()` token tree stores a growing permutation of [`PadNum`]s.
/// When it reaches four [`PadNum`]s, try to implement [`RxpoTxpo`].
///
/// The next `[]` token tree is a list of possible [`PadNum`]s to append to the
/// growing permutation. Loop through this list and append each option to the
/// permutation.
///
/// The final, optional `[]` token tree exists to temporarily store the entire
/// list before pushing it down for the next permutation element.
macro_rules! padnum_permutations {
    // If we have built up four [`PadNum`]s, try to implement [`RxpoTxpo`].
    // Ignore the remaining list of [`PadNum`]s.
    (
        ( $RX:ident $TX:ident $RTS:ident $CTS:ident ) [ $( $Pads:ident )* ]
    ) => {
        impl_rxpotxpo!($RX, $TX, $RTS, $CTS);
    };
    // If we only have one list of [`PadNum`]s, duplicate it, to save it for the
    // next permutation element.
    (
        ( $($Perm:ident)* ) [ $($Pads:ident)+ ]
    ) => {
        padnum_permutations!( ( $($Perm)* ) [ $($Pads)+ ] [ $($Pads)+ ] );
    };
    (
        ( $($Perm:ident)* ) [ $Head:ident $($Tail:ident)* ] [ $($Pads:ident)+ ]
    ) => {
        // Append the first [`PadNum`] from the list, then push down to the next
        // permutation element.
        padnum_permutations!( ( $($Perm)* $Head ) [ $($Pads)+ ] );

        // Loop through the remaining [`PadNum`]s to do the same thing for each.
        padnum_permutations!( ( $($Perm)* ) [ $($Tail)* ] [ $($Pads)+ ] );
    };
    // Once the list of [`PadNum`]s is empty, we're done with this element.
    ( ( $($Perm:ident)* ) [ ] [ $($Pads:ident)+ ] ) => { };
}

padnum_permutations!( () [NoneT Pad0 Pad1 Pad2 Pad3] );

//=============================================================================
// Pads
//=============================================================================

/// Container for a set of SERCOM [`Pad`]s
///
/// See the [module-level](self) documentation for more details on specifying
/// a `Pads` type and creating instances.
pub struct Pads<S, RX = NoneT, TX = NoneT, RTS = NoneT, CTS = NoneT>
where
    S: Sercom,
    RX: OptionalPad,
    TX: OptionalPad,
    RTS: OptionalPad,
    CTS: OptionalPad,
{
    sercom: PhantomData<S>,
    receive: RX,
    transmit: TX,
    ready_to_send: RTS,
    clear_to_send: CTS,
}

impl<S: Sercom> Default for Pads<S> {
    fn default() -> Self {
        Self {
            sercom: PhantomData,
            receive: NoneT,
            transmit: NoneT,
            ready_to_send: NoneT,
            clear_to_send: NoneT,
        }
    }
}

impl<S, RX, TX, RTS, CTS> Pads<S, RX, TX, RTS, CTS>
where
    S: Sercom,
    RX: OptionalPad,
    TX: OptionalPad,
    RTS: OptionalPad,
    CTS: OptionalPad,
{
    /// Consume the [`Pads`] and return each individual [`Pad`]
    #[inline]
    pub fn free(self) -> (RX, TX, RTS, CTS) {
        (
            self.receive,
            self.transmit,
            self.ready_to_send,
            self.clear_to_send,
        )
    }
}

#[cfg(feature = "samd11")]
impl<S, RX, TX, RTS, CTS> Pads<S, RX, TX, RTS, CTS>
where
    S: Sercom,
    RX: OptionalPad,
    TX: OptionalPad,
    RTS: OptionalPad,
    CTS: OptionalPad,
{
    /// Set the `RX` [`Pad`]
    #[inline]
    pub fn rx<P: IsPad>(self, pin: P) -> Pads<S, P, TX, RTS, CTS> {
        Pads {
            sercom: self.sercom,
            receive: pin,
            transmit: self.transmit,
            ready_to_send: self.ready_to_send,
            clear_to_send: self.clear_to_send,
        }
    }

    /// Set the `TX` [`Pad`]
    #[inline]
    pub fn tx<P: IsPad>(self, pin: P) -> Pads<S, RX, P, RTS, CTS> {
        Pads {
            sercom: self.sercom,
            receive: self.receive,
            transmit: pin,
            ready_to_send: self.ready_to_send,
            clear_to_send: self.clear_to_send,
        }
    }

    /// Set the `RTS` [`Pad`]
    #[inline]
    pub fn rts<P: IsPad>(self, pin: P) -> Pads<S, RX, TX, P, CTS> {
        Pads {
            sercom: self.sercom,
            receive: self.receive,
            transmit: self.transmit,
            ready_to_send: pin,
            clear_to_send: self.clear_to_send,
        }
    }

    /// Set the `CTS` [`Pad`]
    #[inline]
    pub fn cts<P: IsPad>(self, pin: P) -> Pads<S, RX, TX, RTS, P> {
        Pads {
            sercom: self.sercom,
            receive: self.receive,
            transmit: self.transmit,
            ready_to_send: self.ready_to_send,
            clear_to_send: pin,
        }
    }
}

#[cfg(feature = "samd21")]
impl<S, RX, TX, RTS, CTS> Pads<S, RX, TX, RTS, CTS>
where
    S: Sercom,
    RX: OptionalPad,
    TX: OptionalPad,
    RTS: OptionalPad,
    CTS: OptionalPad,
{
    /// Set the `RX` [`Pad`]
    #[inline]
    pub fn rx<I>(self, pin: impl AnyPin<Id = I>) -> Pads<S, Pad<S, I>, TX, RTS, CTS>
    where
        I: GetPad<S>,
        Pad<S, I>: IsPad,
    {
        Pads {
            sercom: self.sercom,
            receive: pin.into().into_mode(),
            transmit: self.transmit,
            ready_to_send: self.ready_to_send,
            clear_to_send: self.clear_to_send,
        }
    }

    /// Set the `TX` [`Pad`]
    #[inline]
    pub fn tx<I>(self, pin: impl AnyPin<Id = I>) -> Pads<S, RX, Pad<S, I>, RTS, CTS>
    where
        I: GetPad<S>,
        Pad<S, I>: IsPad,
    {
        Pads {
            sercom: self.sercom,
            receive: self.receive,
            transmit: pin.into().into_mode(),
            ready_to_send: self.ready_to_send,
            clear_to_send: self.clear_to_send,
        }
    }

    /// Set the `RTS` [`Pad`]
    #[inline]
    pub fn rts<I>(self, pin: impl AnyPin<Id = I>) -> Pads<S, RX, TX, Pad<S, I>, CTS>
    where
        I: GetPad<S>,
        Pad<S, I>: IsPad,
    {
        Pads {
            sercom: self.sercom,
            receive: self.receive,
            transmit: self.transmit,
            ready_to_send: pin.into().into_mode(),
            clear_to_send: self.clear_to_send,
        }
    }

    /// Set the `CTS` [`Pad`]
    #[inline]
    pub fn cts<I>(self, pin: impl AnyPin<Id = I>) -> Pads<S, RX, TX, RTS, Pad<S, I>>
    where
        I: GetPad<S>,
        Pad<S, I>: IsPad,
    {
        Pads {
            sercom: self.sercom,
            receive: self.receive,
            transmit: self.transmit,
            ready_to_send: self.ready_to_send,
            clear_to_send: pin.into().into_mode(),
        }
    }
}

/// Define a set of [`Pads`] using [`PinId`]s instead of [`Pin`]s
///
/// In some cases, it is more convenient to specify a set of `Pads` using
/// `PinId`s rather than `Pin`s. This alias makes it easier to do so.
///
/// The first type parameter is the [`Sercom`], while the remaining four are
/// effectively [`OptionalPinId`]s representing the corresponding type
/// parameters of [`Pads`], i.e. `RX`, `TX`, `RTS` & `CTS`. Each of the
/// remaining type parameters defaults to [`NoneT`].
///
/// ```
/// use atsamd_hal::pac::Peripherals;
/// use atsamd_hal::gpio::v2::{PA08, PA09, Pins};
/// use atsamd_hal::sercom::v2::{Sercom0, uart};
/// use atsamd_hal::typelevel::NoneT;
///
/// pub type Pads = uart::PadsFromIds<Sercom0, PA08, PA09>;
///
/// pub fn create_pads() -> Pads {
///     let peripherals = Peripherals::take().unwrap();
///     let pins = Pins::new(peripherals.PORT);
///     uart::Pads::default().rx(pins.pa09).tx(pins.pa08)
/// }
/// ```
///
/// [`Pin`]: crate::gpio::v2::Pin
/// [`PinId`]: crate::gpio::v2::PinId
/// [`OptionalPinId`]: crate::gpio::v2::OptionalPinId

#[cfg(feature = "samd21")]
pub type PadsFromIds<S, RX = NoneT, TX = NoneT, RTS = NoneT, CTS = NoneT> = Pads<
    S,
    <RX as GetOptionalPad<S>>::Pad,
    <TX as GetOptionalPad<S>>::Pad,
    <RTS as GetOptionalPad<S>>::Pad,
    <CTS as GetOptionalPad<S>>::Pad,
>;

//=============================================================================
// PadSet
//=============================================================================

/// Type-level function to recover the [`OptionalPad`] types from a generic set
/// of [`Pads`]
///
/// This trait is used as an interface between the [`Pads`] type and other
/// types in this module. It acts as a [type-level function], returning the
/// corresponding [`Sercom`] and [`OptionalPad`] types. It serves to cut down on
/// the total number of type parameters needed in the [`Config`] struct. The
/// [`Config`] struct doesn't need access to the [`Pin`]s directly.  Rather, it
/// only needs to apply the [`SomePad`] trait bound when a `Pin` is required.
/// The [`PadSet`] trait allows each [`Config`] struct to store an instance of
/// [`Pads`] without itself being generic over all six type parameters of the
/// [`Pads`] type.
///
/// [`Pin`]: crate::gpio::v2::Pin
/// [type-level function]: crate::typelevel#type-level-functions
pub trait PadSet: Sealed {
    type Sercom: Sercom;
    type Rx: OptionalPad;
    type Tx: OptionalPad;
    type Rts: OptionalPad;
    type Cts: OptionalPad;
}

impl<S, RX, TX, RTS, CTS> Sealed for Pads<S, RX, TX, RTS, CTS>
where
    S: Sercom,
    RX: OptionalPad,
    TX: OptionalPad,
    RTS: OptionalPad,
    CTS: OptionalPad,
{
}

impl<S, RX, TX, RTS, CTS> PadSet for Pads<S, RX, TX, RTS, CTS>
where
    S: Sercom,
    RX: OptionalPad,
    TX: OptionalPad,
    RTS: OptionalPad,
    CTS: OptionalPad,
{
    type Sercom = S;
    type Rx = RX;
    type Tx = TX;
    type Rts = RTS;
    type Cts = CTS;
}

//=============================================================================
// ValidPads
//=============================================================================

/// Marker trait for valid sets of [`Pads`]
///
/// This trait labels sets of [`Pads`] that satisfy the [`RxpoTxpo`] trait. It
/// guarantees to the [`Config`] struct that this set of `Pads` can be
/// configured through that trait.
pub trait ValidPads: PadSet + RxpoTxpo {}

impl<P: PadSet + RxpoTxpo> ValidPads for P {}

//=============================================================================
// Tx/Rx
//=============================================================================

/// Marker trait for a set of [`Pads`] that can transmit
///
/// To transmit, Tx must be [`SomePad`].
pub trait Tx: ValidPads {}

impl<P> Tx for P
where
    P: ValidPads,
    P::Tx: SomePad,
{
}

/// Marker trait for a set of [`Pads`] that can receive
///
/// To receive, Rx must be [`SomePad`].
pub trait Rx: ValidPads {}

impl<P> Rx for P
where
    P: ValidPads,
    P::Rx: SomePad,
{
}

/// Marker trait for a set of [`Pads`] that cannot transmit
///
/// A set of [`Pads`] cannot be used to transmit when the Tx pad is
/// [`NoneT`].
pub trait NotTx: ValidPads {}

impl<P> NotTx for P where P: ValidPads<Tx = NoneT> {}

/// Marker trait for a set of [`Pads`] that cannot receive
///
/// A set of [`Pads`] cannot be used to receive when the Rx pad is
/// [`NoneT`].
pub trait NotRx: ValidPads {}

impl<P> NotRx for P where P: ValidPads<Rx = NoneT> {}

/// Marker trait for a set of [`Pads`] that can transmit OR receive
///
/// To satisfy this trait, one or both of Rx and Tx must be [`SomePad`].
pub trait TxOrRx: ValidPads {}

impl<S, RX, RTS> TxOrRx for Pads<S, RX, NoneT, RTS, NoneT>
where
    S: Sercom,
    RX: SomePad,
    RTS: OptionalPad,
    Self: RxpoTxpo,
{
}

impl<S, TX, CTS> TxOrRx for Pads<S, NoneT, TX, NoneT, CTS>
where
    S: Sercom,
    TX: SomePad,
    CTS: OptionalPad,
    Self: RxpoTxpo,
{
}

impl<P: Tx + Rx> TxOrRx for P {}

//=============================================================================
// Character size
//=============================================================================

/// Type-level `enum` representing the UART character size
///
/// The UART character size affects the word size for the embedded HAL traits.
/// Eight or less bit transactions use a `u8` word, while nine-bit transactions
/// use a `u16` word.
pub trait CharSize: Sealed {
    /// Word size for the character size
    type Word: 'static;

    /// TODO
    const BITS: u8;

    /// Configure the `LENGTH` register and enable the `LENGTH` counter
    #[inline]
    fn configure(sercom: &RegisterBlock) -> () {
        sercom
            .usart()
            .ctrlb
            .modify(|_, w| unsafe { w.chsize().bits(Self::BITS) });
    }
}

/// Type alias to recover the `Word` type from an implementation of [`CharSize`]
pub type Word<C> = <C as CharSize>::Word;

/// [`CharSize`] variant for 5-bit transactions
pub enum FiveBit {}

/// [`CharSize`] variant for 6-bit transactions
pub enum SixBit {}

/// [`CharSize`] variant for 7-bit transactions
pub enum SevenBit {}

/// [`CharSize`] variant for 8-bit transactions
pub enum EightBit {}

/// [`CharSize`] variant for 9-bit transactions
pub enum NineBit {}

impl Sealed for FiveBit {}
impl CharSize for FiveBit {
    type Word = u8;
    const BITS: u8 = 0x5;
}

impl Sealed for SixBit {}
impl CharSize for SixBit {
    type Word = u8;
    const BITS: u8 = 0x6;
}

impl Sealed for SevenBit {}
impl CharSize for SevenBit {
    type Word = u8;
    const BITS: u8 = 0x7;
}

impl Sealed for EightBit {}
impl CharSize for EightBit {
    type Word = u8;
    const BITS: u8 = 0x0;
}

impl Sealed for NineBit {}
impl CharSize for NineBit {
    type Word = u16;
    const BITS: u8 = 0x1;
}

//=============================================================================
// Flags
//=============================================================================
#[doc(hidden)]
pub trait Bitfield<B>: Sealed {
    fn from_bits_truncate(bits: B) -> Self;
    fn into_bits(self) -> B;
}

/// Type-level enum representing available interrupt flag types
pub trait Flags: Sealed + Bitfield<u8> {}

bitflags! {
    /// Interrupt bit flags for UART Rx transactions
    ///
    /// The available interrupt flags are `RXC`, `RXS`, `RXBRK` and
    /// `ERROR`. The binary format of the underlying bits exactly matches the
    /// INTFLAG bits that logically belong to a Tx transaction.
    pub struct RxFlags: u8 {
        const RXC = 0x04;
        const RXS = 0x08;
        const RXBRK = 0x20;
        const ERROR = 0x80;
    }
}

impl Bitfield<u8> for RxFlags {
    #[inline]
    fn from_bits_truncate(bits: u8) -> Self {
        Self::from_bits_truncate(bits)
    }

    #[inline]
    fn into_bits(self) -> u8 {
        self.bits()
    }
}
impl Flags for RxFlags {}
impl Sealed for RxFlags {}

bitflags! {
    /// Interrupt bit flags for UART Tx transactions
    ///
    /// The available interrupt flags are `DRE`, `RXC` and `CTSIC`
    /// The binary format of the underlying bits exactly matches the
    /// INTFLAG bits that logically belong to a Tx transaction.
    pub struct TxFlags: u8 {
        const DRE = 0x01;
        const TXC = 0x02;
        const CTSIC = 0x10;
    }
}

impl Bitfield<u8> for TxFlags {
    #[inline]
    fn from_bits_truncate(bits: u8) -> Self {
        Self::from_bits_truncate(bits)
    }

    #[inline]
    fn into_bits(self) -> u8 {
        self.bits()
    }
}
impl Flags for TxFlags {}
impl Sealed for TxFlags {}

//=============================================================================
// Status
//=============================================================================

/// Type-level enum representing available Uart status types
pub trait Status: Sealed + Bitfield<u16> {}

bitflags! {
    /// Status flags for UART Rx transactions
    ///
    /// The available error flags are `PERR`, `FERR`, `BUFOVF`,
    /// `CTS`, `ISF` and `COLL`.
    /// The binary format of the underlying bits exactly matches
    /// the STATUS bits that logically belong to a Rx transaction.
    pub struct RxStatus: u16 {
        const PERR = 0x0001;
        const FERR = 0x0002;
        const BUFOVF = 0x0004;
        const ISF = 0x0010;
        const COLL = 0x0020;
    }
}

impl Bitfield<u16> for RxStatus {
    #[inline]
    fn from_bits_truncate(bits: u16) -> Self {
        Self::from_bits_truncate(bits)
    }

    #[inline]
    fn into_bits(self) -> u16 {
        self.bits()
    }
}
impl Status for RxStatus {}
impl Sealed for RxStatus {}

bitflags! {
    /// Status flags for UART Rx transactions
    ///
    /// The available status flags are `CTS``.
    /// The binary format of the underlying bits exactly matches
    /// the STATUS bits that logically belong to a Tx transaction.
    pub struct TxStatus: u16 {
        const CTS = 0x0008;
    }
}

impl Bitfield<u16> for TxStatus {
    #[inline]
    fn from_bits_truncate(bits: u16) -> Self {
        Self::from_bits_truncate(bits)
    }

    #[inline]
    fn into_bits(self) -> u16 {
        self.bits()
    }
}
impl Status for TxStatus {}
impl Sealed for TxStatus {}

/// Error `enum` for UART RX transactions
#[derive(Debug)]
pub enum RxError {
    /// Detected a parity error
    ParityError,
    /// Detected a frame error
    FrameError,
    /// Detected a buffer overflow
    Overflow,
    /// Detected an inconsistent sync field
    InconsistentSyncField,
    /// Detected a collision
    CollisionDetected,
}

impl TryFrom<RxStatus> for () {
    type Error = RxError;

    #[inline]
    fn try_from(errors: RxStatus) -> Result<(), RxError> {
        use RxError::*;
        if errors.contains(RxStatus::PERR) {
            Err(ParityError)
        } else if errors.contains(RxStatus::FERR) {
            Err(FrameError)
        } else if errors.contains(RxStatus::BUFOVF) {
            Err(Overflow)
        } else if errors.contains(RxStatus::ISF) {
            Err(InconsistentSyncField)
        } else if errors.contains(RxStatus::COLL) {
            Err(CollisionDetected)
        } else {
            Ok(())
        }
    }
}

impl From<RxError> for RxStatus {
    #[inline]
    fn from(err: RxError) -> Self {
        use RxError::*;
        match err {
            ParityError => RxStatus::PERR,
            FrameError => RxStatus::FERR,
            Overflow => RxStatus::BUFOVF,
            InconsistentSyncField => RxStatus::ISF,
            CollisionDetected => RxStatus::COLL,
        }
    }
}

//=============================================================================
// Config
//=============================================================================

/// A configurable, disabled UART peripheral
///
/// This `struct` represents a configurable UART peripheral in its disabled
/// state. It is generic over the set of [`Pads`] and [`CharSize`].
/// Upon creation, the [`Config`] takes ownership of the
/// [`Sercom`] and resets it, returning it configured as an UART peripheral
/// with a default configuration:
///
/// * [`EightBit`]
/// * No parity
/// * One stop bit
/// * LSB-first
///
/// [`Config`] uses a builder-pattern API to configure the peripheral,
/// culminating in a call to [`enable`], which consumes the [`Config`] and
/// returns enabled [`UartRx`], [`UartTx`], or both. The [`enable`] method is
/// restricted to [`ValidConfig`]s.
///
/// [`enable`]: Enable::enable
pub struct Config<P, C = EightBit>
where
    P: ValidPads,
    C: CharSize,
{
    sercom: P::Sercom,
    pads: P,
    chsize: PhantomData<C>,
    freq: Hertz,
}

impl<P: ValidPads> Config<P> {
    /// Create a new [`Config`] in the default configuration
    #[inline]
    fn create(sercom: P::Sercom, pads: P, freq: impl Into<Hertz>) -> Self {
        Self::swrst(&sercom);
        P::configure(&sercom);
        EightBit::configure(&sercom);

        // Enable internal clock mode
        sercom
            .usart()
            .ctrla
            .modify(|_, w| w.mode().variant(MODE_A::USART_INT_CLK));

        Self {
            sercom,
            pads,
            chsize: PhantomData,
            freq: freq.into(),
        }
    }

    /// Create a new [`Config`] in the default configuration
    ///
    /// This function will enable the corresponding APB clock, reset the
    /// [`Sercom`] peripheral, and return a [`Config`] in the default
    /// configuration:
    ///
    /// * [`EightBit`] [`CharSize`]
    /// * No parity
    /// * One stop bit
    /// * LSB-first
    ///
    /// [`Config`] takes ownership of the [`Sercom`] and [`Pads`].
    ///
    /// Users must configure GCLK manually. The `freq` parameter represents the
    /// GCLK frequency for this [`Sercom`] instance.
    #[inline]
    pub fn new(pm: &PM, mut sercom: P::Sercom, pads: P, freq: impl Into<Hertz>) -> Self {
        sercom.enable_apb_clock(pm);

        // Use LSB-first frames. This is the standard for UART transfers. Not sure why
        // the default is MSB-first in the hardware.
        Self::create(sercom, pads, freq).msb_first(false)
    }
}

impl<P, C> Config<P, C>
where
    P: ValidPads,
    C: CharSize,
{
    /// Reset the SERCOM peripheral
    #[inline]
    fn swrst(sercom: &P::Sercom) {
        sercom.usart().ctrla.write(|w| w.swrst().set_bit());
        while sercom.usart().syncbusy.read().swrst().bit_is_set() {}
    }

    /// Change the [`Config`] [`CharSize`]
    #[inline]
    fn change<C2>(self) -> Config<P, C2>
    where
        C2: CharSize,
    {
        Config {
            sercom: self.sercom,
            pads: self.pads,
            chsize: PhantomData,
            freq: self.freq,
        }
    }

    /// Trigger the [`Sercom`]'s SWRST and return a [`Config`] in the
    /// default configuration.
    #[inline]
    pub fn reset(self) -> Config<P> {
        Config::create(self.sercom, self.pads, self.freq)
    }

    /// Obtain a reference to the PAC `SERCOM` struct
    ///
    /// Directly accessing the `SERCOM` could break the invariants of the
    /// type-level tracking in this module, so it is unsafe.
    #[inline]
    pub unsafe fn sercom(&self) -> &P::Sercom {
        &self.sercom
    }

    /// Consume the [`Config`], reset the peripheral, and return the [`Sercom`]
    /// and [`Pads`]
    #[inline]
    pub fn free(self) -> (P::Sercom, P) {
        Self::swrst(&self.sercom);
        (self.sercom, self.pads)
    }

    /// Change the [`CharSize`]
    #[inline]
    pub fn char_size<C2: CharSize>(self) -> Config<P, C2> {
        C2::configure(&self.sercom);
        self.change()
    }

    /// Change the bit order of transmission (MSB/LSB first)
    #[inline]
    pub fn msb_first(self, msb_first: bool) -> Self {
        self.sercom
            .usart()
            .ctrla
            .modify(|_, w| w.dord().bit(!msb_first));
        self
    }

    /// Change the parity setting
    #[inline]
    pub fn parity(self, parity: Option<Parity>) -> Self {
        // Use only the first two available settings in the FORM field.
        // Ignore auto-baud options.
        let enabled = if let Some(p) = parity {
            let odd = match p {
                Parity::Even => false,
                Parity::Odd => true,
            };
            self.sercom.usart().ctrlb.modify(|_, w| w.pmode().bit(odd));
            true
        } else {
            false
        };

        self.sercom
            .usart()
            .ctrla
            .modify(|_, w| unsafe { w.form().bits(!enabled as u8) });
        self
    }

    /// Change the stop bit setting
    #[inline]
    pub fn stop_bit(self, stop_bit: StopBits) -> Self {
        let two_bits = match stop_bit {
            StopBits::OneBit => false,
            StopBits::TwoBits => true,
        };

        self.sercom
            .usart()
            .ctrlb
            .modify(|_, w| w.sbmode().bit(two_bits));
        self
    }

    /// Enable or disable the start of frame detector.
    ///
    /// When set, the UART will generate interrupts for
    /// RXC and/or RXS if these interrupt flags have been enabled.
    #[inline]
    pub fn start_of_frame_detection(self, enabled: bool) -> Self {
        self.sercom
            .usart()
            .ctrlb
            .modify(|_, w| w.sfde().bit(enabled));
        self
    }

    /// Enable or disable the collision detector.
    ///
    /// When set, the UART will detect collisions and update the
    /// corresponding flag in the STATUS register.
    #[inline]
    pub fn collision_detection(self, enabled: bool) -> Self {
        self.sercom
            .usart()
            .ctrlb
            .modify(|_, w| w.colden().bit(enabled));
        self
    }

    /// Set the baud rate
    ///
    /// This function will calculate the best BAUD register setting based on the
    /// stored GCLK frequency and desired baud rate. The maximum baud rate is
    /// GCLK frequency/oversampling. Values outside this range will saturate at
    /// the maximum supported baud rate.
    ///
    /// Note that 3x oversampling is not supported.
    #[inline]
    pub fn baud<B: Into<Hertz>>(self, baud: B, mode: BaudMode) -> Self {
        let baud: Hertz = baud.into();
        let usart = self.sercom.usart();

        usart
            .ctrla
            .modify(|_, w| unsafe { w.sampr().bits(mode.sampr()) });

        match mode {
            BaudMode::Arithmetic(n) => {
                let baud = Self::calculate_baud_asynchronous_arithm(baud.0, self.freq.0, n as u8);
                unsafe { usart.baud_usartfp_mode().write(|w| w.baud().bits(baud)) };
            }

            BaudMode::Fractional(n) => {
                let (baud, frac) =
                    Self::calculate_baud_asynchronous_fractional(baud.0, self.freq.0, n as u8);
                unsafe {
                    usart.baud_frac_mode().write(|w| {
                        w.fp().bits(frac);
                        w.baud().bits(baud)
                    });
                }
            }
        };

        self
    }

    /// Control the buffer overflow notification
    ///
    /// If set to true, an [`RxError::Overflow`] will be issued as soon as an
    /// overflow occurs. Otherwise, it will not be issued until its place within
    /// the data stream.
    #[inline]
    pub fn immediate_overflow_notification(self, set: bool) -> Self {
        self.sercom.usart().ctrla.modify(|_, w| w.ibon().bit(set));
        self
    }

    /// Run in standby mode
    ///
    /// When set, the UART peripheral will run in standby mode. See the
    /// datasheet for more details.
    #[inline]
    pub fn run_in_standby(self, set: bool) -> Self {
        self.sercom
            .usart()
            .ctrla
            .modify(|_, w| w.runstdby().bit(set));
        self
    }

    /// Enable or disable IrDA encoding. The pulse length controls the minimum
    /// pulse length that is required for a pulse to be accepted by the IrDA
    /// receiver with regards to the serial engine clock period.
    /// See datasheet for more information.
    pub fn irda_encoding(self, pulse_length: Option<u8>) -> Self {
        match pulse_length {
            Some(l) => {
                self.sercom
                    .usart()
                    .rxpl
                    .write(|w| unsafe { w.rxpl().bits(l) });
                self.sercom.usart().ctrlb.modify(|_, w| w.enc().bit(true));
            }
            None => {
                self.sercom.usart().ctrlb.modify(|_, w| w.enc().bit(false));
            }
        }
        self
    }

    /// Enable the UART peripheral
    ///
    /// UART transactions are not possible until the peripheral is enabled.
    /// This function is limited to [`ValidConfig`]s.
    #[inline]
    pub(super) fn _enable(&mut self)
    where
        Self: ValidConfig,
    {
        // Enable RX
        self.sercom.usart().ctrlb.modify(|_, w| w.rxen().set_bit());
        while self.sercom.usart().syncbusy.read().ctrlb().bit_is_set() {}

        // Enable TX
        self.sercom.usart().ctrlb.modify(|_, w| w.txen().set_bit());
        while self.sercom.usart().syncbusy.read().ctrlb().bit_is_set() {}

        // Globally enable peripheral
        self.enable_peripheral(true);
    }

    /// Enable or disable the SERCOM peripheral, and wait for the ENABLE bit to
    /// synchronize.
    fn enable_peripheral(&mut self, enable: bool) {
        self.sercom
            .usart()
            .ctrla
            .modify(|_, w| w.enable().bit(enable));
        while self.sercom.usart().syncbusy.read().enable().bit_is_set() {}
    }

    /// Calculate baudrate value using the asynchronous arithmetic method (Table
    /// 24-2)
    #[inline]
    fn calculate_baud_asynchronous_arithm(baudrate: u32, clk_freq: u32, n_samples: u8) -> u16 {
        const SHIFT: u8 = 32;
        let sample_rate = (n_samples as u64 * baudrate as u64) << SHIFT;
        let ratio = sample_rate / clk_freq as u64;
        let scale = (1u64 << SHIFT) - ratio;
        let baud_calculated = (65536u64 * scale) >> SHIFT;

        baud_calculated as u16
    }

    /// Calculate baudrate value using the asynchronous frational method (Table
    /// 24-2)
    #[inline]
    fn calculate_baud_asynchronous_fractional(
        baudrate: u32,
        clk_freq: u32,
        n_samples: u8,
    ) -> (u16, u8) {
        let baud_mult = (clk_freq * 8) / (n_samples as u32 * baudrate);
        ((baud_mult / 8) as u16, (baud_mult % 8) as u8)
    }
}

//=============================================================================
// AnyConfig
//=============================================================================

/// Type class for all possible [`Config`] types
///
/// This trait uses the [`AnyKind`] trait pattern to create a [type class] for
/// [`Config`] types. See the `AnyKind` documentation for more details on the
/// pattern.
///
/// In addition to the normal, `AnyKind` associated types. This trait also
/// copies the [`Sercom`] and [`Word`] types, to make it easier to apply
/// bounds to these types at the next level of abstraction.
///
/// [`AnyKind`]: crate::typelevel#anykind-trait-patter
pub trait AnyConfig: Sealed + Is<Type = SpecificConfig<Self>> {
    type Sercom: Sercom;
    type Pads: ValidPads<Sercom = Self::Sercom>;
    type Word: 'static;
    type CharSize: CharSize<Word = Self::Word>;
}

/// Type alias to recover the specific [`Config`] type from an implementation of
/// [`AnyConfig`]
pub type SpecificConfig<C> = Config<<C as AnyConfig>::Pads, <C as AnyConfig>::CharSize>;

/// Type alias to recover the specific [`Sercom`] type from an implementation of
/// [`AnyConfig`]
pub type ConfigSercom<C> = <C as AnyConfig>::Sercom;

impl<P, C> AsRef<Self> for Config<P, C>
where
    P: ValidPads,
    C: CharSize,
{
    #[inline]
    fn as_ref(&self) -> &Self {
        self
    }
}

impl<P, C> AsMut<Self> for Config<P, C>
where
    P: ValidPads,
    C: CharSize,
{
    #[inline]
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl<P, C> Sealed for Config<P, C>
where
    P: ValidPads,
    C: CharSize,
{
}

impl<P, C> AnyConfig for Config<P, C>
where
    P: ValidPads,
    C: CharSize,
{
    type Sercom = P::Sercom;
    type Word = C::Word;
    type Pads = P;
    type CharSize = C;
}

//=============================================================================
// ValidConfig
//=============================================================================

/// Marker trait for valid UART [`Config`]urations
///
/// A functional UART peripheral must have, at a minimum either a Rx or a Tx
/// [`Pad`].
pub trait ValidConfig: AnyConfig {}

impl<P, C> ValidConfig for Config<P, C>
where
    P: ValidPads + TxOrRx,
    C: CharSize,
{
}

/// Methods that are available to both [`UartRx`] and [`UartTx`] structs
pub trait Uart: Sealed {
    type Sercom: Sercom;
    type Flags: Flags;
    type Status: Status;

    /// Obtain a reference to the PAC `SERCOM` struct
    ///
    /// # Safety
    ///
    /// Directly accessing the `SERCOM` could break the invariants of the
    /// type-level tracking in this module, so it is unsafe.
    unsafe fn sercom(&self) -> &Self::Sercom;

    /// Read the interrupt flags
    fn read_flags(&self) -> Self::Flags {
        let bits = unsafe { self.sercom().usart().intflag.read().bits() };
        Self::Flags::from_bits_truncate(bits)
    }

    /// Clear interrupt status flags
    ///
    /// Setting the ERROR, RXBRK, CTSIC, RXS, or TXC flag will clear the
    /// interrupts. This function has no effect on the DRE or
    /// RXC flags.
    ///
    /// **Warning:** The implementation of of [`Write::flush`] waits on and
    /// clears the `TXC` flag. Manually clearing this flag could cause it to
    /// hang indefinitely.
    fn clear_flags(&mut self, flags: Self::Flags) {
        unsafe {
            self.sercom()
                .usart()
                .intflag
                .modify(|_, w| w.bits(flags.into_bits()))
        }
    }

    /// Enable interrupts for the specified flags
    #[inline]
    fn enable_interrupts(&mut self, flags: Self::Flags) {
        unsafe {
            self.sercom()
                .usart()
                .intenset
                .write(|w| w.bits(flags.into_bits()))
        }
    }

    /// Disable interrupts for the specified flags
    #[inline]
    fn disable_interrupts(&mut self, flags: Self::Flags) {
        unsafe {
            self.sercom()
                .usart()
                .intenclr
                .write(|w| w.bits(flags.into_bits()))
        };
    }

    /// Read the status flags
    #[inline]
    fn read_status(&self) -> Self::Status {
        let bits = unsafe { self.sercom().usart().status.read().bits() };
        Self::Status::from_bits_truncate(bits)
    }

    /// Clear the status flags
    #[inline]
    fn clear_status(&mut self, status: Self::Status) {
        unsafe {
            self.sercom()
                .usart()
                .status
                .modify(|_, w| w.bits(status.into_bits()))
        }
    }
}

/// Enable a [`Config`] and split it into its `Rx` and/or `Tx` parts
pub trait Enable<C>
where
    Self: ValidConfig,
{
    /// Enable a [`Config`] and return its consitituting [`UartRx`] and/or
    /// [`UartTx`]
    fn enable(self) -> C;
}

/// Disable or reconfigure [`UartRx`] and/or [`UartTx`] and join the parts back
/// into a [`Config`].
///
/// Some methods require both the constituting [`UartRx`] and [`UartTx`] parts
/// (if they exist) to operate. This trait is necessary to implement the
/// [`disable`] and [`reconfigure`] method
///
/// [`disable`]: Disable::disable
/// [`reconfigure`]: Disable::reconfigure
pub trait Disable<C>
where
    C: ValidConfig,
    Self: Sized,
{
    /// Disable back the parts into `Self`
    ///
    /// # Safety
    ///
    /// Calling this method may violate the typelevel invariants laid out by
    /// this module. It is therefore unsafe.
    unsafe fn join(self) -> C;

    /// Borrow the underlying [`Config`] struct
    ///
    /// # Safety
    ///
    /// Calling this method may violate the typelevel invariants laid out by
    /// this module. It is therefore unsafe.
    unsafe fn borrow_mut(&mut self) -> &mut C;

    /// Update the UART configuration.
    ///
    /// Calling this method will temporarily disable the SERCOM peripheral, as
    /// some registers are enable-protected. This may interrupt any ongoing
    /// transactions.
    ///
    /// ```
    /// use atsamd_hal::sercom::v2::uart::{BaudMode, Oversampling, Uart};
    /// // Asume rx is an UartRx and tx is an UartTx
    /// (rx, tx).reconfigure(|c| c.baud(9600, BaudMode::Fractional(Oversampling::Bits16)));
    /// ```
    #[inline]
    fn reconfigure<F>(&mut self, update: F)
    where
        F: FnOnce(SpecificConfig<C>) -> SpecificConfig<C>,
    {
        let config = unsafe { self.borrow_mut().as_mut() };
        config.enable_peripheral(false);

        // Perform a bitwise copy of the old configuration. This will be used as default
        // in case the call to update(self.config) panics. This should be safe
        // as either one of self.config or old_config will be used, and Config
        // does not deallocate when dropped.
        let old_config = unsafe { core::ptr::read(config) };
        replace_with::replace_with(config, || old_config, |c| update(c.into()).into());

        config.enable_peripheral(true);
    }

    /// Disable the UART peripheral and return the underlying [`Config`]
    ///
    /// ```
    /// use atsamd_hal::sercom::v2::uart::{Uart};
    /// // Asume rx is an UartRx and tx is an UartTx
    /// let config = (rx, tx).disable();
    /// ```
    fn disable(self) -> C {
        let mut config = unsafe { Self::join(self) };
        let usart = config.as_mut().sercom.usart();

        usart.ctrlb.modify(|_, w| w.rxen().clear_bit());
        while usart.syncbusy.read().ctrlb().bit_is_set() {}
        config.as_mut().enable_peripheral(false);

        config
    }
}

impl<C> Enable<(UartRx<C>, UartTx<C>)> for C
where
    C: ValidConfig,
    C::Pads: Rx + Tx + TxOrRx,
{
    #[inline]
    fn enable(mut self) -> (UartRx<C>, UartTx<C>) {
        self.as_mut()._enable();
        let config_copy = unsafe { core::ptr::read(self.as_ref()) }.into();
        (
            UartRx { config: self },
            UartTx {
                config: config_copy,
            },
        )
    }
}

impl<C> Disable<C> for (UartRx<C>, UartTx<C>)
where
    C: ValidConfig,
    C::Pads: Tx + Rx,
{
    #[inline]
    unsafe fn join(self) -> C {
        self.0.config
    }

    #[inline]
    unsafe fn borrow_mut(&mut self) -> &mut C {
        &mut self.0.config
    }
}

impl<C> Enable<UartRx<C>> for C
where
    C: ValidConfig,
    C::Pads: Rx + NotTx + TxOrRx,
{
    #[inline]
    fn enable(mut self) -> UartRx<C> {
        self.as_mut()._enable();
        UartRx { config: self }
    }
}

impl<C> Disable<C> for UartRx<C>
where
    C: ValidConfig,
    C::Pads: Rx + NotTx,
{
    #[inline]
    unsafe fn join(self) -> C {
        self.config
    }

    #[inline]
    unsafe fn borrow_mut(&mut self) -> &mut C {
        &mut self.config
    }
}

impl<C> Enable<UartTx<C>> for C
where
    C: ValidConfig,
    C::Pads: Tx + NotRx + TxOrRx,
{
    #[inline]
    fn enable(mut self) -> UartTx<C> {
        self.as_mut()._enable();
        UartTx { config: self }
    }
}

impl<C> Disable<C> for UartTx<C>
where
    C: ValidConfig,
    C::Pads: Tx + NotRx,
{
    #[inline]
    unsafe fn join(self) -> C {
        self.config
    }

    #[inline]
    unsafe fn borrow_mut(&mut self) -> &mut C {
        &mut self.config
    }
}

//=============================================================================
// Rx/Tx halves
//=============================================================================

/// Receive half of a UART peripheral
pub struct UartRx<C: ValidConfig> {
    config: C,
}

impl<C> UartRx<C>
where
    C: ValidConfig,
    C::Word: PrimInt,
    u16: AsPrimitive<C::Word>,
    C::Pads: Rx,
{
    /// Read from the DATA register
    ///
    /// Reading from the data register directly is `unsafe`, because it will
    /// clear the RXC flag, which could break assumptions made elsewhere in
    /// this module.
    #[inline]
    pub unsafe fn read_data(&mut self) -> u16 {
        self.sercom().usart().data.read().data().bits()
    }

    /// Clear error status flags
    #[inline]
    pub fn clear_errors(&mut self, errors: RxStatus) {
        self.clear_status(errors);
    }

    /// Read the status register and convert into a [`Result`]
    /// containing the corresponding [`RxFlags`] or [`RxError`]
    #[inline]
    fn read_flags_errors(&self) -> Result<RxFlags, RxError> {
        self.read_status().try_into()?;
        Ok(self.read_flags())
    }

    /// Flush the RX buffer and clear errors
    #[inline]
    pub fn flush(&mut self) {
        let usart = unsafe { self.sercom().usart() };

        // TODO
        // The datasheet states that disabling the receiver (RXEN) clears
        // the RX buffer, and clears the BUFOVF, PERR and FERR bits.
        // However, in practice, it seems like BUFOVF errors still pop
        // up after a disable/enable cycle of the receiver, then immediately begin
        // reading bytes from the DATA register.
        //
        // Is this a hardware bug???
        /*
        usart.ctrlb.modify(|_, w| w.rxen().clear_bit());
        while usart.syncbusy.read().ctrlb().bit() || usart.ctrlb.read().rxen().bit_is_set() {}

        usart.ctrlb.modify(|_, w| w.rxen().set_bit());
        while usart.syncbusy.read().ctrlb().bit() || usart.ctrlb.read().rxen().bit_is_clear() {}
        */

        // Workaround is to read a few bytes to clear the RX buffer (3 bytes seems to do
        // the trick), then manually clear the BUFOVF bit Clear rx buffer
        for _ in 0..=2 {
            let _data = usart.data.read();
        }

        // Clear all errors
        self.clear_errors(
            RxStatus::BUFOVF | RxStatus::FERR | RxStatus::PERR | RxStatus::ISF | RxStatus::COLL,
        );
    }
}

impl<C> Uart for UartRx<C>
where
    C: ValidConfig,
{
    type Sercom = C::Sercom;
    type Flags = RxFlags;
    type Status = RxStatus;

    unsafe fn sercom(&self) -> &Self::Sercom {
        &self.config.as_ref().sercom
    }
}
impl<C: ValidConfig> Sealed for UartRx<C> {}

/// Transmit half of a split UART
pub struct UartTx<C: ValidConfig> {
    config: C,
}

impl<C> UartTx<C>
where
    C: ValidConfig,
    C::Pads: Tx,
{
    /// Write to the DATA register
    ///
    /// Writing to the data register directly is `unsafe`, because it will clear
    /// the DRE flag, which could break assumptions made elsewhere in this
    /// module.
    #[inline]
    pub unsafe fn write_data(&mut self, data: u16) {
        self.sercom().usart().data.write(|w| w.data().bits(data))
    }
}

impl<C: ValidConfig> Uart for UartTx<C> {
    type Sercom = C::Sercom;
    type Flags = TxFlags;
    type Status = TxStatus;

    unsafe fn sercom(&self) -> &Self::Sercom {
        &self.config.as_ref().sercom
    }
}
impl<C: ValidConfig> Sealed for UartTx<C> {}

//=============================================================================
// Embedded HAL traits
//=============================================================================

/// Implement [`Read`] for [`UartRx`].
impl<C> Read<C::Word> for UartRx<C>
where
    C: ValidConfig,
    C::Word: PrimInt,
    C::Pads: Rx,
    u16: AsPrimitive<C::Word>,
{
    type Error = RxError;

    /// Wait for an `RXC` flag, then read the word
    #[inline]
    fn read(&mut self) -> nb::Result<C::Word, RxError> {
        let flags = self.read_flags_errors()?;
        if flags.contains(RxFlags::RXC) {
            unsafe { Ok(self.read_data().as_()) }
        } else {
            Err(WouldBlock)
        }
    }
}

/// Implement [`Write`] for [`UartTx`].
impl<C> Write<C::Word> for UartTx<C>
where
    C: ValidConfig,
    C::Pads: Tx,
    C::Word: PrimInt + AsPrimitive<u16>,
{
    type Error = core::convert::Infallible;

    /// Wait for a `DRE` flag, then write a word
    #[inline]
    fn write(&mut self, word: C::Word) -> nb::Result<(), Self::Error> {
        // Ignore buffer overflow errors
        if self.read_flags().contains(TxFlags::DRE) {
            unsafe { self.write_data(word.as_()) };
            Ok(())
        } else {
            Err(WouldBlock)
        }
    }

    /// Wait for a `TXC` flag
    #[inline]
    fn flush(&mut self) -> nb::Result<(), Self::Error> {
        // Ignore buffer overflow errors
        if self.read_flags().contains(TxFlags::TXC) {
            Ok(())
        } else {
            Err(WouldBlock)
        }
    }
}

impl<C> blocking::serial::write::Default<C::Word> for UartTx<C>
where
    C: ValidConfig,
    C::Pads: Tx,
    UartTx<C>: Write<C::Word>,
{
}

/// Number of stop bits in a UART frame
pub enum StopBits {
    /// 1 stop bit
    OneBit = 0,
    /// 2 stop bits
    TwoBits = 1,
}

/// Parity setting of a UART frame
pub enum Parity {
    /// Even parity
    Even = 0,
    /// Odd parity
    Odd = 1,
}

/// Baudrate oversampling values
///
/// *NOTE* 3x oversampling has been intentionally left out
pub enum Oversampling {
    // 3 samples per bit
    // Bits3 = 3,
    /// 8 samples per bit
    Bits8 = 8,
    /// 16 samples per bit
    Bits16 = 16,
}

/// Baudrate calculation in asynchronous mode
pub enum BaudMode {
    Arithmetic(Oversampling),
    Fractional(Oversampling),
}

impl BaudMode {
    pub(super) fn sampr(&self) -> u8 {
        use BaudMode::*;
        use Oversampling::*;
        match self {
            Arithmetic(n) => match n {
                Bits16 => 0,
                Bits8 => 2,
            },

            Fractional(n) => match n {
                Bits16 => 1,
                Bits8 => 3,
            },
        }
    }
}
