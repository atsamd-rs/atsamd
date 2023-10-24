#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control A"]
    pub ctrla: CTRLA,
    #[doc = "0x01 - Reference Control"]
    pub refctrl: REFCTRL,
    #[doc = "0x02 - Average Control"]
    pub avgctrl: AVGCTRL,
    #[doc = "0x03 - Sampling Time Control"]
    pub sampctrl: SAMPCTRL,
    #[doc = "0x04 - Control B"]
    pub ctrlb: CTRLB,
    _reserved5: [u8; 0x02],
    #[doc = "0x08 - Window Monitor Control"]
    pub winctrl: WINCTRL,
    _reserved6: [u8; 0x03],
    #[doc = "0x0c - Software Trigger"]
    pub swtrig: SWTRIG,
    _reserved7: [u8; 0x03],
    #[doc = "0x10 - Input Control"]
    pub inputctrl: INPUTCTRL,
    #[doc = "0x14 - Event Control"]
    pub evctrl: EVCTRL,
    _reserved9: [u8; 0x01],
    #[doc = "0x16 - Interrupt Enable Clear"]
    pub intenclr: INTENCLR,
    #[doc = "0x17 - Interrupt Enable Set"]
    pub intenset: INTENSET,
    #[doc = "0x18 - Interrupt Flag Status and Clear"]
    pub intflag: INTFLAG,
    #[doc = "0x19 - Status"]
    pub status: STATUS,
    #[doc = "0x1a - Result"]
    pub result: RESULT,
    #[doc = "0x1c - Window Monitor Lower Threshold"]
    pub winlt: WINLT,
    _reserved15: [u8; 0x02],
    #[doc = "0x20 - Window Monitor Upper Threshold"]
    pub winut: WINUT,
    _reserved16: [u8; 0x02],
    #[doc = "0x24 - Gain Correction"]
    pub gaincorr: GAINCORR,
    #[doc = "0x26 - Offset Correction"]
    pub offsetcorr: OFFSETCORR,
    #[doc = "0x28 - Calibration"]
    pub calib: CALIB,
    #[doc = "0x2a - Debug Control"]
    pub dbgctrl: DBGCTRL,
}
#[doc = "CTRLA (rw) register accessor: Control A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrla::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrla::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrla`]
module"]
pub type CTRLA = crate::Reg<ctrla::CTRLA_SPEC>;
#[doc = "Control A"]
pub mod ctrla;
#[doc = "REFCTRL (rw) register accessor: Reference Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`refctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`refctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@refctrl`]
module"]
pub type REFCTRL = crate::Reg<refctrl::REFCTRL_SPEC>;
#[doc = "Reference Control"]
pub mod refctrl;
#[doc = "AVGCTRL (rw) register accessor: Average Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`avgctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`avgctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@avgctrl`]
module"]
pub type AVGCTRL = crate::Reg<avgctrl::AVGCTRL_SPEC>;
#[doc = "Average Control"]
pub mod avgctrl;
#[doc = "SAMPCTRL (rw) register accessor: Sampling Time Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sampctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sampctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sampctrl`]
module"]
pub type SAMPCTRL = crate::Reg<sampctrl::SAMPCTRL_SPEC>;
#[doc = "Sampling Time Control"]
pub mod sampctrl;
#[doc = "CTRLB (rw) register accessor: Control B\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrlb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrlb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrlb`]
module"]
pub type CTRLB = crate::Reg<ctrlb::CTRLB_SPEC>;
#[doc = "Control B"]
pub mod ctrlb;
#[doc = "WINCTRL (rw) register accessor: Window Monitor Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`winctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`winctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@winctrl`]
module"]
pub type WINCTRL = crate::Reg<winctrl::WINCTRL_SPEC>;
#[doc = "Window Monitor Control"]
pub mod winctrl;
#[doc = "SWTRIG (rw) register accessor: Software Trigger\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swtrig::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swtrig::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swtrig`]
module"]
pub type SWTRIG = crate::Reg<swtrig::SWTRIG_SPEC>;
#[doc = "Software Trigger"]
pub mod swtrig;
#[doc = "INPUTCTRL (rw) register accessor: Input Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`inputctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`inputctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@inputctrl`]
module"]
pub type INPUTCTRL = crate::Reg<inputctrl::INPUTCTRL_SPEC>;
#[doc = "Input Control"]
pub mod inputctrl;
#[doc = "EVCTRL (rw) register accessor: Event Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`evctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`evctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@evctrl`]
module"]
pub type EVCTRL = crate::Reg<evctrl::EVCTRL_SPEC>;
#[doc = "Event Control"]
pub mod evctrl;
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
#[doc = "STATUS (r) register accessor: Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`]
module"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "Status"]
pub mod status;
#[doc = "RESULT (r) register accessor: Result\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`result::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@result`]
module"]
pub type RESULT = crate::Reg<result::RESULT_SPEC>;
#[doc = "Result"]
pub mod result;
#[doc = "WINLT (rw) register accessor: Window Monitor Lower Threshold\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`winlt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`winlt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@winlt`]
module"]
pub type WINLT = crate::Reg<winlt::WINLT_SPEC>;
#[doc = "Window Monitor Lower Threshold"]
pub mod winlt;
#[doc = "WINUT (rw) register accessor: Window Monitor Upper Threshold\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`winut::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`winut::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@winut`]
module"]
pub type WINUT = crate::Reg<winut::WINUT_SPEC>;
#[doc = "Window Monitor Upper Threshold"]
pub mod winut;
#[doc = "GAINCORR (rw) register accessor: Gain Correction\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gaincorr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gaincorr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gaincorr`]
module"]
pub type GAINCORR = crate::Reg<gaincorr::GAINCORR_SPEC>;
#[doc = "Gain Correction"]
pub mod gaincorr;
#[doc = "OFFSETCORR (rw) register accessor: Offset Correction\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`offsetcorr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`offsetcorr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@offsetcorr`]
module"]
pub type OFFSETCORR = crate::Reg<offsetcorr::OFFSETCORR_SPEC>;
#[doc = "Offset Correction"]
pub mod offsetcorr;
#[doc = "CALIB (rw) register accessor: Calibration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`calib::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`calib::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@calib`]
module"]
pub type CALIB = crate::Reg<calib::CALIB_SPEC>;
#[doc = "Calibration"]
pub mod calib;
#[doc = "DBGCTRL (rw) register accessor: Debug Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dbgctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dbgctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbgctrl`]
module"]
pub type DBGCTRL = crate::Reg<dbgctrl::DBGCTRL_SPEC>;
#[doc = "Debug Control"]
pub mod dbgctrl;
