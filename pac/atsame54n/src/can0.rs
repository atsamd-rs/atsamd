#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Core Release"]
    pub crel: CREL,
    #[doc = "0x04 - Endian"]
    pub endn: ENDN,
    #[doc = "0x08 - Message RAM Configuration"]
    pub mrcfg: MRCFG,
    #[doc = "0x0c - Fast Bit Timing and Prescaler"]
    pub dbtp: DBTP,
    #[doc = "0x10 - Test"]
    pub test: TEST,
    #[doc = "0x14 - RAM Watchdog"]
    pub rwd: RWD,
    #[doc = "0x18 - CC Control"]
    pub cccr: CCCR,
    #[doc = "0x1c - Nominal Bit Timing and Prescaler"]
    pub nbtp: NBTP,
    #[doc = "0x20 - Timestamp Counter Configuration"]
    pub tscc: TSCC,
    #[doc = "0x24 - Timestamp Counter Value"]
    pub tscv: TSCV,
    #[doc = "0x28 - Timeout Counter Configuration"]
    pub tocc: TOCC,
    #[doc = "0x2c - Timeout Counter Value"]
    pub tocv: TOCV,
    _reserved12: [u8; 0x10],
    #[doc = "0x40 - Error Counter"]
    pub ecr: ECR,
    #[doc = "0x44 - Protocol Status"]
    pub psr: PSR,
    #[doc = "0x48 - Extended ID Filter Configuration"]
    pub tdcr: TDCR,
    _reserved15: [u8; 0x04],
    #[doc = "0x50 - Interrupt"]
    pub ir: IR,
    #[doc = "0x54 - Interrupt Enable"]
    pub ie: IE,
    #[doc = "0x58 - Interrupt Line Select"]
    pub ils: ILS,
    #[doc = "0x5c - Interrupt Line Enable"]
    pub ile: ILE,
    _reserved19: [u8; 0x20],
    #[doc = "0x80 - Global Filter Configuration"]
    pub gfc: GFC,
    #[doc = "0x84 - Standard ID Filter Configuration"]
    pub sidfc: SIDFC,
    #[doc = "0x88 - Extended ID Filter Configuration"]
    pub xidfc: XIDFC,
    _reserved22: [u8; 0x04],
    #[doc = "0x90 - Extended ID AND Mask"]
    pub xidam: XIDAM,
    #[doc = "0x94 - High Priority Message Status"]
    pub hpms: HPMS,
    #[doc = "0x98 - New Data 1"]
    pub ndat1: NDAT1,
    #[doc = "0x9c - New Data 2"]
    pub ndat2: NDAT2,
    #[doc = "0xa0 - Rx FIFO 0 Configuration"]
    pub rxf0c: RXF0C,
    #[doc = "0xa4 - Rx FIFO 0 Status"]
    pub rxf0s: RXF0S,
    #[doc = "0xa8 - Rx FIFO 0 Acknowledge"]
    pub rxf0a: RXF0A,
    #[doc = "0xac - Rx Buffer Configuration"]
    pub rxbc: RXBC,
    #[doc = "0xb0 - Rx FIFO 1 Configuration"]
    pub rxf1c: RXF1C,
    #[doc = "0xb4 - Rx FIFO 1 Status"]
    pub rxf1s: RXF1S,
    #[doc = "0xb8 - Rx FIFO 1 Acknowledge"]
    pub rxf1a: RXF1A,
    #[doc = "0xbc - Rx Buffer / FIFO Element Size Configuration"]
    pub rxesc: RXESC,
    #[doc = "0xc0 - Tx Buffer Configuration"]
    pub txbc: TXBC,
    #[doc = "0xc4 - Tx FIFO / Queue Status"]
    pub txfqs: TXFQS,
    #[doc = "0xc8 - Tx Buffer Element Size Configuration"]
    pub txesc: TXESC,
    #[doc = "0xcc - Tx Buffer Request Pending"]
    pub txbrp: TXBRP,
    #[doc = "0xd0 - Tx Buffer Add Request"]
    pub txbar: TXBAR,
    #[doc = "0xd4 - Tx Buffer Cancellation Request"]
    pub txbcr: TXBCR,
    #[doc = "0xd8 - Tx Buffer Transmission Occurred"]
    pub txbto: TXBTO,
    #[doc = "0xdc - Tx Buffer Cancellation Finished"]
    pub txbcf: TXBCF,
    #[doc = "0xe0 - Tx Buffer Transmission Interrupt Enable"]
    pub txbtie: TXBTIE,
    #[doc = "0xe4 - Tx Buffer Cancellation Finished Interrupt Enable"]
    pub txbcie: TXBCIE,
    _reserved44: [u8; 0x08],
    #[doc = "0xf0 - Tx Event FIFO Configuration"]
    pub txefc: TXEFC,
    #[doc = "0xf4 - Tx Event FIFO Status"]
    pub txefs: TXEFS,
    #[doc = "0xf8 - Tx Event FIFO Acknowledge"]
    pub txefa: TXEFA,
}
#[doc = "CREL (r) register accessor: Core Release\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`crel::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crel`]
module"]
pub type CREL = crate::Reg<crel::CREL_SPEC>;
#[doc = "Core Release"]
pub mod crel;
#[doc = "ENDN (r) register accessor: Endian\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`endn::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@endn`]
module"]
pub type ENDN = crate::Reg<endn::ENDN_SPEC>;
#[doc = "Endian"]
pub mod endn;
#[doc = "MRCFG (rw) register accessor: Message RAM Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mrcfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mrcfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mrcfg`]
module"]
pub type MRCFG = crate::Reg<mrcfg::MRCFG_SPEC>;
#[doc = "Message RAM Configuration"]
pub mod mrcfg;
#[doc = "DBTP (rw) register accessor: Fast Bit Timing and Prescaler\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dbtp::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dbtp::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbtp`]
module"]
pub type DBTP = crate::Reg<dbtp::DBTP_SPEC>;
#[doc = "Fast Bit Timing and Prescaler"]
pub mod dbtp;
#[doc = "TEST (rw) register accessor: Test\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`test::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`test::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@test`]
module"]
pub type TEST = crate::Reg<test::TEST_SPEC>;
#[doc = "Test"]
pub mod test;
#[doc = "RWD (rw) register accessor: RAM Watchdog\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rwd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rwd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rwd`]
module"]
pub type RWD = crate::Reg<rwd::RWD_SPEC>;
#[doc = "RAM Watchdog"]
pub mod rwd;
#[doc = "CCCR (rw) register accessor: CC Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cccr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cccr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cccr`]
module"]
pub type CCCR = crate::Reg<cccr::CCCR_SPEC>;
#[doc = "CC Control"]
pub mod cccr;
#[doc = "NBTP (rw) register accessor: Nominal Bit Timing and Prescaler\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nbtp::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nbtp::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nbtp`]
module"]
pub type NBTP = crate::Reg<nbtp::NBTP_SPEC>;
#[doc = "Nominal Bit Timing and Prescaler"]
pub mod nbtp;
#[doc = "TSCC (rw) register accessor: Timestamp Counter Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tscc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tscc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tscc`]
module"]
pub type TSCC = crate::Reg<tscc::TSCC_SPEC>;
#[doc = "Timestamp Counter Configuration"]
pub mod tscc;
#[doc = "TSCV (r) register accessor: Timestamp Counter Value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tscv::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tscv`]
module"]
pub type TSCV = crate::Reg<tscv::TSCV_SPEC>;
#[doc = "Timestamp Counter Value"]
pub mod tscv;
#[doc = "TOCC (rw) register accessor: Timeout Counter Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tocc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tocc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tocc`]
module"]
pub type TOCC = crate::Reg<tocc::TOCC_SPEC>;
#[doc = "Timeout Counter Configuration"]
pub mod tocc;
#[doc = "TOCV (rw) register accessor: Timeout Counter Value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tocv::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tocv::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tocv`]
module"]
pub type TOCV = crate::Reg<tocv::TOCV_SPEC>;
#[doc = "Timeout Counter Value"]
pub mod tocv;
#[doc = "ECR (r) register accessor: Error Counter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ecr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecr`]
module"]
pub type ECR = crate::Reg<ecr::ECR_SPEC>;
#[doc = "Error Counter"]
pub mod ecr;
#[doc = "PSR (r) register accessor: Protocol Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`psr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@psr`]
module"]
pub type PSR = crate::Reg<psr::PSR_SPEC>;
#[doc = "Protocol Status"]
pub mod psr;
#[doc = "TDCR (rw) register accessor: Extended ID Filter Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tdcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tdcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tdcr`]
module"]
pub type TDCR = crate::Reg<tdcr::TDCR_SPEC>;
#[doc = "Extended ID Filter Configuration"]
pub mod tdcr;
#[doc = "IR (rw) register accessor: Interrupt\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ir::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ir::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ir`]
module"]
pub type IR = crate::Reg<ir::IR_SPEC>;
#[doc = "Interrupt"]
pub mod ir;
#[doc = "IE (rw) register accessor: Interrupt Enable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ie::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ie::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ie`]
module"]
pub type IE = crate::Reg<ie::IE_SPEC>;
#[doc = "Interrupt Enable"]
pub mod ie;
#[doc = "ILS (rw) register accessor: Interrupt Line Select\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ils::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ils::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ils`]
module"]
pub type ILS = crate::Reg<ils::ILS_SPEC>;
#[doc = "Interrupt Line Select"]
pub mod ils;
#[doc = "ILE (rw) register accessor: Interrupt Line Enable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ile::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ile::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ile`]
module"]
pub type ILE = crate::Reg<ile::ILE_SPEC>;
#[doc = "Interrupt Line Enable"]
pub mod ile;
#[doc = "GFC (rw) register accessor: Global Filter Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gfc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gfc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gfc`]
module"]
pub type GFC = crate::Reg<gfc::GFC_SPEC>;
#[doc = "Global Filter Configuration"]
pub mod gfc;
#[doc = "SIDFC (rw) register accessor: Standard ID Filter Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sidfc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sidfc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sidfc`]
module"]
pub type SIDFC = crate::Reg<sidfc::SIDFC_SPEC>;
#[doc = "Standard ID Filter Configuration"]
pub mod sidfc;
#[doc = "XIDFC (rw) register accessor: Extended ID Filter Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`xidfc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`xidfc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@xidfc`]
module"]
pub type XIDFC = crate::Reg<xidfc::XIDFC_SPEC>;
#[doc = "Extended ID Filter Configuration"]
pub mod xidfc;
#[doc = "XIDAM (rw) register accessor: Extended ID AND Mask\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`xidam::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`xidam::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@xidam`]
module"]
pub type XIDAM = crate::Reg<xidam::XIDAM_SPEC>;
#[doc = "Extended ID AND Mask"]
pub mod xidam;
#[doc = "HPMS (r) register accessor: High Priority Message Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hpms::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hpms`]
module"]
pub type HPMS = crate::Reg<hpms::HPMS_SPEC>;
#[doc = "High Priority Message Status"]
pub mod hpms;
#[doc = "NDAT1 (rw) register accessor: New Data 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ndat1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ndat1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ndat1`]
module"]
pub type NDAT1 = crate::Reg<ndat1::NDAT1_SPEC>;
#[doc = "New Data 1"]
pub mod ndat1;
#[doc = "NDAT2 (rw) register accessor: New Data 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ndat2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ndat2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ndat2`]
module"]
pub type NDAT2 = crate::Reg<ndat2::NDAT2_SPEC>;
#[doc = "New Data 2"]
pub mod ndat2;
#[doc = "RXF0C (rw) register accessor: Rx FIFO 0 Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxf0c::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxf0c::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxf0c`]
module"]
pub type RXF0C = crate::Reg<rxf0c::RXF0C_SPEC>;
#[doc = "Rx FIFO 0 Configuration"]
pub mod rxf0c;
#[doc = "RXF0S (r) register accessor: Rx FIFO 0 Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxf0s::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxf0s`]
module"]
pub type RXF0S = crate::Reg<rxf0s::RXF0S_SPEC>;
#[doc = "Rx FIFO 0 Status"]
pub mod rxf0s;
#[doc = "RXF0A (rw) register accessor: Rx FIFO 0 Acknowledge\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxf0a::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxf0a::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxf0a`]
module"]
pub type RXF0A = crate::Reg<rxf0a::RXF0A_SPEC>;
#[doc = "Rx FIFO 0 Acknowledge"]
pub mod rxf0a;
#[doc = "RXBC (rw) register accessor: Rx Buffer Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxbc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxbc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxbc`]
module"]
pub type RXBC = crate::Reg<rxbc::RXBC_SPEC>;
#[doc = "Rx Buffer Configuration"]
pub mod rxbc;
#[doc = "RXF1C (rw) register accessor: Rx FIFO 1 Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxf1c::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxf1c::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxf1c`]
module"]
pub type RXF1C = crate::Reg<rxf1c::RXF1C_SPEC>;
#[doc = "Rx FIFO 1 Configuration"]
pub mod rxf1c;
#[doc = "RXF1S (r) register accessor: Rx FIFO 1 Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxf1s::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxf1s`]
module"]
pub type RXF1S = crate::Reg<rxf1s::RXF1S_SPEC>;
#[doc = "Rx FIFO 1 Status"]
pub mod rxf1s;
#[doc = "RXF1A (rw) register accessor: Rx FIFO 1 Acknowledge\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxf1a::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxf1a::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxf1a`]
module"]
pub type RXF1A = crate::Reg<rxf1a::RXF1A_SPEC>;
#[doc = "Rx FIFO 1 Acknowledge"]
pub mod rxf1a;
#[doc = "RXESC (rw) register accessor: Rx Buffer / FIFO Element Size Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxesc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxesc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxesc`]
module"]
pub type RXESC = crate::Reg<rxesc::RXESC_SPEC>;
#[doc = "Rx Buffer / FIFO Element Size Configuration"]
pub mod rxesc;
#[doc = "TXBC (rw) register accessor: Tx Buffer Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txbc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txbc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txbc`]
module"]
pub type TXBC = crate::Reg<txbc::TXBC_SPEC>;
#[doc = "Tx Buffer Configuration"]
pub mod txbc;
#[doc = "TXFQS (r) register accessor: Tx FIFO / Queue Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txfqs::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txfqs`]
module"]
pub type TXFQS = crate::Reg<txfqs::TXFQS_SPEC>;
#[doc = "Tx FIFO / Queue Status"]
pub mod txfqs;
#[doc = "TXESC (rw) register accessor: Tx Buffer Element Size Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txesc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txesc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txesc`]
module"]
pub type TXESC = crate::Reg<txesc::TXESC_SPEC>;
#[doc = "Tx Buffer Element Size Configuration"]
pub mod txesc;
#[doc = "TXBRP (r) register accessor: Tx Buffer Request Pending\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txbrp::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txbrp`]
module"]
pub type TXBRP = crate::Reg<txbrp::TXBRP_SPEC>;
#[doc = "Tx Buffer Request Pending"]
pub mod txbrp;
#[doc = "TXBAR (rw) register accessor: Tx Buffer Add Request\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txbar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txbar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txbar`]
module"]
pub type TXBAR = crate::Reg<txbar::TXBAR_SPEC>;
#[doc = "Tx Buffer Add Request"]
pub mod txbar;
#[doc = "TXBCR (rw) register accessor: Tx Buffer Cancellation Request\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txbcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txbcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txbcr`]
module"]
pub type TXBCR = crate::Reg<txbcr::TXBCR_SPEC>;
#[doc = "Tx Buffer Cancellation Request"]
pub mod txbcr;
#[doc = "TXBTO (r) register accessor: Tx Buffer Transmission Occurred\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txbto::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txbto`]
module"]
pub type TXBTO = crate::Reg<txbto::TXBTO_SPEC>;
#[doc = "Tx Buffer Transmission Occurred"]
pub mod txbto;
#[doc = "TXBCF (r) register accessor: Tx Buffer Cancellation Finished\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txbcf::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txbcf`]
module"]
pub type TXBCF = crate::Reg<txbcf::TXBCF_SPEC>;
#[doc = "Tx Buffer Cancellation Finished"]
pub mod txbcf;
#[doc = "TXBTIE (rw) register accessor: Tx Buffer Transmission Interrupt Enable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txbtie::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txbtie::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txbtie`]
module"]
pub type TXBTIE = crate::Reg<txbtie::TXBTIE_SPEC>;
#[doc = "Tx Buffer Transmission Interrupt Enable"]
pub mod txbtie;
#[doc = "TXBCIE (rw) register accessor: Tx Buffer Cancellation Finished Interrupt Enable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txbcie::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txbcie::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txbcie`]
module"]
pub type TXBCIE = crate::Reg<txbcie::TXBCIE_SPEC>;
#[doc = "Tx Buffer Cancellation Finished Interrupt Enable"]
pub mod txbcie;
#[doc = "TXEFC (rw) register accessor: Tx Event FIFO Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txefc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txefc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txefc`]
module"]
pub type TXEFC = crate::Reg<txefc::TXEFC_SPEC>;
#[doc = "Tx Event FIFO Configuration"]
pub mod txefc;
#[doc = "TXEFS (r) register accessor: Tx Event FIFO Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txefs::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txefs`]
module"]
pub type TXEFS = crate::Reg<txefs::TXEFS_SPEC>;
#[doc = "Tx Event FIFO Status"]
pub mod txefs;
#[doc = "TXEFA (rw) register accessor: Tx Event FIFO Acknowledge\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txefa::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txefa::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txefa`]
module"]
pub type TXEFA = crate::Reg<txefa::TXEFA_SPEC>;
#[doc = "Tx Event FIFO Acknowledge"]
pub mod txefa;
