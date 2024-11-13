//! Use the SERCOM peripheral for UART communications
//!
//! Configuring an UART peripheral occurs in three steps. First, you must create
//! a set of [`Pads`] for use by the peripheral. Next, you assemble pieces into
//! a [`Config`] struct. After configuring the peripheral, you then [`enable`]
//! it, yielding a functional [`Uart`] struct.
//! Transactions are performed using the [`embedded_io::Write`],
//! [`embedded_io::Read`], [`embedded_hal_nb::serial::Write`], and
//! [`embedded_hal_nb::serial::Read`] traits.
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
//! A `Pads` type takes five or six type parameters, depending on the chip.The
//! first type always specifies the `Sercom`. On SAMx5x chips, the second type
//! specifies the `IoSet`. The remaining four, `DI`, `DO`, `CK` and `SS`,
//! represent the Data In, Data Out, Sclk and SS pads respectively. Each of the
//! remaining type parameters is an [`OptionalPad`] and defaults to [`NoneT`]. A
//! [`Pad`] is just a [`Pin`] configured in the correct [`PinMode`] that
//! implements [`IsPad`]. The [`bsp_pins!`](crate::bsp_pins) macro can be
//! used to define convenient type aliases for [`Pad`] types.
//!
//! ```
//! use atsamd_hal::gpio::{PA08, PA09, AlternateC};
//! use atsamd_hal::sercom::{Sercom0, uart};
//! use atsamd_hal::typelevel::NoneT;
//!
//! type Rx = Pin<PA08, AlternateC>;
//! type Tx = Pin<PA09, AlternateC>;
//! type Pads = uart::Pads<Sercom0, Rx, Tx>;
//! ```
//!
//! Alternatively, you can use the [`PadsFromIds`] alias to define a set of
//! `Pads` in terms of [`PinId`]s instead of `Pin`s. This is useful when you
//! don't have [`Pin`] aliases pre-defined.
//!
//! ```
//! use atsamd_hal::gpio::{PA08, PA09};
//! use atsamd_hal::sercom::{Sercom0, uart};
//!
//! type Pads = uart::PadsFromIds<Sercom0, PA08, PA09>;
//! ```
//!
//! Instances of [`Pads`] are created using the builder pattern. Start by
//! creating an empty set of [`Pads`] using [`Default`]. Then pass each
//! respective [`Pin`] using the corresponding methods.
//!
//! On SAMD21 and SAMx5x chips, the builder methods automatically convert each
//! pin to the correct [`PinMode`]. But for SAMD11 chips, users must manually
//! convert each pin before calling the builder methods. This is a consequence
//! of inherent ambiguities in the SAMD11 SERCOM pad definitions. Specifically,
//! the same [`PinId`] can correspond to two different [`PadNum`]s for the
//! *same* `Sercom`.
//!
//! ```
//! use atsamd_hal::pac::Peripherals;
//! use atsamd_hal::gpio::Pins;
//! use atsamd_hal::sercom::{Sercom0, uart};
//!
//! let mut peripherals = Peripherals::take().unwrap();
//! let pins = Pins::new(peripherals.PORT);
//! let pads = uart::Pads::<Sercom0>::default()
//!     .rx(pins.pa09)
//!     .tx(pins.pa08);
//! ```
//!
//! To be accepted as [`ValidPads`], a set of [`Pads`] must do two things:
//! - Specify a type for at least one of `RX` or `TX`
//! - Satisfy the `RxpoTxpo` trait (SAMD11/SAMD21), or the `Rxpo` and `Txpo`
//!   traits (SAMx5x)
//!
//! # [`Config`]
//!
//! Next, create a [`Config`] struct, which represents the UART peripheral in
//! its disabled state. A [`Config`] is specified with two type parameters: the
//! [`Pads`] type; and a [`CharSize`], which defaults to [`EightBit`].
//!
//! ```
//! use atsamd_hal::gpio::{PA08, PA09};
//! use atsamd_hal::sercom::{Sercom0, uart};
//! use atsamd_hal::sercom::uart::{NineBit};
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
//! The [`Config`] struct can configure the peripheral in one of two ways:
//!
//! * A set of methods is provided to use in a builder pattern: for example
//!   [`baud`](Config::baud), [`stop_bits`](Config::stop_bits), etc. These
//!   methods take `self` and return `Self`.
//! * A set of methods is provided to use as setters: for example
//!   [`set_baud`](Config::set_baud), [`set_stop_bits`](Config::set_stop_bits),
//!   etc. These methods take `&mut self` and return nothing.
//!
//! In any case, the peripheral setup ends with a call to [`enable`], which
//! consumes the [`Config`] and returns an enabled [`Uart`] peripheral.
//!
//! ```
//! use atsamd_hal::sercom::uart::{StopBits, NineBit, BitOrder, BaudMode, Oversampling};
//!
//! let uart = uart::Config::new(&mclk, sercom, pads, freq)
//!     .baud(1.mhz(), BaudMode::Arithmetic(Oversampling::Bits16))
//!     .char_size::<NineBit>()
//!     .bit_order(BitOrder::LsbFirst)
//!     .stop_bits(StopBits::TwoBits)
//!     .enable();
//! ```
//!
//! Alternatively,
//!
//! ```
//! use atsamd_hal::sercom::uart::{StopBits, NineBit, BitOrder, BaudMode, Oversampling};
//!
//! let uart = uart::Config::new(&mclk, sercom, pads, freq);
//!     uart.set_baud(1.mhz(), BaudMode::Arithmetic(Oversampling::Bits16));
//!     uart.set_char_size::<NineBit>();
//!     uart.set_bit_order(BitOrder::LsbFirst);
//!     uart.set_stop_bits(StopBits::TwoBits);
//!     let uart = uart.enable();
//! ```
//!
//!
//! To be accepted as a [`ValidConfig`], the [`Config`] must have at least one
//! of `Rx` or `Tx` pads.
//!
//! ## [`CharSize`]
//!
//! The UART peripheral can be configured to use different character sizes. By
//! default, a [`Config`] is configured with an [`EightBit`] character size.
//! This can be changed through the [`char_size`](Config::char_size) method.
//! Changing the character normally also changes the [`Config`]'s type.
//! Alternatively, you can also use a [`DynCharSize`] through the
//! [`dyn_char_size`](Config::dyn_char_size) method. This enables you to
//! dynamically change the character size on the fly through the
//! [`set_dyn_char_size`](Config::set_dyn_char_size) method when calling
//! [`reconfigure`](Uart::reconfigure).
//!
//! ## Reading the current configuration
//!
//! It is possible to read the current configuration by using the getter methods
//! provided: for example [`get_baud`](Config::get_baud),
//! [`get_stop_bits`](Config::get_stop_bits), etc.
//!
//! # [`Uart`] and capabilities
//!
//! [`Uart`] structs can only be created from a [`Config`]. They have two type
//! parameters: the first one represents the underlying [`Config`], while the
//! second represents the [`Uart`]'s capabilities. The second type parameter can
//! be one of:
//!
//! * [`Rx`] or [`RxDuplex`]: Can perform receive transactions
//! * [`Tx`] or [`TxDuplex`]: Can perform transmit transactions
//! * [`Duplex`]: UART configured as duplex that can perform receive and
//!   transmit transactions. Additionally, the [`split`] method can be called to
//!   return a `Uart<C, RxDuplex>, Uart<C, TxDuplex>)` tuple. See the
//!   [Splitting](self#Splitting) section for more information.
//!
//! The nature of the underlying [`Pads`] contained inside [`Config`] determines
//! the type returned by a call to [`enable`]. If the pads only have a `TX` pin
//! specified, then [`enable`] will return a `Uart<C, Tx>`. Similarly, If the
//! pads only have a `RX` pin specified, then [`enable`] will return a `Uart<C,
//! Rx>`. Finally, if both `RX` and `TX` pins are specified, then [`enable`]
//! will return a `Uart<C, Duplex>`, which can be further split into a `Uart<C,
//! RxDuplex>` and a `Uart<C, TxDuplex>`.
//!
//! ```
//! use atsamd_hal::gpio::{PA08, PA09};
//! use atsamd_hal::sercom::{Sercom0, uart};
//! use atsamd_hal::sercom::uart::NineBit;
//! use atsamd_hal::typelevel::NoneT;
//!
//! // Assuming SAMD21 or SAMx5x
//! type Pads = uart::PadsFromIds<Sercom0, PA08, NoneT, PA09>;
//! type Config = uart::Config<Pads, NineBit>;
//! type UartRx = uart::Uart<Config, RxDuplex>;
//! type UartTx = uart::UartTx<Config, RxDuples>;
//! ```
//!
//! Only the [`Uart`] struct can actually perform
//! transactions. To do so, use the embedded HAL traits, like
//! [`embedded_hal_nb::serial::Read`], [`embedded_hal_nb::serial::Write`],
//! [`embedded_io::Read`], and [`embedded_io::Write`].
//!
//! ```
//! use nb::block;
//! use atsamd_hal::embedded_hal_nb::serial::Write;
//!
//! block!(uart_tx.write(0x0fe));
//! ```
//!
//! # UART flow control (CTS/RTS)
//!
//! This module supports CTS and RTS pins.
//!
//! The `RTS` pin is a fully hardware-controlled output pin that gets deasserted
//! when:
//!
//! * The USART receiver is disabled;
//! * The USART's RX buffer is full.
//!
//! The `CTS` pin is an input pin that provides an interrupt when a change
//! (rising or falling edge) is detected on the corresponding Pad. This
//! interrupt, `CTSIC`, can be enabled with the
//! [`enable_ctsic`](Uart::enable_ctsic) method only when the corresponding
//! [`Config`] has a `CTS` pad specified. The
//! [`disable_ctsic`](Uart::disable_ctsic) and
//! [`clear_ctsic`](Uart::clear_ctsic) methods are also available under the same
//! conditions. [This application note](https://www.silabs.com/documents/public/application-notes/an0059.0-uart-flow-control.pdf)
//! provides more information about UART hardware flow control.
//!
//! # Splitting
//!
//! A `Uart<C, Duplex>` can be split into its [`RxDuplex`] and [`TxDuplex`]
//! constituents:
//!
//! ```
//! use atsamd_hal::sercom::uart::Uart;
//! // Assume uart is a Uart<C, Duplex>
//! let (rx, tx) = uart.split();
//! ```
//!
//! # Joining
//!
//! When a `Uart<C, Duplex>` has been split into its [`RxDuplex`] and
//! [`TxDuplex`] parts, these parts can be joined back into a `Uart<C, Duplex>`
//! by calling the [`join`] function for `Uart<C, Duplex>`. It takes a `Uart<C,
//! RxDuplex>` and a `Uart<C, TxDuplex>` and moves them into a full [`Duplex`]
//! [`Uart`].
//!
//! ```
//! use atsamd_hal::sercom::uart::Uart;
//!
//! // Assume rx is a Uart<C, RxDuplex> and tx is a Uart<C, TxDuplex>
//! let uart = Uart::join(rx, tx);
//! // uart is now a Uart<C, Duplex>
//! ```
//!
//! The [`AsMut<Uart<C, Duplex>>`] trait is also implemented for `(&mut Uart<C,
//! RxDuplex>, &mut Uart<C, TxDuplex>)`. This is useful if you need an `&mut
//! Uart<C, Duplex>` but you only have a pair of `&mut Uart<C, RxDuplex>` and
//! `&mut Uart<C, TxDuplex>`. This can be leveraged to use the [`reconfigure`]
//! method when all you have is a pair of mutable references to the [`RxDuplex`]
//! and [`TxDuplex`] halves.
//!
//! ```
//! use atsamd_hal::sercom::uart::Uart;
//!
//! // Assume rx is a Uart<C, RxDuplex> and tx is a Uart<C, TxDuplex>
//!
//! // Reconfigure peripheral from mutable references to RxDuplex
//! // and TxDuplex halves
//! (&mut rx, &mut tx).as_mut().reconfigure(|c| c.set_run_in_standby(false));
//! ```
//!
//! # Reading the current configuration
//!
//! The `AsRef<Config<P, C>>` trait is implemented for `Uart<Config<P, C>, D>`.
//! This means you can use the `get_` methods implemented for `Config`, since
//! they take an `&self` argument.
//!
//! ```
//! // Assume uart is a Uart<C, D>
//! let (baud, baud_mode) = uart.as_ref().get_baud();
//! ```
//!
//! # Disabling and reconfiguring
//!
//! Some methods, such as [`disable`] and [`reconfigure`], need to operate on
//! all parts of a UART at once. In practice, this means that these methods
//! operate on the type that was returned by [`enable`]. This can be `Uart<C,
//! Rx>`, `Uart<C, Tx>`, or `Uart<C, Duplex>`, depending on how the
//! peripheral was configured.
//!
//! The [`reconfigure`] method gives out an `&mut Config` reference, which can
//! then use the `set_*` methods.
//!
//! ```
//! use atsamd_hal::sercom::uart::Uart;
//! use atsamd_hal::time::*;
//!
//! // Assume config is a valid Duplex UART Config struct
//! let (rx, tx)= config.enable().split();
//!
//! // Send/receive data with tx/rx halves...
//!
//! // If the UART peripheral is configured in Duplex mode,
//! // the two constituting halves need to be joined back into
//! // a Uart<C, Duplex> before calling disable()
//! let uart = Uart::join(rx, tx);
//!
//! // Reconfigure UART peripheral
//! uart.reconfigure(|c| c.set_run_in_standby(false));
//!
//! // Disable UART peripheral
//! let config = uart.disable();
//! ```
//!
//! # Non-supported advanced features
//!
//! * Synchronous mode (USART) is not supported
//! * LIN mode is not supported (SAMx5x)
//! * 32-bit extension mode is not supported (SAMx5x). If you need to transfer
//!   slices, consider using the DMA methods instead. The <span class="stab
//!   portability" title="Available on crate feature `dma`
//!   only"><code>dma</code></span> Cargo feature must be enabled.
//!
//! # Using UART with DMA <span class="stab portability" title="Available on crate feature `dma` only"><code>dma</code></span>
//!
//! This HAL includes support for DMA-enabled UART transfers. Use
//! [`Uart::with_rx_channel`] and [`Uart::with_tx_channel`] to attach DMA
//! channels to the [`Uart`] struct. A DMA-enabled [`Uart`] implements the
//! blocking [`embedded_io::Write`] and/or [`embedded_io::Read`] traits, which
//! can be used to perform UART read/writes which are fast, continuous and low
//! jitter, even if they are preemped by a higher priority interrupt.
//!
//!
//! ```no_run
//! use atsamd_hal::dmac::channel::{AnyChannel, Ready};
//! use atsand_hal::sercom::Uart::{I2c, ValidConfig, Error, TxDuplex};
//! use atsamd_hal::embedded_io::Write;
//! fn uart_send_with_dma<A: ValidConfig, C: AnyChannel<Status = Ready>>(uart: Uart<A, TxDuplex>, channel: C, bytes: &[u8]) -> Result<(), Error>{
//!     // Attach a DMA channel
//!     let uart = uart.with_tx_channel(channel);
//!     uart.write(bytes)?;
//! }
//! ```
//!
//! ## Non-blocking DMA transfers
//!
//! Non-blocking DMA transfers are also supported.
//!
//! The provided [`send_with_dma`] and
//! [`receive_with_dma`] build and begin a
//! [`dmac::Transfer`], thus starting the UART
//! in a non-blocking way. Note that these methods require `'static` buffers in
//! order to remain memory-safe.
//!
//! Optionally, interrupts can be enabled on the provided
//! [`Channel`]. Please refer to the [`dmac`](crate::dmac) module-level
//! documentation for more information.
//!
//! ```
//! // Assume channel0 and channel1 are configured `dmac::Channel`s,
//! // rx is a Uart<C, RxDuplex>, and tx is a Uart<C, TxDuplex>.
//!
//! /// Create data to send
//! let tx_buffer: [u8; 50] = [0xff; 50];
//! let rx_buffer: [u8; 100] = [0xab; 100];
//!
//! // Launch transmit transfer
//! let tx_dma = tx.send_with_dma(&mut tx_buffer, channel0, |_| {});
//!
//! // Launch receive transfer
//! let rx_dma = rx.receive_with_dma(&mut rx_buffer, channel1, |_| {});
//!
//! // Wait for transfers to complete and reclaim resources
//! let (chan0, tx_buffer, tx) = tx_dma.wait();
//! let (chan1, rx, rx_buffer) = rx_dma.wait();
//! ```
//!
//! # `async` operation <span class="stab portability" title="Available on crate feature `async` only"><code>async</code></span>
//!
//! A [`Uart`] can be used for `async` operations. Configuring a [`Uart`] in
//! async mode is relatively simple:
//!
//! * Bind the corresponding `SERCOM` interrupt source to the UART
//!   [`InterruptHandler`] (refer to the module-level [`async_hal`]
//!   documentation for more information).
//! * Turn a previously configured [`Uart`] into a [`UartFuture`] by calling
//!   [`Uart::into_future`]
//! * Optionally, add DMA channels to RX, TX or both using
//!   [`UartFuture::with_rx_dma_channel`] and
//!   [`UartFuture::with_tx_dma_channel`]. The API is exactly the same whether
//!   DMA channels are used or not.
//! * Use the provided async methods for reading or writing to the UART
//!   peripheral.
//!
//! `UartFuture` implements `AsRef<Uart>` and `AsMut<Uart>` so
//! that it can be reconfigured using the regular [`Uart`] methods. It also
//! exposes a [`split`](UartFuture::split) method to split it into its RX and TX
//! parts.
//!
//!  ## Considerations when using `async` [`Uart`] with DMA <span class="stab
//! portability" title="Available on crate feature `async`
//! only"><code>async</code></span> <span class="stab portability"
//! title="Available on crate feature `dma` only"><code>dma</code></span>
//!
//! * An [`Uart`] struct must be turned into an [`UartFuture`] by calling
//!   [`Uart::into_future`] before calling `with_dma_channel`. The DMA channel
//!   itself must also be configured in async mode by using
//!   [`DmaController::into_future`](crate::dmac::DmaController::into_future).
//!   If a DMA channel is added to the [`Uart`] struct before it is turned into
//!   an [`UartFuture`], it will not be able to use DMA in async mode.
//!
//! ```
//! // This will work
//! let uart = uart.into_future().with_dma_channels(rx_channel, tx_channel);
//!
//! // This won't
//! let uart = uart.with_dma_channels(rx_channel, tx_channel).into_future();
//! ```
//!
//! ### Safety considerations
//!
//! In `async` mode, an SPI+DMA transfer does not require `'static` source and
//! destination buffers. This, in theory, makes its use `unsafe`. However it is
//! marked as safe for better ergonomics.
//!
//! This means that, as an user, you **must** ensure that the [`Future`]s
//! returned by the [`read`](UartFuture::read) and [`write`](UartFuture::write)
//! methods may never be forgotten through [`forget`] or by wrapping them with a
//! [`ManuallyDrop`].
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
//! uart.read(&mut buffer).await?;
//!
//! // This is also safe: we launch a transfer, which is then immediately cancelled
//! futures::select_biased! {
//!     _ = uart.read(&mut buffer)?,
//!     _ = always_ready(),
//! }
//!
//! // This, while contrived, is also safe.
//! {
//!     use core::future::Future;
//!
//!     let future = uart.read(&mut buffer);
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
//!     let future = core::mem::ManuallyDrop::new(uart.read(&mut buffer));
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
//! [`disable`]: Uart::disable
//! [`reconfigure`]: Uart::reconfigure
//! [`bsp_pins`]: crate::bsp_pins
//! [`Pin`]: crate::gpio::pin::Pin
//! [`Pin`]: crate::gpio::pin::Pin
//! [`PinId`]: crate::gpio::pin::PinId
//! [`PinMode`]: crate::gpio::pin::PinMode
//! [`split`]: Uart::split
//! [`join`]: Uart::join
//! [`NoneT`]: crate::typelevel::NoneT
//! [`serial::Write`]: embedded_hal::serial::Write
//! [`serial::Read`]: embedded_hal::serial::Read
//! [`receive_with_dma`]: Self::receive_with_dma
//! [`send_with_dma`]: Self::send_with_dma
//! [`dmac::Transfer`]: crate::dmac::Transfer
//! [`Channel`]: crate::dmac::Channel
//! [`async_hal`]: crate::async_hal
//! [`forget`]: core::mem::forget
//! [`ManuallyDrop`]: core::mem::ManuallyDrop
//! [`Future`]: core::future::Future
//! [`poll`]: core::future::Future::poll

use atsamd_hal_macros::{hal_cfg, hal_module};

#[hal_module(
    any("sercom0-d11", "sercom0-d21") => "uart/pads_thumbv6m.rs",
    "sercom0-d5x" => "uart/pads_thumbv7em.rs",
)]
mod pads {}

pub use pads::*;

mod reg;
use reg::Registers;

mod charsize;
pub use charsize::*;

mod flags;
pub use flags::*;

mod config;
pub use config::*;

pub mod impl_ehal;

#[cfg(feature = "async")]
mod async_api;
#[cfg(feature = "async")]
pub use async_api::*;

use crate::{
    sercom::pad::SomePad,
    typelevel::{NoneT, Sealed},
};
use core::marker::PhantomData;
use num_traits::AsPrimitive;

/// Size of the SERCOM's `DATA` register
#[hal_cfg(any("sercom0-d11", "sercom0-d21"))]
pub type DataReg = u16;

/// Size of the SERCOM's `DATA` register
#[hal_cfg("sercom0-d5x")]
pub type DataReg = u32;

//=============================================================================
// Stop bits, parity, baud rate, bit order
//=============================================================================

/// Number of stop bits in a UART frame
#[derive(Debug, Clone, Copy)]
pub enum StopBits {
    /// 1 stop bit
    OneBit,
    /// 2 stop bits
    TwoBits,
}

/// Parity setting of a UART frame
#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum Parity {
    /// No parity
    None,
    /// Even parity
    Even,
    /// Odd parity
    Odd,
}

/// Bit order of a UART frame
#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum BitOrder {
    /// MSB-first
    MsbFirst,
    /// LSB-first
    LsbFirst,
}

/// Baudrate oversampling values
///
/// *NOTE* 3x oversampling has been intentionally left out
#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum Oversampling {
    // 3 samples per bit
    // Bits3 = 3,
    /// 8 samples per bit
    Bits8 = 8,
    /// 16 samples per bit
    Bits16 = 16,
}

/// Baudrate calculation in asynchronous mode
#[derive(Debug, Clone, Copy)]
pub enum BaudMode {
    /// Asynchronous arithmetic baud calculation
    Arithmetic(Oversampling),
    /// Asynchronous fractional baud calculation
    Fractional(Oversampling),
}

//=============================================================================
// Capability
//=============================================================================

/// Type-level `enum` representing the capabilities of a UART peripheral
pub trait Capability: Sealed {
    /// Available interrupt flags for the specified capability
    const FLAG_MASK: u8;
    /// Available status flags for the specified capability
    const STATUS_MASK: u16;
    /// Enable `CTRLA.RXEN` field?
    const RXEN: bool;
    /// Enable `CTRLA.TXEN` field?
    const TXEN: bool;
}

/// Type-level enum representing a UART that can transmit
pub trait Transmit: Capability {}

/// Type-level enum representing a UART that can receive
pub trait Receive: Capability {}

/// Type-level enum representing a UART that has transmit or receive
/// capability, but not both
pub trait Simplex: Capability {}

/// Type-level enum representing a UART that is *not* half of a split
/// [`Duplex`]
pub trait SingleOwner: Capability {}

/// Marker type representing a UART that has both transmit and receive
/// capability
pub enum Duplex {}
impl Sealed for Duplex {}
impl Capability for Duplex {
    // All flags are valid for a Duplex UART
    const FLAG_MASK: u8 = DUPLEX_FLAG_MASK;

    // All status flags are valid for a Duplex UART
    const STATUS_MASK: u16 = DUPLEX_STATUS_MASK;

    const RXEN: bool = true;
    const TXEN: bool = true;
}
impl Receive for Duplex {}
impl Transmit for Duplex {}
impl SingleOwner for Duplex {}

/// Marker type representing a UART that can only receive
pub enum Rx {}
impl Sealed for Rx {}
impl Capability for Rx {
    // Available interrupt flags for a RX half-UART
    const FLAG_MASK: u8 = RX_FLAG_MASK;

    // Available status flags for a RX half-UART
    const STATUS_MASK: u16 = RX_STATUS_MASK;

    const RXEN: bool = true;
    const TXEN: bool = false;
}
impl Receive for Rx {}
impl Simplex for Rx {}
impl SingleOwner for Rx {}

/// Marker type representing a UART that can only transmit
pub enum Tx {}
impl Sealed for Tx {}
impl Capability for Tx {
    // Available interrupt flags for a TX half-UART
    const FLAG_MASK: u8 = TX_FLAG_MASK;

    // There are no settable/clearable status flags for TX half-UARTs
    const STATUS_MASK: u16 = 0;

    const RXEN: bool = false;
    const TXEN: bool = true;
}
impl Transmit for Tx {}
impl Simplex for Tx {}
impl SingleOwner for Tx {}

/// Marker type representing the Rx half of a  [`Duplex`] UART
pub enum RxDuplex {}
impl Sealed for RxDuplex {}
impl Capability for RxDuplex {
    // Available interrupt flags for a RX half-UART
    const FLAG_MASK: u8 = RX_FLAG_MASK;

    // Available status flags for a RX half-UART
    const STATUS_MASK: u16 = RX_STATUS_MASK;

    const RXEN: bool = true;
    const TXEN: bool = false;
}
impl Receive for RxDuplex {}

/// Marker type representing a the Tx half of a [`Duplex`] UART
pub enum TxDuplex {}
impl Sealed for TxDuplex {}
impl Capability for TxDuplex {
    // Available interrupt flags for a TX half-UART
    const FLAG_MASK: u8 = TX_FLAG_MASK;

    // There are no settable/clearable status flags for TX half-UARTs
    const STATUS_MASK: u16 = 0;

    const RXEN: bool = false;
    const TXEN: bool = true;
}

impl Transmit for TxDuplex {}

//=============================================================================
// Uart
//=============================================================================

/// Abstraction over a UART peripheral, allowing to perform UART transactions.
/// The second type parameter, `D`, denotes what the struct's [`Capability`] is.
///
/// * [`Rx`] or [`RxDuplex`]: Can perform receive transactions
/// * [`Tx`] or [`TxDuplex`]: Can perform transmit transactions
/// * [`Duplex`]: Can perform receive and transmit transactions. Additionally,
///   you can call [`split`](Uart::split) to return a `(Uart<C, RxDuplex>,
///   Uart<C, TxDuplex>)` tuple.
pub struct Uart<C, D, RxDma = NoneT, TxDma = NoneT>
where
    C: ValidConfig,
    D: Capability,
{
    config: C,
    capability: PhantomData<D>,
    rx_channel: RxDma,
    tx_channel: TxDma,
}

impl<C, D, R, T> Uart<C, D, R, T>
where
    C: ValidConfig,
    D: Capability,
{
    /// Obtain a pointer to the `DATA` register. Necessary for DMA transfers.
    #[cfg(feature = "dma")]
    #[inline]
    pub(crate) fn data_ptr(&self) -> *mut C::Word {
        self.config.as_ref().registers.data_ptr()
    }

    /// Helper method to remove the interrupt flags not pertinent to `Self`'s
    /// `Capability`
    #[inline]
    fn capability_flags(flags: Flags) -> Flags {
        flags & Flags::from_bits_retain(D::FLAG_MASK)
    }

    /// Helper method to remove the status flags not pertinent to `Self`'s
    /// `Capability`
    #[inline]
    fn capability_status(status: Status) -> Status {
        status & Status::from_bits_retain(D::STATUS_MASK)
    }

    /// Read the interrupt flags
    #[inline]
    pub fn read_flags(&self) -> Flags {
        self.config.as_ref().registers.read_flags()
    }

    /// Clear interrupt status flags
    ///
    /// Setting the `ERROR`, `RXBRK`, `CTSIC`, `RXS`, or `TXC` flag will clear
    /// the interrupts. This function has no effect on the `DRE` or
    /// `RXC` flags.
    ///
    /// Note that only the flags pertinent to `Self`'s [`Capability`]
    /// will be cleared. The other flags will be **SILENTLY IGNORED**.
    ///
    /// * Available flags for [`Receive`] capability: `RXC`, `RXS`, `RXBRK` and
    ///   `ERROR`
    /// * Available flags for [`Transmit`] capability: `DRE` and `TXC`.
    ///   **Note**: The `CTSIC` flag can only be cleared if a `CTS` Pad was
    ///   specified in the [`Config`] via the [`clear_ctsic`](Uart::clear_ctsic)
    ///   method.
    /// * Since [`Duplex`] [`Uart`]s are [`Receive`] + [`Transmit`] they have
    ///   all flags available.
    ///
    /// **Warning:** The implementations of of
    /// [`Write::flush`](embedded_hal_nb::serial::Write::flush) waits on and
    /// clears the `TXC` flag. Manually clearing this flag could cause it to
    /// hang indefinitely.
    #[inline]
    pub fn clear_flags(&mut self, flags: Flags) {
        // Remove flags not pertinent to Self's Capability
        let flags = Self::capability_flags(flags);
        self.config.as_mut().registers.clear_flags(flags);
    }

    /// Enable interrupts for the specified flags.
    ///
    /// Note that only the flags pertinent to `Self`'s [`Capability`]
    /// will be cleared. The other flags will be **SILENTLY IGNORED**.
    ///
    /// * Available flags for [`Receive`] capability: `RXC`, `RXS`, `RXBRK` and
    ///   `ERROR`
    /// * Available flags for [`Transmit`] capability: `DRE` and `TXC`.
    ///   **Note**: The `CTSIC` interrupt can only be enabled if a `CTS` Pad was
    ///   specified in the [`Config`] via the
    ///   [`enable_ctsic`](Uart::enable_ctsic) method.
    /// * Since [`Duplex`] [`Uart`]s are [`Receive`] + [`Transmit`] they have
    ///   all flags available.
    #[inline]
    pub fn enable_interrupts(&mut self, flags: Flags) {
        // Remove flags not pertinent to Self's Capability
        let flags = Self::capability_flags(flags);
        self.config.as_mut().registers.enable_interrupts(flags);
    }

    /// Disable interrupts for the specified flags.
    ///
    /// Note that only the flags pertinent to `Self`'s [`Capability`]
    /// will be cleared. The other flags will be **SILENTLY IGNORED**
    ///
    /// * Available flags for [`Receive`] capability: `RXC`, `RXS`, `RXBRK` and
    ///   `ERROR`
    /// * Available flags for [`Transmit`] capability: `DRE` and `TXC`.
    ///   **Note**: The `CTSIC` interrupt can only be disabled if a `CTS` Pad
    ///   was specified in the [`Config`] via the
    ///   [`disable_ctsic`](Uart::disable_ctsic) method.
    /// * Since [`Duplex`] [`Uart`]s are [`Receive`] + [`Transmit`] they have
    ///   all flags available.
    #[inline]
    pub fn disable_interrupts(&mut self, flags: Flags) {
        // Remove flags not pertinent to Self's Capability
        let flags = Self::capability_flags(flags);
        self.config.as_mut().registers.disable_interrupts(flags);
    }

    /// Read the status flags
    #[inline]
    pub fn read_status(&self) -> Status {
        self.config.as_ref().registers.read_status()
    }

    /// Clear the status flags
    ///
    /// Note that only the status flags pertinent to `Self`'s [`Capability`]
    /// will be cleared. The other stattus flags will be **SILENTLY IGNORED**.
    ///
    /// * Available status flags for [`Receive`] capability: `PERR`, `FERR`,
    ///   `BUFOVF`, `ISF` and `COLL`
    /// * [`Transmit`]-only [`Uart`]s have no clearable status flags.
    /// * Since [`Duplex`] [`Uart`]s are [`Receive`] + [`Transmit`] they have
    ///   all status flags available.
    #[inline]
    pub fn clear_status(&mut self, status: Status) {
        // Remove status flags not pertinent to Self's Capability
        let flags = Self::capability_status(status);
        self.config.as_mut().registers.clear_status(flags);
    }

    #[inline]
    pub(super) fn _reconfigure<F>(&mut self, update: F)
    where
        F: FnOnce(&mut SpecificConfig<C>),
    {
        self.config.as_mut().registers.enable_peripheral(false);
        update(self.config.as_mut());
        self.config.as_mut().registers.enable_peripheral(true);
    }
}

impl<C, D, R, T> Uart<C, D, R, T>
where
    C: ValidConfig,
    <C::Pads as PadSet>::Cts: SomePad,
    D: Transmit,
{
    /// Clear the `CTSIC` interrupt flag
    #[inline]
    pub fn clear_ctsic(&mut self) {
        let bit = CTSIC;
        self.config
            .as_mut()
            .registers
            .clear_flags(Flags::from_bits_retain(bit));
    }

    /// Enable the `CTSIC` interrupt
    #[inline]
    pub fn enable_ctsic(&mut self) {
        let bit = CTSIC;
        self.config
            .as_mut()
            .registers
            .enable_interrupts(Flags::from_bits_retain(bit));
    }

    /// Disable the `CTSIC` interrupt
    #[inline]
    pub fn disable_ctsic(&mut self) {
        let bit = CTSIC;
        self.config
            .as_mut()
            .registers
            .disable_interrupts(Flags::from_bits_retain(bit));
    }
}

impl<C, D, R, T> Uart<C, D, R, T>
where
    C: ValidConfig,
    D: Simplex,
{
    /// Disable the UART peripheral and return the underlying [`Config`]
    #[inline]
    pub fn disable(self) -> C {
        let mut config = self.config;
        config.as_mut().registers.disable();
        config
    }

    /// Reconfigure the UART.
    ///
    /// Calling this method will temporarily disable the SERCOM peripheral, as
    /// some registers are enable-protected. This may interrupt any ongoing
    /// transactions.
    ///
    /// ```
    /// use atsamd_hal::sercom::uart::{BaudMode, Oversampling, Uart};
    /// uart.reconfigure(|c| c.set_run_in_standby(false));
    /// ```
    #[inline]
    pub fn reconfigure<U>(&mut self, update: U)
    where
        U: FnOnce(&mut SpecificConfig<C>),
    {
        self._reconfigure(update);
    }
}

#[cfg(feature = "dma")]
impl<C, D, T> Uart<C, D, NoneT, T>
where
    C: ValidConfig,
    D: Capability,
{
    /// Attach a DMA channel to this [`Uart`] for RX transactions. Its
    /// [`Read`](embedded_io::Read) implementation will use DMA to
    /// carry out its transactions.
    pub fn with_rx_channel<R: crate::dmac::AnyChannel<Status = crate::dmac::Ready>>(
        self,
        rx_channel: R,
    ) -> Uart<C, D, R, T> {
        Uart {
            config: self.config,
            capability: self.capability,
            tx_channel: self.tx_channel,
            rx_channel,
        }
    }
}

#[cfg(feature = "dma")]
impl<C, D, R> Uart<C, D, R, NoneT>
where
    C: ValidConfig,
    D: Capability,
{
    /// Attach a DMA channel to this [`Uart`] for TX transactions. Its
    /// [`Write`](embedded_io::Write) implementation will use DMA to
    /// carry out its transactions.
    pub fn with_tx_channel<T: crate::dmac::AnyChannel<Status = crate::dmac::Ready>>(
        self,
        tx_channel: T,
    ) -> Uart<C, D, R, T> {
        Uart {
            config: self.config,
            capability: self.capability,
            rx_channel: self.rx_channel,
            tx_channel,
        }
    }
}

#[cfg(feature = "dma")]
impl<C, D, R, T, S> Uart<C, D, R, T>
where
    C: ValidConfig,
    D: Capability,
    R: crate::dmac::AnyChannel<Status = S>,
    S: crate::dmac::ReadyChannel,
{
    /// Reclaim the RX DMA channel. Subsequent RX operations will no longer use
    /// DMA.
    pub fn take_rx_channel(self) -> (Uart<C, D, NoneT, T>, R) {
        (
            Uart {
                config: self.config,
                capability: self.capability,
                tx_channel: self.tx_channel,
                rx_channel: NoneT,
            },
            self.rx_channel,
        )
    }
}

#[cfg(feature = "dma")]
impl<C, D, R, T, S> Uart<C, D, R, T>
where
    C: ValidConfig,
    D: Capability,
    T: crate::dmac::AnyChannel<Status = S>,
    S: crate::dmac::ReadyChannel,
{
    /// Reclaim the TX DMA channel. Subsequent TX operations will no longer use
    /// DMA.
    pub fn take_tx_channel(self) -> (Uart<C, D, R, NoneT>, T) {
        (
            Uart {
                config: self.config,
                capability: self.capability,
                rx_channel: self.rx_channel,
                tx_channel: NoneT,
            },
            self.tx_channel,
        )
    }
}

impl<C, R, T> Uart<C, Duplex, R, T>
where
    C: ValidConfig,
{
    /// Split the [`Uart`] into [`RxDuplex`] and [`TxDuplex`] halves
    #[allow(clippy::type_complexity)]
    #[inline]
    pub fn split(self) -> (Uart<C, RxDuplex, R, NoneT>, Uart<C, TxDuplex, NoneT, T>) {
        let config = unsafe { core::ptr::read(&self.config) };
        (
            Uart {
                config: self.config,
                capability: PhantomData,
                rx_channel: self.rx_channel,
                tx_channel: NoneT,
            },
            Uart {
                config,
                capability: PhantomData,
                rx_channel: NoneT,
                tx_channel: self.tx_channel,
            },
        )
    }

    /// Disable the UART peripheral and return the underlying [`Config`]
    #[inline]
    pub fn disable(self) -> C {
        let mut config = self.config;
        config.as_mut().registers.disable();
        config
    }

    /// Update the UART [`Config`]uration.
    ///
    /// Calling this method will temporarily disable the SERCOM peripheral, as
    /// some registers are enable-protected. This may interrupt any ongoing
    /// transactions.
    ///
    /// ```
    /// use atsamd_hal::sercom::uart::{BaudMode, Oversampling, Uart};
    /// uart.reconfigure(|c| c.set_run_in_standby(false));
    /// ```
    #[inline]
    pub fn reconfigure<F>(&mut self, update: F)
    where
        F: FnOnce(&mut SpecificConfig<C>),
    {
        self._reconfigure(update);
    }

    /// Join [`RxDuplex`] and [`TxDuplex`] halves back into a full `Uart<C,
    /// Duplex>`
    pub fn join(rx: Uart<C, RxDuplex, R, NoneT>, tx: Uart<C, TxDuplex, NoneT, T>) -> Self {
        Self {
            config: rx.config,
            capability: PhantomData,
            rx_channel: rx.rx_channel,
            tx_channel: tx.tx_channel,
        }
    }
}

impl<C: ValidConfig, R, T> AsMut<Uart<C, Duplex, R, T>>
    for (
        &mut Uart<C, RxDuplex, R, NoneT>,
        &mut Uart<C, TxDuplex, NoneT, T>,
    )
{
    #[inline]
    fn as_mut(&mut self) -> &mut Uart<C, Duplex, R, T> {
        // SAFETY: Pointer casting &mut Uart<C, RxDuplex> into &mut
        // Uart<C, Duplex> should be safe as long as RxDuplex, TxDuplex, R and T are all
        // zero-sized types
        unsafe { &mut *(self.0 as *mut _ as *mut Uart<C, Duplex, R, T>) }
    }
}

impl<C, D> AsRef<SpecificConfig<C>> for Uart<C, D>
where
    C: ValidConfig,
    D: Capability,
{
    #[inline]
    fn as_ref(&self) -> &SpecificConfig<C> {
        self.config.as_ref()
    }
}

//=============================================================================
// Rx/Tx specific functionality
//=============================================================================

impl<C, D, R, T> Uart<C, D, R, T>
where
    C: ValidConfig,
    D: Receive,
    DataReg: AsPrimitive<C::Word>,
{
    /// Read from the DATA register
    ///
    /// # Safety
    ///
    /// Reading from the data register directly is `unsafe`, because it will
    /// clear the RXC flag, which could break assumptions made elsewhere in
    /// this module.
    #[inline]
    pub unsafe fn read_data(&mut self) -> DataReg {
        self.config.as_mut().registers.read_data()
    }

    /// Read the status register and convert into a [`Result`]
    /// containing the corresponding [`Flags`] or [`Error`]
    #[inline]
    fn read_flags_errors(&self) -> Result<Flags, Error> {
        self.read_status().check_bus_error()?;
        Ok(self.read_flags())
    }

    /// Flush the RX buffer and clear RX errors.
    ///
    /// **Note**: The datasheet states that disabling the receiver (RXEN) clears
    /// the RX buffer, and clears the BUFOVF, PERR and FERR bits.
    /// However, in practice, it seems like BUFOVF errors still pop
    /// up after a disable/enable cycle of the receiver, then immediately begin
    /// reading bytes from the DATA register. Instead, this method uses a
    /// workaround, which reads a few bytes to clear the RX buffer (3 bytes
    /// seems to be the trick), then manually clear the error bits.
    #[inline]
    pub fn flush_rx_buffer(&mut self) {
        // TODO Is this a hardware bug???

        // usart.ctrlb.modify(|_, w| w.rxen().clear_bit());
        // while usart.syncbusy.read().ctrlb().bit() ||
        // usart.ctrlb.read().rxen().bit_is_set() {}

        // usart.ctrlb.modify(|_, w| w.rxen().set_bit());
        // while usart.syncbusy.read().ctrlb().bit() ||
        // usart.ctrlb.read().rxen().bit_is_clear() {}

        for _ in 0..=2 {
            let _data = unsafe { self.config.as_mut().registers.read_data() };
        }

        // Clear all errors
        self.clear_status(
            Status::BUFOVF | Status::FERR | Status::PERR | Status::ISF | Status::COLL,
        );
    }
}

impl<C, D, R, T> Uart<C, D, R, T>
where
    C: ValidConfig,
    D: Transmit,
{
    /// Write to the DATA register
    ///
    /// # Safety
    ///
    /// Writing to the data register directly is `unsafe`, because it will clear
    /// the DRE flag, which could break assumptions made elsewhere in this
    /// module.
    #[inline]
    pub unsafe fn write_data(&mut self, data: DataReg) {
        self.config.as_mut().registers.write_data(data);
    }
}
