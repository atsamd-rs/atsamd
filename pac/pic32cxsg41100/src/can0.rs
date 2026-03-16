#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    crel: Crel,
    endn: Endn,
    mrcfg: Mrcfg,
    dbtp: Dbtp,
    test: Test,
    rwd: Rwd,
    cccr: Cccr,
    nbtp: Nbtp,
    tscc: Tscc,
    tscv: Tscv,
    tocc: Tocc,
    tocv: Tocv,
    _reserved12: [u8; 0x10],
    ecr: Ecr,
    psr: Psr,
    tdcr: Tdcr,
    _reserved15: [u8; 0x04],
    ir: Ir,
    ie: Ie,
    ils: Ils,
    ile: Ile,
    _reserved19: [u8; 0x20],
    gfc: Gfc,
    sidfc: Sidfc,
    xidfc: Xidfc,
    _reserved22: [u8; 0x04],
    xidam: Xidam,
    hpms: Hpms,
    ndat1: Ndat1,
    ndat2: Ndat2,
    rxf0c: Rxf0c,
    rxf0s: Rxf0s,
    rxf0a: Rxf0a,
    rxbc: Rxbc,
    rxf1c: Rxf1c,
    rxf1s: Rxf1s,
    rxf1a: Rxf1a,
    rxesc: Rxesc,
    txbc: Txbc,
    txfqs: Txfqs,
    txesc: Txesc,
    txbrp: Txbrp,
    txbar: Txbar,
    txbcr: Txbcr,
    txbto: Txbto,
    txbcf: Txbcf,
    txbtie: Txbtie,
    txbcie: Txbcie,
    _reserved44: [u8; 0x08],
    txefc: Txefc,
    txefs: Txefs,
    txefa: Txefa,
}
impl RegisterBlock {
    #[doc = "0x00 - Core Release"]
    #[inline(always)]
    pub const fn crel(&self) -> &Crel {
        &self.crel
    }
    #[doc = "0x04 - Endian"]
    #[inline(always)]
    pub const fn endn(&self) -> &Endn {
        &self.endn
    }
    #[doc = "0x08 - Message RAM Configuration"]
    #[inline(always)]
    pub const fn mrcfg(&self) -> &Mrcfg {
        &self.mrcfg
    }
    #[doc = "0x0c - Fast Bit Timing and Prescaler"]
    #[inline(always)]
    pub const fn dbtp(&self) -> &Dbtp {
        &self.dbtp
    }
    #[doc = "0x10 - Test"]
    #[inline(always)]
    pub const fn test(&self) -> &Test {
        &self.test
    }
    #[doc = "0x14 - RAM Watchdog"]
    #[inline(always)]
    pub const fn rwd(&self) -> &Rwd {
        &self.rwd
    }
    #[doc = "0x18 - CC Control"]
    #[inline(always)]
    pub const fn cccr(&self) -> &Cccr {
        &self.cccr
    }
    #[doc = "0x1c - Nominal Bit Timing and Prescaler"]
    #[inline(always)]
    pub const fn nbtp(&self) -> &Nbtp {
        &self.nbtp
    }
    #[doc = "0x20 - Timestamp Counter Configuration"]
    #[inline(always)]
    pub const fn tscc(&self) -> &Tscc {
        &self.tscc
    }
    #[doc = "0x24 - Timestamp Counter Value"]
    #[inline(always)]
    pub const fn tscv(&self) -> &Tscv {
        &self.tscv
    }
    #[doc = "0x28 - Timeout Counter Configuration"]
    #[inline(always)]
    pub const fn tocc(&self) -> &Tocc {
        &self.tocc
    }
    #[doc = "0x2c - Timeout Counter Value"]
    #[inline(always)]
    pub const fn tocv(&self) -> &Tocv {
        &self.tocv
    }
    #[doc = "0x40 - Error Counter"]
    #[inline(always)]
    pub const fn ecr(&self) -> &Ecr {
        &self.ecr
    }
    #[doc = "0x44 - Protocol Status"]
    #[inline(always)]
    pub const fn psr(&self) -> &Psr {
        &self.psr
    }
    #[doc = "0x48 - Extended ID Filter Configuration"]
    #[inline(always)]
    pub const fn tdcr(&self) -> &Tdcr {
        &self.tdcr
    }
    #[doc = "0x50 - Interrupt"]
    #[inline(always)]
    pub const fn ir(&self) -> &Ir {
        &self.ir
    }
    #[doc = "0x54 - Interrupt Enable"]
    #[inline(always)]
    pub const fn ie(&self) -> &Ie {
        &self.ie
    }
    #[doc = "0x58 - Interrupt Line Select"]
    #[inline(always)]
    pub const fn ils(&self) -> &Ils {
        &self.ils
    }
    #[doc = "0x5c - Interrupt Line Enable"]
    #[inline(always)]
    pub const fn ile(&self) -> &Ile {
        &self.ile
    }
    #[doc = "0x80 - Global Filter Configuration"]
    #[inline(always)]
    pub const fn gfc(&self) -> &Gfc {
        &self.gfc
    }
    #[doc = "0x84 - Standard ID Filter Configuration"]
    #[inline(always)]
    pub const fn sidfc(&self) -> &Sidfc {
        &self.sidfc
    }
    #[doc = "0x88 - Extended ID Filter Configuration"]
    #[inline(always)]
    pub const fn xidfc(&self) -> &Xidfc {
        &self.xidfc
    }
    #[doc = "0x90 - Extended ID AND Mask"]
    #[inline(always)]
    pub const fn xidam(&self) -> &Xidam {
        &self.xidam
    }
    #[doc = "0x94 - High Priority Message Status"]
    #[inline(always)]
    pub const fn hpms(&self) -> &Hpms {
        &self.hpms
    }
    #[doc = "0x98 - New Data 1"]
    #[inline(always)]
    pub const fn ndat1(&self) -> &Ndat1 {
        &self.ndat1
    }
    #[doc = "0x9c - New Data 2"]
    #[inline(always)]
    pub const fn ndat2(&self) -> &Ndat2 {
        &self.ndat2
    }
    #[doc = "0xa0 - Rx FIFO 0 Configuration"]
    #[inline(always)]
    pub const fn rxf0c(&self) -> &Rxf0c {
        &self.rxf0c
    }
    #[doc = "0xa4 - Rx FIFO 0 Status"]
    #[inline(always)]
    pub const fn rxf0s(&self) -> &Rxf0s {
        &self.rxf0s
    }
    #[doc = "0xa8 - Rx FIFO 0 Acknowledge"]
    #[inline(always)]
    pub const fn rxf0a(&self) -> &Rxf0a {
        &self.rxf0a
    }
    #[doc = "0xac - Rx Buffer Configuration"]
    #[inline(always)]
    pub const fn rxbc(&self) -> &Rxbc {
        &self.rxbc
    }
    #[doc = "0xb0 - Rx FIFO 1 Configuration"]
    #[inline(always)]
    pub const fn rxf1c(&self) -> &Rxf1c {
        &self.rxf1c
    }
    #[doc = "0xb4 - Rx FIFO 1 Status"]
    #[inline(always)]
    pub const fn rxf1s(&self) -> &Rxf1s {
        &self.rxf1s
    }
    #[doc = "0xb8 - Rx FIFO 1 Acknowledge"]
    #[inline(always)]
    pub const fn rxf1a(&self) -> &Rxf1a {
        &self.rxf1a
    }
    #[doc = "0xbc - Rx Buffer / FIFO Element Size Configuration"]
    #[inline(always)]
    pub const fn rxesc(&self) -> &Rxesc {
        &self.rxesc
    }
    #[doc = "0xc0 - Tx Buffer Configuration"]
    #[inline(always)]
    pub const fn txbc(&self) -> &Txbc {
        &self.txbc
    }
    #[doc = "0xc4 - Tx FIFO / Queue Status"]
    #[inline(always)]
    pub const fn txfqs(&self) -> &Txfqs {
        &self.txfqs
    }
    #[doc = "0xc8 - Tx Buffer Element Size Configuration"]
    #[inline(always)]
    pub const fn txesc(&self) -> &Txesc {
        &self.txesc
    }
    #[doc = "0xcc - Tx Buffer Request Pending"]
    #[inline(always)]
    pub const fn txbrp(&self) -> &Txbrp {
        &self.txbrp
    }
    #[doc = "0xd0 - Tx Buffer Add Request"]
    #[inline(always)]
    pub const fn txbar(&self) -> &Txbar {
        &self.txbar
    }
    #[doc = "0xd4 - Tx Buffer Cancellation Request"]
    #[inline(always)]
    pub const fn txbcr(&self) -> &Txbcr {
        &self.txbcr
    }
    #[doc = "0xd8 - Tx Buffer Transmission Occurred"]
    #[inline(always)]
    pub const fn txbto(&self) -> &Txbto {
        &self.txbto
    }
    #[doc = "0xdc - Tx Buffer Cancellation Finished"]
    #[inline(always)]
    pub const fn txbcf(&self) -> &Txbcf {
        &self.txbcf
    }
    #[doc = "0xe0 - Tx Buffer Transmission Interrupt Enable"]
    #[inline(always)]
    pub const fn txbtie(&self) -> &Txbtie {
        &self.txbtie
    }
    #[doc = "0xe4 - Tx Buffer Cancellation Finished Interrupt Enable"]
    #[inline(always)]
    pub const fn txbcie(&self) -> &Txbcie {
        &self.txbcie
    }
    #[doc = "0xf0 - Tx Event FIFO Configuration"]
    #[inline(always)]
    pub const fn txefc(&self) -> &Txefc {
        &self.txefc
    }
    #[doc = "0xf4 - Tx Event FIFO Status"]
    #[inline(always)]
    pub const fn txefs(&self) -> &Txefs {
        &self.txefs
    }
    #[doc = "0xf8 - Tx Event FIFO Acknowledge"]
    #[inline(always)]
    pub const fn txefa(&self) -> &Txefa {
        &self.txefa
    }
}
#[doc = "CREL (r) register accessor: Core Release\n\nYou can [`read`](crate::Reg::read) this register and get [`crel::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crel`]
module"]
#[doc(alias = "CREL")]
pub type Crel = crate::Reg<crel::CrelSpec>;
#[doc = "Core Release"]
pub mod crel;
#[doc = "ENDN (r) register accessor: Endian\n\nYou can [`read`](crate::Reg::read) this register and get [`endn::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@endn`]
module"]
#[doc(alias = "ENDN")]
pub type Endn = crate::Reg<endn::EndnSpec>;
#[doc = "Endian"]
pub mod endn;
#[doc = "MRCFG (rw) register accessor: Message RAM Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`mrcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mrcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mrcfg`]
module"]
#[doc(alias = "MRCFG")]
pub type Mrcfg = crate::Reg<mrcfg::MrcfgSpec>;
#[doc = "Message RAM Configuration"]
pub mod mrcfg;
#[doc = "DBTP (rw) register accessor: Fast Bit Timing and Prescaler\n\nYou can [`read`](crate::Reg::read) this register and get [`dbtp::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbtp::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbtp`]
module"]
#[doc(alias = "DBTP")]
pub type Dbtp = crate::Reg<dbtp::DbtpSpec>;
#[doc = "Fast Bit Timing and Prescaler"]
pub mod dbtp;
#[doc = "TEST (rw) register accessor: Test\n\nYou can [`read`](crate::Reg::read) this register and get [`test::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`test::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@test`]
module"]
#[doc(alias = "TEST")]
pub type Test = crate::Reg<test::TestSpec>;
#[doc = "Test"]
pub mod test;
#[doc = "RWD (rw) register accessor: RAM Watchdog\n\nYou can [`read`](crate::Reg::read) this register and get [`rwd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rwd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rwd`]
module"]
#[doc(alias = "RWD")]
pub type Rwd = crate::Reg<rwd::RwdSpec>;
#[doc = "RAM Watchdog"]
pub mod rwd;
#[doc = "CCCR (rw) register accessor: CC Control\n\nYou can [`read`](crate::Reg::read) this register and get [`cccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cccr`]
module"]
#[doc(alias = "CCCR")]
pub type Cccr = crate::Reg<cccr::CccrSpec>;
#[doc = "CC Control"]
pub mod cccr;
#[doc = "NBTP (rw) register accessor: Nominal Bit Timing and Prescaler\n\nYou can [`read`](crate::Reg::read) this register and get [`nbtp::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nbtp::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nbtp`]
module"]
#[doc(alias = "NBTP")]
pub type Nbtp = crate::Reg<nbtp::NbtpSpec>;
#[doc = "Nominal Bit Timing and Prescaler"]
pub mod nbtp;
#[doc = "TSCC (rw) register accessor: Timestamp Counter Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`tscc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tscc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tscc`]
module"]
#[doc(alias = "TSCC")]
pub type Tscc = crate::Reg<tscc::TsccSpec>;
#[doc = "Timestamp Counter Configuration"]
pub mod tscc;
#[doc = "TSCV (r) register accessor: Timestamp Counter Value\n\nYou can [`read`](crate::Reg::read) this register and get [`tscv::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tscv`]
module"]
#[doc(alias = "TSCV")]
pub type Tscv = crate::Reg<tscv::TscvSpec>;
#[doc = "Timestamp Counter Value"]
pub mod tscv;
#[doc = "TOCC (rw) register accessor: Timeout Counter Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`tocc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tocc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tocc`]
module"]
#[doc(alias = "TOCC")]
pub type Tocc = crate::Reg<tocc::ToccSpec>;
#[doc = "Timeout Counter Configuration"]
pub mod tocc;
#[doc = "TOCV (rw) register accessor: Timeout Counter Value\n\nYou can [`read`](crate::Reg::read) this register and get [`tocv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tocv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tocv`]
module"]
#[doc(alias = "TOCV")]
pub type Tocv = crate::Reg<tocv::TocvSpec>;
#[doc = "Timeout Counter Value"]
pub mod tocv;
#[doc = "ECR (r) register accessor: Error Counter\n\nYou can [`read`](crate::Reg::read) this register and get [`ecr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecr`]
module"]
#[doc(alias = "ECR")]
pub type Ecr = crate::Reg<ecr::EcrSpec>;
#[doc = "Error Counter"]
pub mod ecr;
#[doc = "PSR (r) register accessor: Protocol Status\n\nYou can [`read`](crate::Reg::read) this register and get [`psr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@psr`]
module"]
#[doc(alias = "PSR")]
pub type Psr = crate::Reg<psr::PsrSpec>;
#[doc = "Protocol Status"]
pub mod psr;
#[doc = "TDCR (rw) register accessor: Extended ID Filter Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`tdcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tdcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tdcr`]
module"]
#[doc(alias = "TDCR")]
pub type Tdcr = crate::Reg<tdcr::TdcrSpec>;
#[doc = "Extended ID Filter Configuration"]
pub mod tdcr;
#[doc = "IR (rw) register accessor: Interrupt\n\nYou can [`read`](crate::Reg::read) this register and get [`ir::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ir::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ir`]
module"]
#[doc(alias = "IR")]
pub type Ir = crate::Reg<ir::IrSpec>;
#[doc = "Interrupt"]
pub mod ir;
#[doc = "IE (rw) register accessor: Interrupt Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`ie::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ie::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ie`]
module"]
#[doc(alias = "IE")]
pub type Ie = crate::Reg<ie::IeSpec>;
#[doc = "Interrupt Enable"]
pub mod ie;
#[doc = "ILS (rw) register accessor: Interrupt Line Select\n\nYou can [`read`](crate::Reg::read) this register and get [`ils::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ils::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ils`]
module"]
#[doc(alias = "ILS")]
pub type Ils = crate::Reg<ils::IlsSpec>;
#[doc = "Interrupt Line Select"]
pub mod ils;
#[doc = "ILE (rw) register accessor: Interrupt Line Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`ile::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ile::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ile`]
module"]
#[doc(alias = "ILE")]
pub type Ile = crate::Reg<ile::IleSpec>;
#[doc = "Interrupt Line Enable"]
pub mod ile;
#[doc = "GFC (rw) register accessor: Global Filter Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`gfc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gfc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gfc`]
module"]
#[doc(alias = "GFC")]
pub type Gfc = crate::Reg<gfc::GfcSpec>;
#[doc = "Global Filter Configuration"]
pub mod gfc;
#[doc = "SIDFC (rw) register accessor: Standard ID Filter Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`sidfc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sidfc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sidfc`]
module"]
#[doc(alias = "SIDFC")]
pub type Sidfc = crate::Reg<sidfc::SidfcSpec>;
#[doc = "Standard ID Filter Configuration"]
pub mod sidfc;
#[doc = "XIDFC (rw) register accessor: Extended ID Filter Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`xidfc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xidfc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@xidfc`]
module"]
#[doc(alias = "XIDFC")]
pub type Xidfc = crate::Reg<xidfc::XidfcSpec>;
#[doc = "Extended ID Filter Configuration"]
pub mod xidfc;
#[doc = "XIDAM (rw) register accessor: Extended ID AND Mask\n\nYou can [`read`](crate::Reg::read) this register and get [`xidam::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xidam::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@xidam`]
module"]
#[doc(alias = "XIDAM")]
pub type Xidam = crate::Reg<xidam::XidamSpec>;
#[doc = "Extended ID AND Mask"]
pub mod xidam;
#[doc = "HPMS (r) register accessor: High Priority Message Status\n\nYou can [`read`](crate::Reg::read) this register and get [`hpms::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hpms`]
module"]
#[doc(alias = "HPMS")]
pub type Hpms = crate::Reg<hpms::HpmsSpec>;
#[doc = "High Priority Message Status"]
pub mod hpms;
#[doc = "NDAT1 (rw) register accessor: New Data 1\n\nYou can [`read`](crate::Reg::read) this register and get [`ndat1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ndat1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ndat1`]
module"]
#[doc(alias = "NDAT1")]
pub type Ndat1 = crate::Reg<ndat1::Ndat1Spec>;
#[doc = "New Data 1"]
pub mod ndat1;
#[doc = "NDAT2 (rw) register accessor: New Data 2\n\nYou can [`read`](crate::Reg::read) this register and get [`ndat2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ndat2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ndat2`]
module"]
#[doc(alias = "NDAT2")]
pub type Ndat2 = crate::Reg<ndat2::Ndat2Spec>;
#[doc = "New Data 2"]
pub mod ndat2;
#[doc = "RXF0C (rw) register accessor: Rx FIFO 0 Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`rxf0c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxf0c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxf0c`]
module"]
#[doc(alias = "RXF0C")]
pub type Rxf0c = crate::Reg<rxf0c::Rxf0cSpec>;
#[doc = "Rx FIFO 0 Configuration"]
pub mod rxf0c;
#[doc = "RXF0S (r) register accessor: Rx FIFO 0 Status\n\nYou can [`read`](crate::Reg::read) this register and get [`rxf0s::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxf0s`]
module"]
#[doc(alias = "RXF0S")]
pub type Rxf0s = crate::Reg<rxf0s::Rxf0sSpec>;
#[doc = "Rx FIFO 0 Status"]
pub mod rxf0s;
#[doc = "RXF0A (rw) register accessor: Rx FIFO 0 Acknowledge\n\nYou can [`read`](crate::Reg::read) this register and get [`rxf0a::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxf0a::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxf0a`]
module"]
#[doc(alias = "RXF0A")]
pub type Rxf0a = crate::Reg<rxf0a::Rxf0aSpec>;
#[doc = "Rx FIFO 0 Acknowledge"]
pub mod rxf0a;
#[doc = "RXBC (rw) register accessor: Rx Buffer Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`rxbc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxbc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxbc`]
module"]
#[doc(alias = "RXBC")]
pub type Rxbc = crate::Reg<rxbc::RxbcSpec>;
#[doc = "Rx Buffer Configuration"]
pub mod rxbc;
#[doc = "RXF1C (rw) register accessor: Rx FIFO 1 Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`rxf1c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxf1c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxf1c`]
module"]
#[doc(alias = "RXF1C")]
pub type Rxf1c = crate::Reg<rxf1c::Rxf1cSpec>;
#[doc = "Rx FIFO 1 Configuration"]
pub mod rxf1c;
#[doc = "RXF1S (r) register accessor: Rx FIFO 1 Status\n\nYou can [`read`](crate::Reg::read) this register and get [`rxf1s::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxf1s`]
module"]
#[doc(alias = "RXF1S")]
pub type Rxf1s = crate::Reg<rxf1s::Rxf1sSpec>;
#[doc = "Rx FIFO 1 Status"]
pub mod rxf1s;
#[doc = "RXF1A (rw) register accessor: Rx FIFO 1 Acknowledge\n\nYou can [`read`](crate::Reg::read) this register and get [`rxf1a::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxf1a::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxf1a`]
module"]
#[doc(alias = "RXF1A")]
pub type Rxf1a = crate::Reg<rxf1a::Rxf1aSpec>;
#[doc = "Rx FIFO 1 Acknowledge"]
pub mod rxf1a;
#[doc = "RXESC (rw) register accessor: Rx Buffer / FIFO Element Size Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`rxesc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxesc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxesc`]
module"]
#[doc(alias = "RXESC")]
pub type Rxesc = crate::Reg<rxesc::RxescSpec>;
#[doc = "Rx Buffer / FIFO Element Size Configuration"]
pub mod rxesc;
#[doc = "TXBC (rw) register accessor: Tx Buffer Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`txbc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txbc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txbc`]
module"]
#[doc(alias = "TXBC")]
pub type Txbc = crate::Reg<txbc::TxbcSpec>;
#[doc = "Tx Buffer Configuration"]
pub mod txbc;
#[doc = "TXFQS (r) register accessor: Tx FIFO / Queue Status\n\nYou can [`read`](crate::Reg::read) this register and get [`txfqs::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txfqs`]
module"]
#[doc(alias = "TXFQS")]
pub type Txfqs = crate::Reg<txfqs::TxfqsSpec>;
#[doc = "Tx FIFO / Queue Status"]
pub mod txfqs;
#[doc = "TXESC (rw) register accessor: Tx Buffer Element Size Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`txesc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txesc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txesc`]
module"]
#[doc(alias = "TXESC")]
pub type Txesc = crate::Reg<txesc::TxescSpec>;
#[doc = "Tx Buffer Element Size Configuration"]
pub mod txesc;
#[doc = "TXBRP (r) register accessor: Tx Buffer Request Pending\n\nYou can [`read`](crate::Reg::read) this register and get [`txbrp::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txbrp`]
module"]
#[doc(alias = "TXBRP")]
pub type Txbrp = crate::Reg<txbrp::TxbrpSpec>;
#[doc = "Tx Buffer Request Pending"]
pub mod txbrp;
#[doc = "TXBAR (rw) register accessor: Tx Buffer Add Request\n\nYou can [`read`](crate::Reg::read) this register and get [`txbar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txbar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txbar`]
module"]
#[doc(alias = "TXBAR")]
pub type Txbar = crate::Reg<txbar::TxbarSpec>;
#[doc = "Tx Buffer Add Request"]
pub mod txbar;
#[doc = "TXBCR (rw) register accessor: Tx Buffer Cancellation Request\n\nYou can [`read`](crate::Reg::read) this register and get [`txbcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txbcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txbcr`]
module"]
#[doc(alias = "TXBCR")]
pub type Txbcr = crate::Reg<txbcr::TxbcrSpec>;
#[doc = "Tx Buffer Cancellation Request"]
pub mod txbcr;
#[doc = "TXBTO (r) register accessor: Tx Buffer Transmission Occurred\n\nYou can [`read`](crate::Reg::read) this register and get [`txbto::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txbto`]
module"]
#[doc(alias = "TXBTO")]
pub type Txbto = crate::Reg<txbto::TxbtoSpec>;
#[doc = "Tx Buffer Transmission Occurred"]
pub mod txbto;
#[doc = "TXBCF (r) register accessor: Tx Buffer Cancellation Finished\n\nYou can [`read`](crate::Reg::read) this register and get [`txbcf::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txbcf`]
module"]
#[doc(alias = "TXBCF")]
pub type Txbcf = crate::Reg<txbcf::TxbcfSpec>;
#[doc = "Tx Buffer Cancellation Finished"]
pub mod txbcf;
#[doc = "TXBTIE (rw) register accessor: Tx Buffer Transmission Interrupt Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`txbtie::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txbtie::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txbtie`]
module"]
#[doc(alias = "TXBTIE")]
pub type Txbtie = crate::Reg<txbtie::TxbtieSpec>;
#[doc = "Tx Buffer Transmission Interrupt Enable"]
pub mod txbtie;
#[doc = "TXBCIE (rw) register accessor: Tx Buffer Cancellation Finished Interrupt Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`txbcie::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txbcie::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txbcie`]
module"]
#[doc(alias = "TXBCIE")]
pub type Txbcie = crate::Reg<txbcie::TxbcieSpec>;
#[doc = "Tx Buffer Cancellation Finished Interrupt Enable"]
pub mod txbcie;
#[doc = "TXEFC (rw) register accessor: Tx Event FIFO Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`txefc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txefc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txefc`]
module"]
#[doc(alias = "TXEFC")]
pub type Txefc = crate::Reg<txefc::TxefcSpec>;
#[doc = "Tx Event FIFO Configuration"]
pub mod txefc;
#[doc = "TXEFS (r) register accessor: Tx Event FIFO Status\n\nYou can [`read`](crate::Reg::read) this register and get [`txefs::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txefs`]
module"]
#[doc(alias = "TXEFS")]
pub type Txefs = crate::Reg<txefs::TxefsSpec>;
#[doc = "Tx Event FIFO Status"]
pub mod txefs;
#[doc = "TXEFA (rw) register accessor: Tx Event FIFO Acknowledge\n\nYou can [`read`](crate::Reg::read) this register and get [`txefa::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txefa::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txefa`]
module"]
#[doc(alias = "TXEFA")]
pub type Txefa = crate::Reg<txefa::TxefaSpec>;
#[doc = "Tx Event FIFO Acknowledge"]
pub mod txefa;
