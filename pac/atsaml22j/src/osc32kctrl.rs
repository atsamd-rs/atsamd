#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Interrupt Enable Clear"]
    pub intenclr: INTENCLR,
    #[doc = "0x04 - Interrupt Enable Set"]
    pub intenset: INTENSET,
    #[doc = "0x08 - Interrupt Flag Status and Clear"]
    pub intflag: INTFLAG,
    #[doc = "0x0c - Power and Clocks Status"]
    pub status: STATUS,
    #[doc = "0x10 - RTC Clock Selection"]
    pub rtcctrl: RTCCTRL,
    #[doc = "0x11 - SLCD Clock Selection"]
    pub slcdctrl: SLCDCTRL,
    _reserved6: [u8; 0x02],
    #[doc = "0x14 - 32kHz External Crystal Oscillator (XOSC32K) Control"]
    pub xosc32k: XOSC32K,
    #[doc = "0x16 - Clock Failure Detector Control"]
    pub cfdctrl: CFDCTRL,
    #[doc = "0x17 - Event Control"]
    pub evctrl: EVCTRL,
    _reserved9: [u8; 0x04],
    #[doc = "0x1c - 32kHz Ultra Low Power Internal Oscillator (OSCULP32K) Control"]
    pub osculp32k: OSCULP32K,
}
#[doc = "INTENCLR (rw) register accessor: an alias for `Reg<INTENCLR_SPEC>`"]
pub type INTENCLR = crate::Reg<intenclr::INTENCLR_SPEC>;
#[doc = "Interrupt Enable Clear"]
pub mod intenclr;
#[doc = "INTENSET (rw) register accessor: an alias for `Reg<INTENSET_SPEC>`"]
pub type INTENSET = crate::Reg<intenset::INTENSET_SPEC>;
#[doc = "Interrupt Enable Set"]
pub mod intenset;
#[doc = "INTFLAG (rw) register accessor: an alias for `Reg<INTFLAG_SPEC>`"]
pub type INTFLAG = crate::Reg<intflag::INTFLAG_SPEC>;
#[doc = "Interrupt Flag Status and Clear"]
pub mod intflag;
#[doc = "STATUS (r) register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "Power and Clocks Status"]
pub mod status;
#[doc = "RTCCTRL (rw) register accessor: an alias for `Reg<RTCCTRL_SPEC>`"]
pub type RTCCTRL = crate::Reg<rtcctrl::RTCCTRL_SPEC>;
#[doc = "RTC Clock Selection"]
pub mod rtcctrl;
#[doc = "SLCDCTRL (rw) register accessor: an alias for `Reg<SLCDCTRL_SPEC>`"]
pub type SLCDCTRL = crate::Reg<slcdctrl::SLCDCTRL_SPEC>;
#[doc = "SLCD Clock Selection"]
pub mod slcdctrl;
#[doc = "XOSC32K (rw) register accessor: an alias for `Reg<XOSC32K_SPEC>`"]
pub type XOSC32K = crate::Reg<xosc32k::XOSC32K_SPEC>;
#[doc = "32kHz External Crystal Oscillator (XOSC32K) Control"]
pub mod xosc32k;
#[doc = "CFDCTRL (rw) register accessor: an alias for `Reg<CFDCTRL_SPEC>`"]
pub type CFDCTRL = crate::Reg<cfdctrl::CFDCTRL_SPEC>;
#[doc = "Clock Failure Detector Control"]
pub mod cfdctrl;
#[doc = "EVCTRL (rw) register accessor: an alias for `Reg<EVCTRL_SPEC>`"]
pub type EVCTRL = crate::Reg<evctrl::EVCTRL_SPEC>;
#[doc = "Event Control"]
pub mod evctrl;
#[doc = "OSCULP32K (rw) register accessor: an alias for `Reg<OSCULP32K_SPEC>`"]
pub type OSCULP32K = crate::Reg<osculp32k::OSCULP32K_SPEC>;
#[doc = "32kHz Ultra Low Power Internal Oscillator (OSCULP32K) Control"]
pub mod osculp32k;
