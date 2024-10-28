//! [`embedded-hal`] trait implementations for [`I2c`]s

use super::{config::AnyConfig, flags::Error, I2c};
use crate::ehal::i2c::{self, ErrorKind, ErrorType, NoAcknowledgeSource};

impl i2c::Error for Error {
    #[allow(unreachable_patterns)]
    fn kind(&self) -> ErrorKind {
        match self {
            Error::BusError => ErrorKind::Bus,
            Error::ArbitrationLost => ErrorKind::ArbitrationLoss,
            Error::LengthError => ErrorKind::Other,
            Error::Nack => ErrorKind::NoAcknowledge(NoAcknowledgeSource::Unknown),
            Error::Timeout => ErrorKind::Other,
            // Pattern reachable when "dma" feature is enabled
            _ => ErrorKind::Other,
        }
    }
}

impl<C: AnyConfig, D> ErrorType for I2c<C, D> {
    type Error = Error;
}

impl<C: AnyConfig, D> I2c<C, D> {
    fn transaction_byte_by_byte(
        &mut self,
        address: u8,
        operations: &mut [i2c::Operation<'_>],
    ) -> Result<(), Error> {
        /// Helper type for keeping track of the type of operation that was
        /// executed last
        #[derive(Clone, Copy)]
        enum Operation {
            Read,
            Write,
        }

        // Keep track of the last executed operation type. The method
        // specification demands, that no repeated start condition is sent
        // between adjacent operations of the same type.
        let mut last_op = None;
        for op in operations {
            match op {
                i2c::Operation::Read(buf) => {
                    if let Some(Operation::Read) = last_op {
                        self.continue_read(buf)?;
                    } else {
                        self.do_read(address, buf)?;
                        last_op = Some(Operation::Read);
                    }
                }
                i2c::Operation::Write(bytes) => {
                    if let Some(Operation::Write) = last_op {
                        self.continue_write(bytes)?;
                    } else {
                        self.do_write(address, bytes)?;
                        last_op = Some(Operation::Write);
                    }
                }
            }
        }

        self.cmd_stop();
        Ok(())
    }
}

impl<C: AnyConfig> i2c::I2c for I2c<C> {
    fn transaction(
        &mut self,
        address: u8,
        operations: &mut [i2c::Operation<'_>],
    ) -> Result<(), Self::Error> {
        self.transaction_byte_by_byte(address, operations)?;
        Ok(())
    }

    fn write(&mut self, address: u8, bytes: &[u8]) -> Result<(), Self::Error> {
        self.do_write(address, bytes)?;
        self.cmd_stop();
        Ok(())
    }

    fn read(&mut self, address: u8, buffer: &mut [u8]) -> Result<(), Self::Error> {
        self.do_read(address, buffer)?;
        self.cmd_stop();
        Ok(())
    }

    fn write_read(
        &mut self,
        address: u8,
        bytes: &[u8],
        buffer: &mut [u8],
    ) -> Result<(), Self::Error> {
        self.do_write_read(address, bytes, buffer)?;
        self.cmd_stop();
        Ok(())
    }
}

#[cfg(feature = "dma")]
mod dma {
    use super::*;
    use crate::dmac::{channel, sram::DmacDescriptor, AnyChannel, Ready};
    use crate::sercom::dma::{read_dma_linked, write_dma_linked, SercomPtr, SharedSliceBuffer};
    use crate::sercom::{self, Sercom};

    impl<C, D, S> I2c<C, D>
    where
        C: AnyConfig<Sercom = S>,
        S: Sercom,
        D: AnyChannel<Status = Ready>,
    {
        fn sercom_ptr(&self) -> SercomPtr<sercom::i2c::Word> {
            SercomPtr(self.data_ptr())
        }

        /// Walk up the transfer linked list, and calculate the number of beats
        /// the entire block list contains.
        ///
        /// # Safety
        ///
        /// If `next` is [`Some`], the pointer in its `descaddr` field, along
        /// with the descriptor it points to, etc, must point to a valid
        /// [`DmacDescriptor`] memory location, or be null. They must not be
        /// circular (ie, points to itself).
        #[inline]
        unsafe fn linked_transfer_length(next: &Option<&mut DmacDescriptor>) -> usize {
            let mut cnt = 0;

            if let Some(next) = next {
                cnt += next.beat_count() as usize;
                let mut next_ptr = next.next_descriptor();

                while !next_ptr.is_null() {
                    let next = *next_ptr;
                    cnt += next.beat_count() as usize;
                    next_ptr = next.next_descriptor();
                }
            }

            cnt
        }

        /// Make an I2C read transaction, with the option to add in linked
        /// transfers after this first transaction.
        ///
        /// # Safety
        ///
        /// If `next` is [`Some`], the pointer in its `descaddr` field, along
        /// with the descriptor it points to, etc, must point to a valid
        /// [`DmacDescriptor`] memory location, or be null. They must not be
        /// circular (ie, points to itself). Any linked transfer must
        /// strictly be a read transaction (destination pointer is a byte
        /// buffer, source pointer is the SERCOM DATA register).
        #[inline]
        unsafe fn read_linked(
            &mut self,
            address: u8,
            mut dest: &mut [u8],
            next: Option<&mut DmacDescriptor>,
        ) -> Result<(), Error> {
            self.check_bus_status()?;
            let sercom_ptr = self.sercom_ptr();

            if dest.is_empty() {
                return Ok(());
            }

            // Calculate the total number of bytes for this transaction across all linked
            // transfers, including the first transfer.
            let transfer_len = dest.len() + Self::linked_transfer_length(&next);

            assert!(
                transfer_len <= 255,
                "I2C read transfers of more than 255 bytes are unsupported."
            );

            self.start_dma_read(address, transfer_len as u8);
            let channel = self.dma_channel.as_mut();

            // SAFETY: We must make sure that any DMA transfer is complete or stopped before
            // returning.
            read_dma_linked::<_, _, S>(channel, sercom_ptr, &mut dest, next);

            while !channel.xfer_complete() {
                core::hint::spin_loop();
            }

            // Defensively disable channel
            channel.stop();

            self.read_status().check_bus_error()?;
            self.dma_channel.as_mut().xfer_success()?;
            Ok(())
        }

        /// Make an I2C write transaction, with the option to add in linked
        /// transfers after this first transaction.
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
        unsafe fn write_linked(
            &mut self,
            address: u8,
            source: &[u8],
            next: Option<&mut DmacDescriptor>,
        ) -> Result<(), Error> {
            self.check_bus_status()?;
            let sercom_ptr = self.sercom_ptr();

            if source.is_empty() {
                return Ok(());
            }

            // Calculate the total number of bytes for this transaction across all linked
            // transfers, including the first transfer.
            let transfer_len = source.len() + Self::linked_transfer_length(&next);

            assert!(
                transfer_len <= 255,
                "I2C write transfers of more than 255 bytes are unsupported."
            );

            self.start_dma_write(address, transfer_len as u8);
            let mut bytes = SharedSliceBuffer::from_slice(source);
            let channel = self.dma_channel.as_mut();

            // SAFETY: We must make sure that any DMA transfer is complete or stopped before
            // returning.
            write_dma_linked::<_, _, S>(channel, sercom_ptr, &mut bytes, next);

            while !channel.xfer_complete() {
                core::hint::spin_loop();
            }

            // Defensively disable channel
            channel.stop();

            while !self.read_status().is_idle() {
                core::hint::spin_loop();
            }

            self.read_status().check_bus_error()?;
            self.dma_channel.as_mut().xfer_success()?;
            Ok(())
        }
    }

    impl<C, D, S> i2c::I2c for I2c<C, D>
    where
        C: AnyConfig<Sercom = S>,
        S: Sercom,
        D: AnyChannel<Status = Ready>,
    {
        #[inline]
        fn transaction(
            &mut self,
            address: u8,
            operations: &mut [i2c::Operation<'_>],
        ) -> Result<(), Self::Error> {
            use i2c::Operation::{Read, Write};

            const NUM_LINKED_TRANSFERS: usize = 16;

            if operations.is_empty() {
                return Ok(());
            }

            let mut sercom_ptr = self.sercom_ptr();

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
                    self.transaction_byte_by_byte(address, group)?;
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
                            self.read_linked(address, buffer, descriptors.first_mut())?;
                        },
                        Write(bytes) => unsafe {
                            self.write_linked(address, bytes, descriptors.first_mut())?;
                        },
                    }
                }
            }

            Ok(())
        }

        #[inline]
        fn write(&mut self, address: u8, bytes: &[u8]) -> Result<(), Self::Error> {
            unsafe { self.write_linked(address, bytes, None) }
        }

        #[inline]
        fn read(&mut self, address: u8, buffer: &mut [u8]) -> Result<(), Self::Error> {
            unsafe { self.read_linked(address, buffer, None) }
        }

        #[inline]
        fn write_read(
            &mut self,
            address: u8,
            bytes: &[u8],
            buffer: &mut [u8],
        ) -> Result<(), Self::Error> {
            self.write(address, bytes)?;
            self.read(address, buffer)?;
            Ok(())
        }
    }
}

impl<C: AnyConfig> crate::ehal_02::blocking::i2c::Write for I2c<C> {
    type Error = Error;

    fn write(&mut self, addr: u8, bytes: &[u8]) -> Result<(), Self::Error> {
        self.do_write(addr, bytes)?;
        self.cmd_stop();
        Ok(())
    }
}

impl<C: AnyConfig> crate::ehal_02::blocking::i2c::Read for I2c<C> {
    type Error = Error;

    fn read(&mut self, addr: u8, buffer: &mut [u8]) -> Result<(), Self::Error> {
        self.do_read(addr, buffer)?;
        self.cmd_stop();
        Ok(())
    }
}

impl<C: AnyConfig> crate::ehal_02::blocking::i2c::WriteRead for I2c<C> {
    type Error = Error;

    fn write_read(&mut self, addr: u8, bytes: &[u8], buffer: &mut [u8]) -> Result<(), Self::Error> {
        self.do_write_read(addr, bytes, buffer)?;
        self.cmd_stop();
        Ok(())
    }
}
