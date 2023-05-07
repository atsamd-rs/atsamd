#[doc = r"Register block"]
#[repr(C)]
pub struct MODE2 {
    #[doc = "0x00 - MODE2 Control A"]
    pub ctrla: CTRLA,
    #[doc = "0x02 - MODE2 Control B"]
    pub ctrlb: CTRLB,
    #[doc = "0x04 - MODE2 Event Control"]
    pub evctrl: EVCTRL,
    #[doc = "0x08 - MODE2 Interrupt Enable Clear"]
    pub intenclr: INTENCLR,
    #[doc = "0x0a - MODE2 Interrupt Enable Set"]
    pub intenset: INTENSET,
    #[doc = "0x0c - MODE2 Interrupt Flag Status and Clear"]
    pub intflag: INTFLAG,
    #[doc = "0x0e - Debug Control"]
    pub dbgctrl: DBGCTRL,
    _reserved7: [u8; 0x01],
    #[doc = "0x10 - MODE2 Synchronization Busy Status"]
    pub syncbusy: SYNCBUSY,
    #[doc = "0x14 - Frequency Correction"]
    pub freqcorr: FREQCORR,
    _reserved9: [u8; 0x03],
    #[doc = "0x18 - MODE2 Clock Value"]
    pub clock: CLOCK,
    _reserved10: [u8; 0x04],
    #[doc = "0x20 - MODE2_ALARM Alarm n Value"]
    pub alarm0: ALARM0,
    #[doc = "0x24 - MODE2_ALARM Alarm n Mask"]
    pub mask0: MASK0,
    _reserved12: [u8; 0x03],
    #[doc = "0x28 - MODE2_ALARM Alarm n Value"]
    pub alarm1: ALARM1,
    #[doc = "0x2c - MODE2_ALARM Alarm n Mask"]
    pub mask1: MASK1,
    _reserved14: [u8; 0x13],
    #[doc = "0x40..0x50 - General Purpose"]
    pub gp: [GP; 4],
    _reserved15: [u8; 0x10],
    #[doc = "0x60 - Tamper Control"]
    pub tampctrl: TAMPCTRL,
    #[doc = "0x64 - MODE2 Timestamp"]
    pub timestamp: TIMESTAMP,
    #[doc = "0x68 - Tamper ID"]
    pub tampid: TAMPID,
    _reserved18: [u8; 0x14],
    #[doc = "0x80..0xa0 - Backup"]
    pub bkup: [BKUP; 8],
}
#[doc = "CTRLA (rw) register accessor: an alias for `Reg<CTRLA_SPEC>`"]
pub type CTRLA = crate::Reg<ctrla::CTRLA_SPEC>;
#[doc = "MODE2 Control A"]
pub mod ctrla;
#[doc = "CTRLB (rw) register accessor: an alias for `Reg<CTRLB_SPEC>`"]
pub type CTRLB = crate::Reg<ctrlb::CTRLB_SPEC>;
#[doc = "MODE2 Control B"]
pub mod ctrlb;
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
#[doc = "DBGCTRL (rw) register accessor: an alias for `Reg<DBGCTRL_SPEC>`"]
pub type DBGCTRL = crate::Reg<dbgctrl::DBGCTRL_SPEC>;
#[doc = "Debug Control"]
pub mod dbgctrl;
#[doc = "SYNCBUSY (r) register accessor: an alias for `Reg<SYNCBUSY_SPEC>`"]
pub type SYNCBUSY = crate::Reg<syncbusy::SYNCBUSY_SPEC>;
#[doc = "MODE2 Synchronization Busy Status"]
pub mod syncbusy;
#[doc = "FREQCORR (rw) register accessor: an alias for `Reg<FREQCORR_SPEC>`"]
pub type FREQCORR = crate::Reg<freqcorr::FREQCORR_SPEC>;
#[doc = "Frequency Correction"]
pub mod freqcorr;
#[doc = "CLOCK (rw) register accessor: an alias for `Reg<CLOCK_SPEC>`"]
pub type CLOCK = crate::Reg<clock::CLOCK_SPEC>;
#[doc = "MODE2 Clock Value"]
pub mod clock;
#[doc = "GP (rw) register accessor: an alias for `Reg<GP_SPEC>`"]
pub type GP = crate::Reg<gp::GP_SPEC>;
#[doc = "General Purpose"]
pub mod gp;
#[doc = "ALARM0 (rw) register accessor: an alias for `Reg<ALARM0_SPEC>`"]
pub type ALARM0 = crate::Reg<alarm0::ALARM0_SPEC>;
#[doc = "MODE2_ALARM Alarm n Value"]
pub mod alarm0;
#[doc = "MASK0 (rw) register accessor: an alias for `Reg<MASK0_SPEC>`"]
pub type MASK0 = crate::Reg<mask0::MASK0_SPEC>;
#[doc = "MODE2_ALARM Alarm n Mask"]
pub mod mask0;
#[doc = "ALARM1 (rw) register accessor: an alias for `Reg<ALARM1_SPEC>`"]
pub type ALARM1 = crate::Reg<alarm1::ALARM1_SPEC>;
#[doc = "MODE2_ALARM Alarm n Value"]
pub mod alarm1;
#[doc = "MASK1 (rw) register accessor: an alias for `Reg<MASK1_SPEC>`"]
pub type MASK1 = crate::Reg<mask1::MASK1_SPEC>;
#[doc = "MODE2_ALARM Alarm n Mask"]
pub mod mask1;
#[doc = "TAMPCTRL (rw) register accessor: an alias for `Reg<TAMPCTRL_SPEC>`"]
pub type TAMPCTRL = crate::Reg<tampctrl::TAMPCTRL_SPEC>;
#[doc = "Tamper Control"]
pub mod tampctrl;
#[doc = "TIMESTAMP (r) register accessor: an alias for `Reg<TIMESTAMP_SPEC>`"]
pub type TIMESTAMP = crate::Reg<timestamp::TIMESTAMP_SPEC>;
#[doc = "MODE2 Timestamp"]
pub mod timestamp;
#[doc = "TAMPID (rw) register accessor: an alias for `Reg<TAMPID_SPEC>`"]
pub type TAMPID = crate::Reg<tampid::TAMPID_SPEC>;
#[doc = "Tamper ID"]
pub mod tampid;
#[doc = "BKUP (rw) register accessor: an alias for `Reg<BKUP_SPEC>`"]
pub type BKUP = crate::Reg<bkup::BKUP_SPEC>;
#[doc = "Backup"]
pub mod bkup;
