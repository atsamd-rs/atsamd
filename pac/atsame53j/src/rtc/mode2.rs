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
#[doc = "CTRLA (rw) register accessor: MODE2 Control A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrla::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrla::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrla`]
module"]
pub type CTRLA = crate::Reg<ctrla::CTRLA_SPEC>;
#[doc = "MODE2 Control A"]
pub mod ctrla;
#[doc = "CTRLB (rw) register accessor: MODE2 Control B\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrlb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrlb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrlb`]
module"]
pub type CTRLB = crate::Reg<ctrlb::CTRLB_SPEC>;
#[doc = "MODE2 Control B"]
pub mod ctrlb;
#[doc = "EVCTRL (rw) register accessor: MODE2 Event Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`evctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`evctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@evctrl`]
module"]
pub type EVCTRL = crate::Reg<evctrl::EVCTRL_SPEC>;
#[doc = "MODE2 Event Control"]
pub mod evctrl;
#[doc = "INTENCLR (rw) register accessor: MODE2 Interrupt Enable Clear\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intenclr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intenclr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intenclr`]
module"]
pub type INTENCLR = crate::Reg<intenclr::INTENCLR_SPEC>;
#[doc = "MODE2 Interrupt Enable Clear"]
pub mod intenclr;
#[doc = "INTENSET (rw) register accessor: MODE2 Interrupt Enable Set\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intenset::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intenset::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intenset`]
module"]
pub type INTENSET = crate::Reg<intenset::INTENSET_SPEC>;
#[doc = "MODE2 Interrupt Enable Set"]
pub mod intenset;
#[doc = "INTFLAG (rw) register accessor: MODE2 Interrupt Flag Status and Clear\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intflag::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intflag::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intflag`]
module"]
pub type INTFLAG = crate::Reg<intflag::INTFLAG_SPEC>;
#[doc = "MODE2 Interrupt Flag Status and Clear"]
pub mod intflag;
#[doc = "DBGCTRL (rw) register accessor: Debug Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dbgctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dbgctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbgctrl`]
module"]
pub type DBGCTRL = crate::Reg<dbgctrl::DBGCTRL_SPEC>;
#[doc = "Debug Control"]
pub mod dbgctrl;
#[doc = "SYNCBUSY (r) register accessor: MODE2 Synchronization Busy Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`syncbusy::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@syncbusy`]
module"]
pub type SYNCBUSY = crate::Reg<syncbusy::SYNCBUSY_SPEC>;
#[doc = "MODE2 Synchronization Busy Status"]
pub mod syncbusy;
#[doc = "FREQCORR (rw) register accessor: Frequency Correction\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`freqcorr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`freqcorr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@freqcorr`]
module"]
pub type FREQCORR = crate::Reg<freqcorr::FREQCORR_SPEC>;
#[doc = "Frequency Correction"]
pub mod freqcorr;
#[doc = "CLOCK (rw) register accessor: MODE2 Clock Value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clock::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clock::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clock`]
module"]
pub type CLOCK = crate::Reg<clock::CLOCK_SPEC>;
#[doc = "MODE2 Clock Value"]
pub mod clock;
#[doc = "GP (rw) register accessor: General Purpose\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gp::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gp::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gp`]
module"]
pub type GP = crate::Reg<gp::GP_SPEC>;
#[doc = "General Purpose"]
pub mod gp;
#[doc = "ALARM0 (rw) register accessor: MODE2_ALARM Alarm n Value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`alarm0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`alarm0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@alarm0`]
module"]
pub type ALARM0 = crate::Reg<alarm0::ALARM0_SPEC>;
#[doc = "MODE2_ALARM Alarm n Value"]
pub mod alarm0;
#[doc = "MASK0 (rw) register accessor: MODE2_ALARM Alarm n Mask\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mask0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mask0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mask0`]
module"]
pub type MASK0 = crate::Reg<mask0::MASK0_SPEC>;
#[doc = "MODE2_ALARM Alarm n Mask"]
pub mod mask0;
#[doc = "ALARM1 (rw) register accessor: MODE2_ALARM Alarm n Value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`alarm1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`alarm1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@alarm1`]
module"]
pub type ALARM1 = crate::Reg<alarm1::ALARM1_SPEC>;
#[doc = "MODE2_ALARM Alarm n Value"]
pub mod alarm1;
#[doc = "MASK1 (rw) register accessor: MODE2_ALARM Alarm n Mask\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mask1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mask1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mask1`]
module"]
pub type MASK1 = crate::Reg<mask1::MASK1_SPEC>;
#[doc = "MODE2_ALARM Alarm n Mask"]
pub mod mask1;
#[doc = "TAMPCTRL (rw) register accessor: Tamper Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tampctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tampctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tampctrl`]
module"]
pub type TAMPCTRL = crate::Reg<tampctrl::TAMPCTRL_SPEC>;
#[doc = "Tamper Control"]
pub mod tampctrl;
#[doc = "TIMESTAMP (r) register accessor: MODE2 Timestamp\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timestamp::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timestamp`]
module"]
pub type TIMESTAMP = crate::Reg<timestamp::TIMESTAMP_SPEC>;
#[doc = "MODE2 Timestamp"]
pub mod timestamp;
#[doc = "TAMPID (rw) register accessor: Tamper ID\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tampid::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tampid::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tampid`]
module"]
pub type TAMPID = crate::Reg<tampid::TAMPID_SPEC>;
#[doc = "Tamper ID"]
pub mod tampid;
#[doc = "BKUP (rw) register accessor: Backup\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bkup::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bkup::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bkup`]
module"]
pub type BKUP = crate::Reg<bkup::BKUP_SPEC>;
#[doc = "Backup"]
pub mod bkup;
