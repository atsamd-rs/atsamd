//! `embedded-hal` and `embedded-io` implementations for DMA-enabled [`Spi`]s

use num_traits::{AsPrimitive, PrimInt};

use crate::dmac::{channel, sram::DmacDescriptor, AnyChannel, Beat, Buffer, Ready};
use crate::ehal::spi::SpiBus;
use crate::sercom::dma::{
    read_dma, read_dma_linked, write_dma, write_dma_linked, SercomPtr, SharedSliceBuffer,
    SinkSourceBuffer,
};

use super::{
    Capability, Config, DataWidth, Duplex, Error, MasterMode, OpMode, Receive, Sercom, Size, Slave,
    Spi, Transmit, ValidConfig, ValidPads, Word,
};

impl<P, M, Z, D, R, T> Spi<Config<P, M, Z>, D, R, T>
where
    P: ValidPads,
    M: OpMode,
    Z: Size,
    Config<P, M, Z>: ValidConfig,
    D: Capability,
    Z::Word: Beat,
{
    #[inline]
    pub(in super::super) fn sercom_ptr(&self) -> SercomPtr<Z::Word> {
        SercomPtr(self.config.regs.spi().data().as_ptr() as *mut _)
    }
}

// Write implementation is the same for Master and Slave SPIs.
impl<P, M, Z, D, R, T, S> Spi<Config<P, M, Z>, D, R, T>
where
    P: ValidPads,
    M: OpMode,
    Z: Size + 'static,
    Config<P, M, Z>: ValidConfig<Sercom = S>,
    D: Transmit,
    S: Sercom,
    Z::Word: PrimInt + AsPrimitive<DataWidth> + Beat,
    DataWidth: AsPrimitive<Z::Word>,
    T: AnyChannel<Status = Ready>,
{
    pub(super) fn write_dma(&mut self, buf: &[Z::Word]) -> Result<usize, Error> {
        if buf.is_empty() {
            return Ok(0);
        }

        // Ignore RX buffer overflows by disabling the receiver
        self.config.as_mut().regs.rx_disable();

        let sercom_ptr = self.sercom_ptr();
        let tx = self._tx_channel.as_mut();
        let mut words = crate::sercom::dma::SharedSliceBuffer::from_slice(buf);

        // SAFETY: We make sure that any DMA transfer is complete or stopped before
        // returning. The order of operations is important; the RX transfer
        // must be ready to receive before the TX transfer is initiated.
        unsafe {
            crate::sercom::dma::write_dma::<_, _, S>(tx, sercom_ptr, &mut words);
        }

        while !tx.xfer_complete() {
            core::hint::spin_loop();
        }

        // Defensively disable channels
        tx.stop();

        // Reenable receiver only if necessary
        if D::RX_ENABLE {
            self.config.as_mut().regs.rx_enable();
        }

        self._tx_channel.as_mut().xfer_success()?;
        Ok(buf.len())
    }
}

impl<P, M, S, C, D, R, T> Spi<Config<P, M, C>, D, R, T>
where
    Config<P, M, C>: ValidConfig<Sercom = S>,
    S: Sercom,
    P: ValidPads,
    M: MasterMode,
    C: Size + 'static,
    C::Word: PrimInt + AsPrimitive<DataWidth> + Beat,
    DataWidth: AsPrimitive<C::Word>,
    D: Capability,
    R: AnyChannel<Status = Ready>,
    T: AnyChannel<Status = Ready>,
{
    #[inline]
    fn transfer_blocking<Source: Buffer<Beat = C::Word>, Dest: Buffer<Beat = C::Word>>(
        &mut self,
        dest: &mut Dest,
        source: &mut Source,
    ) -> Result<(), Error> {
        let sercom_ptr = self.sercom_ptr();
        let rx = self._rx_channel.as_mut();
        let tx = self._tx_channel.as_mut();

        // SAFETY: We make sure that any DMA transfer is complete or stopped before
        // returning. The order of operations is important; the RX transfer
        // must be ready to receive before the TX transfer is initiated.
        unsafe {
            read_dma::<_, _, S>(rx, sercom_ptr.clone(), dest);
            write_dma::<_, _, S>(tx, sercom_ptr, source);
        }

        while !(rx.xfer_complete() && tx.xfer_complete()) {
            core::hint::spin_loop();
        }

        // Defensively disable channels
        tx.stop();
        rx.stop();

        // Check for overflows or DMA errors
        self.read_status().check_bus_error()?;
        self._rx_channel
            .as_mut()
            .xfer_success()
            .and(self._tx_channel.as_mut().xfer_success())?;
        Ok(())
    }

    #[inline]
    pub(super) fn read_dma_master(&mut self, mut words: &mut [C::Word]) -> Result<(), Error> {
        if words.is_empty() {
            return Ok(());
        }

        let mut source_word = self.config.nop_word.as_();
        let mut source = SinkSourceBuffer::new(&mut source_word, words.len());

        self.transfer_blocking(&mut words, &mut source)
    }
}

/// [`SpiBus`] implementation for [`Spi`], using DMA transfers.
impl<P, M, S, C, R, T> SpiBus<Word<C>> for Spi<Config<P, M, C>, Duplex, R, T>
where
    Config<P, M, C>: ValidConfig<Sercom = S>,
    S: Sercom,
    P: ValidPads,
    M: MasterMode,
    C: Size + 'static,
    C::Word: PrimInt + AsPrimitive<DataWidth> + Beat,
    DataWidth: AsPrimitive<C::Word>,
    R: AnyChannel<Status = Ready>,
    T: AnyChannel<Status = Ready>,
{
    #[inline]
    fn read(&mut self, words: &mut [C::Word]) -> Result<(), Self::Error> {
        self.read_dma_master(words)
    }

    #[inline]
    fn write(&mut self, words: &[C::Word]) -> Result<(), Self::Error> {
        self.write_dma(words)?;
        Ok(())
    }

    #[inline]
    fn transfer(&mut self, mut read: &mut [C::Word], write: &[C::Word]) -> Result<(), Self::Error> {
        use core::cmp::Ordering;

        // No work to do here
        if write.is_empty() && read.is_empty() {
            return Ok(());
        }

        // Handle 0-length special cases
        if write.is_empty() {
            return self.read_dma_master(read);
        } else if read.is_empty() {
            self.write_dma(write)?;
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
        let mut source_sink_word = self.config.nop_word.as_();
        let mut sercom_ptr = self.sercom_ptr();

        let (read_link, write_link) = match read.len().cmp(&write.len()) {
            Ordering::Equal => {
                let mut write = SharedSliceBuffer::from_slice(write);
                return self.transfer_blocking(&mut read, &mut write);
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

        let rx = self._rx_channel.as_mut();
        let tx = self._tx_channel.as_mut();

        let mut write = SharedSliceBuffer::from_slice(write);

        // SAFETY: We make sure that any DMA transfer is complete or stopped before
        // returning. The order of operations is important; the RX transfer
        // must be ready to receive before the TX transfer is initiated.
        unsafe {
            read_dma_linked::<_, _, S>(rx, sercom_ptr.clone(), &mut read, read_link);
            write_dma_linked::<_, _, S>(tx, sercom_ptr, &mut write, write_link);
        }

        while !(rx.xfer_complete() && tx.xfer_complete()) {
            core::hint::spin_loop();
        }

        // Defensively disable channels
        tx.stop();
        rx.stop();

        // Check for overflows or DMA errors
        self.read_status().check_bus_error()?;
        self._rx_channel
            .as_mut()
            .xfer_success()
            .and(self._tx_channel.as_mut().xfer_success())?;
        Ok(())
    }

    #[inline]
    fn transfer_in_place(&mut self, words: &mut [C::Word]) -> Result<(), Self::Error> {
        // Safety: Aliasing the buffer is only safe because the DMA read will always be
        // lagging one word behind the write, so they don't overlap on the same memory.
        // It's preferable to use two `SharedSliceBuffer`s here; using the `words` slice
        // directly as a buffer could potentially cause UB issues if not careful when
        // aliasing, as it could be easy to create two `&mut` references pointing to the
        // same buffer. `read_buf` and `write_buf` may only be read/written to by the
        // DMAC, otherwise an `UnsafeCell` would be necessary.
        unsafe {
            let mut read_buf = SharedSliceBuffer::from_slice_unchecked(words);
            let mut write_buf = SharedSliceBuffer::from_slice(words);
            self.transfer_blocking(&mut read_buf, &mut write_buf)
        }
    }

    #[inline]
    fn flush(&mut self) -> Result<(), Error> {
        self.flush_tx();
        Ok(())
    }
}

/// [`embedded_io::Write`] implementation for [`Transmit`] [`Spi`]s in either
/// [`Slave`] or [`MasterMode`], using DMA transfers.
impl<P, M, Z, D, R, T, S> embedded_io::Write for Spi<Config<P, M, Z>, D, R, T>
where
    P: ValidPads,
    M: OpMode,
    Z: Size<Word = u8> + 'static,
    Config<P, M, Z>: ValidConfig<Sercom = S>,
    D: Transmit,
    S: Sercom,
    T: AnyChannel<Status = Ready>,
{
    fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error> {
        Spi::write_dma(self, buf)
    }

    fn flush(&mut self) -> Result<(), Self::Error> {
        self.flush_tx();
        Ok(())
    }
}

/// [`embedded_io::Read`] implementation for [`Receive`] [`Spi`]s in
/// [`MasterMode`], using DMA transfers.
impl<P, M, Z, D, R, T, S> embedded_io::Read for Spi<Config<P, M, Z>, D, R, T>
where
    P: ValidPads,
    M: MasterMode,
    Z: Size<Word = u8> + 'static,
    Config<P, M, Z>: ValidConfig<Sercom = S>,
    D: Receive,
    S: Sercom,
    R: AnyChannel<Status = Ready>,
    T: AnyChannel<Status = Ready>,
{
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
        self.read_dma_master(buf)?;
        Ok(buf.len())
    }
}

/// [`embedded_io::Read`] implementation for [`Receive`] [`Spi`]s in [`Slave`]
/// mode, using DMA transfers.
impl<P, Z, D, R, T, S> embedded_io::Read for Spi<Config<P, Slave, Z>, D, R, T>
where
    P: ValidPads,
    Z: Size<Word = u8> + 'static,
    Config<P, Slave, Z>: ValidConfig<Sercom = S>,
    D: Receive,
    S: Sercom,
    R: AnyChannel<Status = Ready>,
{
    fn read(&mut self, mut buf: &mut [u8]) -> Result<usize, Self::Error> {
        if buf.is_empty() {
            return Ok(0);
        }

        // In Slave mode, RX words can come in even if we haven't sent anything. This
        // means some words can arrive asynchronously while we weren't looking (similar
        // to UART RX). We need to check if we haven't missed any.
        self.flush_rx()?;
        let sercom_ptr = self.sercom_ptr();
        let rx = self._rx_channel.as_mut();

        // SAFETY: We make sure that any DMA transfer is complete or stopped before
        // returning.
        unsafe {
            read_dma::<_, _, S>(rx, sercom_ptr.clone(), &mut buf);
        }

        while !(rx.xfer_complete()) {
            core::hint::spin_loop();
        }

        // Defensively disable channel
        rx.stop();

        // Check for overflows or DMA errors
        self.read_status().check_bus_error()?;
        self._rx_channel.as_mut().xfer_success()?;
        Ok(buf.len())
    }
}
