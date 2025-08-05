#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Interrupt Enable Clear"]
    pub intenclr: INTENCLR,
    #[doc = "0x04 - Interrupt Enable Set"]
    pub intenset: INTENSET,
    #[doc = "0x08 - Interrupt Flag Status and Clear"]
    pub intflag: INTFLAG,
    #[doc = "0x0c - Power and Clocks Status"]
    pub status: STATUS,
    #[doc = "0x10 - External Multipurpose Crystal Oscillator (XOSC) Control"]
    pub xoscctrl: XOSCCTRL,
    #[doc = "0x12 - Cloc Failure Detector Prescaler"]
    pub cfdpresc: CFDPRESC,
    #[doc = "0x13 - Event Control"]
    pub evctrl: EVCTRL,
    #[doc = "0x14 - 16MHz Internal Oscillator (OSC16M) Control"]
    pub osc16mctrl: OSC16MCTRL,
    _reserved8: [u8; 0x03],
    #[doc = "0x18 - DFLL48M Control"]
    pub dfllctrl: DFLLCTRL,
    _reserved9: [u8; 0x02],
    #[doc = "0x1c - DFLL48M Value"]
    pub dfllval: DFLLVAL,
    #[doc = "0x20 - DFLL48M Multiplier"]
    pub dfllmul: DFLLMUL,
    #[doc = "0x24 - DFLL48M Synchronization"]
    pub dfllsync: DFLLSYNC,
    _reserved12: [u8; 0x03],
    #[doc = "0x28 - DPLL Control"]
    pub dpllctrla: DPLLCTRLA,
    _reserved13: [u8; 0x03],
    #[doc = "0x2c - DPLL Ratio Control"]
    pub dpllratio: DPLLRATIO,
    #[doc = "0x30 - Digital Core Configuration"]
    pub dpllctrlb: DPLLCTRLB,
    #[doc = "0x34 - DPLL Prescaler"]
    pub dpllpresc: DPLLPRESC,
    _reserved16: [u8; 0x03],
    #[doc = "0x38 - DPLL Synchronization Busy"]
    pub dpllsyncbusy: DPLLSYNCBUSY,
    _reserved17: [u8; 0x03],
    #[doc = "0x3c - DPLL Status"]
    pub dpllstatus: DPLLSTATUS,
}
#[doc = "INTENCLR (rw) register accessor: an alias for `Reg<INTENCLR_SPEC>`"]
pub type INTENCLR = crate::Reg<intenclr::INTENCLR_SPEC>;
#[doc = "Interrupt Enable Clear"]
pub mod intenclr;
#[doc = "INTENSET (rw) register accessor: an alias for `Reg<INTENSET_SPEC>`"]
pub type INTENSET = crate::Reg<intenset::INTENSET_SPEC>;
#[doc = "Interrupt Enable Set"]
pub mod intenset;
#[doc = "INTFLAG (rw) register accessor: an alias for `Reg<INTFLAG_SPEC>`"]
pub type INTFLAG = crate::Reg<intflag::INTFLAG_SPEC>;
#[doc = "Interrupt Flag Status and Clear"]
pub mod intflag;
#[doc = "STATUS (r) register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "Power and Clocks Status"]
pub mod status;
#[doc = "XOSCCTRL (rw) register accessor: an alias for `Reg<XOSCCTRL_SPEC>`"]
pub type XOSCCTRL = crate::Reg<xoscctrl::XOSCCTRL_SPEC>;
#[doc = "External Multipurpose Crystal Oscillator (XOSC) Control"]
pub mod xoscctrl;
#[doc = "CFDPRESC (rw) register accessor: an alias for `Reg<CFDPRESC_SPEC>`"]
pub type CFDPRESC = crate::Reg<cfdpresc::CFDPRESC_SPEC>;
#[doc = "Cloc Failure Detector Prescaler"]
pub mod cfdpresc;
#[doc = "EVCTRL (rw) register accessor: an alias for `Reg<EVCTRL_SPEC>`"]
pub type EVCTRL = crate::Reg<evctrl::EVCTRL_SPEC>;
#[doc = "Event Control"]
pub mod evctrl;
#[doc = "OSC16MCTRL (rw) register accessor: an alias for `Reg<OSC16MCTRL_SPEC>`"]
pub type OSC16MCTRL = crate::Reg<osc16mctrl::OSC16MCTRL_SPEC>;
#[doc = "16MHz Internal Oscillator (OSC16M) Control"]
pub mod osc16mctrl;
#[doc = "DFLLCTRL (rw) register accessor: an alias for `Reg<DFLLCTRL_SPEC>`"]
pub type DFLLCTRL = crate::Reg<dfllctrl::DFLLCTRL_SPEC>;
#[doc = "DFLL48M Control"]
pub mod dfllctrl;
#[doc = "DFLLVAL (rw) register accessor: an alias for `Reg<DFLLVAL_SPEC>`"]
pub type DFLLVAL = crate::Reg<dfllval::DFLLVAL_SPEC>;
#[doc = "DFLL48M Value"]
pub mod dfllval;
#[doc = "DFLLMUL (rw) register accessor: an alias for `Reg<DFLLMUL_SPEC>`"]
pub type DFLLMUL = crate::Reg<dfllmul::DFLLMUL_SPEC>;
#[doc = "DFLL48M Multiplier"]
pub mod dfllmul;
#[doc = "DFLLSYNC (rw) register accessor: an alias for `Reg<DFLLSYNC_SPEC>`"]
pub type DFLLSYNC = crate::Reg<dfllsync::DFLLSYNC_SPEC>;
#[doc = "DFLL48M Synchronization"]
pub mod dfllsync;
#[doc = "DPLLCTRLA (rw) register accessor: an alias for `Reg<DPLLCTRLA_SPEC>`"]
pub type DPLLCTRLA = crate::Reg<dpllctrla::DPLLCTRLA_SPEC>;
#[doc = "DPLL Control"]
pub mod dpllctrla;
#[doc = "DPLLRATIO (rw) register accessor: an alias for `Reg<DPLLRATIO_SPEC>`"]
pub type DPLLRATIO = crate::Reg<dpllratio::DPLLRATIO_SPEC>;
#[doc = "DPLL Ratio Control"]
pub mod dpllratio;
#[doc = "DPLLCTRLB (rw) register accessor: an alias for `Reg<DPLLCTRLB_SPEC>`"]
pub type DPLLCTRLB = crate::Reg<dpllctrlb::DPLLCTRLB_SPEC>;
#[doc = "Digital Core Configuration"]
pub mod dpllctrlb;
#[doc = "DPLLPRESC (rw) register accessor: an alias for `Reg<DPLLPRESC_SPEC>`"]
pub type DPLLPRESC = crate::Reg<dpllpresc::DPLLPRESC_SPEC>;
#[doc = "DPLL Prescaler"]
pub mod dpllpresc;
#[doc = "DPLLSYNCBUSY (r) register accessor: an alias for `Reg<DPLLSYNCBUSY_SPEC>`"]
pub type DPLLSYNCBUSY = crate::Reg<dpllsyncbusy::DPLLSYNCBUSY_SPEC>;
#[doc = "DPLL Synchronization Busy"]
pub mod dpllsyncbusy;
#[doc = "DPLLSTATUS (r) register accessor: an alias for `Reg<DPLLSTATUS_SPEC>`"]
pub type DPLLSTATUS = crate::Reg<dpllstatus::DPLLSTATUS_SPEC>;
#[doc = "DPLL Status"]
pub mod dpllstatus;
