#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Interrupt Enable Clear"]
    pub intenclr: crate::Reg<intenclr::INTENCLR_SPEC>,
    #[doc = "0x04 - Interrupt Enable Set"]
    pub intenset: crate::Reg<intenset::INTENSET_SPEC>,
    #[doc = "0x08 - Interrupt Flag Status and Clear"]
    pub intflag: crate::Reg<intflag::INTFLAG_SPEC>,
    #[doc = "0x0c - Power and Clocks Status"]
    pub status: crate::Reg<status::STATUS_SPEC>,
    #[doc = "0x10 - External Multipurpose Crystal Oscillator (XOSC) Control"]
    pub xoscctrl: crate::Reg<xoscctrl::XOSCCTRL_SPEC>,
    #[doc = "0x12 - Clock Failure Detector Prescaler"]
    pub cfdpresc: crate::Reg<cfdpresc::CFDPRESC_SPEC>,
    #[doc = "0x13 - Event Control"]
    pub evctrl: crate::Reg<evctrl::EVCTRL_SPEC>,
    #[doc = "0x14 - 48MHz Internal Oscillator (OSC48M) Control"]
    pub osc48mctrl: crate::Reg<osc48mctrl::OSC48MCTRL_SPEC>,
    #[doc = "0x15 - OSC48M Divider"]
    pub osc48mdiv: crate::Reg<osc48mdiv::OSC48MDIV_SPEC>,
    #[doc = "0x16 - OSC48M Startup Time"]
    pub osc48mstup: crate::Reg<osc48mstup::OSC48MSTUP_SPEC>,
    _reserved10: [u8; 0x01],
    #[doc = "0x18 - OSC48M Synchronization Busy"]
    pub osc48msyncbusy: crate::Reg<osc48msyncbusy::OSC48MSYNCBUSY_SPEC>,
    #[doc = "0x1c - DPLL Control"]
    pub dpllctrla: crate::Reg<dpllctrla::DPLLCTRLA_SPEC>,
    _reserved12: [u8; 0x03],
    #[doc = "0x20 - DPLL Ratio Control"]
    pub dpllratio: crate::Reg<dpllratio::DPLLRATIO_SPEC>,
    #[doc = "0x24 - Digital Core Configuration"]
    pub dpllctrlb: crate::Reg<dpllctrlb::DPLLCTRLB_SPEC>,
    #[doc = "0x28 - DPLL Prescaler"]
    pub dpllpresc: crate::Reg<dpllpresc::DPLLPRESC_SPEC>,
    _reserved15: [u8; 0x03],
    #[doc = "0x2c - DPLL Synchronization Busy"]
    pub dpllsyncbusy: crate::Reg<dpllsyncbusy::DPLLSYNCBUSY_SPEC>,
    _reserved16: [u8; 0x03],
    #[doc = "0x30 - DPLL Status"]
    pub dpllstatus: crate::Reg<dpllstatus::DPLLSTATUS_SPEC>,
    _reserved17: [u8; 0x07],
    #[doc = "0x38 - 48MHz Oscillator Calibration"]
    pub cal48m: crate::Reg<cal48m::CAL48M_SPEC>,
}
#[doc = "INTENCLR register accessor: an alias for `Reg<INTENCLR_SPEC>`"]
pub type INTENCLR = crate::Reg<intenclr::INTENCLR_SPEC>;
#[doc = "Interrupt Enable Clear"]
pub mod intenclr;
#[doc = "INTENSET register accessor: an alias for `Reg<INTENSET_SPEC>`"]
pub type INTENSET = crate::Reg<intenset::INTENSET_SPEC>;
#[doc = "Interrupt Enable Set"]
pub mod intenset;
#[doc = "INTFLAG register accessor: an alias for `Reg<INTFLAG_SPEC>`"]
pub type INTFLAG = crate::Reg<intflag::INTFLAG_SPEC>;
#[doc = "Interrupt Flag Status and Clear"]
pub mod intflag;
#[doc = "STATUS register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "Power and Clocks Status"]
pub mod status;
#[doc = "XOSCCTRL register accessor: an alias for `Reg<XOSCCTRL_SPEC>`"]
pub type XOSCCTRL = crate::Reg<xoscctrl::XOSCCTRL_SPEC>;
#[doc = "External Multipurpose Crystal Oscillator (XOSC) Control"]
pub mod xoscctrl;
#[doc = "CFDPRESC register accessor: an alias for `Reg<CFDPRESC_SPEC>`"]
pub type CFDPRESC = crate::Reg<cfdpresc::CFDPRESC_SPEC>;
#[doc = "Clock Failure Detector Prescaler"]
pub mod cfdpresc;
#[doc = "EVCTRL register accessor: an alias for `Reg<EVCTRL_SPEC>`"]
pub type EVCTRL = crate::Reg<evctrl::EVCTRL_SPEC>;
#[doc = "Event Control"]
pub mod evctrl;
#[doc = "OSC48MCTRL register accessor: an alias for `Reg<OSC48MCTRL_SPEC>`"]
pub type OSC48MCTRL = crate::Reg<osc48mctrl::OSC48MCTRL_SPEC>;
#[doc = "48MHz Internal Oscillator (OSC48M) Control"]
pub mod osc48mctrl;
#[doc = "OSC48MDIV register accessor: an alias for `Reg<OSC48MDIV_SPEC>`"]
pub type OSC48MDIV = crate::Reg<osc48mdiv::OSC48MDIV_SPEC>;
#[doc = "OSC48M Divider"]
pub mod osc48mdiv;
#[doc = "OSC48MSTUP register accessor: an alias for `Reg<OSC48MSTUP_SPEC>`"]
pub type OSC48MSTUP = crate::Reg<osc48mstup::OSC48MSTUP_SPEC>;
#[doc = "OSC48M Startup Time"]
pub mod osc48mstup;
#[doc = "OSC48MSYNCBUSY register accessor: an alias for `Reg<OSC48MSYNCBUSY_SPEC>`"]
pub type OSC48MSYNCBUSY = crate::Reg<osc48msyncbusy::OSC48MSYNCBUSY_SPEC>;
#[doc = "OSC48M Synchronization Busy"]
pub mod osc48msyncbusy;
#[doc = "DPLLCTRLA register accessor: an alias for `Reg<DPLLCTRLA_SPEC>`"]
pub type DPLLCTRLA = crate::Reg<dpllctrla::DPLLCTRLA_SPEC>;
#[doc = "DPLL Control"]
pub mod dpllctrla;
#[doc = "DPLLRATIO register accessor: an alias for `Reg<DPLLRATIO_SPEC>`"]
pub type DPLLRATIO = crate::Reg<dpllratio::DPLLRATIO_SPEC>;
#[doc = "DPLL Ratio Control"]
pub mod dpllratio;
#[doc = "DPLLCTRLB register accessor: an alias for `Reg<DPLLCTRLB_SPEC>`"]
pub type DPLLCTRLB = crate::Reg<dpllctrlb::DPLLCTRLB_SPEC>;
#[doc = "Digital Core Configuration"]
pub mod dpllctrlb;
#[doc = "DPLLPRESC register accessor: an alias for `Reg<DPLLPRESC_SPEC>`"]
pub type DPLLPRESC = crate::Reg<dpllpresc::DPLLPRESC_SPEC>;
#[doc = "DPLL Prescaler"]
pub mod dpllpresc;
#[doc = "DPLLSYNCBUSY register accessor: an alias for `Reg<DPLLSYNCBUSY_SPEC>`"]
pub type DPLLSYNCBUSY = crate::Reg<dpllsyncbusy::DPLLSYNCBUSY_SPEC>;
#[doc = "DPLL Synchronization Busy"]
pub mod dpllsyncbusy;
#[doc = "DPLLSTATUS register accessor: an alias for `Reg<DPLLSTATUS_SPEC>`"]
pub type DPLLSTATUS = crate::Reg<dpllstatus::DPLLSTATUS_SPEC>;
#[doc = "DPLL Status"]
pub mod dpllstatus;
#[doc = "CAL48M register accessor: an alias for `Reg<CAL48M_SPEC>`"]
pub type CAL48M = crate::Reg<cal48m::CAL48M_SPEC>;
#[doc = "48MHz Oscillator Calibration"]
pub mod cal48m;
