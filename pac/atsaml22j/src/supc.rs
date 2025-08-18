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
    #[doc = "0x10 - BOD33 Control"]
    pub bod33: BOD33,
    #[doc = "0x14 - BOD12 Control"]
    pub bod12: BOD12,
    #[doc = "0x18 - VREG Control"]
    pub vreg: VREG,
    #[doc = "0x1c - VREF Control"]
    pub vref: VREF,
    #[doc = "0x20 - Battery Backup Power Switch"]
    pub bbps: BBPS,
    #[doc = "0x24 - Backup Output Control"]
    pub bkout: BKOUT,
    #[doc = "0x28 - Backup Input Control"]
    pub bkin: BKIN,
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
#[doc = "BOD33 (rw) register accessor: an alias for `Reg<BOD33_SPEC>`"]
pub type BOD33 = crate::Reg<bod33::BOD33_SPEC>;
#[doc = "BOD33 Control"]
pub mod bod33;
#[doc = "BOD12 (rw) register accessor: an alias for `Reg<BOD12_SPEC>`"]
pub type BOD12 = crate::Reg<bod12::BOD12_SPEC>;
#[doc = "BOD12 Control"]
pub mod bod12;
#[doc = "VREG (rw) register accessor: an alias for `Reg<VREG_SPEC>`"]
pub type VREG = crate::Reg<vreg::VREG_SPEC>;
#[doc = "VREG Control"]
pub mod vreg;
#[doc = "VREF (rw) register accessor: an alias for `Reg<VREF_SPEC>`"]
pub type VREF = crate::Reg<vref::VREF_SPEC>;
#[doc = "VREF Control"]
pub mod vref;
#[doc = "BBPS (rw) register accessor: an alias for `Reg<BBPS_SPEC>`"]
pub type BBPS = crate::Reg<bbps::BBPS_SPEC>;
#[doc = "Battery Backup Power Switch"]
pub mod bbps;
#[doc = "BKOUT (rw) register accessor: an alias for `Reg<BKOUT_SPEC>`"]
pub type BKOUT = crate::Reg<bkout::BKOUT_SPEC>;
#[doc = "Backup Output Control"]
pub mod bkout;
#[doc = "BKIN (r) register accessor: an alias for `Reg<BKIN_SPEC>`"]
pub type BKIN = crate::Reg<bkin::BKIN_SPEC>;
#[doc = "Backup Input Control"]
pub mod bkin;
