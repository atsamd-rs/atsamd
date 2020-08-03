#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control A"]
    pub ctrla: CTRLA,
    #[doc = "0x01 - Sleep Configuration"]
    pub sleepcfg: SLEEPCFG,
    _reserved2: [u8; 2usize],
    #[doc = "0x04 - Interrupt Enable Clear"]
    pub intenclr: INTENCLR,
    #[doc = "0x05 - Interrupt Enable Set"]
    pub intenset: INTENSET,
    #[doc = "0x06 - Interrupt Flag Status and Clear"]
    pub intflag: INTFLAG,
    _reserved5: [u8; 1usize],
    #[doc = "0x08 - Standby Configuration"]
    pub stdbycfg: STDBYCFG,
    #[doc = "0x09 - Hibernate Configuration"]
    pub hibcfg: HIBCFG,
    #[doc = "0x0a - Backup Configuration"]
    pub bkupcfg: BKUPCFG,
    _reserved8: [u8; 7usize],
    #[doc = "0x12 - Power Switch Acknowledge Delay"]
    pub pwsakdly: PWSAKDLY,
}
#[doc = "Control A\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrla](ctrla) module"]
pub type CTRLA = crate::Reg<u8, _CTRLA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRLA;
#[doc = "`read()` method returns [ctrla::R](ctrla::R) reader structure"]
impl crate::Readable for CTRLA {}
#[doc = "`write(|w| ..)` method takes [ctrla::W](ctrla::W) writer structure"]
impl crate::Writable for CTRLA {}
#[doc = "Control A"]
pub mod ctrla;
#[doc = "Sleep Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sleepcfg](sleepcfg) module"]
pub type SLEEPCFG = crate::Reg<u8, _SLEEPCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SLEEPCFG;
#[doc = "`read()` method returns [sleepcfg::R](sleepcfg::R) reader structure"]
impl crate::Readable for SLEEPCFG {}
#[doc = "`write(|w| ..)` method takes [sleepcfg::W](sleepcfg::W) writer structure"]
impl crate::Writable for SLEEPCFG {}
#[doc = "Sleep Configuration"]
pub mod sleepcfg;
#[doc = "Interrupt Enable Clear\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intenclr](intenclr) module"]
pub type INTENCLR = crate::Reg<u8, _INTENCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTENCLR;
#[doc = "`read()` method returns [intenclr::R](intenclr::R) reader structure"]
impl crate::Readable for INTENCLR {}
#[doc = "`write(|w| ..)` method takes [intenclr::W](intenclr::W) writer structure"]
impl crate::Writable for INTENCLR {}
#[doc = "Interrupt Enable Clear"]
pub mod intenclr;
#[doc = "Interrupt Enable Set\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intenset](intenset) module"]
pub type INTENSET = crate::Reg<u8, _INTENSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTENSET;
#[doc = "`read()` method returns [intenset::R](intenset::R) reader structure"]
impl crate::Readable for INTENSET {}
#[doc = "`write(|w| ..)` method takes [intenset::W](intenset::W) writer structure"]
impl crate::Writable for INTENSET {}
#[doc = "Interrupt Enable Set"]
pub mod intenset;
#[doc = "Interrupt Flag Status and Clear\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intflag](intflag) module"]
pub type INTFLAG = crate::Reg<u8, _INTFLAG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTFLAG;
#[doc = "`read()` method returns [intflag::R](intflag::R) reader structure"]
impl crate::Readable for INTFLAG {}
#[doc = "`write(|w| ..)` method takes [intflag::W](intflag::W) writer structure"]
impl crate::Writable for INTFLAG {}
#[doc = "Interrupt Flag Status and Clear"]
pub mod intflag;
#[doc = "Standby Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stdbycfg](stdbycfg) module"]
pub type STDBYCFG = crate::Reg<u8, _STDBYCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STDBYCFG;
#[doc = "`read()` method returns [stdbycfg::R](stdbycfg::R) reader structure"]
impl crate::Readable for STDBYCFG {}
#[doc = "`write(|w| ..)` method takes [stdbycfg::W](stdbycfg::W) writer structure"]
impl crate::Writable for STDBYCFG {}
#[doc = "Standby Configuration"]
pub mod stdbycfg;
#[doc = "Hibernate Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hibcfg](hibcfg) module"]
pub type HIBCFG = crate::Reg<u8, _HIBCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HIBCFG;
#[doc = "`read()` method returns [hibcfg::R](hibcfg::R) reader structure"]
impl crate::Readable for HIBCFG {}
#[doc = "`write(|w| ..)` method takes [hibcfg::W](hibcfg::W) writer structure"]
impl crate::Writable for HIBCFG {}
#[doc = "Hibernate Configuration"]
pub mod hibcfg;
#[doc = "Backup Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bkupcfg](bkupcfg) module"]
pub type BKUPCFG = crate::Reg<u8, _BKUPCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BKUPCFG;
#[doc = "`read()` method returns [bkupcfg::R](bkupcfg::R) reader structure"]
impl crate::Readable for BKUPCFG {}
#[doc = "`write(|w| ..)` method takes [bkupcfg::W](bkupcfg::W) writer structure"]
impl crate::Writable for BKUPCFG {}
#[doc = "Backup Configuration"]
pub mod bkupcfg;
#[doc = "Power Switch Acknowledge Delay\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwsakdly](pwsakdly) module"]
pub type PWSAKDLY = crate::Reg<u8, _PWSAKDLY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PWSAKDLY;
#[doc = "`read()` method returns [pwsakdly::R](pwsakdly::R) reader structure"]
impl crate::Readable for PWSAKDLY {}
#[doc = "`write(|w| ..)` method takes [pwsakdly::W](pwsakdly::W) writer structure"]
impl crate::Writable for PWSAKDLY {}
#[doc = "Power Switch Acknowledge Delay"]
pub mod pwsakdly;
