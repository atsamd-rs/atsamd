//! Use the DMA Controller to perform transfers using the SERCOM peripheral
//!
//! See the [`mod@uart`], [`mod@i2c`] and [`mod@spi`] modules for the
//! corresponding DMA transfer implementations.

use core::{marker::PhantomData, ops::Range};

use atsamd_hal_macros::hal_macro_helper;

use crate::{
    dmac::{
        self,
        channel::{AnyChannel, Busy, Channel, InterruptFlags, Ready},
        sram::DmacDescriptor,
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

/// Wrapper type over an `&[T]` that can be used as a source buffer for DMA
/// transfers. This is an implementation detail to make SERCOM-DMA
/// transfers work. Should not be used outside of this crate.
///
/// # Safety
///
/// [`SharedSliceBuffer`]s should only ever be used as **source** buffers for
/// DMA transfers, and never as destination buffers.
#[doc(hidden)]
pub(crate) struct SharedSliceBuffer<'a, T: Beat> {
    ptrs: Range<*mut T>,
    _lifetime: PhantomData<&'a T>,
}

impl<'a, T: Beat> SharedSliceBuffer<'a, T> {
    #[inline]
    pub(in super::super) fn from_slice(slice: &'a [T]) -> Self {
        unsafe { Self::from_slice_unchecked(slice) }
    }

    #[inline]
    pub(in super::super) unsafe fn from_slice_unchecked(slice: &[T]) -> Self {
        let ptrs = slice.as_ptr_range();

        let ptrs = Range {
            start: ptrs.start.cast_mut(),
            end: ptrs.end.cast_mut(),
        };

        Self {
            ptrs,
            _lifetime: PhantomData,
        }
    }
}

unsafe impl<T: Beat> Buffer for SharedSliceBuffer<'_, T> {
    type Beat = T;
    #[inline]
    fn dma_ptr(&mut self) -> *mut Self::Beat {
        if self.incrementing() {
            self.ptrs.end
        } else {
            self.ptrs.start
        }
    }

    #[inline]
    fn incrementing(&self) -> bool {
        self.buffer_len() > 1
    }

    #[inline]
    fn buffer_len(&self) -> usize {
        self.ptrs.end as usize - self.ptrs.start as usize
    }
}

/// Sink/source buffer to use for unidirectional SPI-DMA transfers.
///
/// When reading/writing from a [`Duplex`] [`Spi`] with DMA enabled,
/// we must always read and write the same number of words, regardless of
/// whether we care about the result (ie, for write, we discard the read
/// words, whereas for read, we must send a no-op word).
///
/// This [`Buffer`] implementation provides a source/sink for a single word,
/// but with a variable length.
pub(super) struct SinkSourceBuffer<'a, T: Beat> {
    word: &'a mut T,
    length: usize,
}

impl<'a, T: Beat> SinkSourceBuffer<'a, T> {
    pub(super) fn new(word: &'a mut T, length: usize) -> Self {
        Self { word, length }
    }
}
unsafe impl<T: Beat> Buffer for SinkSourceBuffer<'_, T> {
    type Beat = T;
    #[inline]
    fn incrementing(&self) -> bool {
        false
    }

    #[inline]
    fn buffer_len(&self) -> usize {
        self.length
    }

    #[inline]
    fn dma_ptr(&mut self) -> *mut Self::Beat {
        self.word as *mut _
    }
}
/// Wrapper type over Sercom instances to get around lifetime issues when using
/// one as a DMA source/destination buffer. This is an implementation detail to
/// make SERCOM-DMA transfers work.
#[doc(hidden)]
#[derive(Clone)]
pub(crate) struct SercomPtr<T: Beat>(pub(in super::super) *mut T);

unsafe impl<T: Beat> Buffer for SercomPtr<T> {
    type Beat = T;

    #[inline]
    fn dma_ptr(&mut self) -> *mut Self::Beat {
        self.0
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

//=============================================================================
// I2C DMA transfers
//=============================================================================

/// Token type representing an [`I2c`] for which the bus is
/// ready to start a transaction.
///
/// For use with [`send_with_dma`](super::i2c::I2c::send_with_dma) and
/// [`receive_with_dma`](super::i2c::I2c::send_with_dma).
#[deprecated(
    since = "0.19.0",
    note = "Use `I2c::with_dma_channel` instead. You will have access to DMA-enabled `embedded-hal` implementations."
)]
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
    /// i2c.send_with_dma(ADDRESS, token, buf_src, chan0);
    /// # }
    /// ```
    ///
    /// [`init_dma_transfer`]: super::i2c::I2c::init_dma_transfer
    /// [`send_with_dma`]: super::i2c::I2c::send_with_dma
    /// [`receive_with_dma`]: super::i2c::I2c::receive_with_dma
    #[deprecated(
        since = "0.19.0",
        note = "Use `I2c::with_dma_channel` instead. You will have access to DMA-enabled `embedded-hal` implementations."
    )]
    #[allow(deprecated)]
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
    #[deprecated(
        since = "0.19.0",
        note = "Use `I2c::with_dma_channel` instead. You will have access to DMA-enabled `embedded-hal` implementations."
    )]
    #[allow(deprecated)]
    #[hal_macro_helper]
    pub fn receive_with_dma<Ch, B>(
        self,
        address: u8,
        _ready_token: I2cBusReady,
        buf: B,
        mut channel: Ch,
    ) -> Transfer<Channel<Ch::Id, Busy>, BufferPair<Self, B>>
    where
        Ch: AnyChannel<Status = Ready>,
        B: Buffer<Beat = i2c::Word> + 'static,
    {
        let len = buf.buffer_len();
        assert!(len > 0 && len <= 255);

        channel
            .as_mut()
            .enable_interrupts(InterruptFlags::new().with_tcmpl(true));

        #[hal_cfg("sercom0-d5x")]
        let trigger_action = TriggerAction::Burst;

        #[hal_cfg(any("sercom0-d11", "sercom0-d21"))]
        let trigger_action = TriggerAction::Beat;

        // SAFETY: This is safe because the of the `'static` bound check
        // for `B`, and the fact that the buffer length of an `I2c` is always 1.
        let xfer = unsafe { dmac::Transfer::new_unchecked(channel, self, buf, false) };
        let mut xfer = xfer.begin(C::Sercom::DMA_RX_TRIGGER, trigger_action);

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
    #[hal_macro_helper]
    #[deprecated(
        since = "0.19.0",
        note = "Use `I2c::with_dma_chahnnel` instead. You will have access to DMA-enabled `embedded-hal` implementations."
    )]
    #[allow(deprecated)]
    pub fn send_with_dma<Ch, B>(
        self,
        address: u8,
        _ready_token: I2cBusReady,
        buf: B,
        mut channel: Ch,
    ) -> Transfer<Channel<Ch::Id, Busy>, BufferPair<B, Self>>
    where
        Ch: AnyChannel<Status = Ready>,
        B: Buffer<Beat = i2c::Word> + 'static,
    {
        let len = buf.buffer_len();
        assert!(len > 0 && len <= 255);

        channel
            .as_mut()
            .enable_interrupts(InterruptFlags::new().with_tcmpl(true));

        #[hal_cfg("sercom0-d5x")]
        let trigger_action = TriggerAction::Burst;

        #[hal_cfg(any("sercom0-d11", "sercom0-d21"))]
        let trigger_action = TriggerAction::Beat;

        // SAFETY: This is safe because the of the `'static` bound check
        // for `B`, and the fact that the buffer length of an `I2c` is always 1.
        let xfer = unsafe { dmac::Transfer::new_unchecked(channel, buf, self, false) };
        let mut xfer = xfer.begin(C::Sercom::DMA_TX_TRIGGER, trigger_action);

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
    /// Transform an [`Uart`] into a DMA [`Transfer`]) and start reveiving into
    /// the provided buffer.
    ///
    /// In order to be (safely) non-blocking, his method has to take a `'static`
    /// buffer. If you'd rather use DMA with the blocking
    /// [`embedded_io::Read`](crate::embedded_io::Read) trait, and avoid having
    /// to use static buffers,
    /// use[`Uart::with_rx_channel`](Self::with_tx_channel) instead.
    #[inline]
    #[hal_macro_helper]
    pub fn receive_with_dma<Ch, B>(
        self,
        buf: B,
        mut channel: Ch,
    ) -> Transfer<Channel<Ch::Id, Busy>, BufferPair<Self, B>>
    where
        Ch: AnyChannel<Status = Ready>,
        B: Buffer<Beat = C::Word> + 'static,
    {
        channel
            .as_mut()
            .enable_interrupts(InterruptFlags::new().with_tcmpl(true));

        #[hal_cfg("sercom0-d5x")]
        let trigger_action = TriggerAction::Burst;

        #[hal_cfg(any("sercom0-d11", "sercom0-d21"))]
        let trigger_action = TriggerAction::Beat;

        // SAFETY: This is safe because the of the `'static` bound check
        // for `B`, and the fact that the buffer length of an `Uart` is always 1.
        let xfer = unsafe { dmac::Transfer::new_unchecked(channel, self, buf, false) };
        xfer.begin(C::Sercom::DMA_RX_TRIGGER, trigger_action)
    }
}

impl<C, D> Uart<C, D>
where
    Self: Buffer<Beat = C::Word>,
    C: uart::ValidConfig,
    D: uart::Transmit,
{
    /// Transform an [`Uart`] into a DMA [`Transfer`]) and start sending the
    /// provided buffer.
    ///
    /// In order to be (safely) non-blocking, his method takes a `'static`
    /// buffer. If you'd rather use DMA with the blocking
    /// [`embedded_io::Write`](crate::embedded_io::Write) trait, and avoid
    /// having to use static buffers,
    /// use[`Uart::with_tx_channel`](Self::with_tx_channel) instead.
    #[inline]
    #[hal_macro_helper]
    pub fn send_with_dma<Ch, B>(
        self,
        buf: B,
        mut channel: Ch,
    ) -> Transfer<Channel<Ch::Id, Busy>, BufferPair<B, Self>>
    where
        Ch: AnyChannel<Status = Ready>,
        B: Buffer<Beat = C::Word> + 'static,
    {
        channel
            .as_mut()
            .enable_interrupts(InterruptFlags::new().with_tcmpl(true));

        #[hal_cfg("sercom0-d5x")]
        let trigger_action = TriggerAction::Burst;

        #[hal_cfg(any("sercom0-d11", "sercom0-d21"))]
        let trigger_action = TriggerAction::Beat;

        // SAFETY: This is safe because the of the `'static` bound check
        // for `B`, and the fact that the buffer length of an `Uart` is always 1.
        let xfer = unsafe { dmac::Transfer::new_unchecked(channel, buf, self, false) };
        xfer.begin(C::Sercom::DMA_TX_TRIGGER, trigger_action)
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
    #[hal_macro_helper]
    #[deprecated(
        since = "0.19.0",
        note = "Use `Spi::with_dma_channels` instead. You will have access to DMA-enabled `embedded-hal` implementations."
    )]
    pub fn send_with_dma<Ch, B>(
        self,
        buf: B,
        mut channel: Ch,
    ) -> Transfer<Channel<Ch::Id, Busy>, BufferPair<B, Self>>
    where
        Ch: AnyChannel<Status = Ready>,
        B: Buffer<Beat = C::Word> + 'static,
    {
        channel
            .as_mut()
            .enable_interrupts(InterruptFlags::new().with_tcmpl(true));

        #[hal_cfg("sercom0-d5x")]
        let trigger_action = TriggerAction::Burst;

        #[hal_cfg(any("sercom0-d11", "sercom0-d21"))]
        let trigger_action = TriggerAction::Beat;

        // SAFETY: This is safe because the of the `'static` bound check
        // for `B`, and the fact that the buffer length of an `Spi` is always 1.
        let xfer = unsafe { Transfer::new_unchecked(channel, buf, self, false) };
        xfer.begin(C::Sercom::DMA_TX_TRIGGER, trigger_action)
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
    #[hal_macro_helper]
    #[deprecated(
        since = "0.19.0",
        note = "Use `Spi::with_dma_channels` instead. You will have access to DMA-enabled `embedded-hal` implementations."
    )]
    pub fn receive_with_dma<Ch, B>(
        self,
        buf: B,
        mut channel: Ch,
    ) -> Transfer<Channel<Ch::Id, Busy>, BufferPair<Self, B>>
    where
        Ch: AnyChannel<Status = Ready>,
        B: Buffer<Beat = C::Word> + 'static,
    {
        channel
            .as_mut()
            .enable_interrupts(InterruptFlags::new().with_tcmpl(true));

        #[hal_cfg("sercom0-d5x")]
        let trigger_action = TriggerAction::Burst;

        #[hal_cfg(any("sercom0-d11", "sercom0-d21"))]
        let trigger_action = TriggerAction::Beat;

        // SAFETY: This is safe because the of the `'static` bound check
        // for `B`, and the fact that the buffer length of an `Spi` is always 1.
        let xfer = unsafe { Transfer::new_unchecked(channel, self, buf, false) };
        xfer.begin(C::Sercom::DMA_RX_TRIGGER, trigger_action)
    }
}

/// Perform a SERCOM DMA read with a provided [`Buffer`]
///
/// # Safety
///
/// You **must** guarantee that the DMA transfer is either stopped or completed
/// before giving back control of `channel` AND `buf`.
#[hal_macro_helper]
pub(super) unsafe fn read_dma<T, B, S>(
    channel: &mut impl AnyChannel<Status = Ready>,
    sercom_ptr: SercomPtr<T>,
    buf: &mut B,
) where
    T: Beat,
    B: Buffer<Beat = T>,
    S: Sercom,
{
    read_dma_linked::<_, _, S>(channel, sercom_ptr, buf, None);
}

/// Perform a SERCOM DMA read with a provided [`Buffer`], and add an optional
/// link to a next [`DmacDescriptor`] to support linked transfers.
///
/// # Safety
///
/// You **must** guarantee that the DMA transfer is either stopped or completed
/// before giving back control of `channel` AND `buf`.
#[hal_macro_helper]
pub(super) unsafe fn read_dma_linked<T, B, S>(
    channel: &mut impl AnyChannel<Status = Ready>,
    mut sercom_ptr: SercomPtr<T>,
    buf: &mut B,
    next: Option<&mut DmacDescriptor>,
) where
    T: Beat,
    B: Buffer<Beat = T>,
    S: Sercom,
{
    #[hal_cfg("dmac-d5x")]
    let trigger_action = TriggerAction::Burst;

    #[hal_cfg(any("dmac-d11", "dmac-d21"))]
    let trigger_action = TriggerAction::Beat;

    // Safety: It is safe to bypass the buffer length check because `SercomPtr`
    // always has a buffer length of 1.
    channel.as_mut().transfer_unchecked(
        &mut sercom_ptr,
        buf,
        S::DMA_RX_TRIGGER,
        trigger_action,
        next,
    );
}

/// Perform a SERCOM DMA write with a provided [`Buffer`]
///
/// # Safety
///
/// You **must** guarantee that the DMA transfer is either stopped or completed
/// before giving back control of `channel` AND `buf`.
#[hal_macro_helper]
pub(super) unsafe fn write_dma<T, B, S>(
    channel: &mut impl AnyChannel<Status = Ready>,
    sercom_ptr: SercomPtr<T>,
    buf: &mut B,
) where
    T: Beat,
    B: Buffer<Beat = T>,
    S: Sercom,
{
    write_dma_linked::<_, _, S>(channel, sercom_ptr, buf, None);
}

/// Perform a SERCOM DMA write with a provided [`Buffer`], and add an optional
/// link to a next [`DmacDescriptor`] to support linked transfers.
///
/// # Safety
///
/// You **must** guarantee that the DMA transfer is either stopped or completed
/// before giving back control of `channel` AND `buf`.
#[hal_macro_helper]
pub(super) unsafe fn write_dma_linked<T, B, S>(
    channel: &mut impl AnyChannel<Status = Ready>,
    mut sercom_ptr: SercomPtr<T>,
    buf: &mut B,
    next: Option<&mut DmacDescriptor>,
) where
    T: Beat,
    B: Buffer<Beat = T>,
    S: Sercom,
{
    #[hal_cfg("dmac-d5x")]
    let trigger_action = TriggerAction::Burst;

    #[hal_cfg(any("dmac-d11", "dmac-d21"))]
    let trigger_action = TriggerAction::Beat;

    // Safety: It is safe to bypass the buffer length check because `SercomPtr`
    // always has a buffer length of 1.
    channel.as_mut().transfer_unchecked(
        buf,
        &mut sercom_ptr,
        S::DMA_TX_TRIGGER,
        trigger_action,
        next,
    );
}

/// Use the DMA Controller to perform async transfers using the SERCOM
/// peripheral
///
/// See the [`mod@uart`], [`mod@i2c`] and [`mod@spi`] modules for the
/// corresponding DMA transfer implementations.
#[cfg(feature = "async")]
pub(crate) mod async_dma {
    use dmac::{Error, ReadyFuture};

    use super::*;

    /// Perform a SERCOM DMA read with a provided `&mut [T]`
    #[inline]
    pub(in super::super) async fn read_dma<T, B, S>(
        channel: &mut impl AnyChannel<Status = ReadyFuture>,
        sercom_ptr: SercomPtr<T>,
        buf: &mut B,
    ) -> Result<(), Error>
    where
        B: Buffer<Beat = T>,
        T: Beat,
        S: Sercom,
    {
        unsafe { read_dma_linked::<_, _, S>(channel, sercom_ptr, buf, None).await }
    }

    /// Perform a SERCOM DMA read with a provided [`Buffer`], and add an
    /// optional link to a next [`DmacDescriptor`] to support linked
    /// transfers.
    ///
    /// # Safety
    ///
    /// You **must** guarantee that the DMA transfer is either stopped or
    /// completed before giving back control of `channel` AND `buf`.
    #[inline]
    #[hal_macro_helper]
    pub(in super::super) async unsafe fn read_dma_linked<T, B, S>(
        channel: &mut impl AnyChannel<Status = ReadyFuture>,
        mut sercom_ptr: SercomPtr<T>,
        buf: &mut B,
        next: Option<&mut DmacDescriptor>,
    ) -> Result<(), Error>
    where
        T: Beat,
        B: Buffer<Beat = T>,
        S: Sercom,
    {
        #[hal_cfg("dmac-d5x")]
        let trigger_action = TriggerAction::Burst;

        #[hal_cfg(any("dmac-d11", "dmac-d21"))]
        let trigger_action = TriggerAction::Beat;

        // Safety: It is safe to bypass the buffer length check because `SercomPtr`
        // always has a buffer length of 1.
        channel
            .as_mut()
            .transfer_future_linked(
                &mut sercom_ptr,
                buf,
                S::DMA_RX_TRIGGER,
                trigger_action,
                next,
            )
            .await
    }

    /// Perform a SERCOM DMA write with a provided `&[T]`
    #[inline]
    pub(in super::super) async fn write_dma<T, B, S>(
        channel: &mut impl AnyChannel<Status = ReadyFuture>,
        sercom_ptr: SercomPtr<T>,
        buf: &mut B,
    ) -> Result<(), Error>
    where
        B: Buffer<Beat = T>,
        T: Beat,
        S: Sercom,
    {
        unsafe { write_dma_linked::<_, _, S>(channel, sercom_ptr, buf, None).await }
    }

    /// Perform a SERCOM DMA write with a provided [`Buffer`], and add an
    /// optional link to a next [`DmacDescriptor`] to support linked
    /// transfers.
    ///
    /// # Safety
    ///
    /// You **must** guarantee that the DMA transfer is either stopped or
    /// completed before giving back control of `channel` AND `buf`.
    #[inline]
    #[hal_macro_helper]
    pub(in super::super) async unsafe fn write_dma_linked<T, B, S>(
        channel: &mut impl AnyChannel<Status = ReadyFuture>,
        mut sercom_ptr: SercomPtr<T>,
        buf: &mut B,
        next: Option<&mut DmacDescriptor>,
    ) -> Result<(), Error>
    where
        B: Buffer<Beat = T>,
        T: Beat,
        S: Sercom,
    {
        #[hal_cfg("dmac-d5x")]
        let trigger_action = TriggerAction::Burst;

        #[hal_cfg(any("dmac-d11", "dmac-d21"))]
        let trigger_action = TriggerAction::Beat;

        // Safety: It is safe to bypass the buffer length check because `SercomPtr`
        // always has a buffer length of 1.
        channel
            .as_mut()
            .transfer_future_linked(
                buf,
                &mut sercom_ptr,
                S::DMA_TX_TRIGGER,
                trigger_action,
                next,
            )
            .await
    }
}
