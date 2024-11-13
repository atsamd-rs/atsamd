use crate::{
    async_hal::interrupts::{Binding, Handler, InterruptSource},
    sercom::{
        i2c::{self, impl_ehal::chunk_operations, AnyConfig, Flags, I2c},
        Sercom,
    },
    typelevel::NoneT,
};
use core::{marker::PhantomData, task::Poll};

use embedded_hal_async::i2c::{ErrorType, I2c as I2cTrait, Operation};

/// Interrupt handler for async I2C operarions
pub struct InterruptHandler<S: Sercom> {
    _private: (),
    _sercom: PhantomData<S>,
}

impl<S: Sercom> crate::typelevel::Sealed for InterruptHandler<S> {}

impl<S: Sercom> Handler<S::Interrupt> for InterruptHandler<S> {
    // TODO the ISR gets called twice on every MB request???
    #[inline]
    unsafe fn on_interrupt() {
        let mut peripherals = unsafe { crate::pac::Peripherals::steal() };
        let i2cm = S::reg_block(&mut peripherals).i2cm();
        let flags_to_check = Flags::all();
        let flags_pending = Flags::from_bits_truncate(i2cm.intflag().read().bits());

        // Disable interrupts, but don't clear the flags. The future will take care of
        // clearing flags and re-enabling interrupts when woken.
        if flags_to_check.intersects(flags_pending) {
            i2cm.intenclr()
                .write(|w| unsafe { w.bits(flags_pending.bits()) });
            S::rx_waker().wake();
        }
    }
}

impl<C, S> I2c<C>
where
    C: AnyConfig<Sercom = S>,
    S: Sercom,
{
    /// Turn an [`I2c`] into an [`I2cFuture`]
    #[inline]
    pub fn into_future<I>(self, _interrupts: I) -> I2cFuture<C>
    where
        I: Binding<S::Interrupt, InterruptHandler<S>>,
    {
        S::Interrupt::unpend();
        unsafe { S::Interrupt::enable() };

        I2cFuture { i2c: self }
    }
}

/// `async` version of [`I2c`].
///
/// Create this struct by calling [`I2c::into_future`](I2c::into_future).
pub struct I2cFuture<C, D = NoneT>
where
    C: AnyConfig,
{
    i2c: I2c<C, D>,
}

impl<C, S, D> I2cFuture<C, D>
where
    C: AnyConfig<Sercom = S>,
    S: Sercom,
{
    async fn wait_flags(&mut self, flags_to_wait: Flags) {
        core::future::poll_fn(|cx| {
            // Scope maybe_pending so we don't forget to re-poll the register later down.
            {
                let maybe_pending = self.i2c.config.as_ref().registers.read_flags();
                if flags_to_wait.intersects(maybe_pending) {
                    return Poll::Ready(());
                }
            }

            self.i2c.disable_interrupts(Flags::all());
            // By convention, I2C uses the sercom's RX waker.
            S::rx_waker().register(cx.waker());
            self.i2c.enable_interrupts(flags_to_wait);
            let maybe_pending = self.i2c.config.as_ref().registers.read_flags();

            if !flags_to_wait.intersects(maybe_pending) {
                Poll::Pending
            } else {
                Poll::Ready(())
            }
        })
        .await;
    }
}

impl<C, S> I2cFuture<C, NoneT>
where
    C: AnyConfig<Sercom = S>,
    S: Sercom,
{
    /// Return the underlying [`I2c`].
    pub fn free(self) -> I2c<C> {
        self.i2c
    }

    /// Use a DMA channel for reads/writes
    #[cfg(feature = "dma")]
    pub fn with_dma_channel<D: crate::dmac::AnyChannel<Status = crate::dmac::ReadyFuture>>(
        self,
        dma_channel: D,
    ) -> I2cFuture<C, D> {
        I2cFuture {
            i2c: I2c {
                config: self.i2c.config,
                _dma_channel: dma_channel,
            },
        }
    }

    async fn write_one(&mut self, byte: u8) -> Result<(), i2c::Error> {
        self.wait_flags(Flags::MB | Flags::ERROR).await;
        self.i2c.read_status().check_bus_error()?;
        self.i2c.config.as_mut().registers.write_one(byte);
        Ok(())
    }

    async fn read_one(&mut self) -> Result<u8, i2c::Error> {
        self.wait_flags(Flags::SB | Flags::ERROR).await;
        self.i2c.read_status().check_bus_error()?;
        Ok(self.i2c.config.as_mut().registers.read_one())
    }
}

impl<C: AnyConfig, D> AsRef<I2c<C, D>> for I2cFuture<C, D> {
    #[inline]
    fn as_ref(&self) -> &I2c<C, D> {
        &self.i2c
    }
}

impl<C: AnyConfig, D> AsMut<I2c<C, D>> for I2cFuture<C, D> {
    #[inline]
    fn as_mut(&mut self) -> &mut I2c<C, D> {
        &mut self.i2c
    }
}

impl<C, D> ErrorType for I2cFuture<C, D>
where
    C: AnyConfig,
{
    type Error = i2c::Error;
}

impl<C: AnyConfig> I2cFuture<C> {
    #[inline]
    async fn do_read(&mut self, addr: u8, buffer: &mut [u8]) -> Result<(), i2c::Error> {
        self.i2c.config.as_mut().registers.start_read(addr)?;

        // Some manual iterator gumph because we need to ack bytes after the first.
        let mut iter = buffer.iter_mut();
        *iter.next().expect("buffer len is at least 1") = self.read_one().await?;

        for byte in iter {
            // Ack the last byte so we can receive another one
            self.i2c.config.as_mut().registers.cmd_read();
            *byte = self.read_one().await?;
        }

        Ok(())
    }

    #[inline]
    async fn continue_read(&mut self, buffer: &mut [u8]) -> Result<(), i2c::Error> {
        for byte in buffer.iter_mut() {
            self.i2c.config.as_mut().registers.cmd_read();
            *byte = self.read_one().await?;
        }
        Ok(())
    }

    #[inline]
    async fn do_write(&mut self, address: u8, buffer: &[u8]) -> Result<(), i2c::Error> {
        self.i2c.config.as_mut().registers.start_write(address)?;

        for byte in buffer {
            self.write_one(*byte).await?;
        }
        Ok(())
    }

    #[inline]
    async fn continue_write(&mut self, buffer: &[u8]) -> Result<(), i2c::Error> {
        for byte in buffer {
            self.write_one(*byte).await?;
        }
        Ok(())
    }
}

impl<C: AnyConfig> I2cTrait for I2cFuture<C> {
    #[inline]
    async fn read(&mut self, address: u8, buffer: &mut [u8]) -> Result<(), Self::Error> {
        self.do_read(address, buffer).await?;
        self.i2c.config.as_mut().registers.cmd_stop();
        Ok(())
    }

    #[inline]
    async fn write(&mut self, address: u8, bytes: &[u8]) -> Result<(), Self::Error> {
        self.do_write(address, bytes).await?;
        self.i2c.cmd_stop();
        Ok(())
    }

    #[inline]
    async fn write_read(
        &mut self,
        address: u8,
        write_buf: &[u8],
        read_buf: &mut [u8],
    ) -> Result<(), Self::Error> {
        self.do_write(address, write_buf).await?;
        self.i2c.config.as_mut().registers.cmd_repeated_start();
        self.do_read(address, read_buf).await?;
        Ok(())
    }

    #[inline]
    async fn transaction(
        &mut self,
        address: u8,
        operations: &mut [Operation<'_>],
    ) -> Result<(), Self::Error> {
        let mut op_groups = chunk_operations(operations).peekable();

        while let Some(group) = op_groups.next() {
            let mut group = group.iter_mut();
            // Unwrapping is OK here because chunk_operations will never give us a 0-length
            // chunk.
            let op = group.next().unwrap();

            // First operation in the group - send a START with the address, and the first
            // operation.
            match op {
                Operation::Read(buf) => self.do_read(address, buf).await?,
                Operation::Write(buf) => self.do_write(address, buf).await?,
            }

            // For all subsequent operations, just send/read more bytes without any more
            // ceremony.
            for op in group {
                match op {
                    Operation::Read(buf) => self.continue_read(buf).await?,
                    Operation::Write(buf) => self.continue_write(buf).await?,
                }
            }

            let regs = &mut self.i2c.config.as_mut().registers;
            if op_groups.peek().is_some() {
                // If we still have more groups to go, send a repeated start
                regs.cmd_repeated_start();
            } else {
                // Otherwise, send a stop
                regs.cmd_stop();
            }
        }

        Ok(())
    }
}

#[cfg(feature = "dma")]
mod dma {
    use embedded_hal_1::i2c::Operation;
    use embedded_hal_async::i2c::I2c as I2cTrait;

    use super::*;
    use crate::dmac::sram::DmacDescriptor;
    use crate::dmac::{AnyChannel, Channel, ReadyFuture};
    use crate::sercom::dma::async_dma::{read_dma_linked, write_dma_linked};
    use crate::sercom::dma::SharedSliceBuffer;

    #[cfg(feature = "dma")]
    /// Convenience type for a [`I2cFuture`] in DMA mode.
    ///
    /// The type parameter `I` represents the DMA channel ID (`ChX`).
    pub type I2cFutureDma<C, I> = I2cFuture<C, Channel<I, ReadyFuture>>;

    impl<C, D> I2cFuture<C, D>
    where
        C: AnyConfig,
        D: AnyChannel<Status = ReadyFuture>,
    {
        /// Reclaim the DMA channel. Any subsequent I2C operations will no
        /// longer use DMA.
        pub fn take_dma_channel(self) -> (I2cFuture<C, crate::typelevel::NoneT>, D) {
            let (i2c, channel) = self.i2c.take_dma_channel();
            (I2cFuture { i2c }, channel)
        }
    }

    impl<C, S, D> I2cFuture<C, D>
    where
        C: AnyConfig<Sercom = S>,
        S: Sercom,
        D: AnyChannel<Status = ReadyFuture>,
    {
        /// Make an async I2C write transaction, with the option to add in
        /// linked transfers after this first transaction.
        ///
        /// # Safety
        ///
        /// If `next` is [`Some`], the pointer in its `descaddr` field, along
        /// with the descriptor it points to, etc, must point to a valid
        /// [`DmacDescriptor`] memory location, or be null. They must not be
        /// circular (ie, points to itself). Any linked transfer must
        /// strictly be a write transaction (source pointer is a byte buffer,
        /// destination pointer is the SERCOM DATA register).
        #[inline]
        pub(super) async unsafe fn write_linked(
            &mut self,
            address: u8,
            bytes: &[u8],
            next: Option<&mut DmacDescriptor>,
        ) -> Result<(), i2c::Error> {
            self.i2c.prepare_write_linked(address, bytes, &next)?;

            let sercom_ptr = self.i2c.sercom_ptr();
            let mut bytes = SharedSliceBuffer::from_slice(bytes);

            write_dma_linked::<_, _, S>(&mut self.i2c._dma_channel, sercom_ptr, &mut bytes, next)
                .await?;

            // Unfortunately, gotta take a polling approach here as there is no interrupt
            // source that can notify us of an IDLE bus state. Fortunately, it's usually not
            // long. 8-10 times around the loop will do the trick.
            while !self.i2c.read_status().is_idle() {
                core::hint::spin_loop();
            }
            self.i2c.read_status().check_bus_error()?;
            Ok(())
        }

        /// Asynchronously read into a buffer using DMA.
        #[inline]
        pub(super) async unsafe fn read_linked(
            &mut self,
            address: u8,
            mut buffer: &mut [u8],
            next: Option<&mut DmacDescriptor>,
        ) -> Result<(), i2c::Error> {
            self.i2c.prepare_read_linked(address, buffer, &next)?;
            let i2c_ptr = self.i2c.sercom_ptr();

            read_dma_linked::<_, _, S>(&mut self.i2c._dma_channel, i2c_ptr, &mut buffer, next)
                .await?;

            // Unfortunately, gotta take a polling approach here as there is no interrupt
            // source that can notify us of an IDLE bus state. Fortunately, it's usually not
            // long. 8-10 times around the loop will do the trick.
            while !self.i2c.read_status().is_idle() {
                core::hint::spin_loop();
            }
            self.i2c.read_status().check_bus_error()?;

            Ok(())
        }

        /// Asynchronously write from a buffer, then read into a buffer, all
        /// using DMA.
        ///
        /// This is an extremely common pattern: for example, writing a
        /// register address, then read its value from the slave.
        #[inline]
        pub async fn write_read(
            &mut self,
            addr: u8,
            write_buf: &[u8],
            read_buf: &mut [u8],
        ) -> Result<(), i2c::Error> {
            unsafe {
                self.write_linked(addr, write_buf, None).await?;
                self.read_linked(addr, read_buf, None).await?;
            }
            Ok(())
        }
    }

    impl<C, D> I2cTrait for I2cFuture<C, D>
    where
        C: AnyConfig,
        D: AnyChannel<Status = ReadyFuture>,
    {
        #[inline]
        async fn read(&mut self, address: u8, buffer: &mut [u8]) -> Result<(), Self::Error> {
            unsafe { self.read_linked(address, buffer, None).await }
        }

        #[inline]
        async fn write(&mut self, address: u8, bytes: &[u8]) -> Result<(), Self::Error> {
            unsafe { self.write_linked(address, bytes, None).await }
        }

        #[inline]
        async fn write_read(
            &mut self,
            address: u8,
            wr_buffer: &[u8],
            rd_buffer: &mut [u8],
        ) -> Result<(), Self::Error> {
            self.write_read(address, wr_buffer, rd_buffer).await?;
            Ok(())
        }

        #[inline]
        async fn transaction(
            &mut self,
            address: u8,
            operations: &mut [Operation<'_>],
        ) -> Result<(), Self::Error> {
            use crate::dmac::{channel, sram::DmacDescriptor};
            use crate::sercom::dma::SharedSliceBuffer;
            use Operation::{Read, Write};

            const NUM_LINKED_TRANSFERS: usize = 16;

            if operations.is_empty() {
                return Ok(());
            }

            let mut sercom_ptr = self.i2c.sercom_ptr();

            // Reserve some space for linked DMA transfer descriptors.
            // Uses 256 bytes of memory.
            //
            // In practice this means that we can only support 17 continuously
            // linked operations of the same type (R/W) before having to issue
            // an I2C STOP. DMA-enabled I2C transfers automatically issue stop
            // commands, and there is no way to turn off that behaviour.
            //
            //  In the event that we have more than 17 contiguous operations of
            //  the same type, we must revert to the byte-by-byte I2C implementations.
            let mut descriptors = heapless::Vec::<DmacDescriptor, NUM_LINKED_TRANSFERS>::new();

            // Arrange operations in groups of contiguous types (R/W)
            let op_groups = operations.chunk_by_mut(|this, next| {
                matches!((this, next), (Write(_), Write(_)) | (Read(_), Read(_)))
            });

            for group in op_groups {
                descriptors.clear();

                // Default to byte-by-byte impl if we have more than 17 continuous operations,
                // as we would overflow our DMA linked transfer reeserved space otherwise.
                if group.len() > NUM_LINKED_TRANSFERS {
                    self.i2c.transaction_byte_by_byte(address, group)?;
                } else {
                    // --- Setup all linked descriptors ---

                    // Skip the first operation; we will deal with it when creating the I2C transfer
                    // (read_dma_linked/write_dma_linked). Every other operation is a linked
                    // transfer, and we must treat them accordingly.
                    for op in group.iter_mut().skip(1) {
                        match op {
                            Read(ref mut buffer) => {
                                if buffer.is_empty() {
                                    continue;
                                }
                                // Add a new linked descriptor to the stack
                                descriptors
                                    .push(DmacDescriptor::default())
                                    .unwrap_or_else(|_| panic!("BUG: DMAC descriptors overflow"));
                                let last_descriptor = descriptors.last_mut().unwrap();
                                let next_ptr =
                                    (last_descriptor as *mut DmacDescriptor).wrapping_add(1);

                                unsafe {
                                    channel::write_descriptor(
                                        last_descriptor,
                                        &mut sercom_ptr,
                                        buffer,
                                        // Always link the next descriptor. We then set the last
                                        // transfer's link pointer to null lower down in the code.
                                        next_ptr,
                                    );
                                }
                            }

                            Write(bytes) => {
                                if bytes.is_empty() {
                                    continue;
                                }
                                // Add a new linked descriptor to the stack
                                descriptors
                                    .push(DmacDescriptor::default())
                                    .unwrap_or_else(|_| panic!("BUG: DMAC descriptors overflow"));
                                let last_descriptor = descriptors.last_mut().unwrap();
                                let next_ptr =
                                    (last_descriptor as *mut DmacDescriptor).wrapping_add(1);

                                let mut bytes = SharedSliceBuffer::from_slice(bytes);
                                unsafe {
                                    channel::write_descriptor(
                                        last_descriptor,
                                        &mut bytes,
                                        &mut sercom_ptr,
                                        // Always link the next descriptor. We then set the last
                                        // transfer's link pointer to null lower down in the code.
                                        next_ptr,
                                    );
                                }
                            }
                        }
                    }

                    // Set the last descriptor to a null pointer to stop the transfer, and avoid
                    // buffer overflow UB.
                    if let Some(d) = descriptors.last_mut() {
                        d.set_next_descriptor(core::ptr::null_mut());
                    }

                    // Now setup and perform the actual transfer
                    match group.first_mut().unwrap() {
                        Read(ref mut buffer) => unsafe {
                            self.read_linked(address, buffer, descriptors.first_mut())
                                .await?;
                        },
                        Write(bytes) => unsafe {
                            self.write_linked(address, bytes, descriptors.first_mut())
                                .await?;
                        },
                    }
                }
            }

            Ok(())
        }
    }
}

#[cfg(feature = "dma")]
pub use dma::*;
