//! # ICM - Integrity Check Module
//!
//! Used to calculate SHA digests of memory regions
//!
//! Multiple modes available
//!
//! * Manual monitor of Internal SRAM (both contiguous and non-contiguous
//!   memory)
//! * Active monitoring of Internal SRAM (both contiguous and non-contiguous
//!   memory)
//! * Manual monitor of Internal Flash (both contiguous memory)
//! * Generates Hash using SHA engine, useful for verifying content
//! * ICM module has additional register protection and tamper detection
//!
//! Reading the Interrupt Status Register (ISR) clears the register,
//! to provide a workaround for cases where multiple bits needs parsing,
//! the [`Icm::get_interrupt_status()`] and
//! [`Region<I>::get_interrupt_status()`] are provided.
//! These return a queryable structure containing the interrupt register
//! contents. Allowing multiple different interrupts to be read.
//!
//! >**IMPORTANT** - Memory safety considerations
//! >
//! >The ICM engine accesses the assigned `DSCR` memory address, so it must be
//! >available. Depending on the application, this might entail making
//! >[`Regions`] **static**.
//! >
//! >The same goes for [`HashArea`], but here it is even more **important** to
//! >ensure the memory is designated for `HashArea` usage, since the ICM
//! >controller will, depending on ICM configuration, write data to that
//! >address.
//! >
//! >Setting [`HashArea`] **static** might be the safest path.
//! >
//! > Another alternative is to utilise the singleton macro provided by
//! > [`cortex_m::singleton`](https://docs.rs/cortex-m/latest/cortex_m/macro.singleton.html)
//! > ```no_run
//! > # use atsamd_hal::{pac::Peripherals, icm::*};
//! > use cortex_m::singleton;
//! >
//! > let hasharea: &'static mut HashArea =
//! > singleton!(: HashArea = HashArea::default()).unwrap();
//! > ```

//!
//! ## Usage:
//!
//! ### General ICM setup
//!
//! Initialise the ICM engine [`Icm::new()`] and reset ICM via [`Icm::swrst()`]
//!
//! Change any of the global options such as [`Icm::set_eomdis()`], if required.
//!
//! Enable and create the interface for required memory regions
//! [`Icm::enable_region0()`] and enable it via
//! [`Region::enable_monitoring()`]
//!
//! Depending on the number of regions required, the helper
//! [`Regions::default()`] alows setting up all 4 regions directly, if one
//! region is sufficient, manually create  [`MainRegionDesc<Region0>::
//! default()`].
//!
//!  Modify the [`MainRegionDesc`], see documentation and cargo doc for all
//! methods.
//!
//!  Set the `DSCR` address to the beginning of the [`MainRegionDesc`] via
//!  [`Icm::set_dscr_addr()`] (or via helper in
//! [`MainRegionDesc<Region0>::set_dscr_addr()`])
//!
//!  Via [`Region`], setup the desired interrupts depending on usecase.
//!
//!  To view which interrupts has been enabled in the debugger, check the
//! `ICM->IMR` register.
//!
//!  Any object in memory can be used as the "Hash" area, but for convenience
//! the provided  [`HashArea`] allows indexing of the 4 regions and is
//! correctly memory aligned.
//!
//!  Set the pointer to [`HashArea`] via [`Icm::set_hash_addr()`]
//!
//!  **See note about memory safety above**
//!
//! ### Hash calculation
//!
//! Assuming general setup is already done, modify the [`RegionConfiguration`]
//! which is part of the [`MainRegionDesc`]:
//!
//! * [`RegionConfiguration::set_rhien()`] to `false` to allow interrupts when
//!   calculation is done
//! * [`RegionConfiguration::set_eom()`] to `true` only for the last region
//!
//! Change [`RegionAddress`] to point to the object to SHA-sum with
//! [`MainRegionDesc<RegionNumT>::set_region_address()`]
//!
//! ### Memory monitoring
//!
//! [`HashArea`] needs to contain the expected SHA-sums of the data to
//! monitor, [`Icm::set_ascd()`] is provided to help with creating this data.
//! Alternatively do it manually and then change mode, or prepopulate the
//! [`HashArea`] with SHA-sums.
//!
//! Assuming general setup is already done, modify the [`RegionConfiguration`]
//! which is part of the [`MainRegionDesc`]:
//!
//! * [`RegionConfiguration::set_dmien()`] to `false` to allow interrupts if
//!   mismatch occurs
//! * [`RegionConfiguration::set_cdwbn()`] to `true` to change to monitor mode
//! * [`RegionConfiguration::set_wrap()`] to `true` only for the last region if
//!   continuous monitoring is desired
//!
//! ## Examples
//!
//! ### Calculate SHA1, SHA224 and SHA256 sums, then switch to memory monitor
//!
//! 4 memory regions, SHA1 in region0 and region1.
//! Region2 uses SHA224 and region3 SHA256.
//!
//! This only covers the setup part, to achieve the functionality of first
//! computing the SHA-sums and then do region monitoring handling of
//! interrupts and changing mode is required.
//!
//! ```no_run
//! # use atsamd_hal::{pac::Peripherals, icm::*};
//!
//! // SHA Test data
//! static MESSAGE_REF0: [u32; 16] = [
//!     0x11111111, 0x22222222, 0x33333333, 0x44444444, 0x55555555, 0x66666666, 0x77777777, 0x88888888,
//!     0x99999999, 0xaaaaaaaa, 0xbbbbbbbb, 0xcccccccc, 0xdddddddd, 0xeeeeeeee, 0xffffffff, 0x00000000,
//! ];
//!
//! static MESSAGE_REF1: [u32; 16] = [
//!     0x80636261, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
//!     0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x18000000,
//! ];
//!
//! // Expected SHA1 sum result
//! static MESSAGE_SHA1_RES: [u32; 8] = [
//!     0x363e99a9, 0x6a810647, 0x71253eba, 0x6cc25078, 0x9dd8d09c, 0x00000000, 0x00000000, 0x00000000,
//! ];
//!
//! static MESSAGE_SHA224_RES: [u32; 8] = [
//!     0x227d0923, 0x22d80534, 0x77a44286, 0xb355a2bd, 0xe4bcad2a, 0xf7b3a0bd, 0xa79d6ce3, 0x00000000,
//! ];
//! static MESSAGE_SHA256_RES: [u32; 8] = [
//!     0xbf1678ba, 0xeacf018f, 0xde404141, 0x2322ae5d, 0xa36103b0, 0x9c7a1796, 0x61ff10b4, 0xad1500f2,
//! ];
//! static mut HASH: HashArea = HashArea::default();
//! static mut ICM_REGION_DESC: Regions = Regions::default();
//!
//! // Alternatively
//! //use cortex_m::singleton;
//! //let hasharea: &'static mut HashArea = singleton!(: HashArea = HashArea::default()).unwrap();
//!
//! // Enable ICM apb clock
//! // Clock v1
//! //mclk.apbcmask.modify(|_, w| w.icm_().set_bit());
//! // Clock v2
//! //tokens.apbs.icm.enable();
//!
//! let mut peripherals = Peripherals::take().unwrap();
//!
//! // Create new ICM
//! let mut icm = Icm::new(peripherals.ICM);
//!
//! // Reset the ICM, clearing past error states
//! icm.swrst();
//!
//! // End of Monitoring is permitted
//! icm.set_eomdis(false);
//! // Write Back is permitted
//! icm.set_wbdis(false);
//! // Secondary List branching is forbidden
//! icm.set_slbdis(false);
//! // Automatic Switch to Compare is disabled
//! icm.set_ascd(false);
//!
//! // Region Descriptor
//! let mut icm_region_desc = Regions::default();
//!
//! // Get the interface for Region0 and enable monitoring
//! let icm_region0 = icm.enable_region0();
//! icm_region0.enable_monitoring();
//!
//! // Setup desired interrupts
//! //
//! // Region Hash Completed
//! icm_region0.set_rhc_int();
//!
//! // Region0 raddr
//! icm_region_desc.region0.set_region_address(MESSAGE_REF0.as_ptr());
//!
//! // Configure the RCFG
//!
//! // Some are default values, just as an example
//!
//! // Activate Write back (should be true when comparing memory)
//! icm_region_desc.region0.rcfg.set_cdwbn(false);
//! // Should the ICM controller loop back to DSCR after this region?
//! icm_region_desc.region0.rcfg.set_wrap(false);
//! // Set this as the end of descriptor linked list
//! icm_region_desc.region0.rcfg.set_eom(false);
//! // The RHC flag is set when the field NEXT = 0
//! // in a descriptor of the main or second list
//! icm_region_desc.region0.rcfg.set_rhien(false);
//! // Set Algorithm to SHA1
//! icm_region_desc.region0.rcfg.set_algo(icm_algorithm::SHA1);
//!
//! // Get the interface for region1
//! let icm_region1 = icm.enable_region1();
//!
//! // Enable region monitoring
//! icm_region1.enable_monitoring();
//!
//! // Setup desired interrupts
//! //
//! // Region Hash Completed
//! icm_region1.set_rhc_int();
//!
//! // Region1 raddr
//! icm_region_desc.region1.set_region_address(MESSAGE_REF1.as_ptr());
//!
//! // Configure the RCFG
//! // The RHC flag is set when the field NEXT = 0
//! // in a descriptor of the main or second list
//! icm_region_desc.region1.rcfg.set_rhien(false);
//! // Set Algorithm to SHA1
//! icm_region_desc.region1.rcfg.set_algo(icm_algorithm::SHA1);
//!
//! // Get the interface for region2
//! let icm_region2 = icm.enable_region2();
//!
//! // Enable region monitoring
//! icm_region2.enable_monitoring();
//!
//! // Setup desired interrupts
//! //
//! // Region Hash Completed
//! icm_region2.set_rhc_int();
//!
//! // Region2 raddr
//! icm_region_desc.region2.set_region_address(MESSAGE_REF1.as_ptr());
//!
//! // Configure the RCFG
//! // The RHC flag is set when the field NEXT = 0
//! // in a descriptor of the main or second list
//! icm_region_desc.region2.rcfg.set_rhien(false);
//! // Set Algorithm to SHA224
//! icm_region_desc.region2.rcfg.set_algo(icm_algorithm::SHA224);
//!
//! // Get the interface for region3
//! let icm_region3 = icm.enable_region3();
//!
//! // Enable region monitoring
//! icm_region3.enable_monitoring();
//!
//! // Setup desired interrupts
//! //
//! // Region Hash Completed
//! icm_region3.set_rhc_int();
//!
//! // Region3 raddr
//! icm_region_desc.region3.set_region_address(MESSAGE_REF1.as_ptr());
//!
//! // Configure the RCFG
//! //
//! // Set this as the end of descriptor linked list
//! icm_region_desc.region3.rcfg.set_eom(true);
//! // The RHC flag is set when the field NEXT = 0
//! // in a descriptor of the main or second list
//! icm_region_desc.region3.rcfg.set_rhien(false);
//! // Set Algorithm to SHA256
//! icm_region_desc.region3.rcfg.set_algo(icm_algorithm::SHA256);
//!
//! unsafe {
//!     // Hash Area
//!     // Set HASH addr to the beginning of the Hash area
//!     icm.set_hash_addr(&HASH);
//! }
//!
//! unsafe {
//!     // Move the icm_region_desc into static
//!     ICM_REGION_DESC = icm_region_desc;
//!     // Set DSCR to the beginning of the region descriptor
//!     icm.set_dscr_addr(&ICM_REGION_DESC.region0);
//!     // the same but via helper function
//!     //ICM_REGION_DESC.region0.set_dscr_addr(&icm);
//! }
//!
//! // Start the ICM calculation
//! icm.enable();
//!
//! // Setup memory region monitoring
//! // Monitor all 4 memory regions
//!
//! // Setup the compare regions
//! let mut message_region0_sha1 = MESSAGE_REF0;
//! let mut message_region1_sha1 = MESSAGE_REF1;
//! let mut message_region2_sha224 = MESSAGE_REF1;
//! let mut message_region3_sha256 = MESSAGE_REF1;
//!
//! // Reset the ICM, clearing past error states
//! icm.swrst();
//!
//! // End of Monitoring is permitted
//! icm.set_eomdis(false);
//! // Write Back is permitted
//! icm.set_wbdis(false);
//! // Secondary List branching is forbidden
//! icm.set_slbdis(false);
//! // Automatic Switch to Compare is disabled
//! icm.set_ascd(false);
//!
//! // Also possible to directly edit `ICM_REGION_DESC`
//! // in an unsafe block
//! let mut icm_region_desc = Regions::default();
//!
//! // Setup region 0 to monitor memory
//! icm_region_desc
//!     .region0
//!     .set_region_address(&message_region0_sha1);
//! icm_region_desc.region0.rcfg.reset_region_configuration_to_default();
//! icm_region_desc.region0.rcfg.set_algo(icm_algorithm::SHA1);
//! // Activate Compare Digest (should be true when comparing memory)
//! icm_region_desc.region0.rcfg.set_cdwbn(true);
//! // Digest Mismatch Interrupt Disable (enabled)
//! icm_region_desc.region0.rcfg.set_dmien(false);
//!
//! // Set Region Mismatch Interrupt
//! icm_region0.set_rdm_int();
//!
//! // Setup region 1 to monitor memory
//! icm_region_desc
//!     .region1
//!     .set_region_address(&message_region1_sha1);
//! icm_region_desc.region1.rcfg.reset_region_configuration_to_default();
//! icm_region_desc.region1.rcfg.set_algo(icm_algorithm::SHA1);
//! // Activate Compare Digest (should be true when comparing memory)
//! icm_region_desc.region1.rcfg.set_cdwbn(true);
//! // Digest Mismatch Interrupt Disable (enabled)
//! icm_region_desc.region1.rcfg.set_dmien(false);
//!
//! // Set Region Mismatch Interrupt
//! icm_region1.set_rdm_int();
//!
//! // Setup region 2 to monitor memory
//! icm_region_desc
//!     .region2
//!     .set_region_address(&message_region2_sha224);
//! icm_region_desc.region2.rcfg.reset_region_configuration_to_default();
//! icm_region_desc.region2.rcfg.set_algo(icm_algorithm::SHA224);
//! // Activate Compare Digest (should be true when comparing memory)
//! icm_region_desc.region2.rcfg.set_cdwbn(true);
//! // Digest Mismatch Interrupt Disable (enabled)
//! icm_region_desc.region2.rcfg.set_dmien(false);
//!
//! // Set Region Mismatch Interrupt
//! icm_region2.set_rdm_int();
//!
//! // Setup region 3 to monitor memory
//! icm_region_desc
//!     .region3
//!     .set_region_address(&message_region3_sha256);
//! icm_region_desc.region3.rcfg.reset_region_configuration_to_default();
//! icm_region_desc.region3.rcfg.set_algo(icm_algorithm::SHA256);
//! // Activate Compare Digest (should be true when comparing memory)
//! icm_region_desc.region3.rcfg.set_cdwbn(true);
//! // Digest Mismatch Interrupt Disable (enabled)
//! icm_region_desc.region3.rcfg.set_dmien(false);
//! // Wrap
//! icm_region_desc.region3.rcfg.set_wrap(true);
//!
//! // Set Region Mismatch Interrupt
//! icm_region3.set_rdm_int();
//!
//! // Modify regions to trigger interrupts
//! message_region0_sha1[3] = 0xDEAD_BEEF;
//! message_region1_sha1[4] = 0xDEAD_BEEF;
//! message_region2_sha224[5] = 0xDEAD_BEEF;
//! message_region3_sha256[6] = 0xDEAD_BEEF;
//!
//! icm.enable()
use crate::pac::icm::uasr::URAT_A;

use paste::paste;
use seq_macro::seq;

use bitflags::bitflags;

use crate::pac::icm::*;
use crate::typelevel::Sealed;
use core::marker::PhantomData;

/// Reexport the User SHA Algorithm
pub use crate::icm::cfg::UALGO_A as icm_algorithm;

// Convenient bitflags representing select parts of
// the status interrupt register `ICM->ISR`

bitflags! {
    /// Region Hash Completed interrupt
    ///
    /// Bit number matches with region number
    pub struct RegionHashCompleted: u8 {
        const R0 = 1;
        const R1 = 2;
        const R2 = 4;
        const R3 = 8;
    }
}
bitflags! {
    /// Region Digest Mismatch interrupt
    ///
    /// Bit number matches with region number
    pub struct RegionDigestMismatch: u8 {
        const R0 = 1;
        const R1 = 2;
        const R2 = 4;
        const R3 = 8;
    }
}

bitflags! {
    /// Region Bus Error interrupt
    ///
    /// Bit number matches with region number
    pub struct RegionBusError: u8 {
        const R0 = 1;
        const R1 = 2;
        const R2 = 4;
        const R3 = 8;
    }
}

bitflags! {
    /// Region Wrap Condition detected interrupt
    ///
    /// Bit number matches with region number
    pub struct RegionWrapConditionDetected: u8 {
        const R0 = 1;
        const R1 = 2;
        const R2 = 4;
        const R3 = 8;
    }
}
bitflags! {
    /// Region End Condition detected interrupt
    ///
    /// Bit number matches with region number
    pub struct RegionEndConditionDetected: u8 {
        const R0 = 1;
        const R1 = 2;
        const R2 = 4;
        const R3 = 8;
    }
}
bitflags! {
    /// Region Status Update detected interrupt
    ///
    /// Bit number matches with region number
    pub struct RegionStatusUpdatedDetected: u8 {
        const R0 = 1;
        const R1 = 2;
        const R2 = 4;
        const R3 = 8;
    }
}

bitfield::bitfield! {
    /// Struct useful for returning the interrupt status
    /// of the ICM. Provides methods for easy parsing of
    /// all the regions or via the `bitmask` argument
    /// narrow it down to the specific set of [`RegionNum`]
    /// of interest.
    #[derive(Default)]
    pub struct Interrupt(u32);
    impl Debug;
    u8;
    get_rhc, _: 3, 0;
    get_rdm, _: 7, 4;
    get_rbe, _: 11, 8;
    get_rwc, _: 15, 12;
    get_rec, _: 19, 16;
    get_rsu, _: 23, 20;
    get_urad, _: 24, 24;
}

impl Interrupt {
    /// Region Status Updated interrupt status
    #[inline]
    pub fn get_rsu_int(&self) -> RegionStatusUpdatedDetected {
        RegionStatusUpdatedDetected::from_bits_truncate(self.get_rsu())
    }

    /// Region End bit Condition Detected interrupt status
    #[inline]
    pub fn get_rec_int(&self) -> RegionEndConditionDetected {
        RegionEndConditionDetected::from_bits_truncate(self.get_rec())
    }

    /// Region Wrap Condition detected interrupt status
    #[inline]
    pub fn get_rwc_int(&self) -> RegionWrapConditionDetected {
        RegionWrapConditionDetected::from_bits_truncate(self.get_rwc())
    }

    /// Region Bus Error interrupt status
    #[inline]
    pub fn get_rbe_int(&self) -> RegionBusError {
        RegionBusError::from_bits_truncate(self.get_rbe())
    }

    /// Region Digest Mis interrupt status
    #[inline]
    pub fn get_rdm_int(&self) -> RegionDigestMismatch {
        RegionDigestMismatch::from_bits_truncate(self.get_rdm())
    }

    /// Region Hash Completed interrupt status
    #[inline]
    pub fn get_rhc_int(&self) -> RegionHashCompleted {
        RegionHashCompleted::from_bits_truncate(self.get_rhc())
    }
}

/// Struct useful for returning the interrupt status
/// of the ICM. Provides methods for easy parsing of
/// the region specific [`RegionNum`]
pub struct RegionInterrupt<I: RegionNum> {
    region: PhantomData<I>,
    interrupt: Interrupt,
}

macro_rules! match_on_interrupt_status {
    ($self:ident, $name:ident) => {
        paste! {
            matches!($self.interrupt.[<get_$name>]() & $self.mask(), 1)
        }
    };
}

impl<I: RegionNum> RegionInterrupt<I> {
    /// Used to mask out the correct bit based on [`RegionNum`]
    #[inline]
    fn mask(&self) -> u8 {
        1 << I::NUM
    }

    /// Region Status Updated interrupt status
    #[inline]
    pub fn get_rsu_int(&self) -> bool {
        match_on_interrupt_status!(self, rsu)
    }

    /// Region End bit Condition Detected interrupt status
    #[inline]
    pub fn get_rec_int(&self) -> bool {
        match_on_interrupt_status!(self, rec)
    }

    /// Region Wrap Condition detected interrupt status
    #[inline]
    pub fn get_rwc_int(&self) -> bool {
        match_on_interrupt_status!(self, rwc)
    }

    /// Region Bus Error interrupt status
    #[inline]
    pub fn get_rbe_int(&self) -> bool {
        match_on_interrupt_status!(self, rbe)
    }

    /// Region Digest Mismatch interrupt status
    #[inline]
    pub fn get_rdm_int(&self) -> bool {
        match_on_interrupt_status!(self, rdm)
    }

    /// Region Hash Completed interrupt status
    #[inline]
    pub fn get_rhc_int(&self) -> bool {
        match_on_interrupt_status!(self, rhc)
    }
}

/// Region provides access to region-specific
/// settings like interrupts and status
pub struct Region<I: RegionNum> {
    region: PhantomData<I>,
}

macro_rules! match_on_interrupt_mask {
    ($self:ident, $name:ident) => {
        paste! {
            matches!($self.imr().read().[<$name>]().bits() & $self.mask(), 1)
        }
    };
}
macro_rules! match_on_interrupt_status {
    ($self:ident, $name:ident) => {
        paste! {
            matches!($self.isr().read().[<$name>]().bits() & $self.mask(), 1)
        }
    };
}

impl<I: RegionNum> Region<I> {
    pub(super) fn new() -> Self {
        Self {
            region: PhantomData,
        }
    }

    /// Used to mask out the correct bit based on [`RegionNum`]
    #[inline]
    fn mask(&self) -> u8 {
        1 << I::NUM
    }

    /// Provides the base pointer to the [``] registers
    ///
    /// # Safety
    ///
    /// Only one [Region] accessible at any given time
    #[inline]
    fn icm(&self) -> &RegisterBlock {
        unsafe { &*crate::pac::ICM::ptr() }
    }

    /// Control
    #[inline]
    fn ctrl(&self) -> &CTRL {
        &self.icm().ctrl
    }

    /// Interrupt Disable
    #[inline]
    fn idr(&self) -> &IDR {
        &self.icm().idr
    }

    /// Interrupt Enable
    #[inline]
    fn ier(&self) -> &IER {
        &self.icm().ier
    }

    /// Interrupt Mask
    #[inline]
    fn imr(&self) -> &IMR {
        &self.icm().imr
    }

    /// Interrupt Status
    #[inline]
    fn isr(&self) -> &ISR {
        &self.icm().isr
    }

    /// Status
    #[inline]
    fn sr(&self) -> &SR {
        &self.icm().sr
    }

    // Beginning of helper functions

    /// Enable this memory monitor region
    #[inline]
    pub fn enable_monitoring(&self) {
        // Each bit in the register represents one of the
        // four memory regions. Writing 0 does not change
        // the state of the region, to disable the regon,
        // writing to `rmdis` is required.
        self.ctrl().write(|w| unsafe { w.rmen().bits(self.mask()) });
    }

    /// Returns true if the region monitoring is active and a check value has
    /// been calculated and written to the hash area
    #[inline]
    pub fn get_monitoring_status(&self) -> bool {
        // If the region monitor disabled bit is set,
        // then the region monitoring is disabled
        self.sr().read().rmdis().bits() & self.mask() != 1
    }

    /// Returns true if the `RMEN` register has been set to one for the region
    #[inline]
    pub fn get_monitoring_raw_status(&self) -> bool {
        // If the region monitor disabled bit is set,
        // then the region monitoring is disabled
        self.sr().read().rawrmdis().bits() & self.mask() != 1
    }

    /// Disable the memory monitor region
    #[inline]
    pub fn disable_monitoring(&self) {
        // Each bit in the register represents one of the
        // four memory regions. Writing 0 does not change
        // the state of the region, to enable the region,
        // writing to `rmen` is required.
        self.ctrl()
            .write(|w| unsafe { w.rmdis().bits(self.mask()) });
    }

    /// Trigger recalculation of the memory monitor region
    #[inline]
    pub fn trigger_rehash(&self) {
        self.ctrl()
            .write(|w| unsafe { w.rehash().bits(self.mask()) });
    }

    /// Set Region Status Updated interrupt enable
    #[inline]
    pub fn set_rsu_int(&self) {
        self.ier().write(|w| unsafe { w.rsu().bits(self.mask()) });
    }

    /// Set Region End bit Condition Detected interrupt enable
    #[inline]
    pub fn set_rec_int(&self) {
        self.ier().write(|w| unsafe { w.rec().bits(self.mask()) });
    }

    /// Set Region Wrap Condition detected interrupt enable
    #[inline]
    pub fn set_rwc_int(&self) {
        self.ier().write(|w| unsafe { w.rwc().bits(self.mask()) });
    }

    /// Set Region Bus Error interrupt enable
    #[inline]
    pub fn set_rbe_int(&self) {
        self.ier().write(|w| unsafe { w.rbe().bits(self.mask()) });
    }

    /// Set Region Digest Mismatch interrupt enable
    #[inline]
    pub fn set_rdm_int(&self) {
        self.ier().write(|w| unsafe { w.rdm().bits(self.mask()) });
    }

    /// Set Region Hash Completed interrupt enable
    #[inline]
    pub fn set_rhc_int(&self) {
        self.ier().write(|w| unsafe { w.rhc().bits(self.mask()) });
    }

    /// Disable Region Status Updated interrupt enable
    #[inline]
    pub fn disable_rsu_int(&self) {
        self.idr().write(|w| unsafe { w.rsu().bits(self.mask()) });
    }

    /// Disable Region End bit Condition Detected interrupt enable
    #[inline]
    pub fn disable_rec_int(&self) {
        self.idr().write(|w| unsafe { w.rec().bits(self.mask()) });
    }

    /// Disable Region Wrap Condition detected interrupt enable
    #[inline]
    pub fn disable_rwc_int(&self) {
        self.idr().write(|w| unsafe { w.rwc().bits(self.mask()) });
    }

    /// Disable Region Bus Error interrupt enable
    #[inline]
    pub fn disable_rbe_int(&self) {
        self.idr().write(|w| unsafe { w.rbe().bits(self.mask()) });
    }

    /// Disable Region Digest Mismatch interrupt enable
    #[inline]
    pub fn disable_rdm_int(&self) {
        self.idr().write(|w| unsafe { w.rdm().bits(self.mask()) });
    }

    /// Disable Region Hash Completed interrupt enable
    #[inline]
    pub fn disable_rhc_int(&self) {
        self.idr().write(|w| unsafe { w.rhc().bits(self.mask()) });
    }

    /// Get Region Status Updated interrupt enable mask
    #[inline]
    pub fn get_rsu_int_mask(&self) -> bool {
        match_on_interrupt_mask!(self, rsu)
    }

    /// Get Region End bit Condition Detected interrupt enable mask
    #[inline]
    pub fn get_rec_int_mask(&self) -> bool {
        match_on_interrupt_mask!(self, rec)
    }

    /// Get Region Wrap Condition detected interrupt enable mask
    #[inline]
    pub fn get_rwc_int_mask(&self) -> bool {
        match_on_interrupt_mask!(self, rwc)
    }

    /// Get Region Bus Error interrupt enable mask
    #[inline]
    pub fn get_rbe_int_mask(&self) -> bool {
        match_on_interrupt_mask!(self, rbe)
    }

    /// Get Region Digest Mismatch interrupt enable mask
    #[inline]
    pub fn get_rdm_int_mask(&self) -> bool {
        match_on_interrupt_mask!(self, rdm)
    }

    /// Get Region Hash Completed interrupt enable mask
    #[inline]
    pub fn get_rhc_int_mask(&self) -> bool {
        match_on_interrupt_mask!(self, rhc)
    }

    /// Region Status Updated interrupt status
    #[inline]
    pub fn get_rsu_int(&self) -> bool {
        match_on_interrupt_status!(self, rsu)
    }

    /// Region End bit Condition Detected interrupt status
    #[inline]
    pub fn get_rec_int(&self) -> bool {
        match_on_interrupt_status!(self, rec)
    }

    /// Region Wrap Condition detected interrupt status
    #[inline]
    pub fn get_rwc_int(&self) -> bool {
        match_on_interrupt_status!(self, rwc)
    }

    /// Region Bus Error interrupt status
    #[inline]
    pub fn get_rbe_int(&self) -> bool {
        match_on_interrupt_status!(self, rbe)
    }

    /// Region Digest Mismatch interrupt status
    #[inline]
    pub fn get_rdm_int(&self) -> bool {
        match_on_interrupt_status!(self, rdm)
    }

    /// Region Hash Completed interrupt status
    #[inline]
    pub fn get_rhc_int(&self) -> bool {
        match_on_interrupt_status!(self, rhc)
    }

    /// When reading the interrupt (ISR) register, it is cleared
    ///
    /// This might result in unexpected results for example
    /// when sequentially trying to determine which interrupt triggered.
    ///
    /// This is an alternative, return all the data from the register
    /// and parse later with the designated `get_[name]_int` functions.
    #[inline]
    pub fn get_interrupt_status(&mut self) -> RegionInterrupt<I> {
        let interrupt = Interrupt(self.isr().read().bits());
        RegionInterrupt {
            region: PhantomData,
            interrupt,
        }
    }
}

/// ICM Peripheral
///
/// Encapsulates the PAC which acts as a token
/// and provides an interface to the ICM hardware
pub struct Icm {
    /// ICM pac register providing hardware access
    icm: crate::pac::ICM,
}

impl Icm {
    /// Create the interface for the ICM peripheral
    ///
    /// Don't forget to enable the `APB` bus for ICM
    ///
    /// `AHB` bus is on by default at reset
    ///
    /// Clock::v1
    /// `mclk.apbcmask.modify(|_, w| w.icm_().set_bit());`
    ///
    /// Clock::v2
    /// `tokens.apbs.icm.enable();`
    #[inline]
    pub fn new(icm: crate::pac::ICM) -> Self {
        Self { icm }
    }

    // Register Interface

    /// Integrity Check Module
    #[inline]
    fn icm(&self) -> &RegisterBlock {
        &self.icm
    }

    /// Configuration
    #[inline]
    fn cfg(&self) -> &CFG {
        &self.icm().cfg
    }

    /// Control
    #[inline]
    fn ctrl(&self) -> &CTRL {
        &self.icm().ctrl
    }

    /// Region Descriptor Area Start Address
    #[inline]
    fn dscr(&self) -> &DSCR {
        &self.icm().dscr
    }

    /// Region Hash Area Start Address
    #[inline]
    fn hash(&self) -> &HASH {
        &self.icm().hash
    }

    /// Interrupt Disable
    #[inline]
    fn idr(&self) -> &IDR {
        &self.icm().idr
    }

    /// Interrupt Enable
    #[inline]
    fn ier(&self) -> &IER {
        &self.icm().ier
    }

    /// Interrupt Mask
    #[inline]
    fn imr(&self) -> &IMR {
        &self.icm().imr
    }

    /// Interrupt Status
    #[inline]
    fn isr(&self) -> &ISR {
        &self.icm().isr
    }

    /// Status
    #[inline]
    fn sr(&self) -> &SR {
        &self.icm().sr
    }

    /// Undefined Access Status
    #[inline]
    fn uasr(&self) -> &UASR {
        &self.icm().uasr
    }

    // User interface for ICM

    /// Enable the ICM peripheral
    #[inline]
    pub fn enable(&mut self) {
        self.ctrl().write(|w| w.enable().set_bit());
    }

    /// Get enabled status of the ICM peripheral
    #[inline]
    pub fn icm_status(&self) -> bool {
        self.sr().read().enable().bit_is_set()
    }

    /// Disable the ICM peripheral
    #[inline]
    pub fn disable(&mut self) {
        self.ctrl().write(|w| w.disable().set_bit());
    }

    /// Reset the ICM controller
    ///
    /// Does not seem to clear DSCR, HASH addr
    ///
    /// The only way to clear the `URAD` and `URAT` fields
    /// is by resetting the ICM controller
    #[inline]
    pub fn swrst(&mut self) {
        self.ctrl().write(|w| w.swrst().set_bit());
    }

    /// Destroy the ICM peripheral and return the underlying ICM register
    #[inline]
    pub fn destroy(self) -> crate::pac::ICM {
        self.icm
    }

    // Region specifics

    #[inline]
    pub fn enable_region<N: RegionNum>(&mut self) -> Region<N> {
        Region::<N>::new()
    }

    /// Enable region0
    ///
    /// Creates an [`Region`] which provides region specific
    /// settings
    #[inline]
    pub fn enable_region0(&mut self) -> Region<Region0> {
        Region::new()
    }
    /// Enable region1
    ///
    /// Creates an [`Region`] which provides region specific
    /// settings
    #[inline]
    pub fn enable_region1(&mut self) -> Region<Region1> {
        Region::new()
    }
    /// Enable region2
    ///
    /// Creates an [`Region`] which provides region specific
    /// settings
    #[inline]
    pub fn enable_region2(&mut self) -> Region<Region2> {
        Region::new()
    }
    /// Enable region3
    ///
    /// Creates an [`Region`] which provides region specific
    /// settings
    #[inline]
    pub fn enable_region3(&mut self) -> Region<Region3> {
        Region::new()
    }

    // Configuration of ICM

    /// Helper for setting the HASH addr
    ///
    /// Expects a raw pointer to the memory address of the beginning of the
    /// designated variable but expressed as a multiple of 128
    #[inline]
    pub fn set_hash_addr(&mut self, hash_addr_pointer: &HashArea) {
        self.hash()
            .write(|w| unsafe { w.hasa().bits((hash_addr_pointer as *const _) as u32 / 128) })
    }

    /// Set the DSCR addr to a specific MainRegionDesc
    ///
    /// HW expects a raw pointer to the memory address of the beginning of the
    /// [`MainRegionDesc`] but expressed as a multiple of 64
    pub fn set_dscr_addr(&self, icm_region_desc: &MainRegionDesc<Region0>) {
        self.dscr()
            .write(|w| unsafe { w.dasa().bits((icm_region_desc as *const _) as u32 / 64) })
    }

    /// Set the user initial hash value
    #[inline]
    pub fn set_user_initial_hash_value(&self, user_initial_hash_value: [u32; 8]) {
        for (index, initial_value) in user_initial_hash_value.iter().enumerate() {
            self.icm().uihval[index].write(|w| unsafe { w.val().bits(*initial_value) });
        }
    }

    /// Set the user hashing algorithm
    #[inline]
    pub fn set_user_algorithm(self, algo: icm_algorithm) {
        self.cfg().write(|w| w.ualgo().variant(algo));
    }

    /// Activate user hash mode
    ///
    /// Allows providing
    /// * hash initial value
    /// * Hash algorithm
    ///
    /// Disables the `ALGO` field in [`MainRegionDesc`]
    ///
    /// Set initial hash value via [`Icm::set_user_initial_hash_value()`]
    /// Set hash algorithm via [`Icm::set_user_algorithm()`]
    #[inline]
    pub fn set_user_configurable_hash(&self, user_configurable_hash: bool) {
        self.cfg().write(|w| w.uihash().bit(user_configurable_hash));
    }

    /// Control dual input buffer
    ///
    /// Enabling dual input buffering allow for better performance
    /// at the cost of higher bandwith requirements on the system bus
    #[inline]
    pub fn set_dual_input_buffer(&self, dualbuffer: bool) {
        self.cfg().write(|w| w.dualbuff().bit(dualbuffer));
    }

    /// Automatic switch to Compare Digest
    ///
    /// When activated, after the first Main List pass the ICM controller
    /// automatically switches to active monitoring.
    ///
    /// `CDWBN` and `WBDIS` bits has no effects, to terminate the monitoring a
    /// `1` needs to be written to End of Monitoring (`RCFG.EOM`)
    #[inline]
    pub fn set_ascd(&self, automaticswitch: bool) {
        self.cfg().write(|w| w.ascd().bit(automaticswitch));
    }

    /// Bus burden control
    ///
    /// Set a delay between block transfers, calculated as
    ///
    /// `2.pow(busburden)`
    ///
    /// Maximum delay is 32768 cycles
    #[inline]
    pub fn set_busburden(&self, busburden: u8) {
        self.cfg().write(|w| unsafe { w.bbc().bits(busburden) });
    }

    /// Secondary List Branching Disable
    ///
    /// * If set to false, allow secondary lists
    /// * If set to true, secondary lists are forbidden, `NEXT` and `RNEXT` is
    ///   always considered 0.
    #[inline]
    pub fn set_slbdis(&self, disable_secondary_lists: bool) {
        self.cfg()
            .write(|w| w.slbdis().bit(disable_secondary_lists));
    }

    /// End of Monitoring Disable
    ///
    /// * If set to false, End of Monitoring is permitter
    /// * If set to true, End of Monitoring is forbidden, the EOM bit in RCFG
    ///   has no effect.
    #[inline]
    pub fn set_eomdis(&self, disable_eom: bool) {
        self.cfg().write(|w| w.eomdis().bit(disable_eom));
    }

    /// Write Back Disable
    ///
    /// * If set to false, Write Back Operations are permitted
    /// * If set to true, Write Back Operations are forbidden, `CDWBN` bit is
    ///   internally set to 1 and cannot be modified by a linked list element.
    ///   The `CDWBN` bit of the `RCFG` structure member has no effect.
    #[inline]
    pub fn set_wbdis(&self, disable_eom: bool) {
        self.cfg().write(|w| w.wbdis().bit(disable_eom));
    }

    // Security and tamper settings

    /// Set Undefined Register Access Detection interrupt enable
    #[inline]
    pub fn set_urad_int(self) {
        self.ier().write(|w| unsafe { w.rsu().bits(1) });
    }

    /// Disable Undefined Register Access Detection interrupt enable
    #[inline]
    pub fn disable_urad_int(self) {
        self.idr().write(|w| unsafe { w.rsu().bits(1) });
    }

    /// Get Undefined Register Access Detection interrupt mask
    #[inline]
    pub fn get_urad_int_mask(&self) -> bool {
        self.imr().read().urad().bits()
    }

    /// Get Undefined Register Access Detection interrupt status
    #[inline]
    pub fn get_urad_int(&self) -> bool {
        self.isr().read().urad().bits()
    }

    /// Get Undefined Register Access Trace
    ///
    /// This field is only reset by `swrst`
    #[inline]
    pub fn get_urat(&self) -> URAT_A {
        self.uasr().read().urat().variant().unwrap()
    }

    /// When reading the interrupt (ISR) register, it is cleared
    ///
    /// This might result in unexpected results for example
    /// when sequentially trying to determine which interrupt triggered.
    ///
    /// This is an alternative, return all the data from the register
    /// and parse later with the designated `get_[name]_int` functions.
    #[inline]
    pub fn get_interrupt_status(&mut self) -> Interrupt {
        let interrupt_vector = self.isr().read().bits();
        Interrupt(interrupt_vector)
    }
    /// Trigger recalculation of memory monitor region specified
    /// by the bitmask:
    /// 0b0001 = region0
    /// 0b0010 = region1
    /// 0b0100 = region2
    /// 0b1000 = region3
    /// ...
    /// 0b1111 = all regions
    #[inline]
    pub fn trigger_rehash(&self, bitmask: u8) {
        self.ctrl().write(|w| unsafe { w.rehash().bits(bitmask) });
    }
}

/// Trait providing numerical identifier and
/// offset for each ICM Region
///
/// ICM supports 4 memory regions
pub trait RegionNum: Sealed {
    /// Numerical ID of the memory region
    const NUM: usize;
    /// Memory offset
    const OFFSET: u32;
}

seq!(N in 0..=3 {
    paste! {
        #[doc = "ICM Region " N]
        pub enum Region~N {}
        impl Sealed for Region~N {}
        impl RegionNum for Region~N {
            const NUM: usize = N;
            #[allow(clippy::identity_op)]
            #[allow(clippy::erasing_op)]
            const OFFSET: u32 = 0x10 * N;
        }
    }
});

/// Functions required by [`MainRegionDesc`]
///
/// Both Main List descriptors and Secondary List descriptors
pub trait RegionDesc {
    /// Set the [`RegionAddress`] start of the region memory region
    fn set_region_address<T>(&mut self, addr: *const T);
    /// Set the specific region configuration
    fn set_region_configuration(&mut self, cfg: RegionConfiguration);
    /// Set the link to next region descriptor
    fn set_region_next(&mut self, next: RegionNext);
    /// Reset RegionConfiguration to default values
    fn reset_region_configuration_to_default(&mut self);
}

/// Helper for creating the Region Descriptor structure
///
/// It is also possible to construct the Region manually,
/// but then care has to be taken to point `rnext` to the appropriate
/// place in memory, here the hardware assumption of 0x10
/// offset to the next descriptor is ensured.
///
/// Follows C-structure conventions and is 64-byte aligned
///
/// >**Important**
/// >
/// >ICM engine will **read** wherever this
/// >is instantiated in memory, based on the [`Icm::set_dscr_addr()`]
/// >so the user must ensure that this variable lives long enough or is
/// >static
#[repr(C)]
#[repr(align(64))]
pub struct Regions {
    /// MainRegionDesc0
    pub region0: MainRegionDesc<Region0>,
    /// MainRegionDesc1
    pub region1: MainRegionDesc<Region1>,
    /// MainRegionDesc2
    pub region2: MainRegionDesc<Region2>,
    /// MainRegionDesc3
    pub region3: MainRegionDesc<Region3>,
}

impl Regions {
    pub const fn default() -> Self {
        let region0 = MainRegionDesc::new_region0();
        let region1 = MainRegionDesc::new_region1();
        let region2 = MainRegionDesc::new_region2();
        let region3 = MainRegionDesc::new_region3();
        Self {
            region0,
            region1,
            region2,
            region3,
        }
    }
}

/// Structure ICM Region Descriptor area.
///
/// Follows C-structure conventions and is 16-byte aligned,
/// being a part of the 64-bytes making up [`Region`]
#[repr(C)]
#[repr(align(16))]
pub struct MainRegionDesc<N: RegionNum> {
    /// Numerical Region Identifier
    num: PhantomData<N>,
    /// The first byte address of the Region.
    pub raddr: RegionAddress,
    /// Configuration Structure Member.
    pub rcfg: RegionConfiguration,
    /// Control Structure Member.
    pub rctrl: RegionControl,
    /// Next Address Structure Member.
    pub rnext: RegionNext,
}

impl MainRegionDesc<Region0> {
    /// Helper for setting the DSCR addr to a the first MainRegionDesc
    ///
    /// See [`Icm::set_dscr_addr()`] for the regular workflow
    ///
    /// HW expects a raw pointer to the memory address of the beginning of the
    /// [`MainRegionDesc`] but expressed as a multiple of 64
    pub fn set_dscr_addr(&self, icm: &Icm) {
        icm.dscr()
            .write(|w| unsafe { w.dasa().bits((self as *const _) as u32 / 64) })
    }
}

seq!(N in 0..=3 {
    paste! {
        #[doc = "Create region descriptor " N]
        impl MainRegionDesc<Region~N> {
            const fn new_region~N() -> Self {
                MainRegionDesc {
                    num: PhantomData,
                    raddr: RegionAddress::default(),
                    rcfg: RegionConfiguration::default(),
                    rctrl: RegionControl::default(),
                    rnext: RegionNext::default(),
                }
            }
        }
    }
});

impl<N: RegionNum> RegionDesc for MainRegionDesc<N> {
    /// Set [`RegionAddress`]
    #[inline]
    fn set_region_address<T>(&mut self, addr: *const T) {
        self.raddr.set_region_address(addr);
    }
    /// Set [`RegionConfiguration`]
    #[inline]
    fn set_region_configuration(&mut self, cfg: RegionConfiguration) {
        self.rcfg = cfg;
    }
    /// Reset [`RegionConfiguration`] to default values
    #[inline]
    fn reset_region_configuration_to_default(&mut self) {
        self.rcfg = RegionConfiguration::default();
    }
    /// Set [`RegionNext`]
    #[inline]
    fn set_region_next(&mut self, next: RegionNext) {
        self.rnext = next;
    }
}

impl<N: RegionNum> MainRegionDesc<N> {
    /// The length of data for the ICM engine to transfer,
    /// expressed as number of `blocks - 1`.
    #[inline]
    pub fn set_rctrl(mut self, ctrl: RegionControl) {
        self.rctrl = ctrl;
    }
}

/// Structure ICM Secondary Region Descriptor area.
///
/// Follows C-structure conventions and is 16-byte aligned
///
/// Used to build the linked lists for non-contiguous memory
#[repr(C)]
#[repr(align(16))]
pub struct SecondaryRegionDesc {
    /// the first byte address of the Region.
    pub raddr: RegionAddress,
    /// Configuration Structure Member.
    pub rcfg: RegionConfiguration,
    /// Not used in Secondary Region Descriptor
    _pad: u32,
    /// Next Address Structure Member.
    pub rnext: RegionNext,
}

impl RegionDesc for SecondaryRegionDesc {
    fn set_region_address<T>(&mut self, addr: *const T) {
        self.raddr.set_region_address(addr);
    }
    /// Set [`RegionConfiguration`]
    #[inline]
    fn set_region_configuration(&mut self, cfg: RegionConfiguration) {
        self.rcfg = cfg;
    }
    /// Reset [`RegionConfiguration`] to default values
    #[inline]
    fn reset_region_configuration_to_default(&mut self) {
        self.rcfg = RegionConfiguration::default();
    }
    /// Set [`RegionNext`]
    #[inline]
    fn set_region_next(&mut self, next: RegionNext) {
        self.rnext = next;
    }
}

impl SecondaryRegionDesc {
    pub const fn default() -> Self {
        SecondaryRegionDesc {
            raddr: RegionAddress::default(),
            rcfg: RegionConfiguration::default(),
            _pad: 0,
            rnext: RegionNext::default(),
        }
    }
}

/// ICM Hash Area
///
/// Follows C-structure conventions and is 128-byte aligned
///
/// >**Important**
/// >
/// >ICM engine will **read** or **write** to wherever this
/// >is instantiated in memory, based on the [`Icm::set_hash_addr()`]
/// >so the user must ensure that this variable lives long enough or is
/// >static
#[derive(Debug)]
#[repr(C)]
#[repr(align(128))]
pub struct HashArea {
    pub region0: [u32; 8],
    pub region1: [u32; 8],
    pub region2: [u32; 8],
    pub region3: [u32; 8],
}

impl HashArea {
    pub const fn default() -> Self {
        HashArea {
            region0: [0; 8],
            region1: [0; 8],
            region2: [0; 8],
            region3: [0; 8],
        }
    }
}

/// Region Start Address Structure
///
/// Follows C-structure conventions
#[derive(Clone, Copy)]
#[repr(C)]
pub struct RegionAddress {
    pub raddr: u32,
}

impl RegionAddress {
    pub fn set_region_address<T>(&mut self, raddr: *const T) {
        self.raddr = raddr as u32;
    }
    const fn default() -> Self {
        // Unsure what a good RegionAddress default should be,
        // this is at least easily spotted when debugging...
        RegionAddress {
            raddr: 0xDEADBEEF_u32,
        }
    }
}

/*
#[derive(Clone, Copy)]
#[repr(C)]
pub struct RegionConfiguration {
    rcfg: u32,
}
*/

bitfield::bitfield! {
    /// Region Configuration Structure
    ///
    /// Follows C-structure conventions
    #[derive(Clone, Copy)]
    #[repr(C)]
    pub struct RegionConfiguration(u32);
    impl Debug;
    u8;
    /// Compare Digest or Write Back Digest
    ///
    /// * true: in Compare Digest mode.
    /// * false: in Write Back Digest mode.
    #[inline]
    pub get_cdwbn, set_cdwbn: 0;
    /// Wrap Command
    ///
    /// * true: next region descriptor address loaded is `DSCR`.
    /// * false: the next region descriptor address is `current + 0x10`.
    #[inline]
    pub
    get_wrap, set_wrap: 1;
    /// End of Monitoring
    ///
    /// * true: the current descriptor terminates the Main List, WRAP bit has
    /// no effect.
    /// * false: the current descriptor does not terminate the
    /// monitoring.
    #[inline]
    pub
    get_eom, set_eom:  2;
    /// Region Hash Completed Interrupt Disable
    /// * true: the `RHC` flag remains cleared even if the setting condition is
    /// met.
    /// * false: the `RHC` flag is set when the field `NEXT = 0` is
    /// found in main or secondary list.
    #[inline]
    pub
    get_rhien, set_rhien: 4;
    /// Digest Mismatch Interrupt Disable
    ///
    /// * true: the `RBE` flag remains cleared even if the setting condition is
    /// met.
    /// * false: the `RBE` flag is set when the hash value just
    /// calculated from the processed region differs from expected hash
    /// value.
    #[inline]
    pub
    get_dmien, set_dmien: 5;
    /// Bus Error Interrupt Disable
    ///
    /// * true: the flag remains cleared even if the setting condition is met.
    /// * false: the flag is set when an error is reported on the sysstem bus
    /// by the bus MATRIX.
    #[inline]
    pub
    get_beien, set_beien: 6;
    /// Wrap Condition Interrupt Disable
    ///
    /// * true: the `RWC` flag remains cleared even if the setting condition is
    /// met.
    /// * false: the `RWC` flag is set when the WRAP is encountered.
    #[inline]
    pub get_wcien, set_wcien: 7;
    /// End Bit Condition Interrupt Enable
    ///
    /// * true: the `REC` flag remains cleared even if the setting condition is
    /// met.
    /// * false: the `REC` flag is set when the descriptor having the
    /// `EOM` bit set is processed.
    #[inline]
    pub get_ecien, set_ecien: 8;
    /// Monitoring Status Updated Condition Interrupt Enable
    ///
    /// * true: the `RSU` flag remains cleared even if the condition is met.
    /// * false: the `RSU` flag is set when the corresponding descriptor is
    /// loaded from memory to ICM.
    #[inline]
    pub get_suien, set_suien: 9;
    /// Processing Delay
    ///
    /// Allows setting short or long delay.
    ///
    /// See [`ProcessingDelay`]
    #[inline]
    pub get_procdly, set_procdly: 10;
    get_algo_bits, set_algo_bits: 14, 12;
}

impl RegionConfiguration {
    const fn default() -> Self {
        //RegionConfiguration { rcfg: 0x3F0 }
        RegionConfiguration(0x3F0)
    }
}

impl RegionConfiguration {
    /// User SHA Algorithm
    ///
    /// Allow setting this regions [`icm_algorithm`].
    #[inline]
    pub fn set_algo(&mut self, value: icm_algorithm) {
        self.set_algo_bits(value.into());
    }

    /// User SHA Algorithm
    ///
    /// Get the current user sha algorithm
    #[inline]
    pub fn get_algo(&mut self) -> icm_algorithm {
        match self.get_algo_bits() {
            2 => icm_algorithm::SHA224,
            4 => icm_algorithm::SHA256,
            _ => icm_algorithm::SHA1,
        }
    }
    /// Reset the [`RegionConfiguration`] to default values
    ///
    /// Useful if changing between hashing and monitoring, etc.
    #[inline]
    pub fn reset_region_configuration_to_default(&mut self) {
        self.0 = 0x3F0_u32;
    }
}

/// Processing Delay
///
/// For a given SHA algorithm, the runtime period has two possible lengths
pub enum ProcessingDelay {
    /// Shortest processing delay
    ///
    /// `SHA1`: 85
    ///
    /// `SHA224`: 72
    ///
    /// `SHA256`: 72
    Shortest = 0,
    /// Longest processing delay
    ///
    /// `SHA1`: 209
    ///
    /// `SHA224`: 194
    ///
    /// `SHA256`: 194
    Longest = 1,
}

/// Region Control Structure
///
/// Follows C-structure conventions
#[derive(Clone, Copy)]
#[repr(C)]
pub struct RegionControl {
    pub trsize: u16,
}

impl RegionControl {
    const fn default() -> Self {
        RegionControl { trsize: 0 }
    }
}

/// Region Next Address Structure
///
/// Is the same as RegionAddress<N+1>
///
/// Follows C-structure conventions
#[derive(Clone, Copy)]
#[repr(C)]
pub struct RegionNext {
    rnext: u32,
}

impl RegionNext {
    pub fn set_region_next(&mut self, rnext: &impl RegionDesc) {
        self.rnext = (rnext as *const _) as u32;
    }

    const fn default() -> Self {
        RegionNext { rnext: 0 }
    }
}
