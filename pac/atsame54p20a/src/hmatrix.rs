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
