//! Implement [`embedded_hal`] traits for [`Spi`] structs
//!
//! As noted in the [spi module](super) documentation, the embedded-hal trait
//! implementations vary by both [`Size`] and [`Capability`]. Each
//! implementation is optimized to take advantage of all information known at
//! compile-time, so it is importatnt to carefully read the documentation in
//! this module.
//!
//! # Variations by [`Size`]
//!
//! Remember that SAMx5x chips operate in 32-bit extension mode and use the
//! hardware `LENGTH` counter to set the number of bytes in each transaction.
//! The transaction [`Length`] is usually tracked at compile-time using
//! type-level integers from the [`typenum`] crate, but it can also be tracked
//! at run-time when using a [`DynLength`].
//!
//! The transaction `Length`s can be sub-divided into three groups:
//! - `Length`s of 1-4 bytes can be completed in a single read/write of the
//!   `DATA` register. These `Length`s are marked as [`AtomicSize`]s.
//! - `Length`s [`GreaterThan4`] are known at compile-time but cannot be
//!   completed atomically.
//! - A `DynLength` can be any length, but the value is only known at run-time.
//!
//! In general, transaction lengths with an `AtomicSize` implement embedded HAL
//! traits with the corresponding [`Word`] type. For example, [`Spi`] structs
//! using a transaction `Length` of 2 bytes implement `FullDuplex<u16>`. These
//! lengths implement both the blocking and non-blocking traits from embedded
//! HAL. The non-blocking traits are found in the [`spi`] and [`serial`]
//! modules, while the blocking traits are found in the [`blocking`] module.
//!
//! Transaction lengths `GreaterThan4` cannot be completed in a single read or
//! write of the `DATA` register, so these lengths do **NOT** implement the
//! non-blocking traits from the embedded HAL `spi` and `serial` modules.
//! Instead, they only implement traits from the `blocking` module. These traits
//! are implemented for `u8` types, e.g. `blocking::spi::Transfer<u8>`, and
//! operate on `[u8]` slices. The length of the slice is checked to ensure it
//! matches the transaction `Length`.
//!
//! Because a `DynLength` is not guaranteed to be an `AtomicSize`, the
//! corresponding `Spi` structs only implement the `blocking` traits as well.
//!
//! For a non-blocking alternative that can be used to transfer arbitrary-length
//! slices, you could use either
#![cfg_attr(feature = "dma", doc = "[`DMA`](crate::dmac)")]
#![cfg_attr(not(feature = "dma"), doc = "`DMA`")]
//! or the [`spi_future`](super::super::spi_future) module.
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

use core::cell::Cell;
use core::cmp::min;

use embedded_hal::blocking;
use typenum::{U1, U2, U3, U4};

use super::impl_ehal_common::*;
use super::*;

//=============================================================================
// blocking::spi::Transfer
//=============================================================================

impl_blocking_spi_transfer!(U1, U2, U3, U4);

/// Implement [`Transfer`] for [`Spi`] structs that can [`Receive`] and have
/// long transaction [`Length`]s
///
/// The transaction [`Length`] must be `> 4`. The transfer accepts a slice of
/// `u8` with a length equal to the transaction [`Length`]. If the slice length
/// is incorrect, it will panic.
///
/// [`Transfer`]: blocking::spi::Transfer
impl<P, M, L> blocking::spi::Transfer<u8> for Spi<P, M, L>
where
    P: ValidPads,
    P::Capability: Receive,
    M: OpMode,
    L: NonAtomicSize,
{
    type Error = Error;

    #[inline]
    fn transfer<'w>(&mut self, buf: &'w mut [u8]) -> Result<&'w [u8], Error> {
        if buf.len() != self.get_length() as usize {
            panic!("Slice length does not equal SPI transfer length");
        }
        blocking_transfer_non_atomic(self, buf)
    }
}

//=============================================================================
// blocking::spi::Write
//=============================================================================

impl_blocking_spi_write!(U1, U2, U3, U4);

/// Implement [`Write`] for [`Spi`] structs with [`Duplex`] [`Capability`] and
/// long transaction [`Length`]s
///
/// The transaction [`Length`] must be `> 4`. The transfer accepts a `[u8]` with
/// a length equal to the transfer [`Length`]. If the slice length is incorrect,
/// it will panic.
///
/// [`Write`]: blocking::spi::Write
impl<P, M, L> blocking::spi::Write<u8> for Spi<P, M, L>
where
    P: ValidPads,
    P::Capability: Transmit,
    M: OpMode,
    L: NonAtomicSize,
{
    type Error = Error;

    #[inline]
    fn write(&mut self, buf: &[u8]) -> Result<(), Error> {
        if buf.len() != self.get_length() as usize {
            panic!("Slice length does not equal SPI transfer length");
        }
        if P::Capability::DYN == DynCapability::Duplex {
            blocking_write_non_atomic_duplex(self, buf)
        } else {
            blocking_write_non_atomic_tx(self, buf)
        }
    }
}

//=============================================================================
// blocking::spi::WriteIter
//=============================================================================

#[cfg(feature = "unproven")]
impl_blocking_spi_write_iter!(U1, U2, U3, U4);

//=============================================================================
// Helper functions
//=============================================================================

#[inline]
pub(super) fn blocking_transfer_non_atomic<'w, S: AnySpi>(
    spi: &mut S,
    buf: &'w mut [u8],
) -> Result<&'w [u8], Error> {
    let spi = spi.as_mut();
    let cells = Cell::from_mut(buf).as_slice_of_cells();
    let mut to_send = cells.iter();
    let mut to_recv = cells.iter();
    while to_recv.len() > 0 {
        let flags = spi.read_flags_errors()?;
        if to_send.len() > 0 && flags.contains(Flags::DRE) {
            let mut bytes = [0; 4];
            for byte in &mut bytes {
                match to_send.next() {
                    Some(cell) => *byte = cell.get(),
                    None => break,
                }
            }
            spi._write_data(u32::from_le_bytes(bytes));
        }
        if to_recv.len() > to_send.len() && flags.contains(Flags::RXC) {
            let bytes = spi._read_data().to_le_bytes();
            for byte in bytes.iter() {
                match to_recv.next() {
                    Some(cell) => cell.set(*byte),
                    None => break,
                }
            }
        }
    }
    Ok(buf)
}

#[inline]
pub(super) fn blocking_write_non_atomic_duplex<S: AnySpi>(
    spi: &mut S,
    buf: &[u8],
) -> Result<(), Error> {
    let spi = spi.as_mut();
    let mut to_send = buf.iter();
    let mut to_recv = to_send.len();
    while to_recv > 0 {
        let flags = spi.read_flags_errors()?;
        if to_send.len() > 0 && flags.contains(Flags::DRE) {
            let mut bytes = [0; 4];
            for byte in &mut bytes {
                match to_send.next() {
                    Some(d) => *byte = *d,
                    None => break,
                }
            }
            spi._write_data(u32::from_le_bytes(bytes));
        }
        if to_recv > to_send.len() && flags.contains(Flags::RXC) {
            spi._read_data();
            to_recv -= min(to_recv - to_send.len(), 4);
        }
    }
    Ok(())
}

#[inline]
pub(super) fn blocking_write_non_atomic_tx<S: AnySpi>(
    spi: &mut S,
    buf: &[u8],
) -> Result<(), Error> {
    let spi = spi.as_mut();
    let mut to_send = buf.iter();
    while to_send.len() > 0 {
        if spi.read_status().contains(Status::LENERR) {
            return Err(Error::LengthError);
        }
        if spi.read_flags().contains(Flags::DRE) {
            let mut bytes = [0; 4];
            for byte in &mut bytes {
                match to_send.next() {
                    Some(d) => *byte = *d,
                    None => break,
                }
            }
            spi._write_data(u32::from_le_bytes(bytes));
        }
    }
    Ok(())
}
