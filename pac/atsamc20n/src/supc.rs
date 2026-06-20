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
    #[doc = "0x10 - BODVDD Control"]
    pub bodvdd: crate::Reg<bodvdd::BODVDD_SPEC>,
    _reserved5: [u8; 0x04],
    #[doc = "0x18 - VREG Control"]
    pub vreg: crate::Reg<vreg::VREG_SPEC>,
    #[doc = "0x1c - VREF Control"]
    pub vref: crate::Reg<vref::VREF_SPEC>,
    #[doc = "0x20 - VREG33 Control"]
    pub vreg33: crate::Reg<vreg33::VREG33_SPEC>,
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
#[doc = "BODVDD register accessor: an alias for `Reg<BODVDD_SPEC>`"]
pub type BODVDD = crate::Reg<bodvdd::BODVDD_SPEC>;
#[doc = "BODVDD Control"]
pub mod bodvdd;
#[doc = "VREG register accessor: an alias for `Reg<VREG_SPEC>`"]
pub type VREG = crate::Reg<vreg::VREG_SPEC>;
#[doc = "VREG Control"]
pub mod vreg;
#[doc = "VREF register accessor: an alias for `Reg<VREF_SPEC>`"]
pub type VREF = crate::Reg<vref::VREF_SPEC>;
#[doc = "VREF Control"]
pub mod vref;
#[doc = "VREG33 register accessor: an alias for `Reg<VREG33_SPEC>`"]
pub type VREG33 = crate::Reg<vreg33::VREG33_SPEC>;
#[doc = "VREG33 Control"]
pub mod vreg33;
