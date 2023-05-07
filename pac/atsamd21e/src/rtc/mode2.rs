#[doc = r"Register block"]
#[repr(C)]
pub struct MODE2 {
    #[doc = "0x00 - MODE2 Control"]
    pub ctrl: CTRL,
    #[doc = "0x02 - Read Request"]
    pub readreq: READREQ,
    #[doc = "0x04 - MODE2 Event Control"]
    pub evctrl: EVCTRL,
    #[doc = "0x06 - MODE2 Interrupt Enable Clear"]
    pub intenclr: INTENCLR,
    #[doc = "0x07 - MODE2 Interrupt Enable Set"]
    pub intenset: INTENSET,
    #[doc = "0x08 - MODE2 Interrupt Flag Status and Clear"]
    pub intflag: INTFLAG,
    _reserved6: [u8; 0x01],
    #[doc = "0x0a - Status"]
    pub status: STATUS,
    #[doc = "0x0b - Debug Control"]
    pub dbgctrl: DBGCTRL,
    #[doc = "0x0c - Frequency Correction"]
    pub freqcorr: FREQCORR,
    _reserved9: [u8; 0x03],
    #[doc = "0x10 - MODE2 Clock Value"]
    pub clock: CLOCK,
    _reserved10: [u8; 0x04],
    #[doc = "0x18 - MODE2 Alarm n Value"]
    pub alarm: [ALARM; 1],
    #[doc = "0x1c - MODE2 Alarm n Mask"]
    pub mask: [MASK; 1],
}
#[doc = "CTRL (rw) register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "MODE2 Control"]
pub mod ctrl;
#[doc = "READREQ (rw) register accessor: an alias for `Reg<READREQ_SPEC>`"]
pub type READREQ = crate::Reg<readreq::READREQ_SPEC>;
#[doc = "Read Request"]
pub mod readreq;
#[doc = "EVCTRL (rw) register accessor: an alias for `Reg<EVCTRL_SPEC>`"]
pub type EVCTRL = crate::Reg<evctrl::EVCTRL_SPEC>;
#[doc = "MODE2 Event Control"]
pub mod evctrl;
#[doc = "INTENCLR (rw) register accessor: an alias for `Reg<INTENCLR_SPEC>`"]
pub type INTENCLR = crate::Reg<intenclr::INTENCLR_SPEC>;
#[doc = "MODE2 Interrupt Enable Clear"]
pub mod intenclr;
#[doc = "INTENSET (rw) register accessor: an alias for `Reg<INTENSET_SPEC>`"]
pub type INTENSET = crate::Reg<intenset::INTENSET_SPEC>;
#[doc = "MODE2 Interrupt Enable Set"]
pub mod intenset;
#[doc = "INTFLAG (rw) register accessor: an alias for `Reg<INTFLAG_SPEC>`"]
pub type INTFLAG = crate::Reg<intflag::INTFLAG_SPEC>;
#[doc = "MODE2 Interrupt Flag Status and Clear"]
pub mod intflag;
#[doc = "STATUS (rw) register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "Status"]
pub mod status;
#[doc = "DBGCTRL (rw) register accessor: an alias for `Reg<DBGCTRL_SPEC>`"]
pub type DBGCTRL = crate::Reg<dbgctrl::DBGCTRL_SPEC>;
#[doc = "Debug Control"]
pub mod dbgctrl;
#[doc = "FREQCORR (rw) register accessor: an alias for `Reg<FREQCORR_SPEC>`"]
pub type FREQCORR = crate::Reg<freqcorr::FREQCORR_SPEC>;
#[doc = "Frequency Correction"]
pub mod freqcorr;
#[doc = "CLOCK (rw) register accessor: an alias for `Reg<CLOCK_SPEC>`"]
pub type CLOCK = crate::Reg<clock::CLOCK_SPEC>;
#[doc = "MODE2 Clock Value"]
pub mod clock;
#[doc = "ALARM (rw) register accessor: an alias for `Reg<ALARM_SPEC>`"]
pub type ALARM = crate::Reg<alarm::ALARM_SPEC>;
#[doc = "MODE2 Alarm n Value"]
pub mod alarm;
#[doc = "MASK (rw) register accessor: an alias for `Reg<MASK_SPEC>`"]
pub type MASK = crate::Reg<mask::MASK_SPEC>;
#[doc = "MODE2 Alarm n Mask"]
pub mod mask;
