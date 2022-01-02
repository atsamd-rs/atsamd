#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Event Control"]
    pub evctrl: crate::Reg<evctrl::EVCTRL_SPEC>,
    _reserved1: [u8; 0x03],
    #[doc = "0x04 - Interrupt Enable Clear"]
    pub intenclr: crate::Reg<intenclr::INTENCLR_SPEC>,
    #[doc = "0x08 - Interrupt Enable Set"]
    pub intenset: crate::Reg<intenset::INTENSET_SPEC>,
    #[doc = "0x0c - Interrupt Flag Status and Clear"]
    pub intflag: crate::Reg<intflag::INTFLAG_SPEC>,
    #[doc = "0x10 - Status"]
    pub status: crate::Reg<status::STATUS_SPEC>,
    #[doc = "0x14..0x1c - External Multipurpose Crystal Oscillator Control"]
    pub xoscctrl: [crate::Reg<xoscctrl::XOSCCTRL_SPEC>; 2],
    #[doc = "0x1c - DFLL48M Control A"]
    pub dfllctrla: crate::Reg<dfllctrla::DFLLCTRLA_SPEC>,
    _reserved7: [u8; 0x03],
    #[doc = "0x20 - DFLL48M Control B"]
    pub dfllctrlb: crate::Reg<dfllctrlb::DFLLCTRLB_SPEC>,
    _reserved8: [u8; 0x03],
    #[doc = "0x24 - DFLL48M Value"]
    pub dfllval: crate::Reg<dfllval::DFLLVAL_SPEC>,
    #[doc = "0x28 - DFLL48M Multiplier"]
    pub dfllmul: crate::Reg<dfllmul::DFLLMUL_SPEC>,
    #[doc = "0x2c - DFLL48M Synchronization"]
    pub dfllsync: crate::Reg<dfllsync::DFLLSYNC_SPEC>,
    _reserved11: [u8; 0x03],
    #[doc = "0x30..0x58 - DPLL\\[%s\\]"]
    pub dpll: [DPLL; 2],
}
#[doc = r"Register block"]
#[repr(C)]
pub struct DPLL {
    #[doc = "0x00 - DPLL Control A"]
    pub dpllctrla: crate::Reg<self::dpll::dpllctrla::DPLLCTRLA_SPEC>,
    _reserved1: [u8; 0x03],
    #[doc = "0x04 - DPLL Ratio Control"]
    pub dpllratio: crate::Reg<self::dpll::dpllratio::DPLLRATIO_SPEC>,
    #[doc = "0x08 - DPLL Control B"]
    pub dpllctrlb: crate::Reg<self::dpll::dpllctrlb::DPLLCTRLB_SPEC>,
    #[doc = "0x0c - DPLL Synchronization Busy"]
    pub dpllsyncbusy: crate::Reg<self::dpll::dpllsyncbusy::DPLLSYNCBUSY_SPEC>,
    #[doc = "0x10 - DPLL Status"]
    pub dpllstatus: crate::Reg<self::dpll::dpllstatus::DPLLSTATUS_SPEC>,
}
#[doc = r"Register block"]
#[doc = "DPLL\\[%s\\]"]
pub mod dpll;
#[doc = "EVCTRL register accessor: an alias for `Reg<EVCTRL_SPEC>`"]
pub type EVCTRL = crate::Reg<evctrl::EVCTRL_SPEC>;
#[doc = "Event Control"]
pub mod evctrl;
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
#[doc = "Status"]
pub mod status;
#[doc = "XOSCCTRL register accessor: an alias for `Reg<XOSCCTRL_SPEC>`"]
pub type XOSCCTRL = crate::Reg<xoscctrl::XOSCCTRL_SPEC>;
#[doc = "External Multipurpose Crystal Oscillator Control"]
pub mod xoscctrl;
#[doc = "DFLLCTRLA register accessor: an alias for `Reg<DFLLCTRLA_SPEC>`"]
pub type DFLLCTRLA = crate::Reg<dfllctrla::DFLLCTRLA_SPEC>;
#[doc = "DFLL48M Control A"]
pub mod dfllctrla;
#[doc = "DFLLCTRLB register accessor: an alias for `Reg<DFLLCTRLB_SPEC>`"]
pub type DFLLCTRLB = crate::Reg<dfllctrlb::DFLLCTRLB_SPEC>;
#[doc = "DFLL48M Control B"]
pub mod dfllctrlb;
#[doc = "DFLLVAL register accessor: an alias for `Reg<DFLLVAL_SPEC>`"]
pub type DFLLVAL = crate::Reg<dfllval::DFLLVAL_SPEC>;
#[doc = "DFLL48M Value"]
pub mod dfllval;
#[doc = "DFLLMUL register accessor: an alias for `Reg<DFLLMUL_SPEC>`"]
pub type DFLLMUL = crate::Reg<dfllmul::DFLLMUL_SPEC>;
#[doc = "DFLL48M Multiplier"]
pub mod dfllmul;
#[doc = "DFLLSYNC register accessor: an alias for `Reg<DFLLSYNC_SPEC>`"]
pub type DFLLSYNC = crate::Reg<dfllsync::DFLLSYNC_SPEC>;
#[doc = "DFLL48M Synchronization"]
pub mod dfllsync;
