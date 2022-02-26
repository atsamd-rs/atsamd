//! A [`Future`]-like interface for SPI transactions
//!
//! An [`SpiFuture`] takes ownership of a `[u8]`-like buffer and a user-defined
//! resource that can act as an [`Spi`] `struct`. It then executes a full-duplex
//! SPI transaction using iterrupts. On each `RXC` or `DRE` interrupt, the
//! `SpiFuture` reads or sends `STEP` bytes of the buffer, where `STEP` is a
//! value that depends on
#![cfg_attr(any(feature = "samd11", feature = "samd21"), doc = "[`CharSize`]")]
#![cfg_attr(feature = "min-samd51g", doc = "`CharSize`")]
//! , for SAMD11 & SAMD21 chips, or
#![cfg_attr(any(feature = "samd11", feature = "samd21"), doc = "`Length`")]
#![cfg_attr(feature = "min-samd51g", doc = "[`Length`]")]
//! , for SAMD51 & SAME5x chips.
//!
//! The provided buffer must implement [`AsRef`] and [`AsMut`] for `[u8]`, it
//! must have an appropriate length (see below), and it must have a `'static`
//! lifetime, i.e. it must be owned or a `&'static mut` reference.
//!
//! The user-defined resource must implement the [`AsSpi`] trait, which provides
//! access to an underlying [`Spi`] struct. Optionally, the resource can also
//! provide access to a GPIO [`Pin`] acting as the SPI `SS` line. If the
//! resource implements the `AsSpi` [`assert_ss`] and [`deassert_ss`] methods,
//! the corresponding `Pin` will be set low at the beginnging of the transfer
//! and brought high at completion. All bare `Spi` structs implement `AsSpi`, so
//! they can be used directly.
//!
//! Finally, an `SpiFuture` can also accept a function or closure that will be
//! called on completion of the transaction, acting like a [`Waker`].
//!
//! Create an `SpiFuture` like so
//!
//! ```
//! use atsamd_hal::sercom::v2::spi::AnySpi;
//! use atsamd_hal::sercom::v2::spi_future::SpiFuture;
//!
//! fn wake_up() {
//!     //...
//! }
//!
//! fn create_future(spi: impl AnySpi) {
//!     let buf = [0_u8; 12];
//!     let future = SpiFuture::new(spi, buf).with_waker(wake_up);
//! }
//! ```
//!
//! Like real [`Future`]s, `SpiFuture`s are lazy. Nothing happens until calling
//! [`start`]. Doing so will enable the `DRE` and `RXC` interrupts and begin the
//! transaction.
//!
//! ```
//! use atsamd_hal::sercom::spi::AnySpi;
//! use atsamd_hal::sercom::spi_future::SpiFuture;
//!
//! fn wake_up() {
//!     //...
//! }
//!
//! fn use_future(spi: impl AnySpi) {
//!     let buf = [0_u8; 12];
//!     let future = SpiFuture::new(spi, buf).with_waker(wake_up);
//!     future.start();
//! }
//! ```
//!
//! When the transaction is complete, the `SpiFuture` will automatically
//! disable the `DRE` and `RXC` interrupts. To test whether an `SpiFuture` is
//! complete, use the [`poll`] method. While the transaction is in progress, it
//! will return [`Poll::Pending`]. When the transaction is complete, it will
//! return [`Poll::Ready`]. Once complete, you can consume the `SpiFuture` and
//! [`free`] the constituent pieces. Doing so before the transfer has completed
//! is `unsafe`.
//!
//! The actual transfer is performed by the [`send`] and [`recv`] methods, which
//! should be called from the `DRE` and `RXC` interrupt handlers, respectively.
//! See the [RTIC example](self#rtic-example) below for a complete example.
//!
//! ## `STEP` size and buffer length
//!
//! For SAMD11 & SAMD21 chips, `STEP` is equal to the number of bytes in the
//! corresponding
#![cfg_attr(
    any(feature = "samd11", feature = "samd21"),
    doc = "[`CharSize::Word`] type, i.e. 1 for [`EightBit`] and 2 for [`NineBit`]."
)]
#![cfg_attr(
    feature = "min-samd51g",
    doc = "`CharSize::Word` type, i.e. 1 for `EightBit` and 2 for `NineBit`."
)]
//! For SAMD51 & SAME5x chips, `STEP` is equal to the `Length` or 4, whichever
//! is less.
//!
//! The provided buffer must have an appropriate length. For SAMD11 & SAMD21
//! chips, as well as SAMDx5x chips with `Length <= 4`, a single write of
//! `STEP` bytes represents an entire SPI transaction, i.e. they have an
//! [`AtomicSize`]. In these cases, the provided buffer must represent an
//! integer number of transactions. For example, a SAMD51 `Spi` struct with a
//! `Length` of 3 could use buffers of length 3, 6, 9, etc. For longer `Length`
//! values, the provided buffer must represent exactly one SPI transaction, so
//! the buffer length must be equal to `Length`. For example, a SAMD51 `Spi`
//! struct with a `Length` of 7 could only use a buffer with exactly 7 bytes.
//!
//! Keep in mind that `SpiFuture` works only with `[u8]`-like things, which
//! can introduce some limitations.
//!
//! Suppose you plan to execute `NineBit` transactions with a SAMD21 chip.
//! Your data may come in the form of a `[u16]` slice. However, to use it with
//! `SpiFuture`, you will need reformulate it as a `[u8]` slice, perhaps using a
//! crate like [`bytemuck`] to safely transmute the data.
//!
//! In another scenario, suppose you wanted to use a SAMx5x chip with a
//! transaction `Length` of 3 bytes. Your data might come in the form of a
//! `[u32]` slice. In this situation, it would **not** be appropriate to
//! transmute the data to a `[u8]` slice. `SpiFuture` expects the data to be a
//! *byte-packed* `[u8]` slice, so the extra byte in each `u32` makes it
//! incompatible.
//!
//! ## RTIC example
//!
//! The [RTIC] framework provides a convenient way to store a `static`ally
//! allocated `SpiFuture`, so that it can be accessed by both the interrupt
//! handlers and user code. The following example shows how an `SpiFuture` might
//! be used for a series of transactions.
//!
//! ```
//! use core::task::Poll;
//! use atsamd_hal::gpio::{PA08, PA09, PA10, PA11, Pin, PushPullOutput};
//! use atsamd_hal::sercom::Sercom0;
//! use atsamd_hal::sercom::pad::IoSet1;
//! use atsamd_hal::sercom::spi::{self, Master, lengths::U12};
//! use atsamd_hal::sercom::spi_future::{AsSpi, SpiFuture};
//!
//! type Pads = spi::PadsFromIds<Sercom0, IoSet1, PA08, PA11, PA09>;
//! type Spi = spi::Spi<spi::Config<Pads, Master, U12>, spi::Duplex>;
//! type SS = Pin<PA10, PushPullOutput>;
//!
//! type Resource = (Spi, SS);
//!
//! impl AsSpi for Resource {
//!     type Spi = Spi;
//!     fn spi(&self) -> &Spi {
//!         &self.0
//!     }
//!     fn spi_mut(&mut self) -> &mut Spi {
//!         &mut self.0
//!     }
//!     fn assert_ss(&mut self) {
//!         let _ = self.1.set_low();
//!     }
//!     fn deassert_ss(&mut self) {
//!         let _ = self.1.set_high();
//!     }
//! }
//!
//! type Future = SpiFuture<Resource, [u8; 12]>;
//!
//! //...
//!
//! #[local]
//! struct Local {
//!     opt_resource: Option<Resource>,
//! }
//!
//! #[shared]
//! struct Shared {
//!     #[lock_free]
//!     opt_future: Option<Future>,
//! }
//!
//! #[task(local = [opt_resource], shared = [opt_future])]
//! fn task(ctx: task::Context) {
//!     let opt_resource = ctx.local.opt_resource;
//!     let opt_future = ctx.shared.opt_future;
//!     match opt_future {
//!         Some(future) => {
//!             if let Poll::Ready(_) = future.poll() {
//!                 let (resource, buf) = opt_future.take().unwrap().free();
//!                 *opt_resource = Some(resource);
//!                 consume_data(buf);
//!             }
//!         }
//!         None => {
//!             let resource = opt_resource.take().unwrap();
//!             let buf: [u8; 12] = produce_data();
//!             let future = opt_future.insert(
//!                 SpiFuture::new(resource, buf).with_waker(|| { task::spawn().ok(); })
//!             );
//!             future.start();
//!         }
//!     }
//! }
//!
//! #[task(binds = SERCOM0_0, shared = [opt_future])]
//! fn dre(ctx: dre::Context) {
//!     ctx.shared.opt_future.as_mut().unwrap().send();
//! }
//!
//! #[task(binds = SERCOM0_2, shared = [opt_future])]
//! fn rxc(ctx: rxc::Context) {
//!     ctx.shared.opt_future.as_mut().unwrap().recv();
//! }
//!
//! //...
//! ```
//!
//! [`assert_ss`]: AsSpi::assert_ss
//! [`deassert_ss`]: AsSpi::assert_ss
//! [`start`]: SpiFuture::start
//! [`poll`]: SpiFuture::poll
//! [`free`]: SpiFuture::free
//! [`send`]: SpiFuture::send
//! [`recv`]: SpiFuture::recv
//! [`Spi`]: super::spi::Spi
//! [`CharSize`]: super::spi::CharSize
//! [`CharSize::Word`]: super::spi::CharSize::Word
//! [`EightBit`]: super::spi::EightBit
//! [`NineBit`]: super::spi::NineBit
//! [`Length`]: super::spi::Length
//! [`AtomicSize`]: super::spi::AtomicSize
//! [`Pin`]: crate::gpio::pin::Pin
//! [`Future`]: core::future::Future
//! [`Waker`]: core::task::Waker
//! [`Poll`]: core::task::Poll
//! [RTIC]: https://rtic.rs/
//! [`bytemuck`]: https://docs.rs/bytemuck

use core::task::Poll;

use super::spi::{AnySpi, Error, Flags, SpecificSpi};

#[cfg(any(feature = "samd11", feature = "samd21"))]
type Data = u16;

#[cfg(feature = "min-samd51g")]
type Data = u32;

//=============================================================================
// AsSpi
//=============================================================================

/// Allow user types to provide access to an internal [`Spi`] struct and
/// optional GPIO [`Pin`] acting as `SS`
///
/// This trait is used by [`SpiFuture`]s to access the stored `Spi` struct
/// within a user-defined type. It also provides an optional interface for a
/// GPIO `Pin` acting as the `SS` line.
///
/// If the [`assert_ss`] and [`deassert_ss`] methods are implemented, the `SS`
/// line will be set low at the start of a transaction and set high at
/// completion. If no implementations are provided, the two functions default to
/// a NOP.
///
/// [`Spi`]: super::spi::Spi
/// [`Pin`]: crate::gpio::v2::pin::Pin
/// [`assert_ss`]: AsSpi::assert_ss
/// [`deassert_ss`]: AsSpi::assert_ss
pub trait AsSpi {
    /// Stored [`Spi`] struct
    ///
    /// [`Spi`]: super::spi::Spi
    type Spi: AnySpi;

    /// Return a shared reference to `Self::Spi`
    ///
    /// Remember that the [`AnySpi`] trait requires that
    /// `Self::Spi == SpecificSpi<Self::Spi>`, so the implementation of this
    /// function is usually trivial, i.e.
    ///
    /// ```
    /// type Resource = (Spi, SS);
    ///
    /// impl AsSpi for Resource {
    ///     type Spi = Spi;
    ///     fn spi(&self) -> &Spi {
    ///         &self.0
    ///     }
    ///     // ...
    /// }
    /// ```
    fn spi(&self) -> &SpecificSpi<Self::Spi>;

    /// Return an exclusive reference to `Self::Spi`
    ///
    /// Remember that the [`AnySpi`] trait requires that
    /// `Self::Spi == SpecificSpi<Self::Spi>`, so the implementation of this
    /// function is usually trivial, i.e.
    ///
    /// ```
    /// type Resource = (Spi, SS);
    ///
    /// impl AsSpi for Resource {
    ///     type Spi = Spi;
    ///     fn spi_mut(&mut self) -> &mut Spi {
    ///         &mut self.0
    ///     }
    ///     // ...
    /// }
    /// ```
    fn spi_mut(&mut self) -> &mut SpecificSpi<Self::Spi>;

    /// Assert the `SS` line by bringing it low
    ///
    /// If the user-defined resource also controls a GPIO [`Pin`] representing
    /// the `SS` line, this function allows an [`SpiFuture`] to assert it.
    ///
    /// ```
    /// type Resource = (Spi, SS);
    ///
    /// impl AsSpi for Resource {
    ///     // ...
    ///     fn assert_ss(&mut self) {
    ///         let _ = self.1.set_low();
    ///     }
    /// }
    /// ```
    ///
    /// [`Pin`]: crate::gpio::v2::pin::Pin
    #[inline]
    fn assert_ss(&mut self) {}

    /// Deassert the `SS` line by bringing it high
    ///
    /// If the user-defined resource also controls a GPIO [`Pin`] representing
    /// the `SS` line, this function allows an [`SpiFuture`] to deassert it.
    ///
    /// ```
    /// type Resource = (Spi, SS);
    ///
    /// impl AsSpi for Resource {
    ///     // ...
    ///     fn deassert_ss(&mut self) {
    ///         let _ = self.1.set_high();
    ///     }
    /// }
    /// ```
    ///
    /// [`Pin`]: crate::gpio::v2::pin::Pin
    #[inline]
    fn deassert_ss(&mut self) {}
}

impl<S: AnySpi> AsSpi for S {
    type Spi = S;
    #[inline]
    fn spi(&self) -> &SpecificSpi<S> {
        self.as_ref()
    }
    #[inline]
    fn spi_mut(&mut self) -> &mut SpecificSpi<S> {
        self.as_mut()
    }
}

//=============================================================================
// SpiFuture
//=============================================================================

/// A [`Future`]-like interface for SPI transactions
///
/// See the [module-level](self) documentation for more details.
///
/// [`Future`]: core::future::Future
pub struct SpiFuture<R, B, W = fn()>
where
    R: AsSpi,
    B: AsRef<[u8]> + AsMut<[u8]> + 'static,
    W: FnOnce() + 'static,
{
    resource: R,
    buf: B,
    sent: usize,
    rcvd: usize,
    waker: Option<W>,
}

impl<R, B> SpiFuture<R, B>
where
    R: AsSpi,
    B: AsRef<[u8]> + AsMut<[u8]> + 'static,
{
    /// Create a new [`SpiFuture`] with no waker
    #[inline]
    pub fn new(resource: R, buf: B) -> Self {
        let buf_len = buf.as_ref().len();
        let spi_len = resource.spi().transaction_length() as usize;
        if (spi_len > 4 && buf_len != spi_len) || (spi_len <= 4 && buf_len % spi_len != 0) {
            panic!("Invalid SpiFuture buffer length");
        }
        SpiFuture {
            resource,
            buf,
            sent: 0,
            rcvd: 0,
            waker: None,
        }
    }
}

impl<S, B> SpiFuture<S, B>
where
    S: AsSpi,
    B: AsRef<[u8]> + AsMut<[u8]> + 'static,
{
    /// Add a waker to the [`SpiFuture`]
    ///
    /// This function changes the waker type, so it must take an owned `self`.
    #[inline]
    pub fn with_waker<W>(self, waker: W) -> SpiFuture<S, B, W>
    where
        W: FnOnce() + 'static,
    {
        SpiFuture {
            resource: self.resource,
            buf: self.buf,
            sent: self.sent,
            rcvd: self.rcvd,
            waker: Some(waker),
        }
    }
}

impl<R, B, W> SpiFuture<R, B, W>
where
    R: AsSpi,
    B: AsRef<[u8]> + AsMut<[u8]> + 'static,
    W: FnOnce() + 'static,
{
    #[inline]
    fn step(&self) -> usize {
        let len = self.resource.spi().transaction_length() as usize;
        if len > 4 {
            4
        } else {
            len
        }
    }

    /// Start the [`SpiFuture`] transaction
    ///
    /// This will assert the SS pin, if present, and enable the `DRE` and `RXC`
    /// interrupts.
    #[inline]
    pub fn start(&mut self) {
        self.resource.assert_ss();
        self.resource
            .spi_mut()
            .enable_interrupts(Flags::DRE | Flags::RXC);
    }

    /// Send the next set of bytes from the buffer
    ///
    /// This method should be called from the `DRE` interrupt handler. Once all
    /// bytes of the transaction have been sent, this function will
    /// automatically disable the `DRE` interrupt.
    #[inline]
    pub fn send(&mut self) -> Result<(), Error> {
        let step = self.step();
        let spi = self.resource.spi_mut();
        let buf = self.buf.as_ref();
        let _ = spi.read_flags_errors()?;
        if let Some(buf) = buf.get(self.sent..) {
            let mut data = buf.iter();
            let mut bytes = [0; 4];
            let mut iter = bytes.iter_mut();
            for _ in 0..step {
                match (iter.next(), data.next()) {
                    (Some(b), Some(d)) => *b = *d,
                    _ => break,
                }
            }
            let word = u32::from_le_bytes(bytes);
            unsafe { spi.write_data(word as Data) };
            self.sent += step;
        }
        if self.sent >= buf.len() {
            spi.disable_interrupts(Flags::DRE);
        }
        Ok(())
    }

    /// Received the next set of bytes and write them to the buffer
    ///
    /// This method should be called from the `RXC` interrupt handler. Once all
    /// bytes of the transaction have been received, this function will
    /// automatically disable the `RXC` interrupt, deassert the SS pin (if
    /// present), and call the waker (if present).
    #[inline]
    pub fn recv(&mut self) -> Result<(), Error> {
        let step = self.step();
        let spi = self.resource.spi_mut();
        let buf = self.buf.as_mut();
        let _ = spi.read_flags_errors()?;
        if self.rcvd < self.sent {
            let buf = unsafe { buf.get_unchecked_mut(self.rcvd..) };
            let mut data = buf.into_iter();
            let word = unsafe { spi.read_data() as u32 };
            let bytes = word.to_le_bytes();
            let mut iter = bytes.iter();
            for _ in 0..step {
                match (data.next(), iter.next()) {
                    (Some(d), Some(b)) => *d = *b,
                    _ => break,
                }
            }
            self.rcvd += step;
        }
        if self.rcvd >= buf.len() {
            spi.disable_interrupts(Flags::RXC);
            self.resource.deassert_ss();
            if let Some(waker) = self.waker.take() {
                waker();
            }
        }
        Ok(())
    }

    /// Poll the [`SpiFuture`]
    ///
    /// This function will return [`Poll::Pending`] until all bytes have been
    /// received. When the transaction is complete, it will return
    /// [`Poll::Ready`] with a reference to the completed buffer.
    #[inline]
    pub fn poll(&self) -> Poll<&[u8]> {
        let buf = self.buf.as_ref();
        if self.rcvd >= buf.len() {
            Poll::Ready(buf)
        } else {
            Poll::Pending
        }
    }

    /// Consume the [`SpiFuture`] and free its components
    ///
    /// If the transaction is complete, this function will consume the
    /// [`SpiFuture`] and return the resource and buffer.
    ///
    /// If the transaction is not complete, it will return `Err(self)`.
    #[inline]
    pub fn free(self) -> Result<(R, B), Self> {
        if self.rcvd >= self.buf.as_ref().len() {
            Ok((self.resource, self.buf))
        } else {
            Err(self)
        }
    }

    /// Consume the [`SpiFuture`] and free its components without checking for
    /// completion
    ///
    /// Ending the transaction prematurely could leave the [`Spi`] in an
    /// inconsistent state. It is not safe to call this function unless the
    /// transaction is complete.
    ///
    /// [`Spi`]: super::spi::Spi
    #[inline]
    pub unsafe fn free_unchecked(self) -> (R, B) {
        (self.resource, self.buf)
    }
}
