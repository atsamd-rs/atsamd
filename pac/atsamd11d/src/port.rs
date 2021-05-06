#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Data Direction"]
    pub dir0: DIR,
    #[doc = "0x04 - Data Direction Clear"]
    pub dirclr0: DIRCLR,
    #[doc = "0x08 - Data Direction Set"]
    pub dirset0: DIRSET,
    #[doc = "0x0c - Data Direction Toggle"]
    pub dirtgl0: DIRTGL,
    #[doc = "0x10 - Data Output Value"]
    pub out0: OUT,
    #[doc = "0x14 - Data Output Value Clear"]
    pub outclr0: OUTCLR,
    #[doc = "0x18 - Data Output Value Set"]
    pub outset0: OUTSET,
    #[doc = "0x1c - Data Output Value Toggle"]
    pub outtgl0: OUTTGL,
    #[doc = "0x20 - Data Input Value"]
    pub in0: IN,
    #[doc = "0x24 - Control"]
    pub ctrl0: CTRL,
    #[doc = "0x28 - Write Configuration"]
    pub wrconfig0: WRCONFIG,
    _reserved11: [u8; 4usize],
    #[doc = "0x30 - Peripheral Multiplexing n - Group 0"]
    pub pmux0_: [PMUX0_; 16],
    #[doc = "0x40 - Pin Configuration n - Group 0"]
    pub pincfg0_: [PINCFG0_; 32],
}
#[doc = "Data Direction\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dir](dir) module"]
pub type DIR = crate::Reg<u32, _DIR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIR;
#[doc = "`read()` method returns [dir::R](dir::R) reader structure"]
impl crate::Readable for DIR {}
#[doc = "`write(|w| ..)` method takes [dir::W](dir::W) writer structure"]
impl crate::Writable for DIR {}
#[doc = "Data Direction"]
pub mod dir;
#[doc = "Data Direction Clear\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dirclr](dirclr) module"]
pub type DIRCLR = crate::Reg<u32, _DIRCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIRCLR;
#[doc = "`read()` method returns [dirclr::R](dirclr::R) reader structure"]
impl crate::Readable for DIRCLR {}
#[doc = "`write(|w| ..)` method takes [dirclr::W](dirclr::W) writer structure"]
impl crate::Writable for DIRCLR {}
#[doc = "Data Direction Clear"]
pub mod dirclr;
#[doc = "Data Direction Set\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dirset](dirset) module"]
pub type DIRSET = crate::Reg<u32, _DIRSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIRSET;
#[doc = "`read()` method returns [dirset::R](dirset::R) reader structure"]
impl crate::Readable for DIRSET {}
#[doc = "`write(|w| ..)` method takes [dirset::W](dirset::W) writer structure"]
impl crate::Writable for DIRSET {}
#[doc = "Data Direction Set"]
pub mod dirset;
#[doc = "Data Direction Toggle\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dirtgl](dirtgl) module"]
pub type DIRTGL = crate::Reg<u32, _DIRTGL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIRTGL;
#[doc = "`read()` method returns [dirtgl::R](dirtgl::R) reader structure"]
impl crate::Readable for DIRTGL {}
#[doc = "`write(|w| ..)` method takes [dirtgl::W](dirtgl::W) writer structure"]
impl crate::Writable for DIRTGL {}
#[doc = "Data Direction Toggle"]
pub mod dirtgl;
#[doc = "Data Output Value\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [out](out) module"]
pub type OUT = crate::Reg<u32, _OUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OUT;
#[doc = "`read()` method returns [out::R](out::R) reader structure"]
impl crate::Readable for OUT {}
#[doc = "`write(|w| ..)` method takes [out::W](out::W) writer structure"]
impl crate::Writable for OUT {}
#[doc = "Data Output Value"]
pub mod out;
#[doc = "Data Output Value Clear\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [outclr](outclr) module"]
pub type OUTCLR = crate::Reg<u32, _OUTCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OUTCLR;
#[doc = "`read()` method returns [outclr::R](outclr::R) reader structure"]
impl crate::Readable for OUTCLR {}
#[doc = "`write(|w| ..)` method takes [outclr::W](outclr::W) writer structure"]
impl crate::Writable for OUTCLR {}
#[doc = "Data Output Value Clear"]
pub mod outclr;
#[doc = "Data Output Value Set\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [outset](outset) module"]
pub type OUTSET = crate::Reg<u32, _OUTSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OUTSET;
#[doc = "`read()` method returns [outset::R](outset::R) reader structure"]
impl crate::Readable for OUTSET {}
#[doc = "`write(|w| ..)` method takes [outset::W](outset::W) writer structure"]
impl crate::Writable for OUTSET {}
#[doc = "Data Output Value Set"]
pub mod outset;
#[doc = "Data Output Value Toggle\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [outtgl](outtgl) module"]
pub type OUTTGL = crate::Reg<u32, _OUTTGL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OUTTGL;
#[doc = "`read()` method returns [outtgl::R](outtgl::R) reader structure"]
impl crate::Readable for OUTTGL {}
#[doc = "`write(|w| ..)` method takes [outtgl::W](outtgl::W) writer structure"]
impl crate::Writable for OUTTGL {}
#[doc = "Data Output Value Toggle"]
pub mod outtgl;
#[doc = "Data Input Value\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [in_](in_) module"]
pub type IN = crate::Reg<u32, _IN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IN;
#[doc = "`read()` method returns [in_::R](in_::R) reader structure"]
impl crate::Readable for IN {}
#[doc = "Data Input Value"]
pub mod in_;
#[doc = "Control\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](ctrl) module"]
pub type CTRL = crate::Reg<u32, _CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRL;
#[doc = "`write(|w| ..)` method takes [ctrl::W](ctrl::W) writer structure"]
impl crate::Writable for CTRL {}
#[doc = "Control"]
pub mod ctrl;
#[doc = "Write Configuration\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wrconfig](wrconfig) module"]
pub type WRCONFIG = crate::Reg<u32, _WRCONFIG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WRCONFIG;
#[doc = "`write(|w| ..)` method takes [wrconfig::W](wrconfig::W) writer structure"]
impl crate::Writable for WRCONFIG {}
#[doc = "Write Configuration"]
pub mod wrconfig;
#[doc = "Peripheral Multiplexing n - Group 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmux0_](pmux0_) module"]
pub type PMUX0_ = crate::Reg<u8, _PMUX0_>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PMUX0_;
#[doc = "`read()` method returns [pmux0_::R](pmux0_::R) reader structure"]
impl crate::Readable for PMUX0_ {}
#[doc = "`write(|w| ..)` method takes [pmux0_::W](pmux0_::W) writer structure"]
impl crate::Writable for PMUX0_ {}
#[doc = "Peripheral Multiplexing n - Group 0"]
pub mod pmux0_;
#[doc = "Pin Configuration n - Group 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pincfg0_](pincfg0_) module"]
pub type PINCFG0_ = crate::Reg<u8, _PINCFG0_>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PINCFG0_;
#[doc = "`read()` method returns [pincfg0_::R](pincfg0_::R) reader structure"]
impl crate::Readable for PINCFG0_ {}
#[doc = "`write(|w| ..)` method takes [pincfg0_::W](pincfg0_::W) writer structure"]
impl crate::Writable for PINCFG0_ {}
#[doc = "Pin Configuration n - Group 0"]
pub mod pincfg0_;
