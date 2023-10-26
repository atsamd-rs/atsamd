#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Network Control Register"]
    pub ncr: NCR,
    #[doc = "0x04 - Network Configuration Register"]
    pub ncfgr: NCFGR,
    #[doc = "0x08 - Network Status Register"]
    pub nsr: NSR,
    #[doc = "0x0c - User Register"]
    pub ur: UR,
    #[doc = "0x10 - DMA Configuration Register"]
    pub dcfgr: DCFGR,
    #[doc = "0x14 - Transmit Status Register"]
    pub tsr: TSR,
    #[doc = "0x18 - Receive Buffer Queue Base Address"]
    pub rbqb: RBQB,
    #[doc = "0x1c - Transmit Buffer Queue Base Address"]
    pub tbqb: TBQB,
    #[doc = "0x20 - Receive Status Register"]
    pub rsr: RSR,
    #[doc = "0x24 - Interrupt Status Register"]
    pub isr: ISR,
    #[doc = "0x28 - Interrupt Enable Register"]
    pub ier: IER,
    #[doc = "0x2c - Interrupt Disable Register"]
    pub idr: IDR,
    #[doc = "0x30 - Interrupt Mask Register"]
    pub imr: IMR,
    #[doc = "0x34 - PHY Maintenance Register"]
    pub man: MAN,
    #[doc = "0x38 - Received Pause Quantum Register"]
    pub rpq: RPQ,
    #[doc = "0x3c - Transmit Pause Quantum Register"]
    pub tpq: TPQ,
    #[doc = "0x40 - TX partial store and forward Register"]
    pub tpsf: TPSF,
    #[doc = "0x44 - RX partial store and forward Register"]
    pub rpsf: RPSF,
    #[doc = "0x48 - RX Jumbo Frame Max Length Register"]
    pub rjfml: RJFML,
    _reserved19: [u8; 0x34],
    #[doc = "0x80 - Hash Register Bottom \\[31:0\\]"]
    pub hrb: HRB,
    #[doc = "0x84 - Hash Register Top \\[63:32\\]"]
    pub hrt: HRT,
    #[doc = "0x88..0xa8 - SA\\[%s\\]"]
    pub sa: [SA; 4],
    #[doc = "0xa8..0xb8 - Type ID Match Register"]
    pub tidm: [TIDM; 4],
    #[doc = "0xb8 - Wake on LAN"]
    pub wol: WOL,
    #[doc = "0xbc - IPG Stretch Register"]
    pub ipgs: IPGS,
    #[doc = "0xc0 - Stacked VLAN Register"]
    pub svlan: SVLAN,
    #[doc = "0xc4 - Transmit PFC Pause Register"]
    pub tpfcp: TPFCP,
    #[doc = "0xc8 - Specific Address 1 Mask Bottom \\[31:0\\]
Register"]
    pub samb1: SAMB1,
    #[doc = "0xcc - Specific Address 1 Mask Top \\[47:32\\]
Register"]
    pub samt1: SAMT1,
    _reserved29: [u8; 0x0c],
    #[doc = "0xdc - Tsu timer comparison nanoseconds Register"]
    pub nsc: NSC,
    #[doc = "0xe0 - Tsu timer second comparison Register"]
    pub scl: SCL,
    #[doc = "0xe4 - Tsu timer second comparison Register"]
    pub sch: SCH,
    #[doc = "0xe8 - PTP Event Frame Transmitted Seconds High Register"]
    pub eftsh: EFTSH,
    #[doc = "0xec - PTP Event Frame Received Seconds High Register"]
    pub efrsh: EFRSH,
    #[doc = "0xf0 - PTP Peer Event Frame Transmitted Seconds High Register"]
    pub peftsh: PEFTSH,
    #[doc = "0xf4 - PTP Peer Event Frame Received Seconds High Register"]
    pub pefrsh: PEFRSH,
    _reserved36: [u8; 0x08],
    #[doc = "0x100 - Octets Transmitted \\[31:0\\]
Register"]
    pub otlo: OTLO,
    #[doc = "0x104 - Octets Transmitted \\[47:32\\]
Register"]
    pub othi: OTHI,
    #[doc = "0x108 - Frames Transmitted Register"]
    pub ft: FT,
    #[doc = "0x10c - Broadcast Frames Transmitted Register"]
    pub bcft: BCFT,
    #[doc = "0x110 - Multicast Frames Transmitted Register"]
    pub mft: MFT,
    #[doc = "0x114 - Pause Frames Transmitted Register"]
    pub pft: PFT,
    #[doc = "0x118 - 64 Byte Frames Transmitted Register"]
    pub bft64: BFT64,
    #[doc = "0x11c - 65 to 127 Byte Frames Transmitted Register"]
    pub tbft127: TBFT127,
    #[doc = "0x120 - 128 to 255 Byte Frames Transmitted Register"]
    pub tbft255: TBFT255,
    #[doc = "0x124 - 256 to 511 Byte Frames Transmitted Register"]
    pub tbft511: TBFT511,
    #[doc = "0x128 - 512 to 1023 Byte Frames Transmitted Register"]
    pub tbft1023: TBFT1023,
    #[doc = "0x12c - 1024 to 1518 Byte Frames Transmitted Register"]
    pub tbft1518: TBFT1518,
    #[doc = "0x130 - Greater Than 1518 Byte Frames Transmitted Register"]
    pub gtbft1518: GTBFT1518,
    #[doc = "0x134 - Transmit Underruns Register"]
    pub tur: TUR,
    #[doc = "0x138 - Single Collision Frames Register"]
    pub scf: SCF,
    #[doc = "0x13c - Multiple Collision Frames Register"]
    pub mcf: MCF,
    #[doc = "0x140 - Excessive Collisions Register"]
    pub ec: EC,
    #[doc = "0x144 - Late Collisions Register"]
    pub lc: LC,
    #[doc = "0x148 - Deferred Transmission Frames Register"]
    pub dtf: DTF,
    #[doc = "0x14c - Carrier Sense Errors Register"]
    pub cse: CSE,
    #[doc = "0x150 - Octets Received \\[31:0\\]
Received"]
    pub orlo: ORLO,
    #[doc = "0x154 - Octets Received \\[47:32\\]
Received"]
    pub orhi: ORHI,
    #[doc = "0x158 - Frames Received Register"]
    pub fr: FR,
    #[doc = "0x15c - Broadcast Frames Received Register"]
    pub bcfr: BCFR,
    #[doc = "0x160 - Multicast Frames Received Register"]
    pub mfr: MFR,
    #[doc = "0x164 - Pause Frames Received Register"]
    pub pfr: PFR,
    #[doc = "0x168 - 64 Byte Frames Received Register"]
    pub bfr64: BFR64,
    #[doc = "0x16c - 65 to 127 Byte Frames Received Register"]
    pub tbfr127: TBFR127,
    #[doc = "0x170 - 128 to 255 Byte Frames Received Register"]
    pub tbfr255: TBFR255,
    #[doc = "0x174 - 256 to 511Byte Frames Received Register"]
    pub tbfr511: TBFR511,
    #[doc = "0x178 - 512 to 1023 Byte Frames Received Register"]
    pub tbfr1023: TBFR1023,
    #[doc = "0x17c - 1024 to 1518 Byte Frames Received Register"]
    pub tbfr1518: TBFR1518,
    #[doc = "0x180 - 1519 to Maximum Byte Frames Received Register"]
    pub tmxbfr: TMXBFR,
    #[doc = "0x184 - Undersize Frames Received Register"]
    pub ufr: UFR,
    #[doc = "0x188 - Oversize Frames Received Register"]
    pub ofr: OFR,
    #[doc = "0x18c - Jabbers Received Register"]
    pub jr: JR,
    #[doc = "0x190 - Frame Check Sequence Errors Register"]
    pub fcse: FCSE,
    #[doc = "0x194 - Length Field Frame Errors Register"]
    pub lffe: LFFE,
    #[doc = "0x198 - Receive Symbol Errors Register"]
    pub rse: RSE,
    #[doc = "0x19c - Alignment Errors Register"]
    pub ae: AE,
    #[doc = "0x1a0 - Receive Resource Errors Register"]
    pub rre: RRE,
    #[doc = "0x1a4 - Receive Overrun Register"]
    pub roe: ROE,
    #[doc = "0x1a8 - IP Header Checksum Errors Register"]
    pub ihce: IHCE,
    #[doc = "0x1ac - TCP Checksum Errors Register"]
    pub tce: TCE,
    #[doc = "0x1b0 - UDP Checksum Errors Register"]
    pub uce: UCE,
    _reserved81: [u8; 0x08],
    #[doc = "0x1bc - 1588 Timer Increment \\[15:0\\]
Sub-Nanoseconds Register"]
    pub tisubn: TISUBN,
    #[doc = "0x1c0 - 1588 Timer Seconds High \\[15:0\\]
Register"]
    pub tsh: TSH,
    _reserved83: [u8; 0x04],
    #[doc = "0x1c8 - 1588 Timer Sync Strobe Seconds \\[31:0\\]
Register"]
    pub tsssl: TSSSL,
    #[doc = "0x1cc - 1588 Timer Sync Strobe Nanoseconds Register"]
    pub tssn: TSSN,
    #[doc = "0x1d0 - 1588 Timer Seconds \\[31:0\\]
Register"]
    pub tsl: TSL,
    #[doc = "0x1d4 - 1588 Timer Nanoseconds Register"]
    pub tn: TN,
    #[doc = "0x1d8 - 1588 Timer Adjust Register"]
    pub ta: TA,
    #[doc = "0x1dc - 1588 Timer Increment Register"]
    pub ti: TI,
    #[doc = "0x1e0 - PTP Event Frame Transmitted Seconds Low Register"]
    pub eftsl: EFTSL,
    #[doc = "0x1e4 - PTP Event Frame Transmitted Nanoseconds"]
    pub eftn: EFTN,
    #[doc = "0x1e8 - PTP Event Frame Received Seconds Low Register"]
    pub efrsl: EFRSL,
    #[doc = "0x1ec - PTP Event Frame Received Nanoseconds"]
    pub efrn: EFRN,
    #[doc = "0x1f0 - PTP Peer Event Frame Transmitted Seconds Low Register"]
    pub peftsl: PEFTSL,
    #[doc = "0x1f4 - PTP Peer Event Frame Transmitted Nanoseconds"]
    pub peftn: PEFTN,
    #[doc = "0x1f8 - PTP Peer Event Frame Received Seconds Low Register"]
    pub pefrsl: PEFRSL,
    #[doc = "0x1fc - PTP Peer Event Frame Received Nanoseconds"]
    pub pefrn: PEFRN,
    _reserved97: [u8; 0x70],
    #[doc = "0x270 - Receive LPI transition Register"]
    pub rlpitr: RLPITR,
    #[doc = "0x274 - Receive LPI Time Register"]
    pub rlpiti: RLPITI,
    #[doc = "0x278 - Receive LPI transition Register"]
    pub tlpitr: TLPITR,
    #[doc = "0x27c - Receive LPI Time Register"]
    pub tlpiti: TLPITI,
}
#[doc = "NCR (rw) register accessor: Network Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ncr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ncr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ncr`]
module"]
pub type NCR = crate::Reg<ncr::NCR_SPEC>;
#[doc = "Network Control Register"]
pub mod ncr;
#[doc = "NCFGR (rw) register accessor: Network Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ncfgr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ncfgr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ncfgr`]
module"]
pub type NCFGR = crate::Reg<ncfgr::NCFGR_SPEC>;
#[doc = "Network Configuration Register"]
pub mod ncfgr;
#[doc = "NSR (r) register accessor: Network Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nsr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nsr`]
module"]
pub type NSR = crate::Reg<nsr::NSR_SPEC>;
#[doc = "Network Status Register"]
pub mod nsr;
#[doc = "UR (rw) register accessor: User Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ur::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ur::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ur`]
module"]
pub type UR = crate::Reg<ur::UR_SPEC>;
#[doc = "User Register"]
pub mod ur;
#[doc = "DCFGR (rw) register accessor: DMA Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcfgr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcfgr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcfgr`]
module"]
pub type DCFGR = crate::Reg<dcfgr::DCFGR_SPEC>;
#[doc = "DMA Configuration Register"]
pub mod dcfgr;
#[doc = "TSR (rw) register accessor: Transmit Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tsr`]
module"]
pub type TSR = crate::Reg<tsr::TSR_SPEC>;
#[doc = "Transmit Status Register"]
pub mod tsr;
#[doc = "RBQB (rw) register accessor: Receive Buffer Queue Base Address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rbqb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rbqb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rbqb`]
module"]
pub type RBQB = crate::Reg<rbqb::RBQB_SPEC>;
#[doc = "Receive Buffer Queue Base Address"]
pub mod rbqb;
#[doc = "TBQB (rw) register accessor: Transmit Buffer Queue Base Address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tbqb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tbqb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tbqb`]
module"]
pub type TBQB = crate::Reg<tbqb::TBQB_SPEC>;
#[doc = "Transmit Buffer Queue Base Address"]
pub mod tbqb;
#[doc = "RSR (rw) register accessor: Receive Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rsr`]
module"]
pub type RSR = crate::Reg<rsr::RSR_SPEC>;
#[doc = "Receive Status Register"]
pub mod rsr;
#[doc = "ISR (rw) register accessor: Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isr`]
module"]
pub type ISR = crate::Reg<isr::ISR_SPEC>;
#[doc = "Interrupt Status Register"]
pub mod isr;
#[doc = "IER (w) register accessor: Interrupt Enable Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ier::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ier`]
module"]
pub type IER = crate::Reg<ier::IER_SPEC>;
#[doc = "Interrupt Enable Register"]
pub mod ier;
#[doc = "IDR (w) register accessor: Interrupt Disable Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`idr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@idr`]
module"]
pub type IDR = crate::Reg<idr::IDR_SPEC>;
#[doc = "Interrupt Disable Register"]
pub mod idr;
#[doc = "IMR (r) register accessor: Interrupt Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`imr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@imr`]
module"]
pub type IMR = crate::Reg<imr::IMR_SPEC>;
#[doc = "Interrupt Mask Register"]
pub mod imr;
#[doc = "MAN (rw) register accessor: PHY Maintenance Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`man::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`man::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@man`]
module"]
pub type MAN = crate::Reg<man::MAN_SPEC>;
#[doc = "PHY Maintenance Register"]
pub mod man;
#[doc = "RPQ (r) register accessor: Received Pause Quantum Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rpq::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rpq`]
module"]
pub type RPQ = crate::Reg<rpq::RPQ_SPEC>;
#[doc = "Received Pause Quantum Register"]
pub mod rpq;
#[doc = "TPQ (rw) register accessor: Transmit Pause Quantum Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tpq::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tpq::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tpq`]
module"]
pub type TPQ = crate::Reg<tpq::TPQ_SPEC>;
#[doc = "Transmit Pause Quantum Register"]
pub mod tpq;
#[doc = "TPSF (rw) register accessor: TX partial store and forward Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tpsf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tpsf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tpsf`]
module"]
pub type TPSF = crate::Reg<tpsf::TPSF_SPEC>;
#[doc = "TX partial store and forward Register"]
pub mod tpsf;
#[doc = "RPSF (rw) register accessor: RX partial store and forward Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rpsf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rpsf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rpsf`]
module"]
pub type RPSF = crate::Reg<rpsf::RPSF_SPEC>;
#[doc = "RX partial store and forward Register"]
pub mod rpsf;
#[doc = "RJFML (rw) register accessor: RX Jumbo Frame Max Length Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rjfml::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rjfml::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rjfml`]
module"]
pub type RJFML = crate::Reg<rjfml::RJFML_SPEC>;
#[doc = "RX Jumbo Frame Max Length Register"]
pub mod rjfml;
#[doc = "HRB (rw) register accessor: Hash Register Bottom \\[31:0\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hrb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hrb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hrb`]
module"]
pub type HRB = crate::Reg<hrb::HRB_SPEC>;
#[doc = "Hash Register Bottom \\[31:0\\]"]
pub mod hrb;
#[doc = "HRT (rw) register accessor: Hash Register Top \\[63:32\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hrt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hrt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hrt`]
module"]
pub type HRT = crate::Reg<hrt::HRT_SPEC>;
#[doc = "Hash Register Top \\[63:32\\]"]
pub mod hrt;
#[doc = "SA\\[%s\\]"]
pub use self::sa::SA;
#[doc = r"Cluster"]
#[doc = "SA\\[%s\\]"]
pub mod sa;
#[doc = "TIDM (rw) register accessor: Type ID Match Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tidm::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tidm::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tidm`]
module"]
pub type TIDM = crate::Reg<tidm::TIDM_SPEC>;
#[doc = "Type ID Match Register"]
pub mod tidm;
#[doc = "WOL (rw) register accessor: Wake on LAN\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wol::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wol::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wol`]
module"]
pub type WOL = crate::Reg<wol::WOL_SPEC>;
#[doc = "Wake on LAN"]
pub mod wol;
#[doc = "IPGS (rw) register accessor: IPG Stretch Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ipgs::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ipgs::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ipgs`]
module"]
pub type IPGS = crate::Reg<ipgs::IPGS_SPEC>;
#[doc = "IPG Stretch Register"]
pub mod ipgs;
#[doc = "SVLAN (rw) register accessor: Stacked VLAN Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`svlan::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`svlan::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@svlan`]
module"]
pub type SVLAN = crate::Reg<svlan::SVLAN_SPEC>;
#[doc = "Stacked VLAN Register"]
pub mod svlan;
#[doc = "TPFCP (rw) register accessor: Transmit PFC Pause Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tpfcp::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tpfcp::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tpfcp`]
module"]
pub type TPFCP = crate::Reg<tpfcp::TPFCP_SPEC>;
#[doc = "Transmit PFC Pause Register"]
pub mod tpfcp;
#[doc = "SAMB1 (rw) register accessor: Specific Address 1 Mask Bottom \\[31:0\\]
Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`samb1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`samb1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@samb1`]
module"]
pub type SAMB1 = crate::Reg<samb1::SAMB1_SPEC>;
#[doc = "Specific Address 1 Mask Bottom \\[31:0\\]
Register"]
pub mod samb1;
#[doc = "SAMT1 (rw) register accessor: Specific Address 1 Mask Top \\[47:32\\]
Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`samt1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`samt1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@samt1`]
module"]
pub type SAMT1 = crate::Reg<samt1::SAMT1_SPEC>;
#[doc = "Specific Address 1 Mask Top \\[47:32\\]
Register"]
pub mod samt1;
#[doc = "NSC (rw) register accessor: Tsu timer comparison nanoseconds Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nsc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nsc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nsc`]
module"]
pub type NSC = crate::Reg<nsc::NSC_SPEC>;
#[doc = "Tsu timer comparison nanoseconds Register"]
pub mod nsc;
#[doc = "SCL (rw) register accessor: Tsu timer second comparison Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scl`]
module"]
pub type SCL = crate::Reg<scl::SCL_SPEC>;
#[doc = "Tsu timer second comparison Register"]
pub mod scl;
#[doc = "SCH (rw) register accessor: Tsu timer second comparison Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sch::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sch::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sch`]
module"]
pub type SCH = crate::Reg<sch::SCH_SPEC>;
#[doc = "Tsu timer second comparison Register"]
pub mod sch;
#[doc = "EFTSH (r) register accessor: PTP Event Frame Transmitted Seconds High Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eftsh::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eftsh`]
module"]
pub type EFTSH = crate::Reg<eftsh::EFTSH_SPEC>;
#[doc = "PTP Event Frame Transmitted Seconds High Register"]
pub mod eftsh;
#[doc = "EFRSH (r) register accessor: PTP Event Frame Received Seconds High Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`efrsh::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@efrsh`]
module"]
pub type EFRSH = crate::Reg<efrsh::EFRSH_SPEC>;
#[doc = "PTP Event Frame Received Seconds High Register"]
pub mod efrsh;
#[doc = "PEFTSH (r) register accessor: PTP Peer Event Frame Transmitted Seconds High Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`peftsh::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peftsh`]
module"]
pub type PEFTSH = crate::Reg<peftsh::PEFTSH_SPEC>;
#[doc = "PTP Peer Event Frame Transmitted Seconds High Register"]
pub mod peftsh;
#[doc = "PEFRSH (r) register accessor: PTP Peer Event Frame Received Seconds High Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pefrsh::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pefrsh`]
module"]
pub type PEFRSH = crate::Reg<pefrsh::PEFRSH_SPEC>;
#[doc = "PTP Peer Event Frame Received Seconds High Register"]
pub mod pefrsh;
#[doc = "OTLO (r) register accessor: Octets Transmitted \\[31:0\\]
Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otlo::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otlo`]
module"]
pub type OTLO = crate::Reg<otlo::OTLO_SPEC>;
#[doc = "Octets Transmitted \\[31:0\\]
Register"]
pub mod otlo;
#[doc = "OTHI (r) register accessor: Octets Transmitted \\[47:32\\]
Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`othi::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@othi`]
module"]
pub type OTHI = crate::Reg<othi::OTHI_SPEC>;
#[doc = "Octets Transmitted \\[47:32\\]
Register"]
pub mod othi;
#[doc = "FT (r) register accessor: Frames Transmitted Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ft::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ft`]
module"]
pub type FT = crate::Reg<ft::FT_SPEC>;
#[doc = "Frames Transmitted Register"]
pub mod ft;
#[doc = "BCFT (r) register accessor: Broadcast Frames Transmitted Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bcft::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bcft`]
module"]
pub type BCFT = crate::Reg<bcft::BCFT_SPEC>;
#[doc = "Broadcast Frames Transmitted Register"]
pub mod bcft;
#[doc = "MFT (r) register accessor: Multicast Frames Transmitted Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mft::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mft`]
module"]
pub type MFT = crate::Reg<mft::MFT_SPEC>;
#[doc = "Multicast Frames Transmitted Register"]
pub mod mft;
#[doc = "PFT (r) register accessor: Pause Frames Transmitted Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pft::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pft`]
module"]
pub type PFT = crate::Reg<pft::PFT_SPEC>;
#[doc = "Pause Frames Transmitted Register"]
pub mod pft;
#[doc = "BFT64 (r) register accessor: 64 Byte Frames Transmitted Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bft64::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bft64`]
module"]
pub type BFT64 = crate::Reg<bft64::BFT64_SPEC>;
#[doc = "64 Byte Frames Transmitted Register"]
pub mod bft64;
#[doc = "TBFT127 (r) register accessor: 65 to 127 Byte Frames Transmitted Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tbft127::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tbft127`]
module"]
pub type TBFT127 = crate::Reg<tbft127::TBFT127_SPEC>;
#[doc = "65 to 127 Byte Frames Transmitted Register"]
pub mod tbft127;
#[doc = "TBFT255 (r) register accessor: 128 to 255 Byte Frames Transmitted Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tbft255::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tbft255`]
module"]
pub type TBFT255 = crate::Reg<tbft255::TBFT255_SPEC>;
#[doc = "128 to 255 Byte Frames Transmitted Register"]
pub mod tbft255;
#[doc = "TBFT511 (r) register accessor: 256 to 511 Byte Frames Transmitted Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tbft511::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tbft511`]
module"]
pub type TBFT511 = crate::Reg<tbft511::TBFT511_SPEC>;
#[doc = "256 to 511 Byte Frames Transmitted Register"]
pub mod tbft511;
#[doc = "TBFT1023 (r) register accessor: 512 to 1023 Byte Frames Transmitted Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tbft1023::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tbft1023`]
module"]
pub type TBFT1023 = crate::Reg<tbft1023::TBFT1023_SPEC>;
#[doc = "512 to 1023 Byte Frames Transmitted Register"]
pub mod tbft1023;
#[doc = "TBFT1518 (r) register accessor: 1024 to 1518 Byte Frames Transmitted Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tbft1518::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tbft1518`]
module"]
pub type TBFT1518 = crate::Reg<tbft1518::TBFT1518_SPEC>;
#[doc = "1024 to 1518 Byte Frames Transmitted Register"]
pub mod tbft1518;
#[doc = "GTBFT1518 (r) register accessor: Greater Than 1518 Byte Frames Transmitted Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtbft1518::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtbft1518`]
module"]
pub type GTBFT1518 = crate::Reg<gtbft1518::GTBFT1518_SPEC>;
#[doc = "Greater Than 1518 Byte Frames Transmitted Register"]
pub mod gtbft1518;
#[doc = "TUR (r) register accessor: Transmit Underruns Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tur::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tur`]
module"]
pub type TUR = crate::Reg<tur::TUR_SPEC>;
#[doc = "Transmit Underruns Register"]
pub mod tur;
#[doc = "SCF (r) register accessor: Single Collision Frames Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scf::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scf`]
module"]
pub type SCF = crate::Reg<scf::SCF_SPEC>;
#[doc = "Single Collision Frames Register"]
pub mod scf;
#[doc = "MCF (r) register accessor: Multiple Collision Frames Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcf::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcf`]
module"]
pub type MCF = crate::Reg<mcf::MCF_SPEC>;
#[doc = "Multiple Collision Frames Register"]
pub mod mcf;
#[doc = "EC (r) register accessor: Excessive Collisions Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ec::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ec`]
module"]
pub type EC = crate::Reg<ec::EC_SPEC>;
#[doc = "Excessive Collisions Register"]
pub mod ec;
#[doc = "LC (r) register accessor: Late Collisions Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lc::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lc`]
module"]
pub type LC = crate::Reg<lc::LC_SPEC>;
#[doc = "Late Collisions Register"]
pub mod lc;
#[doc = "DTF (r) register accessor: Deferred Transmission Frames Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dtf::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dtf`]
module"]
pub type DTF = crate::Reg<dtf::DTF_SPEC>;
#[doc = "Deferred Transmission Frames Register"]
pub mod dtf;
#[doc = "CSE (r) register accessor: Carrier Sense Errors Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cse::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cse`]
module"]
pub type CSE = crate::Reg<cse::CSE_SPEC>;
#[doc = "Carrier Sense Errors Register"]
pub mod cse;
#[doc = "ORLO (r) register accessor: Octets Received \\[31:0\\]
Received\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`orlo::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@orlo`]
module"]
pub type ORLO = crate::Reg<orlo::ORLO_SPEC>;
#[doc = "Octets Received \\[31:0\\]
Received"]
pub mod orlo;
#[doc = "ORHI (r) register accessor: Octets Received \\[47:32\\]
Received\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`orhi::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@orhi`]
module"]
pub type ORHI = crate::Reg<orhi::ORHI_SPEC>;
#[doc = "Octets Received \\[47:32\\]
Received"]
pub mod orhi;
#[doc = "FR (r) register accessor: Frames Received Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fr`]
module"]
pub type FR = crate::Reg<fr::FR_SPEC>;
#[doc = "Frames Received Register"]
pub mod fr;
#[doc = "BCFR (r) register accessor: Broadcast Frames Received Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bcfr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bcfr`]
module"]
pub type BCFR = crate::Reg<bcfr::BCFR_SPEC>;
#[doc = "Broadcast Frames Received Register"]
pub mod bcfr;
#[doc = "MFR (r) register accessor: Multicast Frames Received Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mfr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mfr`]
module"]
pub type MFR = crate::Reg<mfr::MFR_SPEC>;
#[doc = "Multicast Frames Received Register"]
pub mod mfr;
#[doc = "PFR (r) register accessor: Pause Frames Received Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pfr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pfr`]
module"]
pub type PFR = crate::Reg<pfr::PFR_SPEC>;
#[doc = "Pause Frames Received Register"]
pub mod pfr;
#[doc = "BFR64 (r) register accessor: 64 Byte Frames Received Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bfr64::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bfr64`]
module"]
pub type BFR64 = crate::Reg<bfr64::BFR64_SPEC>;
#[doc = "64 Byte Frames Received Register"]
pub mod bfr64;
#[doc = "TBFR127 (r) register accessor: 65 to 127 Byte Frames Received Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tbfr127::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tbfr127`]
module"]
pub type TBFR127 = crate::Reg<tbfr127::TBFR127_SPEC>;
#[doc = "65 to 127 Byte Frames Received Register"]
pub mod tbfr127;
#[doc = "TBFR255 (r) register accessor: 128 to 255 Byte Frames Received Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tbfr255::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tbfr255`]
module"]
pub type TBFR255 = crate::Reg<tbfr255::TBFR255_SPEC>;
#[doc = "128 to 255 Byte Frames Received Register"]
pub mod tbfr255;
#[doc = "TBFR511 (r) register accessor: 256 to 511Byte Frames Received Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tbfr511::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tbfr511`]
module"]
pub type TBFR511 = crate::Reg<tbfr511::TBFR511_SPEC>;
#[doc = "256 to 511Byte Frames Received Register"]
pub mod tbfr511;
#[doc = "TBFR1023 (r) register accessor: 512 to 1023 Byte Frames Received Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tbfr1023::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tbfr1023`]
module"]
pub type TBFR1023 = crate::Reg<tbfr1023::TBFR1023_SPEC>;
#[doc = "512 to 1023 Byte Frames Received Register"]
pub mod tbfr1023;
#[doc = "TBFR1518 (r) register accessor: 1024 to 1518 Byte Frames Received Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tbfr1518::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tbfr1518`]
module"]
pub type TBFR1518 = crate::Reg<tbfr1518::TBFR1518_SPEC>;
#[doc = "1024 to 1518 Byte Frames Received Register"]
pub mod tbfr1518;
#[doc = "TMXBFR (r) register accessor: 1519 to Maximum Byte Frames Received Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tmxbfr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tmxbfr`]
module"]
pub type TMXBFR = crate::Reg<tmxbfr::TMXBFR_SPEC>;
#[doc = "1519 to Maximum Byte Frames Received Register"]
pub mod tmxbfr;
#[doc = "UFR (r) register accessor: Undersize Frames Received Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ufr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ufr`]
module"]
pub type UFR = crate::Reg<ufr::UFR_SPEC>;
#[doc = "Undersize Frames Received Register"]
pub mod ufr;
#[doc = "OFR (r) register accessor: Oversize Frames Received Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ofr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ofr`]
module"]
pub type OFR = crate::Reg<ofr::OFR_SPEC>;
#[doc = "Oversize Frames Received Register"]
pub mod ofr;
#[doc = "JR (r) register accessor: Jabbers Received Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`jr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@jr`]
module"]
pub type JR = crate::Reg<jr::JR_SPEC>;
#[doc = "Jabbers Received Register"]
pub mod jr;
#[doc = "FCSE (r) register accessor: Frame Check Sequence Errors Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fcse::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fcse`]
module"]
pub type FCSE = crate::Reg<fcse::FCSE_SPEC>;
#[doc = "Frame Check Sequence Errors Register"]
pub mod fcse;
#[doc = "LFFE (r) register accessor: Length Field Frame Errors Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lffe::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lffe`]
module"]
pub type LFFE = crate::Reg<lffe::LFFE_SPEC>;
#[doc = "Length Field Frame Errors Register"]
pub mod lffe;
#[doc = "RSE (r) register accessor: Receive Symbol Errors Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rse::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rse`]
module"]
pub type RSE = crate::Reg<rse::RSE_SPEC>;
#[doc = "Receive Symbol Errors Register"]
pub mod rse;
#[doc = "AE (r) register accessor: Alignment Errors Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ae::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ae`]
module"]
pub type AE = crate::Reg<ae::AE_SPEC>;
#[doc = "Alignment Errors Register"]
pub mod ae;
#[doc = "RRE (r) register accessor: Receive Resource Errors Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rre::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rre`]
module"]
pub type RRE = crate::Reg<rre::RRE_SPEC>;
#[doc = "Receive Resource Errors Register"]
pub mod rre;
#[doc = "ROE (r) register accessor: Receive Overrun Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`roe::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@roe`]
module"]
pub type ROE = crate::Reg<roe::ROE_SPEC>;
#[doc = "Receive Overrun Register"]
pub mod roe;
#[doc = "IHCE (r) register accessor: IP Header Checksum Errors Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ihce::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ihce`]
module"]
pub type IHCE = crate::Reg<ihce::IHCE_SPEC>;
#[doc = "IP Header Checksum Errors Register"]
pub mod ihce;
#[doc = "TCE (r) register accessor: TCP Checksum Errors Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tce::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tce`]
module"]
pub type TCE = crate::Reg<tce::TCE_SPEC>;
#[doc = "TCP Checksum Errors Register"]
pub mod tce;
#[doc = "UCE (r) register accessor: UDP Checksum Errors Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uce::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uce`]
module"]
pub type UCE = crate::Reg<uce::UCE_SPEC>;
#[doc = "UDP Checksum Errors Register"]
pub mod uce;
#[doc = "TISUBN (rw) register accessor: 1588 Timer Increment \\[15:0\\]
Sub-Nanoseconds Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tisubn::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tisubn::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tisubn`]
module"]
pub type TISUBN = crate::Reg<tisubn::TISUBN_SPEC>;
#[doc = "1588 Timer Increment \\[15:0\\]
Sub-Nanoseconds Register"]
pub mod tisubn;
#[doc = "TSH (rw) register accessor: 1588 Timer Seconds High \\[15:0\\]
Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tsh::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tsh::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tsh`]
module"]
pub type TSH = crate::Reg<tsh::TSH_SPEC>;
#[doc = "1588 Timer Seconds High \\[15:0\\]
Register"]
pub mod tsh;
#[doc = "TSSSL (rw) register accessor: 1588 Timer Sync Strobe Seconds \\[31:0\\]
Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tsssl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tsssl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tsssl`]
module"]
pub type TSSSL = crate::Reg<tsssl::TSSSL_SPEC>;
#[doc = "1588 Timer Sync Strobe Seconds \\[31:0\\]
Register"]
pub mod tsssl;
#[doc = "TSSN (rw) register accessor: 1588 Timer Sync Strobe Nanoseconds Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tssn::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tssn::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tssn`]
module"]
pub type TSSN = crate::Reg<tssn::TSSN_SPEC>;
#[doc = "1588 Timer Sync Strobe Nanoseconds Register"]
pub mod tssn;
#[doc = "TSL (rw) register accessor: 1588 Timer Seconds \\[31:0\\]
Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tsl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tsl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tsl`]
module"]
pub type TSL = crate::Reg<tsl::TSL_SPEC>;
#[doc = "1588 Timer Seconds \\[31:0\\]
Register"]
pub mod tsl;
#[doc = "TN (rw) register accessor: 1588 Timer Nanoseconds Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tn::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tn::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tn`]
module"]
pub type TN = crate::Reg<tn::TN_SPEC>;
#[doc = "1588 Timer Nanoseconds Register"]
pub mod tn;
#[doc = "TA (w) register accessor: 1588 Timer Adjust Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ta::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ta`]
module"]
pub type TA = crate::Reg<ta::TA_SPEC>;
#[doc = "1588 Timer Adjust Register"]
pub mod ta;
#[doc = "TI (rw) register accessor: 1588 Timer Increment Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ti::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ti::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ti`]
module"]
pub type TI = crate::Reg<ti::TI_SPEC>;
#[doc = "1588 Timer Increment Register"]
pub mod ti;
#[doc = "EFTSL (r) register accessor: PTP Event Frame Transmitted Seconds Low Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eftsl::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eftsl`]
module"]
pub type EFTSL = crate::Reg<eftsl::EFTSL_SPEC>;
#[doc = "PTP Event Frame Transmitted Seconds Low Register"]
pub mod eftsl;
#[doc = "EFTN (r) register accessor: PTP Event Frame Transmitted Nanoseconds\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eftn::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eftn`]
module"]
pub type EFTN = crate::Reg<eftn::EFTN_SPEC>;
#[doc = "PTP Event Frame Transmitted Nanoseconds"]
pub mod eftn;
#[doc = "EFRSL (r) register accessor: PTP Event Frame Received Seconds Low Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`efrsl::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@efrsl`]
module"]
pub type EFRSL = crate::Reg<efrsl::EFRSL_SPEC>;
#[doc = "PTP Event Frame Received Seconds Low Register"]
pub mod efrsl;
#[doc = "EFRN (r) register accessor: PTP Event Frame Received Nanoseconds\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`efrn::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@efrn`]
module"]
pub type EFRN = crate::Reg<efrn::EFRN_SPEC>;
#[doc = "PTP Event Frame Received Nanoseconds"]
pub mod efrn;
#[doc = "PEFTSL (r) register accessor: PTP Peer Event Frame Transmitted Seconds Low Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`peftsl::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peftsl`]
module"]
pub type PEFTSL = crate::Reg<peftsl::PEFTSL_SPEC>;
#[doc = "PTP Peer Event Frame Transmitted Seconds Low Register"]
pub mod peftsl;
#[doc = "PEFTN (r) register accessor: PTP Peer Event Frame Transmitted Nanoseconds\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`peftn::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peftn`]
module"]
pub type PEFTN = crate::Reg<peftn::PEFTN_SPEC>;
#[doc = "PTP Peer Event Frame Transmitted Nanoseconds"]
pub mod peftn;
#[doc = "PEFRSL (r) register accessor: PTP Peer Event Frame Received Seconds Low Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pefrsl::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pefrsl`]
module"]
pub type PEFRSL = crate::Reg<pefrsl::PEFRSL_SPEC>;
#[doc = "PTP Peer Event Frame Received Seconds Low Register"]
pub mod pefrsl;
#[doc = "PEFRN (r) register accessor: PTP Peer Event Frame Received Nanoseconds\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pefrn::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pefrn`]
module"]
pub type PEFRN = crate::Reg<pefrn::PEFRN_SPEC>;
#[doc = "PTP Peer Event Frame Received Nanoseconds"]
pub mod pefrn;
#[doc = "RLPITR (r) register accessor: Receive LPI transition Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rlpitr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rlpitr`]
module"]
pub type RLPITR = crate::Reg<rlpitr::RLPITR_SPEC>;
#[doc = "Receive LPI transition Register"]
pub mod rlpitr;
#[doc = "RLPITI (r) register accessor: Receive LPI Time Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rlpiti::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rlpiti`]
module"]
pub type RLPITI = crate::Reg<rlpiti::RLPITI_SPEC>;
#[doc = "Receive LPI Time Register"]
pub mod rlpiti;
#[doc = "TLPITR (r) register accessor: Receive LPI transition Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tlpitr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tlpitr`]
module"]
pub type TLPITR = crate::Reg<tlpitr::TLPITR_SPEC>;
#[doc = "Receive LPI transition Register"]
pub mod tlpitr;
#[doc = "TLPITI (r) register accessor: Receive LPI Time Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tlpiti::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tlpiti`]
module"]
pub type TLPITI = crate::Reg<tlpiti::TLPITI_SPEC>;
#[doc = "Receive LPI Time Register"]
pub mod tlpiti;
