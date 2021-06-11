//! Use the SERCOM peripheral for UART communications
//!
//! Configuring an UART peripheral occurs in three steps. First, you must create
//! a set of [`Pads`] for use by the peripheral. Next, you assemble pieces into
//! a [`Config`] struct. After configuring the peripheral, you then [`enable`]
//! it, yielding a functional [`Uart`] struct. Transactions are performed using
//! the [`serial`](embedded_hal::serial) traits
//! from embedded HAL.
//!
//! # [`Pads`]
//!
//! A [`Sercom`] can use up to four [`Pin`]s as peripheral [`Pad`]s, but only
//! certain `Pin` combinations are acceptable. In particular, all `Pin`s must be
//! mapped to the same `Sercom` and [`IoSet`] (see section 6.2.8.1 of the
//! datasheet). This HAL makes it impossible to use invalid `Pin`/`Pad`
//! combinations, and the [`Pads`] struct is responsible for enforcing these
//! constraints.
//!
//! A `Pads` type takes up to siz type parameters. The two specifiy the
//! `Sercom` and `IoSet`. The remaining four, `RX`, `TX`, `RTS` and `CTS`,
//! are effectively [`OptionalPinId`]s for the `RX`, `TX`, `RTS` and `CTS`
//! `Pad`s respectively, and each defaults to [`NoneT`]. To be accepted as part
//! of a [`ValidConfig`], you must specify a `PinId` for at least one of
//! `RX` or `TX`.
//!
//! ```
//! use atsamd_hal::gpio::v2::{PA04, PA05, PA08, PA09};
//! use atsamd_hal::sercom::v2::{Sercom0, uart};
//! use atsamd_hal::sercom::v2::pad::{Pad0, Pad1};
//! use atsamd_hal::typelevel::NoneT;
//!
//! // For SAMD21 chips
//! type Pads = uart::Pads<Sercom0, PA08, NoneT, PA09>;
//! ```
//!
//! `Pads` are created using the builder pattern. Start by creating an empty set
//! of `Pads` using [`Default`]. Then pass each respective `Pin` using the
//! corresponding methods. Both `v1::Pin` and `v2::Pin` types are accepted here.
//! Note that the `TX` `Pin` must map to [`Pad0`], and if specified, the `RTS`
//! `Pin` must map to [`Pad2`], and the `CTS` `Pin` must map to [`Pad3`].
//! The `RX` `Pin` can vary in [`PadNum`] based on the [`Rxpo`] values
//!
//! To be accepted as part of a [`ValidConfig`], a set of `Pads` must do two
//! things: specify a type for at least one of `RX` or `TX`; and
//! satisfy the [`Rxpo`] + [`Txpo`] traits.
//!
//! ```
//! use atsamd_hal::target_device::Peripherals;
//! use atsamd_hal::gpio::v2::Pins;
//! use atsamd_hal::sercom::v2::{Sercom0, uart};
//!
//! let mut peripherals = Peripherals::take().unwrap();
//! let pins = Pins::new(peripherals.PORT);
//! let pads = uart::Pads::<Sercom0>::default()
//!     .rx(pins.pa08)
//!     .tx(pins.pa10);
//! ```
//!
//! # [`Config`]
//!
//! Next, create a [`Config`] struct, which represents the UART peripheral in
//! its disabled state. A `Config` is specified with three type parameters: the
//! [`Pads`] type; an [`OpMode`], which defaults to [`Master`]; and a
//! [`CharSize`], which defaults to [`EightBit`].
//!
//! ```
//! use atsamd_hal::gpio::v2::{PA08, PA09};
//! use atsamd_hal::sercom::v2::{Sercom0, uart};
//! use atsamd_hal::sercom::v2::uart::{Master, NineBit};
//! use atsamd_hal::typelevel::NoneT;
//!
//! // Assuming SAMD21
//! type Pads = uart::Pads<Sercom0, PA08, NoneT, PA09>;
//! type Config = uart::Config<Pads, NineBit>;
//! ```
//!
//! Upon creation, the [`Config`] takes ownership of both the [`Pads`] and the
//! PAC [`Sercom`] struct. It takes a reference to the PM, so that it can
//! enable the APB clock, and it takes a frequency to indicate the GCLK
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
//! an enabled [`Uart`] peripheral.
//!
//! ```
//! use atsamd_hal::sercom::v2::uart::{StopBits, NineBit};
//!
//! let uart = uart::Config::new(&mclk, sercom, pads, freq)
//!     .baud(1.mhz())
//!     .char_size::<NineBit>()
//!     .msb_first(false)
//!     .stop_bits(StopBits::TwoBits)
//!     .enable();
//! ```
//!
//! # [`Uart`]
//!
//! An [`Uart`] struct can only be created from a [`Config`], and it has only
//! one type parameter, the corresponding config.
//!
//! ```
//! use atsamd_hal::gpio::v2::{PA08, PA09};
//! use atsamd_hal::sercom::v2::{Sercom0, uart};
//! use atsamd_hal::sercom::v2::uart::{Master, NineBit};
//! use atsamd_hal::typelevel::NoneT;
//!
//! // Assuming SAMD21
//! type Pads = uart::Pads<Sercom0, PA08, NoneT, PA09>;
//! type Config = uart::Config<Pads, NineBit>;
//! type Uart = uart::Uart<Config>;
//! ```
//!
//! Only the [`Uart`] struct can actually perform transactions. To do so, use
//! the embedded HAL traits, like [`serial::Read`](Read) and
//! [`serial::Write`](Write).
//!
//! ```
//! use nb::block;
//! use embedded_hal::serial::Write;
//!
//! block!(uart.write(0x0fe));
//! ```
//!
//! # Splitting and joining
//!
//! A fully configured [`Uart`] struct can be split into `Tx` and `Rx` halves.
//! That way, different parts of the program can individually send or receive
//! UART transactions.
//!
//! ## Splitting
//!
//! The `tx` and `rx` fields can be moved out of a [`Uart`] struct. They can
//! then be used to individually send/receive data over the UART port, perhaps
//! in different contexts.
//!
//! ```
//! use nb::block;
//! use embedded_hal::serial::{Read, Write};
//!
//! // Assume uart is a fully configured `Uart` with transmit/receive capability
//! let rx = uart.rx;
//! let tx = uart.tx;
//!
//! block!(tx.write(0xfe));
//! let _received = block!(rx.read());
//! ```
//!
//! ## Joining
//!
//! Moving the [`UartRx`] and [`UartTx`] halves back into the [`Uart`]
//! struct is necessary if the UART peripheral should call methods taking `&mut
//! self`, such as [`reconfigure`](Uart::reconfigure).
//!
//! ```
//! // Assume uart is a fully configured `Uart` with transmit/receive capability
//! let rx = uart.rx;
//! let tx = uart.tx
//!
//! // Send/receive data with tx/rx...
//!
//! uart.tx = tx;
//! uart.rx = rx;
//! uart.reconfigure(|c| c.baud(1.mhz()));
//! ```
//!
//! # Using UART with DMA
//!
//! This HAL includes support for DMA-enabled UART transfers. An enabled `Uart`
//! struct contains `rx` and `tx` fields, which both implement the DMAC
//! [`Buffer`](crate::dmac::transfer::Buffer) trait. The provided
//! [`send_with_dma`](UartTx::send_with_dma) and
//! [`receive_with_dma`](UartRx::receive_with_dma) build and begin a
//! [`dmac::Transfer`](crate::dmac::Transfer), thus starting the UART in a
//! non-blocking way. Optionally, interrupts can be enabled on the provided
//! [`Channel`](crate::dmac::channel::Channel). Note that the `dma` feature must
//! be enabled. Please refer to the [`dmac`](crate::dmac) module-level
//! documentation for more information.
//!
//! ```
//! // Assume channel0 and channel1 are configured `dmac::Channel`s, and uart a
//! // fully-configured `Uart`
//!
//! // Create data to send
//! let tx_buffer: [u8; 50] = [0xff; 50];
//! let rx_buffer: [u8; 100] = [0xab; 100];
//!
//! // Launch transmit transfer
//! let tx_dma = uart.tx.send_with_dma(&mut tx_buffer, channel0, ());
//!
//! // Launch receive transfer
//! let rx_dma = uart.rx.receive_with_dma(&mut rx_buffer, channel1, ());
//!
//! // Wait for transfers to complete and reclaim resources
//! let (chan0, _, tx, _) = tx_dma.wait();
//! let (chan1, _, rx, _) = rx_dma.wait();
//!
//! // Optionally join `rx` and `tx` back into `uart`
//! uart.tx = tx;
//! uart.rx = rx;
//! ```
//!
//! # Non-supported advanced features
//!
//! * 32-bit extension mode support is explicitely omitted to allow for
//!   different character sizes. If large transfers are to be made, it is
//!   recommended to use DMA transfers.
//! * LIN mode is not supported
//! * Synchronous mode (USART) is not supported
//!
//! [`enable`]: Config::enable
//! [`Pin`]: crate::gpio::v2::pin::Pin
//! [`PinId`]: crate::gpio::v2::pin::PinId
//! [`OptionalPinId`]: crate::gpio::v2::pin::OptionalPinId

use core::convert::{TryFrom, TryInto};
use core::marker::PhantomData;

use crate::target_device as pac;
use crate::time::Hertz;
use pac::{
    sercom0::{
        usart_int::ctrla::{MODE_A, RXPO_A, TXPO_A},
        RegisterBlock,
    },
    MCLK,
};

use crate::gpio::v2::AnyPin;
use crate::sercom::v2::*;
use crate::typelevel::{Is, NoneT, Sealed};

use bitflags::bitflags;
use embedded_hal::blocking;
use embedded_hal::serial::{Read, Write};
use nb::Error::WouldBlock;
use num_traits::{AsPrimitive, PrimInt};

//=============================================================================
// Rxpo
//=============================================================================

/// Control the `RXPO` field as a function of the [`PadNum`] type
pub trait Rxpo: Sealed {
    /// Corresponding variant from the PAC `enum`
    const RXPO: RXPO_A;

    /// Configure the pad according to [`Self::RXPO`]
    #[inline]
    fn configure(sercom: &RegisterBlock) {
        sercom
            .usart_int()
            .ctrla
            .modify(|_, w| w.rxpo().variant(Self::RXPO));
    }
}

impl Rxpo for Pad0 {
    const RXPO: RXPO_A = RXPO_A::PAD0;
}
impl Rxpo for Pad1 {
    const RXPO: RXPO_A = RXPO_A::PAD1;
}
impl Rxpo for Pad2 {
    const RXPO: RXPO_A = RXPO_A::PAD2;
}
impl Rxpo for Pad3 {
    const RXPO: RXPO_A = RXPO_A::PAD3;
}

impl Rxpo for NoneT {
    /// This value is arbitrary and meaningless for [`NoneT`]
    const RXPO: RXPO_A = RXPO_A::PAD0;

    /// Override the default implementation to do nothing
    fn configure(_: &RegisterBlock) {}
}

/// Lift the implementations of [`Rxpo`] from [`OptionalPadNum`]s to the
/// corresponding [`Pads`] types.
impl<S, I, RX, TX, RTS, CTS> Rxpo for Pads<S, I, RX, TX, RTS, CTS>
where
    S: Sercom,
    I: IoSet,
    RX: GetOptionalPad<S>,
    TX: GetOptionalPad<S>,
    RTS: GetOptionalPad<S>,
    CTS: GetOptionalPad<S>,
    RX::PadNum: Rxpo,
{
    const RXPO: RXPO_A = RX::PadNum::RXPO;
}

//=============================================================================
// Txpo
//=============================================================================

/// Control the `TXPO` field as a function of the [`PadNum`] type
pub trait Txpo: Sealed {
    /// Corresponding variant from the PAC `enum`
    const TXPO: TXPO_A;

    /// Configure the pad according to [`Self::TXPO`]
    #[inline]
    fn configure(sercom: &RegisterBlock) {
        sercom
            .usart_int()
            .ctrla
            .modify(|_, w| w.txpo().variant(Self::TXPO));
    }
}

impl Txpo for Pad0 {
    const TXPO: TXPO_A = TXPO_A::PAD0;
}
impl Txpo for Pad3 {
    const TXPO: TXPO_A = TXPO_A::PAD3;
}

impl Txpo for NoneT {
    /// This value is arbitrary and meaningless for [`NoneT`]
    const TXPO: TXPO_A = TXPO_A::PAD0;

    /// Override the default implementation to do nothing
    fn configure(_: &RegisterBlock) {}
}

/// Lift the implementations of [`Txpo`] from [`OptionalPadNum`]s to the
/// corresponding [`Pads`] types.
impl<S, I, RX, TX, RTS, CTS> Txpo for Pads<S, I, RX, TX, RTS, CTS>
where
    S: Sercom,
    I: IoSet,
    RX: GetOptionalPad<S>,
    TX: GetOptionalPad<S>,
    RTS: GetOptionalPad<S>,
    CTS: GetOptionalPad<S>,
    TX::PadNum: Txpo,
{
    const TXPO: TXPO_A = TX::PadNum::TXPO;
}

//=============================================================================
// Pads
//=============================================================================

/// Container for a set of SERCOM [`Pad`]s
///
/// A [`Sercom`] can use up to four [`Pin`]s as peripheral [`Pad`]s, but only
/// certain `Pin` combinations are acceptable. In particular, all `Pin`s must be
/// mapped to the same `Sercom` and [`IoSet`] (see section 6.2.8.1 of the
/// datasheet). This HAL makes it impossible to use invalid `Pin`/`Pad`
/// combinations, and the `Pads` struct is responsible for enforcing these
/// constraints.
///
/// A `Pads` type takes up to six type parameters. The first two specify the
/// `Sercom` and `IoSet`. The remaining four, `RX`, `TX`, `RTS` and `CTS`, are
/// effectively [`OptionalPinId`]s for the Rx, Tx, RTS and CTS `Pad`s
/// respectively, and each defaults to [`NoneT`]. To be accepted as part of a
/// [`ValidConfig`], you must specify a `PinId` for at least one of
/// `RX` or `TX`.
///
/// ```
/// use atsamd_hal::gpio::v2::{PA08, PA09};
/// use atsamd_hal::sercom::v2::{Sercom0, uart};
/// use atsamd_hal::sercom::v2::pad::IoSet1;
/// use atsamd_hal::typelevel::NoneT;
///
/// type Pads = uart::Pads<Sercom0, IoSet1, PA08, NoneT, PA09>;
/// ```
///
/// `Pads` are created using the builder pattern. Start by creating an empty set
/// of `Pads` using [`Default`]. Then pass each respective `Pin` using the
/// corresponding methods. Both `v1::Pin` and `v2::Pin` types are accepted here.
/// Note that the `TX` `Pin` must map to [`Pad0`], and if specified, the `RTS`
/// `Pin` must map to [`Pad2`] and the `CTS` Pin` must map to [`Pad3`].
/// The `RX` `Pin` can vary in [`PadNum`]
/// based on the [`Rxpo`] values.
///
/// ```
/// use atsamd_hal::target_device::Peripherals;
/// use atsamd_hal::gpio::v2::Pins;
/// use atsamd_hal::sercom::v2::{Sercom0, uart};
/// use atsamd_hal::sercom::v2::pad::IoSet1;
///
/// let mut peripherals = Peripherals::take().unwrap();
/// let pins = Pins::new(peripherals.PORT);
/// let pads = uart::Pads::<Sercom0, IoSet1>::default()
///     .rts(pins.pa09)
///     .rx(pins.pa08)
///     .tx(pins.pa11);
/// ```
///
/// [`Pin`]: crate::gpio::v2::pin::Pin
/// [`OptionalPinId`]: crate::gpio::v2::pin::OptionalPinId
pub struct Pads<S, I, RX = NoneT, TX = NoneT, RTS = NoneT, CTS = NoneT>
where
    S: Sercom,
    I: IoSet,
    RX: GetOptionalPad<S>,
    TX: GetOptionalPad<S>,
    RTS: GetOptionalPad<S>,
    CTS: GetOptionalPad<S>,
{
    ioset: PhantomData<I>,
    receive: RX::Pad,
    transmit: TX::Pad,
    ready_to_send: RTS::Pad,
    clear_to_send: CTS::Pad,
}

impl<S: Sercom, I: IoSet> Default for Pads<S, I> {
    fn default() -> Self {
        Self {
            ioset: PhantomData,
            receive: NoneT,
            transmit: NoneT,
            ready_to_send: NoneT,
            clear_to_send: NoneT,
        }
    }
}

impl<S, I, RX, TX, RTS, CTS> Pads<S, I, RX, TX, RTS, CTS>
where
    S: Sercom,
    I: IoSet,
    RX: GetOptionalPad<S>,
    TX: GetOptionalPad<S>,
    RTS: GetOptionalPad<S>,
    CTS: GetOptionalPad<S>,
{
    /// Set the `RX` [`Pad`]
    #[inline]
    pub fn rx<Id>(self, pin: impl AnyPin<Id = Id>) -> Pads<S, I, Id, TX, RTS, CTS>
    where
        Id: PadInfo<S>,
        Id::PadNum: Rxpo,
        Pad<S, Id::PadNum, Id>: InIoSet<I>,
    {
        Pads {
            ioset: self.ioset,
            receive: pin.into().into(),
            transmit: self.transmit,
            ready_to_send: self.ready_to_send,
            clear_to_send: self.clear_to_send,
        }
    }

    /// Set the `TX` [`Pad`]
    #[inline]
    pub fn tx<Id>(self, pin: impl AnyPin<Id = Id>) -> Pads<S, I, RX, Id, RTS, CTS>
    where
        Id: PadInfo<S>,
        Id::PadNum: Txpo,
        Pad<S, Id::PadNum, Id>: InIoSet<I>,
    {
        Pads {
            ioset: self.ioset,
            receive: self.receive,
            transmit: pin.into().into(),
            ready_to_send: self.ready_to_send,
            clear_to_send: self.clear_to_send,
        }
    }

    /// Set the `RTS` [`Pad`], which is always [`Pad2`]
    #[inline]
    pub fn rts<Id>(self, pin: impl AnyPin<Id = Id>) -> Pads<S, I, RX, TX, Id, CTS>
    where
        Id: PadInfo<S, PadNum = Pad1>,
        Pad<S, Pad1, Id>: InIoSet<I>,
    {
        Pads {
            ioset: self.ioset,
            receive: self.receive,
            transmit: self.transmit,
            ready_to_send: pin.into().into(),
            clear_to_send: self.clear_to_send,
        }
    }

    /// Set the `CTS` [`Pad`], which is always [`Pad3`]
    #[inline]
    pub fn cts<Id>(self, pin: impl AnyPin<Id = Id>) -> Pads<S, I, RX, TX, RTS, Id>
    where
        Id: PadInfo<S, PadNum = Pad2>,
        Pad<S, Pad2, Id>: InIoSet<I>,
    {
        Pads {
            ioset: self.ioset,
            receive: self.receive,
            transmit: self.transmit,
            ready_to_send: self.ready_to_send,
            clear_to_send: pin.into().into(),
        }
    }

    /// Consume the [`Pads`] and return each individual [`Pad`]
    #[inline]
    pub fn free(self) -> (RX::Pad, TX::Pad, RTS::Pad, CTS::Pad) {
        (
            self.receive,
            self.transmit,
            self.ready_to_send,
            self.clear_to_send,
        )
    }
}

//=============================================================================
// uart_pads_from_pins
//=============================================================================

/// Define a set of [`uart::Pads`] using [`Pin`]s instead of [`PinId`]s
///
/// In some cases, it is more convenient to specify a set of `uart::Pads` using
/// `Pin`s or `Pin` aliases than it is to use the corresponding [`PinId`]s. This
/// macro makes it easier to do so.
///
/// The first two arguments to the macro are required and represent the
/// [`Sercom`] and [`IoSet`] respectively. The remaining four arguments are all
/// optional. Each represents a corresponding type parameter of the `uart::Pads`
/// type. Some of the types may be omitted, but any types that are specified,
/// must be done in the order `RX`, `TX`, `RTS` & `CTS`.
///
/// ```
/// use atsamd_hal::pac::Peripherals;
/// use atsamd_hal::uart_pads_from_pins;
/// use atsamd_hal::gpio::v2::{Pin, PA08, PA09, AlternateC, Pins};
/// use atsamd_hal::sercom::v2::{Sercom0, pad::IoSet1, uart};
///
/// type Miso = Pin<PA08, AlternateC>;
/// type RTS = Pin<PA09, AlternateC>;
/// pub type Pads = uart_pads_from_pins!(Sercom0, IoSet1, RX = Miso, RTS = RTS);
///
/// pub fn test() -> Pads {
///     let peripherals = Peripherals::take().unwrap();
///     let pins = Pins::new(peripherals.PORT);
///     uart::Pads::<Sercom0, IoSet1>::default()
///         .rts(pins.pa09)
///         .rx(pins.pa08)
/// }
/// ```
///
/// [`uart::Pads`]: Pads
/// [`Pin`]: crate::gpio::v2::Pin
/// [`PinId`]: crate::gpio::v2::PinId
#[macro_export]
macro_rules! uart_pads_from_pins {
    (
        $Sercom:ident,
        $IoSet:ident
        $( , RX = $RX:ty )?
        $( , TX = $TX:ty )?
        $( , RTS = $RTS:ty )?
        $( , CTS = $CTS:ty )?
    ) => {
        $crate::sercom::v2::uart::Pads<
            $crate::sercom::v2::$Sercom,
            $crate::sercom::v2::pad::$IoSet,
            $crate::__opt_type!( $( $crate::gpio::v2::SpecificPinId<$RX> )? ),
            $crate::__opt_type!( $( $crate::gpio::v2::SpecificPinId<$TX> )? ),
            $crate::__opt_type!( $( $crate::gpio::v2::SpecificPinId<$RTS> )? ),
            $crate::__opt_type!( $( $crate::gpio::v2::SpecificPinId<$CTS> )? ),
        >
    };
}

//=============================================================================
// PadSet
//=============================================================================

/// Type-level function to recover the [`OptionalPad`] types from a generic set
/// of [`Pads`]
///
/// This trait is used as an interface between the [`Pads`] type and other
/// types in this module. It acts as a [type-level function], returning the
/// corresponding [`Sercom`], [`IoSet`] and [`OptionalPad`] types. It serves to
/// cut down on the total number of type parameters needed in the [`Config`]
/// struct. The `Config` struct doesn't need access to the [`Pad`]s directly.
/// Rather, it only needs to apply the [`SomePad`] trait bound when a `Pad` is
/// required. The `PadSet` trait allows each `Config` struct to store an
/// instance of `Pads` without itself being generic over all six type parameters
/// of the `Pads` type.
///
/// [type-level function]: crate::typelevel#type-level-functions
pub trait PadSet: Sealed {
    type Sercom: Sercom;
    type IoSet: IoSet;
    type Rx: OptionalPad;
    type Tx: OptionalPad;
    type RTS: OptionalPad;
    type CTS: OptionalPad;
}

impl<S, I, RX, TX, RTS, CTS> Sealed for Pads<S, I, RX, TX, RTS, CTS>
where
    S: Sercom,
    I: IoSet,
    RX: GetOptionalPad<S>,
    TX: GetOptionalPad<S>,
    RTS: GetOptionalPad<S>,
    CTS: GetOptionalPad<S>,
{
}

impl<S, I, RX, TX, RTS, CTS> PadSet for Pads<S, I, RX, TX, RTS, CTS>
where
    S: Sercom,
    I: IoSet,
    RX: GetOptionalPad<S>,
    TX: GetOptionalPad<S>,
    RTS: GetOptionalPad<S>,
    CTS: GetOptionalPad<S>,
{
    type Sercom = S;
    type IoSet = I;
    type Rx = RX::Pad;
    type Tx = TX::Pad;
    type RTS = RTS::Pad;
    type CTS = CTS::Pad;
}

//=============================================================================
// ValidPads
//=============================================================================

/// Marker trait for valid sets of [`Pads`]
///
/// This trait labels sets of [`Pads`] that satisfy the [`Rxpo`] and [`Txpo`]
/// traits. It guarantees to the [`Config`] struct that this set of `Pads` can
/// be configured through those traits.
pub trait ValidPads: PadSet + Rxpo + Txpo {}

impl<P: PadSet + Rxpo + Txpo> ValidPads for P {}

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
/// A set of [`Pads`] cannot be used to transmit when the Tx [`Pad`] is
/// [`NoneT`].
pub trait NotTx: ValidPads {}

impl<P> NotTx for P where P: ValidPads<Tx = NoneT> {}

/// Marker trait for a set of [`Pads`] that cannot receive
///
/// A set of [`Pads`] cannot be used to receive when the Rx [`Pad`] is
/// [`NoneT`].
pub trait NotRx: ValidPads {}

impl<P> NotRx for P where P: ValidPads<Rx = NoneT> {}

/// Marker trait for a set of [`Pads`] that can transmit OR receive
///
/// To satisfy this trait, one or both of Rx and Tx must be [`SomePad`].
pub trait TxOrRx: ValidPads {}

impl<S, I, RX, RTS, CTS> TxOrRx for Pads<S, I, RX, NoneT, RTS, CTS>
where
    S: Sercom,
    I: IoSet,
    RX: GetPad<S> + GetPadMarker,
    RTS: GetOptionalPad<S>,
    CTS: GetOptionalPad<S>,
    RX::PadNum: Rxpo,
{
}

impl<S, I, TX, RTS, CTS> TxOrRx for Pads<S, I, NoneT, TX, RTS, CTS>
where
    S: Sercom,
    I: IoSet,
    TX: GetPad<S> + GetPadMarker,
    RTS: GetOptionalPad<S>,
    CTS: GetOptionalPad<S>,
    TX::PadNum: Txpo,
{
}

impl<S, I, RX, TX, RTS, CTS> TxOrRx for Pads<S, I, RX, TX, RTS, CTS>
where
    S: Sercom,
    I: IoSet,
    RX: GetPad<S> + GetPadMarker,
    TX: GetPad<S> + GetPadMarker,
    RTS: GetOptionalPad<S>,
    CTS: GetOptionalPad<S>,
    RX::PadNum: Rxpo,
    TX::PadNum: Txpo,
{
}

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
            .usart_int()
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

bitflags! {
    /// Interrupt bit flags for UART transactions
    ///
    /// The available interrupt flags are `DRE`, `RXC`, `TXC`, `RXS`, `CTSIC`, `RXBRK` and
    /// `ERROR`. The binary format of the underlying bits exactly matches the
    /// INTFLAG register.
    pub struct Flags: u8 {
        const DRE = 0x01;
        const TXC = 0x02;
        const RXC = 0x04;
        const RXS = 0x08;
        const CTSIC = 0x10;
        const RXBRK = 0x20;
        const ERROR = 0x80;
    }
}

//=============================================================================
// Status
//=============================================================================

bitflags! {
    /// Status flags for UART transactions
    ///
    /// The available error flags are `PERR`, `FERR`, `BUFOVF`, `CTS`, `ISF`, `COLL`,
    /// `TXE` and `ITER`.
    /// The binary format of the underlying bits exactly matches the STATUS register.
    pub struct Status: u16 {
        const PERR = 0x0001;
        const FERR = 0x0002;
        const BUFOVF = 0x0004;
        const CTS = 0x0008;
        const ISF = 0x0010;
        const COLL = 0x0020;
        const TXE = 0x0040;
        const ITER = 0x0080;
    }
}

/// Error `enum` for UART transactions
#[derive(Debug)]
pub enum Error {
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
    /// Maximum number of NACK repetitions or retransmissions is met
    /// in ISO7816 T = 0 mode.
    MaximumRepetitions,
}

impl TryFrom<Status> for () {
    type Error = Error;
    fn try_from(errors: Status) -> Result<(), Error> {
        if errors.contains(Status::PERR) {
            Err(Error::ParityError)
        } else if errors.contains(Status::FERR) {
            Err(Error::FrameError)
        } else if errors.contains(Status::BUFOVF) {
            Err(Error::Overflow)
        } else if errors.contains(Status::ISF) {
            Err(Error::InconsistentSyncField)
        } else if errors.contains(Status::COLL) {
            Err(Error::CollisionDetected)
        } else if errors.contains(Status::ITER) {
            Err(Error::MaximumRepetitions)
        } else {
            Ok(())
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
/// with an [`EightBit`], no parity, one stop bit frame .
///
/// [`Config`] uses a builder-pattern API to configure the peripheral,
/// culminating in a call to [`enable`], which consumes the [`Config`] and
/// returns an enabled [`Uart`] peripheral. The [`enable`] function is
/// restricted to [`ValidConfig`]s.
///
/// [`enable`]: Config::enable
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
    fn create(sercom: P::Sercom, pads: P, freq: impl Into<Hertz>) -> Self {
        Self::swrst(&sercom);
        <P as Rxpo>::configure(&sercom);
        <P as Txpo>::configure(&sercom);
        EightBit::configure(&sercom);

        // Enable internal clock mode
        sercom
            .usart_int()
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
    /// configuration, [`EightBit`] [`CharSize`].
    /// [`Config`] takes ownership of the [`Sercom`] and [`Pads`].
    ///
    /// Users must configure GCLK manually. The `freq` parameter represents the
    /// GCLK frequency for this [`Sercom`] instance.
    #[inline]
    pub fn new(mclk: &MCLK, mut sercom: P::Sercom, pads: P, freq: impl Into<Hertz>) -> Self {
        sercom.enable_apb_clock(mclk);
        Self::create(sercom, pads, freq)
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
        sercom.usart_int().ctrla.write(|w| w.swrst().set_bit());
        while sercom.usart_int().syncbusy.read().swrst().bit_is_set() {}
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
            .usart_int()
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
            self.sercom
                .usart_int()
                .ctrlb
                .modify(|_, w| w.pmode().bit(odd));
            true
        } else {
            false
        };

        self.sercom
            .usart_int()
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
            .usart_int()
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
            .usart_int()
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
            .usart_int()
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

        self.sercom
            .usart_int()
            .ctrla
            .modify(|_, w| unsafe { w.sampr().bits(mode.sampr()) });

        let baud = match mode {
            BaudMode::Arithmetic(n) => {
                Self::calculate_baud_asynchronous_arithm(baud.0, self.freq.0, n as u8)
            }

            BaudMode::Fractional(n) => {
                todo!();
            }
        };

        self.sercom
            .usart_int()
            .baud()
            .modify(|_, w| unsafe { w.baud().bits(baud) });

        self
    }

    #[inline]
    /// Calculate baudrate value using the asynchronous arithmetic method (Table
    /// 24-2)
    fn calculate_baud_asynchronous_arithm(baudrate: u32, clk_freq: u32, n_samples: u8) -> u16 {
        const SHIFT: u8 = 32;
        let sample_rate = (n_samples as u64 * baudrate as u64) << SHIFT;
        let ratio = sample_rate / clk_freq as u64;
        let scale = (1u64 << SHIFT) - ratio;
        let baud_calculated = (65536u64 * scale) >> SHIFT;

        baud_calculated as u16
    }

    #[inline]
    /// Calculate baudrate value using the asynchronous frational method (Table
    /// 24-2)
    fn calculate_baud_asynchronous_fractional(baudrate: u32, clk_freq: u32, n_samples: u8) -> u16 {
        todo!();
    }

    /// Control the buffer overflow notification
    ///
    /// If set to true, an [`Error::Overflow`] will be issued as soon as an
    /// overflow occurs. Otherwise, it will not be issued until its place within
    /// the data stream.
    #[inline]
    pub fn immediate_overflow_notification(&mut self, set: bool) {
        self.sercom
            .usart_int()
            .ctrla
            .modify(|_, w| w.ibon().bit(set));
    }

    /// Run in standby mode
    ///
    /// When set, the UART peripheral will run in standby mode. See the
    /// datasheet for more details.
    #[inline]
    pub fn run_in_standby(&mut self, set: bool) {
        self.sercom
            .usart_int()
            .ctrla
            .modify(|_, w| w.runstdby().bit(set));
    }

    /// Enable interrupts for the specified flags
    pub fn enable_interrupts(&mut self, flags: Flags) {
        self.sercom
            .usart_int()
            .intenset
            .write(|w| unsafe { w.bits(flags.bits()) });
    }

    /// Disable interrupts for the specified flags
    pub fn disable_interrupts(&mut self, flags: Flags) {
        self.sercom
            .usart_int()
            .intenclr
            .write(|w| unsafe { w.bits(flags.bits()) });
    }

    pub fn irda_encoding(&mut self, irda: bool) {
        self.sercom
            .usart_int()
            .ctrlb
            .modify(|_, w| w.enc().bit(irda));
    }

    /// Enable the UART peripheral
    ///
    /// UART transactions are not possible until the peripheral is enabled.
    /// This function is limited to [`ValidConfig`]s.
    #[inline]
    pub fn enable(self) -> Uart<Self>
    where
        Self: ValidConfig,
    {
        // Enable RX
        self.sercom
            .usart_int()
            .ctrlb
            .modify(|_, w| w.rxen().set_bit());
        while self.sercom.usart_int().syncbusy.read().ctrlb().bit_is_set() {}

        // Enable TX
        self.sercom
            .usart_int()
            .ctrlb
            .modify(|_, w| w.txen().set_bit());
        while self.sercom.usart_int().syncbusy.read().ctrlb().bit_is_set() {}

        // Globally enable peripheral
        self.sercom
            .usart_int()
            .ctrla
            .modify(|_, w| w.enable().set_bit());
        while self
            .sercom
            .usart_int()
            .syncbusy
            .read()
            .enable()
            .bit_is_set()
        {}

        // Perform a copy of the sercom instance, so that
        // the read and write halves can access the sercom
        // PAC struct independently.
        let rx = UartRx {
            sercom: unsafe { ConfigSercom::<Self>::steal() },
            _config: PhantomData,
        };

        let tx = UartTx {
            sercom: unsafe { ConfigSercom::<Self>::steal() },
            _config: PhantomData,
        };

        Uart {
            config: self,
            rx,
            tx,
        }
    }

    /// Enable or disable the SERCOM peripheral, and wait for the ENABLE bit to
    /// synchronize.
    fn enable_peripheral(&mut self, enable: bool) {
        self.sercom
            .usart_int()
            .ctrla
            .modify(|_, w| w.enable().bit(enable));
        while self
            .sercom
            .usart_int()
            .syncbusy
            .read()
            .enable()
            .bit_is_set()
        {}
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

trait Registers: Sealed {
    type Sercom: Sercom;
    unsafe fn sercom(&self) -> &Self::Sercom;

    /// Read the interrupt flags
    fn read_flags(&self) -> Flags {
        let bits = unsafe { self.sercom().usart_int().intflag.read().bits() };
        Flags::from_bits_truncate(bits)
    }

    /// Read the error status flags
    #[inline]
    fn read_status(&self) -> Status {
        let bits = unsafe { self.sercom().usart_int().status.read().bits() };
        Status::from_bits_truncate(bits)
    }

    #[inline]
    fn read_flags_errors(&self) -> Result<Flags, Error> {
        self.read_status().try_into()?;
        Ok(self.read_flags())
    }
}

trait RxRegisters: Registers {}

trait TxRegisters: Registers {}

//=============================================================================
// Uart
//=============================================================================
/// An enabled UART peripheral that can perform transactions using the embedded
/// HAL traits
///
/// When an [`Uart`] is [`Tx`]` + `[`Rx`], it implements [`Read`] + [`Write`],
/// with a word type that depends on [`CharSize`]. The word type is [`u8`] for
/// an [`EightBit`] or less [`CharSize`] and [`u16`] for a [`NineBit`]
/// [`CharSize`].
///
/// For half-duplex transactions, [`Uart`] implements the `serial` [`Read`] or
/// [`Write`] traits.
///
/// [`Uart`] uses the default implementations for the [`blocking::serial`]
/// traits.
///
/// For a non-blocking alternative that can be used to transfer slices, see the
/// [`UartFuture`] type.
///
/// [`UartFuture`]: crate::sercom::v2::uart_future::UartFuture
pub struct Uart<C>
where
    C: ValidConfig,
{
    config: C,
    pub rx: UartRx<C, ConfigSercom<C>>,
    pub tx: UartTx<C, ConfigSercom<C>>,
}

impl<C: ValidConfig> Uart<C> {
    /// Obtain a reference to the PAC `SERCOM` struct
    ///
    /// Directly accessing the `SERCOM` could break the invariants of the
    /// type-level tracking in this module, so it is unsafe.
    #[inline]
    pub unsafe fn sercom(&self) -> &C::Sercom {
        &self.config.as_ref().sercom()
    }

    /// Update the UART configuration.
    ///
    /// Calling this method will temporarily disable the SERCOM peripheral, as
    /// some registers are enable-protected. This may interrupt any ongoing
    /// transactions.
    #[inline]
    pub fn reconfigure<F>(&mut self, update: F)
    where
        F: FnOnce(SpecificConfig<C>) -> SpecificConfig<C>,
    {
        self.config.as_mut().enable_peripheral(false);

        // Perform a bitwise copy of the old configuration. This will be used as default
        // in case the call to update(self.config) panics. This should be safe
        // as either one of self.config or old_config will be used, and Config
        // does not deallocate when dropped.
        let old_config = unsafe { core::ptr::read(&mut self.config) };
        replace_with::replace_with(&mut self.config, || old_config, |c| update(c.into()).into());

        self.config.as_mut().enable_peripheral(true);
    }

    /// Enable interrupts for the specified flags
    #[inline]
    pub fn enable_interrupts(&mut self, flags: Flags) {
        self.config.as_mut().enable_interrupts(flags)
    }

    /// Disable interrupts for the specified flags
    #[inline]
    pub fn disable_interrupts(&mut self, flags: Flags) {
        self.config.as_mut().disable_interrupts(flags);
    }

    /// Clear interrupt status flags
    ///
    /// Setting the ERROR, CTSL or TXC flag will clear the interrupt. Clearing
    /// any flag will have no effect. This function has no effect on the DRE or
    /// RXC flags.
    ///
    /// **Warning:** The implementation of of [`Write::flush`] waits on and
    /// clears the `TXC` flag. Manually clearing this flag could cause it to
    /// hang indefinitely.
    #[inline]
    pub fn clear_flags(&mut self, flags: Flags) {
        unsafe {
            self.sercom()
                .usart_int()
                .intflag
                .write(|w| w.bits(flags.bits()))
        };
    }

    /// Read the interrupt flags
    pub fn read_flags(&self) -> Flags {
        <Self as Registers>::read_flags(self)
    }

    /// Read the status flags
    #[inline]
    pub fn read_status(&self) -> Status {
        <Self as Registers>::read_status(self)
    }

    /// Clear error status flags
    ///
    /// Setting a flag will clear the error. Clearing any flag will have no
    /// effect.
    #[inline]
    pub fn clear_errors(&mut self, errors: Status) {
        unsafe {
            self.sercom()
                .usart_int()
                .status
                .write(|w| w.bits(errors.bits()))
        };
    }

    /// Read the status register and convert into a [`Result`]
    /// containing the corresponding [`Error`]
    #[inline]
    pub fn read_flags_errors(&self) -> Result<Flags, Error> {
        <Self as Registers>::read_flags_errors(self)
    }

    /// Read from the DATA register
    ///
    /// Reading from the data register directly is `unsafe`, because it will
    /// clear the RXC flag, which could break assumptions made elsewhere in
    /// this module.
    #[inline]
    pub unsafe fn read_data(&mut self) -> u32 {
        self.rx.read_data()
    }

    /// Write to the DATA register
    ///
    /// Writing to the data register directly is `unsafe`, because it will clear
    /// the DRE flag, which could break assumptions made elsewhere in this
    /// module.
    #[inline]
    pub unsafe fn write_data(&mut self, data: u32) {
        self.tx.write_data(data)
    }

    /// Disable the UART peripheral and return the [`Config`] struct
    #[inline]
    pub fn disable(mut self) -> C {
        let usart = unsafe { self.sercom().usart_int() };
        usart.ctrlb.modify(|_, w| w.rxen().clear_bit());
        while usart.syncbusy.read().ctrlb().bit_is_set() {}
        self.config.as_mut().enable_peripheral(false);
        self.config
    }
}

impl<C: ValidConfig> Registers for Uart<C> {
    type Sercom = C::Sercom;

    unsafe fn sercom(&self) -> &Self::Sercom {
        self.config.as_ref().sercom()
    }
}

//=============================================================================
// AnyUart
//=============================================================================

/// Type class for all possible [`Uart`] types
///
/// This trait uses the [`AnyKind`] trait pattern to create a [type class] for
/// [`Uart`] types. See the `AnyKind` documentation for more details on the
/// pattern.
///
/// In addition to the normal, `AnyKind` associated types. This trait also
/// copies the [`Sercom`], [`Pads`], [`CharSize`] and [`Word`]
/// types, to make it easier to apply bounds to these types at the next level of
/// abstraction.
///
/// [`AnyKind`]: crate::typelevel#anykind-trait-pattern
/// [type class]: crate::typelevel#type-classes
pub trait AnyUart: Is<Type = SpecificUart<Self>> {
    type Sercom: Sercom;
    type Pads: PadSet<Sercom = Self::Sercom>;
    type CharSize: CharSize<Word = Self::Word>;
    type Word: 'static;
    type Config: ValidConfig<
        Sercom = Self::Sercom,
        Pads = Self::Pads,
        CharSize = Self::CharSize,
        Word = Self::Word,
    >;
}

/// Type alias to recover the specific [`Uart`] type from an implementation of
/// [`AnyUart`]
pub type SpecificUart<S> = Uart<<S as AnyUart>::Config>;

impl<C: ValidConfig> AsRef<Self> for Uart<C> {
    #[inline]
    fn as_ref(&self) -> &Self {
        self
    }
}

impl<C: ValidConfig> AsMut<Self> for Uart<C> {
    #[inline]
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl<C: ValidConfig> Sealed for Uart<C> {}

impl<C: ValidConfig> AnyUart for Uart<C> {
    type Sercom = C::Sercom;
    type Pads = C::Pads;
    type CharSize = C::CharSize;
    type Word = C::Word;
    type Config = C;
}

//=============================================================================
// Rx/Tx halves
//=============================================================================

/// Read half of a split UART
pub struct UartRx<C: ValidConfig, S: Sercom> {
    sercom: S,
    _config: PhantomData<C>,
}

impl<C: ValidConfig, S: Sercom> UartRx<C, S> {
    /// Read from the DATA register
    ///
    /// Reading from the data register directly is `unsafe`, because it will
    /// clear the RXC flag, which could break assumptions made elsewhere in
    /// this module.
    #[inline]
    pub unsafe fn read_data(&mut self) -> u32 {
        self.sercom.usart_int().data.read().data().bits()
    }
}

impl<C: ValidConfig, S: Sercom> Registers for UartRx<C, S> {
    type Sercom = S;

    unsafe fn sercom(&self) -> &Self::Sercom {
        &self.sercom
    }
}

impl<C: ValidConfig, S: Sercom> Sealed for UartRx<C, S> {}

/// Write half of a split UART
pub struct UartTx<C: ValidConfig, S: Sercom> {
    sercom: S,
    _config: PhantomData<C>,
}

impl<C: ValidConfig, S: Sercom> UartTx<C, S> {
    /// Write to the DATA register
    ///
    /// Writing to the data register directly is `unsafe`, because it will clear
    /// the DRE flag, which could break assumptions made elsewhere in this
    /// module.
    #[inline]
    pub unsafe fn write_data(&mut self, data: u32) {
        self.sercom.usart_int().data.write(|w| w.data().bits(data))
    }
}

impl<C: ValidConfig, S: Sercom> Registers for UartTx<C, S> {
    type Sercom = S;

    unsafe fn sercom(&self) -> &Self::Sercom {
        &self.sercom
    }
}

impl<C: ValidConfig, S: Sercom> Sealed for UartTx<C, S> {}

//=============================================================================
// UART DMA transfers
//=============================================================================
#[cfg(feature = "dma")]
pub use uart_dma::*;

#[cfg(feature = "dma")]
mod uart_dma {
    use super::*;
    use crate::dmac::{
        self,
        channel::{self, Busy, Channel, ChannelId, Ready},
        transfer, Transfer,
    };

    unsafe impl<C, S> dmac::transfer::Buffer for UartTx<C, S>
    where
        S: Sercom,
        C: ValidConfig,
        C::Word: dmac::transfer::Beat,
    {
        type Beat = C::Word;

        #[inline]
        fn dma_ptr(&mut self) -> *mut Self::Beat {
            self.sercom.usart_int().data.as_ptr() as *mut _
        }

        #[inline]
        fn incrementing(&self) -> bool {
            false
        }

        #[inline]
        fn buffer_len(&self) -> usize {
            1
        }
    }

    unsafe impl<C, S> dmac::transfer::Buffer for UartRx<C, S>
    where
        S: Sercom,
        C: ValidConfig,
        C::Word: dmac::transfer::Beat,
    {
        type Beat = C::Word;

        #[inline]
        fn dma_ptr(&mut self) -> *mut Self::Beat {
            self.sercom.usart_int().data.as_ptr() as *mut _
        }

        #[inline]
        fn incrementing(&self) -> bool {
            false
        }

        #[inline]
        fn buffer_len(&self) -> usize {
            1
        }
    }

    impl<C, S> UartTx<C, S>
    where
        Self: dmac::transfer::Buffer<Beat = C::Word>,
        S: Sercom,
        C: ValidConfig,
    {
        /// Transform an [`UartTx`] into a DMA [`Transfer`]) and
        /// start sending the provided buffer.
        #[inline]
        pub fn send_with_dma<Chan, B, W>(
            self,
            buf: B,
            mut channel: Chan,
            waker: W,
        ) -> Transfer<Channel<ChannelId<Chan>, Busy>, transfer::BufferPair<B, Self>, (), W>
        where
            Chan: channel::AnyChannel<Status = Ready>,
            B: dmac::Buffer<Beat = C::Word> + 'static,
            W: FnOnce(crate::dmac::channel::CallbackStatus) + 'static,
        {
            channel
                .as_mut()
                .enable_interrupts(dmac::channel::InterruptFlags::new().with_tcmpl(true));

            // SAFETY: We use new_unchecked to avoid having to pass a 'static self as the
            // destination buffer. This is safe as long as we guarantee the source buffer is
            // static.
            unsafe { dmac::Transfer::new_unchecked(channel, buf, self, false) }
                .with_waker(waker)
                .begin(S::DMA_TX_TRIGGER, dmac::TriggerAction::BURST)
        }
    }

    impl<C, S> UartRx<C, S>
    where
        Self: dmac::transfer::Buffer<Beat = C::Word>,
        S: Sercom,
        C: ValidConfig,
    {
        /// Transform an [`UartRx`] into a DMA [`Transfer`]) and
        /// start receiving into the provided buffer.
        #[inline]
        pub fn receive_with_dma<Chan, B, W>(
            self,
            buf: B,
            mut channel: Chan,
            waker: W,
        ) -> Transfer<Channel<ChannelId<Chan>, Busy>, transfer::BufferPair<Self, B>, (), W>
        where
            Chan: channel::AnyChannel<Status = Ready>,
            B: dmac::Buffer<Beat = C::Word> + 'static,
            W: FnOnce(crate::dmac::channel::CallbackStatus) + 'static,
        {
            channel
                .as_mut()
                .enable_interrupts(dmac::channel::InterruptFlags::new().with_tcmpl(true));

            // SAFETY: We use new_unchecked to avoid having to pass a 'static self as the
            // destination buffer. This is safe as long as we guarantee the source buffer is
            // static.
            unsafe { dmac::Transfer::new_unchecked(channel, self, buf, false) }
                .with_waker(waker)
                .begin(S::DMA_RX_TRIGGER, dmac::TriggerAction::BURST)
        }
    }
}

//=============================================================================
// Embedded HAL traits
//=============================================================================

/// Implement [`Read`] for [`UartRx`].
impl<C, S> Read<C::Word> for UartRx<C, S>
where
    C: ValidConfig,
    S: Sercom,
    C::Word: PrimInt,
    u32: AsPrimitive<C::Word>,
{
    type Error = Error;

    /// Wait for an `RXC` flag, then read the word
    #[inline]
    fn read(&mut self) -> nb::Result<C::Word, Error> {
        let flags = self.read_flags_errors()?;
        if flags.contains(Flags::RXC) {
            unsafe { Ok(self.read_data().as_()) }
        } else {
            Err(WouldBlock)
        }
    }
}

/// Implement [`Read`] for [`Uart`]
///
/// [`Read`] is only implemented when the [`Pads`] are [`Rx`].
impl<C, E> Read<C::Word> for Uart<C>
where
    C: ValidConfig,
    C::Pads: Rx,
    C::Word: PrimInt,
    u32: AsPrimitive<C::Word>,
    UartRx<C, ConfigSercom<C>>: Read<C::Word, Error = E>,
{
    type Error = E;

    /// Wait for an `RXC` flag, then read the word
    #[inline]
    fn read(&mut self) -> nb::Result<C::Word, E> {
        self.rx.read()
    }
}

/// Implement [`Write`] for [`UartTx`].
impl<C, S> Write<C::Word> for UartTx<C, S>
where
    C: ValidConfig,
    S: Sercom,
    C::Word: PrimInt + AsPrimitive<u32>,
{
    type Error = Error;

    /// Wait for a `DRE` flag, then write a word
    #[inline]
    fn write(&mut self, word: C::Word) -> nb::Result<(), Error> {
        // Ignore buffer overflow errors
        if self.read_flags().contains(Flags::DRE) {
            unsafe { self.write_data(word.as_()) };
            Ok(())
        } else {
            Err(WouldBlock)
        }
    }

    /// Wait for a `TXC` flag
    #[inline]
    fn flush(&mut self) -> nb::Result<(), Error> {
        // Ignore buffer overflow errors
        if self.read_flags().contains(Flags::TXC) {
            Ok(())
        } else {
            Err(WouldBlock)
        }
    }
}

/// Implement [`Write`] for [`Uart`]
///
/// [`Write`] is only implemented when the [`Pads`] are [`Tx`].
///
/// This implementation never reads the DATA register and ignores all buffer
/// overflow errors.
impl<C, E> Write<C::Word> for Uart<C>
where
    C: ValidConfig,
    C::Pads: Tx,
    C::Word: PrimInt + AsPrimitive<u16>,
    UartTx<C, ConfigSercom<C>>: Write<C::Word, Error = E>,
{
    type Error = E;

    /// Wait for a `DRE` flag, then write a word
    #[inline]
    fn write(&mut self, word: C::Word) -> nb::Result<(), E> {
        self.tx.write(word)
    }

    /// Wait for a `TXC` flag
    #[inline]
    fn flush(&mut self) -> nb::Result<(), E> {
        self.tx.flush()
    }
}

impl<C> blocking::serial::write::Default<C::Word> for Uart<C>
where
    C: ValidConfig,
    Uart<C>: Write<C::Word>,
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
