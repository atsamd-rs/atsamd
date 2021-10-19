//! Device Service Unit abstraction
//! TODO: this might be a good fit for generalizing over multiple chips later
//! on, but for now it will only be tested on the SAME54
#![deny(missing_docs)]
#![deny(warnings)]

use super::nvm::Bank;
use crate::target_device::{DSU, PAC};

/// Device Service Unit
pub struct Dsu {
    /// PAC peripheral
    dsu: DSU,
}

/// Errors from hardware
#[derive(Debug)]
pub enum PeripheralError {
    /// Usually misaligned address of length
    BusError,
}

/// Error from within the DSU
#[derive(Debug)]
pub enum Error {
    /// Address or length was not word aligned
    AlignmentError,
    /// The PAC would not unlock the DSU for us
    PacUnlockFailed,
    /// CRC32 operation failed
    CrcFailed,
    /// Hardware-generated errors
    Peripheral(PeripheralError),
}

/// NVM result type
pub type Result<T> = core::result::Result<T, Error>;

impl Dsu {
    /// Unlock the DSU and instantiate peripheral
    pub fn new(dsu: DSU, pac: &PAC) -> Result<Self> {
        // Attempt to unlock DSU
        pac.wrctrl
            .write(|w| unsafe { w.perid().bits(33).key().clr() });

        // Check if DSU was unlocked
        if pac.statusb.read().dsu_().bit_is_set() {
            Err(Error::PacUnlockFailed)
        } else {
            Ok(Self { dsu })
        }
    }

    /// Clear bus error bit
    fn clear_bus_error(&mut self) {
        self.dsu.statusa.write(|w| w.berr().set_bit());
    }

    /// Read bus error bit
    fn bus_error(&mut self) -> bool {
        self.dsu.statusa.read().berr().bit()
    }

    /// Check if operation is done
    fn is_done(&self) -> bool {
        self.dsu.statusa.read().done().bit_is_set()
    }

    /// Check if an operation has failed
    fn has_failed(&self) -> bool {
        self.dsu.statusa.read().fail().bit_is_set()
    }

    /// Set target address given as number of words offset
    fn set_address(&mut self, address: u32) -> Result<()> {
        self.dsu.addr.write(|w| unsafe { w.addr().bits(address) });
        Ok(())
    }

    /// Set length given as number of words
    fn set_length(&mut self, length: u32) -> Result<()> {
        self.dsu
            .length
            .write(|w| unsafe { w.length().bits(length) });
        Ok(())
    }

    /// Seed CRC32
    fn seed(&mut self, data: u32) {
        self.dsu.data.write(|w| unsafe { w.data().bits(data) });
    }

    /// Calculate CRC32 of a region
    /// `address` is an address within `bank`, which must be word-aligned and
    pub fn crc32(&mut self, bank: &Bank, address: u32, num_words: u32) -> Result<u32> {
        // The algorithm employed is the industry standard CRC32 algorithm using the
        // generator polynomial 0xEDB88320
        // (reversed representation of 0x04C11DB7).
        //
        // https://crccalc.com/, Hex input same as memory contents, Calc CRC-32
        // but output is reversed

        if address % 4 != 0 {
            return Err(Error::AlignmentError);
        }

        // Calculate bank offset
        let bank_address = bank.address() / 4;

        // Calculate target flash address
        let flash_address = bank_address + address / 4;

        // Set the ADDR of where to start calculation, as number of words
        self.set_address(flash_address)?;

        // Amount of words to check
        self.set_length(num_words)?;

        // Set CRC32 seed
        self.seed(0xffff_ffff);

        // Clear the status flags indicating termination of the operation
        self.dsu
            .statusa
            .write(|w| w.done().set_bit().fail().set_bit());

        // Initialize CRC calculation
        self.dsu.ctrl.write(|w| w.crc().set_bit());

        // Wait until done or failed
        while !self.is_done() && !self.has_failed() {}
        if self.has_failed() {
            return Err(Error::CrcFailed);
        }

        // CRC startup generated a bus error
        // Generally misaligned length or address
        if self.bus_error() {
            // Return the reported bus error and clear it
            self.clear_bus_error();
            Err(Error::Peripheral(PeripheralError::BusError))
        } else {
            // Return the calculated CRC32 (complement of data register)
            Ok(!self.dsu.data.read().data().bits())
        }
    }
}
