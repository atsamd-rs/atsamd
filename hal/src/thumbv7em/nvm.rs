//! # Non-volatile Memory Controller
//!
//! This module allows users to interact with non-volatile memory controller.
//!
//! NVMCTRL is an intermediary between memory buses and physical non-volatile
//! memory. It provides means of managing a flash memory content, its properties
//! (cache, wait states, bootloader blocks protection), power management and
//! address remapping if necessary (in case bank mechanism is used). It also
//! provides an indirection mechanism to achieve non-volatile RAM-like memory
//! within last sectors of a physical flash (More in [`smart_eeprom`] module).
//!
//! NVM supports splitting flash into two sections (opt-in feature) called
//! banks. Bank considered active is mapped to _virtual_ address `0x0`, meaning
//! it contains currently executed application. Through NVM command & control
//! interface, banks can be swapped and MCU reset, so the firmware from the
//! other bank will run after restart.
//!
//! Module features:
//! - Erase & write over non-volatile memory in a device.
//! - Swap banks
#![warn(missing_docs)]

pub mod smart_eeprom;

pub use crate::pac::nvmctrl::ctrla::PRM_A;
use crate::pac::nvmctrl::ctrlb::CMD_AW;
use crate::pac::NVMCTRL;
use core::num::NonZeroU32;
use core::ops::Range;
use core::ptr::addr_of;

use bitfield::bitfield;

/// Retrieve a total NVM size using HW registers
#[inline(always)]
pub fn retrieve_flash_size() -> u32 {
    static mut FLASHSIZE: Option<NonZeroU32> = None;
    // Safety: Lazy initialization of a static variable - interactions with
    // `Option<NonZeroU32>` should be atomic
    unsafe {
        match FLASHSIZE {
            Some(x) => x.into(),
            None => {
                let nvm = &*NVMCTRL::ptr();
                let nvm_params = nvm.param.read();
                if !nvm_params.psz().is_512() {
                    unreachable!("NVM page size is always expected to be 512 bytes");
                }
                let nvm_pages = nvm_params.nvmp().bits() as u32;
                let flash_size = nvm_pages * 512;
                // Safety: `flash_size` will never be 0
                FLASHSIZE = Some(NonZeroU32::new_unchecked(flash_size));
                flash_size
            }
        }
    }
}

/// Retrieve a bank size using HW registers
#[inline(always)]
pub fn retrieve_bank_size() -> u32 {
    retrieve_flash_size() / 2
}

/// Size of a page in bytes
pub const PAGESIZE: u32 = 512;

/// Size of one block
pub const BLOCKSIZE: u32 = 512 * 16;

/// Size of a quad word
pub const QUADWORDSIZE: u32 = 16;

/// Non-volatile memory controller
pub struct Nvm {
    /// PAC peripheral
    nvm: NVMCTRL,
}

/// Errors generated by the NVM peripheral
#[derive(Debug)]
pub enum PeripheralError {
    /// NVM error
    NvmError,
    /// Single ECC error
    EccSingleError,
    /// Dual ECC error
    EccDualError,
    /// Locked error
    LockError,
    /// Programming error
    ProgrammingError,
    /// Address error
    AddressError,
}

/// Driver errors
#[non_exhaustive]
#[derive(Debug)]
pub enum Error {
    /// Address range outside of flash
    NonFlash,
    /// Target sector is protected
    Protected,
    /// Memory region is used by SmartEEPROM
    SmartEepromArea,
    /// Errors generated by hardware
    Peripheral(PeripheralError),
    /// The DSU failed in some way
    Dsu(super::dsu::Error),
    /// An alignment requirement was not fulfilled
    Alignment,
}

/// Physical flash banks
#[derive(PartialEq, Debug)]
pub enum PhysicalBank {
    /// Flash bank A
    A,
    /// Flash bank B
    B,
}

#[derive(PartialEq, Debug, Copy, Clone)]
/// Flash banks identified by which one we boot from.
///
/// Memory layout:
/// ```text
/// [  Active bank  | Inactive bank ]
/// ^               ^               ^
/// 0x0000_0000     flash_size/2    flash_size
/// ```
pub enum Bank {
    /// Bank that is mapped to 0x0000_0000
    ///
    /// Active bank occupies first half of the flash memory.
    Active,
    /// Bank that is not mapped to 0x0000_0000
    ///
    /// Inactive bank occupies second half of the flash memory.
    Inactive,
}

impl Bank {
    /// Provides the address of the bank
    #[inline]
    pub fn address(&self) -> u32 {
        match self {
            Bank::Active => 0,
            Bank::Inactive => retrieve_bank_size(),
        }
    }

    /// Provides bank length in bytes
    #[inline]
    pub fn length(&self) -> u32 {
        retrieve_bank_size()
    }
}

/// NVM result type
pub type Result<T> = core::result::Result<T, Error>;

impl Nvm {
    /// Pointer to the userpage region of the flash memory
    ///
    /// Note: *Never* call `core::ptr::read_volatile` on this pointer, yields
    /// very poor codegen (around 9 KiB bytes of code)
    pub const USERPAGE_ADDR: *const [u8; 512] = 0x0080_4000 as _;

    /// Create a new NVM controller or handle failure from DSU
    #[inline]
    pub fn new(nvm: NVMCTRL) -> Self {
        Self { nvm }
    }

    /// Raw access to the registers.
    ///
    /// # Safety
    ///
    /// The abstraction assumes that it has exclusive ownership of the
    /// registers. Direct access can break such assumptions.
    pub unsafe fn registers(&self) -> &NVMCTRL {
        &self.nvm
    }

    /// Swap the flash banks. The processor will be reset, after which the
    /// inactive bank will become the active bank.
    ///
    /// # Safety
    ///
    /// Ensure there is a working, memory safe program in place in the inactive
    /// bank before calling.
    #[inline]
    pub unsafe fn bank_swap(&mut self) -> ! {
        let _ = self.command_sync(CMD_AW::BKSWRST);
        // The reset will happen atomically with the rest of the command, so getting
        // here is an error.
        unreachable!();
    }

    /// Set the power reduction mode
    #[inline]
    pub fn power_reduction_mode(&mut self, prm: PRM_A) {
        self.nvm.ctrla.modify(|_, w| w.prm().variant(prm));
    }

    /// Check if the flash is boot protected
    #[inline]
    pub fn is_boot_protected(&self) -> bool {
        !self.nvm.status.read().bpdis().bit()
    }

    /// Get first bank
    #[inline]
    pub fn first_bank(&self) -> PhysicalBank {
        if self.nvm.status.read().afirst().bit() {
            PhysicalBank::A
        } else {
            PhysicalBank::B
        }
    }

    /// Set address for reading/writing
    #[inline]
    fn set_address(&mut self, address: u32) {
        unsafe {
            self.nvm
                .addr
                .write(|w| w.addr().bits(address & 0x00ff_ffff));
        }
    }

    /// Execute a command, wait until it is done and check error states
    #[inline]
    fn command_sync(&mut self, command: CMD_AW) -> Result<()> {
        // Wait until STATUS.READY
        while !self.nvm.status.read().ready().bit() {}

        self.nvm
            .ctrlb
            .write(|w| w.cmdex().key().cmd().variant(command));

        // Wait until INTFLAG.DONE
        while !self.nvm.intflag.read().done().bit() {}
        // Clear INTFLAG.DONE
        self.nvm.intflag.write(|w| w.done().set_bit());

        self.manage_error_states()
    }

    /// Read the peripheral state to check error flags and clear the up
    /// afterwards
    #[inline]
    fn manage_error_states(&mut self) -> Result<()> {
        let read_intflag = self.nvm.intflag.read();
        // Check ADDRE and LOCKE first as it is more specific than PROGE
        let state = if read_intflag.addre().bit_is_set() {
            Err(Error::Peripheral(PeripheralError::AddressError))
        } else if read_intflag.locke().bit_is_set() {
            Err(Error::Peripheral(PeripheralError::LockError))
        } else if read_intflag.proge().bit_is_set() {
            Err(Error::Peripheral(PeripheralError::ProgrammingError))
        } else {
            Ok(())
        };

        // Clear error flags
        self.nvm
            .intflag
            .write(|w| w.addre().set_bit().locke().set_bit().proge().set_bit());

        state
    }

    /// Read the user page from the flash memory
    #[inline]
    pub fn read_userpage(&self) -> Userpage {
        let mut userpage = RawUserpage([0_u8; 512]);
        // Safety:
        // - Nvm is a singleton because it is constructed using the PAC singleton
        //   NVMCTRL.
        // - You need Nvm or NVMCTRL to modify the memory, so the &self singleton is
        //   enough to prevent concurrent modification.
        // - Underlying [u8; 512] has no reserved bit patterns.
        // - The pointer is aligned.
        // Note: userpage is accessed through the iterator in order to avoid poor
        // codegen for `read_volatile` call on the array pointer.
        userpage
            .0
            .iter_mut()
            .zip((0..512).map(|i| unsafe {
                Self::USERPAGE_ADDR
                    .cast::<u8>()
                    .wrapping_offset(i)
                    .read_volatile()
            }))
            .for_each(|(l, r)| *l = r);
        userpage
    }

    /// Modify the NVM User Page (aka User Row/UROW)
    ///
    /// User is expected to provide a closure that modifies the user page
    /// according to the user's needs.
    ///
    /// This method will read the current user page, call the closure on it,
    /// *erase the page in the flash memory* and *write it* back again.
    ///
    /// Erasure and flashing is skipped if the userpage stays the same after
    /// calling the closure on it.
    ///
    /// # Safety
    ///
    /// Even though factory calibration settings are not modifiable via setters
    /// they can be still mutated via raw access to the `userpage.0` field.
    ///
    /// Power loss between the erase and the write will result in *data loss*.
    ///
    /// Thus, users are advised to *backup factory calibration settings* before
    /// mutating the user page!
    ///
    /// If these settings are erased, device might stop behaving correctly!
    #[inline]
    pub unsafe fn modify_userpage(
        &mut self,
        f: impl FnOnce(&mut Userpage),
    ) -> Result<UserpageStatus> {
        let original = self.read_userpage();
        let mut modified = original.clone();

        f(&mut modified);

        if original != modified {
            unsafe { self.erase(NvmErase::Userpage)? };
            unsafe { self.write(NvmWrite::Userpage(&modified))? };

            Ok(UserpageStatus::Updated)
        } else {
            Ok(UserpageStatus::Skipped)
        }
    }

    /// Read the calibration area
    #[inline]
    pub fn calibration_area(&self) -> CalibrationArea {
        let mut buffer = 0_u64;
        let base_addr: *const u8 = 0x0080_0080 as *const u8;

        for i in 0..6 {
            buffer |=
                unsafe { core::ptr::read_volatile(base_addr.offset(i as isize)) as u64 } << (i * 8);
        }

        CalibrationArea(buffer)
    }

    /// Read the calibration area for temperatures
    #[inline]
    pub fn temperatures_calibration_area(&self) -> TemperaturesCalibrationArea {
        let mut buffer = 0_u128;
        let base_addr: *const u8 = 0x0080_0100 as *const u8;

        for i in 0..11 {
            buffer |= unsafe { core::ptr::read_volatile(base_addr.offset(i as isize)) as u128 }
                << (i * 8);
        }

        TemperaturesCalibrationArea(buffer)
    }

    /// Enable security bit
    ///
    /// It locks the chip from external access for code security. Consult the
    /// datasheet for more details.
    ///
    /// In order to disable it, chip erase command must be issued through the
    /// debugger.
    #[inline]
    pub fn enable_security_bit(&mut self) -> Result<()> {
        self.command_sync(CMD_AW::SSB)
    }

    /// Enable the chip erase lock
    ///
    /// It disables the chip erase capability.
    ///
    /// # Safety
    ///
    /// Together with [`Self::enable_security_bit`], it completely locks the MCU
    /// down from any external interaction via debugger, thus effectively
    /// *bricking the device*. Flashed firmware *must provide a way* to
    /// execute [`Self::disable_chip_erase_lock`] method in order to enable
    /// the debugger access again.
    #[inline]
    pub unsafe fn enable_chip_erase_lock(&mut self) -> Result<()> {
        self.command_sync(CMD_AW::CELCK)
    }

    /// Disable the chip erase lock
    ///
    /// It enables the chip erase capability through the debugger.
    #[inline]
    pub fn disable_chip_erase_lock(&mut self) -> Result<()> {
        self.command_sync(CMD_AW::CEULCK)
    }

    /// Enable/disable boot protection
    ///
    /// Userpage's NVM BOOT field defines a memory region that is supposed to be
    /// protected. `NVMCTRL.STATUS.BOOTPROT` is a readonly HW register populated
    /// on reset with a value from a userpage. By default, 0
    #[inline]
    pub fn boot_protection(&mut self, protect: bool) -> Result<()> {
        // Check if requested state differs from current state
        if self.is_boot_protected() != protect {
            // Requires both command and key so the command is allowed to execute
            if !protect {
                // Issue Set boot protection disable (disable bootprotection)
                self.command_sync(CMD_AW::SBPDIS)
            } else {
                // Issue Clear boot protection disable (enable bootprotection)
                self.command_sync(CMD_AW::CBPDIS)
            }
        } else {
            Ok(())
        }
    }

    /// Write to the main address space flash memory from a slice
    ///
    /// This call will fail if area that is being written to is
    /// - outside of the main address space flash area
    /// - write protected (BOOTPROT)
    /// - overlapping with SmartEEPROM flash region
    ///
    /// `destination` has to be 4 bytes aligned.
    ///
    /// # Safety
    ///
    /// Writes to the main address space flash area containing currently
    /// executed application are unsound.
    #[inline]
    pub unsafe fn write_flash_from_slice(
        &mut self,
        destination: *mut u32,
        source_slice: &[u32],
        write_granularity: WriteGranularity,
    ) -> Result<()> {
        let source = source_slice.as_ptr();
        let words = source_slice.len() as u32;

        // Safety: prerequisites bubbled up to the method signature
        unsafe {
            self.write(NvmWrite::MainAddressSpace {
                destination,
                source,
                words,
                write_granularity,
            })
        }
    }

    /// Write to the main address space flash memory
    ///
    /// This call will fail if area that is being written to is
    /// - outside of the main address space flash area
    /// - write protected (BOOTPROT)
    /// - overlapping with SmartEEPROM flash region
    ///
    /// `destination` has to be 4 bytes aligned.
    /// `source` has to be 4 bytes aligned.
    ///
    /// # Safety
    ///
    /// Writes to the main address space flash area containing currently
    /// executed application are unsound.
    #[inline]
    pub unsafe fn write_flash(
        &mut self,
        destination: *mut u32,
        source: *const u32,
        words: u32,
        write_granularity: WriteGranularity,
    ) -> Result<()> {
        // Safety: prerequisites bubbled up to the method signature
        unsafe {
            self.write(NvmWrite::MainAddressSpace {
                destination,
                source,
                words,
                write_granularity,
            })
        }
    }

    /// Write to the flash memory
    ///
    /// Failure modes regarding writes to the main address space flash
    /// area are mentioned in [`Self::write_flash`] documentation
    ///
    /// # Safety
    ///
    /// Safety requirements regarding writes to the main address space flash
    /// area are mentioned in [`Self::write_flash`] documentation
    ///
    /// Safety requirements regarding userpage modifications are mentioned in
    /// [`Self::modify_userpage`] documentation
    #[inline]
    unsafe fn write(&mut self, op: NvmWrite) -> Result<()> {
        let (destination_address, source_address, words, granularity) = match op {
            NvmWrite::MainAddressSpace {
                destination,
                source,
                words,
                write_granularity,
            } => (destination as u32, source as u32, words, write_granularity),
            NvmWrite::Userpage(userpage) => (
                Self::USERPAGE_ADDR as u32,
                addr_of!(userpage.0) as u32,
                PAGESIZE / core::mem::size_of::<u32>() as u32,
                WriteGranularity::QuadWord,
            ),
        };

        // Length of memory step
        let step_size = core::mem::size_of::<u32>() as u32;
        // Length of data in bytes
        let length = words * step_size;
        let write_size = granularity.size();

        let read_addresses = source_address..(source_address + length);
        let write_addresses = destination_address..(destination_address + length);

        if source_address % step_size != 0 {
            return Err(Error::Alignment);
        }

        if destination_address % step_size != 0 {
            return Err(Error::Alignment);
        }

        match op {
            NvmWrite::MainAddressSpace { .. } => {
                if self.contains_non_flash_memory_area(&write_addresses) {
                    return Err(Error::NonFlash);
                } else if self.contains_bootprotected(&write_addresses) {
                    return Err(Error::Protected);
                } else if self.contains_smart_eeprom(&write_addresses) {
                    return Err(Error::SmartEepromArea);
                }
            }
            NvmWrite::Userpage(_) => {
                // Nothing to check
            }
        }

        self.command_sync(CMD_AW::PBC)?;
        // Track whether we have unwritten data in the page buffer
        let mut dirty = false;
        // Zip two iterators, one counter and one with the addr word aligned
        for (destination_address, source_address) in write_addresses
            .step_by(step_size as usize)
            .zip(read_addresses.step_by(step_size as usize))
        {
            // Write to memory, 32 bits, 1 word.
            // The data is placed in the page buffer and ADDR is updated automatically.
            // Memory is not written until the write page command is issued later.
            let value = core::ptr::read_volatile(source_address as *const u32);
            core::ptr::write_volatile(destination_address as *mut u32, value);
            dirty = true;

            // If we are about to cross a page boundary (and run out of page buffer), write
            // to flash
            if destination_address % write_size >= write_size - step_size {
                // Perform a write
                self.command_sync(granularity.command())?;
                dirty = false;
            }
        }

        if dirty {
            // The dirty flag has fulfilled its role here, so we don't bother to maintain
            // its invariant anymore. Otherwise, the compiler would warn of
            // unused assignments. Write last page
            self.command_sync(granularity.command())?
        }

        Ok(())
    }

    /// Erase the portion of the main address space flash memory
    ///
    /// Erase granularity is expressed in blocks (16 pages == 8192 bytes)
    ///
    /// This call will fail if area that is being erased is
    /// - outside of the main address space flash area
    /// - write protected (BOOTPROT)
    /// - overlapping with SmartEEPROM flash region
    ///
    /// # Safety
    ///
    /// Erasure of the main address space flash area containing currently
    /// executed application is unsound.
    #[inline]
    pub unsafe fn erase_flash(&mut self, address: *mut u32, blocks: u32) -> Result<()> {
        // Safety: prerequisites bubbled up to the method signature
        unsafe { self.erase(NvmErase::Flash { address, blocks }) }
    }

    /// Erase the flash memory.
    ///
    /// Failure modes regarding erasure of the main address space flash
    /// area are mentioned in [`Self::erase_flash`] documentation
    ///
    /// # Safety
    ///
    /// Safety requirements regarding erasure of the main address space flash
    /// area are mentioned in [`Self::erase_flash`] documentation
    ///
    /// Safety requirements regarding userpage modifications are mentioned in
    /// [`Self::modify_userpage`] documentation
    #[inline]
    unsafe fn erase(&mut self, op: NvmErase) -> Result<()> {
        let (address, length, granularity) = match op {
            NvmErase::Flash { address, blocks } => {
                (address as u32, blocks, EraseGranularity::Block)
            }
            NvmErase::Userpage => (Self::USERPAGE_ADDR as u32, 1, EraseGranularity::Page),
        };

        // Align to block/page boundary
        // While the NVM will accept any address in the block, we need to compute the
        // aligned address to check for boot protection.
        let flash_address = address - address % granularity.size();
        let range_to_erase = flash_address..(flash_address + length * granularity.size());

        match op {
            NvmErase::Flash { .. } => {
                if self.contains_non_flash_memory_area(&range_to_erase) {
                    return Err(Error::NonFlash);
                } else if self.contains_bootprotected(&range_to_erase) {
                    return Err(Error::Protected);
                } else if self.contains_smart_eeprom(&range_to_erase) {
                    return Err(Error::SmartEepromArea);
                }
            }
            NvmErase::Userpage => {
                // Nothing to check
            }
        }

        for address in range_to_erase.step_by(granularity.size() as usize) {
            // Set target address to current block/page offset
            self.set_address(address);

            // Erase block/page, wait for completion
            self.command_sync(granularity.command())?
        }

        Ok(())
    }

    #[inline]
    fn contains_bootprotected(&self, input: &Range<u32>) -> bool {
        // Calculate size that is protected for bootloader
        //   * 15 = no bootprotection, default value
        //   * 0 = max bootprotection, 15 * 8Kibyte = 120KiB
        //   * (15 - bootprot) * 8KiB = protected size
        let bootprot = self.nvm.status.read().bootprot().bits();
        let bp_space = 8 * 1024 * (15 - bootprot) as u32;

        let boot = &(Bank::Active.address()..(Bank::Active.address() + bp_space));
        self.is_boot_protected() && range_overlap(input, boot)
    }

    #[inline]
    fn contains_smart_eeprom(&self, input: &Range<u32>) -> bool {
        let smart_eeprom_allocated_blocks = self.nvm.seestat.read().sblk().bits() as u32;
        let smart_eeprom_end = Bank::Inactive.address() + Bank::Inactive.length();
        let smart_eeprom_start = smart_eeprom_end - smart_eeprom_allocated_blocks * BLOCKSIZE;
        let smart_eeprom = &(smart_eeprom_start..smart_eeprom_end);
        range_overlap(input, smart_eeprom)
    }

    #[inline]
    fn contains_non_flash_memory_area(&self, input: &Range<u32>) -> bool {
        input.end > retrieve_flash_size()
    }

    /// Retrieve SmartEEPROM
    #[inline]
    pub fn smart_eeprom(&mut self) -> smart_eeprom::Result<'_> {
        smart_eeprom::SmartEepromMode::retrieve(self)
    }
}

/// The outcome of [`Nvm::modify_userpage`]
#[derive(Copy, Clone, Debug)]
pub enum UserpageStatus {
    /// Userpage has been updated
    Updated,
    /// Update has been skipped; expected value is already present.
    Skipped,
}

enum NvmWrite<'a> {
    /// Writes to the flash memory within the main address space
    MainAddressSpace {
        destination: *mut u32,
        source: *const u32,
        words: u32,
        write_granularity: WriteGranularity,
    },
    /// Writes the userpage to the corresponding flash memory
    Userpage(&'a Userpage),
}

enum NvmErase {
    Flash { address: *mut u32, blocks: u32 },
    Userpage,
}

/// Data erased per command
#[derive(Copy, Clone, Debug)]
enum EraseGranularity {
    /// One block. This erase type is supported by main memory
    Block,
    /// One page. This erase type is supported for the AUX memory
    Page,
}

/// Data written per command
#[derive(Copy, Clone, Debug)]
pub enum WriteGranularity {
    /// Four words (16 bytes). Expected for user page writes
    QuadWord,
    /// One page (512 bytes)
    Page,
}

impl EraseGranularity {
    #[inline]
    fn command(&self) -> CMD_AW {
        match self {
            Self::Block => CMD_AW::EB,
            Self::Page => CMD_AW::EP,
        }
    }

    #[inline]
    fn size(&self) -> u32 {
        match self {
            Self::Block => BLOCKSIZE,
            Self::Page => PAGESIZE,
        }
    }
}

impl WriteGranularity {
    #[inline]
    fn command(&self) -> CMD_AW {
        match self {
            Self::QuadWord => CMD_AW::WQW,
            Self::Page => CMD_AW::WP,
        }
    }

    #[inline]
    fn size(&self) -> u32 {
        match self {
            Self::QuadWord => QUADWORDSIZE,
            Self::Page => PAGESIZE,
        }
    }
}

fn range_overlap(a: &Range<u32>, b: &Range<u32>) -> bool {
    a.start < b.end && b.start < a.end
}

/// Type alias to the userpage with a concrete underlying storage type
pub type Userpage = RawUserpage<[u8; 512]>;

bitfield! {
    /// Raw userpage POD struct that exposes bitfields via methods
    #[derive(Clone, PartialEq, Eq)]
    pub struct RawUserpage([u8]);
    impl Debug;
    u8;
    /// Access the `bod33_disable` field
    pub bod33_disable, set_bod33_disable: 0;
    /// Access the `bod33_level` field
    pub bod33_level, set_bod33_level: 8, 1;
    /// Access the `bod33_action` field
    pub bod33_action, set_bod33_action: 10, 9;
    /// Access the `bod33_hysteresis` field
    pub bod33_hysteresis, set_bod33_hysteresis: 14, 11;
    /// Access the `bod12_calibration_parameters` field
    pub u16, bod12_calibration_parameters, set_bod12_calibration_parameters: 25, 15;
    /// Access the `nvm_bootloader_size` field
    pub nvm_bootloader_size, set_nvm_bootloader_size: 29, 26;
    /// Access the `reserved_0` field
    pub reserved_0, set_reserved_0: 31, 30;
    /// Access the `see_sblk` field
    pub see_sblk, set_see_sblk: 35, 32;
    /// Access the `see_psz` field
    pub see_psz, set_see_psz: 38, 36;
    /// Access the `ram_ecc_disable` field
    pub ram_ecc_disable, set_ram_ecc_disable: 39;
    /// Access the `reserved_1` field
    pub reserved_1, set_reserved_1: 47, 40;
    /// Access the `wdt_enable` field
    pub wdt_enable, set_wdt_enable: 48;
    /// Access the `wdt_always_on` field
    pub wdt_always_on, set_wdt_always_on: 49;
    /// Access the `wdt_period` field
    pub wdt_period, set_wdt_period: 53, 50;
    /// Access the `wdt_window` field
    pub wdt_window, set_wdt_window: 57, 54;
    /// Access the `wdt_ewoffset` field
    pub wdt_ewoffset, set_wdt_ewoffset: 61, 58;
    /// Access the `wdt_wen` field
    pub wdt_wen, set_wdt_wen: 62;
    /// Access the `reserved_2` field
    pub reserved_2, set_reserved_2: 63;
    /// Access the `nvm_locks` field
    pub u32, nvm_locks, set_nvm_locks: 95, 64;
    /// Access the `userpage_0` field
    pub u32, userpage_0, set_userpage_0: 127, 96;
    /// Access the `reserved_3` field
    pub u32, reserved_3, set_reserved_3: 159, 128;
}

impl<T: AsRef<[u8]>> RawUserpage<T> {
    /// Access the general purpose user-writable section of the userpage via
    /// slice
    #[inline]
    pub fn userpage1_as_slice(&self) -> &[u8] {
        // `userpage1` starts on 160th bit (20th byte) and continues till the end of
        // the userpage
        &self.0.as_ref()[20..512]
    }
}
impl<T: AsMut<[u8]>> RawUserpage<T> {
    /// Access the general purpose user-writable section of the userpage via
    /// mutable slice
    #[inline]
    pub fn userpage1_as_slice_mut(&mut self) -> &mut [u8] {
        // `userpage1` starts on 160th bit (20th byte) and continues till the end of
        // the userpage
        &mut self.0.as_mut()[20..512]
    }
}

bitfield! {
    #[derive(Copy, Clone, Default)]
    /// POD-style struct representing NVM calibration area
    pub struct CalibrationArea(u64);
    impl Debug;
    u32;
    /// Access the `ac_bias` field. Setter is not provided.
    pub ac_bias, _: 1, 0;
    /// Access the `adc0_biascomp` field. Setter is not provided.
    pub adc0_biascomp, _: 4, 2;
    /// Access the `adc0_biasrefbuf` field. Setter is not provided.
    pub adc0_biasrefbuf, _: 7, 5;
    /// Access the `adc0_biasr2r` field. Setter is not provided.
    pub adc0_biasr2r, _: 10, 8;
    /// Access the `adc1_biascomp` field. Setter is not provided.
    pub adc1_biascomp, _: 18, 16;
    /// Access the `adc1_biasrefbuf` field. Setter is not provided.
    pub adc1_biasrefbuf, _: 21, 19;
    /// Access the `adc1_biasr2r` field. Setter is not provided.
    pub adc1_biasr2r, _: 24, 22;
    /// Access the `usb_transn` field. Setter is not provided.
    pub usb_transn, _: 36, 32;
    /// Access the `usb_transp` field. Setter is not provided.
    pub usb_transp, _: 41, 37;
    /// Access the `usb_trim` field. Setter is not provided.
    pub usb_trim, _: 44, 42;
}

bitfield! {
    #[derive(Copy, Clone, Default)]
    /// POD-style struct representing NVM calibration area for
    /// temperature calibration
    pub struct TemperaturesCalibrationArea(u128);
    impl Debug;
    u32;
    /// Access the `tli` field. Setter is not provided.
    pub tli, _: 7, 0;
    /// Access the `tld` field. Setter is not provided.
    pub tld, _: 11, 8;
    /// Access the `thi` field. Setter is not provided.
    pub thi, _: 19, 12;
    /// Access the `thd` field. Setter is not provided.
    pub thd, _: 23, 20;
    /// Access the `vpl` field. Setter is not provided.
    pub vpl, _: 51, 40;
    /// Access the `vph` field. Setter is not provided.
    pub vph, _: 63, 52;
    /// Access the `vcl` field. Setter is not provided.
    pub vcl, _: 75, 63;
    /// Access the `vch` field. Setter is not provided.
    pub vch, _: 87, 76;
}
