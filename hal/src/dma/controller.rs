//! Direct Memory Access Controller (DMAC)
//!
//! This module provides a `dma::controller::Registers` trait (defined
//! differently for SAMD21 vs SAMD51) which abstracts over the DMAC
//! types in various `SAMD*` controllers, and allows writing DMA drivers in
//! an abstract way.
//!
//! It's designed to be used with the controller-specific DMAC types generated
//! automatically by svd2rust, which are located in the `dmac` module of each
//! of the respective controller crates.
//!
//! The `impl_dmac_registers!` macro can be used to automatically impl the
//! trait against such a type.

/// Define trait-based readers for DMAC registers
macro_rules! dmac_register {
    ($read:ident, $type:ty, $doc:expr) => {
        #[doc = "Perform volatile read from register "]
        #[doc = $doc]
        fn $read(&self) -> $type;
    }
}

/// Define trait-based readers and writers for DMAC registers
macro_rules! dmac_io_register {
    ($read:ident, $write:ident, $type:ty, $doc:expr) => {
        dmac_register!($read, $type, $doc);

        #[doc = "Perform volatile write to register"]
        #[doc = $doc]
        unsafe fn $write(&self, value: $type);
    }
}

/// DMAC registers common to both SAMD21 and SAMD51
macro_rules! dmac_common_registers {
    () => {
        dmac_io_register!(read_ctrl, write_ctrl, u16, "0x00 - Control");
        dmac_io_register!(read_crcctrl, write_crcctrl, u16, "0x02 - CRC Control");
        dmac_io_register!(read_crcdatain, write_crcdatain, u32, "0x04 - CRC Data Input");
        dmac_io_register!(read_crcchksum, write_crcchksum, u32, "0x08 - CRC Checksum");
        dmac_io_register!(read_crcstatus, write_crcstatus, u8, "0x0c - CRC Status");
        dmac_io_register!(read_dbgctrl, write_dbgctrl, u8, "0x0d - Debug Control");
        dmac_io_register!(read_swtrigctrl, write_swtrigctrl, u32, "0x10 - Software Trigger Control");
        dmac_io_register!(read_prictrl0, write_prictrl0, u32, "0x14 - Priority Control 0");
        dmac_io_register!(read_intpend, write_intpend, u16, "0x20 - Interrupt Pending");
        dmac_register!(read_intstatus, u32, "0x24 - Interrupt Status");
        dmac_register!(read_busych, u32, "0x28 - Busy Channels");
        dmac_register!(read_pendch, u32, "0x2c - Pending Channels");
        dmac_register!(read_active, u32, "0x30 - Active Channel and Levels");
        dmac_io_register!(read_baseaddr, write_baseaddr, u32, "0x34 - Descriptor Memory Section Base Address");
        dmac_io_register!(read_wrbaddr, write_wrbaddr, u32, "0x38 - Write-Back Memory Section Base Address");
    }
}

/// Impl trait-based readers for DMAC registers
macro_rules! impl_dmac_register {
    ($field:ident, $read:ident, $struct:ty, $type:ty) => {
        fn $read(&self) -> $type {
            self.$field.read().bits()
        }
    }
}

/// Impl trait-based readers and writers for DMAC registers
macro_rules! impl_dmac_io_register {
    ($field:ident, $read:ident, $write:ident, $struct:ty, $type:ty) => {
        impl_dmac_register!($field, $read, $struct, $type);

        unsafe fn $write(&self, value: $type) {
            self.$field.write(|reg| reg.bits(value));
        }
    }
}

/// Impl the `Registers` trait for registers common to both SAMD21 and SAMD51
macro_rules! impl_dmac_common_registers {
    () => {
        impl_dmac_io_register!(ctrl, read_ctrl, write_ctrl, CTRL, u16);
        impl_dmac_io_register!(crcctrl, read_crcctrl, write_crcctrl, CRCCTRL, u16);
        impl_dmac_io_register!(crcdatain, read_crcdatain, write_crcdatain, CRCDATAIN, u32);
        impl_dmac_io_register!(crcchksum, read_crcchksum, write_crcchksum, CRCCHKSUM, u32);
        impl_dmac_io_register!(crcstatus, read_crcstatus, write_crcstatus, CRCSTATUS, u8);
        impl_dmac_io_register!(dbgctrl, read_dbgctrl, write_dbgctrl, DBGCTRL, u8);
        impl_dmac_io_register!(swtrigctrl, read_swtrigctrl, write_swtrigctrl, SWTRIGCTRL, u32);
        impl_dmac_io_register!(prictrl0, read_prictrl0, write_prictrl0, PRICTRL0, u32);
        impl_dmac_io_register!(intpend, read_intpend, write_intpend, INTPEND, u16);
        impl_dmac_register!(intstatus, read_intstatus, INTSTATUS, u32);
        impl_dmac_register!(busych, read_busych, BUSYCH, u32);
        impl_dmac_register!(pendch, read_pendch, PENDCH, u32);
        impl_dmac_register!(active, read_active, ACTIVE, u32);
        impl_dmac_io_register!(baseaddr, read_baseaddr, write_baseaddr, BASEADDR, u32);
        impl_dmac_io_register!(wrbaddr, read_wrbaddr, write_wrbaddr, WRBADDR, u32);
    }
}

/// SAMD21 DMAC registers
#[cfg(not(feature = "samd51"))]
mod samd21 {
    use crate::target_device::dmac::RegisterBlock;

    /// Number of DMA channels
    pub const NUM_CHANNELS: usize = 1;

    /// DMAC registers (SAMD21)
    pub trait Registers {
        // Define accessors for registers common to SAMD21 and SAMD51
        dmac_common_registers!();
        dmac_io_register!(read_qosctrl, write_qosctrl, u8, "0x0e - QOS control");
        dmac_io_register!(read_chid, write_chid, u8, "0x3f - Channel ID");
        dmac_io_register!(read_chctrla, write_chctrla, u8, "0x40 - Channel control (A)");
        dmac_io_register!(read_chctrlb, write_chctrlb, u32, "0x44 - Channel control (B)");
        dmac_io_register!(read_chintenclr, write_chintenclr, u8, "0x4c - Channel interrupt enable clear");
        dmac_io_register!(read_chintenset, write_chintenset, u8, "0x4d - Channel interrupt enable set");
        dmac_io_register!(read_chintflag, write_chintflag, u8, "0x4e - Channel interrupt flag");
        dmac_register!(read_chstatus, u8, "0x4f - Channel status");
    }

    impl Registers for RegisterBlock {
        impl_dmac_common_registers!();
        impl_dmac_io_register!(qosctrl, read_qosctrl, write_qosctrl, QOSCTRL, u8);
        impl_dmac_io_register!(chid, read_chid, write_chid, CHID, u8);
        impl_dmac_io_register!(chctrla, read_chctrla, write_chctrla, CHCTRLA, u8);
        impl_dmac_io_register!(chctrlb, read_chctrlb, write_chctrlb, CHCTRLB, u32);
        impl_dmac_io_register!(chintenclr, read_chintenclr, write_chintenclr, CHINTENCLR, u8);
        impl_dmac_io_register!(chintenset, read_chintenset, write_chintenset, CHINTENSET, u8);
        impl_dmac_io_register!(chintflag, read_chintflag, write_chintflag, CHINTFLAG, u8);
        impl_dmac_register!(chstatus, read_chstatus, CHSTATUS, u8);
    }
}

#[cfg(not(feature = "samd51"))]
pub use samd21::*;

/// SAMD51 DMAC registers
#[cfg(feature = "samd51")]
mod samd51 {
    use crate::target_device::dmac::RegisterBlock;

    /// Number of DMA channels
    pub const NUM_CHANNELS: usize = 32;

    /// DMAC registers (SAMD51)
    pub trait Registers {
        // Define accessors for registers common to SAMD21 and SAMD51
        dmac_common_registers!();
    }

    impl Registers for RegisterBlock {
        // Impl the `Registers` trait for the target device's DMAC registers type
        impl_dmac_common_registers!();
    }
}

#[cfg(feature = "samd51")]
pub use samd51::*;
