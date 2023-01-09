#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control A"]
    pub ctrla: CTRLA,
    #[doc = "0x01 - Sleep Configuration"]
    pub sleepcfg: SLEEPCFG,
    #[doc = "0x02 - Performance Level Configuration"]
    pub plcfg: PLCFG,
    _reserved3: [u8; 0x01],
    #[doc = "0x04 - Interrupt Enable Clear"]
    pub intenclr: INTENCLR,
    #[doc = "0x05 - Interrupt Enable Set"]
    pub intenset: INTENSET,
    #[doc = "0x06 - Interrupt Flag Status and Clear"]
    pub intflag: INTFLAG,
    _reserved6: [u8; 0x01],
    #[doc = "0x08 - Standby Configuration"]
    pub stdbycfg: STDBYCFG,
}
#[doc = "CTRLA (rw) register accessor: an alias for `Reg<CTRLA_SPEC>`"]
pub type CTRLA = crate::Reg<ctrla::CTRLA_SPEC>;
#[doc = "Control A"]
pub mod ctrla;
#[doc = "SLEEPCFG (rw) register accessor: an alias for `Reg<SLEEPCFG_SPEC>`"]
pub type SLEEPCFG = crate::Reg<sleepcfg::SLEEPCFG_SPEC>;
#[doc = "Sleep Configuration"]
pub mod sleepcfg;
#[doc = "PLCFG (rw) register accessor: an alias for `Reg<PLCFG_SPEC>`"]
pub type PLCFG = crate::Reg<plcfg::PLCFG_SPEC>;
#[doc = "Performance Level Configuration"]
pub mod plcfg;
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
#[doc = "STDBYCFG (rw) register accessor: an alias for `Reg<STDBYCFG_SPEC>`"]
pub type STDBYCFG = crate::Reg<stdbycfg::STDBYCFG_SPEC>;
#[doc = "Standby Configuration"]
pub mod stdbycfg;
