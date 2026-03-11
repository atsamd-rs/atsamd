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
    /// Command not allowed whilst in protected state
    ProtectionError,
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

    /// Clear statusa bits (all errors and done flag)
    #[inline]
    fn clear_status_a(&mut self) {
        self.dsu.statusa().write(|w| {
            w.berr().set_bit();
            w.perr().set_bit();
            w.done().set_bit();
            w.fail().set_bit()
        });
    }

    /// Read bus error bit
    #[inline]
    fn bus_error(&mut self) -> bool {
        self.dsu.statusa().read().berr().bit()
    }

    /// Read protection error bit
    #[inline]
    fn protection_error(&mut self) -> bool {
        self.dsu.statusa().read().perr().bit()
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

    /// Abort a current DSU operation (CRC32 or MBIST)
    #[inline]
    fn abort_operation(&self) {
        self.dsu.ctrl().write(|w| w.swrst().set_bit());
    }

    /// Setup memory length and address data, checking for word alignment
    #[inline]
    fn setup_mem_addr_regs(&mut self, address: u32, length: u32) -> Result<()> {
        if address % 4 != 0 {
            return Err(Error::AlignmentError);
        }

        if length % 4 != 0 {
            return Err(Error::AlignmentError);
        }

        self.set_address(address / 4);
        self.set_length(length / 4);
        Ok(())
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

        self.setup_mem_addr_regs(address, length)?;

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

        // Clear previous done/error status
        self.clear_status_a();

        // Initialize CRC calculation
        self.dsu.ctrl().write(|w| w.crc().set_bit());

        // Wait until done or failed
        while !self.is_done() {
            core::hint::spin_loop();
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

        if self.has_failed() {
            Err(Error::CrcFailed)
        } else if self.bus_error() {
            Err(Error::Peripheral(PeripheralError::BusError))
        } else if self.protection_error() {
            Err(Error::Peripheral(PeripheralError::ProtectionError))
        } else {
            // Return the calculated CRC32 (complement of data register)
            Ok(!self.dsu.data().read().data().bits())
        }
    }

    /// Performs a memory test on a section of RAM using "March LR"
    /// algorithm, blocking until the test is completed.
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
        self.setup_mem_addr_regs(address, length)?;

        self.clear_status_a();

        // Initialize RAM Test
        self.dsu.ctrl().write(|w| w.mbist().set_bit());

        // Wait until done
        while !self.is_done() {
            core::hint::spin_loop();
        }
        if self.has_failed() {
            let addr = self.dsu.addr().read().addr().bits();
            let data = self.dsu.data().read().data().bits();
            let bit = (data & 0b11111) as u8;
            let phase = ((data >> 8) & 0b111) as u8;
            Err(Error::RamTestFailed { addr, phase, bit })
        } else if self.bus_error() {
            Err(Error::Peripheral(PeripheralError::BusError))
        } else if self.protection_error() {
            Err(Error::Peripheral(PeripheralError::ProtectionError))
        } else {
            Ok(())
        }
    }

    /// Begins a DSU Memory test on a section of memory, returning
    /// a handle that can be polled for the result of the test.
    ///
    /// **CAUTION** This can overwrite critical data in RAM, use
    /// with caution
    ///
    /// This is useful for background memory check tasks when the
    /// CPU is at idle, as the test can be aborted immediately
    /// using the handle once the CPU receives an incomming
    /// interrupt.
    ///
    /// This should always be ran inside a critical-section
    /// to avoid the CPU context-switching mid memory test.
    ///
    /// (See [Self::memory_test] for how the algorithm works)
    ///
    /// - `address` is an address within the CPUs RAM space; must be
    ///   word-aligned
    /// - `length` is a length of memory region that is being tested. Must be
    ///   word-aligned
    pub unsafe fn polling_memory_test(
        &mut self,
        address: u32,
        length: u32,
    ) -> Result<PollingMemoryTest<'_>> {
        self.setup_mem_addr_regs(address, length)?;
        self.clear_status_a();
        // Initialize RAM Test
        self.dsu.ctrl().write(|w| w.mbist().set_bit());
        Ok(PollingMemoryTest { dsu: self })
    }
}

/// Handle to an ongoing background memory test
/// that can be aborted at any time
///
/// An example usage in an RTIC idle task
/// may look like the following:
///
/// ```
/// // dsu = rtic_sync::arbiter::Arbiter<Dsu>
/// #[idle(shared=[&dsu])]
/// fn idle(ctx: idle::Context) -> ! {
///     let mut ram_offset = 0;
///     // Important that this NEVER overwrites idle task stack
///
///     // Size of each RAM test (Per cycle)
///     const RAM_TEST_SIZE: usize = 128;
///     // Max address in RAM to test (Ensuring it doesn't
///     // overwrite the idle task stack)
///     const RAM_TEST_LEN: usize = 0xFFFF;
///     let mut ram_offset = 0;
///     let mut ram_buf = [0u8; RAM_TEST_SIZE];
///     let ram_buf_ptr = ram_buf.as_mut_ptr();
///     loop {
///         cortex_m::interrupt::free(|_| {
///             if let Some(mut lock) = ctx.shared.dsu.try_access() {
///                 unsafe {
///                     // Copy RAM to temp buffer
///                     let ram_ptr = ((0x2000_0000+ram_offset) as *const u8);
///                     ram_ptr.copy_to(ram_buf_ptr, RAM_TEST_SIZE);
///
///                     // Guaranteed alignment (So unwrap)
///                     let test = lock.polling_memory_test(0x2000_0000+ram_offset, RAM_TEST_LEN as u32).unwrap();
///                     // DSU does ram test whilst CPU is waiting
///                     wfi();
///                     // CPU woke up, abort running test, and check our results
///                     let test_res = test.finish_now();
///                     // Copy ram back
///                     ram_buf_ptr.copy_to(ram_buf_ptr, RAM_TEST_SIZE);
///                     match test_res {
///                         MemoryTestResult::Ok=> {
///                             ram_offset += 128;
///                             if (ram_offset > RAM_TEST_LEN) {
///                                 ram_offset = 0;
///                             }
///                         }
///                         MemoryTestResult::Aborted => {
///                             // Aborted, so don't increase counter
///                             // since the test wasn't finished
///                         },
///                         MemoryTestResult::Error(error) => {
///                             if let atsamd_hal::dsu::Error::RamTestFailed { addr, phase, bit } = error {
///                                 // Handle what happens when RAM test fails (Probably panic)
///                             }
///                         },
///                     }
///                 }
///             } else {
///                 // DSU not available so we just wait
///                 wfi();
///             }
///         })
///     }
/// }
/// ```
pub struct PollingMemoryTest<'a> {
    dsu: &'a mut Dsu,
}

/// Tri-state result for [PollingMemoryTest]
pub enum MemoryTestResult {
    /// Memory test OK
    Ok,
    /// Memory test aborted
    Aborted,
    /// Memory test error
    Error(Error),
}

impl<'a> PollingMemoryTest<'a> {
    /// Attempts to get the result of a memory test currently running,
    /// If the memory test is not yet completed when called,
    /// it is aborted, and the returned result will show this.
    pub fn finish_now(self) -> MemoryTestResult {
        if self.dsu.is_done() {
            if self.dsu.has_failed() {
                let addr = self.dsu.dsu.addr().read().addr().bits();
                let data = self.dsu.dsu.data().read().data().bits();
                let bit = (data & 0b11111) as u8;
                let phase = ((data >> 8) & 0b111) as u8;
                MemoryTestResult::Error(Error::RamTestFailed { addr, phase, bit })
            } else if self.dsu.bus_error() {
                MemoryTestResult::Error(Error::Peripheral(PeripheralError::BusError))
            } else if self.dsu.protection_error() {
                MemoryTestResult::Error(Error::Peripheral(PeripheralError::ProtectionError))
            } else {
                MemoryTestResult::Ok
            }
        } else {
            self.dsu.abort_operation();
            MemoryTestResult::Aborted
        }
    }

    /// Polls to see if the memory test is completed, without
    /// aborting it.
    ///
    /// Use [Self::finish_now] to obtain the result of the
    /// memory test
    #[inline]
    pub fn is_completed(&self) -> bool {
        self.dsu.is_done()
    }
}
