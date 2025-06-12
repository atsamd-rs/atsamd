#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ctrla: Ctrla,
    ctrlb: Ctrlb,
    intenclr: Intenclr,
    intenset: Intenset,
    intflag: Intflag,
    databufptr: Databufptr,
    dbgctrl: Dbgctrl,
    _reserved7: [u8; 0x02],
    keyword: [Keyword; 8],
    _reserved8: [u8; 0x0c],
    indata: Indata,
    intvectv: [Intvectv; 4],
    _reserved10: [u8; 0x10],
    hashkey: [Hashkey; 4],
    ghash: [Ghash; 4],
    _reserved12: [u8; 0x04],
    ciplen: Ciplen,
    randseed: Randseed,
}
impl RegisterBlock {
    #[doc = "0x00 - Control A"]
    #[inline(always)]
    pub const fn ctrla(&self) -> &Ctrla {
        &self.ctrla
    }
    #[doc = "0x04 - Control B"]
    #[inline(always)]
    pub const fn ctrlb(&self) -> &Ctrlb {
        &self.ctrlb
    }
    #[doc = "0x05 - Interrupt Enable Clear"]
    #[inline(always)]
    pub const fn intenclr(&self) -> &Intenclr {
        &self.intenclr
    }
    #[doc = "0x06 - Interrupt Enable Set"]
    #[inline(always)]
    pub const fn intenset(&self) -> &Intenset {
        &self.intenset
    }
    #[doc = "0x07 - Interrupt Flag Status"]
    #[inline(always)]
    pub const fn intflag(&self) -> &Intflag {
        &self.intflag
    }
    #[doc = "0x08 - Data buffer pointer"]
    #[inline(always)]
    pub const fn databufptr(&self) -> &Databufptr {
        &self.databufptr
    }
    #[doc = "0x09 - Debug control"]
    #[inline(always)]
    pub const fn dbgctrl(&self) -> &Dbgctrl {
        &self.dbgctrl
    }
    #[doc = "0x0c..0x2c - Keyword n"]
    #[inline(always)]
    pub const fn keyword(&self, n: usize) -> &Keyword {
        &self.keyword[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x0c..0x2c - Keyword n"]
    #[inline(always)]
    pub fn keyword_iter(&self) -> impl Iterator<Item = &Keyword> {
        self.keyword.iter()
    }
    #[doc = "0x38 - Indata"]
    #[inline(always)]
    pub const fn indata(&self) -> &Indata {
        &self.indata
    }
    #[doc = "0x3c..0x4c - Initialisation Vector n"]
    #[inline(always)]
    pub const fn intvectv(&self, n: usize) -> &Intvectv {
        &self.intvectv[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x3c..0x4c - Initialisation Vector n"]
    #[inline(always)]
    pub fn intvectv_iter(&self) -> impl Iterator<Item = &Intvectv> {
        self.intvectv.iter()
    }
    #[doc = "0x5c..0x6c - Hash key n"]
    #[inline(always)]
    pub const fn hashkey(&self, n: usize) -> &Hashkey {
        &self.hashkey[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x5c..0x6c - Hash key n"]
    #[inline(always)]
    pub fn hashkey_iter(&self) -> impl Iterator<Item = &Hashkey> {
        self.hashkey.iter()
    }
    #[doc = "0x6c..0x7c - Galois Hash n"]
    #[inline(always)]
    pub const fn ghash(&self, n: usize) -> &Ghash {
        &self.ghash[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x6c..0x7c - Galois Hash n"]
    #[inline(always)]
    pub fn ghash_iter(&self) -> impl Iterator<Item = &Ghash> {
        self.ghash.iter()
    }
    #[doc = "0x80 - Cipher Length"]
    #[inline(always)]
    pub const fn ciplen(&self) -> &Ciplen {
        &self.ciplen
    }
    #[doc = "0x84 - Random Seed"]
    #[inline(always)]
    pub const fn randseed(&self) -> &Randseed {
        &self.randseed
    }
}
#[doc = "CTRLA (rw) register accessor: Control A\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrla::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrla::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrla`] module"]
#[doc(alias = "CTRLA")]
pub type Ctrla = crate::Reg<ctrla::CtrlaSpec>;
#[doc = "Control A"]
pub mod ctrla;
#[doc = "CTRLB (rw) register accessor: Control B\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrlb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrlb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrlb`] module"]
#[doc(alias = "CTRLB")]
pub type Ctrlb = crate::Reg<ctrlb::CtrlbSpec>;
#[doc = "Control B"]
pub mod ctrlb;
#[doc = "INTENCLR (rw) register accessor: Interrupt Enable Clear\n\nYou can [`read`](crate::Reg::read) this register and get [`intenclr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenclr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intenclr`] module"]
#[doc(alias = "INTENCLR")]
pub type Intenclr = crate::Reg<intenclr::IntenclrSpec>;
#[doc = "Interrupt Enable Clear"]
pub mod intenclr;
#[doc = "INTENSET (rw) register accessor: Interrupt Enable Set\n\nYou can [`read`](crate::Reg::read) this register and get [`intenset::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenset::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intenset`] module"]
#[doc(alias = "INTENSET")]
pub type Intenset = crate::Reg<intenset::IntensetSpec>;
#[doc = "Interrupt Enable Set"]
pub mod intenset;
#[doc = "INTFLAG (rw) register accessor: Interrupt Flag Status\n\nYou can [`read`](crate::Reg::read) this register and get [`intflag::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intflag::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intflag`] module"]
#[doc(alias = "INTFLAG")]
pub type Intflag = crate::Reg<intflag::IntflagSpec>;
#[doc = "Interrupt Flag Status"]
pub mod intflag;
#[doc = "DATABUFPTR (rw) register accessor: Data buffer pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`databufptr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`databufptr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@databufptr`] module"]
#[doc(alias = "DATABUFPTR")]
pub type Databufptr = crate::Reg<databufptr::DatabufptrSpec>;
#[doc = "Data buffer pointer"]
pub mod databufptr;
#[doc = "DBGCTRL (rw) register accessor: Debug control\n\nYou can [`read`](crate::Reg::read) this register and get [`dbgctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbgctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbgctrl`] module"]
#[doc(alias = "DBGCTRL")]
pub type Dbgctrl = crate::Reg<dbgctrl::DbgctrlSpec>;
#[doc = "Debug control"]
pub mod dbgctrl;
#[doc = "KEYWORD (w) register accessor: Keyword n\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`keyword::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@keyword`] module"]
#[doc(alias = "KEYWORD")]
pub type Keyword = crate::Reg<keyword::KeywordSpec>;
#[doc = "Keyword n"]
pub mod keyword;
#[doc = "INDATA (rw) register accessor: Indata\n\nYou can [`read`](crate::Reg::read) this register and get [`indata::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`indata::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@indata`] module"]
#[doc(alias = "INDATA")]
pub type Indata = crate::Reg<indata::IndataSpec>;
#[doc = "Indata"]
pub mod indata;
#[doc = "INTVECTV (w) register accessor: Initialisation Vector n\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvectv::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvectv`] module"]
#[doc(alias = "INTVECTV")]
pub type Intvectv = crate::Reg<intvectv::IntvectvSpec>;
#[doc = "Initialisation Vector n"]
pub mod intvectv;
#[doc = "HASHKEY (rw) register accessor: Hash key n\n\nYou can [`read`](crate::Reg::read) this register and get [`hashkey::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hashkey::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hashkey`] module"]
#[doc(alias = "HASHKEY")]
pub type Hashkey = crate::Reg<hashkey::HashkeySpec>;
#[doc = "Hash key n"]
pub mod hashkey;
#[doc = "GHASH (rw) register accessor: Galois Hash n\n\nYou can [`read`](crate::Reg::read) this register and get [`ghash::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ghash::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ghash`] module"]
#[doc(alias = "GHASH")]
pub type Ghash = crate::Reg<ghash::GhashSpec>;
#[doc = "Galois Hash n"]
pub mod ghash;
#[doc = "CIPLEN (rw) register accessor: Cipher Length\n\nYou can [`read`](crate::Reg::read) this register and get [`ciplen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ciplen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ciplen`] module"]
#[doc(alias = "CIPLEN")]
pub type Ciplen = crate::Reg<ciplen::CiplenSpec>;
#[doc = "Cipher Length"]
pub mod ciplen;
#[doc = "RANDSEED (rw) register accessor: Random Seed\n\nYou can [`read`](crate::Reg::read) this register and get [`randseed::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`randseed::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@randseed`] module"]
#[doc(alias = "RANDSEED")]
pub type Randseed = crate::Reg<randseed::RandseedSpec>;
#[doc = "Random Seed"]
pub mod randseed;
