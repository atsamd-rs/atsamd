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
//! [`IcmRegionToken<I>::get_interrupt_status()`] are provided.
//! These return a queryable structure containing the interrupt register
//! contents. Allowing multiple different interrupts to be read.
//!
//! >**IMPORTANT** - Memory safety considerations
//! >
//! >The ICM engine accesses the assigned `DSCR` memory address, so it must be available.
//! >Depending on the application, this might entail making [`IcmRegion`] **static**.
//! >
//! >The same goes for [`IcmHashArea`], but here it is even more important to ensure
//! >the memory is designated for `IcmHashArea` usage, since the ICM controller may
//! >write data, depending on ICM configuration.
//! >
//! >Setting [`IcmHashArea`] **static** might be the safest path.
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
//! [`IcmRegionToken::enable_monitoring()`]
//!
//! Depending on the number of regions required, the helper
//! [`IcmRegion::default()`] alows setting up all 4 regions directly, if one
//! region is sufficient, manually create  [`IcmRegionDesc<Region0>::
//! default()`].
//!
//!  Modify the [`IcmRegionDesc`], see documentation and cargo doc for all
//! methods.
//!
//!  Set the `DSCR` address to the beginning of the [`IcmRegionDesc`] via
//!  [`Icm::set_dscr_addr()`] (or via helper in
//! [`IcmRegionDesc<Region0>::set_dscr_addr()`])
//!
//!  Via [`IcmRegionToken`], setup the desired interrupts depending on usecase.
//!
//!  To view which interrupts has been enabled in the debugger, check the
//! `ICM->IMR` register.
//!
//!  Any object in memory can be used as the "Hash" area, but for convenience
//! the provided  [`IcmHashArea`] allows indexing of the 4 regions and is
//! correctly memory aligned.
//!
//!  Set the pointer to [`IcmHashArea`] via [`Icm::set_hash_addr()`]
//!
//!  **See note about memory safety above**
//!
//! ### Hash calculation
//!
//! Assuming general setup is already done, modify the [`Rcfg`] which is part of
//! the [`IcmRegionDesc`]:
//!
//! * [`Rcfg::set_rhien()`] to `false` to allow interrupts when calculation is
//!   done
//! * [`Rcfg::set_eom()`] to `true` only for the last region
//!
//! Change [`Raddr`] to point to the object to SHA-sum with
//! [`IcmRegionDesc<RegionNumT>::set_raddr()`]
//!
//! ### Memory monitoring
//!
//! [`IcmHashArea`] needs to contain the expected SHA-sums of the data to
//! monitor, [`Icm::set_ascd()`] is provided to help with creating this data.
//! Alternatively do it manually and then change mode, or prepopulate the
//! [`IcmHashArea`] with SHA-sums.
//!
//! Assuming general setup is already done, modify the [`Rcfg`] which is part of
//! the [`IcmRegionDesc`]:
//!
//! * [`Rcfg::set_dmien()`] to `false` to allow interrupts if mismatch occurs
//! * [`Rcfg::set_cdwbn()`] to `true` to change to monitor mode
//! * [`Rcfg::set_wrap()`] to `true` only for the last region if continuous
//!   monitoring is desired
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
//! static mut HASH: IcmHashArea = IcmHashArea::default();
//! static mut ICM_REGION_DESC: IcmRegion = IcmRegion::default();
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
//! let icmtoken = IcmToken::new(peripherals.ICM);
//!
//! let mut icm = Icm::new(icmtoken);
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
//! let mut icm_region_desc = IcmRegion::default();
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
//! icm_region_desc.region0.set_raddr(MESSAGE_REF0.as_ptr());
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
//! icm_region_desc.region1.set_raddr(MESSAGE_REF1.as_ptr());
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
//! icm_region_desc.region2.set_raddr(MESSAGE_REF1.as_ptr());
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
//! icm_region_desc.region3.set_raddr(MESSAGE_REF1.as_ptr());
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
//! // Setup region 0 to monitor memory
//! icm_region_desc
//!     .region0
//!     .set_raddr(&message_region0_sha1);
//! icm_region_desc.region0.rcfg.reset_rcfg_to_default();
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
//!     .set_raddr(&message_region1_sha1);
//! icm_region_desc.region1.rcfg.reset_rcfg_to_default();
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
//!     .set_raddr(&message_region2_sha224);
//! icm_region_desc.region2.rcfg.reset_rcfg_to_default();
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
//!     .set_raddr(&message_region3_sha256);
//! icm_region_desc.region3.rcfg.reset_rcfg_to_default();
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
use crate::pac::generic::Variant;
use crate::pac::icm::uasr::URAT_A;

use crate::pac::icm::*;
use crate::typelevel::Sealed;
use core::marker::PhantomData;

/// Reexport the User SHA Algorithm
pub use crate::icm::cfg::UALGO_A as icm_algorithm;

/// IcmToken represents the peripheral and it encapsulates the hardware register
pub struct IcmToken {
    icm: crate::pac::ICM,
}

impl IcmToken {
    /// Create a new instance of [`IcmToken`]
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
    pub fn new(icm: crate::pac::ICM) -> Self {
        Self { icm }
    }

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
    pub fn imr(&self) -> &IMR {
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
}

/// Struct useful for returning the interrupt status
/// of the ICM. Provides methods for easy parsing of
/// all the regions or via the `bitmask` argument
/// narrow it down to the specific set of [`IcmRegionNum`]
/// of interest.
pub struct IcmInterrupt {
    interrupt_vector: u32,
}

impl IcmInterrupt {
    /// Region Status Updated interrupt status
    #[inline]
    pub fn get_rsu_int(&self, bitmask: u8) -> u8 {
        (self.interrupt_vector >> 20 & 0x0f) as u8 & bitmask
    }

    /// Region End bit Condition Detected interrupt status
    #[inline]
    pub fn get_rec_int(&self, bitmask: u8) -> u8 {
        (self.interrupt_vector >> 16 & 0x0f) as u8 & bitmask
    }

    /// Region Wrap Condition detected interrupt status
    #[inline]
    pub fn get_rwc_int(&self, bitmask: u8) -> u8 {
        (self.interrupt_vector >> 12 & 0x0f) as u8 & bitmask
    }

    /// Region Bus Error interrupt status
    #[inline]
    pub fn get_rbe_int(&self, bitmask: u8) -> u8 {
        (self.interrupt_vector >> 8 & 0x0f) as u8 & bitmask
    }

    /// Region Digest Mis interrupt status
    #[inline]
    pub fn get_rdm_int(&self, bitmask: u8) -> u8 {
        (self.interrupt_vector >> 4 & 0x0f) as u8 & bitmask
    }

    /// Region Hash Completed interrupt status
    #[inline]
    pub fn get_rhc_int(&self, bitmask: u8) -> u8 {
        (self.interrupt_vector & 0x0f) as u8 & bitmask
    }
}
impl Default for IcmInterrupt {
    fn default() -> Self {
        Self {
            interrupt_vector: 0,
        }
    }
}

/// Struct useful for returning the interrupt status
/// of the ICM. Provides methods for easy parsing of
/// the region specific [`IcmRegionNum`]
pub struct IcmRegionInterrupt<I: IcmRegionNum> {
    region: PhantomData<I>,
    interrupt_vector: u32,
}

impl<I: IcmRegionNum> IcmRegionInterrupt<I> {
    /// Used to mask out the correct bit based on [`IcmRegionNum`]
    #[inline]
    fn mask(&self) -> u8 {
        1 << I::NUM
    }

    /// Region Status Updated interrupt status
    #[inline]
    pub fn get_rsu_int(&self) -> bool {
        matches!((self.interrupt_vector >> 20 & 0x0f) as u8 & self.mask(), 1)
    }

    /// Region End bit Condition Detected interrupt status
    #[inline]
    pub fn get_rec_int(&self) -> bool {
        matches!((self.interrupt_vector >> 16 & 0x0f) as u8 & self.mask(), 1)
    }

    /// Region Wrap Condition detected interrupt status
    #[inline]
    pub fn get_rwc_int(&self) -> bool {
        matches!((self.interrupt_vector >> 12 & 0x0f) as u8 & self.mask(), 1)
    }

    /// Region Bus Error interrupt status
    #[inline]
    pub fn get_rbe_int(&self) -> bool {
        matches!((self.interrupt_vector >> 8 & 0x0f) as u8 & self.mask(), 1)
    }

    /// Region Digest Mismatches!( interrupt status
    #[inline]
    pub fn get_rdm_int(&self) -> bool {
        matches!((self.interrupt_vector >> 4 & 0x0f) as u8 & self.mask(), 1)
    }

    /// Region Hash Completed interrupt status
    #[inline]
    pub fn get_rhc_int(&self) -> bool {
        matches!((self.interrupt_vector & 0x0f) as u8 & self.mask(), 1)
    }
}

/// IcmRegionToken provides access to region-specific
/// settings like interrupts and status
pub struct IcmRegionToken<I: IcmRegionNum> {
    region: PhantomData<I>,
}

impl<I: IcmRegionNum> IcmRegionToken<I> {
    pub(super) fn new() -> Self {
        Self {
            region: PhantomData,
        }
    }

    /// Used to mask out the correct bit based on [`IcmRegionNum`]
    #[inline]
    fn mask(&self) -> u8 {
        1 << I::NUM
    }

    /// Provides the base pointer to the [`Icm`] registers
    ///
    /// # Safety
    ///
    /// Only one [IcmRegionToken] accessible at any given time
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
        matches!(self.imr().read().rsu().bits() & self.mask(), 1)
    }

    /// Get Region End bit Condition Detected interrupt enable mask
    #[inline]
    pub fn get_rec_int_mask(&self) -> bool {
        matches!(self.imr().read().rec().bits() & self.mask(), 1)
    }

    /// Get Region Wrap Condition detected interrupt enable mask
    #[inline]
    pub fn get_rwc_int_mask(&self) -> bool {
        matches!(self.imr().read().rwc().bits() & self.mask(), 1)
    }

    /// Get Region Bus Error interrupt enable mask
    #[inline]
    pub fn get_rbe_int_mask(&self) -> bool {
        matches!(self.imr().read().rbe().bits() & self.mask(), 1)
    }

    /// Get Region Digest Mismatches!( interrupt enable mask
    #[inline]
    pub fn get_rdm_int_mask(&self) -> bool {
        matches!(self.imr().read().rdm().bits() & self.mask(), 1)
    }

    /// Get Region Hash Completed interrupt enable mask
    #[inline]
    pub fn get_rhc_int_mask(&self) -> bool {
        matches!(self.imr().read().rhc().bits() & self.mask(), 1)
    }

    /// Region Status Updated interrupt status
    #[inline]
    pub fn get_rsu_int(&self) -> bool {
        matches!(self.isr().read().rsu().bits() & self.mask(), 1)
    }

    /// Region End bit Condition Detected interrupt status
    #[inline]
    pub fn get_rec_int(&self) -> bool {
        matches!(self.isr().read().rec().bits() & self.mask(), 1)
    }

    /// Region Wrap Condition detected interrupt status
    #[inline]
    pub fn get_rwc_int(&self) -> bool {
        matches!(self.isr().read().rwc().bits() & self.mask(), 1)
    }

    /// Region Bus Error interrupt status
    #[inline]
    pub fn get_rbe_int(&self) -> bool {
        matches!(self.isr().read().rbe().bits() & self.mask(), 1)
    }

    /// Region Digest Mismatches!( interrupt status
    #[inline]
    pub fn get_rdm_int(&self) -> bool {
        matches!(self.isr().read().rdm().bits() & self.mask(), 1)
    }

    /// Region Hash Completed interrupt status
    #[inline]
    pub fn get_rhc_int(&self) -> bool {
        matches!(self.isr().read().rhc().bits() & self.mask(), 1)
    }

    /// When reading the interrupt (ISR) register, it is cleared
    ///
    /// This might result in unexpected results for example
    /// when sequentially trying to determine which interrupt triggered.
    ///
    /// This is an alternative, return all the data from the register
    /// and parse later with the designated `get_[name]_int` functions.
    #[inline]
    pub fn get_interrupt_status(&mut self) -> IcmRegionInterrupt<I> {
        let interrupt_vector = self.isr().read().bits();
        IcmRegionInterrupt {
            region: PhantomData,
            interrupt_vector,
        }
    }
}

/// ICM Peripheral
///
/// Encapsulates the [`IcmToken`] and provides an interface
/// to the ICM hardware
pub struct Icm {
    /// IcmToken providing hardware access
    token: IcmToken,
}

impl Icm {
    /// Enable the ICM peripheral
    #[inline]
    pub fn new(token: IcmToken) -> Self {
        Self {
            token,
            interrupt_cache: 0,
        }
    }

    /// Enable the ICM peripheral
    #[inline]
    pub fn enable(&mut self) {
        self.token.ctrl().write(|w| w.enable().set_bit());
    }

    /// Get enabled status of the ICM peripheral
    #[inline]
    pub fn icm_status(&self) -> bool {
        self.token.sr().read().enable().bit_is_set()
    }

    /// Disable the ICM peripheral
    #[inline]
    pub fn disable(&mut self) {
        self.token.ctrl().write(|w| w.disable().set_bit());
    }

    /// Reset the ICM controller
    ///
    /// Does not seem to clear DSCR, HASH addr
    ///
    /// The only way to clear the `URAD` and `URAT` fields
    /// is by resetting the ICM controller
    #[inline]
    pub fn swrst(&mut self) {
        self.token.ctrl().write(|w| w.swrst().set_bit());
    }

    /// Destroy the ICM peripheral and return the underlying ICM register
    #[inline]
    pub fn destroy(self) -> crate::pac::ICM {
        self.token.icm
    }

    // Region specifics

    /// Enable region0
    ///
    /// Creates an [`IcmRegionToken`] which provides region specific
    /// settings
    #[inline]
    pub fn enable_region0(&mut self) -> IcmRegionToken<Region0> {
        IcmRegionToken::new()
    }
    /// Enable region1
    ///
    /// Creates an [`IcmRegionToken`] which provides region specific
    /// settings
    #[inline]
    pub fn enable_region1(&mut self) -> IcmRegionToken<Region1> {
        IcmRegionToken::new()
    }
    /// Enable region2
    ///
    /// Creates an [`IcmRegionToken`] which provides region specific
    /// settings
    #[inline]
    pub fn enable_region2(&mut self) -> IcmRegionToken<Region2> {
        IcmRegionToken::new()
    }
    /// Enable region3
    ///
    /// Creates an [`IcmRegionToken`] which provides region specific
    /// settings
    #[inline]
    pub fn enable_region3(&mut self) -> IcmRegionToken<Region3> {
        IcmRegionToken::new()
    }

    // Configuration of Icm

    /// Helper for setting the HASH addr
    ///
    /// Expects a raw pointer to the memory address of the beginning of the
    /// designated variable but expressed as a multiple of 128
    #[inline]
    pub fn set_hash_addr(&mut self, hash_addr_pointer: &IcmHashArea) {
        self.token
            .hash()
            .write(|w| unsafe { w.hasa().bits((hash_addr_pointer as *const _) as u32 / 128) })
    }

    /// Set the DSCR addr to a specific IcmRegionDesc
    ///
    /// HW expects a raw pointer to the memory address of the beginning of the
    /// [`IcmRegionDesc`] but expressed as a multiple of 64
    pub fn set_dscr_addr(&self, icm_region_desc: &IcmRegionDesc<Region0>) {
        self.token
            .dscr()
            .write(|w| unsafe { w.dasa().bits((icm_region_desc as *const _) as u32 / 64) })
    }

    /// Set the user initial hash value
    #[inline]
    pub fn set_user_initial_hash_value(&self, user_initial_hash_value: [u32; 8]) {
        for (index, initial_value) in user_initial_hash_value.iter().enumerate() {
            self.token.icm().uihval[index].write(|w| unsafe { w.val().bits(*initial_value) });
        }
    }

    /// Set the user hashing algorithm
    #[inline]
    pub fn set_user_algorithm(self, algo: icm_algorithm) {
        self.token.cfg().write(|w| w.ualgo().variant(algo));
    }

    /// Activate user hash mode
    ///
    /// Allows providing
    /// * hash initial value
    /// * Hash algorithm
    ///
    /// Disables the `ALGO` field in [`IcmRegionDesc`]
    ///
    /// Set initial hash value via [`Icm::set_user_initial_hash_value()`]
    /// Set hash algorithm via [`Icm::set_user_algorithm()`]
    #[inline]
    pub fn set_user_configurable_hash(&self, user_configurable_hash: bool) {
        self.token
            .cfg()
            .write(|w| w.uihash().bit(user_configurable_hash));
    }

    /// Control dual input buffer
    ///
    /// Enabling dual input buffering allow for better performance
    /// at the cost of higher bandwith requirements on the system bus
    #[inline]
    pub fn set_dual_input_buffer(&self, dualbuffer: bool) {
        self.token.cfg().write(|w| w.dualbuff().bit(dualbuffer));
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
        self.token.cfg().write(|w| w.ascd().bit(automaticswitch));
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
        self.token
            .cfg()
            .write(|w| unsafe { w.bbc().bits(busburden) });
    }

    /// Secondary List Branching Disable
    ///
    /// * If set to false, allow secondary lists
    /// * If set to true, secondary lists are forbidden, `NEXT` and `RNEXT` is
    ///   always considered 0.
    #[inline]
    pub fn set_slbdis(&self, disable_secondary_lists: bool) {
        self.token
            .cfg()
            .write(|w| w.slbdis().bit(disable_secondary_lists));
    }

    /// End of Monitoring Disable
    ///
    /// * If set to false, End of Monitoring is permitter
    /// * If set to true, End of Monitoring is forbidden, the EOM bit in RCFG
    ///   has no effect.
    #[inline]
    pub fn set_eomdis(&self, disable_eom: bool) {
        self.token.cfg().write(|w| w.eomdis().bit(disable_eom));
    }

    /// Write Back Disable
    ///
    /// * If set to false, Write Back Operations are permitted
    /// * If set to true, Write Back Operations are forbidden, `CDWBN` bit is
    ///   internally set to 1 and cannot be modified by a linked list element.
    ///   The `CDWBN` bit of the `RCFG` structure member has no effect.
    #[inline]
    pub fn set_wbdis(&self, disable_eom: bool) {
        self.token.cfg().write(|w| w.wbdis().bit(disable_eom));
    }

    // Security and tamper settings

    /// Set Undefined Register Access Detection interrupt enable
    #[inline]
    pub fn set_urad_int(self) {
        self.token.ier().write(|w| unsafe { w.rsu().bits(1) });
    }

    /// Disable Undefined Register Access Detection interrupt enable
    #[inline]
    pub fn disable_urad_int(self) {
        self.token.idr().write(|w| unsafe { w.rsu().bits(1) });
    }

    /// Get Undefined Register Access Detection interrupt mask
    #[inline]
    pub fn get_urad_int_mask(&self) -> bool {
        self.token.imr().read().urad().bits()
    }

    /// Get Undefined Register Access Detection interrupt status
    #[inline]
    pub fn get_urad_int(&self) -> bool {
        self.token.isr().read().urad().bits()
    }

    /// Get Undefined Register Access Trace
    ///
    /// This field is only reset by `swrst`
    #[inline]
    pub fn get_urat(&self) -> Variant<u8, URAT_A> {
        self.token.uasr().read().urat().variant()
    }

    /// When reading the interrupt (ISR) register, it is cleared
    ///
    /// This might result in unexpected results for example
    /// when sequentially trying to determine which interrupt triggered.
    ///
    /// This is an alternative, return all the data from the register
    /// and parse later with the designated `get_[name]_int` functions.
    #[inline]
    pub fn get_interrupt_status(&mut self) -> IcmInterrupt {
        let interrupt_vector = self.token.isr().read().bits();
        IcmInterrupt { interrupt_vector }
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
        self.token
            .ctrl()
            .write(|w| unsafe { w.rehash().bits(bitmask) });
    }
}

/// Trait providing numerical identifier and
/// offset for each ICM Region
///
/// ICM supports 4 memory regions
pub trait IcmRegionNum: Sealed {
    /// Numerical ID of the memory region
    const NUM: usize;
    /// Memory offset
    const OFFSET: u32;
}

/// ICM Region 0
pub enum Region0 {}
impl Sealed for Region0 {}
impl IcmRegionNum for Region0 {
    const NUM: usize = 0;
    const OFFSET: u32 = 0;
}

/// ICM Region 1
pub enum Region1 {}
impl Sealed for Region1 {}
impl IcmRegionNum for Region1 {
    const NUM: usize = 1;
    const OFFSET: u32 = 0x10;
}

/// ICM Region 2
pub enum Region2 {}
impl Sealed for Region2 {}
impl IcmRegionNum for Region2 {
    const NUM: usize = 2;
    const OFFSET: u32 = 0x10 * 2;
}

/// ICM Region 3
pub enum Region3 {}
impl Sealed for Region3 {}
impl IcmRegionNum for Region3 {
    const NUM: usize = 3;
    const OFFSET: u32 = 0x10 * 3;
}

/// Functions required by [`IcmRegionDesc`]
///
/// Both Main List descriptors and Secondary List descriptors
pub trait IcmRegionDescT {
    /// Set the [`Raddr`] start of the region memory region
    fn set_raddr<T>(&mut self, addr: *const T);
    /// Set the specific region configuration
    fn set_rcfg(&mut self, cfg: Rcfg);
    /// Set the link to next region descriptor
    fn set_rnext(&mut self, next: Rnext);
    /// Reset Rcfg to default values
    fn reset_rcfg_to_default(&mut self);
}

/// Helper for creating the Region Descriptor structure
///
/// It is also possible to construct the IcmRegion manually,
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
pub struct IcmRegion {
    /// IcmRegionDesc0
    pub region0: IcmRegionDesc<Region0>,
    /// IcmRegionDesc1
    pub region1: IcmRegionDesc<Region1>,
    /// IcmRegionDesc2
    pub region2: IcmRegionDesc<Region2>,
    /// IcmRegionDesc3
    pub region3: IcmRegionDesc<Region3>,
}

impl IcmRegion {
    pub const fn default() -> Self {
        let region0 = IcmRegionDesc::new_region0();
        let region1 = IcmRegionDesc::new_region1();
        let region2 = IcmRegionDesc::new_region2();
        let region3 = IcmRegionDesc::new_region3();
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
/// being a part of the 64-bytes making up [`IcmRegion`]
#[repr(C)]
#[repr(align(16))]
pub struct IcmRegionDesc<N: IcmRegionNum> {
    /// Numerical Region Identifier
    num: PhantomData<N>,
    /// The first byte address of the Region.
    pub raddr: Raddr,
    /// Configuration Structure Member.
    pub rcfg: Rcfg,
    /// Control Structure Member.
    pub rctrl: Rctrl,
    /// Next Address Structure Member.
    pub rnext: Rnext,
}

impl IcmRegionDesc<Region0> {
    /// Helper for setting the DSCR addr to a the first IcmRegionDesc
    ///
    /// See [`Icm::set_dscr_addr()`] for the regular workflow
    ///
    /// HW expects a raw pointer to the memory address of the beginning of the
    /// [`IcmRegionDesc`] but expressed as a multiple of 64
    pub fn set_dscr_addr(&self, icm: &Icm) {
        icm.token
            .dscr()
            .write(|w| unsafe { w.dasa().bits((self as *const _) as u32 / 64) })
    }

    const fn new_region0() -> Self {
        IcmRegionDesc {
            num: PhantomData,
            raddr: Raddr::default(),
            rcfg: Rcfg::default(),
            rctrl: Rctrl::default(),
            rnext: Rnext::default(),
        }
    }
}
impl IcmRegionDesc<Region1> {
    const fn new_region1() -> Self {
        IcmRegionDesc {
            num: PhantomData,
            raddr: Raddr::default(),
            rcfg: Rcfg::default(),
            rctrl: Rctrl::default(),
            rnext: Rnext::default(),
        }
    }
}
impl IcmRegionDesc<Region2> {
    const fn new_region2() -> Self {
        IcmRegionDesc {
            num: PhantomData,
            raddr: Raddr::default(),
            rcfg: Rcfg::default(),
            rctrl: Rctrl::default(),
            rnext: Rnext::default(),
        }
    }
}
impl IcmRegionDesc<Region3> {
    const fn new_region3() -> Self {
        IcmRegionDesc {
            num: PhantomData,
            raddr: Raddr::default(),
            rcfg: Rcfg::default(),
            rctrl: Rctrl::default(),
            rnext: Rnext::default(),
        }
    }
}

impl<N: IcmRegionNum> IcmRegionDescT for IcmRegionDesc<N> {
    /// Set [`Raddr`]
    #[inline]
    fn set_raddr<T>(&mut self, addr: *const T) {
        self.raddr.set_raddr(addr);
    }
    /// Set [`Rcfg`]
    #[inline]
    fn set_rcfg(&mut self, cfg: Rcfg) {
        self.rcfg = cfg;
    }
    /// Reset [`Rcfg`] to default values
    #[inline]
    fn reset_rcfg_to_default(&mut self) {
        self.rcfg = Rcfg::default();
    }
    /// Set [`Rnext`]
    #[inline]
    fn set_rnext(&mut self, next: Rnext) {
        self.rnext = next;
    }
}

impl<N: IcmRegionNum> IcmRegionDesc<N> {
    /// The length of data for the ICM engine to transfer,
    /// expressed as number of `blocks - 1`.
    #[inline]
    pub fn set_rctrl(mut self, ctrl: Rctrl) {
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
pub struct IcmSecondaryRegionDesc {
    /// the first byte address of the Region.
    pub raddr: Raddr,
    /// Configuration Structure Member.
    pub rcfg: Rcfg,
    /// Not used in Secondary Region Descriptor
    _pad: u32,
    /// Next Address Structure Member.
    pub rnext: Rnext,
}

impl IcmRegionDescT for IcmSecondaryRegionDesc {
    fn set_raddr<T>(&mut self, addr: *const T) {
        self.raddr.set_raddr(addr);
    }
    /// Set [`Rcfg`]
    #[inline]
    fn set_rcfg(&mut self, cfg: Rcfg) {
        self.rcfg = cfg;
    }
    /// Reset [`Rcfg`] to default values
    #[inline]
    fn reset_rcfg_to_default(&mut self) {
        self.rcfg = Rcfg::default();
    }
    /// Set [`Rnext`]
    #[inline]
    fn set_rnext(&mut self, next: Rnext) {
        self.rnext = next;
    }
}

impl IcmSecondaryRegionDesc {
    pub const fn default() -> Self {
        IcmSecondaryRegionDesc {
            raddr: Raddr::default(),
            rcfg: Rcfg::default(),
            _pad: 0,
            rnext: Rnext::default(),
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
pub struct IcmHashArea {
    pub region0: [u32; 8],
    pub region1: [u32; 8],
    pub region2: [u32; 8],
    pub region3: [u32; 8],
}

impl IcmHashArea {
    pub fn region0(&self) -> [u32; 8] {
        self.region0
    }
    pub fn region1(&self) -> [u32; 8] {
        self.region1
    }
    pub fn region2(&self) -> [u32; 8] {
        self.region2
    }
    pub fn region3(&self) -> [u32; 8] {
        self.region3
    }
    pub const fn default() -> Self {
        IcmHashArea {
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
pub struct Raddr {
    pub raddr: u32,
}

impl Raddr {
    pub fn set_raddr<T>(&mut self, raddr: *const T) {
        self.raddr = raddr as u32;
    }
    const fn default() -> Self {
        // Unsure what a good Raddr default should be,
        // this is at least easily spotted when debugging...
        Raddr {
            raddr: 0xDEADBEEF_u32,
        }
    }
}

/// Region Configuration Structure
///
/// Follows C-structure conventions
#[derive(Clone, Copy)]
#[repr(C)]
pub struct Rcfg {
    rcfg: u32,
}

impl Rcfg {
    const fn default() -> Self {
        Rcfg { rcfg: 0x3F0 }
    }
}

impl Rcfg {
    /// Compare Digest or Write Back Digest
    ///
    /// * true: in Compare Digest mode.
    /// * false: in Write Back Digest mode.
    #[inline]
    pub fn set_cdwbn(&mut self, value: bool) {
        self.rcfg = (self.rcfg & !0x01) | (value as u32);
    }
    /// Wrap Command
    ///
    /// * true: next region descriptor address loaded is `DSCR`.
    /// * false: the next region descriptor address is `current + 0x10`.
    #[inline]
    pub fn set_wrap(&mut self, value: bool) {
        self.rcfg = (self.rcfg & !(0x01 << 1)) | ((value as u32) << 1);
    }
    /// End of Monitoring
    ///
    /// * true: the current descriptor terminates the Main List, WRAP bit has
    /// no effect.
    /// * false: the current descriptor does not terminate the
    /// monitoring.
    #[inline]
    pub fn set_eom(&mut self, value: bool) {
        self.rcfg = (self.rcfg & !(0x01 << 2)) | ((value as u32) << 2);
    }
    /// Region Hash Completed Interrupt Disable
    /// * true: the `RHC` flag remains cleared even if the setting condition is
    /// met.
    /// * false: the `RHC` flag is set when the field `NEXT = 0` is
    /// found in main or secondary list.
    #[inline]
    pub fn set_rhien(&mut self, value: bool) {
        self.rcfg = (self.rcfg & !(0x01 << 4)) | ((value as u32) << 4);
    }
    /// Digest Mismatch Interrupt Disable
    ///
    /// * true: the `RBE` flag remains cleared even if the setting condition is
    /// met.
    /// * false: the `RBE` flag is set when the hash value just
    /// calculated from the processed region differs from expected hash
    /// value.
    #[inline]
    pub fn set_dmien(&mut self, value: bool) {
        self.rcfg = (self.rcfg & !(0x01 << 5)) | ((value as u32) << 5);
    }
    /// Bus Error Interrupt Disable
    ///
    /// * true: the flag remains cleared even if the setting condition is met.
    /// * false: the flag is set when an error is reported on the sysstem bus
    /// by the bus MATRIX.
    #[inline]
    pub fn set_beien(&mut self, value: bool) {
        self.rcfg = (self.rcfg & !(0x01 << 6)) | ((value as u32) << 6);
    }
    /// Wrap Condition Interrupt Disable
    ///
    /// * true: the `RWC` flag remains cleared even if the setting condition is
    /// met.
    /// * false: the `RWC` flag is set when the WRAP is encountered.
    #[inline]
    pub fn set_wcien(&mut self, value: bool) {
        self.rcfg = (self.rcfg & !(0x01 << 7)) | ((value as u32) << 7);
    }
    /// End Bit Condition Interrupt Enable
    ///
    /// * true: the `REC` flag remains cleared even if the setting condition is
    /// met.
    /// * false: the `REC` flag is set when the descriptor having the
    /// `EOM` bit set is processed.
    #[inline]
    pub fn set_ecien(&mut self, value: bool) {
        self.rcfg = (self.rcfg & !(0x01 << 8)) | ((value as u32) << 8);
    }
    /// Monitoring Status Updated Condition Interrupt Enable
    ///
    /// * true: the `RSU` flag remains cleared even if the condition is met.
    /// * false: the `RSU` flag is set when the corresponding descriptor is
    /// loaded from memory to ICM.
    #[inline]
    pub fn set_suien(&mut self, value: bool) {
        self.rcfg = (self.rcfg & !(0x01 << 9)) | ((value as u32) << 9);
    }
    /// Processing Delay
    ///
    /// Allows setting short or long delay.
    ///
    /// See [`Procdly`]
    #[inline]
    pub fn set_procdly(&mut self, value: Procdly) {
        self.rcfg = (self.rcfg & !(0x01 << 10)) | ((value as u32) << 10);
    }
    /// User SHA Algorithm
    ///
    /// Allow setting this regions [`icm_algorithm`].
    #[inline]
    pub fn set_algo(&mut self, value: icm_algorithm) {
        self.rcfg = (self.rcfg & !(0x07 << 12)) | (((value as u32) & 0x07) << 12);
    }
    /// Reset the [`Rcfg`] to default values
    ///
    /// Useful if changing between hashing and monitoring, etc.
    #[inline]
    pub fn reset_rcfg_to_default(&mut self) {
        self.rcfg = 0x3F0;
    }
}

/// Processing Delay
///
/// For a given SHA algorithm, the runtime period has two possible lengths
pub enum Procdly {
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
pub struct Rctrl {
    pub trsize: u16,
}

impl Rctrl {
    const fn default() -> Self {
        Rctrl { trsize: 0 }
    }
}

/// Region Next Address Structure
///
/// Is the same as Raddr<N+1>
///
/// Follows C-structure conventions
#[derive(Clone, Copy)]
#[repr(C)]
pub struct Rnext {
    rnext: u32,
}

impl Rnext {
    pub fn set_rnext(&mut self, rnext: &impl IcmRegionDescT) {
        self.rnext = (rnext as *const _) as u32;
    }

    const fn default() -> Self {
        Rnext { rnext: 0 }
    }
}
