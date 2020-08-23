#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 128usize],
    #[doc = "0x80 - PRS\\[%s\\]"]
    pub prs: [PRS; 16],
}
#[doc = r"Register block"]
#[repr(C)]
pub struct PRS {
    #[doc = "0x00 - Priority A for Slave"]
    pub pras: self::prs::PRAS,
    #[doc = "0x04 - Priority B for Slave"]
    pub prbs: self::prs::PRBS,
}
#[doc = r"Register block"]
#[doc = "PRS\\[%s\\]"]
pub mod prs;
