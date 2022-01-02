#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control A"]
    pub ctrla: crate::Reg<ctrla::CTRLA_SPEC>,
    #[doc = "0x01 - Sleep Configuration"]
    pub sleepcfg: crate::Reg<sleepcfg::SLEEPCFG_SPEC>,
    _reserved2: [u8; 0x02],
    #[doc = "0x04 - Interrupt Enable Clear"]
    pub intenclr: crate::Reg<intenclr::INTENCLR_SPEC>,
    #[doc = "0x05 - Interrupt Enable Set"]
    pub intenset: crate::Reg<intenset::INTENSET_SPEC>,
    #[doc = "0x06 - Interrupt Flag Status and Clear"]
    pub intflag: crate::Reg<intflag::INTFLAG_SPEC>,
    _reserved5: [u8; 0x01],
    #[doc = "0x08 - Standby Configuration"]
    pub stdbycfg: crate::Reg<stdbycfg::STDBYCFG_SPEC>,
    #[doc = "0x09 - Hibernate Configuration"]
    pub hibcfg: crate::Reg<hibcfg::HIBCFG_SPEC>,
    #[doc = "0x0a - Backup Configuration"]
    pub bkupcfg: crate::Reg<bkupcfg::BKUPCFG_SPEC>,
    _reserved8: [u8; 0x07],
    #[doc = "0x12 - Power Switch Acknowledge Delay"]
    pub pwsakdly: crate::Reg<pwsakdly::PWSAKDLY_SPEC>,
}
#[doc = "CTRLA register accessor: an alias for `Reg<CTRLA_SPEC>`"]
pub type CTRLA = crate::Reg<ctrla::CTRLA_SPEC>;
#[doc = "Control A"]
pub mod ctrla;
#[doc = "SLEEPCFG register accessor: an alias for `Reg<SLEEPCFG_SPEC>`"]
pub type SLEEPCFG = crate::Reg<sleepcfg::SLEEPCFG_SPEC>;
#[doc = "Sleep Configuration"]
pub mod sleepcfg;
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
#[doc = "STDBYCFG register accessor: an alias for `Reg<STDBYCFG_SPEC>`"]
pub type STDBYCFG = crate::Reg<stdbycfg::STDBYCFG_SPEC>;
#[doc = "Standby Configuration"]
pub mod stdbycfg;
#[doc = "HIBCFG register accessor: an alias for `Reg<HIBCFG_SPEC>`"]
pub type HIBCFG = crate::Reg<hibcfg::HIBCFG_SPEC>;
#[doc = "Hibernate Configuration"]
pub mod hibcfg;
#[doc = "BKUPCFG register accessor: an alias for `Reg<BKUPCFG_SPEC>`"]
pub type BKUPCFG = crate::Reg<bkupcfg::BKUPCFG_SPEC>;
#[doc = "Backup Configuration"]
pub mod bkupcfg;
#[doc = "PWSAKDLY register accessor: an alias for `Reg<PWSAKDLY_SPEC>`"]
pub type PWSAKDLY = crate::Reg<pwsakdly::PWSAKDLY_SPEC>;
#[doc = "Power Switch Acknowledge Delay"]
pub mod pwsakdly;
