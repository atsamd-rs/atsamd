#[doc = r"Register block"]
#[repr(C)]
pub struct USART {
    #[doc = "0x00 - USART Control A"]
    pub ctrla: CTRLA,
    #[doc = "0x04 - USART Control B"]
    pub ctrlb: CTRLB,
    _reserved2: [u8; 0x04],
    _reserved_2_baud: [u8; 0x02],
    #[doc = "0x0e - USART Receive Pulse Length"]
    pub rxpl: RXPL,
    _reserved4: [u8; 0x05],
    #[doc = "0x14 - USART Interrupt Enable Clear"]
    pub intenclr: INTENCLR,
    _reserved5: [u8; 0x01],
    #[doc = "0x16 - USART Interrupt Enable Set"]
    pub intenset: INTENSET,
    _reserved6: [u8; 0x01],
    #[doc = "0x18 - USART Interrupt Flag Status and Clear"]
    pub intflag: INTFLAG,
    _reserved7: [u8; 0x01],
    #[doc = "0x1a - USART Status"]
    pub status: STATUS,
    #[doc = "0x1c - USART Syncbusy"]
    pub syncbusy: SYNCBUSY,
    _reserved9: [u8; 0x08],
    #[doc = "0x28 - USART Data"]
    pub data: DATA,
    _reserved10: [u8; 0x06],
    #[doc = "0x30 - USART Debug Control"]
    pub dbgctrl: DBGCTRL,
}
impl USART {
    #[doc = "0x0c - USART Baud Rate"]
    #[inline(always)]
    pub const fn baud_usartfp_mode(&self) -> &BAUD_USARTFP_MODE {
        unsafe { &*(self as *const Self).cast::<u8>().add(12usize).cast() }
    }
    #[doc = "0x0c - USART Baud Rate"]
    #[inline(always)]
    pub const fn baud_fracfp_mode(&self) -> &BAUD_FRACFP_MODE {
        unsafe { &*(self as *const Self).cast::<u8>().add(12usize).cast() }
    }
    #[doc = "0x0c - USART Baud Rate"]
    #[inline(always)]
    pub const fn baud_frac_mode(&self) -> &BAUD_FRAC_MODE {
        unsafe { &*(self as *const Self).cast::<u8>().add(12usize).cast() }
    }
    #[doc = "0x0c - USART Baud Rate"]
    #[inline(always)]
    pub const fn baud(&self) -> &BAUD {
        unsafe { &*(self as *const Self).cast::<u8>().add(12usize).cast() }
    }
}
#[doc = "CTRLA (rw) register accessor: an alias for `Reg<CTRLA_SPEC>`"]
pub type CTRLA = crate::Reg<ctrla::CTRLA_SPEC>;
#[doc = "USART Control A"]
pub mod ctrla;
#[doc = "CTRLB (rw) register accessor: an alias for `Reg<CTRLB_SPEC>`"]
pub type CTRLB = crate::Reg<ctrlb::CTRLB_SPEC>;
#[doc = "USART Control B"]
pub mod ctrlb;
#[doc = "BAUD (rw) register accessor: an alias for `Reg<BAUD_SPEC>`"]
pub type BAUD = crate::Reg<baud::BAUD_SPEC>;
#[doc = "USART Baud Rate"]
pub mod baud;
#[doc = "BAUD_FRAC_MODE (rw) register accessor: an alias for `Reg<BAUD_FRAC_MODE_SPEC>`"]
pub type BAUD_FRAC_MODE = crate::Reg<baud_frac_mode::BAUD_FRAC_MODE_SPEC>;
#[doc = "USART Baud Rate"]
pub mod baud_frac_mode;
#[doc = "BAUD_FRACFP_MODE (rw) register accessor: an alias for `Reg<BAUD_FRACFP_MODE_SPEC>`"]
pub type BAUD_FRACFP_MODE = crate::Reg<baud_fracfp_mode::BAUD_FRACFP_MODE_SPEC>;
#[doc = "USART Baud Rate"]
pub mod baud_fracfp_mode;
#[doc = "BAUD_USARTFP_MODE (rw) register accessor: an alias for `Reg<BAUD_USARTFP_MODE_SPEC>`"]
pub type BAUD_USARTFP_MODE = crate::Reg<baud_usartfp_mode::BAUD_USARTFP_MODE_SPEC>;
#[doc = "USART Baud Rate"]
pub mod baud_usartfp_mode;
#[doc = "RXPL (rw) register accessor: an alias for `Reg<RXPL_SPEC>`"]
pub type RXPL = crate::Reg<rxpl::RXPL_SPEC>;
#[doc = "USART Receive Pulse Length"]
pub mod rxpl;
#[doc = "INTENCLR (rw) register accessor: an alias for `Reg<INTENCLR_SPEC>`"]
pub type INTENCLR = crate::Reg<intenclr::INTENCLR_SPEC>;
#[doc = "USART Interrupt Enable Clear"]
pub mod intenclr;
#[doc = "INTENSET (rw) register accessor: an alias for `Reg<INTENSET_SPEC>`"]
pub type INTENSET = crate::Reg<intenset::INTENSET_SPEC>;
#[doc = "USART Interrupt Enable Set"]
pub mod intenset;
#[doc = "INTFLAG (rw) register accessor: an alias for `Reg<INTFLAG_SPEC>`"]
pub type INTFLAG = crate::Reg<intflag::INTFLAG_SPEC>;
#[doc = "USART Interrupt Flag Status and Clear"]
pub mod intflag;
#[doc = "STATUS (rw) register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "USART Status"]
pub mod status;
#[doc = "SYNCBUSY (r) register accessor: an alias for `Reg<SYNCBUSY_SPEC>`"]
pub type SYNCBUSY = crate::Reg<syncbusy::SYNCBUSY_SPEC>;
#[doc = "USART Syncbusy"]
pub mod syncbusy;
#[doc = "DATA (rw) register accessor: an alias for `Reg<DATA_SPEC>`"]
pub type DATA = crate::Reg<data::DATA_SPEC>;
#[doc = "USART Data"]
pub mod data;
#[doc = "DBGCTRL (rw) register accessor: an alias for `Reg<DBGCTRL_SPEC>`"]
pub type DBGCTRL = crate::Reg<dbgctrl::DBGCTRL_SPEC>;
#[doc = "USART Debug Control"]
pub mod dbgctrl;
