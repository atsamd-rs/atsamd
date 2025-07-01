//! Implement Embedded HAL ([v0.2](crate::ehal_02) and [nb](ehal_nb)) traits for
//! [`Spi`] structs
//!
//! As noted in the [spi module](super) documentation, the embedded-hal trait
//! implementations vary by both [`Size`] and [`Capability`]. Each
//! implementation is optimized to take advantage of all information known at
//! compile-time, so it is importatnt to carefully read the documentation in
//! this module.
//!
//! # Variations by [`Size`]
//!
//! SAMD11 and SAMD21 chips do not have 32-bit extension mode, so their
//! transaction `Size` can only vary by the [`CharSize`]. Both options,
//! [`EightBit`] and [`NineBit`], are [`AtomicSize`]s, because each can be
//! completed with a single read/write of the `DATA` register. Consequently,
//! each can implement both the blocking and non-blocking embedded HAL traits.
//! These traits are implemented for the [`Word`] type of the corresponding
//! `CharSize`. For example, an [`Spi`] struct with a `NineBit` `CharSize` would
//! implement `spi::FullDuplex<u16>`.
//!
//! Note that embedded HAL does not offer a way to transmit slices in a
//! non-blocking fashion, but this can be done using
#![cfg_attr(feature = "dma", doc = "[`DMA`](crate::dmac)")]
#![cfg_attr(not(feature = "dma"), doc = "`DMA`")]
//! .
//!
//! # Variations by [`Capability`]
//!
//! The implementations in this module also seek to optimize as much as possible
//! based on the `Capability` of the `Spi` struct. They follow a few general
//! rules:
//! - [`Tx`] structs can never receive data, so their corresponding trait
//!   implementations never read the `DATA` register and can never return an
//!   [`Error::Overflow`].
//! - [`Rx`] structs in a [`MasterMode`](super::MasterMode) must initiate all
//!   transactions, so their implementations of non-blocking traits must track
//!   the state of on-going transactions.
//! - [`Duplex`] structs must always read as many bytes as they send, even when
//!   implementing `Write`-only traits, to ensure they never introduce an
//!   [`Error::Overflow`].
//!
//! # Notes on individual embedded HAL traits
//!
//! ## `spi::FullDuplex`
//!
//! `spi::FullDuplex` is only implemented for structs with `Duplex`
//! `Capability`. Although the embedded HAL documentation assumes a
//! `MasterMode`, this module also implements it for the [`Slave`] [`OpMode`].
//!
//! ## `serial::Read`
//!
//! `serial::Read` is only implemented for structs with `Rx` `Capability`. When
//! in a `MasterMode`, it initiates and tracks the state of the on-going
//! transactions. But this is not required when acting as a `Slave`.
//!
//! ## `serial::Write`
//!
//! `serial::Write` is only implemented for structs with `Tx` `Capability`.
//! These implementations never read the `DATA` register and ignore all
//! instances of [`Error::Overflow`].
//!
//! ## `blocking::serial::Write`
//!
//! This trait uses the `blocking::serial::write::Default` implementation for
//! implementers of `serial::Write`.
//!
//! ## `blocking::spi` traits
//!
//! These traits are implemented following all of the rules outlined above for
//! the different [`Size`] and [`Capability`] options.

use super::*;
use crate::ehal_nb;
use nb::Error::WouldBlock;
use num_traits::{AsPrimitive, PrimInt};

//=============================================================================
// serial::Read
//=============================================================================

/// Implement [`Read`] for [`Rx`] [`Spi`] structs in a [`MasterMode`]
///
/// [`Read`] is only implemented for `Spi` structs with `Rx`
/// [`Capability`]. In a `MasterMode`, `Read` has to initiate transactions, so
/// it keeps track of the transaction state. If a transaction is in progress,
/// it will wait on `RXC`. If not, it will wait on `DRE`, and then send `0`.
///
/// [`Read`]: ehal_nb::serial::Read
impl<P, M, C> ehal_nb::serial::Read<C::Word> for Spi<Config<P, M, C>, Rx>
where
    Config<P, M, C>: ValidConfig,
    P: ValidPads,
    M: MasterMode,
    C: CharSize,
    C::Word: PrimInt,
    DataWidth: AsPrimitive<C::Word>,
{
    #[inline]
    fn read(&mut self) -> nb::Result<C::Word, Error> {
        let in_progress = self.capability.in_progress;
        let flags = self.read_flags_errors()?;
        if !in_progress && flags.contains(Flags::DRE) {
            unsafe { self.write_data(0) };
            self.capability.in_progress = true;
            Err(WouldBlock)
        } else if in_progress && flags.contains(Flags::RXC) {
            self.capability.in_progress = false;
            unsafe { Ok(self.read_data().as_()) }
        } else {
            Err(WouldBlock)
        }
    }
}

/// Implement [`Read`] for [`Rx`] [`Spi`] structs in [`Slave`]
/// [`OpMode`]
///
/// [`Read`] is only implemented for `Spi` structs with `Rx`
/// [`Capability`]. In `Slave` `OpMode`, `Read` does not have to initiate
/// transactions, so it does not have to store any internal state. It only has
/// to wait on `RXC`.
///
/// [`Read`]: ehal_nb::serial::Read
impl<P, C> ehal_nb::serial::Read<C::Word> for Spi<Config<P, Slave, C>, Rx>
where
    Config<P, Slave, C>: ValidConfig,
    P: ValidPads,
    C: CharSize,
    C::Word: PrimInt,
    DataWidth: AsPrimitive<C::Word>,
{
    /// Wait for an `RXC` flag, then read the word
    #[inline]
    fn read(&mut self) -> nb::Result<C::Word, Error> {
        let flags = self.read_flags_errors()?;
        if flags.contains(Flags::RXC) {
            unsafe { Ok(self.read_data().as_()) }
        } else {
            Err(WouldBlock)
        }
    }
}

//=============================================================================
// embedded_io::Read
//=============================================================================
impl<P, M> embedded_io::Read for Spi<Config<P, M, EightBit>, Rx>
where
    Config<P, M, EightBit>: ValidConfig,
    P: ValidPads,
    M: MasterMode,
{
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
        for byte in buf.iter_mut() {
            let w = nb::block!(<Self as ehal_nb::serial::Read>::read(self))?;
            *byte = w;
        }

        Ok(buf.len())
    }
}

impl<P> embedded_io::Read for Spi<Config<P, Slave, EightBit>, Rx>
where
    Config<P, Slave, EightBit>: ValidConfig,
    P: ValidPads,
{
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
        if buf.is_empty() {
            return Ok(0);
        };

        for byte in buf.iter_mut() {
            let w = nb::block!(<Self as ehal_nb::serial::Read>::read(self))?;
            *byte = w;
        }

        Ok(buf.len())
    }
}

//=============================================================================
// serial::Write
//=============================================================================

/// Implement [`Write`] for [`Tx`] [`Spi`] structs
///
/// [`Write`] is only implemented for `Spi` structs with `Tx`
/// [`Capability`]. Because the `Capability` is `Tx`, this implementation never
/// reads the DATA register and ignores all buffer overflow errors.
///
/// [`Write`]: ehal_nb::serial::Write
impl<P, M, C> ehal_nb::serial::Write<C::Word> for Spi<Config<P, M, C>, Tx>
where
    Config<P, M, C>: ValidConfig,
    P: ValidPads,
    M: OpMode,
    C: CharSize,
    C::Word: PrimInt + AsPrimitive<DataWidth>,
{
    #[inline]
    fn write(&mut self, word: C::Word) -> nb::Result<(), Error> {
        // Ignore buffer overflow errors
        if self.read_flags().contains(Flags::DRE) {
            unsafe { self.write_data(word.as_()) };
            Ok(())
        } else {
            Err(WouldBlock)
        }
    }

    #[inline]
    fn flush(&mut self) -> nb::Result<(), Error> {
        // Ignore buffer overflow errors
        if self.read_flags().contains(Flags::TXC) {
            Ok(())
        } else {
            Err(WouldBlock)
        }
    }
}

//=============================================================================
// embedded_io::Write
//=============================================================================
impl<P, M> embedded_io::Write for Spi<Config<P, M, EightBit>, Tx>
where
    Config<P, M, EightBit>: ValidConfig,
    P: ValidPads,
    M: OpMode,
{
    fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error> {
        for byte in buf {
            nb::block!(<Self as ehal_nb::serial::Write>::write(self, *byte))?;
        }

        Ok(buf.len())
    }

    fn flush(&mut self) -> Result<(), Self::Error> {
        nb::block!(<Self as ehal_nb::serial::Write>::flush(self))
    }
}

//=============================================================================
// spi::FullDuplex
//=============================================================================

/// Implement embedded-hal-nb [`spi::FullDuplex`] for [`Spi`] structs with [`AtomicSize`]
///
/// `spi::FullDuplex` is only implemented when the `Spi` struct has [`Duplex`]
/// [`Capability`]. The [`Word`] size used in the implementation depends on the
/// corresponding [`CharSize`].
///
/// [`spi::FullDuplex`]: ehal_nb::spi::FullDuplex
impl<C> ehal_nb::spi::FullDuplex<C::Word> for Spi<C, Duplex>
where
    C: ValidConfig,
    C::Word: PrimInt + AsPrimitive<u16>,
    u16: AsPrimitive<C::Word>,
{
    #[inline]
    fn read(&mut self) -> nb::Result<C::Word, Error> {
        let flags = self.read_flags_errors()?;
        if flags.contains(Flags::RXC) {
            unsafe { Ok(self.read_data().as_()) }
        } else {
            Err(WouldBlock)
        }
    }

    #[inline]
    fn write(&mut self, word: C::Word) -> nb::Result<(), Self::Error> {
        let flags = self.read_flags_errors()?;
        if flags.contains(Flags::DRE) {
            unsafe { self.write_data(word.as_()) };
            Ok(())
        } else {
            Err(WouldBlock)
        }
    }
}
