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
//! certain [`Pin`] combinations are acceptable. In particular, all [`Pin`]s
//! must be mapped to the same [`Sercom`] and [`IoSet`] (see section 6.2.8.1 of
//! the datasheet).
//!
//! This HAL makes it impossible to use invalid [`Pin`]/[`Pad`] combinations.
//! The [`Pads`] struct is responsible for enforcing these constraints. To
//! create a set of [`Pads`], start by specifying the [`Sercom`] and [`IoSet`].
//!
//! ```
//! use atsamd_hal::sercom::v2::{Sercom0, pads::IoSet1, spi};
//!
//! let pads = spi::Pads::<Sercom0, IoSet1>::new();
//! ```
//!
//! Next, specify the [`Pin`]s and their corresponding [`PadNum`]s, when
//! necessary. Both `v1` and `v2` pin types are accepted here. The SCLK and SS
//! signals always use [`Pad1`] and [`Pad2`] respectively, but the Data In and
//! Data Out [`Pad`]s can change [`PadNum`], based on the [`Dipo`] and [`Dopo`]
//! configuration. The [`Pads`] struct enforces that the [`Pin`] type matches
//! the specified [`Sercom`], [`PadNum`], and [`IoSet`].
//!
//! ```
//! use atsamd_hal::target_device::Peripherals;
//! use atsamd_hal::gpio::v2::Pins;
//! use atsamd_hal::sercom::v2::{Sercom0, spi};
//! use atsamd_hal::sercom::v2::pads::{IoSet1, Pad0, Pad3};
//!
//! let mut peripherals = Peripherals::take().unwrap();
//! let pins = Pins::new(peripherals.PORT);
//! let pads = spi::Pads::<Sercom0, IoSet1>::new()
//!     .sclk(pins.pa09)
//!     .data_in::<Pad0, _>(pins.pa08)
//!     .data_out::<Pad3, _>(pins.pa11);
//! ```
//!
//! Not every [`Pad`] must be specified. Each [`Pad`] within a set of [`Pads`]
//! is actually an [`OptionalPad`]. If a [`Pad`] is unused, it can be left as
//! [`NoneT`]. However, if they are ever going to be used for transactions, the
//! [`Pads`] must satisfy the requirements specified for a [`ValidConfig`].
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
//! MCLK, so that it can enable the APB clock, and a frequency to indicate the
//! GCLK configuration. Users are responsible for correctly configuring the
//! GCLK.
//!
//! ```
//! use atsamd_hal::time::U32Ext;
//!
//! let mclk = peripherals.MCLK;
//! let sercom = peripherals.SERCOM0;
//! // Configure GCLK for 10 MHz
//! let freq = 10.mhz();
//! let config = spi::Config::new(&mclk, sercom, pads, freq);
//! ```
//!
//! The SPI peripheral has two different ways to control the transaction length,
//! the character size and the length counter. The character size can be set to
//! 8-bit or 9-bit transactions. The length counter can be set to produce
//! transactions of any length from 1-255 bytes. For simplicity, this module
//! ignores character size. Instead, the SPI peripheral is always configured to
//! use 32-bit extension mode and the length counter.
//!
//! By default, the peripheral is set to use [`Master`] [`Mode`] with a
//! transaction [`Length`] of one byte. Transaction [`Length`]s are specified at
//! the type level using the [`typenum`] crate. The usable lengths, [`U1`] to
//! [`U255`], are re-exported in the [`lengths`] submodule.
//!
//! The [`Config`] struct uses the builder pattern to configure the peripheral,
//! ending with a call to [`enable`], which consumes the [`Config`] and returns
//! an enabled [`Spi`] peripheral.
//!
//! ```
//! use embedded_hal::spi::MODE_1;
//!
//! let spi = spi::Config::new(&mclk, sercom, pads, freq)
//!     .baud(1.mhz())
//!     .length::<U2>()
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
//! which vary based on the transaction [`Length`] and [`Pads`]. For instance,
//! [`FullDuplex`] is only implemented if the [`Pads`] are both [`Tx`] and
//! [`Rx`] and if the transaction [`Length`] is less than [`U4`].
//!
//! ```
//! use nb::block;
//! use embedded_hal::spi::FullDuplex;
//!
//! block!(spi.send(0xAA55));
//! let rcvd: u16 = block!(spi.read());
//! ```
//!
//! [`enable`]: Config::enable
//! [`Pin`]: crate::gpio::v2::pin::Pin
//! [`U255`]: typenum::U255

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
use seq_macro::seq;
use typenum::{Unsigned, U0, U1, U2, U3, U4};

use crate::target_device as pac;
use pac::sercom0::spim::ctrla::{CPHA_A, CPOL_A, DIPO_A, DOPO_A, DORD_A, MODE_A};
use pac::sercom0::RegisterBlock;
use pac::MCLK;

use crate::gpio::v2::{AnyPin, SpecificPin};
use crate::sercom::v2::pads::{IoSet, Map, Pad0, Pad1, Pad2, Pad3, PadNum};
use crate::sercom::v2::pads::{OptionalPad, Pad, SomePad};
use crate::sercom::v2::Sercom;
use crate::time::Hertz;
use crate::typelevel::{Is, NoneT, Sealed};

/// Re-export [`typenum`] constants for use as [`Length`] type parameters
///
/// Only the values `U1` - `U255` are valid [`Length`]s
pub mod lengths {
    use seq_macro::seq;

    seq!(N in 1..=255 {
        pub use typenum::U#N;
    });
}

//=============================================================================
// Pad configuration
//=============================================================================

macro_rules! define_pad_config_traits {
    ( $( $name:ident),+ ) => {
        paste! {
            $(
                #[doc = "Control the `" $name:upper "` field as a function of the [`PadNum`] type"]
                pub trait [<$name:camel>]: Sealed {
                    /// Corresponding variant from the PAC `enum`
                    const VARIANT: [<$name:upper _A>];
                    /// Configure the pad according to [`Self::VARIANT`]
                    #[inline]
                    fn configure(sercom: &RegisterBlock) {
                        sercom.spim().ctrla.modify(|_, w|
                            w.[<$name:lower>]().variant(Self::VARIANT)
                        );
                    }
                }
            )+
        }
    };
}

define_pad_config_traits!(dipo, dopo);

impl Dipo for Pad0 {
    const VARIANT: DIPO_A = DIPO_A::PAD0;
}
impl Dipo for Pad1 {
    const VARIANT: DIPO_A = DIPO_A::PAD1;
}
impl Dipo for Pad2 {
    const VARIANT: DIPO_A = DIPO_A::PAD2;
}
impl Dipo for Pad3 {
    const VARIANT: DIPO_A = DIPO_A::PAD3;
}

/// Implement [`Dipo`] for each [`Pad`] based on its [`PadNum`]
impl<S, P, I> Dipo for Pad<S, P, I>
where
    S: Sercom,
    P: PadNum + Dipo,
    I: IoSet + Map<S, P>,
{
    const VARIANT: DIPO_A = P::VARIANT;
    #[inline]
    fn configure(sercom: &RegisterBlock) {
        P::configure(sercom)
    }
}

/// Implement [`Dipo`] for [`NoneT`] to allow [`OptionalPad`]s
impl Dipo for NoneT {
    /// This value is arbitrary and meaningless for [`NoneT`]
    const VARIANT: DIPO_A = DIPO_A::PAD0;

    /// Override the default implementation to do nothing
    fn configure(_: &RegisterBlock) {}
}

impl Dopo for Pad0 {
    const VARIANT: DOPO_A = DOPO_A::PAD0;
}
impl Dopo for Pad3 {
    const VARIANT: DOPO_A = DOPO_A::PAD2;
}

/// Implement [`Dopo`] for each [`Pad`] based on its [`PadNum`]
impl<S, P, I> Dopo for Pad<S, P, I>
where
    S: Sercom,
    P: PadNum + Dopo,
    I: IoSet + Map<S, P>,
{
    const VARIANT: DOPO_A = P::VARIANT;

    #[inline]
    fn configure(sercom: &RegisterBlock) {
        P::configure(sercom)
    }
}

/// Implement [`Dopo`] for [`NoneT`] to allow [`OptionalPad`]s
impl Dopo for NoneT {
    /// This value is arbitrary and meaningless for [`NoneT`]
    const VARIANT: DOPO_A = DOPO_A::PAD0;

    /// Override the default implementation to do nothing
    fn configure(_: &RegisterBlock) {}
}

//=============================================================================
// Pads
//=============================================================================

/// Encapsulate the set of pads for an SPI peripheral
///
/// This struct acts to encapsulate up to four [`Pad`]s for use with an SPI
/// peripheral. All of the [`Pad`]s must share the same [`Sercom`] and
/// [`IoSet`]. The four type parameters `DI`, `DO`, `CK` and `SS` represent the
/// respective DI, DO, SCK and SS [`Pad`] types.
///
/// Each pad in this struct is an [`OptionalPad`]. When first initialized, each
/// pad is set to [`NoneT`]. Individual pads are set using a builder-pattern
/// API. Both `v1` and `v2` pin types are accepted. The `CK` and `SS` pads are
/// always constrained to be [`Pad1`] and [`Pad2`] respectively, while the `DI`
/// and `DO` pads can vary, based on the [`Dipo`] and [`Dopo`] configuration.
/// Consequently, the [`data_in`] and [`data_out`] methods take a [`PadNum`] as
/// a type parameter.
///
/// ```no_run
/// # use atsamd_hal::target_device::Peripherals;
/// # use atsamd_hal::gpio::v2::Pins;
/// # use atsamd_hal::sercom::v2::Sercom4;
/// # use atsamd_hal::sercom::v2::pads::{IoSet2, Pad0, Pad3};
/// # use atsamd_hal::sercom::v2::spi;
/// let mut peripherals = Peripherals::take().unwrap();
/// let pins = Pins::new(peripherals.PORT);
/// let pads = spi::Pads::<Sercom4, IoSet2>::new()
///     .sclk(pins.pb09)
///     .data_in::<Pad0, _>(pins.pb08)
///     .data_out::<Pad3, _>(pins.pb11);
/// ```
///
/// The [`Map`] trait enforces that all [`Pad`]s have the same [`Sercom`] and
/// [`IoSet`], and that each [`Pad`] uses the correct [`Pin`].
///
/// Keep in mind that the language used here is from the perspective of the
/// chip, regardless of [`Mode`]. When used in a [`MasterMode`], [`data_in`]
/// sets the MISO pad, but in [`Slave`] [`Mode`], it sets the MOSI pad.
///
/// The [`Tx`], [`Rx`], [`NotTx`], [`NotRx`] and [`TxOrRx`] marker traits are
/// implemented only for [`Pad`] combinations reflecting each trait's name.
/// Again, the labels here are always from the chips perspective, regardless of
/// [`Mode`].
///
/// [`Pin`]: crate::gpio::v2::pin::Pin
/// [`data_in`]: Pads::data_in
/// [`data_out`]: Pads::data_out
pub struct Pads<S, I, DI = NoneT, DO = NoneT, CK = NoneT, SS = NoneT>
where
    S: Sercom,
    I: IoSet,
    DI: OptionalPad + Dipo,
    DO: OptionalPad + Dopo,
    CK: OptionalPad,
    SS: OptionalPad,
{
    sercom: PhantomData<S>,
    ioset: PhantomData<I>,
    data_in: DI,
    data_out: DO,
    sclk: CK,
    ss: SS,
}

impl<S: Sercom, I: IoSet> Pads<S, I> {
    /// Create a new [`Pads`] struct
    ///
    /// All of the pads are initialized to [`NoneT`]
    #[inline]
    pub fn new() -> Pads<S, I> {
        Pads {
            sercom: PhantomData,
            ioset: PhantomData,
            data_in: NoneT,
            data_out: NoneT,
            sclk: NoneT,
            ss: NoneT,
        }
    }
}

impl<S, I, DI, DO, CK, SS> Pads<S, I, DI, DO, CK, SS>
where
    S: Sercom,
    I: IoSet,
    DI: OptionalPad + Dipo,
    DO: OptionalPad + Dopo,
    CK: OptionalPad,
    SS: OptionalPad,
{
    /// Set the `DI` [`Pad`] using [`PadNum`] `P`
    ///
    /// In a [`MasterMode`], this is MISO. In [`Slave`] [`Mode`], this is MOSI.
    #[inline]
    pub fn data_in<P, T>(self, data_in: T) -> Pads<S, I, Pad<S, P, I>, DO, CK, SS>
    where
        P: PadNum + Dipo,
        T: AnyPin,
        I: Map<S, P>,
        Pad<S, P, I>: From<SpecificPin<T>>,
    {
        Pads {
            sercom: self.sercom,
            ioset: self.ioset,
            data_in: data_in.into().into(),
            data_out: self.data_out,
            sclk: self.sclk,
            ss: self.ss,
        }
    }

    /// Set the `DO` [`Pad`] using [`PadNum`] `P`
    ///
    /// In a [`MasterMode`], this is MOSI. In [`Slave`] [`Mode`], this is MISO.
    #[inline]
    pub fn data_out<P, T>(self, data_out: T) -> Pads<S, I, DI, Pad<S, P, I>, CK, SS>
    where
        P: PadNum + Dopo,
        T: AnyPin,
        I: Map<S, P>,
        Pad<S, P, I>: From<SpecificPin<T>>,
    {
        Pads {
            sercom: self.sercom,
            ioset: self.ioset,
            data_in: self.data_in,
            data_out: data_out.into().into(),
            sclk: self.sclk,
            ss: self.ss,
        }
    }

    /// Set the `SCK` [`Pad`], which is always [`Pad1`]
    #[inline]
    pub fn sclk<T>(self, sclk: T) -> Pads<S, I, DI, DO, Pad<S, Pad1, I>, SS>
    where
        T: AnyPin,
        I: Map<S, Pad1>,
        Pad<S, Pad1, I>: From<SpecificPin<T>>,
    {
        Pads {
            sercom: self.sercom,
            ioset: self.ioset,
            data_in: self.data_in,
            data_out: self.data_out,
            sclk: sclk.into().into(),
            ss: self.ss,
        }
    }

    /// Set the `SS` [`Pad`], which is always [`Pad2`]
    #[inline]
    pub fn ss<T>(self, ss: T) -> Pads<S, I, DI, DO, CK, Pad<S, Pad2, I>>
    where
        T: AnyPin,
        I: Map<S, Pad2>,
        Pad<S, Pad2, I>: From<T>,
        Pad<S, Pad2, I>: From<SpecificPin<T>>,
    {
        Pads {
            sercom: self.sercom,
            ioset: self.ioset,
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
/// Because it takes six type parameters, fully specifying a [`Pads`] type is
/// tedious and error-prone. In normal code, the type parameters can usually be
/// inferred. But some cases, like `static` variables, cannot use inference. In
/// these cases, the [`pads_alias`] macro can make the process easier.
///
/// A normal [`Pads`] alias declaration might look like this:
///
/// ```
/// use atsamd_hal::sercom::v2::Sercom0;
/// use atsamd_hal::sercom::v2::pads::{IoSet1, Pad, Pad0, Pad1, Pad2};
/// use atsamd_hal::sercom::v2::spi;
/// use atsamd_hal::typelevel::NoneT;
///
/// pub type Alias = spi::Pads<
///     Sercom0,
///     IoSet1,
///     Pad<Sercom0, Pad3, IoSet1>,
///     NoneT,
///     Pad<Sercom0, Pad1, IoSet1>,
///     Pad<Sercom0, Pad2, IoSet1>,
/// >;
/// ```
///
/// There is a lot of repetition and room for error in this declaration. The
/// [`pads_alias`] macro simplifies this example to:
///
/// ```
/// use atsamd_hal::pads_alias;
///
/// pads_alias!(pub type Alias = Pads<Sercom0, IoSet1, DI = Pad3, SS = Pad2>);
/// ```
///
/// The arguments `DI = Pad3` and `SS = Pad2` specify the [`PadNum`]s for the
/// Data In and SS lines. No [`PadNum`] argument is provided for `DO`, so the
/// Data Out [`Pad`] type will be set to [`NoneT`].
///
/// The `DI`, `DO` and `SS` arguments are all optional. If a corresponding
/// [`PadNum`] is not given, the respective [`Pad`] type will be [`NoneT`]. The
/// SCLK line is always required, and its [`PadNum`] must always be [`Pad1`], so
/// this macro always includes an implicit `CK = Pad1` argument. Any remaining
/// [`PadNum`] arguments must be specified in the indicated order: `DI`, `DO`,
/// `SS`.
#[macro_export]
macro_rules! pads_alias {
    (
        $vis:vis type $Name:ident = Pads<
            $Sercom:ident,
            $IoSet:ident
            $(, DI = $DI:ident)?
            $(, DO = $DO:ident)?
            $(, SS = $SS:ident)?
        >
    ) => {
        $vis type $Name = $crate::sercom::v2::spi::Pads<
            $crate::sercom::v2::$Sercom,
            $crate::sercom::v2::pads::$IoSet,
            __pad_type!($($Sercom, $DI, $IoSet)?),
            __pad_type!($($Sercom, $DO, $IoSet)?),
            __pad_type!($Sercom, Pad1, $IoSet),
            __pad_type!($($Sercom, $SS, $IoSet)?),
        >;
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! __pad_type {
    () => { NoneT };
    ($Sercom:ident, $PadNum:ident, $IoSet:ident) => {
        $crate::sercom::v2::pads::Pad<
            $crate::sercom::v2::$Sercom,
            $crate::sercom::v2::pads::$PadNum,
            $crate::sercom::v2::pads::$IoSet
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
/// The [`configure`] function serves as the interface to configure [`Dipo`]
/// and [`Dopo`] based on the chosen [`Pads`].
///
/// Like other `Any*` types in this HAL, the [`SpecificPads`] type can be
/// recovered using the [`Into`], [`AsRef`] and [`AsMut`] traits. However, there
/// is unlikely to be a situation where that is useful for the [`Pads`] type.
///
/// [`configure`]: Dipo::configure
pub trait AnyPads: Sealed + Is<Type = SpecificPads<Self>> {
    /// [`Sercom`] of the corresponding [`Pads`]
    type Sercom: Sercom;

    /// [`IoSet`] of the corresponding [`Pads`]
    type IoSet: IoSet;

    /// Data In [`Pad`] from the corresponding [`Pads`]
    type DataIn: OptionalPad + Dipo;

    /// Data Out [`Pad`] from the corresponding [`Pads`]
    type DataOut: OptionalPad + Dopo;

    /// SCLK [`Pad`] from the corresponding [`Pads`]
    type Sclk: OptionalPad;

    /// SS [`Pad`] from the corresponding [`Pads`]
    type SS: OptionalPad;

    /// Configure the pads with the correct [`Dipo`] and [`Dopo`] values
    ///
    /// This function will have no effect on the corresponding pad if it is
    /// [`NoneT`].
    #[inline]
    fn configure(sercom: &RegisterBlock) {
        Self::DataIn::configure(sercom);
        Self::DataOut::configure(sercom);
    }
}

/// Type alias to recover the specific [`Pads`] type from an implementation of
/// [`AnyPads`]
pub type SpecificPads<P> = Pads<
    <P as AnyPads>::Sercom,
    <P as AnyPads>::IoSet,
    <P as AnyPads>::DataIn,
    <P as AnyPads>::DataOut,
    <P as AnyPads>::Sclk,
    <P as AnyPads>::SS,
>;

/// Type alias to recover the [`Sercom`] type from an implementation of
/// [`AnyPads`]
pub type PadsSercom<P> = <P as AnyPads>::Sercom;

/// Type alias to recover the [`IoSet`] type from an implementation of
/// [`AnyPads`]
pub type PadsIoSet<P> = <P as AnyPads>::IoSet;

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

impl<S, I, DI, DO, CK, SS> Sealed for Pads<S, I, DI, DO, CK, SS>
where
    S: Sercom,
    I: IoSet,
    DI: OptionalPad + Dipo,
    DO: OptionalPad + Dopo,
    CK: OptionalPad,
    SS: OptionalPad,
{
}

impl<S, I, DI, DO, CK, SS> AnyPads for Pads<S, I, DI, DO, CK, SS>
where
    S: Sercom,
    I: IoSet,
    DI: OptionalPad + Dipo,
    DO: OptionalPad + Dopo,
    CK: OptionalPad,
    SS: OptionalPad,
{
    type Sercom = S;
    type IoSet = I;
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

impl<S, I, DI, CK, SS> TxOrRx for Pads<S, I, DI, NoneT, CK, SS>
where
    S: Sercom,
    I: IoSet,
    DI: SomePad + Dipo,
    CK: SomePad,
    SS: OptionalPad,
{
}

impl<S, I, DO, CK, SS> TxOrRx for Pads<S, I, NoneT, DO, CK, SS>
where
    S: Sercom,
    I: IoSet,
    DO: SomePad + Dopo,
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
    ///
    /// For maximum flexibility, this module chooses to always operate in 32-bit
    /// extension mode. The LENGTH counter is used to control the number of byes
    /// in each SPI transaction. Due to a hardware bug, ICSPACE must be at least
    /// one. See the silicon errata for more details.
    #[inline]
    fn configure(sercom: &RegisterBlock) -> () {
        sercom
            .spim()
            .ctrla
            .modify(|_, w| w.mode().variant(Self::MODE));
        sercom
            .spim()
            .ctrlb
            .modify(|_, w| w.mssen().bit(Self::MSSEN));
        sercom.spim().ctrlc.write(|w| unsafe {
            w.data32b().data_trans_32bit();
            w.icspace().bits(1)
        });
        while sercom.spim().syncbusy.read().ctrlb().bit_is_set() {}
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
// Transaction length
//=============================================================================

/// Type-level `enum` representing the SPI transaction length, in bytes
///
/// As mentioned in the [`Mode`] documentation, this module chooses to always
/// operate in 32-bit extension mode. The LENGTH counter is used to control the
/// number of byes in each SPI transaction.
///
/// The SPI transaction length is represented in the type domain using
/// [`Unsigned`] types from the [`typenum`] crate. The length can be set
/// statically, using a length from `U1` to `U255`, or it can be set
/// dynamically, using the [`DynLength`] marker type. The static [`Length`]
/// types can be imported from the [`lengths`] submodule.
///
/// The SPI transaction length affects the word size for the embedded HAL
/// traits, as well as other aspects of the SPI API. Transaction lengths of 1-4
/// only require a single read/write of the DATA register, so they behave
/// differently than longer transaction lengths.
pub trait Length: Sealed + Unsigned + 'static {
    /// Word size for the transaction length
    ///
    /// For lengths 1-4, this type is `u8`, `u16` or `u32`. For longer
    /// transactions, this type is `[u8, Self::USIZE]`.
    type Word: 'static;

    /// Configure the `LENGTH` register and enable the `LENGTH` counter
    #[inline]
    fn configure(sercom: &RegisterBlock) -> () {
        sercom.spim().length.write(|w| unsafe {
            w.len().bits(Self::U8);
            w.lenen().set_bit()
        });
        while sercom.spim().syncbusy.read().length().bit_is_set() {}
    }
}

/// Type alias to recover the `Word` type from an implementation of [`Length`]
pub type Word<L> = <L as Length>::Word;

/// Marker type for a run-time dynamic [`Length`]
pub type DynLength = U0;

impl Sealed for DynLength {}
impl Length for DynLength {
    type Word = ();
}

/// Marker trait for statically known transaction [`Length`]s
pub trait StaticLength: Length {}

impl Sealed for U1 {}
impl StaticLength for U1 {}
impl Length for U1 {
    type Word = u8;
}

impl Sealed for U2 {}
impl StaticLength for U2 {}
impl Length for U2 {
    type Word = u16;
}

impl Sealed for U3 {}
impl StaticLength for U3 {}
impl Length for U3 {
    type Word = u32;
}

impl Sealed for U4 {}
impl StaticLength for U4 {}
impl Length for U4 {
    type Word = u32;
}

/// Marker trait for transaction [`Length`]s greater than four
pub trait GreaterThan4: Length {}

seq!(N in 5..=255 {
    impl Sealed for typenum::U#N {}
    impl StaticLength for typenum::U#N {}
    impl GreaterThan4 for typenum::U#N {}
    impl Length for typenum::U#N {
        type Word = [u8; typenum::U#N::USIZE];
    }
});

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
    /// The available error flags are `BUFOVF` and `LENERR`. The binary format
    /// of the underlying bits exactly matches the STATUS register.
    pub struct Errors: u16 {
        const BUFOVF = 0x0004;
        const LENERR = 0x0800;
    }
}

/// Error `enum` for SPI transactions
///
/// The SPI peripheral only has two error types, buffer overflow and transaction
/// length error.
#[derive(Debug)]
pub enum Error {
    Overflow,
    LengthError,
}

impl TryFrom<Errors> for () {
    type Error = Error;
    fn try_from(errors: Errors) -> Result<(), Error> {
        // Buffer overflow has priority
        if errors.contains(Errors::BUFOVF) {
            Err(Error::Overflow)
        } else if errors.contains(Errors::LENERR) {
            Err(Error::LengthError)
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
/// transaction [`Length`]. Upon creation, the [`Config`] takes ownership of the
/// [`Sercom`] and resets it, returning it configured as an SPI peripheral in
/// [`Master`] [`Mode`] with [`Length`] [`U1`].
///
/// [`Config`] uses a builder-pattern API to configure the peripheral,
/// culminating in a call to [`enable`], which consumes the [`Config`] and
/// returns an enabled [`Spi`] peripheral. The [`enable`] function is restricted
/// to [`ValidConfig`]s.
///
/// [`enable`]: Config::enable
pub struct Config<P, M = Master, L = U1>
where
    P: AnyPads,
    M: Mode,
    L: Length,
{
    sercom: P::Sercom,
    pads: P,
    mode: PhantomData<M>,
    len: PhantomData<L>,
    freq: Hertz,
}

impl<P: AnyPads> Config<P> {
    /// Create a new [`Config`] in the default configuration.
    fn create(sercom: P::Sercom, pads: P, freq: impl Into<Hertz>) -> Self {
        Self::swrst(&sercom);
        Master::configure(&sercom);
        P::configure(&sercom);
        U1::configure(&sercom);
        Self {
            sercom,
            pads,
            mode: PhantomData,
            len: PhantomData,
            freq: freq.into(),
        }
    }

    /// Create a new [`Config`] in the default configuration
    ///
    /// This function will enable the corresponding APB clock, reset the
    /// [`Sercom`] peripheral, and return a [`Config`] in the default
    /// configuration, [`Master`] [`Mode`] with [`Length`] [`U1`]. [`Config`]
    /// takes ownership of the [`Sercom`] and [`Pads`].
    ///
    /// Users must configure GCLK manually. The `freq` parameter represents the
    /// GCLK frequency for this [`Sercom`] instance.
    #[inline]
    pub fn new(mclk: &MCLK, mut sercom: P::Sercom, pads: P, freq: impl Into<Hertz>) -> Self {
        sercom.enable_apb_clock(mclk);
        Self::create(sercom, pads, freq)
    }
}

impl<P, M, L> Config<P, M, L>
where
    P: AnyPads,
    M: Mode,
    L: Length,
{
    /// Reset the SERCOM peripheral
    #[inline]
    fn swrst(sercom: &P::Sercom) {
        sercom.spim().ctrla.write(|w| w.swrst().set_bit());
        while sercom.spim().syncbusy.read().swrst().bit_is_set() {}
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

    /// Change the [`Config`] [`Mode`] or [`Length`]
    #[inline]
    fn change<M2, L2>(self) -> Config<P, M2, L2>
    where
        M2: Mode,
        L2: Length,
    {
        Config {
            sercom: self.sercom,
            pads: self.pads,
            mode: PhantomData,
            len: PhantomData,
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
    pub fn op_mode<M2: Mode>(self) -> Config<P, M2, L> {
        M2::configure(&self.sercom);
        self.change()
    }

    /// Change the transaction [`Length`]
    ///
    /// To use a run-time dynamic length, set the [`Length`] type to
    /// [`DynLength`] and then use the [`get_dyn_length`] and [`set_dyn_length`]
    /// methods.
    ///
    /// [`get_dyn_length`]: Config::get_dyn_length
    /// [`set_dyn_length`]: Config::set_dyn_length
    #[inline]
    pub fn length<L2: Length>(self) -> Config<P, M, L2> {
        L2::configure(&self.sercom);
        self.change()
    }

    /// Change the clock polarity
    #[inline]
    pub fn cpol(self, cpol: Polarity) -> Self {
        let cpol = match cpol {
            Polarity::IdleLow => CPOL_A::IDLE_LOW,
            Polarity::IdleHigh => CPOL_A::IDLE_HIGH,
        };
        self.sercom
            .spim()
            .ctrla
            .modify(|_, w| w.cpol().variant(cpol));
        self
    }

    /// Change the clock phase
    #[inline]
    pub fn cpha(self, cpha: Phase) -> Self {
        let cpha = match cpha {
            Phase::CaptureOnFirstTransition => CPHA_A::LEADING_EDGE,
            Phase::CaptureOnSecondTransition => CPHA_A::TRAILING_EDGE,
        };
        self.sercom
            .spim()
            .ctrla
            .modify(|_, w| w.cpha().variant(cpha));
        self
    }

    /// Change the SPI mode (clock polarity & phase)
    #[inline]
    pub fn spi_mode(self, mode: spi::Mode) -> Self {
        let cpol = match mode.polarity {
            Polarity::IdleLow => CPOL_A::IDLE_LOW,
            Polarity::IdleHigh => CPOL_A::IDLE_HIGH,
        };
        let cpha = match mode.phase {
            Phase::CaptureOnFirstTransition => CPHA_A::LEADING_EDGE,
            Phase::CaptureOnSecondTransition => CPHA_A::TRAILING_EDGE,
        };
        self.sercom.spim().ctrla.modify(|_, w| {
            w.cpol().variant(cpol);
            w.cpha().variant(cpha)
        });
        self
    }

    /// Change the bit order of transmission (MSB/LSB first)
    ///
    /// This only affects the order of bits within each byte. Bytes are always
    /// transferred in little endian order from the 32-bit DATA register.
    #[inline]
    pub fn msb_first(self, msb_first: bool) -> Self {
        let dord = match msb_first {
            true => DORD_A::MSB,
            false => DORD_A::LSB,
        };
        self.sercom
            .spim()
            .ctrla
            .modify(|_, w| w.dord().variant(dord));
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
            .spim()
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
        self.sercom.spim().ctrla.modify(|_, w| w.ibon().bit(set));
    }

    /// Run in standby mode
    ///
    /// When set, the SPI peripheral will run in standby mode. See the datasheet
    /// for more details.
    #[inline]
    pub fn run_in_standby(&mut self, set: bool) {
        self.sercom
            .spim()
            .ctrla
            .modify(|_, w| w.runstdby().bit(set));
    }

    /// Enable interrupts for the specified flags
    pub fn enable_interrupts(&mut self, flags: Flags) {
        self.sercom
            .spim()
            .intenset
            .write(|w| unsafe { w.bits(flags.bits()) });
    }

    /// Disable interrupts for the specified flags
    pub fn disable_interrupts(&mut self, flags: Flags) {
        self.sercom
            .spim()
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
        self.sercom.spim().ctrlb.modify(|_, w| w.rxen().set_bit());
        while self.sercom.spim().syncbusy.read().ctrlb().bit_is_set() {}
        self.sercom.spim().ctrla.modify(|_, w| w.enable().set_bit());
        while self.sercom.spim().syncbusy.read().enable().bit_is_set() {}
        Spi { config: self }
    }
}

impl<P, M> Config<P, M, DynLength>
where
    P: AnyPads,
    M: Mode,
{
    /// Return the current transaction length
    ///
    /// Read the LENGTH register to determine the current transaction length
    pub fn get_dyn_length(&self) -> u8 {
        self.sercom.spim().length.read().len().bits()
    }

    /// Set the transaction length
    ///
    /// Write the LENGTH register to set the transaction length. Panics if the
    /// length is zero.
    pub fn set_dyn_length(&mut self, length: u8) {
        if length == 0 {
            panic!("Cannot set SPI LENGTH field to zero")
        }
        self.sercom.spim().length.write(|w| unsafe {
            w.lenen().set_bit();
            w.len().bits(length)
        });
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
/// `Config<P, M, L>`. But when using this trait, only one type parameter is
/// required, i.e. `C: AnyConfig`. However, even though we have dropped type
/// parameters, no information is lost, because the [`Pads`], [`Mode`] and
/// [`Length`] type parameters are stored as associated types in the trait. The
/// implementation of [`AnyConfig`] looks like this:
///
/// ```
/// impl<P: Pads, M: Mode, L: Length> AnyConfig for Config<P, M, L> {
///     type Pads = P;
///     type Mode = M;
///     type Length = L;
///     // ...
/// }
/// ```
///
/// Thus, there is a one-to-one mapping between `Config<P, M, L>` and
/// `AnyConfig<Pads = P, Mode = M, Length = L>`, so you can always recover the
/// specific [`Config`] type from an implementation of [`AnyConfig`]. The type
/// alias [`SpecificConfig`] is provided for this purpose. You can convert
/// between [`AnyConfig`] and its corresponding [`SpecificConfig`] using the
/// [`Into`], [`AsRef`] and [`AsMut`] traits.
pub trait AnyConfig: Sealed + Is<Type = SpecificConfig<Self>> {
    type Pads: AnyPads;
    type Mode: Mode;
    type Length: Length;
}

/// Type alias to recover the specific [`Config`] type from an implementation of
/// [`AnyConfig`]
pub type SpecificConfig<C> =
    Config<<C as AnyConfig>::Pads, <C as AnyConfig>::Mode, <C as AnyConfig>::Length>;

/// Type alias to recover the [`Pads`] type from an implementation of
/// [`AnyConfig`]
pub type SpiPads<C> = <C as AnyConfig>::Pads;

/// Type alias to recover the [`Mode`] type from an implementation of
/// [`AnyConfig`]
pub type SpiMode<C> = <C as AnyConfig>::Mode;

/// Type alias to recover the [`Length`] type from an implementation of
/// [`AnyConfig`]
pub type SpiLength<C> = <C as AnyConfig>::Length;

/// Type alias to recover the [`Pads`]' [`Sercom`] type from an implementation
/// of [`AnyConfig`]
pub type SpiSercom<C> = PadsSercom<SpiPads<C>>;

/// Type alias to recover the [`Pads`]' [`IoSet`] type from an implementation of
/// [`AnyConfig`]
pub type SpiIoSet<C> = PadsIoSet<SpiPads<C>>;

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

/// Type alias to recover the [`Length`]'s [`Word`] type from an implementation
/// of [`AnyConfig`]
pub type SpiWord<C> = Word<SpiLength<C>>;

impl<P, M, L> Sealed for Config<P, M, L>
where
    P: AnyPads,
    M: Mode,
    L: Length,
{
}

impl<P, M, L> AnyConfig for Config<P, M, L>
where
    P: AnyPads,
    M: Mode,
    L: Length,
{
    type Pads = P;
    type Mode = M;
    type Length = L;
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

impl<P, L> ValidConfig for Config<P, Master, L>
where
    P: TxOrRx,
    L: Length,
{
}

impl<P, L> ValidConfig for Config<P, MasterHWSS, L>
where
    P: TxOrRx,
    L: Length,
    P::SS: SomePad,
{
}

impl<P, L> ValidConfig for Config<P, Slave, L>
where
    P: TxOrRx,
    L: Length,
    P::SS: SomePad,
{
}

//=============================================================================
// Spi
//=============================================================================

/// An enabled SPI peripheral that can perform transactions
///
/// As noted in the [`Mode`] and [`Length`] traits, this module chooses to
/// always operate in 32-bit extension mode and uses the LENGTH counter to set
/// the number of bytes in each transaction, from 1 to 255. In 32-bit extension
/// mode, transaction [`Length`]s of four bytes or fewer can be completed in a
/// single read or write of the DATA register. Longer transactions require
/// multiple reads or writes of the DATA register.
///
/// # Embedded HAL traits
///
/// The embedded HAL [`FullDuplex`] trait is non-blocking, which has
/// consequences for different transaction [`Length`]s. Transaction [`Length`]s
/// of 1-4 bytes fit the [`FullDuplex`] model well, because they can return
/// [`WouldBlock`] until it is possible to perform a single read or write that
/// completes the transaction. The [`FullDuplex`] word type is [`u8`] for 1-byte
/// transactions, [`u16`] for 2-byte transactions and [`u32`] for 3- and 4-byte
/// transactions.
///
/// For half-duplex, non-blocking transactions, this type implements the
/// `serial` [`Read`] and [`Write`] traits. They are only implemented for
/// transaction [`Length`]s `<= 4` and only when the [`Pads`] are not
/// [`Tx`]` + `[`Rx`], so they don't compete with [`FullDuplex`].
///
/// Longer [`Length`]s require multiple reads or writes, which does not fit the
/// [`FullDuplex`] model very well. When returning [`WouldBlock`], the
/// implementation would have to store some sort of internal state to track the
/// progress of an on-going transaction. As a consequence, [`FullDuplex`] is
/// only implemented for [`Length`]s of 1-4.
///
/// However, the [`blocking::spi`] traits are different, because they can always
/// execute a transaction to completion before returning. For [`Length`]s of
/// 1-4 bytes, these traits are implemented using an approach very similar to
/// the `Default` implementations. Longer [`Length`]s use a custom
/// implementation of [`Transfer`] and [`Write`](blocking::spi::Write) and
/// accept slices of `u8` with a length matching the transaction [`Length`].
///
/// When using a [`DynLength`], [`Spi`] only implements [`Transfer`] and
/// [`Write`](blocking::spi::Write), using the same approach as long transaction
/// [`Length`]s.
///
/// For a non-blocking alternative that can be used to transfer slices, see the
/// [`SpiFuture`] type.
///
/// # Further documentation
///
/// The individual implementations of the embedded HAL traits are well
/// documented. Unfortunately, however, rustdoc does not produce the most
/// visually scannable documentation, especially when using `typenum`. Here are
/// some tips for finding more information in the `impl` blocks below:
///
/// - [`FullDuplex`] uses a single blanket implementation for [`Length`]s of 1-4
/// - [`Read`] has two different implementations, one for [`Slave`] [`Mode`] and
///   another for [`MasterMode`]s
/// - [`Write`] uses a single blanket implementation, like [`FullDuplex`].
///   However, watch out for ambiguity between [`serial::Write`](Write),
///   [`blocking::serial::Write`] and [`blocking::spi::Write`].
/// - [`blocking::serial::Write`] uses the [`blocking::serial::write::Default`]
///   implementation
/// - [`blocking::spi::Transfer`] uses custom implementations. The
///   implementations for [`Length`]s of 1-4 are generated by macro, so there
///   are four different implementations for `U1` - `U4`. This is responsible
///   for most of the visual noise in the docs. [`Length`]s greater than four
///   use a blanket implementation. [`DynLength`] uses its own implementation
///   with an added run-time check of the transaction length.
/// - [`blocking::spi::Write`] takes the same approach as [`Transfer`].
/// - [`blocking::spi::WriteIter`] is implemented via macro for [`Length`]s 1-4,
///   but it is not implemented for longer [`Length`]s or [`DynLength`].
///
/// [`SpiFuture`]: crate::sercom::v2::spi_future::SpiFuture
/// [`Transfer`]: blocking::spi::Transfer
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

    /// Change the transaction [`Length`]
    ///
    /// Changing the transaction [`Length`] while is enabled is permissible but
    /// `unsafe`. If you have sent or received *any* bytes at the current
    /// [`Length`], you **must** wait for a TXC flag before changing to a new
    /// [`Length`].
    #[inline]
    pub unsafe fn length<L: Length>(self) -> Spi<Config<C::Pads, C::Mode, L>>
    where
        Config<C::Pads, C::Mode, L>: ValidConfig,
    {
        Spi {
            config: self.config.into().length(),
        }
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
        let bits = unsafe { self.sercom().spim().intflag.read().bits() };
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
        unsafe { self.sercom().spim().intflag.write(|w| w.bits(flags.bits())) };
    }

    /// Read the error status flags
    #[inline]
    pub fn read_errors(&self) -> Errors {
        let bits = unsafe { self.sercom().spim().status.read().bits() };
        Errors::from_bits_truncate(bits)
    }

    /// Clear error status flags
    ///
    /// Setting a flag will clear the error. Clearing any flag will have no
    /// effect.
    #[inline]
    pub fn clear_errors(&mut self, errors: Errors) {
        unsafe { self.sercom().spim().status.write(|w| w.bits(errors.bits())) };
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
    pub unsafe fn read_data(&mut self) -> u32 {
        self.sercom().spim().data.read().bits()
    }

    /// Write to the DATA register
    ///
    /// Writing to the data register directly is `unsafe`, because it will clear
    /// the DRE flag, which could break assumptions made elsewhere in this
    /// module.
    #[inline]
    pub unsafe fn write_data(&mut self, data: u32) {
        self.sercom().spim().data.write(|w| w.bits(data))
    }

    /// Disable the SPI peripheral and return the [`Config`] struct
    #[inline]
    pub fn disable(self) -> C {
        // SAFETY: The read state must be reset when disabling the peripheral
        unsafe { Config::<C::Pads>::reset_serial_read_state() };
        let spim = unsafe { self.sercom().spim() };
        spim.ctrlb.modify(|_, w| w.rxen().clear_bit());
        while spim.syncbusy.read().ctrlb().bit_is_set() {}
        spim.ctrla.modify(|_, w| w.enable().clear_bit());
        while spim.syncbusy.read().enable().bit_is_set() {}
        self.config
    }
}

impl<C> Spi<C>
where
    C: ValidConfig<Length = DynLength>,
{
    /// Return the current transaction length
    ///
    /// Read the LENGTH register to determine the current transaction length
    pub fn get_dyn_length(&self) -> u8 {
        self.config.as_ref().get_dyn_length()
    }

    /// Set the transaction length
    ///
    /// Write the LENGTH register to set the transaction length. Panics if the
    /// length is zero.
    ///
    /// # Safety
    ///
    /// If you have sent any data at the current transaction length, you
    /// **must** wait for `TXC` before changing the length.
    pub unsafe fn set_dyn_length(&mut self, length: u8) {
        self.config.as_mut().set_dyn_length(length)
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
        SpecificConfig::<C>::reset_serial_read_state();
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
impl<P, M, L> Read<L::Word> for Spi<Config<P, M, L>>
where
    Config<P, M, L>: ValidConfig,
    P: Rx + NotTx,
    M: MasterMode,
    L: Length,
    L::Word: PrimInt,
    u32: AsPrimitive<L::Word>,
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
    fn read(&mut self) -> nb::Result<L::Word, Error> {
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
impl<P, L> Read<L::Word> for Spi<Config<P, Slave, L>>
where
    Config<P, Slave, L>: ValidConfig,
    P: Rx + NotTx,
    L: Length,
    L::Word: PrimInt,
    u32: AsPrimitive<L::Word>,
{
    type Error = Error;

    /// Wait for an `RXC` flag, then read the word
    #[inline]
    fn read(&mut self) -> nb::Result<L::Word, Error> {
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
    SpiWord<C>: PrimInt + AsPrimitive<u32>,
{
    type Error = Error;

    /// Wait for a `DRE` flag, then write a word
    #[inline]
    fn write(&mut self, word: SpiWord<C>) -> nb::Result<(), Error> {
        // Ignore buffer overflow errors
        if self.read_errors().contains(Errors::LENERR) {
            Err(Error::LengthError.into())
        } else if self.read_flags().contains(Flags::DRE) {
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
        if self.read_errors().contains(Errors::LENERR) {
            Err(Error::LengthError.into())
        } else if self.read_flags().contains(Flags::TXC) {
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

/// Implement [`FullDuplex`] for short [`Spi`] transaction [`Length`]s
///
/// [`FullDuplex` is only implemented when [`Pads`] is both [`Tx`] and [`Rx`],
/// the [`Mode`] is a [`MasterMode`], and the transaction [`Length`] is `<= 4`
/// bytes. When the [`Length`] is `<= 4`, the [`SpiWord`] is a primitive
/// integer, with a size that depends on the [`Length`] (`u8`, `u16` or `u32`).
impl<C> FullDuplex<SpiWord<C>> for Spi<C>
where
    C: ValidConfig,
    C::Pads: Tx + Rx,
    C::Mode: MasterMode,
    SpiWord<C>: PrimInt + AsPrimitive<u32>,
    u32: AsPrimitive<SpiWord<C>>,
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

macro_rules! impl_blocking_traits {
    ( $($Length:ident),+ ) => {
        $(

            /// Implement [`Transfer`] for short [`Spi`] transaction [`Length`]s
            ///
            /// The [`Spi`] [`Pads`] must be [`Rx`] and the transaction
            /// [`Length`] must be `<= 4`. The transfer accepts a slice of
            /// primitive integers, depending on the [`Length`] (`u8`, `u16` or
            /// `u32`).
            ///
            /// [`Transfer`]: blocking::spi::Transfer
            impl<P, M> blocking::spi::Transfer<Word<$Length>> for Spi<Config<P, M, $Length>>
            where
                P: Rx,
                M: Mode,
                Config<P, M, $Length>: ValidConfig,
            {
                type Error = Error;

                #[inline]
                fn transfer<'w>(&mut self, words: &'w mut [Word<$Length>]) -> Result<&'w [Word<$Length>], Error> {
                    for word in words.iter_mut() {
                        loop {
                            let flags = self.read_flags_errors()?;
                            if flags.contains(Flags::DRE) {
                                unsafe { self.write_data(*word as u32) };
                                break
                            }
                        }
                        loop {
                            let flags = self.read_flags_errors()?;
                            if flags.contains(Flags::RXC) {
                                *word = unsafe { self.read_data() as Word<$Length> };
                                break
                            }
                        }
                    }
                    Ok(words)
                }
            }

            /// Implement [`Write`] for short [`Spi`] transaction [`Length`]s
            ///
            /// The [`Spi`] [`Pads`] must be [`Tx`] but [`NotRx`] and the
            /// transaction [`Length`] must be `<= 4`. The transfer accepts a
            /// slice of primitive integers, depending on the [`Length`] (`u8`,
            /// `u16` or `u32`).
            ///
            /// Because [`Write`] is only implemented when the [`Pads`] are
            /// [`NotRx`], this implementation never reads the DATA register and
            /// ignores all buffer overflow errors.
            ///
            /// [`Write`]: blocking::spi::Write
            impl<P, M> blocking::spi::Write<Word<$Length>> for Spi<Config<P, M, $Length>>
            where
                P: Tx + NotRx,
                M: Mode,
                Config<P, M, $Length>: ValidConfig,
            {
                type Error = Error;

                #[inline]
                fn write(&mut self, words: &[Word<$Length>]) -> Result<(), Error> {
                    for word in words {
                        loop {
                            // Ignore buffer overflow errors
                            if self.read_errors().contains(Errors::LENERR) {
                                return Err(Error::LengthError)
                            } else if self.read_flags().contains(Flags::DRE) {
                                unsafe { self.write_data(*word as u32) };
                                break
                            }
                        }
                    }
                    Ok(())
                }
            }

            /// Implement [`WriteIter`] for short [`Spi`] transaction [`Length`]s
            ///
            /// The [`Spi`] [`Pads`] must be [`Tx`] but [`NotRx`] and the
            /// transaction [`Length`] must be `<= 4`. The transfer accepts an
            /// iterator that yields primitive integers, depending on the
            /// [`Length`] (`u8`, `u16` or `u32`).
            ///
            /// Because [`WriteIter`] is only implemented when the [`Pads`] are
            /// [`NotRx`], this implementation never reads the DATA register and
            /// ignores all buffer overflow errors.
            ///
            /// [`WriteIter`]: blocking::spi::WriteIter
            #[cfg(feature = "unproven")]
            impl<P, M> blocking::spi::WriteIter<Word<$Length>> for Spi<Config<P, M, $Length>>
            where
                P: Tx + NotRx,
                M: Mode,
                Config<P, M, $Length>: ValidConfig,
            {
                type Error = Error;

                #[inline]
                fn write_iter<WI>(&mut self, words: WI) -> Result<(), Error>
                where
                    WI: IntoIterator<Item = Word<$Length>>,
                {
                    for word in words.into_iter() {
                        loop {
                            // Ignore buffer overflow errors
                            if self.read_errors().contains(Errors::LENERR) {
                                return Err(Error::LengthError)
                            } else if self.read_flags().contains(Flags::DRE) {
                                unsafe { self.write_data(word as u32) };
                                break
                            }
                        }
                    }
                    Ok(())
                }
            }
        )+
    };
}

impl_blocking_traits!(U1, U2, U3, U4);

/// Implement [`Transfer`] for longer [`Spi`] transaction [`Length`]s
///
/// The [`Spi`] [`Pads`] must be [`Rx`] and the transaction [`Length`] must be
/// `> 4`. The transfer accepts a slice of `u8` with a length equal to the
/// transaction [`Length`]. If the slice length is incorrect, it will panic.
///
/// [`Transfer`]: blocking::spi::Transfer
impl<'a, P, M, L> blocking::spi::Transfer<u8> for Spi<Config<P, M, L>>
where
    Config<P, M, L>: ValidConfig,
    P: Rx,
    M: Mode,
    L: GreaterThan4,
{
    type Error = Error;

    #[inline]
    fn transfer<'w>(&mut self, buf: &'w mut [u8]) -> Result<&'w [u8], Error> {
        assert_eq!(buf.len(), L::USIZE);
        let sercom = unsafe { self.sercom() };
        transfer_slice(sercom, buf)
    }
}

/// Implement [`Transfer`] for [`Spi`] types with [`DynLength`]
///
/// The [`Spi`] [`Pads`] must be [`Rx`]. The transfer accepts a slice of `u8`
/// with a length equal to the run-time dynamic transaction length. If the slice
/// length does not match the result of [`Spi::get_dyn_length`], it will panic.
///
/// [`Transfer`]: blocking::spi::Transfer
impl<P, M> blocking::spi::Transfer<u8> for Spi<Config<P, M, DynLength>>
where
    P: Rx,
    M: Mode,
    Config<P, M, DynLength>: ValidConfig<Length = DynLength>,
{
    type Error = Error;

    #[inline]
    fn transfer<'w>(&mut self, buf: &'w mut [u8]) -> Result<&'w [u8], Error> {
        assert_eq!(buf.len(), self.get_dyn_length() as usize);
        let sercom = unsafe { self.sercom() };
        transfer_slice(sercom, buf)
    }
}

/// Transfer a slice over SPI
///
/// This function exists to avoid monomorphization code bloat
fn transfer_slice<'w>(sercom: &RegisterBlock, buf: &'w mut [u8]) -> Result<&'w [u8], Error> {
    let cells = core::cell::Cell::from_mut(buf).as_slice_of_cells();
    let mut to_send = cells.iter();
    let mut to_recv = cells.iter();
    while to_recv.len() > 0 {
        let errors = sercom.spim().status.read();
        if errors.bufovf().bit_is_set() {
            return Err(Error::Overflow);
        } else if errors.lenerr().bit_is_set() {
            return Err(Error::LengthError);
        }
        let flags = sercom.spim().intflag.read();
        if to_send.len() > 0 && flags.dre().bit_is_set() {
            let mut bytes = [0; 4];
            for byte in bytes.iter_mut() {
                match to_send.next() {
                    Some(cell) => *byte = cell.get(),
                    None => break,
                }
            }
            let word = u32::from_le_bytes(bytes);
            sercom.spim().data.write(|w| unsafe { w.data().bits(word) });
        }
        if to_recv.len() > to_send.len() && flags.rxc().bit_is_set() {
            let word = sercom.spim().data.read().data().bits();
            let bytes = word.to_le_bytes();
            for byte in bytes.iter() {
                match to_recv.next() {
                    Some(cell) => cell.set(*byte),
                    None => break,
                }
            }
        }
    }
    Ok(buf)
}

/// Implement [`Write`] for longer [`Spi`] transaction [`Length`]s
///
/// The [`Spi`] [`Pads`] must be [`Tx`] but [`NotRx`] and the transaction
/// [`Length`] must be `> 4`. The transfer accepts a slice of `u8` with a length
/// equal to the transfer [`Length`]. If the slice length is incorrect, it will
/// panic.
///
/// Because [`Write`] is only implemented when the [`Pads`] are [`NotRx`], this
/// implementation never reads the DATA register and ignores all buffer overflow
/// errors.
///
/// [`Write`]: blocking::spi::Write
impl<P, M, L> blocking::spi::Write<u8> for Spi<Config<P, M, L>>
where
    Config<P, M, L>: ValidConfig,
    P: Tx,
    M: Mode,
    L: GreaterThan4,
{
    type Error = Error;

    #[inline]
    fn write(&mut self, buf: &[u8]) -> Result<(), Error> {
        assert_eq!(buf.len(), L::USIZE);
        let sercom = unsafe { self.sercom() };
        write_slice(sercom, buf)
    }
}

/// Implement [`Write`] for [`Spi`] types with [`DynLength`]
///
/// The [`Spi`] [`Pads`] must be [`Tx`]. The transfer accepts a slice of `u8`
/// with a length equal to the run-time dynamic transaction length. If the slice
/// length does not match the result of [`Spi::get_dyn_length`], it will panic.
///
/// Because [`Write`] is only implemented when the [`Pads`] are [`NotRx`], this
/// implementation never reads the DATA register and ignores all buffer overflow
/// errors.
///
/// [`Write`]: blocking::spi::Write
impl<P, M> blocking::spi::Write<u8> for Spi<Config<P, M, DynLength>>
where
    P: Tx,
    M: Mode,
    Config<P, M, DynLength>: ValidConfig<Length = DynLength>,
{
    type Error = Error;

    #[inline]
    fn write(&mut self, buf: &[u8]) -> Result<(), Error> {
        assert_eq!(buf.len(), self.get_dyn_length() as usize);
        let sercom = unsafe { self.sercom() };
        write_slice(sercom, buf)
    }
}

/// Write a slice over SPI
///
/// This function exists to avoid monomorphization code bloat
fn write_slice(sercom: &RegisterBlock, buf: &[u8]) -> Result<(), Error> {
    let mut data = buf.iter();
    while data.len() > 0 {
        let errors = sercom.spim().status.read();
        if errors.lenerr().bit_is_set() {
            return Err(Error::LengthError);
        }
        if sercom.spim().intflag.read().dre().bit_is_set() {
            let mut bytes = [0; 4];
            for byte in bytes.iter_mut() {
                match data.next() {
                    Some(d) => *byte = *d,
                    None => break,
                }
            }
            let word = u32::from_le_bytes(bytes);
            sercom.spim().data.write(|w| unsafe { w.data().bits(word) });
        }
    }
    Ok(())
}
