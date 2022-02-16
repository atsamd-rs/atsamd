#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Mode Register"]
    pub rtt_mr: crate::Reg<rtt_mr::RTT_MR_SPEC>,
    #[doc = "0x04 - Alarm Register"]
    pub rtt_ar: crate::Reg<rtt_ar::RTT_AR_SPEC>,
    #[doc = "0x08 - Value Register"]
    pub rtt_vr: crate::Reg<rtt_vr::RTT_VR_SPEC>,
    #[doc = "0x0c - Status Register"]
    pub rtt_sr: crate::Reg<rtt_sr::RTT_SR_SPEC>,
}
#[doc = "RTT_MR register accessor: an alias for `Reg<RTT_MR_SPEC>`"]
pub type RTT_MR = crate::Reg<rtt_mr::RTT_MR_SPEC>;
#[doc = "Mode Register"]
pub mod rtt_mr;
#[doc = "RTT_AR register accessor: an alias for `Reg<RTT_AR_SPEC>`"]
pub type RTT_AR = crate::Reg<rtt_ar::RTT_AR_SPEC>;
#[doc = "Alarm Register"]
pub mod rtt_ar;
#[doc = "RTT_VR register accessor: an alias for `Reg<RTT_VR_SPEC>`"]
pub type RTT_VR = crate::Reg<rtt_vr::RTT_VR_SPEC>;
#[doc = "Value Register"]
pub mod rtt_vr;
#[doc = "RTT_SR register accessor: an alias for `Reg<RTT_SR_SPEC>`"]
pub type RTT_SR = crate::Reg<rtt_sr::RTT_SR_SPEC>;
#[doc = "Status Register"]
pub mod rtt_sr;
