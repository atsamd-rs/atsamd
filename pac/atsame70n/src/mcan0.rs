#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Core Release Register"]
    pub mcan_crel: crate::Reg<mcan_crel::MCAN_CREL_SPEC>,
    #[doc = "0x04 - Endian Register"]
    pub mcan_endn: crate::Reg<mcan_endn::MCAN_ENDN_SPEC>,
    #[doc = "0x08 - Customer Register"]
    pub mcan_cust: crate::Reg<mcan_cust::MCAN_CUST_SPEC>,
    #[doc = "0x0c - Data Bit Timing and Prescaler Register"]
    pub mcan_dbtp: crate::Reg<mcan_dbtp::MCAN_DBTP_SPEC>,
    #[doc = "0x10 - Test Register"]
    pub mcan_test: crate::Reg<mcan_test::MCAN_TEST_SPEC>,
    #[doc = "0x14 - RAM Watchdog Register"]
    pub mcan_rwd: crate::Reg<mcan_rwd::MCAN_RWD_SPEC>,
    #[doc = "0x18 - CC Control Register"]
    pub mcan_cccr: crate::Reg<mcan_cccr::MCAN_CCCR_SPEC>,
    #[doc = "0x1c - Nominal Bit Timing and Prescaler Register"]
    pub mcan_nbtp: crate::Reg<mcan_nbtp::MCAN_NBTP_SPEC>,
    #[doc = "0x20 - Timestamp Counter Configuration Register"]
    pub mcan_tscc: crate::Reg<mcan_tscc::MCAN_TSCC_SPEC>,
    #[doc = "0x24 - Timestamp Counter Value Register"]
    pub mcan_tscv: crate::Reg<mcan_tscv::MCAN_TSCV_SPEC>,
    #[doc = "0x28 - Timeout Counter Configuration Register"]
    pub mcan_tocc: crate::Reg<mcan_tocc::MCAN_TOCC_SPEC>,
    #[doc = "0x2c - Timeout Counter Value Register"]
    pub mcan_tocv: crate::Reg<mcan_tocv::MCAN_TOCV_SPEC>,
    _reserved12: [u8; 0x10],
    #[doc = "0x40 - Error Counter Register"]
    pub mcan_ecr: crate::Reg<mcan_ecr::MCAN_ECR_SPEC>,
    #[doc = "0x44 - Protocol Status Register"]
    pub mcan_psr: crate::Reg<mcan_psr::MCAN_PSR_SPEC>,
    #[doc = "0x48 - Transmit Delay Compensation Register"]
    pub mcan_tdcr: crate::Reg<mcan_tdcr::MCAN_TDCR_SPEC>,
    _reserved15: [u8; 0x04],
    #[doc = "0x50 - Interrupt Register"]
    pub mcan_ir: crate::Reg<mcan_ir::MCAN_IR_SPEC>,
    #[doc = "0x54 - Interrupt Enable Register"]
    pub mcan_ie: crate::Reg<mcan_ie::MCAN_IE_SPEC>,
    #[doc = "0x58 - Interrupt Line Select Register"]
    pub mcan_ils: crate::Reg<mcan_ils::MCAN_ILS_SPEC>,
    #[doc = "0x5c - Interrupt Line Enable Register"]
    pub mcan_ile: crate::Reg<mcan_ile::MCAN_ILE_SPEC>,
    _reserved19: [u8; 0x20],
    #[doc = "0x80 - Global Filter Configuration Register"]
    pub mcan_gfc: crate::Reg<mcan_gfc::MCAN_GFC_SPEC>,
    #[doc = "0x84 - Standard ID Filter Configuration Register"]
    pub mcan_sidfc: crate::Reg<mcan_sidfc::MCAN_SIDFC_SPEC>,
    #[doc = "0x88 - Extended ID Filter Configuration Register"]
    pub mcan_xidfc: crate::Reg<mcan_xidfc::MCAN_XIDFC_SPEC>,
    _reserved22: [u8; 0x04],
    #[doc = "0x90 - Extended ID AND Mask Register"]
    pub mcan_xidam: crate::Reg<mcan_xidam::MCAN_XIDAM_SPEC>,
    #[doc = "0x94 - High Priority Message Status Register"]
    pub mcan_hpms: crate::Reg<mcan_hpms::MCAN_HPMS_SPEC>,
    #[doc = "0x98 - New Data 1 Register"]
    pub mcan_ndat1: crate::Reg<mcan_ndat1::MCAN_NDAT1_SPEC>,
    #[doc = "0x9c - New Data 2 Register"]
    pub mcan_ndat2: crate::Reg<mcan_ndat2::MCAN_NDAT2_SPEC>,
    #[doc = "0xa0 - Receive FIFO 0 Configuration Register"]
    pub mcan_rxf0c: crate::Reg<mcan_rxf0c::MCAN_RXF0C_SPEC>,
    #[doc = "0xa4 - Receive FIFO 0 Status Register"]
    pub mcan_rxf0s: crate::Reg<mcan_rxf0s::MCAN_RXF0S_SPEC>,
    #[doc = "0xa8 - Receive FIFO 0 Acknowledge Register"]
    pub mcan_rxf0a: crate::Reg<mcan_rxf0a::MCAN_RXF0A_SPEC>,
    #[doc = "0xac - Receive Rx Buffer Configuration Register"]
    pub mcan_rxbc: crate::Reg<mcan_rxbc::MCAN_RXBC_SPEC>,
    #[doc = "0xb0 - Receive FIFO 1 Configuration Register"]
    pub mcan_rxf1c: crate::Reg<mcan_rxf1c::MCAN_RXF1C_SPEC>,
    #[doc = "0xb4 - Receive FIFO 1 Status Register"]
    pub mcan_rxf1s: crate::Reg<mcan_rxf1s::MCAN_RXF1S_SPEC>,
    #[doc = "0xb8 - Receive FIFO 1 Acknowledge Register"]
    pub mcan_rxf1a: crate::Reg<mcan_rxf1a::MCAN_RXF1A_SPEC>,
    #[doc = "0xbc - Receive Buffer / FIFO Element Size Configuration Register"]
    pub mcan_rxesc: crate::Reg<mcan_rxesc::MCAN_RXESC_SPEC>,
    #[doc = "0xc0 - Transmit Buffer Configuration Register"]
    pub mcan_txbc: crate::Reg<mcan_txbc::MCAN_TXBC_SPEC>,
    #[doc = "0xc4 - Transmit FIFO/Queue Status Register"]
    pub mcan_txfqs: crate::Reg<mcan_txfqs::MCAN_TXFQS_SPEC>,
    #[doc = "0xc8 - Transmit Buffer Element Size Configuration Register"]
    pub mcan_txesc: crate::Reg<mcan_txesc::MCAN_TXESC_SPEC>,
    #[doc = "0xcc - Transmit Buffer Request Pending Register"]
    pub mcan_txbrp: crate::Reg<mcan_txbrp::MCAN_TXBRP_SPEC>,
    #[doc = "0xd0 - Transmit Buffer Add Request Register"]
    pub mcan_txbar: crate::Reg<mcan_txbar::MCAN_TXBAR_SPEC>,
    #[doc = "0xd4 - Transmit Buffer Cancellation Request Register"]
    pub mcan_txbcr: crate::Reg<mcan_txbcr::MCAN_TXBCR_SPEC>,
    #[doc = "0xd8 - Transmit Buffer Transmission Occurred Register"]
    pub mcan_txbto: crate::Reg<mcan_txbto::MCAN_TXBTO_SPEC>,
    #[doc = "0xdc - Transmit Buffer Cancellation Finished Register"]
    pub mcan_txbcf: crate::Reg<mcan_txbcf::MCAN_TXBCF_SPEC>,
    #[doc = "0xe0 - Transmit Buffer Transmission Interrupt Enable Register"]
    pub mcan_txbtie: crate::Reg<mcan_txbtie::MCAN_TXBTIE_SPEC>,
    #[doc = "0xe4 - Transmit Buffer Cancellation Finished Interrupt Enable Register"]
    pub mcan_txbcie: crate::Reg<mcan_txbcie::MCAN_TXBCIE_SPEC>,
    _reserved44: [u8; 0x08],
    #[doc = "0xf0 - Transmit Event FIFO Configuration Register"]
    pub mcan_txefc: crate::Reg<mcan_txefc::MCAN_TXEFC_SPEC>,
    #[doc = "0xf4 - Transmit Event FIFO Status Register"]
    pub mcan_txefs: crate::Reg<mcan_txefs::MCAN_TXEFS_SPEC>,
    #[doc = "0xf8 - Transmit Event FIFO Acknowledge Register"]
    pub mcan_txefa: crate::Reg<mcan_txefa::MCAN_TXEFA_SPEC>,
}
#[doc = "MCAN_CREL register accessor: an alias for `Reg<MCAN_CREL_SPEC>`"]
pub type MCAN_CREL = crate::Reg<mcan_crel::MCAN_CREL_SPEC>;
#[doc = "Core Release Register"]
pub mod mcan_crel;
#[doc = "MCAN_ENDN register accessor: an alias for `Reg<MCAN_ENDN_SPEC>`"]
pub type MCAN_ENDN = crate::Reg<mcan_endn::MCAN_ENDN_SPEC>;
#[doc = "Endian Register"]
pub mod mcan_endn;
#[doc = "MCAN_CUST register accessor: an alias for `Reg<MCAN_CUST_SPEC>`"]
pub type MCAN_CUST = crate::Reg<mcan_cust::MCAN_CUST_SPEC>;
#[doc = "Customer Register"]
pub mod mcan_cust;
#[doc = "MCAN_DBTP register accessor: an alias for `Reg<MCAN_DBTP_SPEC>`"]
pub type MCAN_DBTP = crate::Reg<mcan_dbtp::MCAN_DBTP_SPEC>;
#[doc = "Data Bit Timing and Prescaler Register"]
pub mod mcan_dbtp;
#[doc = "MCAN_TEST register accessor: an alias for `Reg<MCAN_TEST_SPEC>`"]
pub type MCAN_TEST = crate::Reg<mcan_test::MCAN_TEST_SPEC>;
#[doc = "Test Register"]
pub mod mcan_test;
#[doc = "MCAN_RWD register accessor: an alias for `Reg<MCAN_RWD_SPEC>`"]
pub type MCAN_RWD = crate::Reg<mcan_rwd::MCAN_RWD_SPEC>;
#[doc = "RAM Watchdog Register"]
pub mod mcan_rwd;
#[doc = "MCAN_CCCR register accessor: an alias for `Reg<MCAN_CCCR_SPEC>`"]
pub type MCAN_CCCR = crate::Reg<mcan_cccr::MCAN_CCCR_SPEC>;
#[doc = "CC Control Register"]
pub mod mcan_cccr;
#[doc = "MCAN_NBTP register accessor: an alias for `Reg<MCAN_NBTP_SPEC>`"]
pub type MCAN_NBTP = crate::Reg<mcan_nbtp::MCAN_NBTP_SPEC>;
#[doc = "Nominal Bit Timing and Prescaler Register"]
pub mod mcan_nbtp;
#[doc = "MCAN_TSCC register accessor: an alias for `Reg<MCAN_TSCC_SPEC>`"]
pub type MCAN_TSCC = crate::Reg<mcan_tscc::MCAN_TSCC_SPEC>;
#[doc = "Timestamp Counter Configuration Register"]
pub mod mcan_tscc;
#[doc = "MCAN_TSCV register accessor: an alias for `Reg<MCAN_TSCV_SPEC>`"]
pub type MCAN_TSCV = crate::Reg<mcan_tscv::MCAN_TSCV_SPEC>;
#[doc = "Timestamp Counter Value Register"]
pub mod mcan_tscv;
#[doc = "MCAN_TOCC register accessor: an alias for `Reg<MCAN_TOCC_SPEC>`"]
pub type MCAN_TOCC = crate::Reg<mcan_tocc::MCAN_TOCC_SPEC>;
#[doc = "Timeout Counter Configuration Register"]
pub mod mcan_tocc;
#[doc = "MCAN_TOCV register accessor: an alias for `Reg<MCAN_TOCV_SPEC>`"]
pub type MCAN_TOCV = crate::Reg<mcan_tocv::MCAN_TOCV_SPEC>;
#[doc = "Timeout Counter Value Register"]
pub mod mcan_tocv;
#[doc = "MCAN_ECR register accessor: an alias for `Reg<MCAN_ECR_SPEC>`"]
pub type MCAN_ECR = crate::Reg<mcan_ecr::MCAN_ECR_SPEC>;
#[doc = "Error Counter Register"]
pub mod mcan_ecr;
#[doc = "MCAN_PSR register accessor: an alias for `Reg<MCAN_PSR_SPEC>`"]
pub type MCAN_PSR = crate::Reg<mcan_psr::MCAN_PSR_SPEC>;
#[doc = "Protocol Status Register"]
pub mod mcan_psr;
#[doc = "MCAN_TDCR register accessor: an alias for `Reg<MCAN_TDCR_SPEC>`"]
pub type MCAN_TDCR = crate::Reg<mcan_tdcr::MCAN_TDCR_SPEC>;
#[doc = "Transmit Delay Compensation Register"]
pub mod mcan_tdcr;
#[doc = "MCAN_IR register accessor: an alias for `Reg<MCAN_IR_SPEC>`"]
pub type MCAN_IR = crate::Reg<mcan_ir::MCAN_IR_SPEC>;
#[doc = "Interrupt Register"]
pub mod mcan_ir;
#[doc = "MCAN_IE register accessor: an alias for `Reg<MCAN_IE_SPEC>`"]
pub type MCAN_IE = crate::Reg<mcan_ie::MCAN_IE_SPEC>;
#[doc = "Interrupt Enable Register"]
pub mod mcan_ie;
#[doc = "MCAN_ILS register accessor: an alias for `Reg<MCAN_ILS_SPEC>`"]
pub type MCAN_ILS = crate::Reg<mcan_ils::MCAN_ILS_SPEC>;
#[doc = "Interrupt Line Select Register"]
pub mod mcan_ils;
#[doc = "MCAN_ILE register accessor: an alias for `Reg<MCAN_ILE_SPEC>`"]
pub type MCAN_ILE = crate::Reg<mcan_ile::MCAN_ILE_SPEC>;
#[doc = "Interrupt Line Enable Register"]
pub mod mcan_ile;
#[doc = "MCAN_GFC register accessor: an alias for `Reg<MCAN_GFC_SPEC>`"]
pub type MCAN_GFC = crate::Reg<mcan_gfc::MCAN_GFC_SPEC>;
#[doc = "Global Filter Configuration Register"]
pub mod mcan_gfc;
#[doc = "MCAN_SIDFC register accessor: an alias for `Reg<MCAN_SIDFC_SPEC>`"]
pub type MCAN_SIDFC = crate::Reg<mcan_sidfc::MCAN_SIDFC_SPEC>;
#[doc = "Standard ID Filter Configuration Register"]
pub mod mcan_sidfc;
#[doc = "MCAN_XIDFC register accessor: an alias for `Reg<MCAN_XIDFC_SPEC>`"]
pub type MCAN_XIDFC = crate::Reg<mcan_xidfc::MCAN_XIDFC_SPEC>;
#[doc = "Extended ID Filter Configuration Register"]
pub mod mcan_xidfc;
#[doc = "MCAN_XIDAM register accessor: an alias for `Reg<MCAN_XIDAM_SPEC>`"]
pub type MCAN_XIDAM = crate::Reg<mcan_xidam::MCAN_XIDAM_SPEC>;
#[doc = "Extended ID AND Mask Register"]
pub mod mcan_xidam;
#[doc = "MCAN_HPMS register accessor: an alias for `Reg<MCAN_HPMS_SPEC>`"]
pub type MCAN_HPMS = crate::Reg<mcan_hpms::MCAN_HPMS_SPEC>;
#[doc = "High Priority Message Status Register"]
pub mod mcan_hpms;
#[doc = "MCAN_NDAT1 register accessor: an alias for `Reg<MCAN_NDAT1_SPEC>`"]
pub type MCAN_NDAT1 = crate::Reg<mcan_ndat1::MCAN_NDAT1_SPEC>;
#[doc = "New Data 1 Register"]
pub mod mcan_ndat1;
#[doc = "MCAN_NDAT2 register accessor: an alias for `Reg<MCAN_NDAT2_SPEC>`"]
pub type MCAN_NDAT2 = crate::Reg<mcan_ndat2::MCAN_NDAT2_SPEC>;
#[doc = "New Data 2 Register"]
pub mod mcan_ndat2;
#[doc = "MCAN_RXF0C register accessor: an alias for `Reg<MCAN_RXF0C_SPEC>`"]
pub type MCAN_RXF0C = crate::Reg<mcan_rxf0c::MCAN_RXF0C_SPEC>;
#[doc = "Receive FIFO 0 Configuration Register"]
pub mod mcan_rxf0c;
#[doc = "MCAN_RXF0S register accessor: an alias for `Reg<MCAN_RXF0S_SPEC>`"]
pub type MCAN_RXF0S = crate::Reg<mcan_rxf0s::MCAN_RXF0S_SPEC>;
#[doc = "Receive FIFO 0 Status Register"]
pub mod mcan_rxf0s;
#[doc = "MCAN_RXF0A register accessor: an alias for `Reg<MCAN_RXF0A_SPEC>`"]
pub type MCAN_RXF0A = crate::Reg<mcan_rxf0a::MCAN_RXF0A_SPEC>;
#[doc = "Receive FIFO 0 Acknowledge Register"]
pub mod mcan_rxf0a;
#[doc = "MCAN_RXBC register accessor: an alias for `Reg<MCAN_RXBC_SPEC>`"]
pub type MCAN_RXBC = crate::Reg<mcan_rxbc::MCAN_RXBC_SPEC>;
#[doc = "Receive Rx Buffer Configuration Register"]
pub mod mcan_rxbc;
#[doc = "MCAN_RXF1C register accessor: an alias for `Reg<MCAN_RXF1C_SPEC>`"]
pub type MCAN_RXF1C = crate::Reg<mcan_rxf1c::MCAN_RXF1C_SPEC>;
#[doc = "Receive FIFO 1 Configuration Register"]
pub mod mcan_rxf1c;
#[doc = "MCAN_RXF1S register accessor: an alias for `Reg<MCAN_RXF1S_SPEC>`"]
pub type MCAN_RXF1S = crate::Reg<mcan_rxf1s::MCAN_RXF1S_SPEC>;
#[doc = "Receive FIFO 1 Status Register"]
pub mod mcan_rxf1s;
#[doc = "MCAN_RXF1A register accessor: an alias for `Reg<MCAN_RXF1A_SPEC>`"]
pub type MCAN_RXF1A = crate::Reg<mcan_rxf1a::MCAN_RXF1A_SPEC>;
#[doc = "Receive FIFO 1 Acknowledge Register"]
pub mod mcan_rxf1a;
#[doc = "MCAN_RXESC register accessor: an alias for `Reg<MCAN_RXESC_SPEC>`"]
pub type MCAN_RXESC = crate::Reg<mcan_rxesc::MCAN_RXESC_SPEC>;
#[doc = "Receive Buffer / FIFO Element Size Configuration Register"]
pub mod mcan_rxesc;
#[doc = "MCAN_TXBC register accessor: an alias for `Reg<MCAN_TXBC_SPEC>`"]
pub type MCAN_TXBC = crate::Reg<mcan_txbc::MCAN_TXBC_SPEC>;
#[doc = "Transmit Buffer Configuration Register"]
pub mod mcan_txbc;
#[doc = "MCAN_TXFQS register accessor: an alias for `Reg<MCAN_TXFQS_SPEC>`"]
pub type MCAN_TXFQS = crate::Reg<mcan_txfqs::MCAN_TXFQS_SPEC>;
#[doc = "Transmit FIFO/Queue Status Register"]
pub mod mcan_txfqs;
#[doc = "MCAN_TXESC register accessor: an alias for `Reg<MCAN_TXESC_SPEC>`"]
pub type MCAN_TXESC = crate::Reg<mcan_txesc::MCAN_TXESC_SPEC>;
#[doc = "Transmit Buffer Element Size Configuration Register"]
pub mod mcan_txesc;
#[doc = "MCAN_TXBRP register accessor: an alias for `Reg<MCAN_TXBRP_SPEC>`"]
pub type MCAN_TXBRP = crate::Reg<mcan_txbrp::MCAN_TXBRP_SPEC>;
#[doc = "Transmit Buffer Request Pending Register"]
pub mod mcan_txbrp;
#[doc = "MCAN_TXBAR register accessor: an alias for `Reg<MCAN_TXBAR_SPEC>`"]
pub type MCAN_TXBAR = crate::Reg<mcan_txbar::MCAN_TXBAR_SPEC>;
#[doc = "Transmit Buffer Add Request Register"]
pub mod mcan_txbar;
#[doc = "MCAN_TXBCR register accessor: an alias for `Reg<MCAN_TXBCR_SPEC>`"]
pub type MCAN_TXBCR = crate::Reg<mcan_txbcr::MCAN_TXBCR_SPEC>;
#[doc = "Transmit Buffer Cancellation Request Register"]
pub mod mcan_txbcr;
#[doc = "MCAN_TXBTO register accessor: an alias for `Reg<MCAN_TXBTO_SPEC>`"]
pub type MCAN_TXBTO = crate::Reg<mcan_txbto::MCAN_TXBTO_SPEC>;
#[doc = "Transmit Buffer Transmission Occurred Register"]
pub mod mcan_txbto;
#[doc = "MCAN_TXBCF register accessor: an alias for `Reg<MCAN_TXBCF_SPEC>`"]
pub type MCAN_TXBCF = crate::Reg<mcan_txbcf::MCAN_TXBCF_SPEC>;
#[doc = "Transmit Buffer Cancellation Finished Register"]
pub mod mcan_txbcf;
#[doc = "MCAN_TXBTIE register accessor: an alias for `Reg<MCAN_TXBTIE_SPEC>`"]
pub type MCAN_TXBTIE = crate::Reg<mcan_txbtie::MCAN_TXBTIE_SPEC>;
#[doc = "Transmit Buffer Transmission Interrupt Enable Register"]
pub mod mcan_txbtie;
#[doc = "MCAN_TXBCIE register accessor: an alias for `Reg<MCAN_TXBCIE_SPEC>`"]
pub type MCAN_TXBCIE = crate::Reg<mcan_txbcie::MCAN_TXBCIE_SPEC>;
#[doc = "Transmit Buffer Cancellation Finished Interrupt Enable Register"]
pub mod mcan_txbcie;
#[doc = "MCAN_TXEFC register accessor: an alias for `Reg<MCAN_TXEFC_SPEC>`"]
pub type MCAN_TXEFC = crate::Reg<mcan_txefc::MCAN_TXEFC_SPEC>;
#[doc = "Transmit Event FIFO Configuration Register"]
pub mod mcan_txefc;
#[doc = "MCAN_TXEFS register accessor: an alias for `Reg<MCAN_TXEFS_SPEC>`"]
pub type MCAN_TXEFS = crate::Reg<mcan_txefs::MCAN_TXEFS_SPEC>;
#[doc = "Transmit Event FIFO Status Register"]
pub mod mcan_txefs;
#[doc = "MCAN_TXEFA register accessor: an alias for `Reg<MCAN_TXEFA_SPEC>`"]
pub type MCAN_TXEFA = crate::Reg<mcan_txefa::MCAN_TXEFA_SPEC>;
#[doc = "Transmit Event FIFO Acknowledge Register"]
pub mod mcan_txefa;
