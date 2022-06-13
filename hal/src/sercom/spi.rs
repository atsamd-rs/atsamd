//! Use a SERCOM peripheral for SPI transactions
//!
//! Using an SPI peripheral occurs in three steps. First, you must supply
//! [`gpio`] [`Pin`]s to create a set of [`Pads`]. Next, you combine the
//! `Pads` with other pieces to form a [`Config`] struct. Finally, after
//! configuring the peripheral, you [`enable`] it to yield a functional
//! [`Spi`] struct. Transactions are performed using traits from the
//! [`embedded_hal`] crate, specifically those from the
//! [`spi`](embedded_hal::spi), [`serial`](embedded_hal::serial), and
//! [`blocking`](embedded_hal::blocking) modules.
//!
//! # Crating a set of [`Pads`]
//!
//! An SPI peripheral can use up to four [`Pin`]s as [`Sercom`] pads. However,
//! only certain `Pin` combinations are acceptable. All `Pin`s must be mapped to
//! the same `Sercom`, and for SAMx5x chips, they must also belong to the same
#![cfg_attr(any(feature = "samd11", feature = "samd21"), doc = "`IoSet`.")]
#![cfg_attr(feature = "min-samd51g", doc = "[`IoSet`].")]
//! This HAL makes it impossible to use invalid `Pin` combinations, and the
//! [`Pads`] struct is responsible for enforcing these constraints.
//!
//! A `Pads` type takes five or six type parameters, depending on the chip. The
//! first type always specifies the `Sercom`. On SAMx5x chips, the second type
//! specifies the `IoSet`. The remaining four type parameters, `DI`, `DO`, `CK`
//! and `SS`, represent the Data In, Data Out, Sclk and SS pads respectively.
//! Each of these type parameters is an [`OptionalPad`] and defaults to
//! [`NoneT`]. A `Pad` is just a `Pin` configured in the correct [`PinMode`]
//! that implements [`IsPad`]. The [`bsp_pins!`](crate::bsp_pins) macro can be
//! used to define convenient type aliases for `Pad` types.
//!
//! ```
//! use atsamd_hal::gpio::{PA08, PA09, AlternateC};
//! use atsamd_hal::sercom::{Sercom0, spi};
//! use atsamd_hal::typelevel::NoneT;
//!
//! // SAMx5x-specific imports
//! use atsamd_hal::sercom::pad::IoSet1;
//!
//! type Miso = Pin<PA08, AlternateC>;
//! type Sclk = Pin<PA09, AlternateC>;
//!
//! // SAMD11/SAMD21 version
//! type Pads = spi::Pads<Sercom0, Miso, NoneT, Sclk>;
//! // SAMx5x version
//! type Pads = spi::Pads<Sercom0, IoSet1, Miso, NoneT, Sclk>;
//! ```
//!
//! [`enable`]: Config::enable
//! [`gpio`]: crate::gpio
//! [`Pin`]: crate::gpio::pin::Pin
//! [`PinId`]: crate::gpio::pin::PinId
//! [`PinMode`]: crate::gpio::pin::PinMode
#![cfg_attr(
    not(feature = "samd11"),
    doc = "
Alternatively, you can use the `PadsFromIds` alias to define a set of
`Pads` in terms of [`PinId`]s instead of [`Pin`]s. This is useful when you
don't have [`Pin`] aliases pre-defined.

```
use atsamd_hal::gpio::{PA08, PA09};
use atsamd_hal::sercom::{Sercom0, spi};
use atsamd_hal::typelevel::NoneT;

// SAMx5x-specific imports
use atsamd_hal::sercom::pad::IoSet1;

// SAMD21 version
type Pads = spi::PadsFromIds<Sercom0, PA08, NoneT, PA09>;
// SAMx5x version
type Pads = spi::PadsFromIds<Sercom0, IoSet1, PA08, NoneT, PA09>;
```
"
)]
//!
//! Instances of `Pads` are created using the builder pattern. Start by creating
//! an empty set of `Pads` using [`Default`]. Then pass each respective `Pin`
//! using the corresponding methods. For SAMD21 and SAMx5x chips, the builder
//! methods automatically convert each pin to the correct [`PinMode`]. However,
//! due to inherent ambiguities, users must manually configure `PinMode`s for
//! SAMD11 chips.
//!
//! ```
//! use atsamd_hal::target_device::Peripherals;
//! use atsamd_hal::gpio::Pins;
//! use atsamd_hal::sercom::{Sercom0, spi};
//!
//! // SAMx5x-specific imports
//! use atsamd_hal::sercom::pad::IoSet1;
//!
//! let mut peripherals = Peripherals::take().unwrap();
//! let pins = Pins::new(peripherals.PORT);
//! // SAMD21 version
//! let pads = spi::Pads::<Sercom0>::default()
//!     .sclk(pins.pa09)
//!     .data_in(pins.pa08)
//!     .data_out(pins.pa11);
//! // SAMx5x version
//! let pads = spi::Pads::<Sercom0, IoSet1>::default()
//!     .sclk(pins.pa09)
//!     .data_in(pins.pa08)
//!     .data_out(pins.pa11);
//! ```
//!
//! To be accepted by the [`Config`] struct as a set of [`ValidPads`], the
//! `Pads` must do two things:
//! - Specify [`SomePad`] for `CK` and at least one of `DI` or `DO`
//! - Use a valid combination of [`PadNum`]s, so that the `Pads` implement
//!   [`DipoDopo`]
//!
//! # `Config`uring the peripheral
//!
//! Next, create a [`Config`] struct, which represents the SPI peripheral in its
//! disabled state. A `Config` is specified with three type parameters: the
//! [`Pads`] type; an [`OpMode`], which defaults to [`Master`]; and a
//! [`Size`] type that varies by chip. [`Size`] essentially acts as a trait
//! alias. On SAMD11 and SAMD21 chips, it represents the
#![cfg_attr(
    any(feature = "samd11", feature = "samd21"),
    doc = "[`CharSize`], which can either be [`EightBit`] or [`NineBit`]. "
)]
#![cfg_attr(
    feature = "min-samd51g",
    doc = "`CharSize`, which can either be `EightBit` or `NineBit`. "
)]
//! While on SAMx5x chips, it represents the transaction
#![cfg_attr(any(feature = "samd11", feature = "samd21"), doc = "`Length`")]
#![cfg_attr(feature = "min-samd51g", doc = "[`Length`]")]
//! in bytes, using type-level numbers provided by the [`typenum`] crate. Valid
//! transaction lengths, from `U1` to `U255`, are re-exported in the
#![cfg_attr(any(feature = "samd11", feature = "samd21"), doc = "`lengths`")]
#![cfg_attr(feature = "min-samd51g", doc = "[`lengths`]")]
//! sub-module.
//!
//! ```
//! use atsamd_hal::gpio::{PA08, PA09};
//! use atsamd_hal::sercom::{Sercom0, spi};
//! use atsamd_hal::sercom::spi::Master;
//! use atsamd_hal::typelevel::NoneT;
//!
//! // SAMD11/SAMD21-specific imports
//! use atsamd_hal::sercom::spi::NineBit;
//!
//! // SAMx5x-specific imports
//! use atsamd_hal::sercom::spi::lengths::U2;
//! use atsamd_hal::sercom::pad::IoSet1;
//!
//! // SAMD11/SAMD21 version
//! type Pads = spi::PadsFromIds<Sercom0, PA08, NoneT, PA09>;
//! type Config = spi::Config<Pads, Master, NineBit>;
//!
//! // SAMx5x version
//! type Pads = spi::PadsFromIds<Sercom0, IoSet1, PA08, NoneT, PA09>;
//! type Config = spi::Config<Pads, Master, U2>;
//! ```
//!
//! For simplicity, this module ignores character size on SAMx5x chips. Instead,
//! the SPI peripheral is always configured to use 32-bit extension mode and the
//! hardware `LENGTH` counter. Note that, due to a hardware bug, `ICSPACE` must
//! be at least one when using the length counter. See the silicon errata for
//! more details.
//!
//! Upon creation, the [`Config`] takes ownership of both the [`Pads`] and the
//! PAC [`Sercom`] struct. It takes a reference to the `PM` or `MCLK`, so that
//! it can enable the APB clock, and it takes a frequency to indicate the GCLK
//! configuration. Users are responsible for correctly configuring the GCLK.
//!
//! ```
//! use atsamd_hal::time::U32Ext;
//!
//! // Not shown: configure GCLK for 10 MHz
//!
//! // SAMD11/SAMD21 version
//! let pm = peripherals.PM;
//! let sercom = peripherals.SERCOM0;
//! let freq = 10.mhz();
//! let config = spi::Config::new(&pm, sercom, pads, freq);
//!
//! // SAMx5x version
//! let mclk = peripherals.MCLK;
//! let sercom = peripherals.SERCOM0;
//! let freq = 10.mhz();
//! let config = spi::Config::new(&mclk, sercom, pads, freq);
//! ```
//!
//! The [`Config`] uses two different APIs for configuration. For most
//! parameters, it provides `get_` and `set_` methods that take `&self` and
//! `&mut self` respectively, e.g. [`get_bit_order`](Config::get_bit_order) and
//! [`set_bit_order`](Config::set_bit_order). However, because `Config` tracks
//! the [`OpMode`] and [`Size`] at compile-time, which requires changing the
//! corresponding type parameters, `Config` also provides a builder-pattern API,
//! where methods take and return `self`, e.g. [`bit_order`](Config::bit_order).
//!
//! Once configured, the [`enable`] method consumes the `Config` and returns an
//! enabled [`Spi`] struct that can be used for transactions. Because the
//! `enable` function takes the `Config` as `self`, the builder-pattern API is
//! usually the more ergonomic option.
//!
//! ```
//! use embedded_hal::spi::MODE_1;
//!
//! // SAMD11/SAMD21 version
//! let spi = spi::Config::new(&pm, sercom, pads, freq)
//!     .baud(1.mhz())
//!     .char_size::<NineBit>()
//!     .bit_order(BitOrder::LsbFirst)
//!     .spi_mode(MODE_1)
//!     .enable();
//!
//! // SAMx5x version
//! let spi = spi::Config::new(&mclk, sercom, pads, freq)
//!     .baud(1.mhz())
//!     .length::<U2>()
//!     .bit_order(BitOrder::LsbFirst)
//!     .spi_mode(MODE_1)
//!     .enable();
//! ```
//!
//! To be accepted as a [`ValidConfig`], the `Config` must have a set of
//! [`ValidPads`] that matches its [`OpMode`]. In particular, the `SS` pad must
//! be [`NoneT`] for [`Master`] mode, where the user is expected to handle it
//! manaully. But it must be [`SomePad`] in [`MasterHWSS`] and [`Slave`] modes,
//! where it is controlled by the hardware.
//!
//! # Using a functional `Spi` peripheral
//!
//! An [`Spi`] struct has two type parameters. The first is the corresponding
//! `Config`, while the second represents its [`Capability`], i.e. [`Rx`],
//! [`Tx`] or [`Duplex`]. The [`enable`] function determines the `Capability`
//! automaically from the set of [`ValidPads`].
//!
//! ```
//! use atsamd_hal::gpio::{PA08, PA09};
//! use atsamd_hal::sercom::{Sercom0, spi};
//! use atsamd_hal::sercom::spi::{Master, Rx};
//! use atsamd_hal::typelevel::NoneT;
//!
//! // SAMD11/SAMD21-specific imports
//! use atsamd_hal::sercom::spi::NineBit;
//!
//! // SAMx5x-specific imports
//! use atsamd_hal::sercom::spi::lengths::U2;
//! use atsamd_hal::sercom::pad::IoSet1;
//!
//! // SAMD11/SAMD21 version
//! type Pads = spi::PadsFromIds<Sercom0, PA08, NoneT, PA09>;
//! type Config = spi::Config<Pads, Master, NineBit>;
//! type Spi = spi::Spi<Config, Rx>;
//!
//! // SAMx5x version
//! type Pads = spi::PadsFromIds<Sercom0, IoSet1, PA08, NoneT, PA09>;
//! type Config = spi::Config<Pads, Master, U2>;
//! type Spi = spi::Spi<Config, Rx>;
//! ```
//!
//! Only `Spi` structs can actually perform transactions. To do so, use the
//! various embedded HAL traits, like
//! [`spi::FullDuplex`](embedded_hal::spi::FullDuplex),
//! [`serial::Read`](embedded_hal::serial::Read) or
//! [`serial::Write`](embedded_hal::serial::Write).
//! See the [`impl_ehal`] module documentation for more details about the
//! specific trait implementations, which vary based on [`Size`] and
//! [`Capability`].
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
//! [`gpio`]: crate::gpio
//! [`Pin`]: crate::gpio::pin::Pin
//! [`PinId`]: crate::gpio::pin::PinId
//! [`PinMode`]: crate::gpio::pin::PinMode
#![cfg_attr(
    feature = "dma",
    doc = "
# Using SPI with DMA

This HAL includes support for DMA-enabled SPI transfers. An enabled [`Spi`]
struct implements the DMAC [`Buffer`] trait. The provided [`send_with_dma`]
and [`receive_with_dma`] methods will build and begin a [`dmac::Transfer`]
to create a non-blocking SPI transfer.

Optionally, interrupts can be enabled on the provided [`Channel`]. Note that
the `dma` feature must be enabled. Refer to the [`dmac`] module-level
documentation for more information.

```
// Assume channel is a configured `dmac::Channel`, and spi a
// fully-configured `Spi`

// Create data to send
let buffer: [u8; 50] = [0xff; 50]

// Launch the transfer
let dma_transfer = spi.send_with_dma(&mut buffer, channel, ());

// Wait for the transfer to complete and reclaim resources
let (chan0, _, spi, _) = dma_transfer.wait();
```

[`Buffer`]: crate::dmac::transfer::Buffer
[`send_with_dma`]: Spi::send_with_dma
[`receive_with_dma`]: Spi::receive_with_dma
[`dmac::Transfer`]: crate::dmac::Transfer
[`Channel`]: crate::dmac::channel::Channel
[`dmac`]: crate::dmac
"
)]

//=============================================================================
// Imports
//=============================================================================

use core::marker::PhantomData;

use num_traits::{AsPrimitive, PrimInt};

use crate::sercom::{Sercom, APB_CLK_CTRL};
use crate::time::Hertz;
use crate::typelevel::{Is, Sealed};

mod reg;
pub use reg::*;

mod config;
pub use config::*;

//=============================================================================
// Chip-specific imports
//=============================================================================

#[cfg(any(feature = "samd11", feature = "samd21"))]
use crate::pac::sercom0::spi::ctrla::MODE_A;
#[cfg(feature = "min-samd51g")]
use crate::pac::sercom0::spim::ctrla::MODE_A;

#[cfg(any(feature = "samd11", feature = "samd21"))]
#[path = "spi/pads_thumbv6m.rs"]
mod pads;

#[cfg(feature = "min-samd51g")]
#[path = "spi/pads_thumbv7em.rs"]
mod pads;

pub use pads::*;

#[cfg(any(feature = "samd11", feature = "samd21"))]
#[path = "spi/char_size.rs"]
mod size;

#[cfg(feature = "min-samd51g")]
#[path = "spi/length.rs"]
mod size;

pub use size::*;

/// Valid transaction [`Length`]s from the [`typenum`] crate
#[cfg(feature = "min-samd51g")]
pub mod lengths {
    seq_macro::seq!(N in 1..=255 {
        pub use typenum::U~N;
    });
}

mod impl_ehal_common;

#[cfg(any(feature = "samd11", feature = "samd21"))]
#[path = "spi/impl_ehal_thumbv6m.rs"]
pub mod impl_ehal;

#[cfg(feature = "min-samd51g")]
#[path = "spi/impl_ehal_thumbv7em.rs"]
pub mod impl_ehal;

//==============================================================================
// DynCapability
//==============================================================================

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum DynCapability {
    Rx,
    Tx,
    Duplex,
}

impl Sealed for DynCapability {}

//==============================================================================
// Capability
//==============================================================================

/// Type-level enum representing the simplex or duplex transaction capability
///
/// The available, type-level variants are [`Rx`], [`Tx`] and [`Duplex`]. See
/// the [type-level enum] documentation for more details.
///
/// [type-level enum]: crate::typelevel#type-level-enums
pub trait Capability: Sealed + Default {
    const DYN: DynCapability;
}

/// Type-level variant of the [`Capability`] enum for simplex, [`Receive`]-only
/// transactions
///
/// [`Spi`] structs are `Rx` when the `DO` (Data Out) type is [`NoneT`] in the
/// corresponding [`Pads`] struct.
///
/// While the [`Tx`] and [`Duplex`] structs are zero-sized, this struct is not.
/// Because an SPI master must initiate all transactions, using it in a simplex,
/// [`Receive`]-only context is slightly complicated. In that case, the [`Spi`]
/// struct must track whether a transaction needs to be started or is already in
/// progress. This struct contains a `bool` to track that progress.
#[derive(Default)]
pub struct Rx {
    pub(super) in_progress: bool,
}

/// Type-level variant of the [`Capability`] enum for simplex, [`Transmit`]-only
/// transactions
///
/// [`Spi`] structs are `Tx` when the `DI` (Data In) type is [`NoneT`] in the
/// corresponding [`Pads`] struct.
#[derive(Default)]
pub struct Tx;

/// Type-level variant of the [`Capability`] enum for duplex transactions
///
/// [`Spi`] structs are `Duplex` when both the `DI` and `DO` [`Pads`] are
/// [`SomePad`].
/// corresponding [`Pads`] struct.
#[derive(Default)]
pub struct Duplex;

macro_rules! impl_capability {
    ( $( $Cap: ident ),+ ) => {
        $(
            impl Sealed for $Cap {}
            impl Capability for $Cap {
                const DYN: DynCapability = DynCapability::$Cap;
            }
        )+
    };
}

impl_capability!(Rx, Tx, Duplex);

//=============================================================================
// Receive
//=============================================================================

/// Sub-set of [`Capability`] variants that can receive data, i.e. [`Rx`] and
/// [`Duplex`]
pub trait Receive: Capability {}

impl Receive for Rx {}
impl Receive for Duplex {}

//=============================================================================
// Transmit
//=============================================================================

/// Sub-set of [`Capability`] variants that can transmit dat, i.e. [`Tx`] and
/// [`Duplex`]
pub trait Transmit: Capability {}

impl Transmit for Tx {}
impl Transmit for Duplex {}

//=============================================================================
// Operating mode
//=============================================================================

/// Type-level enum representing the SPI operating mode
///
/// See the documentation on [type-level enums] for a discussion of the pattern.
///
/// The available operating modes are [`Master`], [`MasterHWSS`] and [`Slave`].
/// In [`Master`] mode, the `SS` signal must be handled by the user, so `SS`
/// must be [`NoneT`]. In [`MasterHWSS`] mode, the hardware drives the `SS`
/// line, so [`SomePad`] is required. In [`Slave`] mode, the `SS` pad is
/// required as well, to indicate when data is valid.
///
/// [type-level enums]: crate::typelevel#type-level-enums
pub trait OpMode: Sealed {
    /// Corresponding variant from the PAC enum
    const MODE: MODE_A;
    /// Bit indicating whether hardware `SS` control is enabled
    const MSSEN: bool;
}

/// [`OpMode`] variant for Master mode
pub enum Master {}

/// [`OpMode`] variant for Master mode with hardware-controlled slave select
pub enum MasterHWSS {}

/// [`OpMode`] variant for Slave mode
pub enum Slave {}

impl Sealed for Master {}
impl Sealed for MasterHWSS {}
impl Sealed for Slave {}

impl OpMode for Master {
    const MODE: MODE_A = MODE_A::SPI_MASTER;
    const MSSEN: bool = false;
}

impl OpMode for MasterHWSS {
    const MODE: MODE_A = MODE_A::SPI_MASTER;
    const MSSEN: bool = true;
}

impl OpMode for Slave {
    const MODE: MODE_A = MODE_A::SPI_SLAVE;
    const MSSEN: bool = false;
}

/// Marker trait for Master operating modes
///
/// This trait is implemented for [`Master`] and [`MasterHWSS`] but not for
/// [`Slave`].
pub trait MasterMode: OpMode {}

impl MasterMode for Master {}
impl MasterMode for MasterHWSS {}

//=============================================================================
// Size
//=============================================================================

/// Trait alias whose definition varies by chip
///
/// On SAMD11 and SAMD21 chips, this represents the [`CharSize`].
#[cfg(any(feature = "samd11", feature = "samd21"))]
pub trait Size: CharSize {}

#[cfg(any(feature = "samd11", feature = "samd21"))]
impl<C: CharSize> Size for C {}

/// Type alias for the default [`Size`] type, which varies by chip
#[cfg(any(feature = "samd11", feature = "samd21"))]
pub type DefaultSize = EightBit;

/// Trait alias whose definition varies by chip
///
/// On SAMx5x chips, this represents the transaction [`Length`].
#[cfg(feature = "min-samd51g")]
pub trait Size: Length {}

#[cfg(feature = "min-samd51g")]
impl<L: Length> Size for L {}

/// Type alias for the default [`Size`] type, which varies by chip
#[cfg(feature = "min-samd51g")]
pub type DefaultSize = typenum::U1;

//==============================================================================
// AtomicSize
//==============================================================================

/// Marker trait for transaction [`Size`]s that can be completed in a single
/// read or write of the `DATA` register
pub trait AtomicSize: Size {}

#[cfg(any(feature = "samd11", feature = "samd21"))]
impl<C: CharSize> AtomicSize for C {}

#[cfg(feature = "min-samd51g")]
seq_macro::seq!(N in 1..=4 {
    impl AtomicSize for lengths::U~N {}
});

//==============================================================================
// NonAtomicSize
//==============================================================================

/// Marker trait for [`Size`]s that can't be completed in one transaction
#[cfg(feature = "min-samd51g")]
pub trait NonAtomicSize: Size<Word = u8> {}

#[cfg(feature = "min-samd51g")]
seq_macro::seq!(N in 5..=255 {
    impl NonAtomicSize for typenum::U~N {}
});

//=============================================================================
// Spi
//=============================================================================

/// An enabled SPI peripheral that can perform transactions
///
/// See the [`impl_ehal`] documentation for details on the implementations of
/// the embedded HAL traits, which vary based on [`Size`] and [`Capability`].
pub struct Spi<P, M = Master, Z = DefaultSize>
where
    P: ValidPads,
    M: OpMode,
    Z: Size,
{
    regs: Registers<P::Sercom>,
    pads: P,
    capability: P::Capability,
    mode: PhantomData<M>,
    size: PhantomData<Z>,
    freq: Hertz,
}

impl<P: ValidPads> Spi<P> {
    #[inline]
    pub fn config(
        apb_clk_ctrl: &APB_CLK_CTRL,
        sercom: P::Sercom,
        pads: P,
        freq: impl Into<Hertz>,
    ) -> Config<P> {
        Config::new(apb_clk_ctrl, sercom, pads, freq)
    }
}

impl<P, M, Z> Spi<P, M, Z>
where
    P: ValidPads,
    M: OpMode,
    Z: Size,
{
    /// Obtain a reference to the PAC `SERCOM` struct
    ///
    /// Directly accessing the `SERCOM` could break the invariants of the
    /// type-level tracking in this module, so it is unsafe.
    #[inline]
    pub unsafe fn sercom(&self) -> &P::Sercom {
        &self.regs.sercom
    }

    /// Obtain a pointer to the `DATA` register. Necessary for DMA transfers.
    #[inline]
    #[cfg(feature = "dma")]
    pub(super) fn data_ptr(&self) -> *mut Word<Z> {
        self.regs.data_ptr::<Z>()
    }

    /// Return the transaction length, in bytes
    ///
    /// This function is valid for all chips and SPI configurations. It returns
    /// the number of bytes in a single SPI transaction.
    #[inline]
    pub fn get_length(&self) -> u8 {
        #[cfg(any(feature = "samd11", feature = "samd21"))]
        let result = Z::BYTES;
        #[cfg(feature = "min-samd51g")]
        let result = match Z::static_length() {
            Some(length) => length.get(),
            None => self.regs.get_length(),
        };
        result
    }

    #[cfg(feature = "min-samd51g")]
    #[inline]
    fn change_length<L: Length>(self) -> Spi<P, M, L> {
        Spi {
            regs: self.regs,
            pads: self.pads,
            capability: self.capability,
            mode: self.mode,
            size: PhantomData,
            freq: self.freq,
        }
    }

    /// Change the transaction [`Length`]
    ///
    /// Changing the transaction [`Length`] while is enabled is permissible but
    /// dangerous. If you have sent or received *any* bytes at the current
    /// [`Length`], you **must** wait for a TXC flag before changing to a new
    /// [`Length`].
    #[cfg(feature = "min-samd51g")]
    #[inline]
    pub fn length<L: StaticLength>(mut self) -> Spi<P, M, L> {
        self.regs.set_length(L::LENGTH);
        self.change_length()
    }

    /// Change the transaction [`Length`]
    ///
    /// Changing the transaction [`Length`] while is enabled is permissible but
    /// dangerous. If you have sent or received *any* bytes at the current
    /// [`Length`], you **must** wait for a TXC flag before changing to a new
    /// [`Length`].
    #[cfg(feature = "min-samd51g")]
    #[inline]
    pub fn dyn_length(mut self, length: u8) -> Spi<P, M, DynLength> {
        let length = if length > 0 { length } else { 1 };
        self.regs.set_length(length);
        self.change_length()
    }

    /// Update the SPI configuration.
    ///
    /// Calling this method will temporarily disable the SERCOM peripheral, as
    /// some registers are enable-protected. This may interrupt any ongoing
    /// transactions.
    #[inline]
    pub fn reconfigure(&mut self, update: impl FnOnce(&mut Reconfig)) {
        self.regs.disable();
        let ctrla = self.regs.get_ctrla();
        let baud = self.regs.get_baud(self.freq);
        let mut reconfig = Reconfig { ctrla, baud };
        update(&mut reconfig);
        self.regs.set_ctrla(reconfig.ctrla);
        self.regs.set_baud(self.freq, reconfig.baud);
        self.regs.enable();
    }

    /// Enable interrupts for the specified flags
    #[inline]
    pub fn enable_interrupts(&mut self, flags: Flags) {
        self.regs.enable_interrupts(flags)
    }

    /// Disable interrupts for the specified flags
    #[inline]
    pub fn disable_interrupts(&mut self, flags: Flags) {
        self.regs.disable_interrupts(flags);
    }

    /// Read the interrupt flags
    #[inline]
    pub fn read_flags(&self) -> Flags {
        self.regs.read_flags()
    }

    /// Clear the corresponding interrupt flags
    ///
    /// Only the `ERROR`, `SSL` and `TXC` flags can be cleared.
    ///
    /// **⚠️Warning⚠️:** The implementation of of [`serial::Write::flush`] waits
    /// on and clears the `TXC` flag. Manually clearing this flag could
    /// cause it to hang indefinitely.
    ///
    /// [`serial::Write::flush`]: embedded_hal::serial::Write::flush
    #[inline]
    pub fn clear_flags(&mut self, flags: Flags) {
        self.regs.clear_flags(flags);
    }

    /// Read the error status flags
    #[inline]
    pub fn read_status(&self) -> Status {
        self.regs.read_status()
    }

    /// Clear the corresponding error status flags
    #[inline]
    pub fn clear_status(&mut self, status: Status) {
        self.regs.clear_status(status);
    }

    /// Try to read the interrupt flags, but first check the error status flags.
    #[inline]
    pub fn read_flags_errors(&self) -> Result<Flags, Error> {
        self.regs.read_flags_errors()
    }

    /// Private interface to read from the DATA register
    #[inline]
    pub(super) fn _read_data(&mut self) -> DataWidth {
        self.regs.read_data()
    }

    /// Private interface to write to the DATA register
    #[inline]
    pub(super) fn _write_data(&mut self, data: DataWidth) {
        self.regs.write_data(data);
    }

    /// Read from the DATA register
    ///
    /// Reading from the data register directly is `unsafe`, because it will
    /// clear the RXC flag, which could break assumptions made elsewhere in
    /// this module.
    #[inline]
    pub unsafe fn read_data(&mut self) -> DataWidth {
        self._read_data()
    }

    /// Write to the DATA register
    ///
    /// Writing to the data register directly is `unsafe`, because it will clear
    /// the DRE flag, which could break assumptions made elsewhere in this
    /// module.
    #[inline]
    pub unsafe fn write_data(&mut self, data: DataWidth) {
        self._write_data(data);
    }

    /// Reset the SPI peripheral and return the [`Config`] struct
    #[inline]
    pub fn disable(mut self) -> (P::Sercom, P) {
        self.regs.reset();
        (self.regs.sercom, self.pads)
    }
}

#[cfg(feature = "min-samd51g")]
impl<P, M> Spi<P, M, DynLength>
where
    P: ValidPads,
    M: OpMode,
{
    /// Change the transaction [`Length`]
    ///
    /// Changing the transaction [`Length`] while is enabled is permissible but
    /// dangerous. If you have sent or received *any* bytes at the current
    /// [`Length`], you **must** wait for a TXC flag before changing to a new
    /// [`Length`].
    #[cfg(feature = "min-samd51g")]
    #[inline]
    pub fn set_dyn_length(&mut self, length: u8) {
        let length = if length > 0 { length } else { 1 };
        self.regs.set_length(length);
    }
}

//=============================================================================
// AnySpi
//=============================================================================

/// Type class for all possible [`Spi`] types
///
/// This trait uses the [`AnyKind`] trait pattern to create a [type class] for
/// [`Spi`] types. See the `AnyKind` documentation for more details on the
/// pattern.
///
/// In addition to the normal, `AnyKind` associated types. This trait also
/// copies the [`Sercom`], [`Pads`], [`Capability`], [`OpMode`], [`Size`] and
/// [`Word`] types, to make it easier to apply bounds to these types at the next
/// level of abstraction.
///
/// [`AnyKind`]: crate::typelevel#anykind-trait-pattern
/// [type class]: crate::typelevel#type-classes
pub trait AnySpi: Is<Type = SpecificSpi<Self>> {
    type Sercom: Sercom;
    type Pads: ValidPads<Sercom = Self::Sercom, Capability = Self::Capability>;
    type Capability: Capability;
    type OpMode: OpMode;
    type Size: Size<Word = Self::Word>;
    type Word: PrimInt + AsPrimitive<DataWidth>;
}

/// Type alias to recover the specific [`Spi`] type from an implementation of
/// [`AnySpi`]
pub type SpecificSpi<S> = Spi<<S as AnySpi>::Pads, <S as AnySpi>::OpMode, <S as AnySpi>::Size>;

impl<P, M, Z> AsRef<Self> for Spi<P, M, Z>
where
    P: ValidPads,
    M: OpMode,
    Z: Size,
{
    #[inline]
    fn as_ref(&self) -> &Self {
        self
    }
}

impl<P, M, Z> AsMut<Self> for Spi<P, M, Z>
where
    P: ValidPads,
    M: OpMode,
    Z: Size,
{
    #[inline]
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl<P, M, Z> Sealed for Spi<P, M, Z>
where
    P: ValidPads,
    M: OpMode,
    Z: Size,
{
}

impl<P, M, Z> AnySpi for Spi<P, M, Z>
where
    P: ValidPads,
    M: OpMode,
    Z: Size,
{
    type Sercom = P::Sercom;
    type Pads = P;
    type Capability = P::Capability;
    type OpMode = M;
    type Size = Z;
    type Word = Z::Word;
}
