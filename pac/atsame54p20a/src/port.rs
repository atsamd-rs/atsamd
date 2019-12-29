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
    #[doc = "0x2c - Event Input Control"]
    pub evctrl0: EVCTRL,
    #[doc = "0x30 - Peripheral Multiplexing - Group 0"]
    pub pmux0_: [PMUX0_; 16],
    #[doc = "0x40 - Pin Configuration - Group 0"]
    pub pincfg0_: [PINCFG0_; 32],
    _reserved14: [u8; 32usize],
    #[doc = "0x80 - Data Direction"]
    pub dir1: DIR,
    #[doc = "0x84 - Data Direction Clear"]
    pub dirclr1: DIRCLR,
    #[doc = "0x88 - Data Direction Set"]
    pub dirset1: DIRSET,
    #[doc = "0x8c - Data Direction Toggle"]
    pub dirtgl1: DIRTGL,
    #[doc = "0x90 - Data Output Value"]
    pub out1: OUT,
    #[doc = "0x94 - Data Output Value Clear"]
    pub outclr1: OUTCLR,
    #[doc = "0x98 - Data Output Value Set"]
    pub outset1: OUTSET,
    #[doc = "0x9c - Data Output Value Toggle"]
    pub outtgl1: OUTTGL,
    #[doc = "0xa0 - Data Input Value"]
    pub in1: IN,
    #[doc = "0xa4 - Control"]
    pub ctrl1: CTRL,
    #[doc = "0xa8 - Write Configuration"]
    pub wrconfig1: WRCONFIG,
    #[doc = "0xac - Event Input Control"]
    pub evctrl1: EVCTRL,
    #[doc = "0xb0 - Peripheral Multiplexing - Group 1"]
    pub pmux1_: [PMUX1_; 16],
    #[doc = "0xc0 - Pin Configuration - Group 1"]
    pub pincfg1_: [PINCFG1_; 32],
    _reserved28: [u8; 32usize],
    #[doc = "0x100 - Data Direction"]
    pub dir2: DIR,
    #[doc = "0x104 - Data Direction Clear"]
    pub dirclr2: DIRCLR,
    #[doc = "0x108 - Data Direction Set"]
    pub dirset2: DIRSET,
    #[doc = "0x10c - Data Direction Toggle"]
    pub dirtgl2: DIRTGL,
    #[doc = "0x110 - Data Output Value"]
    pub out2: OUT,
    #[doc = "0x114 - Data Output Value Clear"]
    pub outclr2: OUTCLR,
    #[doc = "0x118 - Data Output Value Set"]
    pub outset2: OUTSET,
    #[doc = "0x11c - Data Output Value Toggle"]
    pub outtgl2: OUTTGL,
    #[doc = "0x120 - Data Input Value"]
    pub in2: IN,
    #[doc = "0x124 - Control"]
    pub ctrl2: CTRL,
    #[doc = "0x128 - Write Configuration"]
    pub wrconfig2: WRCONFIG,
    #[doc = "0x12c - Event Input Control"]
    pub evctrl2: EVCTRL,
    #[doc = "0x130 - Peripheral Multiplexing - Group 2"]
    pub pmux2_: [PMUX2_; 16],
    #[doc = "0x140 - Pin Configuration - Group 2"]
    pub pincfg2_: [PINCFG2_; 32],
    _reserved42: [u8; 32usize],
    #[doc = "0x180 - Data Direction"]
    pub dir3: DIR,
    #[doc = "0x184 - Data Direction Clear"]
    pub dirclr3: DIRCLR,
    #[doc = "0x188 - Data Direction Set"]
    pub dirset3: DIRSET,
    #[doc = "0x18c - Data Direction Toggle"]
    pub dirtgl3: DIRTGL,
    #[doc = "0x190 - Data Output Value"]
    pub out3: OUT,
    #[doc = "0x194 - Data Output Value Clear"]
    pub outclr3: OUTCLR,
    #[doc = "0x198 - Data Output Value Set"]
    pub outset3: OUTSET,
    #[doc = "0x19c - Data Output Value Toggle"]
    pub outtgl3: OUTTGL,
    #[doc = "0x1a0 - Data Input Value"]
    pub in3: IN,
    #[doc = "0x1a4 - Control"]
    pub ctrl3: CTRL,
    #[doc = "0x1a8 - Write Configuration"]
    pub wrconfig3: WRCONFIG,
    #[doc = "0x1ac - Event Input Control"]
    pub evctrl3: EVCTRL,
    #[doc = "0x1b0 - Peripheral Multiplexing - Group 3"]
    pub pmux3_: [PMUX3_; 16],
    #[doc = "0x1c0 - Pin Configuration - Group 3"]
    pub pincfg3_: [PINCFG3_; 32],
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
#[doc = "Event Input Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [evctrl](evctrl) module"]
pub type EVCTRL = crate::Reg<u32, _EVCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVCTRL;
#[doc = "`read()` method returns [evctrl::R](evctrl::R) reader structure"]
impl crate::Readable for EVCTRL {}
#[doc = "`write(|w| ..)` method takes [evctrl::W](evctrl::W) writer structure"]
impl crate::Writable for EVCTRL {}
#[doc = "Event Input Control"]
pub mod evctrl;
#[doc = "Peripheral Multiplexing - Group 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmux0_](pmux0_) module"]
pub type PMUX0_ = crate::Reg<u8, _PMUX0_>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PMUX0_;
#[doc = "`read()` method returns [pmux0_::R](pmux0_::R) reader structure"]
impl crate::Readable for PMUX0_ {}
#[doc = "`write(|w| ..)` method takes [pmux0_::W](pmux0_::W) writer structure"]
impl crate::Writable for PMUX0_ {}
#[doc = "Peripheral Multiplexing - Group 0"]
pub mod pmux0_;
#[doc = "Peripheral Multiplexing - Group 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmux1_](pmux1_) module"]
pub type PMUX1_ = crate::Reg<u8, _PMUX1_>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PMUX1_;
#[doc = "`read()` method returns [pmux1_::R](pmux1_::R) reader structure"]
impl crate::Readable for PMUX1_ {}
#[doc = "`write(|w| ..)` method takes [pmux1_::W](pmux1_::W) writer structure"]
impl crate::Writable for PMUX1_ {}
#[doc = "Peripheral Multiplexing - Group 1"]
pub mod pmux1_;
#[doc = "Peripheral Multiplexing - Group 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmux2_](pmux2_) module"]
pub type PMUX2_ = crate::Reg<u8, _PMUX2_>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PMUX2_;
#[doc = "`read()` method returns [pmux2_::R](pmux2_::R) reader structure"]
impl crate::Readable for PMUX2_ {}
#[doc = "`write(|w| ..)` method takes [pmux2_::W](pmux2_::W) writer structure"]
impl crate::Writable for PMUX2_ {}
#[doc = "Peripheral Multiplexing - Group 2"]
pub mod pmux2_;
#[doc = "Peripheral Multiplexing - Group 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmux3_](pmux3_) module"]
pub type PMUX3_ = crate::Reg<u8, _PMUX3_>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PMUX3_;
#[doc = "`read()` method returns [pmux3_::R](pmux3_::R) reader structure"]
impl crate::Readable for PMUX3_ {}
#[doc = "`write(|w| ..)` method takes [pmux3_::W](pmux3_::W) writer structure"]
impl crate::Writable for PMUX3_ {}
#[doc = "Peripheral Multiplexing - Group 3"]
pub mod pmux3_;
#[doc = "Pin Configuration - Group 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pincfg0_](pincfg0_) module"]
pub type PINCFG0_ = crate::Reg<u8, _PINCFG0_>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PINCFG0_;
#[doc = "`read()` method returns [pincfg0_::R](pincfg0_::R) reader structure"]
impl crate::Readable for PINCFG0_ {}
#[doc = "`write(|w| ..)` method takes [pincfg0_::W](pincfg0_::W) writer structure"]
impl crate::Writable for PINCFG0_ {}
#[doc = "Pin Configuration - Group 0"]
pub mod pincfg0_;
#[doc = "Pin Configuration - Group 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pincfg1_](pincfg1_) module"]
pub type PINCFG1_ = crate::Reg<u8, _PINCFG1_>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PINCFG1_;
#[doc = "`read()` method returns [pincfg1_::R](pincfg1_::R) reader structure"]
impl crate::Readable for PINCFG1_ {}
#[doc = "`write(|w| ..)` method takes [pincfg1_::W](pincfg1_::W) writer structure"]
impl crate::Writable for PINCFG1_ {}
#[doc = "Pin Configuration - Group 1"]
pub mod pincfg1_;
#[doc = "Pin Configuration - Group 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pincfg2_](pincfg2_) module"]
pub type PINCFG2_ = crate::Reg<u8, _PINCFG2_>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PINCFG2_;
#[doc = "`read()` method returns [pincfg2_::R](pincfg2_::R) reader structure"]
impl crate::Readable for PINCFG2_ {}
#[doc = "`write(|w| ..)` method takes [pincfg2_::W](pincfg2_::W) writer structure"]
impl crate::Writable for PINCFG2_ {}
#[doc = "Pin Configuration - Group 2"]
pub mod pincfg2_;
#[doc = "Pin Configuration - Group 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pincfg3_](pincfg3_) module"]
pub type PINCFG3_ = crate::Reg<u8, _PINCFG3_>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PINCFG3_;
#[doc = "`read()` method returns [pincfg3_::R](pincfg3_::R) reader structure"]
impl crate::Readable for PINCFG3_ {}
#[doc = "`write(|w| ..)` method takes [pincfg3_::W](pincfg3_::W) writer structure"]
impl crate::Writable for PINCFG3_ {}
#[doc = "Pin Configuration - Group 3"]
pub mod pincfg3_;
