#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control"]
    pub ctrl: crate::Reg<ctrl::CTRL_SPEC>,
    #[doc = "0x02 - CRC Control"]
    pub crcctrl: crate::Reg<crcctrl::CRCCTRL_SPEC>,
    #[doc = "0x04 - CRC Data Input"]
    pub crcdatain: crate::Reg<crcdatain::CRCDATAIN_SPEC>,
    #[doc = "0x08 - CRC Checksum"]
    pub crcchksum: crate::Reg<crcchksum::CRCCHKSUM_SPEC>,
    #[doc = "0x0c - CRC Status"]
    pub crcstatus: crate::Reg<crcstatus::CRCSTATUS_SPEC>,
    #[doc = "0x0d - Debug Control"]
    pub dbgctrl: crate::Reg<dbgctrl::DBGCTRL_SPEC>,
    _reserved6: [u8; 0x02],
    #[doc = "0x10 - Software Trigger Control"]
    pub swtrigctrl: crate::Reg<swtrigctrl::SWTRIGCTRL_SPEC>,
    #[doc = "0x14 - Priority Control 0"]
    pub prictrl0: crate::Reg<prictrl0::PRICTRL0_SPEC>,
    _reserved8: [u8; 0x08],
    #[doc = "0x20 - Interrupt Pending"]
    pub intpend: crate::Reg<intpend::INTPEND_SPEC>,
    _reserved9: [u8; 0x02],
    #[doc = "0x24 - Interrupt Status"]
    pub intstatus: crate::Reg<intstatus::INTSTATUS_SPEC>,
    #[doc = "0x28 - Busy Channels"]
    pub busych: crate::Reg<busych::BUSYCH_SPEC>,
    #[doc = "0x2c - Pending Channels"]
    pub pendch: crate::Reg<pendch::PENDCH_SPEC>,
    #[doc = "0x30 - Active Channel and Levels"]
    pub active: crate::Reg<active::ACTIVE_SPEC>,
    #[doc = "0x34 - Descriptor Memory Section Base Address"]
    pub baseaddr: crate::Reg<baseaddr::BASEADDR_SPEC>,
    #[doc = "0x38 - Write-Back Memory Section Base Address"]
    pub wrbaddr: crate::Reg<wrbaddr::WRBADDR_SPEC>,
    _reserved15: [u8; 0x04],
    #[doc = "0x40..0x240 - CHANNEL\\[%s\\]"]
    pub channel: [CHANNEL; 32],
}
#[doc = r"Register block"]
#[repr(C)]
pub struct CHANNEL {
    #[doc = "0x00 - Channel n Control A"]
    pub chctrla: crate::Reg<self::channel::chctrla::CHCTRLA_SPEC>,
    #[doc = "0x04 - Channel n Control B"]
    pub chctrlb: crate::Reg<self::channel::chctrlb::CHCTRLB_SPEC>,
    #[doc = "0x05 - Channel n Priority Level"]
    pub chprilvl: crate::Reg<self::channel::chprilvl::CHPRILVL_SPEC>,
    #[doc = "0x06 - Channel n Event Control"]
    pub chevctrl: crate::Reg<self::channel::chevctrl::CHEVCTRL_SPEC>,
    _reserved4: [u8; 0x05],
    #[doc = "0x0c - Channel n Interrupt Enable Clear"]
    pub chintenclr: crate::Reg<self::channel::chintenclr::CHINTENCLR_SPEC>,
    #[doc = "0x0d - Channel n Interrupt Enable Set"]
    pub chintenset: crate::Reg<self::channel::chintenset::CHINTENSET_SPEC>,
    #[doc = "0x0e - Channel n Interrupt Flag Status and Clear"]
    pub chintflag: crate::Reg<self::channel::chintflag::CHINTFLAG_SPEC>,
    #[doc = "0x0f - Channel n Status"]
    pub chstatus: crate::Reg<self::channel::chstatus::CHSTATUS_SPEC>,
}
#[doc = r"Register block"]
#[doc = "CHANNEL\\[%s\\]"]
pub mod channel;
#[doc = "CTRL register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Control"]
pub mod ctrl;
#[doc = "CRCCTRL register accessor: an alias for `Reg<CRCCTRL_SPEC>`"]
pub type CRCCTRL = crate::Reg<crcctrl::CRCCTRL_SPEC>;
#[doc = "CRC Control"]
pub mod crcctrl;
#[doc = "CRCDATAIN register accessor: an alias for `Reg<CRCDATAIN_SPEC>`"]
pub type CRCDATAIN = crate::Reg<crcdatain::CRCDATAIN_SPEC>;
#[doc = "CRC Data Input"]
pub mod crcdatain;
#[doc = "CRCCHKSUM register accessor: an alias for `Reg<CRCCHKSUM_SPEC>`"]
pub type CRCCHKSUM = crate::Reg<crcchksum::CRCCHKSUM_SPEC>;
#[doc = "CRC Checksum"]
pub mod crcchksum;
#[doc = "CRCSTATUS register accessor: an alias for `Reg<CRCSTATUS_SPEC>`"]
pub type CRCSTATUS = crate::Reg<crcstatus::CRCSTATUS_SPEC>;
#[doc = "CRC Status"]
pub mod crcstatus;
#[doc = "DBGCTRL register accessor: an alias for `Reg<DBGCTRL_SPEC>`"]
pub type DBGCTRL = crate::Reg<dbgctrl::DBGCTRL_SPEC>;
#[doc = "Debug Control"]
pub mod dbgctrl;
#[doc = "SWTRIGCTRL register accessor: an alias for `Reg<SWTRIGCTRL_SPEC>`"]
pub type SWTRIGCTRL = crate::Reg<swtrigctrl::SWTRIGCTRL_SPEC>;
#[doc = "Software Trigger Control"]
pub mod swtrigctrl;
#[doc = "PRICTRL0 register accessor: an alias for `Reg<PRICTRL0_SPEC>`"]
pub type PRICTRL0 = crate::Reg<prictrl0::PRICTRL0_SPEC>;
#[doc = "Priority Control 0"]
pub mod prictrl0;
#[doc = "INTPEND register accessor: an alias for `Reg<INTPEND_SPEC>`"]
pub type INTPEND = crate::Reg<intpend::INTPEND_SPEC>;
#[doc = "Interrupt Pending"]
pub mod intpend;
#[doc = "INTSTATUS register accessor: an alias for `Reg<INTSTATUS_SPEC>`"]
pub type INTSTATUS = crate::Reg<intstatus::INTSTATUS_SPEC>;
#[doc = "Interrupt Status"]
pub mod intstatus;
#[doc = "BUSYCH register accessor: an alias for `Reg<BUSYCH_SPEC>`"]
pub type BUSYCH = crate::Reg<busych::BUSYCH_SPEC>;
#[doc = "Busy Channels"]
pub mod busych;
#[doc = "PENDCH register accessor: an alias for `Reg<PENDCH_SPEC>`"]
pub type PENDCH = crate::Reg<pendch::PENDCH_SPEC>;
#[doc = "Pending Channels"]
pub mod pendch;
#[doc = "ACTIVE register accessor: an alias for `Reg<ACTIVE_SPEC>`"]
pub type ACTIVE = crate::Reg<active::ACTIVE_SPEC>;
#[doc = "Active Channel and Levels"]
pub mod active;
#[doc = "BASEADDR register accessor: an alias for `Reg<BASEADDR_SPEC>`"]
pub type BASEADDR = crate::Reg<baseaddr::BASEADDR_SPEC>;
#[doc = "Descriptor Memory Section Base Address"]
pub mod baseaddr;
#[doc = "WRBADDR register accessor: an alias for `Reg<WRBADDR_SPEC>`"]
pub type WRBADDR = crate::Reg<wrbaddr::WRBADDR_SPEC>;
#[doc = "Write-Back Memory Section Base Address"]
pub mod wrbaddr;
