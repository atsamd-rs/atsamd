//! Use the SERCOM peripheral for SPI transactions
//!
//! Configuring the SPI peripheral occurs in three steps. First, you must create
//! a set of [`Pads`] for use by the peripheral. Next, you assemble pieces into
//! a [`Config`] struct. After configuring the peripheral, you then [`enable`]
//! it, yielding a functional [`Spi`] struct. Transactions are performed using
//! [`spi`](embedded_hal::spi) and [`serial`](embedded_hal::serial) traits from
//! embedded HAL.
//!
//! # [`Pads`]
//!
//! A [`Sercom`] can use up to four [`Pin`]s as peripheral [`Pad`]s, but only
//! certain [`Pin`] combinations are acceptable. All [`Pin`]s must be mapped to
//! the same [`Sercom`], and only certain [`PinId`]s can be used for specific
//! [`PadNum`]s.
//!
//! This HAL makes it impossible to use invalid [`Pin`]/[`Pad`] combinations.
//! The [`Pads`] struct is responsible for enforcing these constraints. To
//! create a set of [`Pads`], start by specifying the [`Sercom`].
//!
//! ```
//! use atsamd_hal::sercom::v2::{Sercom0, spi};
//!
//! let pads = spi::Pads::<Sercom0>::new();
//! ```
//!
//! Next, specify the [`Pin`]s and their corresponding [`PadNum`]s. Both `v1`
//! and `v2` pin types are accepted here. Each [`PadNum`] can be a used for
//! different purpose, depending on the [`DipoDopo`] configuration. The [`Pads`]
//! struct enforces that the [`Pin`] type matches the specified [`Sercom`] and
//! [`PadNum`].
//!
//! ```
//! use atsamd_hal::target_device::Peripherals;
//! use atsamd_hal::gpio::v2::Pins;
//! use atsamd_hal::sercom::v2::{Sercom0, spi};
//! use atsamd_hal::sercom::v2::pads::{Pad0, Pad1, Pad3};
//!
//! let mut peripherals = Peripherals::take().unwrap();
//! let pins = Pins::new(peripherals.PORT);
//! let pads = spi::Pads::<Sercom0>::new()
//!     .sclk<Pad1, _>(pins.pa09)
//!     .data_in::<Pad0, _>(pins.pa08)
//!     .data_out::<Pad3, _>(pins.pa11);
//! ```
//!
//! Not every [`Pad`] must be specified. Each [`Pad`] within a set of [`Pads`]
//! is actually an [`OptionalPad`]. If a [`Pad`] is unused, it can be left as
//! [`NoneT`]. However, if they are ever going to be used for transactions, the
//! [`Pads`] must satisfy the requirements specified for [`DipoDopo`] and
//! [`ValidConfig`].
//!
//! Sometimes it is necessary to specify the full [`Pads`] type, e.g. in
//! `static` variables. In these cases, the [`pads_alias`] macro can help
//! simplify the declaration.
//!
//! # [`Config`]
//!
//! Use the [`Pads`] struct to create a [`Config`] struct, which represents the
//! SPI peripheral in its disabled state. The [`Config`] takes ownership of both
//! the [`Pads`] and the PAC [`Sercom`] struct. It also takes a reference to the
//! PM, so that it can enable the APB clock, and a frequency to indicate the
//! GCLK configuration. Users are responsible for correctly configuring the
//! GCLK.
//!
//! ```
//! use atsamd_hal::time::U32Ext;
//!
//! let pm = peripherals.PM;
//! let sercom = peripherals.SERCOM0;
//! // Configure GCLK for 10 MHz
//! let freq = 10.mhz();
//! let config = spi::Config::new(&pm, sercom, pads, freq);
//! ```
//!
//! By default, the peripheral is set to use [`Master`] [`Mode`] with an
//! [`EightBit`] [`CharSize`]. The [`Config`] struct uses the builder pattern to
//! configure the peripheral, ending with a call to [`enable`], which consumes
//! the [`Config`] and returns an enabled [`Spi`] peripheral.
//!
//! ```
//! use embedded_hal::spi::MODE_1;
//! use atsamd_hal::sercom::v2::spi::NineBit;
//!
//! let spi = spi::Config::new(&mclk, sercom, pads, freq)
//!     .baud(1.mhz())
//!     .char_size::<NineBit>()
//!     .msb_first(false)
//!     .spi_mode(MODE_1)
//!     .enable();
//! ```
//!
//! # [`Spi`]
//!
//! Only the [`Spi`] struct can actually perform transactions. To do so, use the
//! embedded HAL traits, like [`FullDuplex`], [`Read`] and [`Write`]. See the
//! [`Spi`] documentation for more information about the trait implementations,
//! which vary based on the [`CharSize`] and [`Pads`]. For instance,
//! [`FullDuplex`] is only implemented if the [`Pads`] are both [`Tx`] and
//! [`Rx`], and its word size depends on [`CharSize`].
//!
//! ```
//! use nb::block;
//! use embedded_hal::spi::FullDuplex;
//!
//! block!(spi.send(0x0155));
//! let rcvd: u16 = block!(spi.read());
//! ```
//!
//! [`enable`]: Config::enable
//! [`Pin`]: crate::gpio::v2::pin::Pin
//! [`PinId`]: crate::gpio::v2::pin::PinId

use core::convert::{TryFrom, TryInto};
use core::marker::PhantomData;
use core::mem::transmute;

use bitflags::bitflags;
use embedded_hal::blocking;
use embedded_hal::serial::{Read, Write};
use embedded_hal::spi::{self, FullDuplex};
pub use embedded_hal::spi::{Phase, Polarity, MODE_0, MODE_1, MODE_2, MODE_3};
use nb::Error::WouldBlock;
use num_traits::{AsPrimitive, PrimInt};
use paste::paste;

use crate::target_device as pac;
use pac::sercom0::spi::ctrla::MODE_A;
use pac::sercom0::RegisterBlock;
use pac::PM;

use crate::gpio::v2::{AnyPin, SpecificPin};
use crate::sercom::v2::pads::{Map, Pad0, Pad1, Pad2, Pad3, PadNum};
use crate::sercom::v2::pads::{OptionalPad, Pad, SomePad};
use crate::sercom::v2::Sercom;
use crate::time::Hertz;
use crate::typelevel::{Is, NoneT, Sealed};

//=============================================================================
// Pad configuration
//=============================================================================

/// Configure the `DIPO` and `DOPO` fields based on a set of [`Pads`]
///
/// The `spi` module for the SAMx5x chips splits this trait into separate `Dipo`
/// and `Dopo` traits. In those chips, `SCLK` must always be [`Pad1`] and `SS`
/// must always be [`Pad2`]. Moreover, those chips are restricted by `IOSET`, so
/// knowledge of the `DI` and `DO` [`PadNum`]s is equivalent to knowledge of the
/// corresponding [`PinId`]s.
///
/// The situation is much different for SAMD11 and SAMD21 chips. Here, `SCLK`
/// and `SS` can change [`PadNum`], and there is no concept of `IOSET`, so
/// knowledge of the [`PadNum`] does not translate to knowledge of the
/// corresponding [`PinId`].
///
/// We can make the same statements in terms of types and traits from the
/// [`v2::pads`] module. For SAMx5x chips, the [`Map`] type is the same for
/// all four [`Pad`]s; namely, it is the `IoSet`. Once the [`Sercom`] and
/// `IoSet` are known, the [`PadNum`] is enough to tell you the corresponding
/// [`PinId`], through the [`Map`] trait. But in the SAMD11 and SAMD21 chips,
/// the [`Map`] type is different for each [`Pad`]. In fact, the [`Map`] type
/// for these chips is the [`PinId`] itself.
///
/// The SAMD11 and SAMD21 chips have fewer restrictions, so there are more
/// possible configurations, which makes the implementation of this trait more
/// complicated.
///
/// This trait is implemented on various [`Pads`] types. Although [`Pads`]
/// allows every pad to be an [`OptionalPad`], [`DipoDopo`] has more strict
/// requirements. The `CK` pad and at least one of the `DI` and `DO` pads must
/// always be [`SomePad`]. This constraint is not expressed as part of the
/// [`DipoDopo`] trait bounds; rather, is a practical effect of the various
/// implementations. There can be no meaningful use of a [`Sercom`] without a
/// clock [`Pad`] and at least one data [`Pad`].
///
/// [`v2::pads`]: crate::sercom::v2::pads
/// [`PinId`]: crate::gpio::v2::pin::PinId
pub trait DipoDopo: AnyPads {
    /// `DIPO` field value
    const DIPO: u8;

    /// `DIPO` field value
    const DOPO: u8;

    /// Configure the pad according to [`Self::DIPO`] and [`Self::DOPO`]
    #[inline]
    fn configure(sercom: &RegisterBlock) {
        sercom.spi().ctrla.modify(|_, w| unsafe {
            w.dipo().bits(Self::DIPO);
            w.dopo().bits(Self::DOPO)
        });
    }
}

/// Implement [`DipoDopo`] for different [`PadNum`] combinations
///
/// This macro uses the push-down accumulation method to build an implementation
/// of [`DipoDopo`] for a variable number of specified [`PadNum`]s. See this
/// [link](https://veykril.github.io/tlborm/patterns/push-down-acc.html) for
/// more details on the technique.
macro_rules! impl_dipo_dopo {

    // This is the entry pattern
    (
        ( $($pad:ident),+ ): ($DIPO:literal, $DOPO:literal)
    ) => {
        impl_dipo_dopo!(
            ( $($pad,)+ ): ($DIPO, $DOPO) -> []
        );
    };

    // If the [`Pad`] type is [`NoneT`], then no extra type parameters or trait
    // bounds are needed in the implementation.
    (
        ( NoneT, $($pad:ident,)* ): ($DIPO:tt, $DOPO:tt) -> [ $($body:tt,)* ]
    ) => {
        impl_dipo_dopo!(
            ( $($pad,)* ): ($DIPO, $DOPO) -> [ $($body,)* { (), (NoneT) }, ]
        );
    };


    // To specify a [`Pad`] type with a particular [`PadNum`], you need to add
    // a type parameter for the [`Map`] type and a trait bound to enforce it.
    // Each type parameter must have a unique name, so create one by prepending
    // the [`PadNum`] type with `M`, using the [`paste`] macro. The actual macro
    // is not used until the final pattern.
    (
        ( $PadNum:ident, $($pad:ident,)* ): ($DIPO:tt, $DOPO:tt) -> [ $($body:tt,)* ]
    ) => {
        impl_dipo_dopo!(
            ( $($pad,)* ): ($DIPO, $DOPO) ->
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
        (): ($DIPO:tt, $DOPO:tt) ->
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
            impl<S: Sercom, $( $($Tp)* )+ > DipoDopo for Pads<S, $( $($Ty)+, )+ > {
                const DIPO: u8 = $DIPO;
                const DOPO: u8 = $DOPO;
            }
        }
    };
}

// The commented implementations are redundant, but they are retained for
// completeness, to help verify that all possible combinations have been
// covered. Only combinations with a valid pin for `CK` and either `DI` or `DO`
// have been considered. Other combinations would have no practical use.

impl_dipo_dopo!((Pad0, NoneT, Pad1, Pad2): (0, 0));
impl_dipo_dopo!((Pad0, NoneT, Pad1, NoneT): (0, 0));
impl_dipo_dopo!((NoneT, Pad0, Pad1, Pad2): (0, 0));
impl_dipo_dopo!((NoneT, Pad0, Pad1, NoneT): (0, 0));

impl_dipo_dopo!((Pad0, Pad2, Pad3, Pad1): (0, 1));
impl_dipo_dopo!((Pad0, Pad2, Pad3, NoneT): (0, 1));
impl_dipo_dopo!((Pad0, NoneT, Pad3, Pad1): (0, 1));
impl_dipo_dopo!((Pad0, NoneT, Pad3, NoneT): (0, 1));
impl_dipo_dopo!((NoneT, Pad2, Pad3, Pad1): (0, 1));
impl_dipo_dopo!((NoneT, Pad2, Pad3, NoneT): (0, 1));

impl_dipo_dopo!((Pad0, Pad3, Pad1, Pad2): (0, 2));
impl_dipo_dopo!((Pad0, Pad3, Pad1, NoneT): (0, 2));
//impl_dipo_dopo!((Pad0, NoneT, Pad1, Pad2): (0, 2));
//impl_dipo_dopo!((Pad0, NoneT, Pad1, NoneT): (0, 2));
impl_dipo_dopo!((NoneT, Pad3, Pad1, Pad2): (0, 2));
impl_dipo_dopo!((NoneT, Pad3, Pad1, NoneT): (0, 2));

//impl_dipo_dopo!((Pad0, NoneT, Pad3, Pad1): (0, 3));
//impl_dipo_dopo!((Pad0, NoneT, Pad3, NoneT): (0, 3));
impl_dipo_dopo!((NoneT, Pad0, Pad3, Pad1): (0, 3));
impl_dipo_dopo!((NoneT, Pad0, Pad3, NoneT): (0, 3));

//impl_dipo_dopo!((NoneT, Pad0, Pad1, Pad2): (1, 0));
//impl_dipo_dopo!((NoneT, Pad0, Pad1, NoneT): (1, 0));

impl_dipo_dopo!((Pad1, Pad2, Pad3, NoneT): (1, 1));
impl_dipo_dopo!((Pad1, NoneT, Pad3, NoneT): (1, 1));
//impl_dipo_dopo!((NoneT, Pad2, Pad3, Pad1): (1, 1));
//impl_dipo_dopo!((NoneT, Pad2, Pad3, NoneT): (1, 1));

//impl_dipo_dopo!((NoneT, Pad3, Pad1, Pad2): (1, 2));
//impl_dipo_dopo!((NoneT, Pad3, Pad1, NoneT): (1, 2));

impl_dipo_dopo!((Pad1, Pad0, Pad3, NoneT): (1, 3));
//impl_dipo_dopo!((Pad1, NoneT, Pad3, NoneT): (1, 3));
//impl_dipo_dopo!((NoneT, Pad0, Pad3, Pad1): (1, 3));
//impl_dipo_dopo!((NoneT, Pad0, Pad3, NoneT): (1, 3));

impl_dipo_dopo!((Pad2, Pad0, Pad1, NoneT): (2, 0));
impl_dipo_dopo!((Pad2, NoneT, Pad1, NoneT): (2, 0));
//impl_dipo_dopo!((NoneT, Pad0, Pad1, Pad2): (2, 0));
//impl_dipo_dopo!((NoneT, Pad0, Pad1, NoneT): (2, 0));

impl_dipo_dopo!((Pad2, NoneT, Pad3, Pad1): (2, 1));
impl_dipo_dopo!((Pad2, NoneT, Pad3, NoneT): (2, 1));
//impl_dipo_dopo!((NoneT, Pad2, Pad3, Pad1): (2, 1));
//impl_dipo_dopo!((NoneT, Pad2, Pad3, NoneT): (2, 1));

impl_dipo_dopo!((Pad2, Pad3, Pad1, NoneT): (2, 2));
//impl_dipo_dopo!((Pad2, NoneT, Pad1, NoneT): (2, 2));
//impl_dipo_dopo!((NoneT, Pad3, Pad1, Pad2): (2, 2));
//impl_dipo_dopo!((NoneT, Pad3, Pad1, NoneT): (2, 2));

impl_dipo_dopo!((Pad2, Pad0, Pad3, Pad1): (2, 3));
impl_dipo_dopo!((Pad2, Pad0, Pad3, NoneT): (2, 3));
//impl_dipo_dopo!((Pad2, NoneT, Pad3, Pad1): (2, 3));
//impl_dipo_dopo!((Pad2, NoneT, Pad3, NoneT): (2, 3));
//impl_dipo_dopo!((NoneT, Pad0, Pad3, Pad1): (2, 3));
//impl_dipo_dopo!((NoneT, Pad0, Pad3, NoneT): (2, 3));

impl_dipo_dopo!((Pad3, Pad0, Pad1, Pad2): (3, 0));
impl_dipo_dopo!((Pad3, NoneT, Pad1, Pad2): (3, 0));
impl_dipo_dopo!((Pad3, Pad0, Pad1, NoneT): (3, 0));
impl_dipo_dopo!((Pad3, NoneT, Pad1, NoneT): (3, 0));
//impl_dipo_dopo!((NoneT, Pad0, Pad1, Pad2): (3, 0));
//impl_dipo_dopo!((NoneT, Pad0, Pad1, NoneT): (3, 0));

//impl_dipo_dopo!((NoneT, Pad2, Pad3, Pad1): (3, 1));
//impl_dipo_dopo!((NoneT, Pad2, Pad3, NoneT): (3, 1));

//impl_dipo_dopo!((Pad3, NoneT, Pad1, Pad2): (3, 2));
//impl_dipo_dopo!((Pad3, NoneT, Pad1, NoneT): (3, 2));
//impl_dipo_dopo!((NoneT, Pad3, Pad1, Pad2): (3, 2));
//impl_dipo_dopo!((NoneT, Pad3, Pad1, NoneT): (3, 2));

//impl_dipo_dopo!((NoneT, Pad0, Pad3, Pad1): (3, 3));
//impl_dipo_dopo!((NoneT, Pad0, Pad3, NoneT): (3, 3));

//=============================================================================
// Pads
//=============================================================================

/// Encapsulate the set of pads for an SPI peripheral
///
/// This struct acts to encapsulate up to four [`Pad`]s for use with an SPI
/// peripheral. All of the [`Pad`]s must share the same [`Sercom`]. The four
/// type parameters `DI`, `DO`, `CK` and `SS` represent the respective DI, DO,
/// SCK and SS [`Pad`] types.
///
/// Each pad in this struct is an [`OptionalPad`]. When first initialized, each
/// pad is set to [`NoneT`]. To be accepted as a valid set of [`Pads`] by the
/// [`Config`] struct, the [`Pads`] must implement [`DipoDopo`], which requires
/// the `CK` pad and either the `DI` or `DO` pad to be [`SomePad`].
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
/// The following example corresponds to a set of [`Pads`] with `DIPO = 0` and
/// `DOPO = 2`. It is possible to create a set of [`Pads`] that does not
/// implement [`DipoDopo`], but such [`Pads`] will not be accepted by
/// [`Config`].
///
/// ```no_run
/// # use atsamd_hal::target_device::Peripherals;
/// # use atsamd_hal::gpio::v2::Pins;
/// # use atsamd_hal::sercom::v2::Sercom0;
/// # use atsamd_hal::sercom::v2::pads::{Pad0, Pad1, Pad2, Pad3};
/// # use atsamd_hal::sercom::v2::spi;
/// let mut peripherals = Peripherals::take().unwrap();
/// let pins = Pins::new(peripherals.PORT);
/// let pads = spi::Pads::<Sercom0>::new()
///     .sclk::<Pad1, _>(pins.pb09)
///     .data_in::<Pad0, _>(pins.pb08)
///     .data_out::<Pad3, _>(pins.pb11);
/// ```
///
/// The [`Tx`], [`Rx`], [`NotTx`], [`NotRx`] and [`TxOrRx`] marker traits are
/// implemented only for [`Pad`] combinations reflecting each trait's name.
///
/// [`Pin`]: crate::gpio::v2::pin::Pin
/// [`PinId`]: crate::gpio::v2::pin::PinId
pub struct Pads<S, DI = NoneT, DO = NoneT, CK = NoneT, SS = NoneT>
where
    S: Sercom,
    DI: OptionalPad,
    DO: OptionalPad,
    CK: OptionalPad,
    SS: OptionalPad,
{
    sercom: PhantomData<S>,
    data_in: DI,
    data_out: DO,
    sclk: CK,
    ss: SS,
}

impl<S: Sercom> Pads<S> {
    /// Create a new [`Pads`] struct
    ///
    /// All of the pads are initialized to [`NoneT`]
    #[inline]
    pub fn new() -> Pads<S> {
        Pads {
            sercom: PhantomData,
            data_in: NoneT,
            data_out: NoneT,
            sclk: NoneT,
            ss: NoneT,
        }
    }
}

impl<S, DI, DO, CK, SS> Pads<S, DI, DO, CK, SS>
where
    S: Sercom,
    DI: OptionalPad,
    DO: OptionalPad,
    CK: OptionalPad,
    SS: OptionalPad,
{
    /// Set the `DI` [`Pad`] using [`PadNum`] `P`
    ///
    /// The type parameter `I` represents the GPIO [`Pin`] type. Its
    /// corresponding [`PinId`] will be used as the [`Pad`]'s [`Map`] type.
    ///
    /// [`Pin`]: crate::gpio::v2::pin::Pin
    /// [`PinId`]: crate::gpio::v2::pin::PinId
    #[inline]
    pub fn data_in<P, T>(self, data_in: T) -> Pads<S, Pad<S, P, T::Id>, DO, CK, SS>
    where
        P: PadNum,
        T: AnyPin,
        T::Id: Map<S, P>,
        Pad<S, P, T::Id>: From<SpecificPin<T>>,
    {
        Pads {
            sercom: self.sercom,
            data_in: data_in.into().into(),
            data_out: self.data_out,
            sclk: self.sclk,
            ss: self.ss,
        }
    }

    /// Set the `DO` [`Pad`] using [`PadNum`] `P`
    ///
    /// The type parameter `I` represents the GPIO [`Pin`] type. Its
    /// corresponding [`PinId`] will be used as the [`Pad`]'s [`Map`] type.
    ///
    /// [`Pin`]: crate::gpio::v2::pin::Pin
    /// [`PinId`]: crate::gpio::v2::pin::PinId
    #[inline]
    pub fn data_out<P, T>(self, data_out: T) -> Pads<S, DI, Pad<S, P, T::Id>, CK, SS>
    where
        P: PadNum,
        T: AnyPin,
        T::Id: Map<S, P>,
        Pad<S, P, T::Id>: From<SpecificPin<T>>,
    {
        Pads {
            sercom: self.sercom,
            data_in: self.data_in,
            data_out: data_out.into().into(),
            sclk: self.sclk,
            ss: self.ss,
        }
    }

    /// Set the `SCK` [`Pad`] using [`PadNum`] `P`
    ///
    /// The type parameter `I` represents the GPIO [`Pin`] type. Its
    /// corresponding [`PinId`] will be used as the [`Pad`]'s [`Map`] type.
    ///
    /// [`Pin`]: crate::gpio::v2::pin::Pin
    /// [`PinId`]: crate::gpio::v2::pin::PinId
    #[inline]
    pub fn sclk<P, T>(self, sclk: T) -> Pads<S, DI, DO, Pad<S, P, T::Id>, SS>
    where
        P: PadNum,
        T: AnyPin,
        T::Id: Map<S, P>,
        Pad<S, P, T::Id>: From<SpecificPin<T>>,
    {
        Pads {
            sercom: self.sercom,
            data_in: self.data_in,
            data_out: self.data_out,
            sclk: sclk.into().into(),
            ss: self.ss,
        }
    }

    /// Set the `SS` [`Pad`], which is always [`Pad2`]
    ///
    /// The type parameter `I` represents the GPIO [`Pin`] type. Its
    /// corresponding [`PinId`] will be used as the [`Pad`]'s [`Map`] type.
    ///
    /// [`Pin`]: crate::gpio::v2::pin::Pin
    /// [`PinId`]: crate::gpio::v2::pin::PinId
    #[inline]
    pub fn ss<P, T>(self, ss: T) -> Pads<S, DI, DO, CK, Pad<S, P, T::Id>>
    where
        P: PadNum,
        T: AnyPin,
        T::Id: Map<S, P>,
        Pad<S, P, T::Id>: From<SpecificPin<T>>,
    {
        Pads {
            sercom: self.sercom,
            data_in: self.data_in,
            data_out: self.data_out,
            sclk: self.sclk,
            ss: ss.into().into(),
        }
    }

    /// Consume the [`Pads`] struct and free the individual [`Pad`]s
    #[inline]
    pub fn free(self) -> (DI, DO, CK, SS) {
        (self.data_in, self.data_out, self.sclk, self.ss)
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
/// use atsamd_hal::sercom::v2::spi;
/// use atsamd_hal::gpio::v2::pins::{PA09, PA10, PA11};
/// use atsamd_hal::typelevel::NoneT;
///
/// pub type Alias = spi::Pads<
///     Sercom0,
///     Pad<Sercom0, Pad3, PA11>,
///     NoneT,
///     Pad<Sercom0, Pad1, PA09>,
///     Pad<Sercom0, Pad2, PA10>,
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
///     DI = (Pad3, PA11),
///     CK = (Pad1, PA09),
///     SS = (Pad2, PA10)
/// >);
/// ```
///
/// The arguments specify a [`PadNum`] and [`PinId`] for the Data In, Sclk and
/// SS lines. No argument is provided for `DO`, so the Data Out [`Pad`] type
/// will be set to [`NoneT`].
///
/// The `DI`, `DO`, `CK` and `SS` arguments are all optional. If a corresponding
/// [`PadNum`] is not given, the respective [`Pad`] type will be [`NoneT`]. The
/// [`PadNum`] arguments must always be specified in the indicated order: `DI`,
/// `DO`, `CK`, `SS`.
///
/// To be accepted by the [`Config`] struct, the [`Pads`] must implement
/// [`DipoDopo`], which requires [`SomePad`] for `CK` and at least one of `DI`
/// or `DO`.
///
/// [`PinId`]: crate::gpio::v2::pin::PinId
#[macro_export]
macro_rules! pads_alias {
    (
        $vis:vis type $Name:ident = Pads<
            $Sercom:ident
            $(, DI = ($DI_PadNum:ident, $DI_Id:ident))?
            $(, DO = ($DO_PadNum:ident, $DO_Id:ident))?
            $(, CK = ($CK_PadNum:ident, $CK_Id:ident))?
            $(, SS = ($SS_PadNum:ident, $SS_Id:ident))?
        >
    ) => {
        $vis type $Name = $crate::sercom::v2::spi::Pads<
            $crate::sercom::v2::$Sercom,
            __pad_type!($($Sercom, $DI_PadNum, $DI_Id)?),
            __pad_type!($($Sercom, $DO_PadNum, $DO_Id)?),
            __pad_type!($($Sercom, $CK_PadNum, $CK_Id)?),
            __pad_type!($($Sercom, $SS_PadNum, $SS_Id)?),
        >;
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! __pad_type {
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

    /// Data In [`Pad`] from the corresponding [`Pads`]
    type DataIn: OptionalPad;

    /// Data Out [`Pad`] from the corresponding [`Pads`]
    type DataOut: OptionalPad;

    /// SCLK [`Pad`] from the corresponding [`Pads`]
    type Sclk: OptionalPad;

    /// SS [`Pad`] from the corresponding [`Pads`]
    type SS: OptionalPad;
}

/// Type alias to recover the specific [`Pads`] type from an implementation of
/// [`AnyPads`]
pub type SpecificPads<P> = Pads<
    <P as AnyPads>::Sercom,
    <P as AnyPads>::DataIn,
    <P as AnyPads>::DataOut,
    <P as AnyPads>::Sclk,
    <P as AnyPads>::SS,
>;

/// Type alias to recover the [`Sercom`] type from an implementation of
/// [`AnyPads`]
pub type PadsSercom<P> = <P as AnyPads>::Sercom;

/// Type alias to recover the Data In [`Pad`] type from an implementation of
/// [`AnyPads`]
pub type PadsDataIn<P> = <P as AnyPads>::DataIn;

/// Type alias to recover the Data Out [`Pad`] type from an implementation of
/// [`AnyPads`]
pub type PadsDataOut<P> = <P as AnyPads>::DataOut;

/// Type alias to recover the SCK [`Pad`] type from an implementation of
/// [`AnyPads`]
pub type PadsSclk<P> = <P as AnyPads>::Sclk;

/// Type alias to recover the SS [`Pad`] type from an implementation of
/// [`AnyPads`]
pub type PadsSS<P> = <P as AnyPads>::SS;

impl<S, DI, DO, CK, SS> Sealed for Pads<S, DI, DO, CK, SS>
where
    S: Sercom,
    DI: OptionalPad,
    DO: OptionalPad,
    CK: OptionalPad,
    SS: OptionalPad,
{
}

impl<S, DI, DO, CK, SS> AnyPads for Pads<S, DI, DO, CK, SS>
where
    S: Sercom,
    DI: OptionalPad,
    DO: OptionalPad,
    CK: OptionalPad,
    SS: OptionalPad,
{
    type Sercom = S;
    type DataIn = DI;
    type DataOut = DO;
    type Sclk = CK;
    type SS = SS;
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
/// To transmit, both SCLK and Data Out must be [`SomePad`].
pub trait Tx: AnyPads {}

impl<P> Tx for P
where
    P: AnyPads,
    P::DataOut: SomePad,
    P::Sclk: SomePad,
{
}

/// Marker trait for a set of [`Pads`] that can receive
///
/// To receive, both SCLK and Data In must be [`SomePad`].
pub trait Rx: AnyPads {}

impl<P> Rx for P
where
    P: AnyPads,
    P::DataIn: SomePad,
    P::Sclk: SomePad,
{
}

/// Marker trait for a set of [`Pads`] that cannot transmit
///
/// A set of [`Pads`] cannot be used to transmit when the Data Out [`Pad`] is
/// [`NoneT`].
pub trait NotTx: AnyPads {}

impl<P> NotTx for P where P: AnyPads<DataOut = NoneT> {}

/// Marker trait for a set of [`Pads`] that cannot receive
///
/// A set of [`Pads`] cannot be used to receive when the Data In [`Pad`] is
/// [`NoneT`].
pub trait NotRx: AnyPads {}

impl<P> NotRx for P where P: AnyPads<DataIn = NoneT> {}

/// Marker trait for a set of [`Pads`] that can transmit OR receive
///
/// To satisfy this trait, SCLK must always be [`SomePad`] and one or both of
/// Data In and Data Out must also be [`SomePad`].
pub trait TxOrRx: AnyPads {}

impl<S, DI, CK, SS> TxOrRx for Pads<S, DI, NoneT, CK, SS>
where
    S: Sercom,
    DI: SomePad,
    CK: SomePad,
    SS: OptionalPad,
{
}

impl<S, DO, CK, SS> TxOrRx for Pads<S, NoneT, DO, CK, SS>
where
    S: Sercom,
    DO: SomePad,
    CK: SomePad,
    SS: OptionalPad,
{
}

impl<P: Tx + Rx> TxOrRx for P {}

//=============================================================================
// Operating mode
//=============================================================================

/// Type-level `enum` representing the SPI operating mode
///
/// The available operating modes are [`Master`], [`MasterHWSS`] and [`Slave`].
/// In [`Master`] mode, the `SS` signal must be handled by the user, so `SS`
/// is an [`OptionalPad`]. In [`MasterHWSS`] mode, the hardware drives the `SS`
/// line, so [`SomePad`] is required. In [`Slave`] mode, the `SS` [`Pad`] is
/// required as well, to indicate when data is valid.
pub trait Mode: Sealed {
    /// Corresponding variant from the PAC enum
    const MODE: MODE_A;

    /// Bit indicating whether hardware `SS` control is enabled
    const MSSEN: bool;

    /// Configure the SPI operating mode
    #[inline]
    fn configure(sercom: &RegisterBlock) -> () {
        sercom
            .spi()
            .ctrla
            .modify(|_, w| w.mode().variant(Self::MODE));
        sercom.spi().ctrlb.modify(|_, w| w.mssen().bit(Self::MSSEN));
        while sercom.spi().syncbusy.read().ctrlb().bit_is_set() {}
    }
}

/// [`Mode`] variant for Master mode
pub enum Master {}

/// [`Mode`] variant for Master mode with hardware-controlled slave select
pub enum MasterHWSS {}

/// [`Mode`] variant for Slave mode
pub enum Slave {}

impl Sealed for Master {}
impl Sealed for MasterHWSS {}
impl Sealed for Slave {}

impl Mode for Master {
    const MODE: MODE_A = MODE_A::SPI_MASTER;
    const MSSEN: bool = false;
}

impl Mode for MasterHWSS {
    const MODE: MODE_A = MODE_A::SPI_MASTER;
    const MSSEN: bool = true;
}

impl Mode for Slave {
    const MODE: MODE_A = MODE_A::SPI_SLAVE;
    const MSSEN: bool = false;
}

/// Marker trait for Master operating modes
///
/// This trait is implemented for [`Master`] and [`MasterHWSS`] but not for
/// [`Slave`].
pub trait MasterMode: Mode {}

impl MasterMode for Master {}
impl MasterMode for MasterHWSS {}

//=============================================================================
// Character size
//=============================================================================

/// Type-level `enum` representing the SPI character size
///
/// The SPI character size affects the word size for the embedded HAL traits.
/// Eight-bit transactions use a `u8` word, while nine-bit transactions use a
/// `u16` word.
pub trait CharSize: Sealed {
    /// Word size for the character size
    type Word: 'static;

    /// TODO
    const BITS: u8;

    /// Configure the `LENGTH` register and enable the `LENGTH` counter
    #[inline]
    fn configure(sercom: &RegisterBlock) -> () {
        sercom
            .spi()
            .ctrlb
            .modify(|_, w| unsafe { w.chsize().bits(Self::BITS) });
    }
}

/// Type alias to recover the `Word` type from an implementation of [`CharSize`]
pub type Word<C> = <C as CharSize>::Word;

/// [`CharSize`] variant for 8-bit transactions
pub enum EightBit {}

/// [`CharSize`] variant for 9-bit transactions
pub enum NineBit {}

impl Sealed for EightBit {}
impl CharSize for EightBit {
    type Word = u8;
    const BITS: u8 = 0;
}

impl Sealed for NineBit {}
impl CharSize for NineBit {
    type Word = u16;
    const BITS: u8 = 1;
}

//=============================================================================
// Flags
//=============================================================================

bitflags! {
    /// Interrupt bit flags for SPI transactions
    ///
    /// The available interrupt flags are `DRE`, `RXC`, `TXC`, `SSL` and
    /// `ERROR`. The binary format of the underlying bits exactly matches the
    /// INTFLAG register.
    pub struct Flags: u8 {
        const DRE = 0x01;
        const RXC = 0x02;
        const TXC = 0x04;
        const SSL = 0x08;
        const ERROR = 0x80;
    }
}

//=============================================================================
// Errors
//=============================================================================

bitflags! {
    /// Error bit flags for SPI transactions
    ///
    /// `BUFOVF` is the only available error flag. The binary format of the
    /// underlying bits exactly matches the STATUS register.
    pub struct Errors: u16 {
        const BUFOVF = 0x0004;
    }
}

/// Error `enum` for SPI transactions
///
/// The SPI peripheral only has one error type: buffer overflow.
#[derive(Debug)]
pub enum Error {
    Overflow,
}

impl TryFrom<Errors> for () {
    type Error = Error;
    fn try_from(errors: Errors) -> Result<(), Error> {
        if errors.contains(Errors::BUFOVF) {
            Err(Error::Overflow)
        } else {
            Ok(())
        }
    }
}

//=============================================================================
// Config
//=============================================================================

/// A configurable, disabled SPI peripheral
///
/// This `struct` represents a configurable SPI peripheral in its disabled
/// state. It is generic over the set of [`Pads`], operating [`Mode`] and
/// [`CharSize`]. Upon creation, the [`Config`] takes ownership of the
/// [`Sercom`] and resets it, returning it configured as an SPI peripheral in
/// [`Master`] [`Mode`] with an [`EightBit`] [`CharSize`].
///
/// [`Config`] uses a builder-pattern API to configure the peripheral,
/// culminating in a call to [`enable`], which consumes the [`Config`] and
/// returns an enabled [`Spi`] peripheral. The [`enable`] function is restricted
/// to [`ValidConfig`]s.
///
/// [`enable`]: Config::enable
pub struct Config<P, M = Master, C = EightBit>
where
    P: DipoDopo,
    M: Mode,
    C: CharSize,
{
    sercom: P::Sercom,
    pads: P,
    mode: PhantomData<M>,
    chsize: PhantomData<C>,
    freq: Hertz,
}

impl<P: DipoDopo> Config<P> {
    /// Create a new [`Config`] in the default configuration
    fn create(sercom: P::Sercom, pads: P, freq: impl Into<Hertz>) -> Self {
        Self::swrst(&sercom);
        Master::configure(&sercom);
        P::configure(&sercom);
        EightBit::configure(&sercom);
        Self {
            sercom,
            pads,
            mode: PhantomData,
            chsize: PhantomData,
            freq: freq.into(),
        }
    }

    /// Create a new [`Config`] in the default configuration
    ///
    /// This function will enable the corresponding APB clock, reset the
    /// [`Sercom`] peripheral, and return a [`Config`] in the default
    /// configuration, [`Master`] [`Mode`] with an [`EightBit`] [`CharSize`].
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

impl<P, M, C> Config<P, M, C>
where
    P: DipoDopo,
    M: Mode,
    C: CharSize,
{
    /// Reset the SERCOM peripheral
    #[inline]
    fn swrst(sercom: &P::Sercom) {
        sercom.spi().ctrla.write(|w| w.swrst().set_bit());
        while sercom.spi().syncbusy.read().swrst().bit_is_set() {}
        unsafe { Self::reset_serial_read_state() };
    }

    /// Reset the `static` state used by the implementation of [`Read`]
    ///
    /// The state should be reset whenever the peripheral is reset or disabled.
    /// It should not be possible for the state to become out of sync from safe
    /// code.
    unsafe fn reset_serial_read_state() {
        SERIAL_READ_STATE[P::Sercom::NUM] = false;
    }

    /// Change the [`Config`] [`Mode`] or [`CharSize`]
    #[inline]
    fn change<M2, C2>(self) -> Config<P, M2, C2>
    where
        M2: Mode,
        C2: CharSize,
    {
        Config {
            sercom: self.sercom,
            pads: self.pads,
            mode: PhantomData,
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

    /// Change the operating [`Mode`]
    #[inline]
    pub fn op_mode<M2: Mode>(self) -> Config<P, M2, C> {
        M2::configure(&self.sercom);
        self.change()
    }

    /// Change the [`CharSize`]
    #[inline]
    pub fn char_size<C2: CharSize>(self) -> Config<P, M, C2> {
        C2::configure(&self.sercom);
        self.change()
    }

    /// Change the clock polarity
    #[inline]
    pub fn cpol(self, cpol: Polarity) -> Self {
        let cpol = match cpol {
            Polarity::IdleLow => false,
            Polarity::IdleHigh => true,
        };
        self.sercom.spi().ctrla.modify(|_, w| w.cpol().bit(cpol));
        self
    }

    /// Change the clock phase
    #[inline]
    pub fn cpha(self, cpha: Phase) -> Self {
        let cpha = match cpha {
            Phase::CaptureOnFirstTransition => false,
            Phase::CaptureOnSecondTransition => true,
        };
        self.sercom.spi().ctrla.modify(|_, w| w.cpha().bit(cpha));
        self
    }

    /// Change the SPI mode (clock polarity & phase)
    #[inline]
    pub fn spi_mode(self, mode: spi::Mode) -> Self {
        let cpol = match mode.polarity {
            Polarity::IdleLow => false,
            Polarity::IdleHigh => true,
        };
        let cpha = match mode.phase {
            Phase::CaptureOnFirstTransition => false,
            Phase::CaptureOnSecondTransition => true,
        };
        self.sercom.spi().ctrla.modify(|_, w| {
            w.cpol().bit(cpol);
            w.cpha().bit(cpha)
        });
        self
    }

    /// Change the bit order of transmission (MSB/LSB first)
    #[inline]
    pub fn msb_first(self, msb_first: bool) -> Self {
        self.sercom
            .spi()
            .ctrla
            .modify(|_, w| w.dord().bit(!msb_first));
        self
    }

    /// Set the baud rate
    ///
    /// This function will calculate the best BAUD register setting based on the
    /// stored GCLK frequency and desired baud rate. The maximum baud rate is
    /// half the GCLK frequency. The minimum baud rate is the GCLK frequency /
    /// 512. Values outside this range will saturate at the extremes.
    #[inline]
    pub fn baud<B: Into<Hertz>>(self, baud: B) -> Self {
        let baud: Hertz = baud.into();
        let baud = (self.freq.0 / 2 / baud.0).saturating_sub(1);
        let baud = if baud <= u8::MAX as u32 {
            baud as u8
        } else {
            u8::MAX
        };
        self.sercom
            .spi()
            .baud
            .modify(|_, w| unsafe { w.baud().bits(baud) });
        self
    }

    /// Control the buffer overflow notification
    ///
    /// If set to true, an [`Error::Overflow`] will be issued as soon as an
    /// overflow occurs. Otherwise, it will not be issued until its place within
    /// the data stream.
    #[inline]
    pub fn immediate_overflow_notification(&mut self, set: bool) {
        self.sercom.spi().ctrla.modify(|_, w| w.ibon().bit(set));
    }

    /// Run in standby mode
    ///
    /// When set, the SPI peripheral will run in standby mode. See the datasheet
    /// for more details.
    #[inline]
    pub fn run_in_standby(&mut self, set: bool) {
        self.sercom.spi().ctrla.modify(|_, w| w.runstdby().bit(set));
    }

    /// Enable interrupts for the specified flags
    pub fn enable_interrupts(&mut self, flags: Flags) {
        self.sercom
            .spi()
            .intenset
            .write(|w| unsafe { w.bits(flags.bits()) });
    }

    /// Disable interrupts for the specified flags
    pub fn disable_interrupts(&mut self, flags: Flags) {
        self.sercom
            .spi()
            .intenclr
            .write(|w| unsafe { w.bits(flags.bits()) });
    }

    /// Enable the SPI peripheral
    ///
    /// SPI transactions are not possible until the peripheral is enabled.
    /// This function is limited to [`ValidConfig`]s.
    #[inline]
    pub fn enable(self) -> Spi<Self>
    where
        Self: ValidConfig,
    {
        self.sercom.spi().ctrlb.modify(|_, w| w.rxen().set_bit());
        while self.sercom.spi().syncbusy.read().ctrlb().bit_is_set() {}
        self.sercom.spi().ctrla.modify(|_, w| w.enable().set_bit());
        while self.sercom.spi().syncbusy.read().enable().bit_is_set() {}
        Spi { config: self }
    }
}

//=============================================================================
// AnyConfig
//=============================================================================

/// Meta-type representing any [`Config`]
///
/// All instances of [`Config`] implement this trait. When used as a trait
/// bound, it acts to encapsulate a [`Config`]. Without this trait, a
/// completely generic [`Config`] requires three type parameters, i.e.
/// `Config<P, M, C>`. But when using this trait, only one type parameter is
/// required, i.e. `C: AnyConfig`. However, even though we have dropped type
/// parameters, no information is lost, because the [`Pads`], [`Mode`] and
/// [`CharSize`] type parameters are stored as associated types in the trait.
/// The implementation of [`AnyConfig`] looks like this:
///
/// ```
/// impl<P: Pads, M: Mode, C: CharSize> AnyConfig for Config<P, M, C> {
///     type Pads = P;
///     type Mode = M;
///     type CharSize = C;
/// }
/// ```
///
/// Thus, there is a one-to-one mapping between `Config<P, M, C>` and
/// `AnyConfig<Pads = P, Mode = M, CharSize = C>`, so you can always recover the
/// specific [`Config`] type from an implementation of [`AnyConfig`]. The type
/// alias [`SpecificConfig`] is provided for this purpose. You can convert
/// between [`AnyConfig`] and its corresponding [`SpecificConfig`] using the
/// [`Into`], [`AsRef`] and [`AsMut`] traits.
pub trait AnyConfig: Sealed + Is<Type = SpecificConfig<Self>> {
    type Pads: DipoDopo;
    type Mode: Mode;
    type CharSize: CharSize;
}

/// Type alias to recover the specific [`Config`] type from an implementation of
/// [`AnyConfig`]
pub type SpecificConfig<C> =
    Config<<C as AnyConfig>::Pads, <C as AnyConfig>::Mode, <C as AnyConfig>::CharSize>;

/// Type alias to recover the [`Pads`] type from an implementation of
/// [`AnyConfig`]
pub type SpiPads<C> = <C as AnyConfig>::Pads;

/// Type alias to recover the [`Mode`] type from an implementation of
/// [`AnyConfig`]
pub type SpiMode<C> = <C as AnyConfig>::Mode;

/// Type alias to recover the [`CharSize`] type from an implementation of
/// [`AnyConfig`]
pub type SpiCharSize<C> = <C as AnyConfig>::CharSize;

/// Type alias to recover the [`Pads`]' [`Sercom`] type from an implementation
/// of [`AnyConfig`]
pub type SpiSercom<C> = PadsSercom<SpiPads<C>>;

/// Type alias to recover the [`Pads`]' Data In [`Pad`] type from an
/// implementation of [`AnyConfig`]
pub type SpiDataIn<C> = PadsDataIn<SpiPads<C>>;

/// Type alias to recover the [`Pads`]' Data Out [`Pad`] type from an
/// implementation of [`AnyConfig`]
pub type SpiDataOut<C> = PadsDataOut<SpiPads<C>>;

/// Type alias to recover the [`Pads`]' SCK [`Pad`] type from an implementation
/// of [`AnyConfig`]
pub type SpiSclk<C> = PadsSclk<SpiPads<C>>;

/// Type alias to recover the [`Pads`]' SS [`Pad`] type from an implementation
/// of [`AnyConfig`]
pub type SpiSS<C> = PadsSS<SpiPads<C>>;

/// Type alias to recover the [`CharSize`]'s [`Word`] type from an
/// implementation of [`AnyConfig`]
pub type SpiWord<C> = Word<SpiCharSize<C>>;

impl<P, M, C> Sealed for Config<P, M, C>
where
    P: DipoDopo,
    M: Mode,
    C: CharSize,
{
}

impl<P, M, C> AnyConfig for Config<P, M, C>
where
    P: DipoDopo,
    M: Mode,
    C: CharSize,
{
    type Pads = P;
    type Mode = M;
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

/// Marker trait for valid SPI [`Config`]urations
///
/// A functional SPI peripheral must have, at a minimum, an SCK [`Pad`] and
/// either a Data In or a Data Out [`Pad`]. Dependeing on the operating
/// [`Mode`], an SS [`Pad`] may also be required.
///
/// The [`ValidConfig`] trait is implemented only for valid combinations of
/// [`Pads`] and operating [`Mode`]. No [`Config`] is valid if the SCK pad is
/// [`NoneT`] or if both the Data In and Data Out pads are [`NoneT`]. And when
/// [`Mode`] is [`MasterHWSS`] or [`Slave`], the SS pad must not be [`NoneT`]
/// either.
pub trait ValidConfig: AnyConfig {}

impl<P, C> ValidConfig for Config<P, Master, C>
where
    P: DipoDopo<SS = NoneT> + TxOrRx,
    C: CharSize,
{
}

impl<P, C> ValidConfig for Config<P, MasterHWSS, C>
where
    P: DipoDopo + TxOrRx,
    C: CharSize,
    P::SS: SomePad,
{
}

impl<P, C> ValidConfig for Config<P, Slave, C>
where
    P: DipoDopo + TxOrRx,
    C: CharSize,
    P::SS: SomePad,
{
}

//=============================================================================
// Spi
//=============================================================================

/// An enabled SPI peripheral that can perform transactions using the embedded
/// HAL traits
///
/// When an [`Spi`] is [`Tx`]` + `[`Rx`], it implements [`FullDuplex`], with a
/// word type that depends on [`CharSize`]. The word type is [`u8`] for an
/// [`EightBit`] [`CharSize`] and [`u16`] for a [`NineBit`] [`CharSize`].
///
/// For half-duplex transactions, [`Spi`] implements the `serial` [`Read`] and
/// [`Write`] traits. They are only implemented when the [`Pads`] are not
/// [`Tx`]` + `[`Rx`], so they don't compete with [`FullDuplex`].
///
/// [`Spi`] uses the default implementations for the [`blocking::spi`] and
/// [`blocking::serial`] traits.
///
/// For a non-blocking alternative that can be used to transfer slices, see the
/// [`SpiFuture`] type.
///
/// [`SpiFuture`]: crate::sercom::v2::spi_future::SpiFuture
pub struct Spi<C: ValidConfig> {
    config: C,
}

impl<C: ValidConfig> Spi<C> {
    /// Obtain a reference to the PAC `SERCOM` struct
    ///
    /// Directly accessing the `SERCOM` could break the invariants of the
    /// type-level tracking in this module, so it is unsafe.
    #[inline]
    pub unsafe fn sercom(&self) -> &SpiSercom<C> {
        &self.config.as_ref().sercom()
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
        let bits = unsafe { self.sercom().spi().intflag.read().bits() };
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
        unsafe { self.sercom().spi().intflag.write(|w| w.bits(flags.bits())) };
    }

    /// Read the error status flags
    #[inline]
    pub fn read_errors(&self) -> Errors {
        let bits = unsafe { self.sercom().spi().status.read().bits() };
        Errors::from_bits_truncate(bits)
    }

    /// Clear error status flags
    ///
    /// Setting a flag will clear the error. Clearing any flag will have no
    /// effect.
    #[inline]
    pub fn clear_errors(&mut self, errors: Errors) {
        unsafe { self.sercom().spi().status.write(|w| w.bits(errors.bits())) };
    }

    #[inline]
    pub fn read_flags_errors(&self) -> Result<Flags, Error> {
        self.read_errors().try_into()?;
        Ok(self.read_flags())
    }

    /// Read from the DATA register
    ///
    /// Reading from the data register directly is `unsafe`, because it will
    /// clear the RXC flag, which could break assumptions made elsewhere in
    /// this module.
    #[inline]
    pub unsafe fn read_data(&mut self) -> u16 {
        self.sercom().spi().data.read().data().bits()
    }

    /// Write to the DATA register
    ///
    /// Writing to the data register directly is `unsafe`, because it will clear
    /// the DRE flag, which could break assumptions made elsewhere in this
    /// module.
    #[inline]
    pub unsafe fn write_data(&mut self, data: u16) {
        self.sercom().spi().data.write(|w| w.data().bits(data))
    }

    /// Disable the SPI peripheral and return the [`Config`] struct
    #[inline]
    pub fn disable(self) -> C {
        // SAFETY: The read state must be reset when disabling the peripheral
        unsafe { Config::<C::Pads>::reset_serial_read_state() };
        let spim = unsafe { self.sercom().spi() };
        spim.ctrlb.modify(|_, w| w.rxen().clear_bit());
        while spim.syncbusy.read().ctrlb().bit_is_set() {}
        spim.ctrla.modify(|_, w| w.enable().clear_bit());
        while spim.syncbusy.read().enable().bit_is_set() {}
        self.config
    }
}

impl<C> Spi<C>
where
    C: ValidConfig,
    C::Pads: Rx + NotTx,
    C::Mode: MasterMode,
    SpiWord<C>: PrimInt,
{
    /// Reset the internal state tracking `serial` [`Read`] transactions
    ///
    /// See the implementation of [`Read`] for more details.
    pub unsafe fn reset_serial_read_state(&mut self) {
        Config::<C::Pads>::reset_serial_read_state();
    }
}

//=============================================================================
// AnySpi
//=============================================================================

/// Meta-type representing any [`Spi`]
///
/// This trait is implemented for every instance of [`Spi`]. It allows you to
/// restrict a generic type to an [`Spi`] with explicitly naming the [`Spi`]
/// type. Like other `Any*` traits in this HAL, you can recover the specific
/// [`Spi`] type with the type alias [`SpecificSpi`], and you can convert
/// between [`AnySpi`] and its corresponding [`SpecificSpi`] using the [`Into`],
/// [`AsRef`] and [`AsMut`] traits.
///
/// ```
/// fn example<P: AnySpi>(mut any_spi: P) {
///     let spi_mut: &mut SpecificSpi<P> = any_spi.as_mut();
///     let spi_ref: &SpecificSpi<P> = any_spi.as_ref();
///     let spi: SpecificSpi<P> = any_spi.into();
/// }
/// ```
pub trait AnySpi: Sealed + Is<Type = SpecificSpi<Self>> {
    type Config: ValidConfig;
}

/// Type alias to recover the specific [`Spi`] type from an implementation of
/// [`AnySpi`]
pub type SpecificSpi<T> = Spi<<T as AnySpi>::Config>;

impl<C: ValidConfig> Sealed for Spi<C> {}

impl<C: ValidConfig> AnySpi for Spi<C> {
    type Config = C;
}

/// Implementation required to satisfy the `Is<Type = SpecificSpi<Self>>` bound
/// on [`AnySpi`]
impl<S: AnySpi> AsRef<S> for SpecificSpi<S> {
    #[inline]
    fn as_ref(&self) -> &S {
        // SAFETY: This is guaranteed to be safe, because S == SpecificSpi<S>
        unsafe { transmute(self) }
    }
}

/// Implementation required to satisfy the `Is<Type = SpecificSpi<Self>>` bound
/// on [`AnySpi`]
impl<S: AnySpi> AsMut<S> for SpecificSpi<S> {
    #[inline]
    fn as_mut(&mut self) -> &mut S {
        // SAFETY: This is guaranteed to be safe, because S == SpecificSpi<S>
        unsafe { transmute(self) }
    }
}

//=============================================================================
// Embedded HAL traits
//=============================================================================

#[cfg(not(feature = "min-samd51n"))]
static mut SERIAL_READ_STATE: [bool; 6] = [false; 6];
#[cfg(feature = "min-samd51n")]
static mut SERIAL_READ_STATE: [bool; 8] = [false; 8];

/// Implement [`Read`] for [`MasterMode`]s
///
/// [`Read`] is only implemented when the [`Pads`] are [`Rx`] but [`NotTx`].
/// If the [`Pads`] are both [`Rx`] and [`Tx`], then use [`FullDuplex`].
///
/// In a [`MasterMode`], [`Read`] has to initiate transactions and receive the
/// responses. To do so, it uses a `static` allocation to keep track of the
/// transaction state. If a transaction is in progress, it will wait on `RXC`.
/// If not, it will wait on `DRE` and then send `0`.
///
/// It should not be possible for the tracked state to become invalid using only
/// safe code. However, if using `unsafe`, the state could fall out of sync. In
/// that case, the [`Spi::reset_serial_read_state`] method can be used to reset
/// the state. After reset, a transaction is assumed to NOT be in progress.
impl<P, M, C> Read<C::Word> for Spi<Config<P, M, C>>
where
    Config<P, M, C>: ValidConfig,
    P: DipoDopo + Rx + NotTx,
    M: MasterMode,
    C: CharSize,
    C::Word: PrimInt,
    u16: AsPrimitive<C::Word>,
{
    type Error = Error;

    /// If a transaction is already in progress, wait for an `RXC` flag, then
    /// read the word. If not, wait for a `DRE` flag, then write `0`.
    ///
    /// Track the transaction state with an internal `static`. It should not be
    /// possible for the state to get out of sync with the hardware using only
    /// safe code. If using `unsafe` and the state does get out of sync, use
    /// [`Spi::reset_serial_read_state`] to reset it.
    #[inline]
    fn read(&mut self) -> nb::Result<C::Word, Error> {
        let index = <PadsSercom<P> as Sercom>::NUM;
        let in_progress = unsafe { &mut SERIAL_READ_STATE[index] };
        let flags = self.read_flags_errors()?;
        if !*in_progress && flags.contains(Flags::DRE) {
            unsafe { self.write_data(0) };
            *in_progress = true;
            Err(WouldBlock)
        } else if *in_progress && flags.contains(Flags::RXC) {
            *in_progress = false;
            unsafe { Ok(self.read_data().as_()) }
        } else {
            Err(WouldBlock)
        }
    }
}

/// Implement [`Read`] for [`Slave`] [`Mode`]
///
/// [`Read`] is only implemented when the [`Pads`] are [`Rx`] but [`NotTx`].
/// If the [`Pads`] are both [`Rx`] and [`Tx`], then use [`FullDuplex`].
///
/// In [`Slave`] [`Mode`], [`Read`] does not have to initiate transactions, so
/// it does not have to store any internal state. It only has to wait on `RXC`.
impl<P, C> Read<C::Word> for Spi<Config<P, Slave, C>>
where
    Config<P, Slave, C>: ValidConfig,
    P: DipoDopo + Rx + NotTx,
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

/// Implement [`Write`] for [`Spi`]
///
/// [`Write`] is only implemented when the [`Pads`] are [`Tx`] but [`NotRx`].
/// If the [`Pads`] are both [`Tx`] and [`Rx`], then use [`FullDuplex`].
///
/// Because [`Write`] is only implemented when the [`Pads`] are [`NotRx`], this
/// implementation never reads the DATA register and ignores all buffer overflow
/// errors.
impl<C> Write<SpiWord<C>> for Spi<C>
where
    C: ValidConfig,
    C::Pads: Tx + NotRx,
    SpiWord<C>: PrimInt + AsPrimitive<u16>,
{
    type Error = Error;

    /// Wait for a `DRE` flag, then write a word
    #[inline]
    fn write(&mut self, word: SpiWord<C>) -> nb::Result<(), Error> {
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

impl<C> blocking::serial::write::Default<SpiWord<C>> for Spi<C>
where
    C: ValidConfig,
    Spi<C>: Write<SpiWord<C>>,
{
}

/// Perform a non-blocking, [`FullDuplex`] trasaction
///
/// [`FullDuplex`] is only implemented when [`Pads`] is both [`Tx`] and [`Rx`]
/// and the [`Mode`] is a [`MasterMode`]. The word type is dependent on
/// [`CharSize`].
impl<C> FullDuplex<SpiWord<C>> for Spi<C>
where
    C: ValidConfig,
    C::Pads: Tx + Rx,
    C::Mode: MasterMode,
    SpiWord<C>: PrimInt + AsPrimitive<u16>,
    u16: AsPrimitive<SpiWord<C>>,
{
    type Error = Error;

    #[inline]
    fn read(&mut self) -> nb::Result<SpiWord<C>, Error> {
        let flags = self.read_flags_errors()?;
        if flags.contains(Flags::RXC) {
            unsafe { Ok(self.read_data().as_()) }
        } else {
            Err(WouldBlock)
        }
    }

    #[inline]
    fn send(&mut self, word: SpiWord<C>) -> nb::Result<(), Error> {
        let flags = self.read_flags_errors()?;
        if flags.contains(Flags::DRE) {
            unsafe { self.write_data(word.as_()) };
            Ok(())
        } else {
            Err(WouldBlock)
        }
    }
}

impl<C> blocking::spi::transfer::Default<SpiWord<C>> for Spi<C>
where
    C: ValidConfig,
    Self: FullDuplex<SpiWord<C>>,
{
}

impl<C> blocking::spi::write::Default<SpiWord<C>> for Spi<C>
where
    C: ValidConfig,
    Self: FullDuplex<SpiWord<C>>,
{
}

#[cfg(feature = "unproven")]
impl<C> blocking::spi::write_iter::Default<SpiWord<C>> for Spi<C>
where
    C: ValidConfig,
    Self: FullDuplex<SpiWord<C>>,
{
}
