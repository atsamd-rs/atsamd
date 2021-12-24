//! Use the DMA Controller to perform transfers using the SERCOM peripheral
//!
//! See the [`mod@uart`] and [`mod@spi`] modules for the corresponding DMA
//! transfer implementations.

use crate::{
    dmac::{
        self,
        channel::{AnyChannel, Busy, CallbackStatus, Channel, InterruptFlags, Ready},
        transfer::BufferPair,
        Beat, Buffer, Transfer, TriggerAction,
    },
    sercom::v2::{
        spi::{self, Spi},
        uart::{self, Uart},
        Sercom,
    },
};

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

        #[cfg(feature = "min-samd51g")]
        let trigger_action = TriggerAction::BURST;

        #[cfg(any(feature = "samd11", feature = "samd20", feature = "samd21"))]
        let trigger_action = TriggerAction::BEAT;

        // SAFETY: We use new_unchecked to avoid having to pass a 'static self as the
        // destination buffer. This is safe as long as we guarantee the destination
        // buffer is static.
        unsafe { dmac::Transfer::new_unchecked(channel, self, buf, false) }
            .with_waker(waker)
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

        #[cfg(feature = "min-samd51g")]
        let trigger_action = TriggerAction::BURST;

        #[cfg(any(feature = "samd11", feature = "samd20", feature = "samd21"))]
        let trigger_action = TriggerAction::BEAT;

        // SAFETY: We use new_unchecked to avoid having to pass a 'static self as the
        // destination buffer. This is safe as long as we guarantee the source buffer is
        // static.
        unsafe { dmac::Transfer::new_unchecked(channel, buf, self, false) }
            .with_waker(waker)
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

        #[cfg(feature = "min-samd51g")]
        let trigger_action = TriggerAction::BURST;

        #[cfg(any(feature = "samd11", feature = "samd20", feature = "samd21"))]
        let trigger_action = TriggerAction::BEAT;

        // SAFETY: We use new_unchecked to avoid having to pass a 'static self as the
        // destination buffer. This is safe as long as we guarantee the source buffer is
        // static.
        unsafe { Transfer::new_unchecked(channel, buf, self, false) }
            .with_waker(waker)
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

        #[cfg(feature = "min-samd51g")]
        let trigger_action = TriggerAction::BURST;

        #[cfg(any(feature = "samd11", feature = "samd20", feature = "samd21"))]
        let trigger_action = TriggerAction::BEAT;

        // SAFETY: We use new_unchecked to avoid having to pass a 'static self as the
        // destination buffer. This is safe as long as we guarantee the destination
        // buffer is static.
        unsafe { Transfer::new_unchecked(channel, self, buf, false) }
            .with_waker(waker)
            .begin(C::Sercom::DMA_RX_TRIGGER, trigger_action)
    }
}
