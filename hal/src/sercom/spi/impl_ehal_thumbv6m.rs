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
//! or using interrupts and the [`spi_future`](super::super::spi_future) module.
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

use embedded_hal::blocking;

use super::impl_ehal_common::*;
use super::*;

impl_blocking_spi_transfer!(EightBit, NineBit);

impl_blocking_spi_write!(EightBit, NineBit);

#[cfg(feature = "unproven")]
impl_blocking_spi_write_iter!(EightBit, NineBit);
