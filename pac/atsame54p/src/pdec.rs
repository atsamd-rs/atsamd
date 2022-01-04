#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control A"]
    pub ctrla: crate::Reg<ctrla::CTRLA_SPEC>,
    #[doc = "0x04 - Control B Clear"]
    pub ctrlbclr: crate::Reg<ctrlbclr::CTRLBCLR_SPEC>,
    #[doc = "0x05 - Control B Set"]
    pub ctrlbset: crate::Reg<ctrlbset::CTRLBSET_SPEC>,
    #[doc = "0x06 - Event Control"]
    pub evctrl: crate::Reg<evctrl::EVCTRL_SPEC>,
    #[doc = "0x08 - Interrupt Enable Clear"]
    pub intenclr: crate::Reg<intenclr::INTENCLR_SPEC>,
    #[doc = "0x09 - Interrupt Enable Set"]
    pub intenset: crate::Reg<intenset::INTENSET_SPEC>,
    #[doc = "0x0a - Interrupt Flag Status and Clear"]
    pub intflag: crate::Reg<intflag::INTFLAG_SPEC>,
    _reserved7: [u8; 0x01],
    #[doc = "0x0c - Status"]
    pub status: crate::Reg<status::STATUS_SPEC>,
    _reserved8: [u8; 0x01],
    #[doc = "0x0f - Debug Control"]
    pub dbgctrl: crate::Reg<dbgctrl::DBGCTRL_SPEC>,
    #[doc = "0x10 - Synchronization Status"]
    pub syncbusy: crate::Reg<syncbusy::SYNCBUSY_SPEC>,
    #[doc = "0x14 - Prescaler Value"]
    pub presc: crate::Reg<presc::PRESC_SPEC>,
    #[doc = "0x15 - Filter Value"]
    pub filter: crate::Reg<filter::FILTER_SPEC>,
    _reserved12: [u8; 0x02],
    #[doc = "0x18 - Prescaler Buffer Value"]
    pub prescbuf: crate::Reg<prescbuf::PRESCBUF_SPEC>,
    #[doc = "0x19 - Filter Buffer Value"]
    pub filterbuf: crate::Reg<filterbuf::FILTERBUF_SPEC>,
    _reserved14: [u8; 0x02],
    #[doc = "0x1c - Counter Value"]
    pub count: crate::Reg<count::COUNT_SPEC>,
    #[doc = "0x20..0x28 - Channel n Compare Value"]
    pub cc: [crate::Reg<cc::CC_SPEC>; 2],
    _reserved16: [u8; 0x08],
    #[doc = "0x30..0x38 - Channel Compare Buffer Value"]
    pub ccbuf: [crate::Reg<ccbuf::CCBUF_SPEC>; 2],
}
#[doc = "CTRLA register accessor: an alias for `Reg<CTRLA_SPEC>`"]
pub type CTRLA = crate::Reg<ctrla::CTRLA_SPEC>;
#[doc = "Control A"]
pub mod ctrla;
#[doc = "CTRLBCLR register accessor: an alias for `Reg<CTRLBCLR_SPEC>`"]
pub type CTRLBCLR = crate::Reg<ctrlbclr::CTRLBCLR_SPEC>;
#[doc = "Control B Clear"]
pub mod ctrlbclr;
#[doc = "CTRLBSET register accessor: an alias for `Reg<CTRLBSET_SPEC>`"]
pub type CTRLBSET = crate::Reg<ctrlbset::CTRLBSET_SPEC>;
#[doc = "Control B Set"]
pub mod ctrlbset;
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
#[doc = "DBGCTRL register accessor: an alias for `Reg<DBGCTRL_SPEC>`"]
pub type DBGCTRL = crate::Reg<dbgctrl::DBGCTRL_SPEC>;
#[doc = "Debug Control"]
pub mod dbgctrl;
#[doc = "SYNCBUSY register accessor: an alias for `Reg<SYNCBUSY_SPEC>`"]
pub type SYNCBUSY = crate::Reg<syncbusy::SYNCBUSY_SPEC>;
#[doc = "Synchronization Status"]
pub mod syncbusy;
#[doc = "PRESC register accessor: an alias for `Reg<PRESC_SPEC>`"]
pub type PRESC = crate::Reg<presc::PRESC_SPEC>;
#[doc = "Prescaler Value"]
pub mod presc;
#[doc = "FILTER register accessor: an alias for `Reg<FILTER_SPEC>`"]
pub type FILTER = crate::Reg<filter::FILTER_SPEC>;
#[doc = "Filter Value"]
pub mod filter;
#[doc = "PRESCBUF register accessor: an alias for `Reg<PRESCBUF_SPEC>`"]
pub type PRESCBUF = crate::Reg<prescbuf::PRESCBUF_SPEC>;
#[doc = "Prescaler Buffer Value"]
pub mod prescbuf;
#[doc = "FILTERBUF register accessor: an alias for `Reg<FILTERBUF_SPEC>`"]
pub type FILTERBUF = crate::Reg<filterbuf::FILTERBUF_SPEC>;
#[doc = "Filter Buffer Value"]
pub mod filterbuf;
#[doc = "COUNT register accessor: an alias for `Reg<COUNT_SPEC>`"]
pub type COUNT = crate::Reg<count::COUNT_SPEC>;
#[doc = "Counter Value"]
pub mod count;
#[doc = "CC register accessor: an alias for `Reg<CC_SPEC>`"]
pub type CC = crate::Reg<cc::CC_SPEC>;
#[doc = "Channel n Compare Value"]
pub mod cc;
#[doc = "CCBUF register accessor: an alias for `Reg<CCBUF_SPEC>`"]
pub type CCBUF = crate::Reg<ccbuf::CCBUF_SPEC>;
#[doc = "Channel Compare Buffer Value"]
pub mod ccbuf;
