use embedded_hal_async::spi::SpiBus;
use num_traits::{AsPrimitive, PrimInt};

use super::SpiFuture;
use crate::{
    dmac::{
        channel::{self, Channel},
        sram::DmacDescriptor,
        AnyChannel, Beat, Buffer, ReadyFuture,
    },
    sercom::{
        dma::{
            async_dma::{self, read_dma, read_dma_linked, write_dma, write_dma_linked},
            SharedSliceBuffer, SinkSourceBuffer,
        },
        spi::{
            Capability, Config, DataWidth, Duplex, Error, MasterMode, OpMode, Receive, Rx, Size,
            Slave, Spi, Transmit, Tx, ValidConfig, ValidPads, Word,
        },
        Sercom,
    },
    typelevel::NoneT,
};

/// Convenience type for a [`SpiFuture`] with RX and TX capabilities in DMA
/// mode.
///
/// The type parameter `R` represents the RX DMA channel ID (`ChX`), and
/// `T` represents the TX DMA channel ID.
pub type SpiFutureDuplexDma<C, R, T> =
    SpiFuture<C, Duplex, Channel<R, ReadyFuture>, Channel<T, ReadyFuture>>;

/// Convenience type for a [`SpiFuture`] with RX capabilities in DMA mode.
///
/// The type parameter `R` represents the RX DMA channel ID (`ChX`).
pub type SpiFutureRxDma<C, R> = SpiFuture<C, Rx, Channel<R, ReadyFuture>, NoneT>;

/// Convenience type for a [`SpiFuture`] with TX capabilities in DMA mode.
///
/// The type parameter `T` represents the TX DMA channel ID (`ChX`).
pub type SpiFutureTxDma<C, T> = SpiFuture<C, Tx, NoneT, Channel<T, ReadyFuture>>;

impl<C, D, RxDma, TxDma> SpiFuture<C, D, RxDma, TxDma>
where
    C: ValidConfig,
    D: Capability,
    RxDma: AnyChannel<Status = ReadyFuture>,
    TxDma: AnyChannel<Status = ReadyFuture>,
{
    /// Reclaim the DMA channels. Any subsequent SPI transaction will not
    /// use DMA.
    pub fn take_dma_channels(self) -> (SpiFuture<C, D, NoneT, NoneT>, RxDma, TxDma) {
        let (spi, rx, tx) = self.spi.take_dma_channels();
        (SpiFuture { spi }, rx, tx)
    }
}

impl<C, D, R, T> SpiFuture<C, D, R, T>
where
    C: ValidConfig,
    D: Receive,
    R: AnyChannel<Status = ReadyFuture>,
{
    /// Reclaim the Rx DMA channel. Any subsequent SPI transaction will not
    /// use DMA.
    pub fn take_rx_channel(self) -> (SpiFuture<C, D, NoneT, T>, R) {
        let (spi, channel) = self.spi.take_rx_channel();
        (SpiFuture { spi }, channel)
    }
}

impl<C, D, R, T> SpiFuture<C, D, R, T>
where
    C: ValidConfig,
    D: Capability,
    T: AnyChannel<Status = ReadyFuture>,
{
    /// Reclaim the DMA channel. Any subsequent SPI transaction will not use
    /// DMA.
    pub fn take_tx_channel(self) -> (SpiFuture<C, D, R, NoneT>, T) {
        let (spi, channel) = self.spi.take_tx_channel();
        (SpiFuture { spi }, channel)
    }
}

impl<C, S, R> SpiFuture<C, Rx, R, NoneT>
where
    C: ValidConfig<Sercom = S>,
    C::Word: PrimInt + AsPrimitive<DataWidth>,
    DataWidth: AsPrimitive<C::Word>,
    S: Sercom,
{
    /// Add a DMA channel for receiving transactions
    #[inline]
    pub fn with_rx_dma_channel<Chan: AnyChannel<Status = ReadyFuture>>(
        self,
        rx_channel: Chan,
    ) -> SpiFuture<C, Rx, Chan, NoneT> {
        SpiFuture {
            spi: Spi {
                config: self.spi.config,
                capability: self.spi.capability,
                _rx_channel: rx_channel,
                _tx_channel: self.spi._tx_channel,
            },
        }
    }
}

impl<C, S, T> SpiFuture<C, Tx, NoneT, T>
where
    C: ValidConfig<Sercom = S>,
    C::Word: PrimInt + AsPrimitive<DataWidth>,
    DataWidth: AsPrimitive<C::Word>,
    S: Sercom,
{
    /// Add a DMA channel for receiving transactions
    #[inline]
    pub fn with_tx_dma_channel<Chan: AnyChannel<Status = ReadyFuture>>(
        self,
        tx_channel: Chan,
    ) -> SpiFuture<C, Tx, NoneT, Chan> {
        SpiFuture {
            spi: Spi {
                config: self.spi.config,
                capability: self.spi.capability,
                _rx_channel: self.spi._rx_channel,
                _tx_channel: tx_channel,
            },
        }
    }
}

impl<C, S, R, T> SpiFuture<C, Duplex, R, T>
where
    C: ValidConfig<Sercom = S>,
    C::Word: PrimInt + AsPrimitive<DataWidth>,
    DataWidth: AsPrimitive<C::Word>,
    S: Sercom,
{
    /// Add a DMA channel for receiving transactions
    #[inline]
    pub fn with_dma_channels<ChanRx, ChanTx>(
        self,
        rx_channel: ChanRx,
        tx_channel: ChanTx,
    ) -> SpiFuture<C, Duplex, ChanRx, ChanTx>
    where
        ChanRx: AnyChannel<Status = ReadyFuture>,
        ChanTx: AnyChannel<Status = ReadyFuture>,
    {
        SpiFuture {
            spi: Spi {
                config: self.spi.config,
                capability: self.spi.capability,
                _rx_channel: rx_channel,
                _tx_channel: tx_channel,
            },
        }
    }
}

// Write implementation is the same for Master and Slave SPIs.
impl<P, M, Z, D, R, T, S> SpiFuture<Config<P, M, Z>, D, R, T>
where
    P: ValidPads,
    M: OpMode,
    Z: Size + 'static,
    Config<P, M, Z>: ValidConfig<Sercom = S>,
    D: Transmit,
    S: Sercom,
    Z::Word: PrimInt + AsPrimitive<DataWidth> + Beat,
    DataWidth: AsPrimitive<Z::Word>,
    T: AnyChannel<Status = ReadyFuture>,
{
    /// Write words from a buffer asynchronously, using DMA.
    #[inline]
    pub async fn write_dma(&mut self, words: &[Z::Word]) -> Result<usize, Error> {
        if words.is_empty() {
            return Ok(0);
        }

        // Ignore RX buffer overflows by disabling the receiver
        self.spi.config.as_mut().regs.rx_disable();

        let sercom_ptr = self.spi.sercom_ptr();
        let tx = self.spi._tx_channel.as_mut();
        let mut buf = SharedSliceBuffer::from_slice(words);

        let tx_result = async_dma::write_dma::<_, _, S>(tx, sercom_ptr, &mut buf).await;

        // Reenable receiver only if necessary
        if D::RX_ENABLE {
            self.spi.config.as_mut().regs.rx_enable();
        }

        tx_result?;
        Ok(words.len())
    }
}

impl<P, M, S, C, D, R, T> SpiFuture<Config<P, M, C>, D, R, T>
where
    Config<P, M, C>: ValidConfig<Sercom = S>,
    S: Sercom,
    P: ValidPads,
    M: MasterMode,
    C: Size + 'static,
    C::Word: PrimInt + AsPrimitive<DataWidth> + Beat,
    D: Capability,
    DataWidth: AsPrimitive<C::Word>,
    R: AnyChannel<Status = ReadyFuture>,
    T: AnyChannel<Status = ReadyFuture>,
{
    #[inline]
    async fn transfer_blocking<Source: Buffer<Beat = C::Word>, Dest: Buffer<Beat = C::Word>>(
        &mut self,
        dest: &mut Dest,
        source: &mut Source,
    ) -> Result<(), Error> {
        let sercom_ptr = self.spi.sercom_ptr();
        let rx = self.spi._rx_channel.as_mut();
        let tx = self.spi._tx_channel.as_mut();

        let (rx_result, tx_result) = futures::join!(
            read_dma::<_, _, S>(rx, sercom_ptr.clone(), dest),
            write_dma::<_, _, S>(tx, sercom_ptr, source)
        );

        // Check for overflows or DMA errors
        self.spi.read_status().check_bus_error()?;
        rx_result.and(tx_result)?;
        Ok(())
    }

    /// Read words into a buffer asynchronously, using DMA.
    #[inline]
    pub async fn read_dma_master(&mut self, mut words: &mut [C::Word]) -> Result<(), Error> {
        if words.is_empty() {
            return Ok(());
        }

        let mut source_word = self.spi.config.nop_word.as_();
        let mut source = SinkSourceBuffer::new(&mut source_word, words.len());

        self.transfer_blocking(&mut words, &mut source).await
    }
}

/// [`SpiBus`] implementation for [`Spi`], using DMA transfers.
impl<P, M, C, S, R, T> SpiBus<Word<C>> for SpiFuture<Config<P, M, C>, Duplex, R, T>
where
    S: Sercom,
    Config<P, M, C>: ValidConfig<Sercom = S>,
    P: ValidPads,
    M: MasterMode,
    C: Size + 'static,
    C::Word: PrimInt + AsPrimitive<DataWidth> + Beat,
    DataWidth: AsPrimitive<C::Word>,
    R: AnyChannel<Status = ReadyFuture>,
    T: AnyChannel<Status = ReadyFuture>,
{
    #[inline]
    async fn read(&mut self, words: &mut [C::Word]) -> Result<(), Self::Error> {
        self.read_dma_master(words).await
    }

    #[inline]
    async fn write(&mut self, words: &[C::Word]) -> Result<(), Self::Error> {
        self.write_dma(words).await?;
        Ok(())
    }

    #[inline]
    async fn transfer(
        &mut self,
        mut read: &mut [C::Word],
        write: &[C::Word],
    ) -> Result<(), Self::Error> {
        use core::cmp::Ordering;

        // No work to do here
        if write.is_empty() && read.is_empty() {
            return Ok(());
        }

        // Handle 0-length special cases
        if write.is_empty() {
            return self.read_dma_master(read).await;
        } else if read.is_empty() {
            self.write_dma(write).await?;
            return Ok(());
        }

        // Reserve space for a DMAC SRAM descriptor if we need to make a linked
        // transfer. Must not be dropped until all transfers have completed
        // or have been stopped.
        let mut linked_descriptor = DmacDescriptor::default();

        // If read < write, the incoming words will be written to this memory location;
        // it will be discarded after. If read > write, all writes after the
        // buffer has been exhausted will write the nop word to "stimulate" the slave
        // into sending data. Must not be dropped until all transfers have
        // completed or have been stopped.
        let mut source_sink_word = self.spi.config.as_mut().nop_word.as_();
        let mut sercom_ptr = self.spi.sercom_ptr();

        let (read_link, write_link) = match read.len().cmp(&write.len()) {
            Ordering::Equal => {
                let mut write = SharedSliceBuffer::from_slice(write);
                return self.transfer_blocking(&mut read, &mut write).await;
            }

            // `read` is shorter; link transfer to sink incoming words after the buffer has been
            // filled.
            Ordering::Less => {
                let mut sink =
                    SinkSourceBuffer::new(&mut source_sink_word, write.len() - read.len());
                unsafe {
                    channel::write_descriptor(
                        &mut linked_descriptor,
                        &mut sercom_ptr,
                        &mut sink,
                        // Add a null descriptor pointer to end the transfer.
                        core::ptr::null_mut(),
                    );
                }

                (Some(&mut linked_descriptor), None)
            }

            // `write` is shorter; link transfer to send NOP word after the buffer has been
            // exhausted.
            Ordering::Greater => {
                let mut source =
                    SinkSourceBuffer::new(&mut source_sink_word, read.len() - write.len());
                unsafe {
                    channel::write_descriptor(
                        &mut linked_descriptor,
                        &mut source,
                        &mut sercom_ptr,
                        // Add a null descriptor pointer to end the transfer.
                        core::ptr::null_mut(),
                    );
                }

                (None, Some(&mut linked_descriptor))
            }
        };

        let rx = self.spi._rx_channel.as_mut();
        let tx = self.spi._tx_channel.as_mut();

        let mut write = SharedSliceBuffer::from_slice(write);

        // SAFETY: We make sure that any DMA transfer is complete or stopped before
        // returning. The order of operations is important; the RX transfer
        // must be ready to receive before the TX transfer is initiated.
        let (rx_result, tx_result) = unsafe {
            futures::join!(
                read_dma_linked::<_, _, S>(rx, sercom_ptr.clone(), &mut read, read_link),
                write_dma_linked::<_, _, S>(tx, sercom_ptr, &mut write, write_link)
            )
        };

        // Check for overflows or DMA errors
        self.spi.read_status().check_bus_error()?;
        rx_result.and(tx_result)?;
        Ok(())
    }

    #[inline]
    async fn transfer_in_place(&mut self, words: &mut [C::Word]) -> Result<(), Self::Error> {
        // Safefy: Aliasing the buffer is only safe because the DMA read will always be
        // lagging one word behind the write, so they don't overlap on the same memory.
        // It's preferable to use two `SharedSliceBuffer`s here; using the `words` slice
        // directly as a buffer could potentially cause UB issues if not careful when
        // aliasing, as it could be easy to create two `&mut` references pointing to the
        // same buffer. `read_buf` and `write_buf` may only be read/written to by the
        // DMAC, otherwise an `UnsafeCell` would be necessary.
        unsafe {
            let mut read_buf = SharedSliceBuffer::from_slice_unchecked(words);
            let mut write_buf = SharedSliceBuffer::from_slice(words);
            self.transfer_blocking(&mut read_buf, &mut write_buf).await
        }
    }

    #[inline]
    async fn flush(&mut self) -> Result<(), Self::Error> {
        // Wait for all transactions to complete, ignoring buffer overflow errors.
        self.flush_tx().await;
        Ok(())
    }
}

/// [`embedded_io::Write`] implementation for [`Transmit`] [`SpiFuture`]s in
/// either [`Slave`] or [`MasterMode`], using DMA transfers.
impl<P, M, Z, D, R, T, S> embedded_io_async::Write for SpiFuture<Config<P, M, Z>, D, R, T>
where
    P: ValidPads,
    M: OpMode,
    Z: Size<Word = u8> + 'static,
    Config<P, M, Z>: ValidConfig<Sercom = S>,
    D: Transmit,
    S: Sercom,
    T: AnyChannel<Status = ReadyFuture>,
{
    async fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error> {
        SpiFuture::write_dma(self, buf).await
    }

    async fn flush(&mut self) -> Result<(), Self::Error> {
        self.flush_tx().await;
        Ok(())
    }
}

/// [`embedded_io::Read`] implementation for [`Receive`] [`SpiFuture`]s in
/// [`MasterMode`], using DMA transfers.
impl<P, M, Z, D, R, T, S> embedded_io_async::Read for SpiFuture<Config<P, M, Z>, D, R, T>
where
    P: ValidPads,
    M: MasterMode,
    Z: Size<Word = u8> + 'static,
    Config<P, M, Z>: ValidConfig<Sercom = S>,
    D: Receive,
    S: Sercom,
    R: AnyChannel<Status = ReadyFuture>,
    T: AnyChannel<Status = ReadyFuture>,
{
    async fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
        self.read_dma_master(buf).await?;
        Ok(buf.len())
    }
}

/// [`embedded_io::Read`] implementation for [`Receive`] [`SpiFuture`]s in
/// [`Slave`] mode, using DMA transfers.
impl<P, Z, D, R, T, S> embedded_io_async::Read for SpiFuture<Config<P, Slave, Z>, D, R, T>
where
    P: ValidPads,
    Z: Size<Word = u8> + 'static,
    Config<P, Slave, Z>: ValidConfig<Sercom = S>,
    D: Receive,
    S: Sercom,
    R: AnyChannel<Status = ReadyFuture>,
{
    async fn read(&mut self, mut buf: &mut [u8]) -> Result<usize, Self::Error> {
        if buf.is_empty() {
            return Ok(0);
        }

        // In Slave mode, RX words can come in even if we haven't sent anything. This
        // means some words can arrive asynchronously while we weren't looking (similar
        // to UART RX). We need to check if we haven't missed any.
        self.flush_rx().await?;
        let sercom_ptr = self.spi.sercom_ptr();
        let rx = self.spi._rx_channel.as_mut();

        // SAFETY: We make sure that any DMA transfer is complete or stopped before
        // returning.
        let result = read_dma::<_, _, S>(rx, sercom_ptr.clone(), &mut buf).await;

        // Check for overflows or DMA errors
        self.flush_rx().await?;
        result?;
        Ok(buf.len())
    }
}
