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
    _reserved7: [u8; 0x02],
    #[doc = "0x0c..0x2c - Keyword n"]
    pub keyword: [KEYWORD; 8],
    _reserved8: [u8; 0x0c],
    #[doc = "0x38 - Indata"]
    pub indata: INDATA,
    #[doc = "0x3c..0x4c - Initialisation Vector n"]
    pub intvectv: [INTVECTV; 4],
    _reserved10: [u8; 0x10],
    #[doc = "0x5c..0x6c - Hash key n"]
    pub hashkey: [HASHKEY; 4],
    #[doc = "0x6c..0x7c - Galois Hash n"]
    pub ghash: [GHASH; 4],
    _reserved12: [u8; 0x04],
    #[doc = "0x80 - Cipher Length"]
    pub ciplen: CIPLEN,
    #[doc = "0x84 - Random Seed"]
    pub randseed: RANDSEED,
}
#[doc = "CTRLA (rw) register accessor: an alias for `Reg<CTRLA_SPEC>`"]
pub type CTRLA = crate::Reg<ctrla::CTRLA_SPEC>;
#[doc = "Control A"]
pub mod ctrla;
#[doc = "CTRLB (rw) register accessor: an alias for `Reg<CTRLB_SPEC>`"]
pub type CTRLB = crate::Reg<ctrlb::CTRLB_SPEC>;
#[doc = "Control B"]
pub mod ctrlb;
#[doc = "INTENCLR (rw) register accessor: an alias for `Reg<INTENCLR_SPEC>`"]
pub type INTENCLR = crate::Reg<intenclr::INTENCLR_SPEC>;
#[doc = "Interrupt Enable Clear"]
pub mod intenclr;
#[doc = "INTENSET (rw) register accessor: an alias for `Reg<INTENSET_SPEC>`"]
pub type INTENSET = crate::Reg<intenset::INTENSET_SPEC>;
#[doc = "Interrupt Enable Set"]
pub mod intenset;
#[doc = "INTFLAG (rw) register accessor: an alias for `Reg<INTFLAG_SPEC>`"]
pub type INTFLAG = crate::Reg<intflag::INTFLAG_SPEC>;
#[doc = "Interrupt Flag Status"]
pub mod intflag;
#[doc = "DATABUFPTR (rw) register accessor: an alias for `Reg<DATABUFPTR_SPEC>`"]
pub type DATABUFPTR = crate::Reg<databufptr::DATABUFPTR_SPEC>;
#[doc = "Data buffer pointer"]
pub mod databufptr;
#[doc = "DBGCTRL (w) register accessor: an alias for `Reg<DBGCTRL_SPEC>`"]
pub type DBGCTRL = crate::Reg<dbgctrl::DBGCTRL_SPEC>;
#[doc = "Debug control"]
pub mod dbgctrl;
#[doc = "KEYWORD (w) register accessor: an alias for `Reg<KEYWORD_SPEC>`"]
pub type KEYWORD = crate::Reg<keyword::KEYWORD_SPEC>;
#[doc = "Keyword n"]
pub mod keyword;
#[doc = "INDATA (rw) register accessor: an alias for `Reg<INDATA_SPEC>`"]
pub type INDATA = crate::Reg<indata::INDATA_SPEC>;
#[doc = "Indata"]
pub mod indata;
#[doc = "INTVECTV (w) register accessor: an alias for `Reg<INTVECTV_SPEC>`"]
pub type INTVECTV = crate::Reg<intvectv::INTVECTV_SPEC>;
#[doc = "Initialisation Vector n"]
pub mod intvectv;
#[doc = "HASHKEY (rw) register accessor: an alias for `Reg<HASHKEY_SPEC>`"]
pub type HASHKEY = crate::Reg<hashkey::HASHKEY_SPEC>;
#[doc = "Hash key n"]
pub mod hashkey;
#[doc = "GHASH (rw) register accessor: an alias for `Reg<GHASH_SPEC>`"]
pub type GHASH = crate::Reg<ghash::GHASH_SPEC>;
#[doc = "Galois Hash n"]
pub mod ghash;
#[doc = "CIPLEN (rw) register accessor: an alias for `Reg<CIPLEN_SPEC>`"]
pub type CIPLEN = crate::Reg<ciplen::CIPLEN_SPEC>;
#[doc = "Cipher Length"]
pub mod ciplen;
#[doc = "RANDSEED (rw) register accessor: an alias for `Reg<RANDSEED_SPEC>`"]
pub type RANDSEED = crate::Reg<randseed::RANDSEED_SPEC>;
#[doc = "Random Seed"]
pub mod randseed;
