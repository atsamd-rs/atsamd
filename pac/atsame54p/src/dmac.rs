#[doc = r"Register block"]
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
    _reserved6: [u8; 0x02],
    #[doc = "0x10 - Software Trigger Control"]
    pub swtrigctrl: SWTRIGCTRL,
    #[doc = "0x14 - Priority Control 0"]
    pub prictrl0: PRICTRL0,
    _reserved8: [u8; 0x08],
    #[doc = "0x20 - Interrupt Pending"]
    pub intpend: INTPEND,
    _reserved9: [u8; 0x02],
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
    _reserved15: [u8; 0x04],
    #[doc = "0x40..0x240 - CHANNEL\\[%s\\]"]
    pub channel: [CHANNEL; 32],
}
#[doc = "CTRL (rw) register accessor: Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`]
module"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Control"]
pub mod ctrl;
#[doc = "CRCCTRL (rw) register accessor: CRC Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`crcctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`crcctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crcctrl`]
module"]
pub type CRCCTRL = crate::Reg<crcctrl::CRCCTRL_SPEC>;
#[doc = "CRC Control"]
pub mod crcctrl;
#[doc = "CRCDATAIN (rw) register accessor: CRC Data Input\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`crcdatain::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`crcdatain::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crcdatain`]
module"]
pub type CRCDATAIN = crate::Reg<crcdatain::CRCDATAIN_SPEC>;
#[doc = "CRC Data Input"]
pub mod crcdatain;
#[doc = "CRCCHKSUM (rw) register accessor: CRC Checksum\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`crcchksum::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`crcchksum::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crcchksum`]
module"]
pub type CRCCHKSUM = crate::Reg<crcchksum::CRCCHKSUM_SPEC>;
#[doc = "CRC Checksum"]
pub mod crcchksum;
#[doc = "CRCSTATUS (rw) register accessor: CRC Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`crcstatus::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`crcstatus::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crcstatus`]
module"]
pub type CRCSTATUS = crate::Reg<crcstatus::CRCSTATUS_SPEC>;
#[doc = "CRC Status"]
pub mod crcstatus;
#[doc = "DBGCTRL (rw) register accessor: Debug Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dbgctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dbgctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbgctrl`]
module"]
pub type DBGCTRL = crate::Reg<dbgctrl::DBGCTRL_SPEC>;
#[doc = "Debug Control"]
pub mod dbgctrl;
#[doc = "SWTRIGCTRL (rw) register accessor: Software Trigger Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swtrigctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swtrigctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swtrigctrl`]
module"]
pub type SWTRIGCTRL = crate::Reg<swtrigctrl::SWTRIGCTRL_SPEC>;
#[doc = "Software Trigger Control"]
pub mod swtrigctrl;
#[doc = "PRICTRL0 (rw) register accessor: Priority Control 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`prictrl0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`prictrl0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prictrl0`]
module"]
pub type PRICTRL0 = crate::Reg<prictrl0::PRICTRL0_SPEC>;
#[doc = "Priority Control 0"]
pub mod prictrl0;
#[doc = "INTPEND (rw) register accessor: Interrupt Pending\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intpend::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intpend::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpend`]
module"]
pub type INTPEND = crate::Reg<intpend::INTPEND_SPEC>;
#[doc = "Interrupt Pending"]
pub mod intpend;
#[doc = "INTSTATUS (r) register accessor: Interrupt Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intstatus::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intstatus`]
module"]
pub type INTSTATUS = crate::Reg<intstatus::INTSTATUS_SPEC>;
#[doc = "Interrupt Status"]
pub mod intstatus;
#[doc = "BUSYCH (r) register accessor: Busy Channels\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`busych::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@busych`]
module"]
pub type BUSYCH = crate::Reg<busych::BUSYCH_SPEC>;
#[doc = "Busy Channels"]
pub mod busych;
#[doc = "PENDCH (r) register accessor: Pending Channels\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pendch::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pendch`]
module"]
pub type PENDCH = crate::Reg<pendch::PENDCH_SPEC>;
#[doc = "Pending Channels"]
pub mod pendch;
#[doc = "ACTIVE (r) register accessor: Active Channel and Levels\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`active::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@active`]
module"]
pub type ACTIVE = crate::Reg<active::ACTIVE_SPEC>;
#[doc = "Active Channel and Levels"]
pub mod active;
#[doc = "BASEADDR (rw) register accessor: Descriptor Memory Section Base Address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`baseaddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`baseaddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@baseaddr`]
module"]
pub type BASEADDR = crate::Reg<baseaddr::BASEADDR_SPEC>;
#[doc = "Descriptor Memory Section Base Address"]
pub mod baseaddr;
#[doc = "WRBADDR (rw) register accessor: Write-Back Memory Section Base Address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wrbaddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wrbaddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wrbaddr`]
module"]
pub type WRBADDR = crate::Reg<wrbaddr::WRBADDR_SPEC>;
#[doc = "Write-Back Memory Section Base Address"]
pub mod wrbaddr;
#[doc = "CHANNEL\\[%s\\]"]
pub use self::channel::CHANNEL;
#[doc = r"Cluster"]
#[doc = "CHANNEL\\[%s\\]"]
pub mod channel;
