#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control"]
    pub ctrl: CTRL,
    #[doc = "0x02 - CRC Control"]
    pub crcctrl: CRCCTRL,
    #[doc = "0x04 - CRC Data Input"]
    pub crcdatain: CRCDATAIN,
    #[doc = "0x08 - CRC Checksum"]
    pub crcchksum: CRCCHKSUM,
    #[doc = "0x0c - CRC Status"]
    pub crcstatus: CRCSTATUS,
    #[doc = "0x0d - Debug Control"]
    pub dbgctrl: DBGCTRL,
    #[doc = "0x0e - QOS Control"]
    pub qosctrl: QOSCTRL,
    _reserved7: [u8; 1usize],
    #[doc = "0x10 - Software Trigger Control"]
    pub swtrigctrl: SWTRIGCTRL,
    #[doc = "0x14 - Priority Control 0"]
    pub prictrl0: PRICTRL0,
    _reserved9: [u8; 8usize],
    #[doc = "0x20 - Interrupt Pending"]
    pub intpend: INTPEND,
    _reserved10: [u8; 2usize],
    #[doc = "0x24 - Interrupt Status"]
    pub intstatus: INTSTATUS,
    #[doc = "0x28 - Busy Channels"]
    pub busych: BUSYCH,
    #[doc = "0x2c - Pending Channels"]
    pub pendch: PENDCH,
    #[doc = "0x30 - Active Channel and Levels"]
    pub active: ACTIVE,
    #[doc = "0x34 - Descriptor Memory Section Base Address"]
    pub baseaddr: BASEADDR,
    #[doc = "0x38 - Write-Back Memory Section Base Address"]
    pub wrbaddr: WRBADDR,
    _reserved16: [u8; 3usize],
    #[doc = "0x3f - Channel ID"]
    pub chid: CHID,
    #[doc = "0x40 - Channel Control A"]
    pub chctrla: CHCTRLA,
    _reserved18: [u8; 3usize],
    #[doc = "0x44 - Channel Control B"]
    pub chctrlb: CHCTRLB,
    _reserved19: [u8; 4usize],
    #[doc = "0x4c - Channel Interrupt Enable Clear"]
    pub chintenclr: CHINTENCLR,
    #[doc = "0x4d - Channel Interrupt Enable Set"]
    pub chintenset: CHINTENSET,
    #[doc = "0x4e - Channel Interrupt Flag Status and Clear"]
    pub chintflag: CHINTFLAG,
    #[doc = "0x4f - Channel Status"]
    pub chstatus: CHSTATUS,
}
#[doc = "Control"]
pub struct CTRL {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Control"]
pub mod ctrl;
#[doc = "CRC Control"]
pub struct CRCCTRL {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "CRC Control"]
pub mod crcctrl;
#[doc = "CRC Data Input"]
pub struct CRCDATAIN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CRC Data Input"]
pub mod crcdatain;
#[doc = "CRC Checksum"]
pub struct CRCCHKSUM {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CRC Checksum"]
pub mod crcchksum;
#[doc = "CRC Status"]
pub struct CRCSTATUS {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "CRC Status"]
pub mod crcstatus;
#[doc = "Debug Control"]
pub struct DBGCTRL {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Debug Control"]
pub mod dbgctrl;
#[doc = "QOS Control"]
pub struct QOSCTRL {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "QOS Control"]
pub mod qosctrl;
#[doc = "Software Trigger Control"]
pub struct SWTRIGCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Software Trigger Control"]
pub mod swtrigctrl;
#[doc = "Priority Control 0"]
pub struct PRICTRL0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Priority Control 0"]
pub mod prictrl0;
#[doc = "Interrupt Pending"]
pub struct INTPEND {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Interrupt Pending"]
pub mod intpend;
#[doc = "Interrupt Status"]
pub struct INTSTATUS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Status"]
pub mod intstatus;
#[doc = "Busy Channels"]
pub struct BUSYCH {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Busy Channels"]
pub mod busych;
#[doc = "Pending Channels"]
pub struct PENDCH {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pending Channels"]
pub mod pendch;
#[doc = "Active Channel and Levels"]
pub struct ACTIVE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Active Channel and Levels"]
pub mod active;
#[doc = "Descriptor Memory Section Base Address"]
pub struct BASEADDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Descriptor Memory Section Base Address"]
pub mod baseaddr;
#[doc = "Write-Back Memory Section Base Address"]
pub struct WRBADDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Write-Back Memory Section Base Address"]
pub mod wrbaddr;
#[doc = "Channel ID"]
pub struct CHID {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Channel ID"]
pub mod chid;
#[doc = "Channel Control A"]
pub struct CHCTRLA {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Channel Control A"]
pub mod chctrla;
#[doc = "Channel Control B"]
pub struct CHCTRLB {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Control B"]
pub mod chctrlb;
#[doc = "Channel Interrupt Enable Clear"]
pub struct CHINTENCLR {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Channel Interrupt Enable Clear"]
pub mod chintenclr;
#[doc = "Channel Interrupt Enable Set"]
pub struct CHINTENSET {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Channel Interrupt Enable Set"]
pub mod chintenset;
#[doc = "Channel Interrupt Flag Status and Clear"]
pub struct CHINTFLAG {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Channel Interrupt Flag Status and Clear"]
pub mod chintflag;
#[doc = "Channel Status"]
pub struct CHSTATUS {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Channel Status"]
pub mod chstatus;
