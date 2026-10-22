#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Core Release"]
    pub crel: crate::Reg<crel::CREL_SPEC>,
    #[doc = "0x04 - Endian"]
    pub endn: crate::Reg<endn::ENDN_SPEC>,
    #[doc = "0x08 - Message RAM Configuration"]
    pub mrcfg: crate::Reg<mrcfg::MRCFG_SPEC>,
    #[doc = "0x0c - Fast Bit Timing and Prescaler"]
    pub dbtp: crate::Reg<dbtp::DBTP_SPEC>,
    #[doc = "0x10 - Test"]
    pub test: crate::Reg<test::TEST_SPEC>,
    #[doc = "0x14 - RAM Watchdog"]
    pub rwd: crate::Reg<rwd::RWD_SPEC>,
    #[doc = "0x18 - CC Control"]
    pub cccr: crate::Reg<cccr::CCCR_SPEC>,
    #[doc = "0x1c - Nominal Bit Timing and Prescaler"]
    pub nbtp: crate::Reg<nbtp::NBTP_SPEC>,
    #[doc = "0x20 - Timestamp Counter Configuration"]
    pub tscc: crate::Reg<tscc::TSCC_SPEC>,
    #[doc = "0x24 - Timestamp Counter Value"]
    pub tscv: crate::Reg<tscv::TSCV_SPEC>,
    #[doc = "0x28 - Timeout Counter Configuration"]
    pub tocc: crate::Reg<tocc::TOCC_SPEC>,
    #[doc = "0x2c - Timeout Counter Value"]
    pub tocv: crate::Reg<tocv::TOCV_SPEC>,
    _reserved12: [u8; 0x10],
    #[doc = "0x40 - Error Counter"]
    pub ecr: crate::Reg<ecr::ECR_SPEC>,
    #[doc = "0x44 - Protocol Status"]
    pub psr: crate::Reg<psr::PSR_SPEC>,
    #[doc = "0x48 - Extended ID Filter Configuration"]
    pub tdcr: crate::Reg<tdcr::TDCR_SPEC>,
    _reserved15: [u8; 0x04],
    #[doc = "0x50 - Interrupt"]
    pub ir: crate::Reg<ir::IR_SPEC>,
    #[doc = "0x54 - Interrupt Enable"]
    pub ie: crate::Reg<ie::IE_SPEC>,
    #[doc = "0x58 - Interrupt Line Select"]
    pub ils: crate::Reg<ils::ILS_SPEC>,
    #[doc = "0x5c - Interrupt Line Enable"]
    pub ile: crate::Reg<ile::ILE_SPEC>,
    _reserved19: [u8; 0x20],
    #[doc = "0x80 - Global Filter Configuration"]
    pub gfc: crate::Reg<gfc::GFC_SPEC>,
    #[doc = "0x84 - Standard ID Filter Configuration"]
    pub sidfc: crate::Reg<sidfc::SIDFC_SPEC>,
    #[doc = "0x88 - Extended ID Filter Configuration"]
    pub xidfc: crate::Reg<xidfc::XIDFC_SPEC>,
    _reserved22: [u8; 0x04],
    #[doc = "0x90 - Extended ID AND Mask"]
    pub xidam: crate::Reg<xidam::XIDAM_SPEC>,
    #[doc = "0x94 - High Priority Message Status"]
    pub hpms: crate::Reg<hpms::HPMS_SPEC>,
    #[doc = "0x98 - New Data 1"]
    pub ndat1: crate::Reg<ndat1::NDAT1_SPEC>,
    #[doc = "0x9c - New Data 2"]
    pub ndat2: crate::Reg<ndat2::NDAT2_SPEC>,
    #[doc = "0xa0 - Rx FIFO 0 Configuration"]
    pub rxf0c: crate::Reg<rxf0c::RXF0C_SPEC>,
    #[doc = "0xa4 - Rx FIFO 0 Status"]
    pub rxf0s: crate::Reg<rxf0s::RXF0S_SPEC>,
    #[doc = "0xa8 - Rx FIFO 0 Acknowledge"]
    pub rxf0a: crate::Reg<rxf0a::RXF0A_SPEC>,
    #[doc = "0xac - Rx Buffer Configuration"]
    pub rxbc: crate::Reg<rxbc::RXBC_SPEC>,
    #[doc = "0xb0 - Rx FIFO 1 Configuration"]
    pub rxf1c: crate::Reg<rxf1c::RXF1C_SPEC>,
    #[doc = "0xb4 - Rx FIFO 1 Status"]
    pub rxf1s: crate::Reg<rxf1s::RXF1S_SPEC>,
    #[doc = "0xb8 - Rx FIFO 1 Acknowledge"]
    pub rxf1a: crate::Reg<rxf1a::RXF1A_SPEC>,
    #[doc = "0xbc - Rx Buffer / FIFO Element Size Configuration"]
    pub rxesc: crate::Reg<rxesc::RXESC_SPEC>,
    #[doc = "0xc0 - Tx Buffer Configuration"]
    pub txbc: crate::Reg<txbc::TXBC_SPEC>,
    #[doc = "0xc4 - Tx FIFO / Queue Status"]
    pub txfqs: crate::Reg<txfqs::TXFQS_SPEC>,
    #[doc = "0xc8 - Tx Buffer Element Size Configuration"]
    pub txesc: crate::Reg<txesc::TXESC_SPEC>,
    #[doc = "0xcc - Tx Buffer Request Pending"]
    pub txbrp: crate::Reg<txbrp::TXBRP_SPEC>,
    #[doc = "0xd0 - Tx Buffer Add Request"]
    pub txbar: crate::Reg<txbar::TXBAR_SPEC>,
    #[doc = "0xd4 - Tx Buffer Cancellation Request"]
    pub txbcr: crate::Reg<txbcr::TXBCR_SPEC>,
    #[doc = "0xd8 - Tx Buffer Transmission Occurred"]
    pub txbto: crate::Reg<txbto::TXBTO_SPEC>,
    #[doc = "0xdc - Tx Buffer Cancellation Finished"]
    pub txbcf: crate::Reg<txbcf::TXBCF_SPEC>,
    #[doc = "0xe0 - Tx Buffer Transmission Interrupt Enable"]
    pub txbtie: crate::Reg<txbtie::TXBTIE_SPEC>,
    #[doc = "0xe4 - Tx Buffer Cancellation Finished Interrupt Enable"]
    pub txbcie: crate::Reg<txbcie::TXBCIE_SPEC>,
    _reserved44: [u8; 0x08],
    #[doc = "0xf0 - Tx Event FIFO Configuration"]
    pub txefc: crate::Reg<txefc::TXEFC_SPEC>,
    #[doc = "0xf4 - Tx Event FIFO Status"]
    pub txefs: crate::Reg<txefs::TXEFS_SPEC>,
    #[doc = "0xf8 - Tx Event FIFO Acknowledge"]
    pub txefa: crate::Reg<txefa::TXEFA_SPEC>,
}
#[doc = "CREL register accessor: an alias for `Reg<CREL_SPEC>`"]
pub type CREL = crate::Reg<crel::CREL_SPEC>;
#[doc = "Core Release"]
pub mod crel;
#[doc = "ENDN register accessor: an alias for `Reg<ENDN_SPEC>`"]
pub type ENDN = crate::Reg<endn::ENDN_SPEC>;
#[doc = "Endian"]
pub mod endn;
#[doc = "MRCFG register accessor: an alias for `Reg<MRCFG_SPEC>`"]
pub type MRCFG = crate::Reg<mrcfg::MRCFG_SPEC>;
#[doc = "Message RAM Configuration"]
pub mod mrcfg;
#[doc = "DBTP register accessor: an alias for `Reg<DBTP_SPEC>`"]
pub type DBTP = crate::Reg<dbtp::DBTP_SPEC>;
#[doc = "Fast Bit Timing and Prescaler"]
pub mod dbtp;
#[doc = "TEST register accessor: an alias for `Reg<TEST_SPEC>`"]
pub type TEST = crate::Reg<test::TEST_SPEC>;
#[doc = "Test"]
pub mod test;
#[doc = "RWD register accessor: an alias for `Reg<RWD_SPEC>`"]
pub type RWD = crate::Reg<rwd::RWD_SPEC>;
#[doc = "RAM Watchdog"]
pub mod rwd;
#[doc = "CCCR register accessor: an alias for `Reg<CCCR_SPEC>`"]
pub type CCCR = crate::Reg<cccr::CCCR_SPEC>;
#[doc = "CC Control"]
pub mod cccr;
#[doc = "NBTP register accessor: an alias for `Reg<NBTP_SPEC>`"]
pub type NBTP = crate::Reg<nbtp::NBTP_SPEC>;
#[doc = "Nominal Bit Timing and Prescaler"]
pub mod nbtp;
#[doc = "TSCC register accessor: an alias for `Reg<TSCC_SPEC>`"]
pub type TSCC = crate::Reg<tscc::TSCC_SPEC>;
#[doc = "Timestamp Counter Configuration"]
pub mod tscc;
#[doc = "TSCV register accessor: an alias for `Reg<TSCV_SPEC>`"]
pub type TSCV = crate::Reg<tscv::TSCV_SPEC>;
#[doc = "Timestamp Counter Value"]
pub mod tscv;
#[doc = "TOCC register accessor: an alias for `Reg<TOCC_SPEC>`"]
pub type TOCC = crate::Reg<tocc::TOCC_SPEC>;
#[doc = "Timeout Counter Configuration"]
pub mod tocc;
#[doc = "TOCV register accessor: an alias for `Reg<TOCV_SPEC>`"]
pub type TOCV = crate::Reg<tocv::TOCV_SPEC>;
#[doc = "Timeout Counter Value"]
pub mod tocv;
#[doc = "ECR register accessor: an alias for `Reg<ECR_SPEC>`"]
pub type ECR = crate::Reg<ecr::ECR_SPEC>;
#[doc = "Error Counter"]
pub mod ecr;
#[doc = "PSR register accessor: an alias for `Reg<PSR_SPEC>`"]
pub type PSR = crate::Reg<psr::PSR_SPEC>;
#[doc = "Protocol Status"]
pub mod psr;
#[doc = "TDCR register accessor: an alias for `Reg<TDCR_SPEC>`"]
pub type TDCR = crate::Reg<tdcr::TDCR_SPEC>;
#[doc = "Extended ID Filter Configuration"]
pub mod tdcr;
#[doc = "IR register accessor: an alias for `Reg<IR_SPEC>`"]
pub type IR = crate::Reg<ir::IR_SPEC>;
#[doc = "Interrupt"]
pub mod ir;
#[doc = "IE register accessor: an alias for `Reg<IE_SPEC>`"]
pub type IE = crate::Reg<ie::IE_SPEC>;
#[doc = "Interrupt Enable"]
pub mod ie;
#[doc = "ILS register accessor: an alias for `Reg<ILS_SPEC>`"]
pub type ILS = crate::Reg<ils::ILS_SPEC>;
#[doc = "Interrupt Line Select"]
pub mod ils;
#[doc = "ILE register accessor: an alias for `Reg<ILE_SPEC>`"]
pub type ILE = crate::Reg<ile::ILE_SPEC>;
#[doc = "Interrupt Line Enable"]
pub mod ile;
#[doc = "GFC register accessor: an alias for `Reg<GFC_SPEC>`"]
pub type GFC = crate::Reg<gfc::GFC_SPEC>;
#[doc = "Global Filter Configuration"]
pub mod gfc;
#[doc = "SIDFC register accessor: an alias for `Reg<SIDFC_SPEC>`"]
pub type SIDFC = crate::Reg<sidfc::SIDFC_SPEC>;
#[doc = "Standard ID Filter Configuration"]
pub mod sidfc;
#[doc = "XIDFC register accessor: an alias for `Reg<XIDFC_SPEC>`"]
pub type XIDFC = crate::Reg<xidfc::XIDFC_SPEC>;
#[doc = "Extended ID Filter Configuration"]
pub mod xidfc;
#[doc = "XIDAM register accessor: an alias for `Reg<XIDAM_SPEC>`"]
pub type XIDAM = crate::Reg<xidam::XIDAM_SPEC>;
#[doc = "Extended ID AND Mask"]
pub mod xidam;
#[doc = "HPMS register accessor: an alias for `Reg<HPMS_SPEC>`"]
pub type HPMS = crate::Reg<hpms::HPMS_SPEC>;
#[doc = "High Priority Message Status"]
pub mod hpms;
#[doc = "NDAT1 register accessor: an alias for `Reg<NDAT1_SPEC>`"]
pub type NDAT1 = crate::Reg<ndat1::NDAT1_SPEC>;
#[doc = "New Data 1"]
pub mod ndat1;
#[doc = "NDAT2 register accessor: an alias for `Reg<NDAT2_SPEC>`"]
pub type NDAT2 = crate::Reg<ndat2::NDAT2_SPEC>;
#[doc = "New Data 2"]
pub mod ndat2;
#[doc = "RXF0C register accessor: an alias for `Reg<RXF0C_SPEC>`"]
pub type RXF0C = crate::Reg<rxf0c::RXF0C_SPEC>;
#[doc = "Rx FIFO 0 Configuration"]
pub mod rxf0c;
#[doc = "RXF0S register accessor: an alias for `Reg<RXF0S_SPEC>`"]
pub type RXF0S = crate::Reg<rxf0s::RXF0S_SPEC>;
#[doc = "Rx FIFO 0 Status"]
pub mod rxf0s;
#[doc = "RXF0A register accessor: an alias for `Reg<RXF0A_SPEC>`"]
pub type RXF0A = crate::Reg<rxf0a::RXF0A_SPEC>;
#[doc = "Rx FIFO 0 Acknowledge"]
pub mod rxf0a;
#[doc = "RXBC register accessor: an alias for `Reg<RXBC_SPEC>`"]
pub type RXBC = crate::Reg<rxbc::RXBC_SPEC>;
#[doc = "Rx Buffer Configuration"]
pub mod rxbc;
#[doc = "RXF1C register accessor: an alias for `Reg<RXF1C_SPEC>`"]
pub type RXF1C = crate::Reg<rxf1c::RXF1C_SPEC>;
#[doc = "Rx FIFO 1 Configuration"]
pub mod rxf1c;
#[doc = "RXF1S register accessor: an alias for `Reg<RXF1S_SPEC>`"]
pub type RXF1S = crate::Reg<rxf1s::RXF1S_SPEC>;
#[doc = "Rx FIFO 1 Status"]
pub mod rxf1s;
#[doc = "RXF1A register accessor: an alias for `Reg<RXF1A_SPEC>`"]
pub type RXF1A = crate::Reg<rxf1a::RXF1A_SPEC>;
#[doc = "Rx FIFO 1 Acknowledge"]
pub mod rxf1a;
#[doc = "RXESC register accessor: an alias for `Reg<RXESC_SPEC>`"]
pub type RXESC = crate::Reg<rxesc::RXESC_SPEC>;
#[doc = "Rx Buffer / FIFO Element Size Configuration"]
pub mod rxesc;
#[doc = "TXBC register accessor: an alias for `Reg<TXBC_SPEC>`"]
pub type TXBC = crate::Reg<txbc::TXBC_SPEC>;
#[doc = "Tx Buffer Configuration"]
pub mod txbc;
#[doc = "TXFQS register accessor: an alias for `Reg<TXFQS_SPEC>`"]
pub type TXFQS = crate::Reg<txfqs::TXFQS_SPEC>;
#[doc = "Tx FIFO / Queue Status"]
pub mod txfqs;
#[doc = "TXESC register accessor: an alias for `Reg<TXESC_SPEC>`"]
pub type TXESC = crate::Reg<txesc::TXESC_SPEC>;
#[doc = "Tx Buffer Element Size Configuration"]
pub mod txesc;
#[doc = "TXBRP register accessor: an alias for `Reg<TXBRP_SPEC>`"]
pub type TXBRP = crate::Reg<txbrp::TXBRP_SPEC>;
#[doc = "Tx Buffer Request Pending"]
pub mod txbrp;
#[doc = "TXBAR register accessor: an alias for `Reg<TXBAR_SPEC>`"]
pub type TXBAR = crate::Reg<txbar::TXBAR_SPEC>;
#[doc = "Tx Buffer Add Request"]
pub mod txbar;
#[doc = "TXBCR register accessor: an alias for `Reg<TXBCR_SPEC>`"]
pub type TXBCR = crate::Reg<txbcr::TXBCR_SPEC>;
#[doc = "Tx Buffer Cancellation Request"]
pub mod txbcr;
#[doc = "TXBTO register accessor: an alias for `Reg<TXBTO_SPEC>`"]
pub type TXBTO = crate::Reg<txbto::TXBTO_SPEC>;
#[doc = "Tx Buffer Transmission Occurred"]
pub mod txbto;
#[doc = "TXBCF register accessor: an alias for `Reg<TXBCF_SPEC>`"]
pub type TXBCF = crate::Reg<txbcf::TXBCF_SPEC>;
#[doc = "Tx Buffer Cancellation Finished"]
pub mod txbcf;
#[doc = "TXBTIE register accessor: an alias for `Reg<TXBTIE_SPEC>`"]
pub type TXBTIE = crate::Reg<txbtie::TXBTIE_SPEC>;
#[doc = "Tx Buffer Transmission Interrupt Enable"]
pub mod txbtie;
#[doc = "TXBCIE register accessor: an alias for `Reg<TXBCIE_SPEC>`"]
pub type TXBCIE = crate::Reg<txbcie::TXBCIE_SPEC>;
#[doc = "Tx Buffer Cancellation Finished Interrupt Enable"]
pub mod txbcie;
#[doc = "TXEFC register accessor: an alias for `Reg<TXEFC_SPEC>`"]
pub type TXEFC = crate::Reg<txefc::TXEFC_SPEC>;
#[doc = "Tx Event FIFO Configuration"]
pub mod txefc;
#[doc = "TXEFS register accessor: an alias for `Reg<TXEFS_SPEC>`"]
pub type TXEFS = crate::Reg<txefs::TXEFS_SPEC>;
#[doc = "Tx Event FIFO Status"]
pub mod txefs;
#[doc = "TXEFA register accessor: an alias for `Reg<TXEFA_SPEC>`"]
pub type TXEFA = crate::Reg<txefa::TXEFA_SPEC>;
#[doc = "Tx Event FIFO Acknowledge"]
pub mod txefa;
