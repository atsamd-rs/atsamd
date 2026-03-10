//! # Device Service Unit
//!
//! This module allows users to interact with a DSU peripheral.
//!
//! - Run a CRC32 checksum over memory
//! - Run a memory test on RAM
//! - Check if a debugger is connected
#![warn(missing_docs)]
#![allow(clippy::doc_lazy_continuation)]
use atsamd_hal_macros::{hal_cfg, hal_macro_helper};

#[hal_cfg("dsu-d5x")]
use crate::pac::{self, Pac};

#[hal_cfg(any("dsu-d21", "dsu-d11"))]
use crate::pac::{self, Pac1};

/// Device Service Unit
pub struct Dsu {
    /// PAC peripheral
    dsu: pac::Dsu,
}

/// Errors from hardware
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PeripheralError {
    /// Usually misaligned address of length
    BusError,
}

/// Error from within the DSU
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Error {
    /// Address or length was not word aligned
    AlignmentError,
    /// The PAC would not unlock the DSU for us
    PacUnlockFailed,
    /// CRC32 operation failed
    CrcFailed,
    /// Hardware-generated errors
    Peripheral(PeripheralError),
    /// RAM Test failed
    RamTestFailed {
        /// Address of failed RAM
        addr: u32,
        /// Phase where MBIST failed (See datasheet)
        phase: u8,
        /// Bit in address which failed
        bit: u8,
    },
}

/// NVM result type
pub type Result<T> = core::result::Result<T, Error>;

impl Dsu {
    /// Unlock the DSU and instantiate peripheral
    #[inline]
    #[hal_cfg("dsu-d5x")]
    pub fn new(dsu: pac::Dsu, pac: &Pac) -> Result<Self> {
        // Attempt to unlock DSU
        pac.wrctrl()
            .write(|w| unsafe { w.perid().bits(33).key().clr() });

        // Check if DSU was unlocked
        if pac.statusb().read().dsu_().bit_is_set() {
            Err(Error::PacUnlockFailed)
        } else {
            Ok(Self { dsu })
        }
    }

    /// Unlock the DSU and instantiate peripheral
    #[inline]
    #[hal_cfg(any("dsu-d21", "dsu-d11"))]
    pub fn new(dsu: pac::Dsu, pac1: &Pac1) -> Result<Self> {
        // Attempt to unlock DSU
        pac1.wpclr().modify(|_, w| unsafe { w.bits(1 << 1) }); // Clear DSU protection
        // Check if DSU was unlocked
        if pac1.wpset().read().bits() & (1 << 1) != 0 {
            Err(Error::PacUnlockFailed)
        } else {
            Ok(Self { dsu })
        }
    }

    /// Releases the DSU peripheral
    pub fn free(self) -> pac::Dsu {
        self.dsu
    }

    /// Clear bus error bit
    #[inline]
    fn clear_bus_error(&mut self) {
        self.dsu.statusa().write(|w| w.berr().set_bit());
    }

    /// Read bus error bit
    #[inline]
    fn bus_error(&mut self) -> bool {
        self.dsu.statusa().read().berr().bit()
    }

    /// Check if operation is done
    #[inline]
    fn is_done(&self) -> bool {
        self.dsu.statusa().read().done().bit_is_set()
    }

    /// Check if an operation has failed
    #[inline]
    fn has_failed(&self) -> bool {
        self.dsu.statusa().read().fail().bit_is_set()
    }

    /// Set target address given as number of words offset
    #[inline]
    fn set_address(&mut self, address: u32) {
        self.dsu.addr().write(|w| unsafe { w.addr().bits(address) });
    }

    /// Set length given as number of words
    #[inline]
    fn set_length(&mut self, length: u32) {
        self.dsu
            .length()
            .write(|w| unsafe { w.length().bits(length) });
    }

    /// Seed CRC32
    #[inline]
    fn seed(&mut self, data: u32) {
        self.dsu.data().write(|w| unsafe { w.data().bits(data) });
    }

    /// Checks if a debugger is current connected
    #[inline]
    pub fn is_debugger_present(&self) -> bool {
        self.dsu.statusb().read().dbgpres().bit_is_set()
    }

    /// Calculate CRC32 of a memory region
    ///
    /// - `address` is an address within the CPUs memory space; must be
    ///  word-aligned
    /// - `length` is a length of memory region that is being processed.
    ///  Must be word-aligned
    #[hal_macro_helper]
    pub fn crc32(&mut self, address: u32, length: u32) -> Result<u32> {
        // The algorithm employed is the industry standard CRC32 algorithm using the
        // generator polynomial 0xEDB88320
        // (reversed representation of 0x04C11DB7).
        //
        // https://crccalc.com/, Hex input same as memory contents, Calc CRC-32
        // but output is reversed

        if address % 4 != 0 {
            return Err(Error::AlignmentError);
        }

        if length % 4 != 0 {
            return Err(Error::AlignmentError);
        }

        let num_words = length / 4;

        // Calculate target flash address
        let word_address = address / 4;

        // Set the ADDR of where to start calculation, as number of words
        self.set_address(word_address);

        // Amount of words to check
        self.set_length(num_words);

        // Set CRC32 seed
        self.seed(0xffff_ffff);

        #[hal_cfg("dsu-d21")]
        {
            // Errita for D21 silicon versions A-D
            if address > 0x20000000 {
                // Address is in RAM
                unsafe {
                    *(0x41007058 as *mut u32) &= !0x30000;
                }
            }
        }

        // Clear the status flags indicating termination of the operation
        self.dsu
            .statusa()
            .write(|w| w.done().set_bit().fail().set_bit());

        // Initialize CRC calculation
        self.dsu.ctrl().write(|w| w.crc().set_bit());

        // Wait until done or failed
        while !self.is_done() && !self.has_failed() {
            core::hint::spin_loop();
        }
        if self.has_failed() {
            return Err(Error::CrcFailed);
        }

        #[hal_cfg("dsu-d21")]
        {
            // Errita for D21 silicon versions A-D
            if address > 0x20000000 {
                // Address is in RAM
                unsafe {
                    *(0x41007058 as *mut u32) |= 0x20000;
                }
            }
        }

        // CRC startup generated a bus error
        // Generally misaligned length or address
        if self.bus_error() {
            // Return the reported bus error and clear it
            self.clear_bus_error();
            Err(Error::Peripheral(PeripheralError::BusError))
        } else {
            // Return the calculated CRC32 (complement of data register)
            Ok(!self.dsu.data().read().data().bits())
        }
    }

    /// Performs a memory test on a section of RAM using "March LR"
    /// algorithm.
    ///
    /// **CAUTION** This can overwrite critical data in RAM, use
    /// with caution
    ///
    /// ## Algorithm:
    /// 1. Write entire memory to '0', in any order.
    /// 2. Bit by bit read '0', write '1', in descending order.
    /// 3. Bit by bit read '1', write '0', read '0', write '1', in ascending
    ///    order.
    /// 4. Bit by bit read '1', write '0', in ascending order.
    /// 5. Bit by bit read '0', write '1', read '1', write '0', in ascending
    ///    order.
    /// 6. Read '0' from entire memory, in ascending order.
    ///
    ///
    /// - `address` is an address within the CPUs RAM space; must be
    ///   word-aligned
    /// - `length` is a length of memory region that is being tested. Must be
    ///   word-aligned
    pub unsafe fn memory_test(&mut self, address: u32, length: u32) -> Result<()> {
        if address % 4 != 0 {
            return Err(Error::AlignmentError);
        }

        if length % 4 != 0 {
            return Err(Error::AlignmentError);
        }

        let num_words = length / 4;

        // Calculate target flash address
        let word_address = address / 4;

        self.set_address(word_address);
        self.set_length(num_words);

        // Clear the status flags indicating termination of the operation
        self.dsu
            .statusa()
            .write(|w| w.done().set_bit().fail().set_bit());

        // Initialize RAM Test
        self.dsu.ctrl().write(|w| w.mbist().set_bit());

        // Wait until done or failed
        while !self.is_done() && !self.has_failed() {
            core::hint::spin_loop();
        }
        if self.has_failed() {
            let addr = self.dsu.addr().read().addr().bits();
            let data = self.dsu.data().read().data().bits();
            let bit = (data & 0b11111) as u8;
            let phase = ((data >> 8) & 0b111) as u8;
            return Err(Error::RamTestFailed { addr, phase, bit });
        }

        if self.bus_error() {
            // Return the reported bus error and clear it
            self.clear_bus_error();
            Err(Error::Peripheral(PeripheralError::BusError))
        } else {
            Ok(())
        }
    }
}
