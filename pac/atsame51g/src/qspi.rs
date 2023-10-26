#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control A"]
    pub ctrla: CTRLA,
    #[doc = "0x04 - Control B"]
    pub ctrlb: CTRLB,
    #[doc = "0x08 - Baud Rate"]
    pub baud: BAUD,
    #[doc = "0x0c - Receive Data"]
    pub rxdata: RXDATA,
    #[doc = "0x10 - Transmit Data"]
    pub txdata: TXDATA,
    #[doc = "0x14 - Interrupt Enable Clear"]
    pub intenclr: INTENCLR,
    #[doc = "0x18 - Interrupt Enable Set"]
    pub intenset: INTENSET,
    #[doc = "0x1c - Interrupt Flag Status and Clear"]
    pub intflag: INTFLAG,
    #[doc = "0x20 - Status Register"]
    pub status: STATUS,
    _reserved9: [u8; 0x0c],
    #[doc = "0x30 - Instruction Address"]
    pub instraddr: INSTRADDR,
    #[doc = "0x34 - Instruction Code"]
    pub instrctrl: INSTRCTRL,
    #[doc = "0x38 - Instruction Frame"]
    pub instrframe: INSTRFRAME,
    _reserved12: [u8; 0x04],
    #[doc = "0x40 - Scrambling Mode"]
    pub scrambctrl: SCRAMBCTRL,
    #[doc = "0x44 - Scrambling Key"]
    pub scrambkey: SCRAMBKEY,
}
#[doc = "CTRLA (rw) register accessor: Control A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrla::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrla::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrla`]
module"]
pub type CTRLA = crate::Reg<ctrla::CTRLA_SPEC>;
#[doc = "Control A"]
pub mod ctrla;
#[doc = "CTRLB (rw) register accessor: Control B\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrlb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrlb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrlb`]
module"]
pub type CTRLB = crate::Reg<ctrlb::CTRLB_SPEC>;
#[doc = "Control B"]
pub mod ctrlb;
#[doc = "BAUD (rw) register accessor: Baud Rate\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`baud::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`baud::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@baud`]
module"]
pub type BAUD = crate::Reg<baud::BAUD_SPEC>;
#[doc = "Baud Rate"]
pub mod baud;
#[doc = "RXDATA (r) register accessor: Receive Data\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxdata::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxdata`]
module"]
pub type RXDATA = crate::Reg<rxdata::RXDATA_SPEC>;
#[doc = "Receive Data"]
pub mod rxdata;
#[doc = "TXDATA (w) register accessor: Transmit Data\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txdata::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txdata`]
module"]
pub type TXDATA = crate::Reg<txdata::TXDATA_SPEC>;
#[doc = "Transmit Data"]
pub mod txdata;
#[doc = "INTENCLR (rw) register accessor: Interrupt Enable Clear\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intenclr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intenclr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intenclr`]
module"]
pub type INTENCLR = crate::Reg<intenclr::INTENCLR_SPEC>;
#[doc = "Interrupt Enable Clear"]
pub mod intenclr;
#[doc = "INTENSET (rw) register accessor: Interrupt Enable Set\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intenset::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intenset::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intenset`]
module"]
pub type INTENSET = crate::Reg<intenset::INTENSET_SPEC>;
#[doc = "Interrupt Enable Set"]
pub mod intenset;
#[doc = "INTFLAG (rw) register accessor: Interrupt Flag Status and Clear\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intflag::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intflag::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intflag`]
module"]
pub type INTFLAG = crate::Reg<intflag::INTFLAG_SPEC>;
#[doc = "Interrupt Flag Status and Clear"]
pub mod intflag;
#[doc = "STATUS (r) register accessor: Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`]
module"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "Status Register"]
pub mod status;
#[doc = "INSTRADDR (rw) register accessor: Instruction Address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`instraddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`instraddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@instraddr`]
module"]
pub type INSTRADDR = crate::Reg<instraddr::INSTRADDR_SPEC>;
#[doc = "Instruction Address"]
pub mod instraddr;
#[doc = "INSTRCTRL (rw) register accessor: Instruction Code\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`instrctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`instrctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@instrctrl`]
module"]
pub type INSTRCTRL = crate::Reg<instrctrl::INSTRCTRL_SPEC>;
#[doc = "Instruction Code"]
pub mod instrctrl;
#[doc = "INSTRFRAME (rw) register accessor: Instruction Frame\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`instrframe::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`instrframe::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@instrframe`]
module"]
pub type INSTRFRAME = crate::Reg<instrframe::INSTRFRAME_SPEC>;
#[doc = "Instruction Frame"]
pub mod instrframe;
#[doc = "SCRAMBCTRL (rw) register accessor: Scrambling Mode\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scrambctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scrambctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scrambctrl`]
module"]
pub type SCRAMBCTRL = crate::Reg<scrambctrl::SCRAMBCTRL_SPEC>;
#[doc = "Scrambling Mode"]
pub mod scrambctrl;
#[doc = "SCRAMBKEY (w) register accessor: Scrambling Key\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scrambkey::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scrambkey`]
module"]
pub type SCRAMBKEY = crate::Reg<scrambkey::SCRAMBKEY_SPEC>;
#[doc = "Scrambling Key"]
pub mod scrambkey;
