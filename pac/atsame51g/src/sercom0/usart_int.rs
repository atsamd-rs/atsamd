#[doc = r"Register block"]
#[repr(C)]
pub struct USART_INT {
    #[doc = "0x00 - USART_INT Control A"]
    pub ctrla: CTRLA,
    #[doc = "0x04 - USART_INT Control B"]
    pub ctrlb: CTRLB,
    #[doc = "0x08 - USART_INT Control C"]
    pub ctrlc: CTRLC,
    _reserved_3_baud: [u8; 0x02],
    #[doc = "0x0e - USART_INT Receive Pulse Length"]
    pub rxpl: RXPL,
    _reserved5: [u8; 0x05],
    #[doc = "0x14 - USART_INT Interrupt Enable Clear"]
    pub intenclr: INTENCLR,
    _reserved6: [u8; 0x01],
    #[doc = "0x16 - USART_INT Interrupt Enable Set"]
    pub intenset: INTENSET,
    _reserved7: [u8; 0x01],
    #[doc = "0x18 - USART_INT Interrupt Flag Status and Clear"]
    pub intflag: INTFLAG,
    _reserved8: [u8; 0x01],
    #[doc = "0x1a - USART_INT Status"]
    pub status: STATUS,
    #[doc = "0x1c - USART_INT Synchronization Busy"]
    pub syncbusy: SYNCBUSY,
    #[doc = "0x20 - USART_INT Receive Error Count"]
    pub rxerrcnt: RXERRCNT,
    _reserved11: [u8; 0x01],
    #[doc = "0x22 - USART_INT Length"]
    pub length: LENGTH,
    _reserved12: [u8; 0x04],
    #[doc = "0x28 - USART_INT Data"]
    pub data: DATA,
    _reserved13: [u8; 0x04],
    #[doc = "0x30 - USART_INT Debug Control"]
    pub dbgctrl: DBGCTRL,
}
impl USART_INT {
    #[doc = "0x0c - USART_INT Baud Rate"]
    #[inline(always)]
    pub const fn baud_usartfp_mode(&self) -> &BAUD_USARTFP_MODE {
        unsafe { &*(self as *const Self).cast::<u8>().add(12usize).cast() }
    }
    #[doc = "0x0c - USART_INT Baud Rate"]
    #[inline(always)]
    pub const fn baud_fracfp_mode(&self) -> &BAUD_FRACFP_MODE {
        unsafe { &*(self as *const Self).cast::<u8>().add(12usize).cast() }
    }
    #[doc = "0x0c - USART_INT Baud Rate"]
    #[inline(always)]
    pub const fn baud_frac_mode(&self) -> &BAUD_FRAC_MODE {
        unsafe { &*(self as *const Self).cast::<u8>().add(12usize).cast() }
    }
    #[doc = "0x0c - USART_INT Baud Rate"]
    #[inline(always)]
    pub const fn baud(&self) -> &BAUD {
        unsafe { &*(self as *const Self).cast::<u8>().add(12usize).cast() }
    }
}
#[doc = "CTRLA (rw) register accessor: an alias for `Reg<CTRLA_SPEC>`"]
pub type CTRLA = crate::Reg<ctrla::CTRLA_SPEC>;
#[doc = "USART_INT Control A"]
pub mod ctrla;
#[doc = "CTRLB (rw) register accessor: an alias for `Reg<CTRLB_SPEC>`"]
pub type CTRLB = crate::Reg<ctrlb::CTRLB_SPEC>;
#[doc = "USART_INT Control B"]
pub mod ctrlb;
#[doc = "CTRLC (rw) register accessor: an alias for `Reg<CTRLC_SPEC>`"]
pub type CTRLC = crate::Reg<ctrlc::CTRLC_SPEC>;
#[doc = "USART_INT Control C"]
pub mod ctrlc;
#[doc = "BAUD (rw) register accessor: an alias for `Reg<BAUD_SPEC>`"]
pub type BAUD = crate::Reg<baud::BAUD_SPEC>;
#[doc = "USART_INT Baud Rate"]
pub mod baud;
#[doc = "BAUD_FRAC_MODE (rw) register accessor: an alias for `Reg<BAUD_FRAC_MODE_SPEC>`"]
pub type BAUD_FRAC_MODE = crate::Reg<baud_frac_mode::BAUD_FRAC_MODE_SPEC>;
#[doc = "USART_INT Baud Rate"]
pub mod baud_frac_mode;
#[doc = "BAUD_FRACFP_MODE (rw) register accessor: an alias for `Reg<BAUD_FRACFP_MODE_SPEC>`"]
pub type BAUD_FRACFP_MODE = crate::Reg<baud_fracfp_mode::BAUD_FRACFP_MODE_SPEC>;
#[doc = "USART_INT Baud Rate"]
pub mod baud_fracfp_mode;
#[doc = "BAUD_USARTFP_MODE (rw) register accessor: an alias for `Reg<BAUD_USARTFP_MODE_SPEC>`"]
pub type BAUD_USARTFP_MODE = crate::Reg<baud_usartfp_mode::BAUD_USARTFP_MODE_SPEC>;
#[doc = "USART_INT Baud Rate"]
pub mod baud_usartfp_mode;
#[doc = "RXPL (rw) register accessor: an alias for `Reg<RXPL_SPEC>`"]
pub type RXPL = crate::Reg<rxpl::RXPL_SPEC>;
#[doc = "USART_INT Receive Pulse Length"]
pub mod rxpl;
#[doc = "INTENCLR (rw) register accessor: an alias for `Reg<INTENCLR_SPEC>`"]
pub type INTENCLR = crate::Reg<intenclr::INTENCLR_SPEC>;
#[doc = "USART_INT Interrupt Enable Clear"]
pub mod intenclr;
#[doc = "INTENSET (rw) register accessor: an alias for `Reg<INTENSET_SPEC>`"]
pub type INTENSET = crate::Reg<intenset::INTENSET_SPEC>;
#[doc = "USART_INT Interrupt Enable Set"]
pub mod intenset;
#[doc = "INTFLAG (rw) register accessor: an alias for `Reg<INTFLAG_SPEC>`"]
pub type INTFLAG = crate::Reg<intflag::INTFLAG_SPEC>;
#[doc = "USART_INT Interrupt Flag Status and Clear"]
pub mod intflag;
#[doc = "STATUS (rw) register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "USART_INT Status"]
pub mod status;
#[doc = "SYNCBUSY (r) register accessor: an alias for `Reg<SYNCBUSY_SPEC>`"]
pub type SYNCBUSY = crate::Reg<syncbusy::SYNCBUSY_SPEC>;
#[doc = "USART_INT Synchronization Busy"]
pub mod syncbusy;
#[doc = "RXERRCNT (r) register accessor: an alias for `Reg<RXERRCNT_SPEC>`"]
pub type RXERRCNT = crate::Reg<rxerrcnt::RXERRCNT_SPEC>;
#[doc = "USART_INT Receive Error Count"]
pub mod rxerrcnt;
#[doc = "LENGTH (rw) register accessor: an alias for `Reg<LENGTH_SPEC>`"]
pub type LENGTH = crate::Reg<length::LENGTH_SPEC>;
#[doc = "USART_INT Length"]
pub mod length;
#[doc = "DATA (rw) register accessor: an alias for `Reg<DATA_SPEC>`"]
pub type DATA = crate::Reg<data::DATA_SPEC>;
#[doc = "USART_INT Data"]
pub mod data;
#[doc = "DBGCTRL (rw) register accessor: an alias for `Reg<DBGCTRL_SPEC>`"]
pub type DBGCTRL = crate::Reg<dbgctrl::DBGCTRL_SPEC>;
#[doc = "USART_INT Debug Control"]
pub mod dbgctrl;
