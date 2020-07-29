#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control A"]
    pub ctrla: CTRLA,
    #[doc = "0x04 - Control B"]
    pub ctrlb: CTRLB,
    #[doc = "0x05 - Interrupt Enable Clear"]
    pub intenclr: INTENCLR,
    #[doc = "0x06 - Interrupt Enable Set"]
    pub intenset: INTENSET,
    #[doc = "0x07 - Interrupt Flag Status"]
    pub intflag: INTFLAG,
    #[doc = "0x08 - Data buffer pointer"]
    pub databufptr: DATABUFPTR,
    #[doc = "0x09 - Debug control"]
    pub dbgctrl: DBGCTRL,
    _reserved7: [u8; 2usize],
    #[doc = "0x0c - Keyword n"]
    pub keyword: [KEYWORD; 8],
    _reserved8: [u8; 12usize],
    #[doc = "0x38 - Indata"]
    pub indata: INDATA,
    #[doc = "0x3c - Initialisation Vector n"]
    pub intvectv: [INTVECTV; 4],
    _reserved10: [u8; 16usize],
    #[doc = "0x5c - Hash key n"]
    pub hashkey: [HASHKEY; 4],
    #[doc = "0x6c - Galois Hash n"]
    pub ghash: [GHASH; 4],
    _reserved12: [u8; 4usize],
    #[doc = "0x80 - Cipher Length"]
    pub ciplen: CIPLEN,
    #[doc = "0x84 - Random Seed"]
    pub randseed: RANDSEED,
}
#[doc = "Control A\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrla](ctrla) module"]
pub type CTRLA = crate::Reg<u32, _CTRLA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRLA;
#[doc = "`read()` method returns [ctrla::R](ctrla::R) reader structure"]
impl crate::Readable for CTRLA {}
#[doc = "`write(|w| ..)` method takes [ctrla::W](ctrla::W) writer structure"]
impl crate::Writable for CTRLA {}
#[doc = "Control A"]
pub mod ctrla;
#[doc = "Control B\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrlb](ctrlb) module"]
pub type CTRLB = crate::Reg<u8, _CTRLB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRLB;
#[doc = "`read()` method returns [ctrlb::R](ctrlb::R) reader structure"]
impl crate::Readable for CTRLB {}
#[doc = "`write(|w| ..)` method takes [ctrlb::W](ctrlb::W) writer structure"]
impl crate::Writable for CTRLB {}
#[doc = "Control B"]
pub mod ctrlb;
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
#[doc = "Interrupt Flag Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intflag](intflag) module"]
pub type INTFLAG = crate::Reg<u8, _INTFLAG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTFLAG;
#[doc = "`read()` method returns [intflag::R](intflag::R) reader structure"]
impl crate::Readable for INTFLAG {}
#[doc = "`write(|w| ..)` method takes [intflag::W](intflag::W) writer structure"]
impl crate::Writable for INTFLAG {}
#[doc = "Interrupt Flag Status"]
pub mod intflag;
#[doc = "Data buffer pointer\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [databufptr](databufptr) module"]
pub type DATABUFPTR = crate::Reg<u8, _DATABUFPTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATABUFPTR;
#[doc = "`read()` method returns [databufptr::R](databufptr::R) reader structure"]
impl crate::Readable for DATABUFPTR {}
#[doc = "`write(|w| ..)` method takes [databufptr::W](databufptr::W) writer structure"]
impl crate::Writable for DATABUFPTR {}
#[doc = "Data buffer pointer"]
pub mod databufptr;
#[doc = "Debug control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dbgctrl](dbgctrl) module"]
pub type DBGCTRL = crate::Reg<u8, _DBGCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DBGCTRL;
#[doc = "`read()` method returns [dbgctrl::R](dbgctrl::R) reader structure"]
impl crate::Readable for DBGCTRL {}
#[doc = "`write(|w| ..)` method takes [dbgctrl::W](dbgctrl::W) writer structure"]
impl crate::Writable for DBGCTRL {}
#[doc = "Debug control"]
pub mod dbgctrl;
#[doc = "Keyword n\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [keyword](keyword) module"]
pub type KEYWORD = crate::Reg<u32, _KEYWORD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _KEYWORD;
#[doc = "`write(|w| ..)` method takes [keyword::W](keyword::W) writer structure"]
impl crate::Writable for KEYWORD {}
#[doc = "Keyword n"]
pub mod keyword;
#[doc = "Indata\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [indata](indata) module"]
pub type INDATA = crate::Reg<u32, _INDATA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INDATA;
#[doc = "`read()` method returns [indata::R](indata::R) reader structure"]
impl crate::Readable for INDATA {}
#[doc = "`write(|w| ..)` method takes [indata::W](indata::W) writer structure"]
impl crate::Writable for INDATA {}
#[doc = "Indata"]
pub mod indata;
#[doc = "Initialisation Vector n\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intvectv](intvectv) module"]
pub type INTVECTV = crate::Reg<u32, _INTVECTV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTVECTV;
#[doc = "`write(|w| ..)` method takes [intvectv::W](intvectv::W) writer structure"]
impl crate::Writable for INTVECTV {}
#[doc = "Initialisation Vector n"]
pub mod intvectv;
#[doc = "Hash key n\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hashkey](hashkey) module"]
pub type HASHKEY = crate::Reg<u32, _HASHKEY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HASHKEY;
#[doc = "`read()` method returns [hashkey::R](hashkey::R) reader structure"]
impl crate::Readable for HASHKEY {}
#[doc = "`write(|w| ..)` method takes [hashkey::W](hashkey::W) writer structure"]
impl crate::Writable for HASHKEY {}
#[doc = "Hash key n"]
pub mod hashkey;
#[doc = "Galois Hash n\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ghash](ghash) module"]
pub type GHASH = crate::Reg<u32, _GHASH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GHASH;
#[doc = "`read()` method returns [ghash::R](ghash::R) reader structure"]
impl crate::Readable for GHASH {}
#[doc = "`write(|w| ..)` method takes [ghash::W](ghash::W) writer structure"]
impl crate::Writable for GHASH {}
#[doc = "Galois Hash n"]
pub mod ghash;
#[doc = "Cipher Length\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ciplen](ciplen) module"]
pub type CIPLEN = crate::Reg<u32, _CIPLEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CIPLEN;
#[doc = "`read()` method returns [ciplen::R](ciplen::R) reader structure"]
impl crate::Readable for CIPLEN {}
#[doc = "`write(|w| ..)` method takes [ciplen::W](ciplen::W) writer structure"]
impl crate::Writable for CIPLEN {}
#[doc = "Cipher Length"]
pub mod ciplen;
#[doc = "Random Seed\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [randseed](randseed) module"]
pub type RANDSEED = crate::Reg<u32, _RANDSEED>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RANDSEED;
#[doc = "`read()` method returns [randseed::R](randseed::R) reader structure"]
impl crate::Readable for RANDSEED {}
#[doc = "`write(|w| ..)` method takes [randseed::W](randseed::W) writer structure"]
impl crate::Writable for RANDSEED {}
#[doc = "Random Seed"]
pub mod randseed;
