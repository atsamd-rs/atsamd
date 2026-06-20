#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Write control"]
    pub wrctrl: crate::Reg<wrctrl::WRCTRL_SPEC>,
    #[doc = "0x04 - Event control"]
    pub evctrl: crate::Reg<evctrl::EVCTRL_SPEC>,
    _reserved2: [u8; 0x03],
    #[doc = "0x08 - Interrupt enable clear"]
    pub intenclr: crate::Reg<intenclr::INTENCLR_SPEC>,
    #[doc = "0x09 - Interrupt enable set"]
    pub intenset: crate::Reg<intenset::INTENSET_SPEC>,
    _reserved4: [u8; 0x06],
    #[doc = "0x10 - Bridge interrupt flag status"]
    pub intflagahb: crate::Reg<intflagahb::INTFLAGAHB_SPEC>,
    #[doc = "0x14 - Peripheral interrupt flag status - Bridge A"]
    pub intflaga: crate::Reg<intflaga::INTFLAGA_SPEC>,
    #[doc = "0x18 - Peripheral interrupt flag status - Bridge B"]
    pub intflagb: crate::Reg<intflagb::INTFLAGB_SPEC>,
    #[doc = "0x1c - Peripheral interrupt flag status - Bridge C"]
    pub intflagc: crate::Reg<intflagc::INTFLAGC_SPEC>,
    #[doc = "0x20 - Peripheral interrupt flag status - Bridge D"]
    pub intflagd: crate::Reg<intflagd::INTFLAGD_SPEC>,
    _reserved9: [u8; 0x10],
    #[doc = "0x34 - Peripheral write protection status - Bridge A"]
    pub statusa: crate::Reg<statusa::STATUSA_SPEC>,
    #[doc = "0x38 - Peripheral write protection status - Bridge B"]
    pub statusb: crate::Reg<statusb::STATUSB_SPEC>,
    #[doc = "0x3c - Peripheral write protection status - Bridge C"]
    pub statusc: crate::Reg<statusc::STATUSC_SPEC>,
    #[doc = "0x40 - Peripheral write protection status - Bridge D"]
    pub statusd: crate::Reg<statusd::STATUSD_SPEC>,
}
#[doc = "WRCTRL register accessor: an alias for `Reg<WRCTRL_SPEC>`"]
pub type WRCTRL = crate::Reg<wrctrl::WRCTRL_SPEC>;
#[doc = "Write control"]
pub mod wrctrl;
#[doc = "EVCTRL register accessor: an alias for `Reg<EVCTRL_SPEC>`"]
pub type EVCTRL = crate::Reg<evctrl::EVCTRL_SPEC>;
#[doc = "Event control"]
pub mod evctrl;
#[doc = "INTENCLR register accessor: an alias for `Reg<INTENCLR_SPEC>`"]
pub type INTENCLR = crate::Reg<intenclr::INTENCLR_SPEC>;
#[doc = "Interrupt enable clear"]
pub mod intenclr;
#[doc = "INTENSET register accessor: an alias for `Reg<INTENSET_SPEC>`"]
pub type INTENSET = crate::Reg<intenset::INTENSET_SPEC>;
#[doc = "Interrupt enable set"]
pub mod intenset;
#[doc = "INTFLAGAHB register accessor: an alias for `Reg<INTFLAGAHB_SPEC>`"]
pub type INTFLAGAHB = crate::Reg<intflagahb::INTFLAGAHB_SPEC>;
#[doc = "Bridge interrupt flag status"]
pub mod intflagahb;
#[doc = "INTFLAGA register accessor: an alias for `Reg<INTFLAGA_SPEC>`"]
pub type INTFLAGA = crate::Reg<intflaga::INTFLAGA_SPEC>;
#[doc = "Peripheral interrupt flag status - Bridge A"]
pub mod intflaga;
#[doc = "INTFLAGB register accessor: an alias for `Reg<INTFLAGB_SPEC>`"]
pub type INTFLAGB = crate::Reg<intflagb::INTFLAGB_SPEC>;
#[doc = "Peripheral interrupt flag status - Bridge B"]
pub mod intflagb;
#[doc = "INTFLAGC register accessor: an alias for `Reg<INTFLAGC_SPEC>`"]
pub type INTFLAGC = crate::Reg<intflagc::INTFLAGC_SPEC>;
#[doc = "Peripheral interrupt flag status - Bridge C"]
pub mod intflagc;
#[doc = "INTFLAGD register accessor: an alias for `Reg<INTFLAGD_SPEC>`"]
pub type INTFLAGD = crate::Reg<intflagd::INTFLAGD_SPEC>;
#[doc = "Peripheral interrupt flag status - Bridge D"]
pub mod intflagd;
#[doc = "STATUSA register accessor: an alias for `Reg<STATUSA_SPEC>`"]
pub type STATUSA = crate::Reg<statusa::STATUSA_SPEC>;
#[doc = "Peripheral write protection status - Bridge A"]
pub mod statusa;
#[doc = "STATUSB register accessor: an alias for `Reg<STATUSB_SPEC>`"]
pub type STATUSB = crate::Reg<statusb::STATUSB_SPEC>;
#[doc = "Peripheral write protection status - Bridge B"]
pub mod statusb;
#[doc = "STATUSC register accessor: an alias for `Reg<STATUSC_SPEC>`"]
pub type STATUSC = crate::Reg<statusc::STATUSC_SPEC>;
#[doc = "Peripheral write protection status - Bridge C"]
pub mod statusc;
#[doc = "STATUSD register accessor: an alias for `Reg<STATUSD_SPEC>`"]
pub type STATUSD = crate::Reg<statusd::STATUSD_SPEC>;
#[doc = "Peripheral write protection status - Bridge D"]
pub mod statusd;
