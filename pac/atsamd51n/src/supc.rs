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
    #[doc = "0x10 - BOD33 Control"]
    pub bod33: crate::Reg<bod33::BOD33_SPEC>,
    _reserved5: [u8; 0x04],
    #[doc = "0x18 - VREG Control"]
    pub vreg: crate::Reg<vreg::VREG_SPEC>,
    #[doc = "0x1c - VREF Control"]
    pub vref: crate::Reg<vref::VREF_SPEC>,
    #[doc = "0x20 - Battery Backup Power Switch"]
    pub bbps: crate::Reg<bbps::BBPS_SPEC>,
    #[doc = "0x24 - Backup Output Control"]
    pub bkout: crate::Reg<bkout::BKOUT_SPEC>,
    #[doc = "0x28 - Backup Input Control"]
    pub bkin: crate::Reg<bkin::BKIN_SPEC>,
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
#[doc = "BOD33 register accessor: an alias for `Reg<BOD33_SPEC>`"]
pub type BOD33 = crate::Reg<bod33::BOD33_SPEC>;
#[doc = "BOD33 Control"]
pub mod bod33;
#[doc = "VREG register accessor: an alias for `Reg<VREG_SPEC>`"]
pub type VREG = crate::Reg<vreg::VREG_SPEC>;
#[doc = "VREG Control"]
pub mod vreg;
#[doc = "VREF register accessor: an alias for `Reg<VREF_SPEC>`"]
pub type VREF = crate::Reg<vref::VREF_SPEC>;
#[doc = "VREF Control"]
pub mod vref;
#[doc = "BBPS register accessor: an alias for `Reg<BBPS_SPEC>`"]
pub type BBPS = crate::Reg<bbps::BBPS_SPEC>;
#[doc = "Battery Backup Power Switch"]
pub mod bbps;
#[doc = "BKOUT register accessor: an alias for `Reg<BKOUT_SPEC>`"]
pub type BKOUT = crate::Reg<bkout::BKOUT_SPEC>;
#[doc = "Backup Output Control"]
pub mod bkout;
#[doc = "BKIN register accessor: an alias for `Reg<BKIN_SPEC>`"]
pub type BKIN = crate::Reg<bkin::BKIN_SPEC>;
#[doc = "Backup Input Control"]
pub mod bkin;
