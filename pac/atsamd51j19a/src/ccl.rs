#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control"]
    pub ctrl: CTRL,
    _reserved1: [u8; 3usize],
    #[doc = "0x04 - SEQ Control x"]
    pub seqctrl: [SEQCTRL; 2],
    _reserved2: [u8; 2usize],
    #[doc = "0x08 - LUT Control x"]
    pub lutctrl: [LUTCTRL; 4],
}
#[doc = "Control"]
pub struct CTRL {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Control"]
pub mod ctrl;
#[doc = "SEQ Control x"]
pub struct SEQCTRL {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "SEQ Control x"]
pub mod seqctrl;
#[doc = "LUT Control x"]
pub struct LUTCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LUT Control x"]
pub mod lutctrl;
