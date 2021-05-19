use core::convert::{TryFrom, TryInto};
use core::marker::PhantomData;
use core::mem::transmute;

use crate::target_device as pac;
use crate::time::Hertz;
use pac::{sercom0::RegisterBlock, PM};

use crate::gpio::v2::{AnyPin, SpecificPin};
use crate::sercom::v2::pads::{Map, Pad0, Pad1, Pad2, Pad3, PadNum};
use crate::sercom::v2::pads::{OptionalPad, Pad, SomePad};
use crate::sercom::v2::Sercom;
use crate::typelevel::{Is, NoneT, Sealed};

use bitflags::bitflags;
use embedded_hal::blocking;
use embedded_hal::serial::{Read, Write};
use nb::Error::WouldBlock;
use num_traits::{AsPrimitive, PrimInt};
use paste::paste;

//=============================================================================
// Pad configuration
//=============================================================================

pub trait RxpoTxpo: AnyPads {
    /// `RXPO` field value
    const RXPO: u8;

    /// `TXPO` field value
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

/// Implement [`RxpoTxpo`] for different [`PadNum`] combinations
///
/// This macro uses the push-down accumulation method to build an implementation
/// of [`RxpoTxpo`] for a variable number of specified [`PadNum`]s. See this
/// [link](https://veykril.github.io/tlborm/patterns/push-down-acc.html) for
/// more details on the technique.
macro_rules! impl_rxpo_txpo
{
    // This is the entry pattern
    (
        ( $($pad:ident),+ ): ($RXPO:literal, $TXPO:literal)
    ) => {
        impl_rxpo_txpo!(
            ( $($pad,)+ ): ($RXPO, $TXPO) -> []
        );
    };

    // If the [`Pad`] type is [`NoneT`], then no extra type parameters or trait
    // bounds are needed in the implementation.
    (
        ( NoneT, $($pad:ident,)* ): ($RXPO:tt, $TXPO:tt) -> [ $($body:tt,)* ]
    ) => {
        impl_rxpo_txpo!(
            ( $($pad,)* ): ($RXPO, $TXPO) -> [ $($body,)* { (), (NoneT) }, ]
        );
    };


    // To specify a [`Pad`] type with a particular [`PadNum`], you need to add
    // a type parameter for the [`Map`] type and a trait bound to enforce it.
    // Each type parameter must have a unique name, so create one by prepending
    // the [`PadNum`] type with `M`, using the [`paste`] macro. The actual macro
    // is not used until the final pattern.
    (
        ( $PadNum:ident, $($pad:ident,)* ): ($RXPO:tt, $TXPO:tt) -> [ $($body:tt,)* ]
    ) => {
        impl_rxpo_txpo!(
            ( $($pad,)* ): ($RXPO, $TXPO) ->
            [
                $($body,)*
                {
                    ( [<M $PadNum>]: Map<S, $PadNum>, ),
                    ( Pad<S, $PadNum, [<M $PadNum>]> )
                },
            ]
        );
    };

    // Build the complete implementation
    (
        (): ($RXPO:tt, $TXPO:tt) ->
        [
            $(
                {
                    ( $($Tp:tt)* ),
                    ( $($Ty:tt)+ )
                },
            )+
        ]
    ) => {
        paste! {
            impl<S: Sercom, $( $($Tp)* )+ > RxpoTxpo for Pads<S, $( $($Ty)+, )+ > {
                const RXPO: u8 = $RXPO;
                const TXPO: u8 = $TXPO;
            }
        }
    };
}

// Only combinations with a valid pin for either `RX` or `TX`
// have been considered. Other combinations would have no practical use.

// Rx Only
impl_rxpo_txpo!((Pad0, NoneT, NoneT, NoneT): (0, 1));
impl_rxpo_txpo!((Pad1, NoneT, NoneT, NoneT): (1, 0));
impl_rxpo_txpo!((Pad2, NoneT, NoneT, NoneT): (2, 0));
impl_rxpo_txpo!((Pad3, NoneT, NoneT, NoneT): (3, 0));

// Rx + Flow Control
impl_rxpo_txpo!((Pad1, NoneT, NoneT, Pad2): (1, 0));

// Tx Only
impl_rxpo_txpo!((NoneT, Pad0, NoneT, NoneT): (0, 0));
impl_rxpo_txpo!((NoneT, Pad2, NoneT, NoneT): (0, 1));

// Tx + Flow Control
impl_rxpo_txpo!((NoneT, Pad0, Pad3, NoneT): (0, 2));

// Rx + Tx
impl_rxpo_txpo!((Pad0, Pad2, NoneT, NoneT): (0, 1));
impl_rxpo_txpo!((Pad1, Pad0, NoneT, NoneT): (1, 0));
impl_rxpo_txpo!((Pad1, Pad2, NoneT, NoneT): (1, 1));
impl_rxpo_txpo!((Pad2, Pad0, NoneT, NoneT): (2, 0));
impl_rxpo_txpo!((Pad3, Pad0, NoneT, NoneT): (3, 0));
impl_rxpo_txpo!((Pad3, Pad2, NoneT, NoneT): (3,1));

// Rx + Tx + Flow Control
impl_rxpo_txpo!((Pad1, Pad0, Pad3, Pad2): (1, 2));

//=============================================================================
// Pads
//============================================================================

/// Encapsulate the set of pads for a UART peripheral
///
/// This struct acts to encapsulate up to four [`Pad`]s for use with a UART
/// peripheral. All of the [`Pad`]s must share the same [`Sercom`]. The four
/// type parameters `RX`, `TX`, `CTS` and `RTS` represent the respective RX, TX,
/// CTS and RTS [`Pad`] types.
///
/// Each pad in this struct is an [`OptionalPad`]. When first initialized, each
/// pad is set to [`NoneT`]. To be accepted as a valid set of [`Pads`] by the
/// [`Config`] struct, the [`Pads`] must implement [`RxpoTxpo], which requires
/// either the `RX` or `TX` pad to be [`SomePad`].
///
/// The [`Sercom`] type must be specified up front and is the same for each
/// [`Pad`], but each [`Pad`] will have different [`PadNum`] and [`Map`] types.
/// For SAMD11 and SAMD21 chips, the [`Map`] type is always a [`PinId`].
///
/// Individual pads are set using a builder-pattern API. The argument to each
/// function is a GPIO [`Pin`]. Both `v1` and `v2` pin types are accepted here.
/// The [`PinId`] can be extracted from the [`Pin`] type, so there is no need to
/// manually specify the [`Map`] type. But you will need to specify the desired
/// [`PadNum`]. Based on the [`Sercom`], [`PadNum`] and [`PinId`] types, the
/// [`Pin`] will be converted to the corresponding [`Pad`] automatically.
///
/// The following example corresponds to a set of [`Pads`] with `RXPO = 1` and
/// `TXPO = 0`. It is possible to create a set of [`Pads`] that does not
/// implement [`RxpoTxpo`], but such [`Pads`] will not be accepted by
/// [`Config`].
///
/// ```no_run
/// # use atsamd_hal::target_device::Peripherals;
/// # use atsamd_hal::gpio::v2::Pins;
/// # use atsamd_hal::sercom::v2::Sercom0;
/// # use atsamd_hal::sercom::v2::pads::{Pad0, Pad1, Pad2, Pad3};
/// # use atsamd_hal::sercom::v2::uart;
/// let mut peripherals = Peripherals::take().unwrap();
/// let pins = Pins::new(peripherals.PORT);
/// let pads = uart::Pads::<Sercom0>::new()
///     .rx::<Pad1, _>(pins.pa09)
///     .tx::<Pad0, _>(pins.pa08);
/// ```
///
/// The [`Tx`], [`Rx`], [`NotTx`], [`NotRx`] and [`TxOrRx`] marker traits are
/// implemented only for [`Pad`] combinations reflecting each trait's name.
///
/// [`Pin`]: crate::gpio::v2::pin::Pin
/// [`PinId`]: crate::gpio::v2::pin::PinId
pub struct Pads<S, RX = NoneT, TX = NoneT, CTS = NoneT, RTS = NoneT>
where
    S: Sercom,
    RX: OptionalPad,
    TX: OptionalPad,
    CTS: OptionalPad,
    RTS: OptionalPad,
{
    sercom: PhantomData<S>,
    rx: RX,
    tx: TX,
    cts: CTS,
    rts: RTS,
}

impl<S: Sercom> Pads<S> {
    /// Create a new [`Pads`] struct
    ///
    /// All of the pads are initialized to [`NoneT`]
    #[inline]
    pub fn new() -> Pads<S> {
        Pads {
            sercom: PhantomData,
            rx: NoneT,
            tx: NoneT,
            cts: NoneT,
            rts: NoneT,
        }
    }
}

impl<S, RX, TX, CTS, RTS> Pads<S, RX, TX, CTS, RTS>
where
    S: Sercom,
    RX: OptionalPad,
    TX: OptionalPad,
    CTS: OptionalPad,
    RTS: OptionalPad,
{
    /// Set the `RX` [`Pad`] using [`PadNum`] `P`
    ///
    /// The type parameter `I` represents the GPIO [`Pin`] type. Its
    /// corresponding [`PinId`] will be used as the [`Pad`]'s [`Map`] type.
    ///
    /// [`Pin`]: crate::gpio::v2::pin::Pin
    /// [`PinId`]: crate::gpio::v2::pin::PinId
    #[inline]
    pub fn rx<P, T>(self, rx: T) -> Pads<S, Pad<S, P, T::Id>, TX, CTS, RTS>
    where
        P: PadNum,
        T: AnyPin,
        T::Id: Map<S, P>,
        Pad<S, P, T::Id>: From<SpecificPin<T>>,
    {
        Pads {
            sercom: self.sercom,
            rx: rx.into().into(),
            tx: self.tx,
            cts: self.cts,
            rts: self.rts,
        }
    }

    /// Set the `TX` [`Pad`] using [`PadNum`] `P`
    ///
    /// The type parameter `I` represents the GPIO [`Pin`] type. Its
    /// corresponding [`PinId`] will be used as the [`Pad`]'s [`Map`] type.
    ///
    /// [`Pin`]: crate::gpio::v2::pin::Pin
    /// [`PinId`]: crate::gpio::v2::pin::PinId
    #[inline]
    pub fn tx<P, T>(self, tx: T) -> Pads<S, RX, Pad<S, P, T::Id>, CTS, RTS>
    where
        P: PadNum,
        T: AnyPin,
        T::Id: Map<S, P>,
        Pad<S, P, T::Id>: From<SpecificPin<T>>,
    {
        Pads {
            sercom: self.sercom,
            rx: self.rx,
            tx: tx.into().into(),
            cts: self.cts,
            rts: self.rts,
        }
    }

    /// Set the `CTS` [`Pad`] using [`PadNum`] `P`
    ///
    /// The type parameter `I` represents the GPIO [`Pin`] type. Its
    /// corresponding [`PinId`] will be used as the [`Pad`]'s [`Map`] type.
    ///
    /// [`Pin`]: crate::gpio::v2::pin::Pin
    /// [`PinId`]: crate::gpio::v2::pin::PinId
    #[inline]
    pub fn cts<P, T>(self, cts: T) -> Pads<S, RX, TX, Pad<S, P, T::Id>, RTS>
    where
        P: PadNum,
        T: AnyPin,
        T::Id: Map<S, P>,
        Pad<S, P, T::Id>: From<SpecificPin<T>>,
    {
        Pads {
            sercom: self.sercom,
            rx: self.rx,
            tx: self.tx,
            cts: cts.into().into(),
            rts: self.rts,
        }
    }

    /// Set the `RTS` [`Pad`] using [`PadNum`] `P`
    ///
    /// The type parameter `I` represents the GPIO [`Pin`] type. Its
    /// corresponding [`PinId`] will be used as the [`Pad`]'s [`Map`] type.
    ///
    /// [`Pin`]: crate::gpio::v2::pin::Pin
    /// [`PinId`]: crate::gpio::v2::pin::PinId
    #[inline]
    pub fn rts<P, T>(self, rts: T) -> Pads<S, RX, TX, CTS, Pad<S, P, T::Id>>
    where
        P: PadNum,
        T: AnyPin,
        T::Id: Map<S, P>,
        Pad<S, P, T::Id>: From<SpecificPin<T>>,
    {
        Pads {
            sercom: self.sercom,
            rx: self.rx,
            tx: self.tx,
            cts: self.cts,
            rts: rts.into().into(),
        }
    }

    /// Consume the [`Pads`] struct and free the individual [`Pad`]s
    #[inline]
    pub fn free(self) -> (RX, TX, CTS, RTS) {
        (self.rx, self.tx, self.cts, self.rts)
    }
}

/// Create an alias for a [`Pads`] type
///
/// Because it takes five type parameters, fully specifying a [`Pads`] type is
/// tedious and error-prone. In normal code, the type parameters can usually be
/// inferred. But some cases, like `static` variables, cannot use inference. In
/// these cases, the [`pads_alias`] macro can make the process easier.
///
/// A normal [`Pads`] alias declaration might look like this:
///
/// ```
/// use atsamd_hal::sercom::v2::Sercom0;
/// use atsamd_hal::sercom::v2::pads::{Pad, Pad1, Pad2, Pad3};
/// use atsamd_hal::sercom::v2::uart;
/// use atsamd_hal::gpio::v2::pins::{PA09, PA10, PA11};
/// use atsamd_hal::typelevel::NoneT;
///
/// pub type Alias = uart::Pads<
///     Sercom0,
///     Pad<Sercom0, Pad1, PA09>,
///     Pad<Sercom0, Pad0, PA08,
///     NoneT,
///     NoneT,
/// >;
/// ```
///
/// There is a lot of repetition and room for error in this declaration. The
/// [`pads_alias`] macro simplifies this example to:
///
/// ```
/// use atsamd_hal::pads_alias;
///
/// pads_alias!(pub type Alias = Pads<
///     Sercom0,
///     RX = (Pad1, PA09),
///     TX = (Pad0, PA08)
/// >);
/// ```
///
/// The arguments specify a [`PadNum`] and [`PinId`] for the Rx and Tx
/// No argument is provided for `CTS` or RTS, so the CTS and RTS [`Pad`] types
/// will be set to [`NoneT`].
///
/// The `RX`, `TX`, `CTS` and `RTS` arguments are all optional. If a
/// corresponding [`PadNum`] is not given, the respective [`Pad`] type will be
/// [`NoneT`]. The [`PadNum`] arguments must always be specified in the
/// indicated order: `RX`, `TX`, `CTS`, `RTS`.
///
/// To be accepted by the [`Config`] struct, the [`Pads`] must implement
/// [`RxpoTxpo`], which requires [`SomePad`] for at least one of `RX`
/// or `TX`.
///
/// [`PinId`]: crate::gpio::v2::pin::PinId
#[macro_export]
macro_rules! uart_pads_alias {
    (
        $vis:vis type $Name:ident = Pads<
            $Sercom:ident
            $(, RX = ($RX_PadNum:ident, $TX_Id:ident))?
            $(, TX = ($TX_PadNum:ident, $RX_Id:ident))?
            $(, CTS = ($CTS_PadNum:ident, $CTS_Id:ident))?
            $(, RTS = ($RTS_PadNum:ident, $RTS_Id:ident))?
        >
    ) => {
        $vis type $Name = $crate::sercom::v2::uart::Pads<
            $crate::sercom::v2::$Sercom,
            __uart_pad_type!($($Sercom, $RX_PadNum, $RX_Id)?),
            __uart_pad_type!($($Sercom, $TX_PadNum, $TX_Id)?),
            __uart_pad_type!($($Sercom, $RTS_PadNum, $CTS_Id)?),
            __uart_pad_type!($($Sercom, $CTS_PadNum, $RTS_Id)?),
        >;
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! __uart_pad_type {
    () => { NoneT };
    ($Sercom:ident, $PadNum:ident, $Id:ident) => {
        $crate::sercom::v2::pads::Pad<
            $crate::sercom::v2::$Sercom,
            $crate::sercom::v2::pads::$PadNum,
            $crate::gpio::v2::pin::$Id
        >
    };
}

//=============================================================================
// AnyPads
//=============================================================================

/// Meta-type representing any set of [`Pads`]
///
/// This trait is used as an interface between the [`Pads`] type and other
/// types in this module. It serves to cut down on the total number of type
/// parameters needed in the [`Config`] struct. The [`Config`] struct doesn't
/// need access to the [`Pad`]s directly. Rather, it only needs to apply the
/// [`SomePad`] trait bound when a [`Pad`] is required. The [`AnyPads`] trait
/// allows each [`Config`] struct to store an instance of [`Pads`] without
/// itself being generic over each [`Pad`] type.
///
/// Like other `Any*` types in this HAL, the [`SpecificPads`] type can be
/// recovered using the [`Into`], [`AsRef`] and [`AsMut`] traits. However, there
/// is unlikely to be a situation where that is useful for the [`Pads`] type.
pub trait AnyPads: Sealed + Is<Type = SpecificPads<Self>> {
    /// [`Sercom`] of the corresponding [`Pads`]
    type Sercom: Sercom;

    /// Rx [`Pad`] from the corresponding [`Pads`]
    type Rx: OptionalPad;

    /// Tx [`Pad`] from the corresponding [`Pads`]
    type Tx: OptionalPad;

    /// CTS [`Pad`] from the corresponding [`Pads`]
    type Cts: OptionalPad;

    /// RTS [`Pad`] from the corresponding [`Pads`]
    type Rts: OptionalPad;
}

/// Type alias to recover the specific [`Pads`] type from an implementation of
/// [`AnyPads`]
pub type SpecificPads<P> = Pads<
    <P as AnyPads>::Sercom,
    <P as AnyPads>::Rx,
    <P as AnyPads>::Tx,
    <P as AnyPads>::Cts,
    <P as AnyPads>::Rts,
>;

/// Type alias to recover the [`Sercom`] type from an implementation of
/// [`AnyPads`]
pub type PadsSercom<P> = <P as AnyPads>::Sercom;

/// Type alias to recover the Rx [`Pad`] type from an implementation of
/// [`AnyPads`]
pub type PadsRx<P> = <P as AnyPads>::Rx;

/// Type alias to recover the Tx [`Pad`] type from an implementation of
/// [`AnyPads`]
pub type PadsTx<P> = <P as AnyPads>::Tx;

/// Type alias to recover the CTS [`Pad`] type from an implementation of
/// [`AnyPads`]
pub type PadsCts<P> = <P as AnyPads>::Cts;

/// Type alias to recover the Rts [`Pad`] type from an implementation of
/// [`AnyPads`]
pub type PadsRts<P> = <P as AnyPads>::Rts;

impl<S, RX, TX, CTS, RTS> Sealed for Pads<S, RX, TX, CTS, RTS>
where
    S: Sercom,
    RX: OptionalPad,
    TX: OptionalPad,
    CTS: OptionalPad,
    RTS: OptionalPad,
{
}

impl<S, RX, TX, CTS, RTS> AnyPads for Pads<S, RX, TX, CTS, RTS>
where
    S: Sercom,
    RX: OptionalPad,
    TX: OptionalPad,
    CTS: OptionalPad,
    RTS: OptionalPad,
{
    type Sercom = S;
    type Rx = RX;
    type Tx = TX;
    type Cts = CTS;
    type Rts = RTS;
}

/// Implementation required to satisfy the `Is<Type = SpecificPads<Self>>` bound
/// on [`AnyPads`]
impl<P: AnyPads> AsRef<P> for SpecificPads<P> {
    #[inline]
    fn as_ref(&self) -> &P {
        // SAFETY: This is guaranteed to be safe, because P == SpecificPads<P>
        unsafe { transmute(self) }
    }
}

/// Implementation required to satisfy the `Is<Type = SpecificPads<Self>>` bound
/// on [`AnyPads`]
impl<P: AnyPads> AsMut<P> for SpecificPads<P> {
    #[inline]
    fn as_mut(&mut self) -> &mut P {
        // SAFETY: This is guaranteed to be safe, because P == SpecificPads<P>
        unsafe { transmute(self) }
    }
}

//=============================================================================
// Tx/Rx
//=============================================================================

/// Marker trait for a set of [`Pads`] that can transmit
///
/// To transmit, Tx must be [`SomePad`].
pub trait Tx: AnyPads {}

impl<P> Tx for P
where
    P: AnyPads,
    P::Tx: SomePad,
{
}

/// Marker trait for a set of [`Pads`] that can receive
///
/// To receive, Rx must be [`SomePad`].
pub trait Rx: AnyPads {}

impl<P> Rx for P
where
    P: AnyPads,
    P::Rx: SomePad,
{
}

/// Marker trait for a set of [`Pads`] that cannot transmit
///
/// A set of [`Pads`] cannot be used to transmit when the Tx [`Pad`] is
/// [`NoneT`].
pub trait NotTx: AnyPads {}

impl<P> NotTx for P where P: AnyPads<Tx = NoneT> {}

/// Marker trait for a set of [`Pads`] that cannot receive
///
/// A set of [`Pads`] cannot be used to receive when the Rx [`Pad`] is
/// [`NoneT`].
pub trait NotRx: AnyPads {}

impl<P> NotRx for P where P: AnyPads<Rx = NoneT> {}

/// Marker trait for a set of [`Pads`] that can transmit OR receive
///
/// To satisfy this trait, one or both of
/// Rx and Tx must be [`SomePad`].
pub trait TxOrRx: AnyPads {}

impl<S, RX, CTS, RTS> TxOrRx for Pads<S, RX, NoneT, CTS, RTS>
where
    S: Sercom,
    RX: SomePad,
    CTS: SomePad,
    RTS: OptionalPad,
{
}

impl<S, TX, CTS, RTS> TxOrRx for Pads<S, NoneT, TX, CTS, RTS>
where
    S: Sercom,
    TX: SomePad,
    CTS: SomePad,
    RTS: OptionalPad,
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

bitflags! {
    /// Interrupt bit flags for UART transactions
    ///
    /// The available interrupt flags are `DRE`, `RXC`, `TXC`, `RXS`, `CTSIC`, `RXBRK` and
    /// `ERROR`. The binary format of the underlying bits exactly matches the
    /// INTFLAG register.
    pub struct Flags: u8 {
        const DRE = 0x01;
        const RXC = 0x02;
        const TXC = 0x04;
        const RXS = 0x08;
        const CTSIC = 0x10;
        const RXBRK = 0x20;
        const ERROR = 0x80;
    }
}

//=============================================================================
// Errors
//=============================================================================

bitflags! {
    /// Status flags for UART transactions
    ///
    /// The available error flags are `PERR`, `FERR`, `BUFOVF`, `CTS`, `ISF` and `COLL`.
    /// The binary format of the underlying bits exactly matches the STATUS register.
    pub struct Status: u16 {
        const PERR = 0x0001;
        const FERR = 0x0002;
        const BUFOVF = 0x0004;
        const CTS = 0x0008;
        const ISF = 0x0010;
        const COLL = 0x0020;
    }
}

/// Error `enum` for UART transactions

#[derive(Debug)]
pub enum Error {
    ParityError,
    FrameError,
    Overflow,
    InconsistentSyncField,
    CollisionDetected,
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
    P: RxpoTxpo,
    C: CharSize,
{
    sercom: P::Sercom,
    pads: P,
    chsize: PhantomData<C>,
    freq: Hertz,
}

impl<P: RxpoTxpo> Config<P> {
    /// Create a new [`Config`] in the default configuration
    fn create(sercom: P::Sercom, pads: P, freq: impl Into<Hertz>) -> Self {
        Self::swrst(&sercom);
        P::configure(&sercom);
        EightBit::configure(&sercom);
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
    pub fn new(pm: &PM, mut sercom: P::Sercom, pads: P, freq: impl Into<Hertz>) -> Self {
        sercom.enable_apb_clock(pm);
        Self::create(sercom, pads, freq)
    }
}

impl<P, C> Config<P, C>
where
    P: RxpoTxpo,
    C: CharSize,
{
    /// Reset the SERCOM peripheral
    #[inline]
    fn swrst(sercom: &P::Sercom) {
        sercom.usart().ctrla.write(|w| w.swrst().set_bit());
        while sercom.usart().syncbusy.read().swrst().bit_is_set() {}
    }

    /// Change the [`Config`] [`Mode`] or [`CharSize`]
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
    pub fn parity(self, parity: Option<ParityMode>) -> Self {
        // Use only the first two available settings in the FORM field.
        // Ignore auto-baud options.
        let enabled = if let Some(p) = parity {
            let odd = match p {
                ParityMode::Even => false,
                ParityMode::Odd => true,
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
    /// GCLK frequency/16. Values outside this range will saturate at the
    /// maximum supported baud rate.
    ///
    /// Note that currently, only 16x oversampling, asynchronous arithmetic mode
    /// is supported.
    #[inline]
    pub fn baud<B: Into<Hertz>>(self, baud: B) -> Self {
        let baud: Hertz = baud.into();

        // Use 16x oversampling, asynchronous arithmetic mode
        const N_SAMPLES: u8 = 16;
        self.sercom
            .usart()
            .ctrla
            .modify(|_, w| unsafe { w.sampr().bits(0) });

        let baud = Self::calculate_baud_asynchronous_arithm(baud.0, self.freq.0, N_SAMPLES);
        self.sercom
            .usart()
            .baud()
            .modify(|_, w| unsafe { w.baud().bits(baud) });
        self
    }

    #[inline]
    /// Calculate baudrate value using the asynchronous arithmetic method (Table
    /// 24-2)
    fn calculate_baud_asynchronous_arithm(baudrate: u32, clk_freq: u32, n_samples: u8) -> u16 {
        const SHIFT: u8 = 32;
        let sample_rate = (n_samples as u64 * baudrate as u64) << 32;
        let ratio = sample_rate / clk_freq as u64;
        let scale = (1u64 << SHIFT) - ratio;
        let baud_calculated = (65536u64 * scale) >> SHIFT;

        baud_calculated as u16
    }

    /// Control the buffer overflow notification
    ///
    /// If set to true, an [`Error::Overflow`] will be issued as soon as an
    /// overflow occurs. Otherwise, it will not be issued until its place within
    /// the data stream.
    #[inline]
    pub fn immediate_overflow_notification(&mut self, set: bool) {
        self.sercom.usart().ctrla.modify(|_, w| w.ibon().bit(set));
    }

    /// Run in standby mode
    ///
    /// When set, the UART peripheral will run in standby mode. See the
    /// datasheet for more details.
    #[inline]
    pub fn run_in_standby(&mut self, set: bool) {
        self.sercom
            .usart()
            .ctrla
            .modify(|_, w| w.runstdby().bit(set));
    }

    /// Enable interrupts for the specified flags
    pub fn enable_interrupts(&mut self, flags: Flags) {
        self.sercom
            .usart()
            .intenset
            .write(|w| unsafe { w.bits(flags.bits()) });
    }

    /// Disable interrupts for the specified flags
    pub fn disable_interrupts(&mut self, flags: Flags) {
        self.sercom
            .usart()
            .intenclr
            .write(|w| unsafe { w.bits(flags.bits()) });
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
        self.sercom.usart().ctrlb.modify(|_, w| w.rxen().set_bit());
        while self.sercom.usart().syncbusy.read().ctrlb().bit_is_set() {}

        // Enable TX
        self.sercom.usart().ctrlb.modify(|_, w| w.txen().set_bit());
        while self.sercom.usart().syncbusy.read().ctrlb().bit_is_set() {}

        // Globally enable peripheral
        self.sercom
            .usart()
            .ctrla
            .modify(|_, w| w.enable().set_bit());
        while self.sercom.usart().syncbusy.read().enable().bit_is_set() {}

        Uart { config: self }
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
}

//=============================================================================
// AnyConfig
//=============================================================================

/// Meta-type representing any [`Config`]
///
/// All instances of [`Config`] implement this trait. When used as a trait
/// bound, it acts to encapsulate a [`Config`]. Without this trait, a
/// completely generic [`Config`] requires two type parameters, i.e.
/// `Config<P, C>`. But when using this trait, only one type parameter is
/// required, i.e. `C: AnyConfig`. However, even though we have dropped type
/// parameters, no information is lost, because the [`Pads`] and
/// [`CharSize`] type parameters are stored as associated types in the trait.
/// The implementation of [`AnyConfig`] looks like this:
///
/// ```
/// impl<P: Pads, C: CharSize> AnyConfig for Config<P, C> {
///     type Pads = P;
///     type CharSize = C;
/// }
/// ```
///
/// Thus, there is a one-to-one mapping between `Config<P, C>` and
/// `AnyConfig<Pads = P, CharSize = C>`, so you can always recover the
/// specific [`Config`] type from an implementation of [`AnyConfig`]. The type
/// alias [`SpecificConfig`] is provided for this purpose. You can convert
/// between [`AnyConfig`] and its corresponding [`SpecificConfig`] using the
/// [`Into`], [`AsRef`] and [`AsMut`] traits.
pub trait AnyConfig: Sealed + Is<Type = SpecificConfig<Self>> {
    type Pads: RxpoTxpo;
    type CharSize: CharSize;
}

/// Type alias to recover the specific [`Config`] type from an implementation of
/// [`AnyConfig`]
pub type SpecificConfig<C> = Config<<C as AnyConfig>::Pads, <C as AnyConfig>::CharSize>;

/// Type alias to recover the [`Pads`] type from an implementation of
/// [`AnyConfig`]
pub type UartPads<C> = <C as AnyConfig>::Pads;

/// Type alias to recover the [`CharSize`] type from an implementation of
/// [`AnyConfig`]
pub type UartCharSize<C> = <C as AnyConfig>::CharSize;

/// Type alias to recover the [`Pads`]' [`Sercom`] type from an implementation
/// of [`AnyConfig`]
pub type UartSercom<C> = PadsSercom<UartPads<C>>;

/// Type alias to recover the [`Pads`]' Rx [`Pad`] type from an
/// implementation of [`AnyConfig`]
pub type UartRx<C> = PadsRx<UartPads<C>>;

/// Type alias to recover the [`Pads`]' Tx [`Pad`] type from an
/// implementation of [`AnyConfig`]
pub type UartTx<C> = PadsTx<UartPads<C>>;

/// Type alias to recover the [`Pads`]' CTS [`Pad`] type from an implementation
/// of [`AnyConfig`]
pub type UartCts<C> = PadsCts<UartPads<C>>;

/// Type alias to recover the [`Pads`]' SS [`Pad`] type from an implementation
/// of [`AnyConfig`]
pub type UartRts<C> = PadsRts<UartPads<C>>;

/// Type alias to recover the [`CharSize`]'s [`Word`] type from an
/// implementation of [`AnyConfig`]
pub type UartWord<C> = Word<UartCharSize<C>>;

impl<P, C> Sealed for Config<P, C>
where
    P: RxpoTxpo,
    C: CharSize,
{
}

impl<P, C> AnyConfig for Config<P, C>
where
    P: RxpoTxpo,
    C: CharSize,
{
    type Pads = P;
    type CharSize = C;
}

/// Implementation required to satisfy the `Is<Type = SpecificConfig<Self>>`
/// bound on [`AnyConfig`]
impl<C: AnyConfig> AsRef<C> for SpecificConfig<C> {
    #[inline]
    fn as_ref(&self) -> &C {
        // SAFETY: This is guaranteed to be safe, because C == SpecificConfig<C>
        unsafe { transmute(self) }
    }
}

/// Implementation required to satisfy the `Is<Type = SpecificConfig<Self>>`
/// bound on [`AnyConfig`]
impl<C: AnyConfig> AsMut<C> for SpecificConfig<C> {
    #[inline]
    fn as_mut(&mut self) -> &mut C {
        // SAFETY: This is guaranteed to be safe, because C == SpecificConfig<C>
        unsafe { transmute(self) }
    }
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
    P: RxpoTxpo + TxOrRx,
    C: CharSize,
{
}

//=============================================================================
// Uart
//=============================================================================
/// An enabled UART peripheral that can perform transactions using the embedded
/// HAL traits
///
/// When an [`Uart`] is [`Tx`]` + `[`Rx`], it implements [`Read`] + [`Write`],
/// with a word type that depends on [`CharSize`]. The word type is [`u8`] for
/// an [`EightBit`] [`CharSize`] and [`u16`] for a [`NineBit`] [`CharSize`].
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
pub struct Uart<C: ValidConfig> {
    config: C,
}

impl<C: ValidConfig> Uart<C> {
    /// Obtain a reference to the PAC `SERCOM` struct
    ///
    /// Directly accessing the `SERCOM` could break the invariants of the
    /// type-level tracking in this module, so it is unsafe.
    #[inline]
    pub unsafe fn sercom(&self) -> &UartSercom<C> {
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
        let old_config = unsafe { core::ptr::read(&mut self.config as *const _) };
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

    /// Read the interrupt status flags
    #[inline]
    pub fn read_flags(&self) -> Flags {
        let bits = unsafe { self.sercom().usart().intflag.read().bits() };
        Flags::from_bits_truncate(bits)
    }

    /// Clear interrupt status flags
    ///
    /// Setting the ERROR, SSL or TXC flag will clear the interrupt. Clearing
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
                .usart()
                .intflag
                .write(|w| w.bits(flags.bits()))
        };
    }

    /// Read the error status flags
    #[inline]
    pub fn read_status(&self) -> Status {
        let bits = unsafe { self.sercom().usart().status.read().bits() };
        Status::from_bits_truncate(bits)
    }

    /// Clear error status flags
    ///
    /// Setting a flag will clear the error. Clearing any flag will have no
    /// effect.
    #[inline]
    pub fn clear_errors(&mut self, errors: Status) {
        unsafe {
            self.sercom()
                .usart()
                .status
                .write(|w| w.bits(errors.bits()))
        };
    }

    #[inline]
    pub fn read_flags_errors(&self) -> Result<Flags, Error> {
        self.read_status().try_into()?;
        Ok(self.read_flags())
    }

    /// Read from the DATA register
    ///
    /// Reading from the data register directly is `unsafe`, because it will
    /// clear the RXC flag, which could break assumptions made elsewhere in
    /// this module.
    #[inline]
    pub unsafe fn read_data(&mut self) -> u16 {
        self.sercom().usart().data.read().data().bits()
    }

    /// Write to the DATA register
    ///
    /// Writing to the data register directly is `unsafe`, because it will clear
    /// the DRE flag, which could break assumptions made elsewhere in this
    /// module.
    #[inline]
    pub unsafe fn write_data(&mut self, data: u16) {
        self.sercom().usart().data.write(|w| w.data().bits(data))
    }

    /// Disable the UART peripheral and return the [`Config`] struct
    #[inline]
    pub fn disable(mut self) -> C {
        let usart = unsafe { self.sercom().usart() };
        usart.ctrlb.modify(|_, w| w.rxen().clear_bit());
        while usart.syncbusy.read().ctrlb().bit_is_set() {}
        self.config.as_mut().enable_peripheral(false);
        self.config
    }
}

//=============================================================================
// AnyUart
//=============================================================================

/// Meta-type representing any [`Uart`]
///
/// This trait is implemented for every instance of [`Uart`]. It allows you to
/// restrict a generic type to an [`Uart`] with explicitly naming the [`Uart`]
/// type. Like other `Any*` traits in this HAL, you can recover the specific
/// [`Uart`] type with the type alias [`SpecificUart`], and you can convert
/// between [`AnyUart`] and its corresponding [`SpecificUart`] using the
/// [`Into`], [`AsRef`] and [`AsMut`] traits.
///
/// ```
/// fn example<P: AnyUart>(mut any_uart: P) {
///     let uart_mut: &mut SpecificUart<P> = any_uart.as_mut();
///     let uart_ref: &SpecificUart<P> = any_uart.as_ref();
///     let uart: SpecificUart<P> = any_uart.into();
/// }
/// ```
pub trait AnyUart: Sealed + Is<Type = SpecificUart<Self>> {
    type Config: ValidConfig;
}

/// Type alias to recover the specific [`Uart`] type from an implementation of
/// [`AnyUart`]
pub type SpecificUart<T> = Uart<<T as AnyUart>::Config>;

impl<C: ValidConfig> Sealed for Uart<C> {}

impl<C: ValidConfig> AnyUart for Uart<C> {
    type Config = C;
}

/// Implementation required to satisfy the `Is<Type = SpecificUart<Self>>` bound
/// on [`AnyUart`]
impl<U: AnyUart> AsRef<U> for SpecificUart<U> {
    #[inline]
    fn as_ref(&self) -> &U {
        // SAFETY: This is guaranteed to be safe, because S == SpecificUart<S>
        unsafe { transmute(self) }
    }
}

/// Implementation required to satisfy the `Is<Type = SpecificUart<Self>>` bound
/// on [`AnyUart`]
impl<U: AnyUart> AsMut<U> for SpecificUart<U> {
    #[inline]
    fn as_mut(&mut self) -> &mut U {
        // SAFETY: This is guaranteed to be safe, because S == SpecificUart<S>
        unsafe { transmute(self) }
    }
}

//=============================================================================
// Embedded HAL traits
//=============================================================================

/// Implement [`Read`] for [`Uart`]
///
/// [`Read`] is only implemented when the [`Pads`] are [`Rx`].
///
/// [`Read`] does not have to initiate transactions, so
/// it does not have to store any internal state. It only has to wait on `RXC`.
impl<P, C> Read<C::Word> for Uart<Config<P, C>>
where
    Config<P, C>: ValidConfig,
    P: RxpoTxpo + Rx,
    C: CharSize,
    C::Word: PrimInt,
    u16: AsPrimitive<C::Word>,
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

/// Implement [`Write`] for [`Uart`]
///
/// [`Write`] is only implemented when the [`Pads`] are [`Tx`].
///
/// This implementation never reads the DATA register and ignores all buffer
/// overflow errors.
impl<C> Write<UartWord<C>> for Uart<C>
where
    C: ValidConfig,
    C::Pads: Tx,
    UartWord<C>: PrimInt + AsPrimitive<u16>,
{
    type Error = Error;

    /// Wait for a `DRE` flag, then write a word
    #[inline]
    fn write(&mut self, word: UartWord<C>) -> nb::Result<(), Error> {
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

impl<C> blocking::serial::write::Default<UartWord<C>> for Uart<C>
where
    C: ValidConfig,
    Uart<C>: Write<UartWord<C>>,
{
}

pub enum StopBits {
    OneBit = 0,
    TwoBits = 1,
}

pub enum ParityMode {
    Even = 0,
    Odd = 1,
}
