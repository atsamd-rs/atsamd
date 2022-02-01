//! Use the SERCOM peripheral for I2C communications
//!
//! Configuring an I2C peripheral occurs in three steps. First, you must create
//! a set of [`Pads`] for use by the peripheral. Next, you assemble pieces into
//! a [`Config`] struct. After configuring the peripheral, you then [`enable`]
//! it, yielding a functional [`I2c`] struct.
//! Transactions are performed using the [`i2c`](embedded_hal::blocking::i2c)
//! traits from embedded HAL.
//!
//! # [`Pads`]
//!
//! A [`Sercom`] uses two [`Pin`]s as peripheral [`Pad`]s, but only
//! certain [`Pin`] combinations are acceptable. In particular, all [`Pin`]s
//! must be mapped to the same [`Sercom`], and SDA is always [`Pad0`], while SCL
//! is always [`Pad1`] (see the datasheet). This HAL makes it impossible to use
//! invalid [`Pin`]/[`Pad`] combinations, and the [`Pads`] struct is responsible
//! for enforcing these constraints.
//!
//!
//! A [`Pads`] type takes three or four type parameters, depending on the chip.
//! The first type always specifies the [`Sercom`]. On SAMx5x chips, the second
//! type specifies the `IoSet`. The remaining two, `SDA` and `SCL` represent the
//! SDA and SCL pads respectively. A [`Pad`] is just a [`Pin`] configured in the
//! correct [`PinMode`] that implements [`IsPad`]. The
//! [`bsp_pins!`](crate::bsp_pins) macro can be used to define convenient type
//! aliases for [`Pad`] types.
//!
//! ```
//! use atsamd_hal::gpio::v2::{PA08, PA09, AlternateC};
//! use atsamd_hal::sercom::v2::{Sercom0, i2c};
//! use atsamd_hal::typelevel::NoneT;
//!
//! // SAMx5x-specific imports
//! use atsamd_hal::sercom::v2::pad::IoSet1;
//!
//! type Sda = Pin<PA08, AlternateC>;
//! type Scl = Pin<PA09, AlternateC>;
//!
//! // SAMD11/SAMD21 version
//! type Pads = i2c::Pads<Sercom0, Sda, Scl>;
//! // SAMx5x version
//! type Pads = i2c::Pads<Sercom0, IoSet1, Sda, Scl>;
//! ```
#![cfg_attr(
    not(feature = "samd11"),
    doc = "
Alternatively, you can use the `PadsFromIds` alias to define a set of
`Pads` in terms of [`PinId`]s instead of [`Pin`]s. This is useful when you
don't have [`Pin`] aliases pre-defined.

```
use atsamd_hal::gpio::v2::{PA08, PA09};
use atsamd_hal::sercom::v2::{Sercom0, i2c};

type Pads = i2c::PadsFromIds<Sercom0, PA08, PA09>;
```

"
)]
//!
//! Instances of [`Pads`] are created using the [`new`](Pads::new) method. Both
//! [`v1::Pin`] and [`v2::Pin`] types are accepted here.
//!
//! On SAMD21 and SAMx5x chips, [`new`](Pads::new) method automatically convert
//! each pin to the correct [`PinMode`]. But for SAMD11 chips, users must
//! manually convert each pin before calling the builder methods. This is a
//! consequence of inherent ambiguities in the SAMD11 SERCOM pad definitions.
//! Specifically, the same [`PinId`] can correspond to two different [`PadNum`]s
//! for the *same* `Sercom`.
//!
//! ```
//! use atsamd_hal::pac::Peripherals;
//! use atsamd_hal::gpio::v2::Pins;
//! use atsamd_hal::sercom::v2::{Sercom0, i2c};
//!
//! let mut peripherals = Peripherals::take().unwrap();
//! let pins = Pins::new(peripherals.PORT);
//! let pads = i2c::Pads::<Sercom0>::new(pins.pa08, pins.pa09);
//! ```
//!
//! # [`Config`]
//!
//! Next, create a [`Config`] struct, which represents the I2C peripheral in
//! its disabled state. A [`Config`] is specified with one type parameters, the
//! [`Pads`] type.
//!
//! Upon creation, the [`Config`] takes ownership of both the [`Pads`] struct
//! and the PAC [`Sercom`] struct. It takes a reference to the PM, so that it
//! can enable the APB clock, and it takes a frequency to indicate the GCLK
//! configuration. Users are responsible for correctly configuring the GCLK.
//!
//! ```
//! use atsamd_hal::gpio::v2::{PA08, PA09};
//! use atsamd_hal::sercom::v2::{Sercom0, i2c};
//!
//! type Pads = i2c::PadsFromIds<Sercom0, PA08, PA09>;
//! type Config = i2c::Config<Pads>;
//!
//! let pm = peripherals.PM;
//! let sercom = peripherals.SERCOM0;
//! // Configure GCLK for 10 MHz
//! let freq = 10.mhz();
//! let config = i2c::Config::new(&pm, sercom, pads, freq);
//! ```
//!
//! The [`Config`] struct can configure the peripheral in one of two ways:
//!
//! * A set of methods is provided to use in a builder pattern: for example
//!   [`baud`](Config::baud), [`run_in_standby`](Config::run_in_standby), etc.
//!   These methods take `self` and return `Self`.
//! * A set of methods is provided to use as setters: for example
//!   [`set_baud`](Config::set_baud),
//!   [`set_run_in_standby`](Config::set_run_in_standby), etc. These methods
//!   take `&mut self` and return nothing.
//!
//! In any case, the peripheral setup ends with a call to [`enable`], which
//! consumes the [`Config`] and returns an enabled [`I2c`] peripheral.
//!
//! ```
//! let i2c = i2c::Config::new(&pm, sercom, pads, freq)
//!     .baud(1.mhz())
//!     .enable();
//! ```
//!
//! Alternatively,
//!
//! ```
//! let i2c = i2c::Config::new(&mclk, sercom, pads, freq);
//!     i2c.set_baud(1.mhz());
//!     let i2c = i2c.enable();
//! ```
//!
//! ## Reading the current configuration
//!
//! It is possible to read the current configuration by using the getter methods
//! provided: for example [`get_baud`](Config::get_baud),
//! [`get_run_in_standby`](Config::get_run_in_standby), etc.
//!
//! # [`I2c`]
//!
//! [`I2c`] structs can only be created from a [`Config`]. They have one type
//! parameter, representing the underlying [`Config`].
//!
//! Only the [`I2c`] struct can actually perform
//! transactions. To do so, use the embedded HAL traits, like
//! [`i2c::WriteRead`], [`i2c::Read`] and [`i2c::Write`].
//!
//! ```
//! use embedded_hal::blocking::i2c::Write;
//!
//! i2c.write(0x54, 0x0fe)
//! ```
//!
//! # Reading the current configuration
//!
//! The `AsRef<Config<P>>` trait is implemented for `I2c<Config<P>>`.
//! This means you can use the `get_` methods implemented for `Config`, since
//! they take an `&self` argument.
//!
//! ```
//! // Assume i2c is a I2c<C<P>>
//! let baud = i2c.as_ref().get_baud();
//! ```
//!
//! # Reconfiguring
//!
//! The [`reconfigure`] method gives out an `&mut Config` reference, which can
//! then use the `set_*` methods.
//!
//! ```
//! use atsamd_hal::sercom::v2::i2c::I2c;
//! use atsamd_hal::time::*;
//!
//! // Assume config is a valid Duplex I2C Config struct
//! let i2c = config.enable();
//!
//! // Send/receive data...
//!
//! // Reconfigure I2C peripheral
//! i2c.reconfigure(|c| c.set_run_in_standby(false));
//!
//! // Disable I2C peripheral
//! let config = i2c.disable();
//! ```
//!
//! # Non-supported features
//!
//! * Slave mode is not supported at this time.
//! * High-speed mode is not supported.
//! * 4-wire mode is not supported.
//! * 32-bit extension mode is not supported (SAMx5x). If you need to transfer
//!   slices, consider using the DMA methods instead. The `dma` Cargo feature
//!   must be enabled.
//!
//! [`enable`]: Config::enable
//! [`disable`]: I2c::disable
//! [`reconfigure`]: I2c::reconfigure
//! [`bsp_pins`]: crate::bsp_pins
//! [`Sercom`]: crate::sercom::v2::Sercom
//! [`Pad0`]: crate::sercom::v2::pad::Pad0
//! [`Pad1`]: crate::sercom::v2::pad::Pad1
//! [`Pad`]: crate::sercom::v2::pad::Pad
//! [`IsPad`]: crate::sercom::v2::pad::IsPad
//! [`PadNum`]: crate::sercom::v2::pad::PadNum
//! [`v1::Pin`]: crate::gpio::v1::Pin
//! [`v2::Pin`]: crate::gpio::v2::pin::Pin
//! [`Pin`]: crate::gpio::v2::pin::Pin
//! [`PinId`]: crate::gpio::v2::pin::PinId
//! [`PinMode`]: crate::gpio::v2::pin::PinMode
//! [`i2c::Write`]: embedded_hal::blocking::i2c::Write
//! [`i2c::Read`]: embedded_hal::blocking::i2c::Read
//! [`i2c::WriteRead`]: embedded_hal::blocking::i2c::WriteRead
#![cfg_attr(
    feature = "dma",
    doc = "
# Using I2C with DMA

This HAL includes support for DMA-enabled I2C transfers. [`I2c`]
implements the DMAC [`Buffer`]
trait. The provided [`send_with_dma`] and
[`receive_with_dma`] build and begin a
[`dmac::Transfer`], thus starting the I2C
in a non-blocking way. 

Note that the [`init_dma_transfer`] method should be called immediately before
starting a DMA transfer with I2C. This will check that the bus is in a correct
state before starting the transfer, and providing a token type to pass to the 
[`send_with_dma`] and [`receive_with_dma`] methods.

Optionally, interrupts can be enabled on the provided
[`Channel`]. Note that the `dma` feature must
be enabled. Please refer to the [`dmac`](crate::dmac) module-level
documentation for more information.

```
// Assume channel0 and channel1 are configured `dmac::Channel`s,
// i2c is an I2c<C>.

// Create data to send. Note that it must be `'static`.
let buffer: [u8; 50] = [0xff; 50];

// Initialize the bus and check for errors
let token = i2c.init_dma_transfer()?;
let transfer = i2c.send_with_dma(0x54, token, buffer, channel0, |_| {})

// Wait for transfers to complete and reclaim resources
let (chan0, buffer, i2c) = transfer.wait();
```

[`Buffer`]: crate::dmac::transfer::Buffer
[`init_dma_transfer`]: I2c::init_dma_transfer
[`send_with_dma`]: I2c::send_with_dma
[`receive_with_dma`]: I2c::receive_with_dma
[`dmac::Transfer`]: crate::dmac::Transfer
[`Channel`]: crate::dmac::channel::Channel
[`dmac`]: crate::dmac

"
)]

#[cfg(any(feature = "samd11", feature = "samd21"))]
#[path = "i2c/pads_thumbv6m.rs"]
mod pads;

#[cfg(feature = "min-samd51g")]
#[path = "i2c/pads_thumbv7em.rs"]
mod pads;

pub use pads::*;

mod reg;
use reg::Registers;

mod flags;
pub use flags::*;

mod config;
pub use config::*;

mod impl_ehal;

/// Word size for an I2C message
pub type Word = u8;

/// Inactive timeout configuration
#[repr(u8)]
#[derive(Clone, Copy)]
pub enum InactiveTimeout {
    /// Disabled
    Disabled = 0x0,
    /// 5-6 SCL cycles (50-60 us @ 100 kHz)
    Us55 = 0x1,
    ///10-11 SCL cycles (100-110 us @ 100 kHz)
    Us105 = 0x2,
    /// 20-21 SCL cycles (200-210 us @ 100 kHz)
    Us205 = 0x3,
}

/// Abstraction over a I2C peripheral, allowing to perform I2C transactions.
pub struct I2c<C: AnyConfig> {
    config: C,
}

impl<C: AnyConfig> I2c<C> {
    /// Obtain a pointer to the `DATA` register. Necessary for DMA transfers.
    #[inline]
    pub fn data_ptr(&self) -> *mut Word {
        self.config.as_ref().registers.data_ptr()
    }

    /// Read the interrupt flags
    #[inline]
    pub fn read_flags(&self) -> Flags {
        self.config.as_ref().registers.read_flags()
    }

    /// Clear interrupt status flags
    #[inline]
    pub fn clear_flags(&mut self, flags: Flags) {
        self.config.as_mut().registers.clear_flags(flags);
    }

    /// Enable interrupts for the specified flags.
    #[inline]
    pub fn enable_interrupts(&mut self, flags: Flags) {
        self.config.as_mut().registers.enable_interrupts(flags);
    }

    /// Disable interrupts for the specified flags.
    #[inline]
    pub fn disable_interrupts(&mut self, flags: Flags) {
        self.config.as_mut().registers.disable_interrupts(flags);
    }

    /// Read the status flags
    #[inline]
    pub fn read_status(&self) -> Status {
        self.config.as_ref().registers.read_status()
    }

    /// Clear the status flags
    #[inline]
    pub fn clear_status(&mut self, status: Status) {
        self.config.as_mut().registers.clear_status(status);
    }

    #[cfg(feature = "dma")]
    #[inline]
    pub(super) fn start_dma_write(&mut self, address: u8, xfer_len: u8) {
        self.config
            .as_mut()
            .registers
            .start_dma_write(address, xfer_len)
    }

    #[cfg(feature = "dma")]
    #[inline]
    pub(super) fn start_dma_read(&mut self, address: u8, xfer_len: u8) {
        self.config
            .as_mut()
            .registers
            .start_dma_read(address, xfer_len)
    }

    #[inline]
    pub(super) fn check_bus_status(&self) -> Result<(), Error> {
        self.config.as_ref().registers.check_bus_status()
    }

    #[inline]
    fn do_write(&mut self, addr: u8, bytes: &[u8]) -> Result<(), Error> {
        self.config.as_mut().registers.do_write(addr, bytes)
    }

    #[inline]
    fn do_read(&mut self, addr: u8, bytes: &mut [u8]) -> Result<(), Error> {
        self.config.as_mut().registers.do_read(addr, bytes)
    }

    #[inline]
    fn do_write_read(&mut self, addr: u8, bytes: &[u8], buffer: &mut [u8]) -> Result<(), Error> {
        self.config
            .as_mut()
            .registers
            .do_write_read(addr, bytes, buffer)
    }
    #[inline]
    fn cmd_stop(&mut self) {
        self.config.as_mut().registers.cmd_stop()
    }

    /// Reconfigure the I2C peripheral.
    ///
    /// Calling this method will temporarily disable the SERCOM peripheral, as
    /// some registers are enable-protected. This may interrupt any ongoing
    /// transactions.
    ///
    /// ```
    /// use atsamd_hal::sercom::v2::i2c::I2c;
    /// i2c.reconfigure(|c| c.set_run_in_standby(false));
    /// ```
    #[inline]
    pub fn reconfigure<F>(&mut self, update: F)
    where
        F: FnOnce(&mut SpecificConfig<C>),
    {
        self.config.as_mut().registers.enable_peripheral(false);
        update(&mut self.config.as_mut());
        self.config.as_mut().registers.enable_peripheral(true);
    }

    /// Disable the I2C peripheral and return the underlying [`Config`]
    #[inline]
    pub fn disable(self) -> C {
        let mut config = self.config;
        config.as_mut().registers.disable();
        config
    }
}

impl<P: PadSet> AsRef<Config<P>> for I2c<Config<P>> {
    #[inline]
    fn as_ref(&self) -> &Config<P> {
        self.config.as_ref()
    }
}
