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
#[doc = "CTRLA (rw) register accessor: Control A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrla::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrla::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrla`]
module"]
pub type CTRLA = crate::Reg<ctrla::CTRLA_SPEC>;
#[doc = "Control A"]
pub mod ctrla;
#[doc = "CTRLB (rw) register accessor: Control B\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrlb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrlb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrlb`]
module"]
pub type CTRLB = crate::Reg<ctrlb::CTRLB_SPEC>;
#[doc = "Control B"]
pub mod ctrlb;
#[doc = "INTENCLR (rw) register accessor: Interrupt Enable Clear\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intenclr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intenclr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intenclr`]
module"]
pub type INTENCLR = crate::Reg<intenclr::INTENCLR_SPEC>;
#[doc = "Interrupt Enable Clear"]
pub mod intenclr;
#[doc = "INTENSET (rw) register accessor: Interrupt Enable Set\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intenset::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intenset::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intenset`]
module"]
pub type INTENSET = crate::Reg<intenset::INTENSET_SPEC>;
#[doc = "Interrupt Enable Set"]
pub mod intenset;
#[doc = "INTFLAG (rw) register accessor: Interrupt Flag Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intflag::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intflag::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intflag`]
module"]
pub type INTFLAG = crate::Reg<intflag::INTFLAG_SPEC>;
#[doc = "Interrupt Flag Status"]
pub mod intflag;
#[doc = "DATABUFPTR (rw) register accessor: Data buffer pointer\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`databufptr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`databufptr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@databufptr`]
module"]
pub type DATABUFPTR = crate::Reg<databufptr::DATABUFPTR_SPEC>;
#[doc = "Data buffer pointer"]
pub mod databufptr;
#[doc = "DBGCTRL (rw) register accessor: Debug control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dbgctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dbgctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbgctrl`]
module"]
pub type DBGCTRL = crate::Reg<dbgctrl::DBGCTRL_SPEC>;
#[doc = "Debug control"]
pub mod dbgctrl;
#[doc = "KEYWORD (w) register accessor: Keyword n\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`keyword::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@keyword`]
module"]
pub type KEYWORD = crate::Reg<keyword::KEYWORD_SPEC>;
#[doc = "Keyword n"]
pub mod keyword;
#[doc = "INDATA (rw) register accessor: Indata\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`indata::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`indata::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@indata`]
module"]
pub type INDATA = crate::Reg<indata::INDATA_SPEC>;
#[doc = "Indata"]
pub mod indata;
#[doc = "INTVECTV (w) register accessor: Initialisation Vector n\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intvectv::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvectv`]
module"]
pub type INTVECTV = crate::Reg<intvectv::INTVECTV_SPEC>;
#[doc = "Initialisation Vector n"]
pub mod intvectv;
#[doc = "HASHKEY (rw) register accessor: Hash key n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hashkey::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hashkey::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hashkey`]
module"]
pub type HASHKEY = crate::Reg<hashkey::HASHKEY_SPEC>;
#[doc = "Hash key n"]
pub mod hashkey;
#[doc = "GHASH (rw) register accessor: Galois Hash n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ghash::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ghash::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ghash`]
module"]
pub type GHASH = crate::Reg<ghash::GHASH_SPEC>;
#[doc = "Galois Hash n"]
pub mod ghash;
#[doc = "CIPLEN (rw) register accessor: Cipher Length\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ciplen::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ciplen::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ciplen`]
module"]
pub type CIPLEN = crate::Reg<ciplen::CIPLEN_SPEC>;
#[doc = "Cipher Length"]
pub mod ciplen;
#[doc = "RANDSEED (rw) register accessor: Random Seed\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`randseed::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`randseed::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@randseed`]
module"]
pub type RANDSEED = crate::Reg<randseed::RANDSEED_SPEC>;
#[doc = "Random Seed"]
pub mod randseed;
