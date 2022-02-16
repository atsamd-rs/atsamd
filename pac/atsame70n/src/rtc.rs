#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub rtc_cr: crate::Reg<rtc_cr::RTC_CR_SPEC>,
    #[doc = "0x04 - Mode Register"]
    pub rtc_mr: crate::Reg<rtc_mr::RTC_MR_SPEC>,
    #[doc = "0x08 - Time Register"]
    pub rtc_timr: crate::Reg<rtc_timr::RTC_TIMR_SPEC>,
    #[doc = "0x0c - Calendar Register"]
    pub rtc_calr: crate::Reg<rtc_calr::RTC_CALR_SPEC>,
    #[doc = "0x10 - Time Alarm Register"]
    pub rtc_timalr: crate::Reg<rtc_timalr::RTC_TIMALR_SPEC>,
    #[doc = "0x14 - Calendar Alarm Register"]
    pub rtc_calalr: crate::Reg<rtc_calalr::RTC_CALALR_SPEC>,
    #[doc = "0x18 - Status Register"]
    pub rtc_sr: crate::Reg<rtc_sr::RTC_SR_SPEC>,
    #[doc = "0x1c - Status Clear Command Register"]
    pub rtc_sccr: crate::Reg<rtc_sccr::RTC_SCCR_SPEC>,
    #[doc = "0x20 - Interrupt Enable Register"]
    pub rtc_ier: crate::Reg<rtc_ier::RTC_IER_SPEC>,
    #[doc = "0x24 - Interrupt Disable Register"]
    pub rtc_idr: crate::Reg<rtc_idr::RTC_IDR_SPEC>,
    #[doc = "0x28 - Interrupt Mask Register"]
    pub rtc_imr: crate::Reg<rtc_imr::RTC_IMR_SPEC>,
    #[doc = "0x2c - Valid Entry Register"]
    pub rtc_ver: crate::Reg<rtc_ver::RTC_VER_SPEC>,
}
#[doc = "RTC_CR register accessor: an alias for `Reg<RTC_CR_SPEC>`"]
pub type RTC_CR = crate::Reg<rtc_cr::RTC_CR_SPEC>;
#[doc = "Control Register"]
pub mod rtc_cr;
#[doc = "RTC_MR register accessor: an alias for `Reg<RTC_MR_SPEC>`"]
pub type RTC_MR = crate::Reg<rtc_mr::RTC_MR_SPEC>;
#[doc = "Mode Register"]
pub mod rtc_mr;
#[doc = "RTC_TIMR register accessor: an alias for `Reg<RTC_TIMR_SPEC>`"]
pub type RTC_TIMR = crate::Reg<rtc_timr::RTC_TIMR_SPEC>;
#[doc = "Time Register"]
pub mod rtc_timr;
#[doc = "RTC_CALR register accessor: an alias for `Reg<RTC_CALR_SPEC>`"]
pub type RTC_CALR = crate::Reg<rtc_calr::RTC_CALR_SPEC>;
#[doc = "Calendar Register"]
pub mod rtc_calr;
#[doc = "RTC_TIMALR register accessor: an alias for `Reg<RTC_TIMALR_SPEC>`"]
pub type RTC_TIMALR = crate::Reg<rtc_timalr::RTC_TIMALR_SPEC>;
#[doc = "Time Alarm Register"]
pub mod rtc_timalr;
#[doc = "RTC_CALALR register accessor: an alias for `Reg<RTC_CALALR_SPEC>`"]
pub type RTC_CALALR = crate::Reg<rtc_calalr::RTC_CALALR_SPEC>;
#[doc = "Calendar Alarm Register"]
pub mod rtc_calalr;
#[doc = "RTC_SR register accessor: an alias for `Reg<RTC_SR_SPEC>`"]
pub type RTC_SR = crate::Reg<rtc_sr::RTC_SR_SPEC>;
#[doc = "Status Register"]
pub mod rtc_sr;
#[doc = "RTC_SCCR register accessor: an alias for `Reg<RTC_SCCR_SPEC>`"]
pub type RTC_SCCR = crate::Reg<rtc_sccr::RTC_SCCR_SPEC>;
#[doc = "Status Clear Command Register"]
pub mod rtc_sccr;
#[doc = "RTC_IER register accessor: an alias for `Reg<RTC_IER_SPEC>`"]
pub type RTC_IER = crate::Reg<rtc_ier::RTC_IER_SPEC>;
#[doc = "Interrupt Enable Register"]
pub mod rtc_ier;
#[doc = "RTC_IDR register accessor: an alias for `Reg<RTC_IDR_SPEC>`"]
pub type RTC_IDR = crate::Reg<rtc_idr::RTC_IDR_SPEC>;
#[doc = "Interrupt Disable Register"]
pub mod rtc_idr;
#[doc = "RTC_IMR register accessor: an alias for `Reg<RTC_IMR_SPEC>`"]
pub type RTC_IMR = crate::Reg<rtc_imr::RTC_IMR_SPEC>;
#[doc = "Interrupt Mask Register"]
pub mod rtc_imr;
#[doc = "RTC_VER register accessor: an alias for `Reg<RTC_VER_SPEC>`"]
pub type RTC_VER = crate::Reg<rtc_ver::RTC_VER_SPEC>;
#[doc = "Valid Entry Register"]
pub mod rtc_ver;
