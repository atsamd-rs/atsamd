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
#[doc = "CTRLA (rw) register accessor: USART Control A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrla::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrla::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrla`]
module"]
pub type CTRLA = crate::Reg<ctrla::CTRLA_SPEC>;
#[doc = "USART Control A"]
pub mod ctrla;
#[doc = "CTRLB (rw) register accessor: USART Control B\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrlb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrlb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrlb`]
module"]
pub type CTRLB = crate::Reg<ctrlb::CTRLB_SPEC>;
#[doc = "USART Control B"]
pub mod ctrlb;
#[doc = "BAUD (rw) register accessor: USART Baud Rate\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`baud::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`baud::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@baud`]
module"]
pub type BAUD = crate::Reg<baud::BAUD_SPEC>;
#[doc = "USART Baud Rate"]
pub mod baud;
#[doc = "BAUD_FRAC_MODE (rw) register accessor: USART Baud Rate\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`baud_frac_mode::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`baud_frac_mode::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@baud_frac_mode`]
module"]
pub type BAUD_FRAC_MODE = crate::Reg<baud_frac_mode::BAUD_FRAC_MODE_SPEC>;
#[doc = "USART Baud Rate"]
pub mod baud_frac_mode;
#[doc = "BAUD_FRACFP_MODE (rw) register accessor: USART Baud Rate\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`baud_fracfp_mode::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`baud_fracfp_mode::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@baud_fracfp_mode`]
module"]
pub type BAUD_FRACFP_MODE = crate::Reg<baud_fracfp_mode::BAUD_FRACFP_MODE_SPEC>;
#[doc = "USART Baud Rate"]
pub mod baud_fracfp_mode;
#[doc = "BAUD_USARTFP_MODE (rw) register accessor: USART Baud Rate\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`baud_usartfp_mode::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`baud_usartfp_mode::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@baud_usartfp_mode`]
module"]
pub type BAUD_USARTFP_MODE = crate::Reg<baud_usartfp_mode::BAUD_USARTFP_MODE_SPEC>;
#[doc = "USART Baud Rate"]
pub mod baud_usartfp_mode;
#[doc = "RXPL (rw) register accessor: USART Receive Pulse Length\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxpl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxpl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxpl`]
module"]
pub type RXPL = crate::Reg<rxpl::RXPL_SPEC>;
#[doc = "USART Receive Pulse Length"]
pub mod rxpl;
#[doc = "INTENCLR (rw) register accessor: USART Interrupt Enable Clear\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intenclr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intenclr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intenclr`]
module"]
pub type INTENCLR = crate::Reg<intenclr::INTENCLR_SPEC>;
#[doc = "USART Interrupt Enable Clear"]
pub mod intenclr;
#[doc = "INTENSET (rw) register accessor: USART Interrupt Enable Set\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intenset::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intenset::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intenset`]
module"]
pub type INTENSET = crate::Reg<intenset::INTENSET_SPEC>;
#[doc = "USART Interrupt Enable Set"]
pub mod intenset;
#[doc = "INTFLAG (rw) register accessor: USART Interrupt Flag Status and Clear\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intflag::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intflag::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intflag`]
module"]
pub type INTFLAG = crate::Reg<intflag::INTFLAG_SPEC>;
#[doc = "USART Interrupt Flag Status and Clear"]
pub mod intflag;
#[doc = "STATUS (rw) register accessor: USART Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`]
module"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "USART Status"]
pub mod status;
#[doc = "SYNCBUSY (r) register accessor: USART Syncbusy\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`syncbusy::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@syncbusy`]
module"]
pub type SYNCBUSY = crate::Reg<syncbusy::SYNCBUSY_SPEC>;
#[doc = "USART Syncbusy"]
pub mod syncbusy;
#[doc = "DATA (rw) register accessor: USART Data\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`data::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`data::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data`]
module"]
pub type DATA = crate::Reg<data::DATA_SPEC>;
#[doc = "USART Data"]
pub mod data;
#[doc = "DBGCTRL (rw) register accessor: USART Debug Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dbgctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dbgctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbgctrl`]
module"]
pub type DBGCTRL = crate::Reg<dbgctrl::DBGCTRL_SPEC>;
#[doc = "USART Debug Control"]
pub mod dbgctrl;
