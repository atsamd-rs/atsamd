#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Cache Type Register"]
    pub type_: TYPE,
    #[doc = "0x04 - Cache Configuration Register"]
    pub cfg: CFG,
    #[doc = "0x08 - Cache Control Register"]
    pub ctrl: CTRL,
    #[doc = "0x0c - Cache Status Register"]
    pub sr: SR,
    #[doc = "0x10 - Cache Lock per Way Register"]
    pub lckway: LCKWAY,
    _reserved5: [u8; 12usize],
    #[doc = "0x20 - Cache Maintenance Register 0"]
    pub maint0: MAINT0,
    #[doc = "0x24 - Cache Maintenance Register 1"]
    pub maint1: MAINT1,
    #[doc = "0x28 - Cache Monitor Configuration Register"]
    pub mcfg: MCFG,
    #[doc = "0x2c - Cache Monitor Enable Register"]
    pub men: MEN,
    #[doc = "0x30 - Cache Monitor Control Register"]
    pub mctrl: MCTRL,
    #[doc = "0x34 - Cache Monitor Status Register"]
    pub msr: MSR,
}
#[doc = "Cache Type Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [type_](type_) module"]
pub type TYPE = crate::Reg<u32, _TYPE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TYPE;
#[doc = "`read()` method returns [type_::R](type_::R) reader structure"]
impl crate::Readable for TYPE {}
#[doc = "Cache Type Register"]
pub mod type_;
#[doc = "Cache Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub type CFG = crate::Reg<u32, _CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CFG;
#[doc = "`read()` method returns [cfg::R](cfg::R) reader structure"]
impl crate::Readable for CFG {}
#[doc = "`write(|w| ..)` method takes [cfg::W](cfg::W) writer structure"]
impl crate::Writable for CFG {}
#[doc = "Cache Configuration Register"]
pub mod cfg;
#[doc = "Cache Control Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](ctrl) module"]
pub type CTRL = crate::Reg<u32, _CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRL;
#[doc = "`write(|w| ..)` method takes [ctrl::W](ctrl::W) writer structure"]
impl crate::Writable for CTRL {}
#[doc = "Cache Control Register"]
pub mod ctrl;
#[doc = "Cache Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr](sr) module"]
pub type SR = crate::Reg<u32, _SR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SR;
#[doc = "`read()` method returns [sr::R](sr::R) reader structure"]
impl crate::Readable for SR {}
#[doc = "Cache Status Register"]
pub mod sr;
#[doc = "Cache Lock per Way Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lckway](lckway) module"]
pub type LCKWAY = crate::Reg<u32, _LCKWAY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LCKWAY;
#[doc = "`read()` method returns [lckway::R](lckway::R) reader structure"]
impl crate::Readable for LCKWAY {}
#[doc = "`write(|w| ..)` method takes [lckway::W](lckway::W) writer structure"]
impl crate::Writable for LCKWAY {}
#[doc = "Cache Lock per Way Register"]
pub mod lckway;
#[doc = "Cache Maintenance Register 0\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [maint0](maint0) module"]
pub type MAINT0 = crate::Reg<u32, _MAINT0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MAINT0;
#[doc = "`write(|w| ..)` method takes [maint0::W](maint0::W) writer structure"]
impl crate::Writable for MAINT0 {}
#[doc = "Cache Maintenance Register 0"]
pub mod maint0;
#[doc = "Cache Maintenance Register 1\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [maint1](maint1) module"]
pub type MAINT1 = crate::Reg<u32, _MAINT1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MAINT1;
#[doc = "`write(|w| ..)` method takes [maint1::W](maint1::W) writer structure"]
impl crate::Writable for MAINT1 {}
#[doc = "Cache Maintenance Register 1"]
pub mod maint1;
#[doc = "Cache Monitor Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcfg](mcfg) module"]
pub type MCFG = crate::Reg<u32, _MCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCFG;
#[doc = "`read()` method returns [mcfg::R](mcfg::R) reader structure"]
impl crate::Readable for MCFG {}
#[doc = "`write(|w| ..)` method takes [mcfg::W](mcfg::W) writer structure"]
impl crate::Writable for MCFG {}
#[doc = "Cache Monitor Configuration Register"]
pub mod mcfg;
#[doc = "Cache Monitor Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [men](men) module"]
pub type MEN = crate::Reg<u32, _MEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MEN;
#[doc = "`read()` method returns [men::R](men::R) reader structure"]
impl crate::Readable for MEN {}
#[doc = "`write(|w| ..)` method takes [men::W](men::W) writer structure"]
impl crate::Writable for MEN {}
#[doc = "Cache Monitor Enable Register"]
pub mod men;
#[doc = "Cache Monitor Control Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mctrl](mctrl) module"]
pub type MCTRL = crate::Reg<u32, _MCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCTRL;
#[doc = "`write(|w| ..)` method takes [mctrl::W](mctrl::W) writer structure"]
impl crate::Writable for MCTRL {}
#[doc = "Cache Monitor Control Register"]
pub mod mctrl;
#[doc = "Cache Monitor Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [msr](msr) module"]
pub type MSR = crate::Reg<u32, _MSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MSR;
#[doc = "`read()` method returns [msr::R](msr::R) reader structure"]
impl crate::Readable for MSR {}
#[doc = "Cache Monitor Status Register"]
pub mod msr;
