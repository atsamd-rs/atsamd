#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control A"]
    pub ctrla: CTRLA,
    #[doc = "0x04 - Control B Clear"]
    pub ctrlbclr: CTRLBCLR,
    #[doc = "0x05 - Control B Set"]
    pub ctrlbset: CTRLBSET,
    _reserved3: [u8; 0x02],
    #[doc = "0x08 - Synchronization Busy"]
    pub syncbusy: SYNCBUSY,
    #[doc = "0x0c - Recoverable Fault A Configuration"]
    pub fctrla: FCTRLA,
    #[doc = "0x10 - Recoverable Fault B Configuration"]
    pub fctrlb: FCTRLB,
    #[doc = "0x14 - Waveform Extension Configuration"]
    pub wexctrl: WEXCTRL,
    #[doc = "0x18 - Driver Control"]
    pub drvctrl: DRVCTRL,
    _reserved8: [u8; 0x02],
    #[doc = "0x1e - Debug Control"]
    pub dbgctrl: DBGCTRL,
    _reserved9: [u8; 0x01],
    #[doc = "0x20 - Event Control"]
    pub evctrl: EVCTRL,
    #[doc = "0x24 - Interrupt Enable Clear"]
    pub intenclr: INTENCLR,
    #[doc = "0x28 - Interrupt Enable Set"]
    pub intenset: INTENSET,
    #[doc = "0x2c - Interrupt Flag Status and Clear"]
    pub intflag: INTFLAG,
    #[doc = "0x30 - Status"]
    pub status: STATUS,
    _reserved_14_count: [u8; 0x04],
    #[doc = "0x38 - Pattern"]
    pub patt: PATT,
    _reserved16: [u8; 0x02],
    #[doc = "0x3c - Waveform Control"]
    pub wave: WAVE,
    _reserved_17_per: [u8; 0x04],
    _reserved_18_cc: [u8; 0x18],
    _reserved19: [u8; 0x08],
    #[doc = "0x64 - Pattern Buffer"]
    pub pattbuf: PATTBUF,
    _reserved20: [u8; 0x06],
    _reserved_20_perbuf: [u8; 0x04],
    _reserved_21_ccbuf: [u8; 0x18],
}
impl RegisterBlock {
    #[doc = "0x34 - Count"]
    #[inline(always)]
    pub const fn count_dith6_mode(&self) -> &COUNT_DITH6_MODE {
        unsafe { &*(self as *const Self).cast::<u8>().add(52usize).cast() }
    }
    #[doc = "0x34 - Count"]
    #[inline(always)]
    pub const fn count_dith5_mode(&self) -> &COUNT_DITH5_MODE {
        unsafe { &*(self as *const Self).cast::<u8>().add(52usize).cast() }
    }
    #[doc = "0x34 - Count"]
    #[inline(always)]
    pub const fn count_dith4_mode(&self) -> &COUNT_DITH4_MODE {
        unsafe { &*(self as *const Self).cast::<u8>().add(52usize).cast() }
    }
    #[doc = "0x34 - Count"]
    #[inline(always)]
    pub const fn count(&self) -> &COUNT {
        unsafe { &*(self as *const Self).cast::<u8>().add(52usize).cast() }
    }
    #[doc = "0x40 - Period"]
    #[inline(always)]
    pub const fn per_dith6_mode(&self) -> &PER_DITH6_MODE {
        unsafe { &*(self as *const Self).cast::<u8>().add(64usize).cast() }
    }
    #[doc = "0x40 - Period"]
    #[inline(always)]
    pub const fn per_dith5_mode(&self) -> &PER_DITH5_MODE {
        unsafe { &*(self as *const Self).cast::<u8>().add(64usize).cast() }
    }
    #[doc = "0x40 - Period"]
    #[inline(always)]
    pub const fn per_dith4_mode(&self) -> &PER_DITH4_MODE {
        unsafe { &*(self as *const Self).cast::<u8>().add(64usize).cast() }
    }
    #[doc = "0x40 - Period"]
    #[inline(always)]
    pub const fn per(&self) -> &PER {
        unsafe { &*(self as *const Self).cast::<u8>().add(64usize).cast() }
    }
    #[doc = "0x44..0x5c - Compare and Capture"]
    #[inline(always)]
    pub const fn cc_dith6_mode(&self) -> &[CC_DITH6_MODE; 6] {
        unsafe { &*(self as *const Self).cast::<u8>().add(68usize).cast() }
    }
    #[doc = "0x44..0x5c - Compare and Capture"]
    #[inline(always)]
    pub const fn cc_dith5_mode(&self) -> &[CC_DITH5_MODE; 6] {
        unsafe { &*(self as *const Self).cast::<u8>().add(68usize).cast() }
    }
    #[doc = "0x44..0x5c - Compare and Capture"]
    #[inline(always)]
    pub const fn cc_dith4_mode(&self) -> &[CC_DITH4_MODE; 6] {
        unsafe { &*(self as *const Self).cast::<u8>().add(68usize).cast() }
    }
    #[doc = "0x44..0x5c - Compare and Capture"]
    #[inline(always)]
    pub const fn cc(&self) -> &[CC; 6] {
        unsafe { &*(self as *const Self).cast::<u8>().add(68usize).cast() }
    }
    #[doc = "0x6c - Period Buffer"]
    #[inline(always)]
    pub const fn perbuf_dith6_mode(&self) -> &PERBUF_DITH6_MODE {
        unsafe { &*(self as *const Self).cast::<u8>().add(108usize).cast() }
    }
    #[doc = "0x6c - Period Buffer"]
    #[inline(always)]
    pub const fn perbuf_dith5_mode(&self) -> &PERBUF_DITH5_MODE {
        unsafe { &*(self as *const Self).cast::<u8>().add(108usize).cast() }
    }
    #[doc = "0x6c - Period Buffer"]
    #[inline(always)]
    pub const fn perbuf_dith4_mode(&self) -> &PERBUF_DITH4_MODE {
        unsafe { &*(self as *const Self).cast::<u8>().add(108usize).cast() }
    }
    #[doc = "0x6c - Period Buffer"]
    #[inline(always)]
    pub const fn perbuf(&self) -> &PERBUF {
        unsafe { &*(self as *const Self).cast::<u8>().add(108usize).cast() }
    }
    #[doc = "0x70..0x88 - Compare and Capture Buffer"]
    #[inline(always)]
    pub const fn ccbuf_dith6_mode(&self) -> &[CCBUF_DITH6_MODE; 6] {
        unsafe { &*(self as *const Self).cast::<u8>().add(112usize).cast() }
    }
    #[doc = "0x70..0x88 - Compare and Capture Buffer"]
    #[inline(always)]
    pub const fn ccbuf_dith5_mode(&self) -> &[CCBUF_DITH5_MODE; 6] {
        unsafe { &*(self as *const Self).cast::<u8>().add(112usize).cast() }
    }
    #[doc = "0x70..0x88 - Compare and Capture Buffer"]
    #[inline(always)]
    pub const fn ccbuf_dith4_mode(&self) -> &[CCBUF_DITH4_MODE; 6] {
        unsafe { &*(self as *const Self).cast::<u8>().add(112usize).cast() }
    }
    #[doc = "0x70..0x88 - Compare and Capture Buffer"]
    #[inline(always)]
    pub const fn ccbuf(&self) -> &[CCBUF; 6] {
        unsafe { &*(self as *const Self).cast::<u8>().add(112usize).cast() }
    }
}
#[doc = "CTRLA (rw) register accessor: Control A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrla::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrla::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrla`]
module"]
pub type CTRLA = crate::Reg<ctrla::CTRLA_SPEC>;
#[doc = "Control A"]
pub mod ctrla;
#[doc = "CTRLBCLR (rw) register accessor: Control B Clear\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrlbclr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrlbclr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrlbclr`]
module"]
pub type CTRLBCLR = crate::Reg<ctrlbclr::CTRLBCLR_SPEC>;
#[doc = "Control B Clear"]
pub mod ctrlbclr;
#[doc = "CTRLBSET (rw) register accessor: Control B Set\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrlbset::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrlbset::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrlbset`]
module"]
pub type CTRLBSET = crate::Reg<ctrlbset::CTRLBSET_SPEC>;
#[doc = "Control B Set"]
pub mod ctrlbset;
#[doc = "SYNCBUSY (r) register accessor: Synchronization Busy\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`syncbusy::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@syncbusy`]
module"]
pub type SYNCBUSY = crate::Reg<syncbusy::SYNCBUSY_SPEC>;
#[doc = "Synchronization Busy"]
pub mod syncbusy;
#[doc = "FCTRLA (rw) register accessor: Recoverable Fault A Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fctrla::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fctrla::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fctrla`]
module"]
pub type FCTRLA = crate::Reg<fctrla::FCTRLA_SPEC>;
#[doc = "Recoverable Fault A Configuration"]
pub mod fctrla;
#[doc = "FCTRLB (rw) register accessor: Recoverable Fault B Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fctrlb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fctrlb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fctrlb`]
module"]
pub type FCTRLB = crate::Reg<fctrlb::FCTRLB_SPEC>;
#[doc = "Recoverable Fault B Configuration"]
pub mod fctrlb;
#[doc = "WEXCTRL (rw) register accessor: Waveform Extension Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wexctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wexctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wexctrl`]
module"]
pub type WEXCTRL = crate::Reg<wexctrl::WEXCTRL_SPEC>;
#[doc = "Waveform Extension Configuration"]
pub mod wexctrl;
#[doc = "DRVCTRL (rw) register accessor: Driver Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`drvctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`drvctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@drvctrl`]
module"]
pub type DRVCTRL = crate::Reg<drvctrl::DRVCTRL_SPEC>;
#[doc = "Driver Control"]
pub mod drvctrl;
#[doc = "DBGCTRL (rw) register accessor: Debug Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dbgctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dbgctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbgctrl`]
module"]
pub type DBGCTRL = crate::Reg<dbgctrl::DBGCTRL_SPEC>;
#[doc = "Debug Control"]
pub mod dbgctrl;
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
#[doc = "STATUS (rw) register accessor: Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`]
module"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "Status"]
pub mod status;
#[doc = "COUNT (rw) register accessor: Count\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`count::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`count::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@count`]
module"]
pub type COUNT = crate::Reg<count::COUNT_SPEC>;
#[doc = "Count"]
pub mod count;
#[doc = "COUNT_DITH4_MODE (rw) register accessor: Count\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`count_dith4_mode::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`count_dith4_mode::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@count_dith4_mode`]
module"]
pub type COUNT_DITH4_MODE = crate::Reg<count_dith4_mode::COUNT_DITH4_MODE_SPEC>;
#[doc = "Count"]
pub mod count_dith4_mode;
#[doc = "COUNT_DITH5_MODE (rw) register accessor: Count\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`count_dith5_mode::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`count_dith5_mode::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@count_dith5_mode`]
module"]
pub type COUNT_DITH5_MODE = crate::Reg<count_dith5_mode::COUNT_DITH5_MODE_SPEC>;
#[doc = "Count"]
pub mod count_dith5_mode;
#[doc = "COUNT_DITH6_MODE (rw) register accessor: Count\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`count_dith6_mode::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`count_dith6_mode::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@count_dith6_mode`]
module"]
pub type COUNT_DITH6_MODE = crate::Reg<count_dith6_mode::COUNT_DITH6_MODE_SPEC>;
#[doc = "Count"]
pub mod count_dith6_mode;
#[doc = "PATT (rw) register accessor: Pattern\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`patt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`patt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@patt`]
module"]
pub type PATT = crate::Reg<patt::PATT_SPEC>;
#[doc = "Pattern"]
pub mod patt;
#[doc = "WAVE (rw) register accessor: Waveform Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wave::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wave::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wave`]
module"]
pub type WAVE = crate::Reg<wave::WAVE_SPEC>;
#[doc = "Waveform Control"]
pub mod wave;
#[doc = "PER (rw) register accessor: Period\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`per::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`per::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@per`]
module"]
pub type PER = crate::Reg<per::PER_SPEC>;
#[doc = "Period"]
pub mod per;
#[doc = "PER_DITH4_MODE (rw) register accessor: Period\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`per_dith4_mode::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`per_dith4_mode::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@per_dith4_mode`]
module"]
pub type PER_DITH4_MODE = crate::Reg<per_dith4_mode::PER_DITH4_MODE_SPEC>;
#[doc = "Period"]
pub mod per_dith4_mode;
#[doc = "PER_DITH5_MODE (rw) register accessor: Period\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`per_dith5_mode::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`per_dith5_mode::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@per_dith5_mode`]
module"]
pub type PER_DITH5_MODE = crate::Reg<per_dith5_mode::PER_DITH5_MODE_SPEC>;
#[doc = "Period"]
pub mod per_dith5_mode;
#[doc = "PER_DITH6_MODE (rw) register accessor: Period\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`per_dith6_mode::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`per_dith6_mode::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@per_dith6_mode`]
module"]
pub type PER_DITH6_MODE = crate::Reg<per_dith6_mode::PER_DITH6_MODE_SPEC>;
#[doc = "Period"]
pub mod per_dith6_mode;
#[doc = "CC (rw) register accessor: Compare and Capture\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cc`]
module"]
pub type CC = crate::Reg<cc::CC_SPEC>;
#[doc = "Compare and Capture"]
pub mod cc;
#[doc = "CC_DITH4_MODE (rw) register accessor: Compare and Capture\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cc_dith4_mode::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cc_dith4_mode::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cc_dith4_mode`]
module"]
pub type CC_DITH4_MODE = crate::Reg<cc_dith4_mode::CC_DITH4_MODE_SPEC>;
#[doc = "Compare and Capture"]
pub mod cc_dith4_mode;
#[doc = "CC_DITH5_MODE (rw) register accessor: Compare and Capture\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cc_dith5_mode::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cc_dith5_mode::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cc_dith5_mode`]
module"]
pub type CC_DITH5_MODE = crate::Reg<cc_dith5_mode::CC_DITH5_MODE_SPEC>;
#[doc = "Compare and Capture"]
pub mod cc_dith5_mode;
#[doc = "CC_DITH6_MODE (rw) register accessor: Compare and Capture\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cc_dith6_mode::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cc_dith6_mode::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cc_dith6_mode`]
module"]
pub type CC_DITH6_MODE = crate::Reg<cc_dith6_mode::CC_DITH6_MODE_SPEC>;
#[doc = "Compare and Capture"]
pub mod cc_dith6_mode;
#[doc = "PATTBUF (rw) register accessor: Pattern Buffer\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pattbuf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pattbuf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pattbuf`]
module"]
pub type PATTBUF = crate::Reg<pattbuf::PATTBUF_SPEC>;
#[doc = "Pattern Buffer"]
pub mod pattbuf;
#[doc = "PERBUF (rw) register accessor: Period Buffer\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`perbuf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`perbuf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@perbuf`]
module"]
pub type PERBUF = crate::Reg<perbuf::PERBUF_SPEC>;
#[doc = "Period Buffer"]
pub mod perbuf;
#[doc = "PERBUF_DITH4_MODE (rw) register accessor: Period Buffer\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`perbuf_dith4_mode::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`perbuf_dith4_mode::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@perbuf_dith4_mode`]
module"]
pub type PERBUF_DITH4_MODE = crate::Reg<perbuf_dith4_mode::PERBUF_DITH4_MODE_SPEC>;
#[doc = "Period Buffer"]
pub mod perbuf_dith4_mode;
#[doc = "PERBUF_DITH5_MODE (rw) register accessor: Period Buffer\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`perbuf_dith5_mode::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`perbuf_dith5_mode::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@perbuf_dith5_mode`]
module"]
pub type PERBUF_DITH5_MODE = crate::Reg<perbuf_dith5_mode::PERBUF_DITH5_MODE_SPEC>;
#[doc = "Period Buffer"]
pub mod perbuf_dith5_mode;
#[doc = "PERBUF_DITH6_MODE (rw) register accessor: Period Buffer\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`perbuf_dith6_mode::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`perbuf_dith6_mode::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@perbuf_dith6_mode`]
module"]
pub type PERBUF_DITH6_MODE = crate::Reg<perbuf_dith6_mode::PERBUF_DITH6_MODE_SPEC>;
#[doc = "Period Buffer"]
pub mod perbuf_dith6_mode;
#[doc = "CCBUF (rw) register accessor: Compare and Capture Buffer\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccbuf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccbuf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccbuf`]
module"]
pub type CCBUF = crate::Reg<ccbuf::CCBUF_SPEC>;
#[doc = "Compare and Capture Buffer"]
pub mod ccbuf;
#[doc = "CCBUF_DITH4_MODE (rw) register accessor: Compare and Capture Buffer\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccbuf_dith4_mode::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccbuf_dith4_mode::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccbuf_dith4_mode`]
module"]
pub type CCBUF_DITH4_MODE = crate::Reg<ccbuf_dith4_mode::CCBUF_DITH4_MODE_SPEC>;
#[doc = "Compare and Capture Buffer"]
pub mod ccbuf_dith4_mode;
#[doc = "CCBUF_DITH5_MODE (rw) register accessor: Compare and Capture Buffer\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccbuf_dith5_mode::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccbuf_dith5_mode::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccbuf_dith5_mode`]
module"]
pub type CCBUF_DITH5_MODE = crate::Reg<ccbuf_dith5_mode::CCBUF_DITH5_MODE_SPEC>;
#[doc = "Compare and Capture Buffer"]
pub mod ccbuf_dith5_mode;
#[doc = "CCBUF_DITH6_MODE (rw) register accessor: Compare and Capture Buffer\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccbuf_dith6_mode::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccbuf_dith6_mode::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccbuf_dith6_mode`]
module"]
pub type CCBUF_DITH6_MODE = crate::Reg<ccbuf_dith6_mode::CCBUF_DITH6_MODE_SPEC>;
#[doc = "Compare and Capture Buffer"]
pub mod ccbuf_dith6_mode;
