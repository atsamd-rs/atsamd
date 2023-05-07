#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Event Control"]
    pub evctrl: EVCTRL,
    _reserved1: [u8; 0x03],
    #[doc = "0x04 - Interrupt Enable Clear"]
    pub intenclr: INTENCLR,
    #[doc = "0x08 - Interrupt Enable Set"]
    pub intenset: INTENSET,
    #[doc = "0x0c - Interrupt Flag Status and Clear"]
    pub intflag: INTFLAG,
    #[doc = "0x10 - Status"]
    pub status: STATUS,
    #[doc = "0x14..0x1c - External Multipurpose Crystal Oscillator Control"]
    pub xoscctrl: [XOSCCTRL; 2],
    #[doc = "0x1c - DFLL48M Control A"]
    pub dfllctrla: DFLLCTRLA,
    _reserved7: [u8; 0x03],
    #[doc = "0x20 - DFLL48M Control B"]
    pub dfllctrlb: DFLLCTRLB,
    _reserved8: [u8; 0x03],
    #[doc = "0x24 - DFLL48M Value"]
    pub dfllval: DFLLVAL,
    #[doc = "0x28 - DFLL48M Multiplier"]
    pub dfllmul: DFLLMUL,
    #[doc = "0x2c - DFLL48M Synchronization"]
    pub dfllsync: DFLLSYNC,
    _reserved11: [u8; 0x03],
    #[doc = "0x30..0x58 - DPLL\\[%s\\]"]
    pub dpll: [DPLL; 2],
}
#[doc = "EVCTRL (rw) register accessor: an alias for `Reg<EVCTRL_SPEC>`"]
pub type EVCTRL = crate::Reg<evctrl::EVCTRL_SPEC>;
#[doc = "Event Control"]
pub mod evctrl;
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
#[doc = "Status"]
pub mod status;
#[doc = "XOSCCTRL (rw) register accessor: an alias for `Reg<XOSCCTRL_SPEC>`"]
pub type XOSCCTRL = crate::Reg<xoscctrl::XOSCCTRL_SPEC>;
#[doc = "External Multipurpose Crystal Oscillator Control"]
pub mod xoscctrl;
#[doc = "DFLLCTRLA (rw) register accessor: an alias for `Reg<DFLLCTRLA_SPEC>`"]
pub type DFLLCTRLA = crate::Reg<dfllctrla::DFLLCTRLA_SPEC>;
#[doc = "DFLL48M Control A"]
pub mod dfllctrla;
#[doc = "DFLLCTRLB (rw) register accessor: an alias for `Reg<DFLLCTRLB_SPEC>`"]
pub type DFLLCTRLB = crate::Reg<dfllctrlb::DFLLCTRLB_SPEC>;
#[doc = "DFLL48M Control B"]
pub mod dfllctrlb;
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
#[doc = "DPLL\\[%s\\]"]
pub use self::dpll::DPLL;
#[doc = r"Cluster"]
#[doc = "DPLL\\[%s\\]"]
pub mod dpll;
