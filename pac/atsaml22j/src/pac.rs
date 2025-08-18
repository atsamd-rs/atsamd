#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Write control"]
    pub wrctrl: WRCTRL,
    #[doc = "0x04 - Event control"]
    pub evctrl: EVCTRL,
    _reserved2: [u8; 0x03],
    #[doc = "0x08 - Interrupt enable clear"]
    pub intenclr: INTENCLR,
    #[doc = "0x09 - Interrupt enable set"]
    pub intenset: INTENSET,
    _reserved4: [u8; 0x06],
    #[doc = "0x10 - Bridge interrupt flag status"]
    pub intflagahb: INTFLAGAHB,
    #[doc = "0x14 - Peripheral interrupt flag status - Bridge A"]
    pub intflaga: INTFLAGA,
    #[doc = "0x18 - Peripheral interrupt flag status - Bridge B"]
    pub intflagb: INTFLAGB,
    #[doc = "0x1c - Peripheral interrupt flag status - Bridge C"]
    pub intflagc: INTFLAGC,
    _reserved8: [u8; 0x14],
    #[doc = "0x34 - Peripheral write protection status - Bridge A"]
    pub statusa: STATUSA,
    #[doc = "0x38 - Peripheral write protection status - Bridge B"]
    pub statusb: STATUSB,
    #[doc = "0x3c - Peripheral write protection status - Bridge C"]
    pub statusc: STATUSC,
}
#[doc = "WRCTRL (rw) register accessor: an alias for `Reg<WRCTRL_SPEC>`"]
pub type WRCTRL = crate::Reg<wrctrl::WRCTRL_SPEC>;
#[doc = "Write control"]
pub mod wrctrl;
#[doc = "EVCTRL (rw) register accessor: an alias for `Reg<EVCTRL_SPEC>`"]
pub type EVCTRL = crate::Reg<evctrl::EVCTRL_SPEC>;
#[doc = "Event control"]
pub mod evctrl;
#[doc = "INTENCLR (rw) register accessor: an alias for `Reg<INTENCLR_SPEC>`"]
pub type INTENCLR = crate::Reg<intenclr::INTENCLR_SPEC>;
#[doc = "Interrupt enable clear"]
pub mod intenclr;
#[doc = "INTENSET (rw) register accessor: an alias for `Reg<INTENSET_SPEC>`"]
pub type INTENSET = crate::Reg<intenset::INTENSET_SPEC>;
#[doc = "Interrupt enable set"]
pub mod intenset;
#[doc = "INTFLAGAHB (rw) register accessor: an alias for `Reg<INTFLAGAHB_SPEC>`"]
pub type INTFLAGAHB = crate::Reg<intflagahb::INTFLAGAHB_SPEC>;
#[doc = "Bridge interrupt flag status"]
pub mod intflagahb;
#[doc = "INTFLAGA (rw) register accessor: an alias for `Reg<INTFLAGA_SPEC>`"]
pub type INTFLAGA = crate::Reg<intflaga::INTFLAGA_SPEC>;
#[doc = "Peripheral interrupt flag status - Bridge A"]
pub mod intflaga;
#[doc = "INTFLAGB (rw) register accessor: an alias for `Reg<INTFLAGB_SPEC>`"]
pub type INTFLAGB = crate::Reg<intflagb::INTFLAGB_SPEC>;
#[doc = "Peripheral interrupt flag status - Bridge B"]
pub mod intflagb;
#[doc = "INTFLAGC (rw) register accessor: an alias for `Reg<INTFLAGC_SPEC>`"]
pub type INTFLAGC = crate::Reg<intflagc::INTFLAGC_SPEC>;
#[doc = "Peripheral interrupt flag status - Bridge C"]
pub mod intflagc;
#[doc = "STATUSA (r) register accessor: an alias for `Reg<STATUSA_SPEC>`"]
pub type STATUSA = crate::Reg<statusa::STATUSA_SPEC>;
#[doc = "Peripheral write protection status - Bridge A"]
pub mod statusa;
#[doc = "STATUSB (r) register accessor: an alias for `Reg<STATUSB_SPEC>`"]
pub type STATUSB = crate::Reg<statusb::STATUSB_SPEC>;
#[doc = "Peripheral write protection status - Bridge B"]
pub mod statusb;
#[doc = "STATUSC (r) register accessor: an alias for `Reg<STATUSC_SPEC>`"]
pub type STATUSC = crate::Reg<statusc::STATUSC_SPEC>;
#[doc = "Peripheral write protection status - Bridge C"]
pub mod statusc;
