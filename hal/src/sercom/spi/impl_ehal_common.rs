use core::cell::Cell;

use embedded_hal::{blocking, serial, spi};
use nb::Error::WouldBlock;
use num_traits::AsPrimitive;

use super::*;

//=============================================================================
// serial::Read
//=============================================================================

/// Implement [`serial::Read`] for [`Rx`] [`Spi`] structs in a [`MasterMode`]
///
/// `serial::Read` is only implemented for `Spi` structs with `Rx`
/// [`Capability`]. In a `MasterMode`, `Read` has to initiate transactions, so
/// it keeps track of the transaction state. If a transaction is in progress,
/// it will wait on `RXC`. If not, it will wait on `DRE`, and then send `0`.
impl<P, M, Z> serial::Read<Z::Word> for Spi<P, M, Z>
where
    P: ValidPads<Capability = Rx>,
    M: MasterMode,
    Z: AtomicSize,
    DataWidth: AsPrimitive<Z::Word>,
{
    type Error = Error;

    #[inline]
    fn read(&mut self) -> nb::Result<Z::Word, Error> {
        let in_progress = self.capability.in_progress;
        let flags = self.read_flags_errors()?;
        if !in_progress && flags.contains(Flags::DRE) {
            self._write_data(0);
            self.capability.in_progress = true;
            Err(WouldBlock)
        } else if in_progress && flags.contains(Flags::RXC) {
            self.capability.in_progress = false;
            Ok(self._read_data().as_())
        } else {
            Err(WouldBlock)
        }
    }
}

/// Implement [`serial::Read`] for [`Rx`] [`Spi`] structs in [`Slave`]
/// [`OpMode`]
///
/// `serial::Read` is only implemented for `Spi` structs with `Rx`
/// [`Capability`]. In `Slave` `OpMode`, `Read` does not have to initiate
/// transactions, so it does not have to store any internal state. It only has
/// to wait on `RXC`.
impl<P, Z> serial::Read<Z::Word> for Spi<P, Slave, Z>
where
    P: ValidPads<Capability = Rx>,
    Z: AtomicSize,
    DataWidth: AsPrimitive<Z::Word>,
{
    type Error = Error;

    #[inline]
    fn read(&mut self) -> nb::Result<Z::Word, Error> {
        let flags = self.read_flags_errors()?;
        if flags.contains(Flags::RXC) {
            Ok(self._read_data().as_())
        } else {
            Err(WouldBlock)
        }
    }
}

//=============================================================================
// serial::Write
//=============================================================================

/// Implement [`serial::Write`] for [`Tx`] [`Spi`] structs
///
/// `serial::Write` is only implemented for `Spi` structs with `Tx`
/// [`Capability`]. Because the `Capability` is `Tx`, this implementation never
/// reads the DATA register and ignores all buffer overflow errors.
impl<P, M, Z> serial::Write<Z::Word> for Spi<P, M, Z>
where
    P: ValidPads<Capability = Tx>,
    M: OpMode,
    Z: AtomicSize,
{
    type Error = Error;

    #[inline]
    fn write(&mut self, word: Z::Word) -> nb::Result<(), Error> {
        // Ignore buffer overflow errors
        if self.read_status().contains(Status::LENERR) {
            Err(Error::LengthError.into())
        } else if self.read_flags().contains(Flags::DRE) {
            self._write_data(word.as_());
            Ok(())
        } else {
            Err(WouldBlock)
        }
    }

    #[inline]
    fn flush(&mut self) -> nb::Result<(), Error> {
        // Ignore buffer overflow errors
        if self.read_status().contains(Status::LENERR) {
            Err(Error::LengthError.into())
        } else if self.read_flags().contains(Flags::TXC) {
            Ok(())
        } else {
            Err(WouldBlock)
        }
    }
}

//=============================================================================
// blocking::serial::Write
//=============================================================================

impl<P, M, Z> blocking::serial::write::Default<Z::Word> for Spi<P, M, Z>
where
    P: ValidPads<Capability = Tx>,
    M: OpMode,
    Z: AtomicSize,
    Spi<P, M, Z>: serial::Write<Z::Word>,
{
}

//=============================================================================
// spi::FullDuplex
//=============================================================================

/// Implement [`spi::FullDuplex`] for [`Spi`] structs with [`AtomicSize`]
///
/// `spi::FullDuplex` is only implemented when the `Spi` struct has [`Duplex`]
/// [`Capability`] and [`AtomicSize`]. The [`Word`] size used in the
/// implementation depends on the corresponding [`Size`].
impl<P, M, Z> spi::FullDuplex<Z::Word> for Spi<P, M, Z>
where
    P: ValidPads<Capability = Duplex>,
    M: OpMode,
    Z: AtomicSize,
    DataWidth: AsPrimitive<Z::Word>,
{
    type Error = Error;

    #[inline]
    fn read(&mut self) -> nb::Result<Z::Word, Error> {
        let flags = self.read_flags_errors()?;
        if flags.contains(Flags::RXC) {
            Ok(self._read_data().as_())
        } else {
            Err(WouldBlock)
        }
    }

    #[inline]
    fn send(&mut self, word: Z::Word) -> nb::Result<(), Error> {
        let flags = self.read_flags_errors()?;
        if flags.contains(Flags::DRE) {
            self._write_data(word.as_());
            Ok(())
        } else {
            Err(WouldBlock)
        }
    }
}

//=============================================================================
// Note on macros
//=============================================================================

// Macros are necessary for the following implementations of the embedded HAL
// `blocking` traits because of a limitation in the Rust trait system. The
// compiler can't seem to recongnize that the `blocking::spi::*::Default` traits
// can never be implemented for [`Spi`] in downstream crates, because that would
// violate the orphan rules.

//=============================================================================
// blocking::spi::Transfer
//=============================================================================

macro_rules! impl_blocking_spi_transfer {
    ( $($Size:ident),+ ) => {
        $(
            /// Implement [`Transfer`] for [`Spi`] structs that can [`Receive`]
            ///
            /// The transfer accepts a slice of primitive integers that varies
            /// depending on [`Size`].
            ///
            /// [`Transfer`]: blocking::spi::Transfer
            impl<P, M> blocking::spi::Transfer<Word<$Size>> for Spi<P, M, $Size>
            where
                P: ValidPads,
                P::Capability: Receive,
                M: OpMode,
            {
                type Error = Error;

                #[inline]
                fn transfer<'w>(&mut self, words: &'w mut [Word<$Size>]) -> Result<&'w [Word<$Size>], Error> {
                    blocking_transfer_atomic(self, words)
                }
            }
        )+
    }
}
pub(super) use impl_blocking_spi_transfer;

//=============================================================================
// blocking::spi::Write
//=============================================================================

macro_rules! impl_blocking_spi_write {
    ( $($Size:ident),+ ) => {
        $(

            /// Implement [`Write`] for [`Spi`] structs with [`Duplex`]
            /// [`Capability`] and an [`AtomicSize`]
            ///
            /// The transfer accepts a slice of primitive integers that varies
            /// depending on [`Size`].
            ///
            /// [`Write`]: blocking::spi::Write
            impl<P, M> blocking::spi::Write<Word<$Size>> for Spi<P, M, $Size>
            where
                P: ValidPads,
                P::Capability: Transmit,
                M: OpMode,
            {
                type Error = Error;

                #[inline]
                fn write(&mut self, words: &[Word<$Size>]) -> Result<(), Error> {
                    if P::Capability::DYN == DynCapability::Duplex {
                        blocking_write_atomic_duplex(self, words)
                    } else {
                        blocking_write_atomic_tx(self, words)
                    }
                }
            }
        )+
    }
}
pub(super) use impl_blocking_spi_write;

//=============================================================================
// blocking::spi::WriteIter
//=============================================================================

#[cfg(feature = "unproven")]
macro_rules! impl_blocking_spi_write_iter {
    ( $($Size:ident),+ ) => {
        $(
            /// Implement [`WriteIter`] for [`Spi`] structs with [`Duplex`]
            /// [`Capability`] and an [`AtomicSize`]
            ///
            /// The transfer accepts a slice of primitive integers that varies
            /// depending on [`Size`].
            ///
            /// [`WriteIter`]: blocking::spi::WriteIter
            #[cfg(feature = "unproven")]
            impl<P, M> blocking::spi::WriteIter<Word<$Size>> for Spi<P, M, $Size>
            where
                P: ValidPads,
                P::Capability: Transmit,
                M: OpMode,
            {
                type Error = Error;

                #[inline]
                fn write_iter<WI>(&mut self, words: WI) -> Result<(), Error>
                where
                    WI: IntoIterator<Item = Word<$Size>>,
                {
                    if P::Capability::DYN == DynCapability::Duplex {
                        blocking_write_iter_atomic_duplex(self, words)
                    } else {
                        blocking_write_iter_atomic_tx(self, words)
                    }
                }
            }
        )+
    };
}
#[cfg(feature = "unproven")]
pub(super) use impl_blocking_spi_write_iter;

//=============================================================================
// Helper functions
//=============================================================================

#[inline]
pub(super) fn blocking_transfer_atomic<'w, S>(
    spi: &mut S,
    words: &'w mut [S::Word],
) -> Result<&'w [S::Word], Error>
where
    S: AnySpi,
    DataWidth: AsPrimitive<S::Word>,
{
    let spi = spi.as_mut();
    let cells = Cell::from_mut(words).as_slice_of_cells();
    let mut to_send = cells.iter();
    let mut to_recv = cells.iter();
    while to_recv.len() > 0 {
        let flags = spi.read_flags_errors()?;
        if to_send.len() > 0 && flags.contains(Flags::DRE) {
            let word = match to_send.next() {
                Some(cell) => cell.get(),
                None => unreachable!(),
            };
            spi._write_data(word.as_());
        }
        if to_recv.len() > to_send.len() && flags.contains(Flags::RXC) {
            let word = spi._read_data().as_();
            match to_recv.next() {
                Some(cell) => cell.set(word),
                None => unreachable!(),
            }
        }
    }
    Ok(words)
}

#[inline]
pub(super) fn blocking_write_atomic_duplex<S: AnySpi>(
    spi: &mut S,
    words: &[S::Word],
) -> Result<(), Error> {
    let spi = spi.as_mut();
    let mut to_send = words.iter();
    let mut to_recv = to_send.len();
    while to_recv > 0 {
        let flags = spi.read_flags_errors()?;
        if to_send.len() > 0 && flags.contains(Flags::DRE) {
            let word = match to_send.next() {
                Some(word) => *word,
                None => unreachable!(),
            };
            spi._write_data(word.as_());
        }
        if to_recv > to_send.len() && flags.contains(Flags::RXC) {
            spi._read_data();
            to_recv -= 1;
        }
    }
    Ok(())
}

#[inline]
pub(super) fn blocking_write_atomic_tx<S: AnySpi>(
    spi: &mut S,
    words: &[S::Word],
) -> Result<(), Error> {
    let spi = spi.as_mut();
    for word in words {
        loop {
            if spi.read_status().contains(Status::LENERR) {
                return Err(Error::LengthError);
            }
            if spi.read_flags().contains(Flags::DRE) {
                spi._write_data(word.as_());
                break;
            }
        }
    }
    Ok(())
}

#[cfg(feature = "unproven")]
#[inline]
pub(super) fn blocking_write_iter_atomic_duplex<S, WI>(spi: &mut S, words: WI) -> Result<(), Error>
where
    S: AnySpi,
    WI: IntoIterator<Item = S::Word>,
{
    let spi = spi.as_mut();
    for word in words.into_iter() {
        loop {
            let flags = spi.read_flags_errors()?;
            if flags.contains(Flags::DRE) {
                spi._write_data(word.as_());
                break;
            }
        }
        loop {
            let flags = spi.read_flags_errors()?;
            if flags.contains(Flags::RXC) {
                spi._read_data();
                break;
            }
        }
    }
    Ok(())
}

#[cfg(feature = "unproven")]
#[inline]
pub(super) fn blocking_write_iter_atomic_tx<S, WI>(spi: &mut S, words: WI) -> Result<(), Error>
where
    S: AnySpi,
    WI: IntoIterator<Item = S::Word>,
{
    let spi = spi.as_mut();
    for word in words.into_iter() {
        loop {
            if spi.read_status().contains(Status::LENERR) {
                return Err(Error::LengthError);
            }
            if spi.read_flags().contains(Flags::DRE) {
                spi._write_data(word.as_());
                break;
            }
        }
    }
    Ok(())
}
