use crate::sercom::{
    i2c::{self, AnyConfig, Flags, I2c},
    Sercom,
};
use core::task::Poll;
use cortex_m::interrupt::InterruptNumber;
use cortex_m_interrupt::NvicInterruptHandle;

impl<C, S> I2c<C>
where
    C: AnyConfig<Sercom = S>,
    S: Sercom,
{
    /// Turn an [`I2c`] into an [`I2cFuture`]
    #[inline]
    pub fn into_future<I, N>(self, irq: I) -> I2cFuture<C, N>
    where
        I: NvicInterruptHandle<N>,
        N: InterruptNumber,
    {
        let irq_number = irq.number();
        irq.register(S::on_interrupt_i2c);
        unsafe { cortex_m::peripheral::NVIC::unmask(irq_number) };

        I2cFuture {
            i2c: self,
            irq_number,
        }
    }
}

pub struct I2cFuture<C, N>
where
    C: AnyConfig,
    N: InterruptNumber,
{
    i2c: I2c<C>,
    irq_number: N,
}

impl<C, N, S> I2cFuture<C, N>
where
    C: AnyConfig<Sercom = S>,
    S: Sercom,
    N: InterruptNumber,
{
    /// Asynchronously write from a buffer.
    #[inline]
    pub async fn write(&mut self, addr: u8, bytes: &[u8]) -> Result<(), i2c::Error> {
        self.i2c.config.as_mut().registers.start_write(addr)?;

        for b in bytes {
            self.wait_flags(Flags::MB | Flags::ERROR).await;
            self.i2c.read_status().check_bus_error()?;

            self.i2c.config.as_mut().registers.write_one(*b);
        }

        self.i2c.cmd_stop();

        Ok(())
    }

    /// Asynchronously read into a buffer.
    #[inline]
    pub async fn read(&mut self, addr: u8, buffer: &mut [u8]) -> Result<(), i2c::Error> {
        self.i2c.config.as_mut().registers.start_read(addr)?;

        // Some manual iterator gumph because we need to ack bytes after the first.
        let mut iter = buffer.iter_mut();
        *iter.next().expect("buffer len is at least 1") = self.read_one().await;

        loop {
            match iter.next() {
                None => break,
                Some(dest) => {
                    // Ack the last byte so we can receive another one
                    self.i2c.config.as_mut().registers.cmd_read();
                    *dest = self.read_one().await;
                }
            }
        }

        // Arrange to send NACK on next command to
        // stop slave from transmitting more data
        self.i2c
            .config
            .as_mut()
            .registers
            .i2c_master()
            .ctrlb
            .modify(|_, w| w.ackact().set_bit());

        Ok(())
    }

    /// Asynchronously write from a buffer, then read into a buffer. This is an
    /// extremely common pattern: writing a register address, then
    /// read its value from the slave.
    #[inline]
    pub async fn write_read(
        &mut self,
        addr: u8,
        write_buf: &[u8],
        read_buf: &mut [u8],
    ) -> Result<(), i2c::Error> {
        self.write(addr, write_buf).await?;
        self.read(addr, read_buf).await?;
        Ok(())
    }

    async fn read_one(&mut self) -> u8 {
        self.wait_flags(Flags::SB | Flags::ERROR).await;
        self.i2c.config.as_mut().registers.read_one()
    }

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

impl<C, N> Drop for I2cFuture<C, N>
where
    C: AnyConfig,
    N: InterruptNumber,
{
    #[inline]
    fn drop(&mut self) {
        cortex_m::peripheral::NVIC::mask(self.irq_number);
    }
}

impl<C, N> AsRef<I2c<C>> for I2cFuture<C, N>
where
    C: AnyConfig,
    N: InterruptNumber,
{
    #[inline]
    fn as_ref(&self) -> &I2c<C> {
        &self.i2c
    }
}

impl<C, N> AsMut<I2c<C>> for I2cFuture<C, N>
where
    C: AnyConfig,
    N: InterruptNumber,
{
    #[inline]
    fn as_mut(&mut self) -> &mut I2c<C> {
        &mut self.i2c
    }
}

#[cfg(feature = "nightly")]
mod impl_ehal {
    use super::*;
    use core::future::Future;
    use embedded_hal_async::i2c::{ErrorType, I2c as I2cTrait, Operation};

    impl<C, N> ErrorType for I2cFuture<C, N>
    where
        C: AnyConfig,
        N: InterruptNumber,
    {
        type Error = i2c::Error;
    }

    impl<C, N> I2cTrait for I2cFuture<C, N>
    where
        C: AnyConfig,
        N: InterruptNumber,
    {
        type ReadFuture<'a> = impl Future<Output = Result<(), Self::Error>> + 'a where Self: 'a;

        #[inline]
        fn read<'a>(&'a mut self, address: u8, buffer: &'a mut [u8]) -> Self::ReadFuture<'a> {
            self.read(address, buffer)
        }

        type WriteFuture<'a> = impl Future<Output = Result<(), Self::Error>> + 'a where Self: 'a;

        #[inline]
        fn write<'a>(&'a mut self, address: u8, bytes: &'a [u8]) -> Self::WriteFuture<'a> {
            self.write(address, bytes)
        }

        type WriteReadFuture<'a> = impl Future<Output = Result<(), Self::Error>> + 'a where Self: 'a;

        #[inline]
        fn write_read<'a>(
            &'a mut self,
            address: u8,
            wr_buffer: &'a [u8],
            rd_buffer: &'a mut [u8],
        ) -> Self::WriteReadFuture<'a> {
            self.write_read(address, wr_buffer, rd_buffer)
        }

        type TransactionFuture<'a, 'b> = impl Future<Output = Result<(), Self::Error>> + 'a where Self: 'a, 'b: 'a;

        #[inline]
        fn transaction<'a, 'b>(
            &'a mut self,
            address: u8,
            operations: &'a mut [embedded_hal_async::i2c::Operation<'b>],
        ) -> Self::TransactionFuture<'a, 'b> {
            async move {
                for op in operations {
                    match op {
                        Operation::Read(buf) => self.read(address, buf).await?,
                        Operation::Write(buf) => self.write(address, buf).await?,
                    }
                }

                Ok(())
            }
        }
    }
}
