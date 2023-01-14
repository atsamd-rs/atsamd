//! Use the DMA Controller to perform transfers using the SERCOM peripheral
//!
//! See the [`mod@uart`], [`mod@i2c`] and [`mod@spi`] modules for the
//! corresponding DMA transfer implementations.

use crate::{
    dmac::{
        self,
        channel::{AnyChannel, Busy, CallbackStatus, Channel, InterruptFlags, Ready},
        transfer::BufferPair,
        Beat, Buffer, Transfer, TriggerAction,
    },
    sercom::{
        i2c::{self, I2c},
        spi::{self, Spi},
        uart::{self, Uart},
        Sercom,
    },
};

//=============================================================================
// I2C DMA transfers
//=============================================================================

/// Token type representing an [`I2c`](super::i2c::I2c) for which the bus is
/// ready to start a transaction. For use with
/// [`send_with_dma`](super::i2c::I2c::send_with_dma) and
/// [`receive_with_dma`](super::i2c::I2c::send_with_dma).
pub struct I2cBusReady;

unsafe impl<C: i2c::AnyConfig> Buffer for I2c<C> {
    type Beat = i2c::Word;

    #[inline]
    fn dma_ptr(&mut self) -> *mut Self::Beat {
        self.data_ptr()
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

impl<C: i2c::AnyConfig> I2c<C> {
    /// Initialize the bus to start receiving with DMA.
    ///
    /// In reality, this function only checks whether or not the I2C bus is
    /// ready to accept a new transaction. A call to [`init_dma_transfer`]
    /// should immediately be followed by a call to [`send_with_dma`] or
    /// [`receive_with_dma`].
    ///
    /// ```no_run
    /// # fn init_transfer<A: i2c::AnyConfig, C: AnyChannel<dmac::Ready>>(i2c: I2c<A>, chan0: C, buf_src: &'static mut [u8]){
    /// // Assume `i2c` is a fully configured `I2c`, and `chan0` a fully configured `dmac::Channel`.
    /// let token = i2c.init_dma_transfer()?;
    /// i2c.send_with_dma(ADDRESS, token, buf_src, chan0, |_| {});
    /// # }
    /// ```
    ///
    /// [`init_dma_transfer`]: super::i2c::I2c::init_dma_transfer
    /// [`send_with_dma`]: super::i2c::I2c::send_with_dma
    /// [`receive_with_dma`]: super::i2c::I2c::receive_with_dma
    pub fn init_dma_transfer(&mut self) -> Result<I2cBusReady, super::i2c::Error> {
        self.check_bus_status()?;
        Ok(I2cBusReady)
    }

    /// Transform an [`I2c`] into a DMA [`Transfer`]) and
    /// start receiving into the provided buffer. The buffer length must be 255
    /// bytes or shorter.
    ///
    /// It is recommended that you check for errors after the transfer is
    /// complete by calling [`read_status`](I2c::read_status).
    pub fn receive_with_dma<Ch, B, W>(
        self,
        address: u8,
        _ready_token: I2cBusReady,
        buf: B,
        mut channel: Ch,
        waker: W,
    ) -> Transfer<Channel<Ch::Id, Busy>, BufferPair<Self, B>, W>
    where
        Ch: AnyChannel<Status = Ready>,
        B: Buffer<Beat = i2c::Word> + 'static,
        W: FnOnce(CallbackStatus) + 'static,
    {
        let len = buf.buffer_len();
        assert!(len > 0 && len <= 255);

        channel
            .as_mut()
            .enable_interrupts(InterruptFlags::new().with_tcmpl(true));

        #[cfg(feature = "thumbv7")]
        let trigger_action = TriggerAction::BURST;

        #[cfg(feature = "thumbv6")]
        let trigger_action = TriggerAction::BEAT;

        // SAFETY: This is safe because the of the `'static` bound check
        // for `B`, and the fact that the buffer length of an `I2c` is always 1.
        let xfer = unsafe { dmac::Transfer::new_unchecked(channel, self, buf, false) };
        let mut xfer = xfer
            .with_waker(waker)
            .begin(C::Sercom::DMA_RX_TRIGGER, trigger_action);

        // SAFETY: we borrow the source from under a `Busy` transfer. While the type
        // system believes the transfer is running, we haven't enabled it in the
        // I2C peripheral yet, and so a trigger won't happen until we call
        // `start_dma_read`.
        unsafe { xfer.borrow_source().start_dma_read(address, len as u8) };
        xfer
    }

    /// Transform an [`I2c`] into a DMA [`Transfer`]) and
    /// start sending the provided buffer. The buffer length must be 255 bytes
    /// or shorter.
    ///
    /// It is recommended that you check for errors after the transfer is
    /// complete by calling [`read_status`](I2c::read_status).
    #[inline]
    pub fn send_with_dma<Ch, B, W>(
        self,
        address: u8,
        _ready_token: I2cBusReady,
        buf: B,
        mut channel: Ch,
        waker: W,
    ) -> Transfer<Channel<Ch::Id, Busy>, BufferPair<B, Self>, W>
    where
        Ch: AnyChannel<Status = Ready>,
        B: Buffer<Beat = i2c::Word> + 'static,
        W: FnOnce(CallbackStatus) + 'static,
    {
        let len = buf.buffer_len();
        assert!(len > 0 && len <= 255);

        channel
            .as_mut()
            .enable_interrupts(InterruptFlags::new().with_tcmpl(true));

        #[cfg(feature = "thumbv7")]
        let trigger_action = TriggerAction::BURST;

        #[cfg(feature = "thumbv6")]
        let trigger_action = TriggerAction::BEAT;

        // SAFETY: This is safe because the of the `'static` bound check
        // for `B`, and the fact that the buffer length of an `I2c` is always 1.
        let xfer = unsafe { dmac::Transfer::new_unchecked(channel, buf, self, false) };
        let mut xfer = xfer
            .with_waker(waker)
            .begin(C::Sercom::DMA_TX_TRIGGER, trigger_action);

        // SAFETY: we borrow the source from under a `Busy` transfer. While the type
        // system believes the transfer is running, we haven't enabled it in the
        // I2C peripheral yet, and so a trigger won't happen until we call
        // `start_dma_write`.
        unsafe {
            xfer.borrow_destination()
                .start_dma_write(address, len as u8)
        };
        xfer
    }
}

//=============================================================================
// UART DMA transfers
//=============================================================================
unsafe impl<C, D> Buffer for Uart<C, D>
where
    C: uart::ValidConfig,
    C::Word: Beat,
    D: uart::Capability,
{
    type Beat = C::Word;

    #[inline]
    fn dma_ptr(&mut self) -> *mut Self::Beat {
        self.data_ptr()
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

impl<C, D> Uart<C, D>
where
    Self: Buffer<Beat = C::Word>,
    C: uart::ValidConfig,
    D: uart::Receive,
{
    /// Transform an [`Uart`] into a DMA [`Transfer`]) and
    /// start receiving into the provided buffer.
    #[inline]
    pub fn receive_with_dma<Ch, B, W>(
        self,
        buf: B,
        mut channel: Ch,
        waker: W,
    ) -> Transfer<Channel<Ch::Id, Busy>, BufferPair<Self, B>, W>
    where
        Ch: AnyChannel<Status = Ready>,
        B: Buffer<Beat = C::Word> + 'static,
        W: FnOnce(CallbackStatus) + 'static,
    {
        channel
            .as_mut()
            .enable_interrupts(InterruptFlags::new().with_tcmpl(true));

        #[cfg(feature = "thumbv7")]
        let trigger_action = TriggerAction::BURST;

        #[cfg(feature = "thumbv6")]
        let trigger_action = TriggerAction::BEAT;

        // SAFETY: This is safe because the of the `'static` bound check
        // for `B`, and the fact that the buffer length of an `Uart` is always 1.
        let xfer = unsafe { dmac::Transfer::new_unchecked(channel, self, buf, false) };
        xfer.with_waker(waker)
            .begin(C::Sercom::DMA_RX_TRIGGER, trigger_action)
    }
}

impl<C, D> Uart<C, D>
where
    Self: Buffer<Beat = C::Word>,
    C: uart::ValidConfig,
    D: uart::Transmit,
{
    /// Transform an [`Uart`] into a DMA [`Transfer`]) and
    /// start sending the provided buffer.
    #[inline]
    pub fn send_with_dma<Ch, B, W>(
        self,
        buf: B,
        mut channel: Ch,
        waker: W,
    ) -> Transfer<Channel<Ch::Id, Busy>, BufferPair<B, Self>, W>
    where
        Ch: AnyChannel<Status = Ready>,
        B: Buffer<Beat = C::Word> + 'static,
        W: FnOnce(CallbackStatus) + 'static,
    {
        channel
            .as_mut()
            .enable_interrupts(InterruptFlags::new().with_tcmpl(true));

        #[cfg(feature = "thumbv7")]
        let trigger_action = TriggerAction::BURST;

        #[cfg(feature = "thumbv6")]
        let trigger_action = TriggerAction::BEAT;

        // SAFETY: This is safe because the of the `'static` bound check
        // for `B`, and the fact that the buffer length of an `Uart` is always 1.
        let xfer = unsafe { dmac::Transfer::new_unchecked(channel, buf, self, false) };
        xfer.with_waker(waker)
            .begin(C::Sercom::DMA_TX_TRIGGER, trigger_action)
    }
}

//=============================================================================
// SPI DMA transfers
//=============================================================================

unsafe impl<C, A> Buffer for Spi<C, A>
where
    C: spi::ValidConfig,
    C::OpMode: spi::MasterMode,
    C::Size: spi::AtomicSize<Word = C::Word>,
    C::Word: Beat,
    A: spi::Capability,
{
    type Beat = C::Word;

    #[inline]
    fn dma_ptr(&mut self) -> *mut Self::Beat {
        self.data_ptr()
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

impl<C, A> Spi<C, A>
where
    C: spi::ValidConfig,
    A: spi::Transmit,
    Self: Buffer<Beat = C::Word>,
{
    /// Transform an [`Spi`] into a DMA [`Transfer`]) and
    /// start a send transaction.
    #[inline]
    pub fn send_with_dma<Ch, B, W>(
        self,
        buf: B,
        mut channel: Ch,
        waker: W,
    ) -> Transfer<Channel<Ch::Id, Busy>, BufferPair<B, Self>, W>
    where
        Ch: AnyChannel<Status = Ready>,
        B: Buffer<Beat = C::Word> + 'static,
        W: FnOnce(CallbackStatus) + 'static,
    {
        channel
            .as_mut()
            .enable_interrupts(InterruptFlags::new().with_tcmpl(true));

        #[cfg(feature = "thumbv7")]
        let trigger_action = TriggerAction::BURST;

        #[cfg(feature = "thumbv6")]
        let trigger_action = TriggerAction::BEAT;

        // SAFETY: This is safe because the of the `'static` bound check
        // for `B`, and the fact that the buffer length of an `Spi` is always 1.
        let xfer = unsafe { Transfer::new_unchecked(channel, buf, self, false) };
        xfer.with_waker(waker)
            .begin(C::Sercom::DMA_TX_TRIGGER, trigger_action)
    }
}

impl<C, A> Spi<C, A>
where
    C: spi::ValidConfig,
    A: spi::Receive,
    Self: Buffer<Beat = C::Word>,
{
    /// Transform an [`Spi`] into a DMA [`Transfer`]) and
    /// start a receive transaction.
    #[inline]
    pub fn receive_with_dma<Ch, B, W>(
        self,
        buf: B,
        mut channel: Ch,
        waker: W,
    ) -> Transfer<Channel<Ch::Id, Busy>, BufferPair<Self, B>, W>
    where
        Ch: AnyChannel<Status = Ready>,
        B: Buffer<Beat = C::Word> + 'static,
        W: FnOnce(CallbackStatus) + 'static,
    {
        channel
            .as_mut()
            .enable_interrupts(InterruptFlags::new().with_tcmpl(true));

        #[cfg(feature = "thumbv7")]
        let trigger_action = TriggerAction::BURST;

        #[cfg(feature = "thumbv6")]
        let trigger_action = TriggerAction::BEAT;

        // SAFETY: This is safe because the of the `'static` bound check
        // for `B`, and the fact that the buffer length of an `Spi` is always 1.
        let xfer = unsafe { Transfer::new_unchecked(channel, self, buf, false) };
        xfer.with_waker(waker)
            .begin(C::Sercom::DMA_RX_TRIGGER, trigger_action)
    }
}
