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
//! ```no_run
//! use atsamd_hal::gpio::{PA08, PA09, AlternateC};
//! use atsamd_hal::sercom::{Sercom0, i2c};
//! use atsamd_hal::typelevel::NoneT;
//!
//! // SAMx5x-specific imports
//! use atsamd_hal::sercom::pad::IoSet1;
//!
//! type Sda = Pin<PA08, AlternateC>;
//! type Scl = Pin<PA09, AlternateC>;
//!
//! // SAMD11/SAMD21 version
//! type Pads = i2c::Pads<Sercom0, Sda, Scl>;
//! // SAMx5x version
//! type Pads = i2c::Pads<Sercom0, IoSet1, Sda, Scl>;
//! ```
//!
//! Alternatively, you can use the [`PadsFromIds`] alias to define a set of
//! `Pads` in terms of [`PinId`]s instead of [`Pin`]s. This is useful when you
//! don't have [`Pin`] aliases pre-defined.
//!
//! ```no_run
//! use atsamd_hal::gpio::{PA08, PA09};
//! use atsamd_hal::sercom::{Sercom0, i2c};
//!
//! type Pads = i2c::PadsFromIds<Sercom0, PA08, PA09>;
//! ```
//!
//! Instances of [`Pads`] are created using the [`new`](Pads::new) method.
//!
//! On SAMD21 and SAMx5x chips, [`new`](Pads::new) method automatically convert
//! each pin to the correct [`PinMode`]. But for SAMD11 chips, users must
//! manually convert each pin before calling the builder methods. This is a
//! consequence of inherent ambiguities in the SAMD11 SERCOM pad definitions.
//! Specifically, the same [`PinId`] can correspond to two different [`PadNum`]s
//! for the *same* `Sercom`.
//!
//! ```no_run
//! use atsamd_hal::pac::Peripherals;
//! use atsamd_hal::gpio::Pins;
//! use atsamd_hal::sercom::{Sercom0, i2c};
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
//! ```no_run
//! use atsamd_hal::gpio::{PA08, PA09};
//! use atsamd_hal::sercom::{Sercom0, i2c};
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
//! ```no_run
//! let i2c = i2c::Config::new(&pm, sercom, pads, freq)
//!     .baud(1.mhz())
//!     .enable();
//! ```
//!
//! Alternatively,
//!
//! ```no_run
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
//! transactions. To do so, use the [`embedded_hal::i2c::I2c`] trait.
//!
//! ```
//! use embedded_hal::i2c::I2c;
//!
//! i2c.write(0x54, 0x0fe).unwrap();
//! ```
//!
//! # Reading the current configuration
//!
//! The `AsRef<Config<P>>` trait is implemented for `I2c<Config<P>>`.
//! This means you can use the `get_` methods implemented for `Config`, since
//! they take an `&self` argument.
//!
//! ```no_run
//! // Assume i2c is a I2c<C<P>>
//! let baud = i2c.as_ref().get_baud();
//! ```
//!
//! # Reconfiguring
//!
//! The [`reconfigure`] method gives out an `&mut Config` reference, which can
//! then use the `set_*` methods.
//!
//! ```no_run
//! use atsamd_hal::sercom::i2c::I2c;
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
//!   slices, consider using the DMA methods instead <span class="stab
//!   portability" title="Available on crate feature `dma`
//!   only"><code>dma</code></span>.
//!
//! # Using I2C with DMA <span class="stab portability" title="Available on crate feature `dma` only"><code>dma</code></span>
//!
//! This HAL includes support for DMA-enabled I2C transfers. Use
//! [`I2c::with_dma_channel`] to attach a DMA channel to the [`I2c`] struct. A
//! DMA-enabled [`I2c`] implements the blocking
//! [`embedded_hal::i2c::I2c`](crate::ehal::i2c::I2c) trait, which can be used
//! to perform I2C transfers which are fast, continuous and low jitter, even
//! if they are preemped by a higher priority interrupt.
//!
//!
//! ```no_run
//! use atsamd_hal::dmac::channel::{AnyChannel, Ready};
//! use atsand_hal::sercom::i2c::{I2c, AnyConfig, Error};
//! use atsamd_hal::embedded_hal::i2c::I2c;
//! fn i2c_write_with_dma<A: AnyConfig, C: AnyChannel<Status = Ready>>(i2c: I2c<A>, channel: C, bytes: &[u8]) -> Result<(), Error>{
//!     // Attach a DMA channel
//!     let i2c = i2c.with_dma_channel(channel);
//!     i2c.write(0x54, bytes)?;
//! }
//! ```
//!
//! ## Limitations of using DMA with I2C
//!
//! * The I2C peripheral only supports continuous DMA read/writes of up to 255
//!   bytes. Trying to read/write with a transfer of 256 bytes or more will
//!   result in a panic. This also applies to using [`I2c::transaction`] with
//!   adjacent write/read operations of the same type; the total number of bytes
//!   across all adjacent operations must not exceed 256. If you need continuous
//!   transfers of 256 bytes or more, use the non-DMA [`I2c`] implementations.
//!
//! * When using [`I2c::transaction`] or [`I2c::write_read`], the
//!   [`embedded_hal::i2c::I2c`] specification mandates that a REPEATED START
//!   (instead of a STOP+START) is sent between transactions of a different type
//!   (read/write). Unfortunately, in DMA mode, the hardware is only capable of
//!   sending STOP+START. If you absolutely need repeated starts, the only
//!   workaround is to use the I2C without DMA.
//!
//! * Using [`I2c::transaction`] consumes significantly more memory than the
//!   other methods provided by [`embedded_hal::i2c::I2c`] (at least 256 bytes
//!   extra).
//!
//! * When using [`I2c::transaction`], up to 17 adjacent operations of the same
//!   type can be continuously handled by DMA without CPU intervention. If you
//!   need more than 17 adjacent operations of the same type, the transfer will
//!   reverted to using the byte-by-byte (non-DMA) implementation.
//!
//! All these limitations also apply to I2C transfers in async mode when using
//! DMA. They do not apply to I2C transfers in async mode when not using DMA.
//!
//! # `async` operation <span class="stab portability" title="Available on crate feature `async` only"><code>async</code></span>
//!
//! An [`I2c`] can be used for
//! `async` operations. Configuring an [`I2c`] in async mode is relatively
//! simple:
//!
//! * Bind the corresponding `SERCOM` interrupt source to the SPI
//!   [`InterruptHandler`] (refer to the module-level [`async_hal`]
//!   documentation for more information).
//! * Turn a previously configured [`I2c`] into an [`I2cFuture`] by calling
//!   [`I2c::into_future`]
//! * Optionally, add a DMA channel by using [`I2cFuture::with_dma_channel`].
//!   The API is exactly the same whether a DMA channel is used or not.
//! * Use the provided async methods for reading or writing to the I2C
//!   peripheral. [`I2cFuture`] implements [`embedded_hal_async::i2c::I2c`].
//!
//! `I2cFuture` implements `AsRef<I2c>` and `AsMut<I2c>` so
//! that it can be reconfigured using the regular [`I2c`] methods.
//!
//! ## Considerations when using `async` [`I2c`] with DMA <span class="stab portability" title="Available on crate feature `async` only"><code>async</code></span> <span class="stab portability" title="Available on crate feature `dma` only"><code>dma</code></span>
//!
//! * An [`I2c`] struct must be turned into an [`I2cFuture`] by calling
//!   [`I2c::into_future`] before calling `with_dma_channel`. The DMA channel
//!   itself must also be configured in async mode by using
//!   [`DmaController::into_future`](crate::dmac::DmaController::into_future).
//!   If a DMA channel is added to the [`I2c`] struct before it is turned into
//!   an [`I2cFuture`], it will not be able to use DMA in async mode.
//!
//! ```
//! // This will work
//! let i2c = i2c.into_future().with_dma_channel(channel);
//!
//! // This won't
//! let i2c = i2c.with_dma_channel(channel).into_future();
//! ```
//!
//! ### Safety considerations
//!
//! In `async` mode, an I2C+DMA transfer does not require `'static` source and
//! destination buffers. This, in theory, makes its use `unsafe`. However it is
//! marked as safe for better ergonomics, and to enable the implementation of
//! the [`embedded_hal_async::i2c::I2c`] trait.
//!
//! This means that, as an user, you **must** ensure that the [`Future`]s
//! returned by the [`embedded_hal_async::i2c::I2c`] methods may never be
//! forgotten through [`forget`] or by wrapping them with a [`ManuallyDrop`].
//!
//! The returned futures implement [`Drop`] and will automatically stop any
//! ongoing transfers; this guarantees that the memory occupied by the
//! now-dropped buffers may not be corrupted by running transfers.
//!
//! This means that using functions like [`futures::select_biased`] to implement
//! timeouts is safe; transfers will be safely cancelled if the timeout expires.
//!
//! This also means that should you [`forget`] this [`Future`] after its
//! first [`poll`] call, the transfer will keep running, ruining the
//! now-reclaimed memory, as well as the rest of your day.
//!
//! * `await`ing is fine: the [`Future`] will run to completion.
//! * Dropping an incomplete transfer is also fine. Dropping can happen, for
//!   example, if the transfer doesn't complete before a timeout expires.
//! * Dropping an incomplete transfer *without running its destructor* is
//!   **unsound** and will trigger undefined behavior.
//!
//! ```ignore
//! async fn always_ready() {}
//!
//! let mut buffer = [0x00; 10];
//!
//! // This is completely safe
//! i2c.read(&mut buffer).await?;
//!
//! // This is also safe: we launch a transfer, which is then immediately cancelled
//! futures::select_biased! {
//!     _ = i2c.read(&mut buffer)?,
//!     _ = always_ready(),
//! }
//!
//! // This, while contrived, is also safe.
//! {
//!     use core::future::Future;
//!
//!     let future = i2c.read(&mut buffer);
//!     futures::pin_mut!(future);
//!     // Assume ctx is a `core::task::Context` given out by the executor.
//!     // The future is polled, therefore starting the transfer
//!     future.as_mut().poll(ctx);
//!
//!     // Future is dropped here - transfer is cancelled.
//! }
//!
//! // DANGER: This is an example of undefined behavior
//! {
//!     use core::future::Future;
//!     use core::ops::DerefMut;
//!
//!     let future = core::mem::ManuallyDrop::new(i2c.read(&mut buffer));
//!     futures::pin_mut!(future);
//!     // To actually make this example compile, we would need to wrap the returned
//!     // future from `i2c.read()` in a newtype that implements Future, because we
//!     // can't actually call as_mut() without being able to name the type we want
//!     // to deref to.
//!     let future_ref: &mut SomeNewTypeFuture = &mut future.as_mut();
//!     future.as_mut().poll(ctx);
//!
//!     // Future is NOT dropped here - transfer is not cancelled, resulting un UB.
//! }
//! ```
//!
//! As you can see, unsoundness is relatively hard to come by - however, caution
//! should still be exercised.
//!
//! [`enable`]: Config::enable
//! [`disable`]: I2c::disable
//! [`reconfigure`]: I2c::reconfigure
//! [`bsp_pins`]: crate::bsp_pins
//! [`Sercom`]: crate::sercom::Sercom
//! [`Pad0`]: crate::sercom::pad::Pad0
//! [`Pad1`]: crate::sercom::pad::Pad1
//! [`Pad`]: crate::sercom::pad::Pad
//! [`IsPad`]: crate::sercom::pad::IsPad
//! [`PadNum`]: crate::sercom::pad::PadNum
//! [`Pin`]: crate::gpio::pin::Pin
//! [`PinId`]: crate::gpio::pin::PinId
//! [`PinMode`]: crate::gpio::pin::PinMode
//! [`embedded_hal::i2c::I2c`]: crate::ehal::i2c::I2c
//! [`I2c::transaction`]: crate::ehal::i2c::I2c::transaction
//! [`I2c::write_read`]: crate::ehal::i2c::I2c::write_read
//! [`async_hal`]: crate::async_hal
//! [`forget`]: core::mem::forget
//! [`ManuallyDrop`]: core::mem::ManuallyDrop
//! [`Future`]: core::future::Future
//! [`poll`]: core::future::Future::poll

use atsamd_hal_macros::hal_module;

#[hal_module(
    any("sercom0-d11", "sercom0-d21") => "i2c/pads_thumbv6m.rs",
    "sercom0-d5x" => "i2c/pads_thumbv7em.rs",
)]
mod pads {}

pub use pads::*;

mod reg;
use reg::Registers;

mod flags;
pub use flags::*;

mod config;
pub use config::*;

mod impl_ehal;

#[cfg(feature = "async")]
mod async_api;

#[cfg(feature = "async")]
pub use async_api::*;

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
pub struct I2c<C: AnyConfig, D = crate::typelevel::NoneT> {
    config: C,
    _dma_channel: D,
}

impl<C: AnyConfig, D> I2c<C, D> {
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

    #[cfg(feature = "dma")]
    #[inline]
    pub(super) fn check_bus_status(&self) -> Result<(), Error> {
        self.config.as_ref().registers.check_bus_status()
    }

    #[inline]
    fn do_write(&mut self, addr: u8, bytes: &[u8]) -> Result<(), Error> {
        self.config.as_mut().registers.do_write(addr, bytes)
    }

    /// Continue a write operation that was issued before with
    /// [`do_write`](Self::do_write) or [`continue_write`](Self::continue_write)
    /// without a repeated start condition in between
    #[inline]
    fn continue_write(&mut self, bytes: &[u8]) -> Result<(), Error> {
        self.config.as_mut().registers.continue_write(bytes)
    }

    #[inline]
    fn do_read(&mut self, addr: u8, bytes: &mut [u8]) -> Result<(), Error> {
        self.config.as_mut().registers.do_read(addr, bytes)
    }

    /// Continue a read operation that was issued before with
    /// [`do_read`](Self::do_read) or [`continue_read`](Self::continue_read)
    /// without a repeated start condition in between
    #[inline]
    fn continue_read(&mut self, bytes: &mut [u8]) -> Result<(), Error> {
        self.config.as_mut().registers.continue_read(bytes)
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
    /// use atsamd_hal::sercom::i2c::I2c;
    /// i2c.reconfigure(|c| c.set_run_in_standby(false));
    /// ```
    #[inline]
    pub fn reconfigure<F>(&mut self, update: F)
    where
        F: FnOnce(&mut SpecificConfig<C>),
    {
        self.config.as_mut().registers.enable_peripheral(false);
        update(self.config.as_mut());
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

impl<C: AnyConfig> I2c<C> {
    /// Attach a DMA channel to this [`I2c`]. Its
    /// [`embedded_hal::i2c::I2c`](crate::ehal::i2c::I2c) implementation will
    /// use DMA to carry out its transactions.
    #[cfg(feature = "dma")]
    #[inline]
    pub fn with_dma_channel<Chan: crate::dmac::AnyChannel<Status = crate::dmac::Ready>>(
        self,
        channel: Chan,
    ) -> I2c<C, Chan> {
        I2c {
            config: self.config,
            _dma_channel: channel,
        }
    }
}

#[cfg(feature = "dma")]
impl<C, D, S> I2c<C, D>
where
    C: AnyConfig,
    D: crate::dmac::AnyChannel<Status = S>,
    S: crate::dmac::ReadyChannel,
{
    /// Reclaim the DMA channel. Any subsequent I2C operations will no longer
    /// use DMA.
    pub fn take_dma_channel(self) -> (I2c<C, crate::typelevel::NoneT>, D) {
        (
            I2c {
                config: self.config,
                _dma_channel: crate::typelevel::NoneT,
            },
            self._dma_channel,
        )
    }
}

impl<P: PadSet> AsRef<Config<P>> for I2c<Config<P>> {
    #[inline]
    fn as_ref(&self) -> &Config<P> {
        self.config.as_ref()
    }
}
