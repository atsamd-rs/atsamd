#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Network Control Register"]
    pub gmac_ncr: crate::Reg<gmac_ncr::GMAC_NCR_SPEC>,
    #[doc = "0x04 - Network Configuration Register"]
    pub gmac_ncfgr: crate::Reg<gmac_ncfgr::GMAC_NCFGR_SPEC>,
    #[doc = "0x08 - Network Status Register"]
    pub gmac_nsr: crate::Reg<gmac_nsr::GMAC_NSR_SPEC>,
    #[doc = "0x0c - User Register"]
    pub gmac_ur: crate::Reg<gmac_ur::GMAC_UR_SPEC>,
    #[doc = "0x10 - DMA Configuration Register"]
    pub gmac_dcfgr: crate::Reg<gmac_dcfgr::GMAC_DCFGR_SPEC>,
    #[doc = "0x14 - Transmit Status Register"]
    pub gmac_tsr: crate::Reg<gmac_tsr::GMAC_TSR_SPEC>,
    #[doc = "0x18 - Receive Buffer Queue Base Address Register"]
    pub gmac_rbqb: crate::Reg<gmac_rbqb::GMAC_RBQB_SPEC>,
    #[doc = "0x1c - Transmit Buffer Queue Base Address Register"]
    pub gmac_tbqb: crate::Reg<gmac_tbqb::GMAC_TBQB_SPEC>,
    #[doc = "0x20 - Receive Status Register"]
    pub gmac_rsr: crate::Reg<gmac_rsr::GMAC_RSR_SPEC>,
    #[doc = "0x24 - Interrupt Status Register"]
    pub gmac_isr: crate::Reg<gmac_isr::GMAC_ISR_SPEC>,
    #[doc = "0x28 - Interrupt Enable Register"]
    pub gmac_ier: crate::Reg<gmac_ier::GMAC_IER_SPEC>,
    #[doc = "0x2c - Interrupt Disable Register"]
    pub gmac_idr: crate::Reg<gmac_idr::GMAC_IDR_SPEC>,
    #[doc = "0x30 - Interrupt Mask Register"]
    pub gmac_imr: crate::Reg<gmac_imr::GMAC_IMR_SPEC>,
    #[doc = "0x34 - PHY Maintenance Register"]
    pub gmac_man: crate::Reg<gmac_man::GMAC_MAN_SPEC>,
    #[doc = "0x38 - Received Pause Quantum Register"]
    pub gmac_rpq: crate::Reg<gmac_rpq::GMAC_RPQ_SPEC>,
    #[doc = "0x3c - Transmit Pause Quantum Register"]
    pub gmac_tpq: crate::Reg<gmac_tpq::GMAC_TPQ_SPEC>,
    #[doc = "0x40 - TX Partial Store and Forward Register"]
    pub gmac_tpsf: crate::Reg<gmac_tpsf::GMAC_TPSF_SPEC>,
    #[doc = "0x44 - RX Partial Store and Forward Register"]
    pub gmac_rpsf: crate::Reg<gmac_rpsf::GMAC_RPSF_SPEC>,
    #[doc = "0x48 - RX Jumbo Frame Max Length Register"]
    pub gmac_rjfml: crate::Reg<gmac_rjfml::GMAC_RJFML_SPEC>,
    _reserved19: [u8; 0x34],
    #[doc = "0x80 - Hash Register Bottom"]
    pub gmac_hrb: crate::Reg<gmac_hrb::GMAC_HRB_SPEC>,
    #[doc = "0x84 - Hash Register Top"]
    pub gmac_hrt: crate::Reg<gmac_hrt::GMAC_HRT_SPEC>,
    #[doc = "0x88..0x90 - Specific Address 1 Bottom Register"]
    pub gmac_sa1: GMAC_SA,
    #[doc = "0x90..0x98 - Specific Address 1 Bottom Register"]
    pub gmac_sa2: GMAC_SA,
    #[doc = "0x98..0xa0 - Specific Address 1 Bottom Register"]
    pub gmac_sa3: GMAC_SA,
    #[doc = "0xa0..0xa8 - Specific Address 1 Bottom Register"]
    pub gmac_sa4: GMAC_SA,
    #[doc = "0xa8 - Type ID Match 1 Register"]
    pub gmac_tidm1: crate::Reg<gmac_tidm1::GMAC_TIDM1_SPEC>,
    #[doc = "0xac - Type ID Match 2 Register"]
    pub gmac_tidm2: crate::Reg<gmac_tidm2::GMAC_TIDM2_SPEC>,
    #[doc = "0xb0 - Type ID Match 3 Register"]
    pub gmac_tidm3: crate::Reg<gmac_tidm3::GMAC_TIDM3_SPEC>,
    #[doc = "0xb4 - Type ID Match 4 Register"]
    pub gmac_tidm4: crate::Reg<gmac_tidm4::GMAC_TIDM4_SPEC>,
    #[doc = "0xb8 - Wake on LAN Register"]
    pub gmac_wol: crate::Reg<gmac_wol::GMAC_WOL_SPEC>,
    #[doc = "0xbc - IPG Stretch Register"]
    pub gmac_ipgs: crate::Reg<gmac_ipgs::GMAC_IPGS_SPEC>,
    #[doc = "0xc0 - Stacked VLAN Register"]
    pub gmac_svlan: crate::Reg<gmac_svlan::GMAC_SVLAN_SPEC>,
    #[doc = "0xc4 - Transmit PFC Pause Register"]
    pub gmac_tpfcp: crate::Reg<gmac_tpfcp::GMAC_TPFCP_SPEC>,
    #[doc = "0xc8 - Specific Address 1 Mask Bottom Register"]
    pub gmac_samb1: crate::Reg<gmac_samb1::GMAC_SAMB1_SPEC>,
    #[doc = "0xcc - Specific Address 1 Mask Top Register"]
    pub gmac_samt1: crate::Reg<gmac_samt1::GMAC_SAMT1_SPEC>,
    _reserved35: [u8; 0x0c],
    #[doc = "0xdc - 1588 Timer Nanosecond Comparison Register"]
    pub gmac_nsc: crate::Reg<gmac_nsc::GMAC_NSC_SPEC>,
    #[doc = "0xe0 - 1588 Timer Second Comparison Low Register"]
    pub gmac_scl: crate::Reg<gmac_scl::GMAC_SCL_SPEC>,
    #[doc = "0xe4 - 1588 Timer Second Comparison High Register"]
    pub gmac_sch: crate::Reg<gmac_sch::GMAC_SCH_SPEC>,
    #[doc = "0xe8 - PTP Event Frame Transmitted Seconds High Register"]
    pub gmac_eftsh: crate::Reg<gmac_eftsh::GMAC_EFTSH_SPEC>,
    #[doc = "0xec - PTP Event Frame Received Seconds High Register"]
    pub gmac_efrsh: crate::Reg<gmac_efrsh::GMAC_EFRSH_SPEC>,
    #[doc = "0xf0 - PTP Peer Event Frame Transmitted Seconds High Register"]
    pub gmac_peftsh: crate::Reg<gmac_peftsh::GMAC_PEFTSH_SPEC>,
    #[doc = "0xf4 - PTP Peer Event Frame Received Seconds High Register"]
    pub gmac_pefrsh: crate::Reg<gmac_pefrsh::GMAC_PEFRSH_SPEC>,
    _reserved42: [u8; 0x08],
    #[doc = "0x100 - Octets Transmitted Low Register"]
    pub gmac_otlo: crate::Reg<gmac_otlo::GMAC_OTLO_SPEC>,
    #[doc = "0x104 - Octets Transmitted High Register"]
    pub gmac_othi: crate::Reg<gmac_othi::GMAC_OTHI_SPEC>,
    #[doc = "0x108 - Frames Transmitted Register"]
    pub gmac_ft: crate::Reg<gmac_ft::GMAC_FT_SPEC>,
    #[doc = "0x10c - Broadcast Frames Transmitted Register"]
    pub gmac_bcft: crate::Reg<gmac_bcft::GMAC_BCFT_SPEC>,
    #[doc = "0x110 - Multicast Frames Transmitted Register"]
    pub gmac_mft: crate::Reg<gmac_mft::GMAC_MFT_SPEC>,
    #[doc = "0x114 - Pause Frames Transmitted Register"]
    pub gmac_pft: crate::Reg<gmac_pft::GMAC_PFT_SPEC>,
    #[doc = "0x118 - 64 Byte Frames Transmitted Register"]
    pub gmac_bft64: crate::Reg<gmac_bft64::GMAC_BFT64_SPEC>,
    #[doc = "0x11c - 65 to 127 Byte Frames Transmitted Register"]
    pub gmac_tbft127: crate::Reg<gmac_tbft127::GMAC_TBFT127_SPEC>,
    #[doc = "0x120 - 128 to 255 Byte Frames Transmitted Register"]
    pub gmac_tbft255: crate::Reg<gmac_tbft255::GMAC_TBFT255_SPEC>,
    #[doc = "0x124 - 256 to 511 Byte Frames Transmitted Register"]
    pub gmac_tbft511: crate::Reg<gmac_tbft511::GMAC_TBFT511_SPEC>,
    #[doc = "0x128 - 512 to 1023 Byte Frames Transmitted Register"]
    pub gmac_tbft1023: crate::Reg<gmac_tbft1023::GMAC_TBFT1023_SPEC>,
    #[doc = "0x12c - 1024 to 1518 Byte Frames Transmitted Register"]
    pub gmac_tbft1518: crate::Reg<gmac_tbft1518::GMAC_TBFT1518_SPEC>,
    #[doc = "0x130 - Greater Than 1518 Byte Frames Transmitted Register"]
    pub gmac_gtbft1518: crate::Reg<gmac_gtbft1518::GMAC_GTBFT1518_SPEC>,
    #[doc = "0x134 - Transmit Underruns Register"]
    pub gmac_tur: crate::Reg<gmac_tur::GMAC_TUR_SPEC>,
    #[doc = "0x138 - Single Collision Frames Register"]
    pub gmac_scf: crate::Reg<gmac_scf::GMAC_SCF_SPEC>,
    #[doc = "0x13c - Multiple Collision Frames Register"]
    pub gmac_mcf: crate::Reg<gmac_mcf::GMAC_MCF_SPEC>,
    #[doc = "0x140 - Excessive Collisions Register"]
    pub gmac_ec: crate::Reg<gmac_ec::GMAC_EC_SPEC>,
    #[doc = "0x144 - Late Collisions Register"]
    pub gmac_lc: crate::Reg<gmac_lc::GMAC_LC_SPEC>,
    #[doc = "0x148 - Deferred Transmission Frames Register"]
    pub gmac_dtf: crate::Reg<gmac_dtf::GMAC_DTF_SPEC>,
    #[doc = "0x14c - Carrier Sense Errors Register"]
    pub gmac_cse: crate::Reg<gmac_cse::GMAC_CSE_SPEC>,
    #[doc = "0x150 - Octets Received Low Received Register"]
    pub gmac_orlo: crate::Reg<gmac_orlo::GMAC_ORLO_SPEC>,
    #[doc = "0x154 - Octets Received High Received Register"]
    pub gmac_orhi: crate::Reg<gmac_orhi::GMAC_ORHI_SPEC>,
    #[doc = "0x158 - Frames Received Register"]
    pub gmac_fr: crate::Reg<gmac_fr::GMAC_FR_SPEC>,
    #[doc = "0x15c - Broadcast Frames Received Register"]
    pub gmac_bcfr: crate::Reg<gmac_bcfr::GMAC_BCFR_SPEC>,
    #[doc = "0x160 - Multicast Frames Received Register"]
    pub gmac_mfr: crate::Reg<gmac_mfr::GMAC_MFR_SPEC>,
    #[doc = "0x164 - Pause Frames Received Register"]
    pub gmac_pfr: crate::Reg<gmac_pfr::GMAC_PFR_SPEC>,
    #[doc = "0x168 - 64 Byte Frames Received Register"]
    pub gmac_bfr64: crate::Reg<gmac_bfr64::GMAC_BFR64_SPEC>,
    #[doc = "0x16c - 65 to 127 Byte Frames Received Register"]
    pub gmac_tbfr127: crate::Reg<gmac_tbfr127::GMAC_TBFR127_SPEC>,
    #[doc = "0x170 - 128 to 255 Byte Frames Received Register"]
    pub gmac_tbfr255: crate::Reg<gmac_tbfr255::GMAC_TBFR255_SPEC>,
    #[doc = "0x174 - 256 to 511 Byte Frames Received Register"]
    pub gmac_tbfr511: crate::Reg<gmac_tbfr511::GMAC_TBFR511_SPEC>,
    #[doc = "0x178 - 512 to 1023 Byte Frames Received Register"]
    pub gmac_tbfr1023: crate::Reg<gmac_tbfr1023::GMAC_TBFR1023_SPEC>,
    #[doc = "0x17c - 1024 to 1518 Byte Frames Received Register"]
    pub gmac_tbfr1518: crate::Reg<gmac_tbfr1518::GMAC_TBFR1518_SPEC>,
    #[doc = "0x180 - 1519 to Maximum Byte Frames Received Register"]
    pub gmac_tmxbfr: crate::Reg<gmac_tmxbfr::GMAC_TMXBFR_SPEC>,
    #[doc = "0x184 - Undersize Frames Received Register"]
    pub gmac_ufr: crate::Reg<gmac_ufr::GMAC_UFR_SPEC>,
    #[doc = "0x188 - Oversize Frames Received Register"]
    pub gmac_ofr: crate::Reg<gmac_ofr::GMAC_OFR_SPEC>,
    #[doc = "0x18c - Jabbers Received Register"]
    pub gmac_jr: crate::Reg<gmac_jr::GMAC_JR_SPEC>,
    #[doc = "0x190 - Frame Check Sequence Errors Register"]
    pub gmac_fcse: crate::Reg<gmac_fcse::GMAC_FCSE_SPEC>,
    #[doc = "0x194 - Length Field Frame Errors Register"]
    pub gmac_lffe: crate::Reg<gmac_lffe::GMAC_LFFE_SPEC>,
    #[doc = "0x198 - Receive Symbol Errors Register"]
    pub gmac_rse: crate::Reg<gmac_rse::GMAC_RSE_SPEC>,
    #[doc = "0x19c - Alignment Errors Register"]
    pub gmac_ae: crate::Reg<gmac_ae::GMAC_AE_SPEC>,
    #[doc = "0x1a0 - Receive Resource Errors Register"]
    pub gmac_rre: crate::Reg<gmac_rre::GMAC_RRE_SPEC>,
    #[doc = "0x1a4 - Receive Overrun Register"]
    pub gmac_roe: crate::Reg<gmac_roe::GMAC_ROE_SPEC>,
    #[doc = "0x1a8 - IP Header Checksum Errors Register"]
    pub gmac_ihce: crate::Reg<gmac_ihce::GMAC_IHCE_SPEC>,
    #[doc = "0x1ac - TCP Checksum Errors Register"]
    pub gmac_tce: crate::Reg<gmac_tce::GMAC_TCE_SPEC>,
    #[doc = "0x1b0 - UDP Checksum Errors Register"]
    pub gmac_uce: crate::Reg<gmac_uce::GMAC_UCE_SPEC>,
    _reserved87: [u8; 0x08],
    #[doc = "0x1bc - 1588 Timer Increment Sub-nanoseconds Register"]
    pub gmac_tisubn: crate::Reg<gmac_tisubn::GMAC_TISUBN_SPEC>,
    #[doc = "0x1c0 - 1588 Timer Seconds High Register"]
    pub gmac_tsh: crate::Reg<gmac_tsh::GMAC_TSH_SPEC>,
    _reserved89: [u8; 0x0c],
    #[doc = "0x1d0 - 1588 Timer Seconds Low Register"]
    pub gmac_tsl: crate::Reg<gmac_tsl::GMAC_TSL_SPEC>,
    #[doc = "0x1d4 - 1588 Timer Nanoseconds Register"]
    pub gmac_tn: crate::Reg<gmac_tn::GMAC_TN_SPEC>,
    #[doc = "0x1d8 - 1588 Timer Adjust Register"]
    pub gmac_ta: crate::Reg<gmac_ta::GMAC_TA_SPEC>,
    #[doc = "0x1dc - 1588 Timer Increment Register"]
    pub gmac_ti: crate::Reg<gmac_ti::GMAC_TI_SPEC>,
    #[doc = "0x1e0 - PTP Event Frame Transmitted Seconds Low Register"]
    pub gmac_eftsl: crate::Reg<gmac_eftsl::GMAC_EFTSL_SPEC>,
    #[doc = "0x1e4 - PTP Event Frame Transmitted Nanoseconds Register"]
    pub gmac_eftn: crate::Reg<gmac_eftn::GMAC_EFTN_SPEC>,
    #[doc = "0x1e8 - PTP Event Frame Received Seconds Low Register"]
    pub gmac_efrsl: crate::Reg<gmac_efrsl::GMAC_EFRSL_SPEC>,
    #[doc = "0x1ec - PTP Event Frame Received Nanoseconds Register"]
    pub gmac_efrn: crate::Reg<gmac_efrn::GMAC_EFRN_SPEC>,
    #[doc = "0x1f0 - PTP Peer Event Frame Transmitted Seconds Low Register"]
    pub gmac_peftsl: crate::Reg<gmac_peftsl::GMAC_PEFTSL_SPEC>,
    #[doc = "0x1f4 - PTP Peer Event Frame Transmitted Nanoseconds Register"]
    pub gmac_peftn: crate::Reg<gmac_peftn::GMAC_PEFTN_SPEC>,
    #[doc = "0x1f8 - PTP Peer Event Frame Received Seconds Low Register"]
    pub gmac_pefrsl: crate::Reg<gmac_pefrsl::GMAC_PEFRSL_SPEC>,
    #[doc = "0x1fc - PTP Peer Event Frame Received Nanoseconds Register"]
    pub gmac_pefrn: crate::Reg<gmac_pefrn::GMAC_PEFRN_SPEC>,
    _reserved101: [u8; 0x70],
    #[doc = "0x270 - Received LPI Transitions"]
    pub gmac_rxlpi: crate::Reg<gmac_rxlpi::GMAC_RXLPI_SPEC>,
    #[doc = "0x274 - Received LPI Time"]
    pub gmac_rxlpitime: crate::Reg<gmac_rxlpitime::GMAC_RXLPITIME_SPEC>,
    #[doc = "0x278 - Transmit LPI Transitions"]
    pub gmac_txlpi: crate::Reg<gmac_txlpi::GMAC_TXLPI_SPEC>,
    #[doc = "0x27c - Transmit LPI Time"]
    pub gmac_txlpitime: crate::Reg<gmac_txlpitime::GMAC_TXLPITIME_SPEC>,
    _reserved105: [u8; 0x0180],
    #[doc = "0x400..0x414 - Interrupt Status Register Priority Queue (1..5)"]
    pub gmac_isrpq: [crate::Reg<gmac_isrpq::GMAC_ISRPQ_SPEC>; 5],
    _reserved106: [u8; 0x2c],
    #[doc = "0x440..0x454 - Transmit Buffer Queue Base Address Register Priority Queue (1..5)"]
    pub gmac_tbqbapq: [crate::Reg<gmac_tbqbapq::GMAC_TBQBAPQ_SPEC>; 5],
    _reserved107: [u8; 0x2c],
    #[doc = "0x480..0x494 - Receive Buffer Queue Base Address Register Priority Queue (1..5)"]
    pub gmac_rbqbapq: [crate::Reg<gmac_rbqbapq::GMAC_RBQBAPQ_SPEC>; 5],
    _reserved108: [u8; 0x0c],
    #[doc = "0x4a0..0x4b4 - Receive Buffer Size Register Priority Queue (1..5)"]
    pub gmac_rbsrpq: [crate::Reg<gmac_rbsrpq::GMAC_RBSRPQ_SPEC>; 5],
    _reserved109: [u8; 0x08],
    #[doc = "0x4bc - Credit-Based Shaping Control Register"]
    pub gmac_cbscr: crate::Reg<gmac_cbscr::GMAC_CBSCR_SPEC>,
    #[doc = "0x4c0 - Credit-Based Shaping IdleSlope Register for Queue A"]
    pub gmac_cbsisqa: crate::Reg<gmac_cbsisqa::GMAC_CBSISQA_SPEC>,
    #[doc = "0x4c4 - Credit-Based Shaping IdleSlope Register for Queue B"]
    pub gmac_cbsisqb: crate::Reg<gmac_cbsisqb::GMAC_CBSISQB_SPEC>,
    _reserved112: [u8; 0x38],
    #[doc = "0x500..0x510 - Screening Type 1 Register Priority Queue"]
    pub gmac_st1rpq: [crate::Reg<gmac_st1rpq::GMAC_ST1RPQ_SPEC>; 4],
    _reserved113: [u8; 0x30],
    #[doc = "0x540..0x560 - Screening Type 2 Register Priority Queue"]
    pub gmac_st2rpq: [crate::Reg<gmac_st2rpq::GMAC_ST2RPQ_SPEC>; 8],
    _reserved114: [u8; 0xa0],
    #[doc = "0x600..0x614 - Interrupt Enable Register Priority Queue (1..5)"]
    pub gmac_ierpq: [crate::Reg<gmac_ierpq::GMAC_IERPQ_SPEC>; 5],
    _reserved115: [u8; 0x0c],
    #[doc = "0x620..0x634 - Interrupt Disable Register Priority Queue (1..5)"]
    pub gmac_idrpq: [crate::Reg<gmac_idrpq::GMAC_IDRPQ_SPEC>; 5],
    _reserved116: [u8; 0x0c],
    #[doc = "0x640..0x654 - Interrupt Mask Register Priority Queue (1..5)"]
    pub gmac_imrpq: [crate::Reg<gmac_imrpq::GMAC_IMRPQ_SPEC>; 5],
    _reserved117: [u8; 0x8c],
    #[doc = "0x6e0..0x6f0 - Screening Type 2 Ethertype Register"]
    pub gmac_st2er: [crate::Reg<gmac_st2er::GMAC_ST2ER_SPEC>; 4],
    _reserved118: [u8; 0x10],
    #[doc = "0x700..0x7c0 - Screening Type 2 Compare Word 0 Register"]
    pub gmac_st2cw: [GMAC_ST2CW; 24],
}
#[doc = r"Register block"]
#[repr(C)]
pub struct GMAC_SA {
    #[doc = "0x00 - Specific Address 1 Bottom Register"]
    pub gmac_sab: crate::Reg<self::gmac_sa::gmac_sab::GMAC_SAB_SPEC>,
    #[doc = "0x04 - Specific Address 1 Top Register"]
    pub gmac_sat: crate::Reg<self::gmac_sa::gmac_sat::GMAC_SAT_SPEC>,
}
#[doc = r"Register block"]
#[doc = "Specific Address 1 Bottom Register"]
pub mod gmac_sa;
#[doc = r"Register block"]
#[repr(C)]
pub struct GMAC_ST2CW {
    #[doc = "0x00 - Screening Type 2 Compare Word 0 Register"]
    pub gmac_st2cw0: crate::Reg<self::gmac_st2cw::gmac_st2cw0::GMAC_ST2CW0_SPEC>,
    #[doc = "0x04 - Screening Type 2 Compare Word 1 Register"]
    pub gmac_st2cw1: crate::Reg<self::gmac_st2cw::gmac_st2cw1::GMAC_ST2CW1_SPEC>,
}
#[doc = r"Register block"]
#[doc = "Screening Type 2 Compare Word 0 Register"]
pub mod gmac_st2cw;
#[doc = "GMAC_NCR register accessor: an alias for `Reg<GMAC_NCR_SPEC>`"]
pub type GMAC_NCR = crate::Reg<gmac_ncr::GMAC_NCR_SPEC>;
#[doc = "Network Control Register"]
pub mod gmac_ncr;
#[doc = "GMAC_NCFGR register accessor: an alias for `Reg<GMAC_NCFGR_SPEC>`"]
pub type GMAC_NCFGR = crate::Reg<gmac_ncfgr::GMAC_NCFGR_SPEC>;
#[doc = "Network Configuration Register"]
pub mod gmac_ncfgr;
#[doc = "GMAC_NSR register accessor: an alias for `Reg<GMAC_NSR_SPEC>`"]
pub type GMAC_NSR = crate::Reg<gmac_nsr::GMAC_NSR_SPEC>;
#[doc = "Network Status Register"]
pub mod gmac_nsr;
#[doc = "GMAC_UR register accessor: an alias for `Reg<GMAC_UR_SPEC>`"]
pub type GMAC_UR = crate::Reg<gmac_ur::GMAC_UR_SPEC>;
#[doc = "User Register"]
pub mod gmac_ur;
#[doc = "GMAC_DCFGR register accessor: an alias for `Reg<GMAC_DCFGR_SPEC>`"]
pub type GMAC_DCFGR = crate::Reg<gmac_dcfgr::GMAC_DCFGR_SPEC>;
#[doc = "DMA Configuration Register"]
pub mod gmac_dcfgr;
#[doc = "GMAC_TSR register accessor: an alias for `Reg<GMAC_TSR_SPEC>`"]
pub type GMAC_TSR = crate::Reg<gmac_tsr::GMAC_TSR_SPEC>;
#[doc = "Transmit Status Register"]
pub mod gmac_tsr;
#[doc = "GMAC_RBQB register accessor: an alias for `Reg<GMAC_RBQB_SPEC>`"]
pub type GMAC_RBQB = crate::Reg<gmac_rbqb::GMAC_RBQB_SPEC>;
#[doc = "Receive Buffer Queue Base Address Register"]
pub mod gmac_rbqb;
#[doc = "GMAC_TBQB register accessor: an alias for `Reg<GMAC_TBQB_SPEC>`"]
pub type GMAC_TBQB = crate::Reg<gmac_tbqb::GMAC_TBQB_SPEC>;
#[doc = "Transmit Buffer Queue Base Address Register"]
pub mod gmac_tbqb;
#[doc = "GMAC_RSR register accessor: an alias for `Reg<GMAC_RSR_SPEC>`"]
pub type GMAC_RSR = crate::Reg<gmac_rsr::GMAC_RSR_SPEC>;
#[doc = "Receive Status Register"]
pub mod gmac_rsr;
#[doc = "GMAC_ISR register accessor: an alias for `Reg<GMAC_ISR_SPEC>`"]
pub type GMAC_ISR = crate::Reg<gmac_isr::GMAC_ISR_SPEC>;
#[doc = "Interrupt Status Register"]
pub mod gmac_isr;
#[doc = "GMAC_IER register accessor: an alias for `Reg<GMAC_IER_SPEC>`"]
pub type GMAC_IER = crate::Reg<gmac_ier::GMAC_IER_SPEC>;
#[doc = "Interrupt Enable Register"]
pub mod gmac_ier;
#[doc = "GMAC_IDR register accessor: an alias for `Reg<GMAC_IDR_SPEC>`"]
pub type GMAC_IDR = crate::Reg<gmac_idr::GMAC_IDR_SPEC>;
#[doc = "Interrupt Disable Register"]
pub mod gmac_idr;
#[doc = "GMAC_IMR register accessor: an alias for `Reg<GMAC_IMR_SPEC>`"]
pub type GMAC_IMR = crate::Reg<gmac_imr::GMAC_IMR_SPEC>;
#[doc = "Interrupt Mask Register"]
pub mod gmac_imr;
#[doc = "GMAC_MAN register accessor: an alias for `Reg<GMAC_MAN_SPEC>`"]
pub type GMAC_MAN = crate::Reg<gmac_man::GMAC_MAN_SPEC>;
#[doc = "PHY Maintenance Register"]
pub mod gmac_man;
#[doc = "GMAC_RPQ register accessor: an alias for `Reg<GMAC_RPQ_SPEC>`"]
pub type GMAC_RPQ = crate::Reg<gmac_rpq::GMAC_RPQ_SPEC>;
#[doc = "Received Pause Quantum Register"]
pub mod gmac_rpq;
#[doc = "GMAC_TPQ register accessor: an alias for `Reg<GMAC_TPQ_SPEC>`"]
pub type GMAC_TPQ = crate::Reg<gmac_tpq::GMAC_TPQ_SPEC>;
#[doc = "Transmit Pause Quantum Register"]
pub mod gmac_tpq;
#[doc = "GMAC_TPSF register accessor: an alias for `Reg<GMAC_TPSF_SPEC>`"]
pub type GMAC_TPSF = crate::Reg<gmac_tpsf::GMAC_TPSF_SPEC>;
#[doc = "TX Partial Store and Forward Register"]
pub mod gmac_tpsf;
#[doc = "GMAC_RPSF register accessor: an alias for `Reg<GMAC_RPSF_SPEC>`"]
pub type GMAC_RPSF = crate::Reg<gmac_rpsf::GMAC_RPSF_SPEC>;
#[doc = "RX Partial Store and Forward Register"]
pub mod gmac_rpsf;
#[doc = "GMAC_RJFML register accessor: an alias for `Reg<GMAC_RJFML_SPEC>`"]
pub type GMAC_RJFML = crate::Reg<gmac_rjfml::GMAC_RJFML_SPEC>;
#[doc = "RX Jumbo Frame Max Length Register"]
pub mod gmac_rjfml;
#[doc = "GMAC_HRB register accessor: an alias for `Reg<GMAC_HRB_SPEC>`"]
pub type GMAC_HRB = crate::Reg<gmac_hrb::GMAC_HRB_SPEC>;
#[doc = "Hash Register Bottom"]
pub mod gmac_hrb;
#[doc = "GMAC_HRT register accessor: an alias for `Reg<GMAC_HRT_SPEC>`"]
pub type GMAC_HRT = crate::Reg<gmac_hrt::GMAC_HRT_SPEC>;
#[doc = "Hash Register Top"]
pub mod gmac_hrt;
#[doc = "GMAC_TIDM1 register accessor: an alias for `Reg<GMAC_TIDM1_SPEC>`"]
pub type GMAC_TIDM1 = crate::Reg<gmac_tidm1::GMAC_TIDM1_SPEC>;
#[doc = "Type ID Match 1 Register"]
pub mod gmac_tidm1;
#[doc = "GMAC_TIDM2 register accessor: an alias for `Reg<GMAC_TIDM2_SPEC>`"]
pub type GMAC_TIDM2 = crate::Reg<gmac_tidm2::GMAC_TIDM2_SPEC>;
#[doc = "Type ID Match 2 Register"]
pub mod gmac_tidm2;
#[doc = "GMAC_TIDM3 register accessor: an alias for `Reg<GMAC_TIDM3_SPEC>`"]
pub type GMAC_TIDM3 = crate::Reg<gmac_tidm3::GMAC_TIDM3_SPEC>;
#[doc = "Type ID Match 3 Register"]
pub mod gmac_tidm3;
#[doc = "GMAC_TIDM4 register accessor: an alias for `Reg<GMAC_TIDM4_SPEC>`"]
pub type GMAC_TIDM4 = crate::Reg<gmac_tidm4::GMAC_TIDM4_SPEC>;
#[doc = "Type ID Match 4 Register"]
pub mod gmac_tidm4;
#[doc = "GMAC_WOL register accessor: an alias for `Reg<GMAC_WOL_SPEC>`"]
pub type GMAC_WOL = crate::Reg<gmac_wol::GMAC_WOL_SPEC>;
#[doc = "Wake on LAN Register"]
pub mod gmac_wol;
#[doc = "GMAC_IPGS register accessor: an alias for `Reg<GMAC_IPGS_SPEC>`"]
pub type GMAC_IPGS = crate::Reg<gmac_ipgs::GMAC_IPGS_SPEC>;
#[doc = "IPG Stretch Register"]
pub mod gmac_ipgs;
#[doc = "GMAC_SVLAN register accessor: an alias for `Reg<GMAC_SVLAN_SPEC>`"]
pub type GMAC_SVLAN = crate::Reg<gmac_svlan::GMAC_SVLAN_SPEC>;
#[doc = "Stacked VLAN Register"]
pub mod gmac_svlan;
#[doc = "GMAC_TPFCP register accessor: an alias for `Reg<GMAC_TPFCP_SPEC>`"]
pub type GMAC_TPFCP = crate::Reg<gmac_tpfcp::GMAC_TPFCP_SPEC>;
#[doc = "Transmit PFC Pause Register"]
pub mod gmac_tpfcp;
#[doc = "GMAC_SAMB1 register accessor: an alias for `Reg<GMAC_SAMB1_SPEC>`"]
pub type GMAC_SAMB1 = crate::Reg<gmac_samb1::GMAC_SAMB1_SPEC>;
#[doc = "Specific Address 1 Mask Bottom Register"]
pub mod gmac_samb1;
#[doc = "GMAC_SAMT1 register accessor: an alias for `Reg<GMAC_SAMT1_SPEC>`"]
pub type GMAC_SAMT1 = crate::Reg<gmac_samt1::GMAC_SAMT1_SPEC>;
#[doc = "Specific Address 1 Mask Top Register"]
pub mod gmac_samt1;
#[doc = "GMAC_NSC register accessor: an alias for `Reg<GMAC_NSC_SPEC>`"]
pub type GMAC_NSC = crate::Reg<gmac_nsc::GMAC_NSC_SPEC>;
#[doc = "1588 Timer Nanosecond Comparison Register"]
pub mod gmac_nsc;
#[doc = "GMAC_SCL register accessor: an alias for `Reg<GMAC_SCL_SPEC>`"]
pub type GMAC_SCL = crate::Reg<gmac_scl::GMAC_SCL_SPEC>;
#[doc = "1588 Timer Second Comparison Low Register"]
pub mod gmac_scl;
#[doc = "GMAC_SCH register accessor: an alias for `Reg<GMAC_SCH_SPEC>`"]
pub type GMAC_SCH = crate::Reg<gmac_sch::GMAC_SCH_SPEC>;
#[doc = "1588 Timer Second Comparison High Register"]
pub mod gmac_sch;
#[doc = "GMAC_EFTSH register accessor: an alias for `Reg<GMAC_EFTSH_SPEC>`"]
pub type GMAC_EFTSH = crate::Reg<gmac_eftsh::GMAC_EFTSH_SPEC>;
#[doc = "PTP Event Frame Transmitted Seconds High Register"]
pub mod gmac_eftsh;
#[doc = "GMAC_EFRSH register accessor: an alias for `Reg<GMAC_EFRSH_SPEC>`"]
pub type GMAC_EFRSH = crate::Reg<gmac_efrsh::GMAC_EFRSH_SPEC>;
#[doc = "PTP Event Frame Received Seconds High Register"]
pub mod gmac_efrsh;
#[doc = "GMAC_PEFTSH register accessor: an alias for `Reg<GMAC_PEFTSH_SPEC>`"]
pub type GMAC_PEFTSH = crate::Reg<gmac_peftsh::GMAC_PEFTSH_SPEC>;
#[doc = "PTP Peer Event Frame Transmitted Seconds High Register"]
pub mod gmac_peftsh;
#[doc = "GMAC_PEFRSH register accessor: an alias for `Reg<GMAC_PEFRSH_SPEC>`"]
pub type GMAC_PEFRSH = crate::Reg<gmac_pefrsh::GMAC_PEFRSH_SPEC>;
#[doc = "PTP Peer Event Frame Received Seconds High Register"]
pub mod gmac_pefrsh;
#[doc = "GMAC_OTLO register accessor: an alias for `Reg<GMAC_OTLO_SPEC>`"]
pub type GMAC_OTLO = crate::Reg<gmac_otlo::GMAC_OTLO_SPEC>;
#[doc = "Octets Transmitted Low Register"]
pub mod gmac_otlo;
#[doc = "GMAC_OTHI register accessor: an alias for `Reg<GMAC_OTHI_SPEC>`"]
pub type GMAC_OTHI = crate::Reg<gmac_othi::GMAC_OTHI_SPEC>;
#[doc = "Octets Transmitted High Register"]
pub mod gmac_othi;
#[doc = "GMAC_FT register accessor: an alias for `Reg<GMAC_FT_SPEC>`"]
pub type GMAC_FT = crate::Reg<gmac_ft::GMAC_FT_SPEC>;
#[doc = "Frames Transmitted Register"]
pub mod gmac_ft;
#[doc = "GMAC_BCFT register accessor: an alias for `Reg<GMAC_BCFT_SPEC>`"]
pub type GMAC_BCFT = crate::Reg<gmac_bcft::GMAC_BCFT_SPEC>;
#[doc = "Broadcast Frames Transmitted Register"]
pub mod gmac_bcft;
#[doc = "GMAC_MFT register accessor: an alias for `Reg<GMAC_MFT_SPEC>`"]
pub type GMAC_MFT = crate::Reg<gmac_mft::GMAC_MFT_SPEC>;
#[doc = "Multicast Frames Transmitted Register"]
pub mod gmac_mft;
#[doc = "GMAC_PFT register accessor: an alias for `Reg<GMAC_PFT_SPEC>`"]
pub type GMAC_PFT = crate::Reg<gmac_pft::GMAC_PFT_SPEC>;
#[doc = "Pause Frames Transmitted Register"]
pub mod gmac_pft;
#[doc = "GMAC_BFT64 register accessor: an alias for `Reg<GMAC_BFT64_SPEC>`"]
pub type GMAC_BFT64 = crate::Reg<gmac_bft64::GMAC_BFT64_SPEC>;
#[doc = "64 Byte Frames Transmitted Register"]
pub mod gmac_bft64;
#[doc = "GMAC_TBFT127 register accessor: an alias for `Reg<GMAC_TBFT127_SPEC>`"]
pub type GMAC_TBFT127 = crate::Reg<gmac_tbft127::GMAC_TBFT127_SPEC>;
#[doc = "65 to 127 Byte Frames Transmitted Register"]
pub mod gmac_tbft127;
#[doc = "GMAC_TBFT255 register accessor: an alias for `Reg<GMAC_TBFT255_SPEC>`"]
pub type GMAC_TBFT255 = crate::Reg<gmac_tbft255::GMAC_TBFT255_SPEC>;
#[doc = "128 to 255 Byte Frames Transmitted Register"]
pub mod gmac_tbft255;
#[doc = "GMAC_TBFT511 register accessor: an alias for `Reg<GMAC_TBFT511_SPEC>`"]
pub type GMAC_TBFT511 = crate::Reg<gmac_tbft511::GMAC_TBFT511_SPEC>;
#[doc = "256 to 511 Byte Frames Transmitted Register"]
pub mod gmac_tbft511;
#[doc = "GMAC_TBFT1023 register accessor: an alias for `Reg<GMAC_TBFT1023_SPEC>`"]
pub type GMAC_TBFT1023 = crate::Reg<gmac_tbft1023::GMAC_TBFT1023_SPEC>;
#[doc = "512 to 1023 Byte Frames Transmitted Register"]
pub mod gmac_tbft1023;
#[doc = "GMAC_TBFT1518 register accessor: an alias for `Reg<GMAC_TBFT1518_SPEC>`"]
pub type GMAC_TBFT1518 = crate::Reg<gmac_tbft1518::GMAC_TBFT1518_SPEC>;
#[doc = "1024 to 1518 Byte Frames Transmitted Register"]
pub mod gmac_tbft1518;
#[doc = "GMAC_GTBFT1518 register accessor: an alias for `Reg<GMAC_GTBFT1518_SPEC>`"]
pub type GMAC_GTBFT1518 = crate::Reg<gmac_gtbft1518::GMAC_GTBFT1518_SPEC>;
#[doc = "Greater Than 1518 Byte Frames Transmitted Register"]
pub mod gmac_gtbft1518;
#[doc = "GMAC_TUR register accessor: an alias for `Reg<GMAC_TUR_SPEC>`"]
pub type GMAC_TUR = crate::Reg<gmac_tur::GMAC_TUR_SPEC>;
#[doc = "Transmit Underruns Register"]
pub mod gmac_tur;
#[doc = "GMAC_SCF register accessor: an alias for `Reg<GMAC_SCF_SPEC>`"]
pub type GMAC_SCF = crate::Reg<gmac_scf::GMAC_SCF_SPEC>;
#[doc = "Single Collision Frames Register"]
pub mod gmac_scf;
#[doc = "GMAC_MCF register accessor: an alias for `Reg<GMAC_MCF_SPEC>`"]
pub type GMAC_MCF = crate::Reg<gmac_mcf::GMAC_MCF_SPEC>;
#[doc = "Multiple Collision Frames Register"]
pub mod gmac_mcf;
#[doc = "GMAC_EC register accessor: an alias for `Reg<GMAC_EC_SPEC>`"]
pub type GMAC_EC = crate::Reg<gmac_ec::GMAC_EC_SPEC>;
#[doc = "Excessive Collisions Register"]
pub mod gmac_ec;
#[doc = "GMAC_LC register accessor: an alias for `Reg<GMAC_LC_SPEC>`"]
pub type GMAC_LC = crate::Reg<gmac_lc::GMAC_LC_SPEC>;
#[doc = "Late Collisions Register"]
pub mod gmac_lc;
#[doc = "GMAC_DTF register accessor: an alias for `Reg<GMAC_DTF_SPEC>`"]
pub type GMAC_DTF = crate::Reg<gmac_dtf::GMAC_DTF_SPEC>;
#[doc = "Deferred Transmission Frames Register"]
pub mod gmac_dtf;
#[doc = "GMAC_CSE register accessor: an alias for `Reg<GMAC_CSE_SPEC>`"]
pub type GMAC_CSE = crate::Reg<gmac_cse::GMAC_CSE_SPEC>;
#[doc = "Carrier Sense Errors Register"]
pub mod gmac_cse;
#[doc = "GMAC_ORLO register accessor: an alias for `Reg<GMAC_ORLO_SPEC>`"]
pub type GMAC_ORLO = crate::Reg<gmac_orlo::GMAC_ORLO_SPEC>;
#[doc = "Octets Received Low Received Register"]
pub mod gmac_orlo;
#[doc = "GMAC_ORHI register accessor: an alias for `Reg<GMAC_ORHI_SPEC>`"]
pub type GMAC_ORHI = crate::Reg<gmac_orhi::GMAC_ORHI_SPEC>;
#[doc = "Octets Received High Received Register"]
pub mod gmac_orhi;
#[doc = "GMAC_FR register accessor: an alias for `Reg<GMAC_FR_SPEC>`"]
pub type GMAC_FR = crate::Reg<gmac_fr::GMAC_FR_SPEC>;
#[doc = "Frames Received Register"]
pub mod gmac_fr;
#[doc = "GMAC_BCFR register accessor: an alias for `Reg<GMAC_BCFR_SPEC>`"]
pub type GMAC_BCFR = crate::Reg<gmac_bcfr::GMAC_BCFR_SPEC>;
#[doc = "Broadcast Frames Received Register"]
pub mod gmac_bcfr;
#[doc = "GMAC_MFR register accessor: an alias for `Reg<GMAC_MFR_SPEC>`"]
pub type GMAC_MFR = crate::Reg<gmac_mfr::GMAC_MFR_SPEC>;
#[doc = "Multicast Frames Received Register"]
pub mod gmac_mfr;
#[doc = "GMAC_PFR register accessor: an alias for `Reg<GMAC_PFR_SPEC>`"]
pub type GMAC_PFR = crate::Reg<gmac_pfr::GMAC_PFR_SPEC>;
#[doc = "Pause Frames Received Register"]
pub mod gmac_pfr;
#[doc = "GMAC_BFR64 register accessor: an alias for `Reg<GMAC_BFR64_SPEC>`"]
pub type GMAC_BFR64 = crate::Reg<gmac_bfr64::GMAC_BFR64_SPEC>;
#[doc = "64 Byte Frames Received Register"]
pub mod gmac_bfr64;
#[doc = "GMAC_TBFR127 register accessor: an alias for `Reg<GMAC_TBFR127_SPEC>`"]
pub type GMAC_TBFR127 = crate::Reg<gmac_tbfr127::GMAC_TBFR127_SPEC>;
#[doc = "65 to 127 Byte Frames Received Register"]
pub mod gmac_tbfr127;
#[doc = "GMAC_TBFR255 register accessor: an alias for `Reg<GMAC_TBFR255_SPEC>`"]
pub type GMAC_TBFR255 = crate::Reg<gmac_tbfr255::GMAC_TBFR255_SPEC>;
#[doc = "128 to 255 Byte Frames Received Register"]
pub mod gmac_tbfr255;
#[doc = "GMAC_TBFR511 register accessor: an alias for `Reg<GMAC_TBFR511_SPEC>`"]
pub type GMAC_TBFR511 = crate::Reg<gmac_tbfr511::GMAC_TBFR511_SPEC>;
#[doc = "256 to 511 Byte Frames Received Register"]
pub mod gmac_tbfr511;
#[doc = "GMAC_TBFR1023 register accessor: an alias for `Reg<GMAC_TBFR1023_SPEC>`"]
pub type GMAC_TBFR1023 = crate::Reg<gmac_tbfr1023::GMAC_TBFR1023_SPEC>;
#[doc = "512 to 1023 Byte Frames Received Register"]
pub mod gmac_tbfr1023;
#[doc = "GMAC_TBFR1518 register accessor: an alias for `Reg<GMAC_TBFR1518_SPEC>`"]
pub type GMAC_TBFR1518 = crate::Reg<gmac_tbfr1518::GMAC_TBFR1518_SPEC>;
#[doc = "1024 to 1518 Byte Frames Received Register"]
pub mod gmac_tbfr1518;
#[doc = "GMAC_TMXBFR register accessor: an alias for `Reg<GMAC_TMXBFR_SPEC>`"]
pub type GMAC_TMXBFR = crate::Reg<gmac_tmxbfr::GMAC_TMXBFR_SPEC>;
#[doc = "1519 to Maximum Byte Frames Received Register"]
pub mod gmac_tmxbfr;
#[doc = "GMAC_UFR register accessor: an alias for `Reg<GMAC_UFR_SPEC>`"]
pub type GMAC_UFR = crate::Reg<gmac_ufr::GMAC_UFR_SPEC>;
#[doc = "Undersize Frames Received Register"]
pub mod gmac_ufr;
#[doc = "GMAC_OFR register accessor: an alias for `Reg<GMAC_OFR_SPEC>`"]
pub type GMAC_OFR = crate::Reg<gmac_ofr::GMAC_OFR_SPEC>;
#[doc = "Oversize Frames Received Register"]
pub mod gmac_ofr;
#[doc = "GMAC_JR register accessor: an alias for `Reg<GMAC_JR_SPEC>`"]
pub type GMAC_JR = crate::Reg<gmac_jr::GMAC_JR_SPEC>;
#[doc = "Jabbers Received Register"]
pub mod gmac_jr;
#[doc = "GMAC_FCSE register accessor: an alias for `Reg<GMAC_FCSE_SPEC>`"]
pub type GMAC_FCSE = crate::Reg<gmac_fcse::GMAC_FCSE_SPEC>;
#[doc = "Frame Check Sequence Errors Register"]
pub mod gmac_fcse;
#[doc = "GMAC_LFFE register accessor: an alias for `Reg<GMAC_LFFE_SPEC>`"]
pub type GMAC_LFFE = crate::Reg<gmac_lffe::GMAC_LFFE_SPEC>;
#[doc = "Length Field Frame Errors Register"]
pub mod gmac_lffe;
#[doc = "GMAC_RSE register accessor: an alias for `Reg<GMAC_RSE_SPEC>`"]
pub type GMAC_RSE = crate::Reg<gmac_rse::GMAC_RSE_SPEC>;
#[doc = "Receive Symbol Errors Register"]
pub mod gmac_rse;
#[doc = "GMAC_AE register accessor: an alias for `Reg<GMAC_AE_SPEC>`"]
pub type GMAC_AE = crate::Reg<gmac_ae::GMAC_AE_SPEC>;
#[doc = "Alignment Errors Register"]
pub mod gmac_ae;
#[doc = "GMAC_RRE register accessor: an alias for `Reg<GMAC_RRE_SPEC>`"]
pub type GMAC_RRE = crate::Reg<gmac_rre::GMAC_RRE_SPEC>;
#[doc = "Receive Resource Errors Register"]
pub mod gmac_rre;
#[doc = "GMAC_ROE register accessor: an alias for `Reg<GMAC_ROE_SPEC>`"]
pub type GMAC_ROE = crate::Reg<gmac_roe::GMAC_ROE_SPEC>;
#[doc = "Receive Overrun Register"]
pub mod gmac_roe;
#[doc = "GMAC_IHCE register accessor: an alias for `Reg<GMAC_IHCE_SPEC>`"]
pub type GMAC_IHCE = crate::Reg<gmac_ihce::GMAC_IHCE_SPEC>;
#[doc = "IP Header Checksum Errors Register"]
pub mod gmac_ihce;
#[doc = "GMAC_TCE register accessor: an alias for `Reg<GMAC_TCE_SPEC>`"]
pub type GMAC_TCE = crate::Reg<gmac_tce::GMAC_TCE_SPEC>;
#[doc = "TCP Checksum Errors Register"]
pub mod gmac_tce;
#[doc = "GMAC_UCE register accessor: an alias for `Reg<GMAC_UCE_SPEC>`"]
pub type GMAC_UCE = crate::Reg<gmac_uce::GMAC_UCE_SPEC>;
#[doc = "UDP Checksum Errors Register"]
pub mod gmac_uce;
#[doc = "GMAC_TISUBN register accessor: an alias for `Reg<GMAC_TISUBN_SPEC>`"]
pub type GMAC_TISUBN = crate::Reg<gmac_tisubn::GMAC_TISUBN_SPEC>;
#[doc = "1588 Timer Increment Sub-nanoseconds Register"]
pub mod gmac_tisubn;
#[doc = "GMAC_TSH register accessor: an alias for `Reg<GMAC_TSH_SPEC>`"]
pub type GMAC_TSH = crate::Reg<gmac_tsh::GMAC_TSH_SPEC>;
#[doc = "1588 Timer Seconds High Register"]
pub mod gmac_tsh;
#[doc = "GMAC_TSL register accessor: an alias for `Reg<GMAC_TSL_SPEC>`"]
pub type GMAC_TSL = crate::Reg<gmac_tsl::GMAC_TSL_SPEC>;
#[doc = "1588 Timer Seconds Low Register"]
pub mod gmac_tsl;
#[doc = "GMAC_TN register accessor: an alias for `Reg<GMAC_TN_SPEC>`"]
pub type GMAC_TN = crate::Reg<gmac_tn::GMAC_TN_SPEC>;
#[doc = "1588 Timer Nanoseconds Register"]
pub mod gmac_tn;
#[doc = "GMAC_TA register accessor: an alias for `Reg<GMAC_TA_SPEC>`"]
pub type GMAC_TA = crate::Reg<gmac_ta::GMAC_TA_SPEC>;
#[doc = "1588 Timer Adjust Register"]
pub mod gmac_ta;
#[doc = "GMAC_TI register accessor: an alias for `Reg<GMAC_TI_SPEC>`"]
pub type GMAC_TI = crate::Reg<gmac_ti::GMAC_TI_SPEC>;
#[doc = "1588 Timer Increment Register"]
pub mod gmac_ti;
#[doc = "GMAC_EFTSL register accessor: an alias for `Reg<GMAC_EFTSL_SPEC>`"]
pub type GMAC_EFTSL = crate::Reg<gmac_eftsl::GMAC_EFTSL_SPEC>;
#[doc = "PTP Event Frame Transmitted Seconds Low Register"]
pub mod gmac_eftsl;
#[doc = "GMAC_EFTN register accessor: an alias for `Reg<GMAC_EFTN_SPEC>`"]
pub type GMAC_EFTN = crate::Reg<gmac_eftn::GMAC_EFTN_SPEC>;
#[doc = "PTP Event Frame Transmitted Nanoseconds Register"]
pub mod gmac_eftn;
#[doc = "GMAC_EFRSL register accessor: an alias for `Reg<GMAC_EFRSL_SPEC>`"]
pub type GMAC_EFRSL = crate::Reg<gmac_efrsl::GMAC_EFRSL_SPEC>;
#[doc = "PTP Event Frame Received Seconds Low Register"]
pub mod gmac_efrsl;
#[doc = "GMAC_EFRN register accessor: an alias for `Reg<GMAC_EFRN_SPEC>`"]
pub type GMAC_EFRN = crate::Reg<gmac_efrn::GMAC_EFRN_SPEC>;
#[doc = "PTP Event Frame Received Nanoseconds Register"]
pub mod gmac_efrn;
#[doc = "GMAC_PEFTSL register accessor: an alias for `Reg<GMAC_PEFTSL_SPEC>`"]
pub type GMAC_PEFTSL = crate::Reg<gmac_peftsl::GMAC_PEFTSL_SPEC>;
#[doc = "PTP Peer Event Frame Transmitted Seconds Low Register"]
pub mod gmac_peftsl;
#[doc = "GMAC_PEFTN register accessor: an alias for `Reg<GMAC_PEFTN_SPEC>`"]
pub type GMAC_PEFTN = crate::Reg<gmac_peftn::GMAC_PEFTN_SPEC>;
#[doc = "PTP Peer Event Frame Transmitted Nanoseconds Register"]
pub mod gmac_peftn;
#[doc = "GMAC_PEFRSL register accessor: an alias for `Reg<GMAC_PEFRSL_SPEC>`"]
pub type GMAC_PEFRSL = crate::Reg<gmac_pefrsl::GMAC_PEFRSL_SPEC>;
#[doc = "PTP Peer Event Frame Received Seconds Low Register"]
pub mod gmac_pefrsl;
#[doc = "GMAC_PEFRN register accessor: an alias for `Reg<GMAC_PEFRN_SPEC>`"]
pub type GMAC_PEFRN = crate::Reg<gmac_pefrn::GMAC_PEFRN_SPEC>;
#[doc = "PTP Peer Event Frame Received Nanoseconds Register"]
pub mod gmac_pefrn;
#[doc = "GMAC_RXLPI register accessor: an alias for `Reg<GMAC_RXLPI_SPEC>`"]
pub type GMAC_RXLPI = crate::Reg<gmac_rxlpi::GMAC_RXLPI_SPEC>;
#[doc = "Received LPI Transitions"]
pub mod gmac_rxlpi;
#[doc = "GMAC_RXLPITIME register accessor: an alias for `Reg<GMAC_RXLPITIME_SPEC>`"]
pub type GMAC_RXLPITIME = crate::Reg<gmac_rxlpitime::GMAC_RXLPITIME_SPEC>;
#[doc = "Received LPI Time"]
pub mod gmac_rxlpitime;
#[doc = "GMAC_TXLPI register accessor: an alias for `Reg<GMAC_TXLPI_SPEC>`"]
pub type GMAC_TXLPI = crate::Reg<gmac_txlpi::GMAC_TXLPI_SPEC>;
#[doc = "Transmit LPI Transitions"]
pub mod gmac_txlpi;
#[doc = "GMAC_TXLPITIME register accessor: an alias for `Reg<GMAC_TXLPITIME_SPEC>`"]
pub type GMAC_TXLPITIME = crate::Reg<gmac_txlpitime::GMAC_TXLPITIME_SPEC>;
#[doc = "Transmit LPI Time"]
pub mod gmac_txlpitime;
#[doc = "GMAC_ISRPQ register accessor: an alias for `Reg<GMAC_ISRPQ_SPEC>`"]
pub type GMAC_ISRPQ = crate::Reg<gmac_isrpq::GMAC_ISRPQ_SPEC>;
#[doc = "Interrupt Status Register Priority Queue (1..5)"]
pub mod gmac_isrpq;
#[doc = "GMAC_TBQBAPQ register accessor: an alias for `Reg<GMAC_TBQBAPQ_SPEC>`"]
pub type GMAC_TBQBAPQ = crate::Reg<gmac_tbqbapq::GMAC_TBQBAPQ_SPEC>;
#[doc = "Transmit Buffer Queue Base Address Register Priority Queue (1..5)"]
pub mod gmac_tbqbapq;
#[doc = "GMAC_RBQBAPQ register accessor: an alias for `Reg<GMAC_RBQBAPQ_SPEC>`"]
pub type GMAC_RBQBAPQ = crate::Reg<gmac_rbqbapq::GMAC_RBQBAPQ_SPEC>;
#[doc = "Receive Buffer Queue Base Address Register Priority Queue (1..5)"]
pub mod gmac_rbqbapq;
#[doc = "GMAC_RBSRPQ register accessor: an alias for `Reg<GMAC_RBSRPQ_SPEC>`"]
pub type GMAC_RBSRPQ = crate::Reg<gmac_rbsrpq::GMAC_RBSRPQ_SPEC>;
#[doc = "Receive Buffer Size Register Priority Queue (1..5)"]
pub mod gmac_rbsrpq;
#[doc = "GMAC_CBSCR register accessor: an alias for `Reg<GMAC_CBSCR_SPEC>`"]
pub type GMAC_CBSCR = crate::Reg<gmac_cbscr::GMAC_CBSCR_SPEC>;
#[doc = "Credit-Based Shaping Control Register"]
pub mod gmac_cbscr;
#[doc = "GMAC_CBSISQA register accessor: an alias for `Reg<GMAC_CBSISQA_SPEC>`"]
pub type GMAC_CBSISQA = crate::Reg<gmac_cbsisqa::GMAC_CBSISQA_SPEC>;
#[doc = "Credit-Based Shaping IdleSlope Register for Queue A"]
pub mod gmac_cbsisqa;
#[doc = "GMAC_CBSISQB register accessor: an alias for `Reg<GMAC_CBSISQB_SPEC>`"]
pub type GMAC_CBSISQB = crate::Reg<gmac_cbsisqb::GMAC_CBSISQB_SPEC>;
#[doc = "Credit-Based Shaping IdleSlope Register for Queue B"]
pub mod gmac_cbsisqb;
#[doc = "GMAC_ST1RPQ register accessor: an alias for `Reg<GMAC_ST1RPQ_SPEC>`"]
pub type GMAC_ST1RPQ = crate::Reg<gmac_st1rpq::GMAC_ST1RPQ_SPEC>;
#[doc = "Screening Type 1 Register Priority Queue"]
pub mod gmac_st1rpq;
#[doc = "GMAC_ST2RPQ register accessor: an alias for `Reg<GMAC_ST2RPQ_SPEC>`"]
pub type GMAC_ST2RPQ = crate::Reg<gmac_st2rpq::GMAC_ST2RPQ_SPEC>;
#[doc = "Screening Type 2 Register Priority Queue"]
pub mod gmac_st2rpq;
#[doc = "GMAC_IERPQ register accessor: an alias for `Reg<GMAC_IERPQ_SPEC>`"]
pub type GMAC_IERPQ = crate::Reg<gmac_ierpq::GMAC_IERPQ_SPEC>;
#[doc = "Interrupt Enable Register Priority Queue (1..5)"]
pub mod gmac_ierpq;
#[doc = "GMAC_IDRPQ register accessor: an alias for `Reg<GMAC_IDRPQ_SPEC>`"]
pub type GMAC_IDRPQ = crate::Reg<gmac_idrpq::GMAC_IDRPQ_SPEC>;
#[doc = "Interrupt Disable Register Priority Queue (1..5)"]
pub mod gmac_idrpq;
#[doc = "GMAC_IMRPQ register accessor: an alias for `Reg<GMAC_IMRPQ_SPEC>`"]
pub type GMAC_IMRPQ = crate::Reg<gmac_imrpq::GMAC_IMRPQ_SPEC>;
#[doc = "Interrupt Mask Register Priority Queue (1..5)"]
pub mod gmac_imrpq;
#[doc = "GMAC_ST2ER register accessor: an alias for `Reg<GMAC_ST2ER_SPEC>`"]
pub type GMAC_ST2ER = crate::Reg<gmac_st2er::GMAC_ST2ER_SPEC>;
#[doc = "Screening Type 2 Ethertype Register"]
pub mod gmac_st2er;
