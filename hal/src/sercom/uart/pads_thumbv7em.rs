//! UART pad definitions for thumbv7em targets

use super::{AnyConfig, Capability, CharSize, Config, Duplex, Rx, Tx};
use crate::{
    gpio::AnyPin,
    sercom::*,
    typelevel::{NoneT, Sealed},
};
use core::marker::PhantomData;

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
}

/// Lift the implementations of [`RxpoTxpo`] from four-tuples of
/// [`OptionalPadNum`]s to the corresponding [`Pads`] types.
impl<S, I, RX, TX, RTS, CTS> RxpoTxpo for Pads<S, I, RX, TX, RTS, CTS>
where
    S: Sercom,
    I: IoSet,
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
macro_rules! impl_rxpotxpo {
    // This is the entry pattern. Start by checking RTS and CTS.
    ($RX:ident, $TX:ident, $RTS:ident, $CTS:ident) => { impl_rxpotxpo!(@check_rts_cts, $RX, $TX, $RTS, $CTS); };

    // Check whether RTS and CTS form a valid pair.
    // They both must be the correct pad or absent.
    (@check_rts_cts, $RX:ident, $TX:ident, NoneT, NoneT) => { impl_rxpotxpo!(@rxpo, $RX, $TX, NoneT, NoneT); };
    (@check_rts_cts, $RX:ident, $TX:ident, Pad2, NoneT) => { impl_rxpotxpo!(@rxpo, $RX, $TX, Pad2, NoneT); };
    (@check_rts_cts, $RX:ident, $TX:ident, NoneT, Pad3) => { impl_rxpotxpo!(@rxpo, $RX, $TX, NoneT, Pad3); };
    (@check_rts_cts, $RX:ident, $TX:ident, Pad2, Pad3) => { impl_rxpotxpo!(@rxpo, $RX, $TX, Pad2, Pad3); };

    // If RTS and CTS are not valid, fall through to this pattern.
    (@check_rts_cts, $RX:ident, $TX:ident, $RTS:ident, $CTS:ident) => { };

    // Assign RXPO based on RX.
    // Our options are exhaustive, so no fall through pattern is needed.
    (@rxpo, NoneT, $TX:ident, $RTS:ident, $CTS:ident) => { impl_rxpotxpo!(@txpo, NoneT, $TX, $RTS, $CTS, 0); };
    (@rxpo, Pad0,  $TX:ident, $RTS:ident, $CTS:ident) => { impl_rxpotxpo!(@txpo, Pad0,  $TX, $RTS, $CTS, 0); };
    (@rxpo, Pad1,  $TX:ident, $RTS:ident, $CTS:ident) => { impl_rxpotxpo!(@txpo, Pad1,  $TX, $RTS, $CTS, 1); };
    (@rxpo, Pad2,  $TX:ident, $RTS:ident, $CTS:ident) => { impl_rxpotxpo!(@txpo, Pad2,  $TX, $RTS, $CTS, 2); };
    (@rxpo, Pad3,  $TX:ident, $RTS:ident, $CTS:ident) => { impl_rxpotxpo!(@txpo, Pad3,  $TX, $RTS, $CTS, 3); };

    // Assign TXPO based on TX, RTS and CTS
    (@txpo, $RX:ident, NoneT, NoneT, NoneT, $RXPO:literal) => { impl_rxpotxpo!(@filter, $RX, NoneT, NoneT, NoneT, $RXPO, 0); };
    (@txpo, $RX:ident, NoneT, Pad2, NoneT, $RXPO:literal) => { impl_rxpotxpo!(@filter, $RX, NoneT, Pad2, NoneT, $RXPO, 3); };
    (@txpo, $RX:ident, NoneT, Pad2, Pad3, $RXPO:literal) => { impl_rxpotxpo!(@filter, $RX, NoneT, Pad2, Pad3, $RXPO, 2); };
    (@txpo, $RX:ident, Pad0, NoneT, NoneT, $RXPO:literal) => { impl_rxpotxpo!(@filter, $RX, Pad0, NoneT, NoneT, $RXPO, 0); };
    (@txpo, $RX:ident, Pad0, Pad2, NoneT, $RXPO:literal) => { impl_rxpotxpo!(@filter, $RX, Pad0, Pad2, NoneT, $RXPO, 3); };
    (@txpo, $RX:ident, Pad0, Pad2, Pad3, $RXPO:literal) => { impl_rxpotxpo!(@filter, $RX, Pad0, Pad2, Pad3, $RXPO, 2); };

    // If TX is not valid, fall through to this pattern.
    (@txpo, $RX:ident, $TX:ident, $RTS:ident, $CTS:ident, $RXPO:literal) => { };

    // Filter any remaining permutations that conflict.
    (@filter, NoneT, NoneT, $RTS:ident, $CTS:ident, $RXPO:literal, $TXPO:literal) => { }; // RX and TX both NoneT
    (@filter, Pad0, Pad0, $RTS:ident, $CTS:ident, $RXPO:literal, $TXPO:literal) => { }; // RX and TX both Pad0
    (@filter, Pad2, $TX:ident, Pad2, $CTS:ident, $RXPO:literal, $TXPO:literal) => { }; // RX can't share a pad with RTS
    (@filter, Pad3, $TX:ident, $RTS:ident, Pad3, $RXPO:literal, $TXPO:literal) => { }; // RX can't share a pad with CTS
    (@filter, Pad1, $TX:ident, $RTS:ident, $CTS:ident, 1, 0) => { }; // RX can't be Pad1 if TXPO is 0 because of XCK conflict
    (@filter, Pad1, $TX:ident, $RTS:ident, $CTS:ident, 1, 3) => { }; // RX can't be Pad1 if TXPO is 3 because of XCK conflict

    // If there are no conflicts, fall through to this pattern
    (@filter, $RX:ident, $TX:ident, $RTS:ident, $CTS:ident, $RXPO:literal, $TXPO:literal) => {
        impl_rxpotxpo!(@implement, $RX, $TX, $RTS, $CTS, $RXPO, $TXPO);
    };

    // Implement RxpoTxpo
    (@implement, $RX:ident, $TX:ident, $RTS:ident, $CTS:ident, $RXPO:literal, $TXPO:literal) => {
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
/// See the [module-level](crate::sercom::uart) documentation for more
/// details on specifying a `Pads` type and creating instances.
pub struct Pads<S, I, RX = NoneT, TX = NoneT, RTS = NoneT, CTS = NoneT>
where
    S: Sercom,
    I: IoSet,
    RX: OptionalPad,
    TX: OptionalPad,
    RTS: OptionalPad,
    CTS: OptionalPad,
{
    sercom: PhantomData<S>,
    ioset: PhantomData<I>,
    receive: RX,
    transmit: TX,
    ready_to_send: RTS,
    clear_to_send: CTS,
}

impl<S: Sercom, I: IoSet> Default for Pads<S, I> {
    fn default() -> Self {
        Self {
            sercom: PhantomData,
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
    RX: OptionalPad,
    TX: OptionalPad,
    RTS: OptionalPad,
    CTS: OptionalPad,
{
    /// Set the `RX` [`Pad`]
    #[inline]
    pub fn rx<Id>(self, pin: impl AnyPin<Id = Id>) -> Pads<S, I, Pad<S, Id>, TX, RTS, CTS>
    where
        Id: GetPad<S>,
        Pad<S, Id>: InIoSet<I>,
    {
        Pads {
            sercom: self.sercom,
            ioset: self.ioset,
            receive: pin.into().into_mode(),
            transmit: self.transmit,
            ready_to_send: self.ready_to_send,
            clear_to_send: self.clear_to_send,
        }
    }

    /// Set the `TX` [`Pad`]
    #[inline]
    pub fn tx<Id>(self, pin: impl AnyPin<Id = Id>) -> Pads<S, I, RX, Pad<S, Id>, RTS, CTS>
    where
        Id: GetPad<S>,
        Pad<S, Id>: InIoSet<I>,
    {
        Pads {
            sercom: self.sercom,
            ioset: self.ioset,
            receive: self.receive,
            transmit: pin.into().into_mode(),
            ready_to_send: self.ready_to_send,
            clear_to_send: self.clear_to_send,
        }
    }

    /// Set the `RTS` [`Pad`], which is always [`Pad2`]
    #[inline]
    pub fn rts<Id>(self, pin: impl AnyPin<Id = Id>) -> Pads<S, I, RX, TX, Pad<S, Id>, CTS>
    where
        Id: GetPad<S>,
        Pad<S, Id>: InIoSet<I>,
    {
        Pads {
            sercom: self.sercom,
            ioset: self.ioset,
            receive: self.receive,
            transmit: self.transmit,
            ready_to_send: pin.into().into_mode(),
            clear_to_send: self.clear_to_send,
        }
    }

    /// Set the `CTS` [`Pad`], which is always [`Pad3`]
    #[inline]
    pub fn cts<Id>(self, pin: impl AnyPin<Id = Id>) -> Pads<S, I, RX, TX, RTS, Pad<S, Id>>
    where
        Id: GetPad<S>,
        Pad<S, Id>: InIoSet<I>,
    {
        Pads {
            sercom: self.sercom,
            ioset: self.ioset,
            receive: self.receive,
            transmit: self.transmit,
            ready_to_send: self.ready_to_send,
            clear_to_send: pin.into().into_mode(),
        }
    }

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

/// Define a set of [`Pads`] using [`PinId`]s instead of [`Pin`]s
///
/// In some cases, it is more convenient to specify a set of `Pads` using
/// `PinId`s rather than `Pin`s. This alias makes it easier to do so.
///
/// The first two type parameters are the [`Sercom`] and [`IoSet`], while the
/// remaining four are effectively [`OptionalPinId`]s representing the
/// corresponding type parameters of [`Pads`], i.e. `RX`, `TX`, `RTS` & `CTS`.
/// Each of the remaining type parameters defaults to [`NoneT`].
///
/// ```
/// use atsamd_hal::pac::Peripherals;
/// use atsamd_hal::gpio::{PA08, PA09, Pins};
/// use atsamd_hal::sercom::{Sercom0, uart};
/// use atsamd_hal::sercom::pad::IoSet1;
/// use atsamd_hal::typelevel::NoneT;
///
/// pub type Pads = uart::PadsFromIds<Sercom0, IoSet1, PA09T, PA08>;
///
/// pub fn create_pads() -> Pads {
///     let peripherals = Peripherals::take().unwrap();
///     let pins = Pins::new(peripherals.PORT);
///     uart::Pads::default().rx(pins.pa09).tx(pins.pa08)
/// }
/// ```
///
/// [`Pin`]: crate::gpio::Pin
/// [`PinId`]: crate::gpio::PinId
/// [`OptionalPinId`]: crate::gpio::OptionalPinId
pub type PadsFromIds<S, I, RX = NoneT, TX = NoneT, RTS = NoneT, CTS = NoneT> = Pads<
    S,
    I,
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
/// corresponding [`Sercom`], and [`OptionalPad`] types. It serves to
/// cut down on the total number of type parameters needed in the [`Config`]
/// struct. The [`Config`] struct doesn't need access to the [`Pad`]s directly.
/// Rather, it only needs to apply the [`SomePad`] trait bound when a `Pin` is
/// required. The [`PadSet`] trait allows each [`Config`] struct to store an
/// instance of [`Pads`] without itself being generic over all six type
/// parameters of the [`Pads`] type.
///
/// [`Pin`]: crate::gpio::Pin
/// [`Config`]: crate::sercom::uart::Config
/// [type-level function]: crate::typelevel#type-level-functions
pub trait PadSet: Sealed {
    type Sercom: Sercom;
    type IoSet: IoSet;
    type Rx: OptionalPad;
    type Tx: OptionalPad;
    type Rts: OptionalPad;
    type Cts: OptionalPad;
}

impl<S, I, RX, TX, RTS, CTS> Sealed for Pads<S, I, RX, TX, RTS, CTS>
where
    S: Sercom,
    I: IoSet,
    RX: OptionalPad,
    TX: OptionalPad,
    RTS: OptionalPad,
    CTS: OptionalPad,
{
}

impl<S, I, RX, TX, RTS, CTS> PadSet for Pads<S, I, RX, TX, RTS, CTS>
where
    S: Sercom,
    I: IoSet,
    RX: OptionalPad,
    TX: OptionalPad,
    RTS: OptionalPad,
    CTS: OptionalPad,
{
    type Sercom = S;
    type IoSet = I;
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
/// This trait labels sets of [`Pads`] that satisfy the [`Rxpo`] and [`Txpo`]
/// traits. It guarantees to the [`Config`] struct that this set of `Pads` can
/// be configured through those traits.
///
/// [`Config`]: crate::sercom::uart::Config
pub trait ValidPads: PadSet + RxpoTxpo {
    type Capability: Capability;
}

impl<S, I, RX, RTS> ValidPads for Pads<S, I, RX, NoneT, RTS, NoneT>
where
    S: Sercom,
    I: IoSet,
    RX: SomePad,
    RTS: OptionalPad,
    Self: PadSet + RxpoTxpo,
{
    type Capability = Rx;
}

impl<S, I, TX, CTS> ValidPads for Pads<S, I, NoneT, TX, NoneT, CTS>
where
    S: Sercom,
    I: IoSet,
    TX: SomePad,
    CTS: OptionalPad,
    Self: PadSet + RxpoTxpo,
{
    type Capability = Tx;
}

impl<S, I, RX, TX, RTS, CTS> ValidPads for Pads<S, I, RX, TX, RTS, CTS>
where
    S: Sercom,
    I: IoSet,
    RX: SomePad,
    TX: SomePad,
    RTS: OptionalPad,
    CTS: OptionalPad,
    Self: PadSet + RxpoTxpo,
{
    type Capability = Duplex;
}

//=============================================================================
// ValidConfig
//=============================================================================

/// Marker trait for valid UART [`Config`]urations
///
/// A functional UART peripheral must have, at a minimum either a Rx or a Tx
/// [`Pad`].
pub trait ValidConfig: AnyConfig {}

impl<P: ValidPads, C: CharSize> ValidConfig for Config<P, C> {}
