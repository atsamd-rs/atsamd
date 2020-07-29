#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 128usize],
    #[doc = "0x80 - Priority A for Slave"]
    pub pras0: PRAS,
    #[doc = "0x84 - Priority B for Slave"]
    pub prbs0: PRBS,
    #[doc = "0x88 - Priority A for Slave"]
    pub pras1: PRAS,
    #[doc = "0x8c - Priority B for Slave"]
    pub prbs1: PRBS,
    #[doc = "0x90 - Priority A for Slave"]
    pub pras2: PRAS,
    #[doc = "0x94 - Priority B for Slave"]
    pub prbs2: PRBS,
    #[doc = "0x98 - Priority A for Slave"]
    pub pras3: PRAS,
    #[doc = "0x9c - Priority B for Slave"]
    pub prbs3: PRBS,
    #[doc = "0xa0 - Priority A for Slave"]
    pub pras4: PRAS,
    #[doc = "0xa4 - Priority B for Slave"]
    pub prbs4: PRBS,
    #[doc = "0xa8 - Priority A for Slave"]
    pub pras5: PRAS,
    #[doc = "0xac - Priority B for Slave"]
    pub prbs5: PRBS,
    #[doc = "0xb0 - Priority A for Slave"]
    pub pras6: PRAS,
    #[doc = "0xb4 - Priority B for Slave"]
    pub prbs6: PRBS,
    #[doc = "0xb8 - Priority A for Slave"]
    pub pras7: PRAS,
    #[doc = "0xbc - Priority B for Slave"]
    pub prbs7: PRBS,
    #[doc = "0xc0 - Priority A for Slave"]
    pub pras8: PRAS,
    #[doc = "0xc4 - Priority B for Slave"]
    pub prbs8: PRBS,
    #[doc = "0xc8 - Priority A for Slave"]
    pub pras9: PRAS,
    #[doc = "0xcc - Priority B for Slave"]
    pub prbs9: PRBS,
    #[doc = "0xd0 - Priority A for Slave"]
    pub pras10: PRAS,
    #[doc = "0xd4 - Priority B for Slave"]
    pub prbs10: PRBS,
    #[doc = "0xd8 - Priority A for Slave"]
    pub pras11: PRAS,
    #[doc = "0xdc - Priority B for Slave"]
    pub prbs11: PRBS,
    #[doc = "0xe0 - Priority A for Slave"]
    pub pras12: PRAS,
    #[doc = "0xe4 - Priority B for Slave"]
    pub prbs12: PRBS,
    #[doc = "0xe8 - Priority A for Slave"]
    pub pras13: PRAS,
    #[doc = "0xec - Priority B for Slave"]
    pub prbs13: PRBS,
    #[doc = "0xf0 - Priority A for Slave"]
    pub pras14: PRAS,
    #[doc = "0xf4 - Priority B for Slave"]
    pub prbs14: PRBS,
    #[doc = "0xf8 - Priority A for Slave"]
    pub pras15: PRAS,
    #[doc = "0xfc - Priority B for Slave"]
    pub prbs15: PRBS,
    _reserved32: [u8; 16usize],
    #[doc = "0x110 - Special Function"]
    pub sfr: [SFR; 16],
}
#[doc = "Priority A for Slave\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pras](pras) module"]
pub type PRAS = crate::Reg<u32, _PRAS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRAS;
#[doc = "`read()` method returns [pras::R](pras::R) reader structure"]
impl crate::Readable for PRAS {}
#[doc = "`write(|w| ..)` method takes [pras::W](pras::W) writer structure"]
impl crate::Writable for PRAS {}
#[doc = "Priority A for Slave"]
pub mod pras;
#[doc = "Priority B for Slave\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prbs](prbs) module"]
pub type PRBS = crate::Reg<u32, _PRBS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRBS;
#[doc = "`read()` method returns [prbs::R](prbs::R) reader structure"]
impl crate::Readable for PRBS {}
#[doc = "`write(|w| ..)` method takes [prbs::W](prbs::W) writer structure"]
impl crate::Writable for PRBS {}
#[doc = "Priority B for Slave"]
pub mod prbs;
#[doc = "Special Function\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sfr](sfr) module"]
pub type SFR = crate::Reg<u32, _SFR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SFR;
#[doc = "`read()` method returns [sfr::R](sfr::R) reader structure"]
impl crate::Readable for SFR {}
#[doc = "`write(|w| ..)` method takes [sfr::W](sfr::W) writer structure"]
impl crate::Writable for SFR {}
#[doc = "Special Function"]
pub mod sfr;
