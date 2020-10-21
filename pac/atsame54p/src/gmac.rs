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
    _reserved19: [u8; 52usize],
    #[doc = "0x80 - Hash Register Bottom \\[31:0\\]"]
    pub hrb: HRB,
    #[doc = "0x84 - Hash Register Top \\[63:32\\]"]
    pub hrt: HRT,
    #[doc = "0x88 - SA\\[%s\\]"]
    pub sa: [SA; 4],
    #[doc = "0xa8 - Type ID Match Register"]
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
    _reserved29: [u8; 12usize],
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
    _reserved36: [u8; 8usize],
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
    _reserved81: [u8; 8usize],
    #[doc = "0x1bc - 1588 Timer Increment \\[15:0\\]
Sub-Nanoseconds Register"]
    pub tisubn: TISUBN,
    #[doc = "0x1c0 - 1588 Timer Seconds High \\[15:0\\]
Register"]
    pub tsh: TSH,
    _reserved83: [u8; 4usize],
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
    _reserved97: [u8; 112usize],
    #[doc = "0x270 - Receive LPI transition Register"]
    pub rlpitr: RLPITR,
    #[doc = "0x274 - Receive LPI Time Register"]
    pub rlpiti: RLPITI,
    #[doc = "0x278 - Receive LPI transition Register"]
    pub tlpitr: TLPITR,
    #[doc = "0x27c - Receive LPI Time Register"]
    pub tlpiti: TLPITI,
}
#[doc = r"Register block"]
#[repr(C)]
pub struct SA {
    #[doc = "0x00 - Specific Address Bottom \\[31:0\\]
Register"]
    pub sab: self::sa::SAB,
    #[doc = "0x04 - Specific Address Top \\[47:32\\]
Register"]
    pub sat: self::sa::SAT,
}
#[doc = r"Register block"]
#[doc = "SA\\[%s\\]"]
pub mod sa;
#[doc = "Network Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ncr](ncr) module"]
pub type NCR = crate::Reg<u32, _NCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NCR;
#[doc = "`read()` method returns [ncr::R](ncr::R) reader structure"]
impl crate::Readable for NCR {}
#[doc = "`write(|w| ..)` method takes [ncr::W](ncr::W) writer structure"]
impl crate::Writable for NCR {}
#[doc = "Network Control Register"]
pub mod ncr;
#[doc = "Network Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ncfgr](ncfgr) module"]
pub type NCFGR = crate::Reg<u32, _NCFGR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NCFGR;
#[doc = "`read()` method returns [ncfgr::R](ncfgr::R) reader structure"]
impl crate::Readable for NCFGR {}
#[doc = "`write(|w| ..)` method takes [ncfgr::W](ncfgr::W) writer structure"]
impl crate::Writable for NCFGR {}
#[doc = "Network Configuration Register"]
pub mod ncfgr;
#[doc = "Network Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nsr](nsr) module"]
pub type NSR = crate::Reg<u32, _NSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NSR;
#[doc = "`read()` method returns [nsr::R](nsr::R) reader structure"]
impl crate::Readable for NSR {}
#[doc = "Network Status Register"]
pub mod nsr;
#[doc = "User Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ur](ur) module"]
pub type UR = crate::Reg<u32, _UR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UR;
#[doc = "`read()` method returns [ur::R](ur::R) reader structure"]
impl crate::Readable for UR {}
#[doc = "`write(|w| ..)` method takes [ur::W](ur::W) writer structure"]
impl crate::Writable for UR {}
#[doc = "User Register"]
pub mod ur;
#[doc = "DMA Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcfgr](dcfgr) module"]
pub type DCFGR = crate::Reg<u32, _DCFGR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCFGR;
#[doc = "`read()` method returns [dcfgr::R](dcfgr::R) reader structure"]
impl crate::Readable for DCFGR {}
#[doc = "`write(|w| ..)` method takes [dcfgr::W](dcfgr::W) writer structure"]
impl crate::Writable for DCFGR {}
#[doc = "DMA Configuration Register"]
pub mod dcfgr;
#[doc = "Transmit Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tsr](tsr) module"]
pub type TSR = crate::Reg<u32, _TSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TSR;
#[doc = "`read()` method returns [tsr::R](tsr::R) reader structure"]
impl crate::Readable for TSR {}
#[doc = "`write(|w| ..)` method takes [tsr::W](tsr::W) writer structure"]
impl crate::Writable for TSR {}
#[doc = "Transmit Status Register"]
pub mod tsr;
#[doc = "Receive Buffer Queue Base Address\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rbqb](rbqb) module"]
pub type RBQB = crate::Reg<u32, _RBQB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RBQB;
#[doc = "`read()` method returns [rbqb::R](rbqb::R) reader structure"]
impl crate::Readable for RBQB {}
#[doc = "`write(|w| ..)` method takes [rbqb::W](rbqb::W) writer structure"]
impl crate::Writable for RBQB {}
#[doc = "Receive Buffer Queue Base Address"]
pub mod rbqb;
#[doc = "Transmit Buffer Queue Base Address\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tbqb](tbqb) module"]
pub type TBQB = crate::Reg<u32, _TBQB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TBQB;
#[doc = "`read()` method returns [tbqb::R](tbqb::R) reader structure"]
impl crate::Readable for TBQB {}
#[doc = "`write(|w| ..)` method takes [tbqb::W](tbqb::W) writer structure"]
impl crate::Writable for TBQB {}
#[doc = "Transmit Buffer Queue Base Address"]
pub mod tbqb;
#[doc = "Receive Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rsr](rsr) module"]
pub type RSR = crate::Reg<u32, _RSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RSR;
#[doc = "`read()` method returns [rsr::R](rsr::R) reader structure"]
impl crate::Readable for RSR {}
#[doc = "`write(|w| ..)` method takes [rsr::W](rsr::W) writer structure"]
impl crate::Writable for RSR {}
#[doc = "Receive Status Register"]
pub mod rsr;
#[doc = "Interrupt Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [isr](isr) module"]
pub type ISR = crate::Reg<u32, _ISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ISR;
#[doc = "`read()` method returns [isr::R](isr::R) reader structure"]
impl crate::Readable for ISR {}
#[doc = "`write(|w| ..)` method takes [isr::W](isr::W) writer structure"]
impl crate::Writable for ISR {}
#[doc = "Interrupt Status Register"]
pub mod isr;
#[doc = "Interrupt Enable Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ier](ier) module"]
pub type IER = crate::Reg<u32, _IER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IER;
#[doc = "`write(|w| ..)` method takes [ier::W](ier::W) writer structure"]
impl crate::Writable for IER {}
#[doc = "Interrupt Enable Register"]
pub mod ier;
#[doc = "Interrupt Disable Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [idr](idr) module"]
pub type IDR = crate::Reg<u32, _IDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IDR;
#[doc = "`write(|w| ..)` method takes [idr::W](idr::W) writer structure"]
impl crate::Writable for IDR {}
#[doc = "Interrupt Disable Register"]
pub mod idr;
#[doc = "Interrupt Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [imr](imr) module"]
pub type IMR = crate::Reg<u32, _IMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IMR;
#[doc = "`read()` method returns [imr::R](imr::R) reader structure"]
impl crate::Readable for IMR {}
#[doc = "Interrupt Mask Register"]
pub mod imr;
#[doc = "PHY Maintenance Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [man](man) module"]
pub type MAN = crate::Reg<u32, _MAN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MAN;
#[doc = "`read()` method returns [man::R](man::R) reader structure"]
impl crate::Readable for MAN {}
#[doc = "`write(|w| ..)` method takes [man::W](man::W) writer structure"]
impl crate::Writable for MAN {}
#[doc = "PHY Maintenance Register"]
pub mod man;
#[doc = "Received Pause Quantum Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rpq](rpq) module"]
pub type RPQ = crate::Reg<u32, _RPQ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RPQ;
#[doc = "`read()` method returns [rpq::R](rpq::R) reader structure"]
impl crate::Readable for RPQ {}
#[doc = "Received Pause Quantum Register"]
pub mod rpq;
#[doc = "Transmit Pause Quantum Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tpq](tpq) module"]
pub type TPQ = crate::Reg<u32, _TPQ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TPQ;
#[doc = "`read()` method returns [tpq::R](tpq::R) reader structure"]
impl crate::Readable for TPQ {}
#[doc = "`write(|w| ..)` method takes [tpq::W](tpq::W) writer structure"]
impl crate::Writable for TPQ {}
#[doc = "Transmit Pause Quantum Register"]
pub mod tpq;
#[doc = "TX partial store and forward Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tpsf](tpsf) module"]
pub type TPSF = crate::Reg<u32, _TPSF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TPSF;
#[doc = "`read()` method returns [tpsf::R](tpsf::R) reader structure"]
impl crate::Readable for TPSF {}
#[doc = "`write(|w| ..)` method takes [tpsf::W](tpsf::W) writer structure"]
impl crate::Writable for TPSF {}
#[doc = "TX partial store and forward Register"]
pub mod tpsf;
#[doc = "RX partial store and forward Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rpsf](rpsf) module"]
pub type RPSF = crate::Reg<u32, _RPSF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RPSF;
#[doc = "`read()` method returns [rpsf::R](rpsf::R) reader structure"]
impl crate::Readable for RPSF {}
#[doc = "`write(|w| ..)` method takes [rpsf::W](rpsf::W) writer structure"]
impl crate::Writable for RPSF {}
#[doc = "RX partial store and forward Register"]
pub mod rpsf;
#[doc = "RX Jumbo Frame Max Length Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rjfml](rjfml) module"]
pub type RJFML = crate::Reg<u32, _RJFML>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RJFML;
#[doc = "`read()` method returns [rjfml::R](rjfml::R) reader structure"]
impl crate::Readable for RJFML {}
#[doc = "`write(|w| ..)` method takes [rjfml::W](rjfml::W) writer structure"]
impl crate::Writable for RJFML {}
#[doc = "RX Jumbo Frame Max Length Register"]
pub mod rjfml;
#[doc = "Hash Register Bottom \\[31:0\\]\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hrb](hrb) module"]
pub type HRB = crate::Reg<u32, _HRB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HRB;
#[doc = "`read()` method returns [hrb::R](hrb::R) reader structure"]
impl crate::Readable for HRB {}
#[doc = "`write(|w| ..)` method takes [hrb::W](hrb::W) writer structure"]
impl crate::Writable for HRB {}
#[doc = "Hash Register Bottom \\[31:0\\]"]
pub mod hrb;
#[doc = "Hash Register Top \\[63:32\\]\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hrt](hrt) module"]
pub type HRT = crate::Reg<u32, _HRT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HRT;
#[doc = "`read()` method returns [hrt::R](hrt::R) reader structure"]
impl crate::Readable for HRT {}
#[doc = "`write(|w| ..)` method takes [hrt::W](hrt::W) writer structure"]
impl crate::Writable for HRT {}
#[doc = "Hash Register Top \\[63:32\\]"]
pub mod hrt;
#[doc = "Type ID Match Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tidm](tidm) module"]
pub type TIDM = crate::Reg<u32, _TIDM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIDM;
#[doc = "`read()` method returns [tidm::R](tidm::R) reader structure"]
impl crate::Readable for TIDM {}
#[doc = "`write(|w| ..)` method takes [tidm::W](tidm::W) writer structure"]
impl crate::Writable for TIDM {}
#[doc = "Type ID Match Register"]
pub mod tidm;
#[doc = "Wake on LAN\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wol](wol) module"]
pub type WOL = crate::Reg<u32, _WOL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WOL;
#[doc = "`read()` method returns [wol::R](wol::R) reader structure"]
impl crate::Readable for WOL {}
#[doc = "`write(|w| ..)` method takes [wol::W](wol::W) writer structure"]
impl crate::Writable for WOL {}
#[doc = "Wake on LAN"]
pub mod wol;
#[doc = "IPG Stretch Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ipgs](ipgs) module"]
pub type IPGS = crate::Reg<u32, _IPGS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IPGS;
#[doc = "`read()` method returns [ipgs::R](ipgs::R) reader structure"]
impl crate::Readable for IPGS {}
#[doc = "`write(|w| ..)` method takes [ipgs::W](ipgs::W) writer structure"]
impl crate::Writable for IPGS {}
#[doc = "IPG Stretch Register"]
pub mod ipgs;
#[doc = "Stacked VLAN Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [svlan](svlan) module"]
pub type SVLAN = crate::Reg<u32, _SVLAN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SVLAN;
#[doc = "`read()` method returns [svlan::R](svlan::R) reader structure"]
impl crate::Readable for SVLAN {}
#[doc = "`write(|w| ..)` method takes [svlan::W](svlan::W) writer structure"]
impl crate::Writable for SVLAN {}
#[doc = "Stacked VLAN Register"]
pub mod svlan;
#[doc = "Transmit PFC Pause Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tpfcp](tpfcp) module"]
pub type TPFCP = crate::Reg<u32, _TPFCP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TPFCP;
#[doc = "`read()` method returns [tpfcp::R](tpfcp::R) reader structure"]
impl crate::Readable for TPFCP {}
#[doc = "`write(|w| ..)` method takes [tpfcp::W](tpfcp::W) writer structure"]
impl crate::Writable for TPFCP {}
#[doc = "Transmit PFC Pause Register"]
pub mod tpfcp;
#[doc = "Specific Address 1 Mask Bottom \\[31:0\\]
Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [samb1](samb1) module"]
pub type SAMB1 = crate::Reg<u32, _SAMB1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SAMB1;
#[doc = "`read()` method returns [samb1::R](samb1::R) reader structure"]
impl crate::Readable for SAMB1 {}
#[doc = "`write(|w| ..)` method takes [samb1::W](samb1::W) writer structure"]
impl crate::Writable for SAMB1 {}
#[doc = "Specific Address 1 Mask Bottom \\[31:0\\]
Register"]
pub mod samb1;
#[doc = "Specific Address 1 Mask Top \\[47:32\\]
Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [samt1](samt1) module"]
pub type SAMT1 = crate::Reg<u32, _SAMT1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SAMT1;
#[doc = "`read()` method returns [samt1::R](samt1::R) reader structure"]
impl crate::Readable for SAMT1 {}
#[doc = "`write(|w| ..)` method takes [samt1::W](samt1::W) writer structure"]
impl crate::Writable for SAMT1 {}
#[doc = "Specific Address 1 Mask Top \\[47:32\\]
Register"]
pub mod samt1;
#[doc = "Tsu timer comparison nanoseconds Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nsc](nsc) module"]
pub type NSC = crate::Reg<u32, _NSC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NSC;
#[doc = "`read()` method returns [nsc::R](nsc::R) reader structure"]
impl crate::Readable for NSC {}
#[doc = "`write(|w| ..)` method takes [nsc::W](nsc::W) writer structure"]
impl crate::Writable for NSC {}
#[doc = "Tsu timer comparison nanoseconds Register"]
pub mod nsc;
#[doc = "Tsu timer second comparison Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scl](scl) module"]
pub type SCL = crate::Reg<u32, _SCL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCL;
#[doc = "`read()` method returns [scl::R](scl::R) reader structure"]
impl crate::Readable for SCL {}
#[doc = "`write(|w| ..)` method takes [scl::W](scl::W) writer structure"]
impl crate::Writable for SCL {}
#[doc = "Tsu timer second comparison Register"]
pub mod scl;
#[doc = "Tsu timer second comparison Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sch](sch) module"]
pub type SCH = crate::Reg<u32, _SCH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCH;
#[doc = "`read()` method returns [sch::R](sch::R) reader structure"]
impl crate::Readable for SCH {}
#[doc = "`write(|w| ..)` method takes [sch::W](sch::W) writer structure"]
impl crate::Writable for SCH {}
#[doc = "Tsu timer second comparison Register"]
pub mod sch;
#[doc = "PTP Event Frame Transmitted Seconds High Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eftsh](eftsh) module"]
pub type EFTSH = crate::Reg<u32, _EFTSH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFTSH;
#[doc = "`read()` method returns [eftsh::R](eftsh::R) reader structure"]
impl crate::Readable for EFTSH {}
#[doc = "PTP Event Frame Transmitted Seconds High Register"]
pub mod eftsh;
#[doc = "PTP Event Frame Received Seconds High Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [efrsh](efrsh) module"]
pub type EFRSH = crate::Reg<u32, _EFRSH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFRSH;
#[doc = "`read()` method returns [efrsh::R](efrsh::R) reader structure"]
impl crate::Readable for EFRSH {}
#[doc = "PTP Event Frame Received Seconds High Register"]
pub mod efrsh;
#[doc = "PTP Peer Event Frame Transmitted Seconds High Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [peftsh](peftsh) module"]
pub type PEFTSH = crate::Reg<u32, _PEFTSH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PEFTSH;
#[doc = "`read()` method returns [peftsh::R](peftsh::R) reader structure"]
impl crate::Readable for PEFTSH {}
#[doc = "PTP Peer Event Frame Transmitted Seconds High Register"]
pub mod peftsh;
#[doc = "PTP Peer Event Frame Received Seconds High Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pefrsh](pefrsh) module"]
pub type PEFRSH = crate::Reg<u32, _PEFRSH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PEFRSH;
#[doc = "`read()` method returns [pefrsh::R](pefrsh::R) reader structure"]
impl crate::Readable for PEFRSH {}
#[doc = "PTP Peer Event Frame Received Seconds High Register"]
pub mod pefrsh;
#[doc = "Octets Transmitted \\[31:0\\]
Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otlo](otlo) module"]
pub type OTLO = crate::Reg<u32, _OTLO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTLO;
#[doc = "`read()` method returns [otlo::R](otlo::R) reader structure"]
impl crate::Readable for OTLO {}
#[doc = "Octets Transmitted \\[31:0\\]
Register"]
pub mod otlo;
#[doc = "Octets Transmitted \\[47:32\\]
Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [othi](othi) module"]
pub type OTHI = crate::Reg<u32, _OTHI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTHI;
#[doc = "`read()` method returns [othi::R](othi::R) reader structure"]
impl crate::Readable for OTHI {}
#[doc = "Octets Transmitted \\[47:32\\]
Register"]
pub mod othi;
#[doc = "Frames Transmitted Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ft](ft) module"]
pub type FT = crate::Reg<u32, _FT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FT;
#[doc = "`read()` method returns [ft::R](ft::R) reader structure"]
impl crate::Readable for FT {}
#[doc = "Frames Transmitted Register"]
pub mod ft;
#[doc = "Broadcast Frames Transmitted Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bcft](bcft) module"]
pub type BCFT = crate::Reg<u32, _BCFT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BCFT;
#[doc = "`read()` method returns [bcft::R](bcft::R) reader structure"]
impl crate::Readable for BCFT {}
#[doc = "Broadcast Frames Transmitted Register"]
pub mod bcft;
#[doc = "Multicast Frames Transmitted Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mft](mft) module"]
pub type MFT = crate::Reg<u32, _MFT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MFT;
#[doc = "`read()` method returns [mft::R](mft::R) reader structure"]
impl crate::Readable for MFT {}
#[doc = "Multicast Frames Transmitted Register"]
pub mod mft;
#[doc = "Pause Frames Transmitted Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pft](pft) module"]
pub type PFT = crate::Reg<u32, _PFT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PFT;
#[doc = "`read()` method returns [pft::R](pft::R) reader structure"]
impl crate::Readable for PFT {}
#[doc = "Pause Frames Transmitted Register"]
pub mod pft;
#[doc = "64 Byte Frames Transmitted Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bft64](bft64) module"]
pub type BFT64 = crate::Reg<u32, _BFT64>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BFT64;
#[doc = "`read()` method returns [bft64::R](bft64::R) reader structure"]
impl crate::Readable for BFT64 {}
#[doc = "64 Byte Frames Transmitted Register"]
pub mod bft64;
#[doc = "65 to 127 Byte Frames Transmitted Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tbft127](tbft127) module"]
pub type TBFT127 = crate::Reg<u32, _TBFT127>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TBFT127;
#[doc = "`read()` method returns [tbft127::R](tbft127::R) reader structure"]
impl crate::Readable for TBFT127 {}
#[doc = "65 to 127 Byte Frames Transmitted Register"]
pub mod tbft127;
#[doc = "128 to 255 Byte Frames Transmitted Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tbft255](tbft255) module"]
pub type TBFT255 = crate::Reg<u32, _TBFT255>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TBFT255;
#[doc = "`read()` method returns [tbft255::R](tbft255::R) reader structure"]
impl crate::Readable for TBFT255 {}
#[doc = "128 to 255 Byte Frames Transmitted Register"]
pub mod tbft255;
#[doc = "256 to 511 Byte Frames Transmitted Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tbft511](tbft511) module"]
pub type TBFT511 = crate::Reg<u32, _TBFT511>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TBFT511;
#[doc = "`read()` method returns [tbft511::R](tbft511::R) reader structure"]
impl crate::Readable for TBFT511 {}
#[doc = "256 to 511 Byte Frames Transmitted Register"]
pub mod tbft511;
#[doc = "512 to 1023 Byte Frames Transmitted Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tbft1023](tbft1023) module"]
pub type TBFT1023 = crate::Reg<u32, _TBFT1023>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TBFT1023;
#[doc = "`read()` method returns [tbft1023::R](tbft1023::R) reader structure"]
impl crate::Readable for TBFT1023 {}
#[doc = "512 to 1023 Byte Frames Transmitted Register"]
pub mod tbft1023;
#[doc = "1024 to 1518 Byte Frames Transmitted Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tbft1518](tbft1518) module"]
pub type TBFT1518 = crate::Reg<u32, _TBFT1518>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TBFT1518;
#[doc = "`read()` method returns [tbft1518::R](tbft1518::R) reader structure"]
impl crate::Readable for TBFT1518 {}
#[doc = "1024 to 1518 Byte Frames Transmitted Register"]
pub mod tbft1518;
#[doc = "Greater Than 1518 Byte Frames Transmitted Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gtbft1518](gtbft1518) module"]
pub type GTBFT1518 = crate::Reg<u32, _GTBFT1518>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GTBFT1518;
#[doc = "`read()` method returns [gtbft1518::R](gtbft1518::R) reader structure"]
impl crate::Readable for GTBFT1518 {}
#[doc = "Greater Than 1518 Byte Frames Transmitted Register"]
pub mod gtbft1518;
#[doc = "Transmit Underruns Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tur](tur) module"]
pub type TUR = crate::Reg<u32, _TUR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TUR;
#[doc = "`read()` method returns [tur::R](tur::R) reader structure"]
impl crate::Readable for TUR {}
#[doc = "Transmit Underruns Register"]
pub mod tur;
#[doc = "Single Collision Frames Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scf](scf) module"]
pub type SCF = crate::Reg<u32, _SCF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCF;
#[doc = "`read()` method returns [scf::R](scf::R) reader structure"]
impl crate::Readable for SCF {}
#[doc = "Single Collision Frames Register"]
pub mod scf;
#[doc = "Multiple Collision Frames Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcf](mcf) module"]
pub type MCF = crate::Reg<u32, _MCF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCF;
#[doc = "`read()` method returns [mcf::R](mcf::R) reader structure"]
impl crate::Readable for MCF {}
#[doc = "Multiple Collision Frames Register"]
pub mod mcf;
#[doc = "Excessive Collisions Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ec](ec) module"]
pub type EC = crate::Reg<u32, _EC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EC;
#[doc = "`read()` method returns [ec::R](ec::R) reader structure"]
impl crate::Readable for EC {}
#[doc = "Excessive Collisions Register"]
pub mod ec;
#[doc = "Late Collisions Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lc](lc) module"]
pub type LC = crate::Reg<u32, _LC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LC;
#[doc = "`read()` method returns [lc::R](lc::R) reader structure"]
impl crate::Readable for LC {}
#[doc = "Late Collisions Register"]
pub mod lc;
#[doc = "Deferred Transmission Frames Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dtf](dtf) module"]
pub type DTF = crate::Reg<u32, _DTF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DTF;
#[doc = "`read()` method returns [dtf::R](dtf::R) reader structure"]
impl crate::Readable for DTF {}
#[doc = "Deferred Transmission Frames Register"]
pub mod dtf;
#[doc = "Carrier Sense Errors Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cse](cse) module"]
pub type CSE = crate::Reg<u32, _CSE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSE;
#[doc = "`read()` method returns [cse::R](cse::R) reader structure"]
impl crate::Readable for CSE {}
#[doc = "Carrier Sense Errors Register"]
pub mod cse;
#[doc = "Octets Received \\[31:0\\]
Received\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [orlo](orlo) module"]
pub type ORLO = crate::Reg<u32, _ORLO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ORLO;
#[doc = "`read()` method returns [orlo::R](orlo::R) reader structure"]
impl crate::Readable for ORLO {}
#[doc = "Octets Received \\[31:0\\]
Received"]
pub mod orlo;
#[doc = "Octets Received \\[47:32\\]
Received\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [orhi](orhi) module"]
pub type ORHI = crate::Reg<u32, _ORHI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ORHI;
#[doc = "`read()` method returns [orhi::R](orhi::R) reader structure"]
impl crate::Readable for ORHI {}
#[doc = "Octets Received \\[47:32\\]
Received"]
pub mod orhi;
#[doc = "Frames Received Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fr](fr) module"]
pub type FR = crate::Reg<u32, _FR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FR;
#[doc = "`read()` method returns [fr::R](fr::R) reader structure"]
impl crate::Readable for FR {}
#[doc = "Frames Received Register"]
pub mod fr;
#[doc = "Broadcast Frames Received Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bcfr](bcfr) module"]
pub type BCFR = crate::Reg<u32, _BCFR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BCFR;
#[doc = "`read()` method returns [bcfr::R](bcfr::R) reader structure"]
impl crate::Readable for BCFR {}
#[doc = "Broadcast Frames Received Register"]
pub mod bcfr;
#[doc = "Multicast Frames Received Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mfr](mfr) module"]
pub type MFR = crate::Reg<u32, _MFR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MFR;
#[doc = "`read()` method returns [mfr::R](mfr::R) reader structure"]
impl crate::Readable for MFR {}
#[doc = "Multicast Frames Received Register"]
pub mod mfr;
#[doc = "Pause Frames Received Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pfr](pfr) module"]
pub type PFR = crate::Reg<u32, _PFR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PFR;
#[doc = "`read()` method returns [pfr::R](pfr::R) reader structure"]
impl crate::Readable for PFR {}
#[doc = "Pause Frames Received Register"]
pub mod pfr;
#[doc = "64 Byte Frames Received Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bfr64](bfr64) module"]
pub type BFR64 = crate::Reg<u32, _BFR64>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BFR64;
#[doc = "`read()` method returns [bfr64::R](bfr64::R) reader structure"]
impl crate::Readable for BFR64 {}
#[doc = "64 Byte Frames Received Register"]
pub mod bfr64;
#[doc = "65 to 127 Byte Frames Received Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tbfr127](tbfr127) module"]
pub type TBFR127 = crate::Reg<u32, _TBFR127>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TBFR127;
#[doc = "`read()` method returns [tbfr127::R](tbfr127::R) reader structure"]
impl crate::Readable for TBFR127 {}
#[doc = "65 to 127 Byte Frames Received Register"]
pub mod tbfr127;
#[doc = "128 to 255 Byte Frames Received Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tbfr255](tbfr255) module"]
pub type TBFR255 = crate::Reg<u32, _TBFR255>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TBFR255;
#[doc = "`read()` method returns [tbfr255::R](tbfr255::R) reader structure"]
impl crate::Readable for TBFR255 {}
#[doc = "128 to 255 Byte Frames Received Register"]
pub mod tbfr255;
#[doc = "256 to 511Byte Frames Received Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tbfr511](tbfr511) module"]
pub type TBFR511 = crate::Reg<u32, _TBFR511>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TBFR511;
#[doc = "`read()` method returns [tbfr511::R](tbfr511::R) reader structure"]
impl crate::Readable for TBFR511 {}
#[doc = "256 to 511Byte Frames Received Register"]
pub mod tbfr511;
#[doc = "512 to 1023 Byte Frames Received Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tbfr1023](tbfr1023) module"]
pub type TBFR1023 = crate::Reg<u32, _TBFR1023>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TBFR1023;
#[doc = "`read()` method returns [tbfr1023::R](tbfr1023::R) reader structure"]
impl crate::Readable for TBFR1023 {}
#[doc = "512 to 1023 Byte Frames Received Register"]
pub mod tbfr1023;
#[doc = "1024 to 1518 Byte Frames Received Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tbfr1518](tbfr1518) module"]
pub type TBFR1518 = crate::Reg<u32, _TBFR1518>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TBFR1518;
#[doc = "`read()` method returns [tbfr1518::R](tbfr1518::R) reader structure"]
impl crate::Readable for TBFR1518 {}
#[doc = "1024 to 1518 Byte Frames Received Register"]
pub mod tbfr1518;
#[doc = "1519 to Maximum Byte Frames Received Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tmxbfr](tmxbfr) module"]
pub type TMXBFR = crate::Reg<u32, _TMXBFR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TMXBFR;
#[doc = "`read()` method returns [tmxbfr::R](tmxbfr::R) reader structure"]
impl crate::Readable for TMXBFR {}
#[doc = "1519 to Maximum Byte Frames Received Register"]
pub mod tmxbfr;
#[doc = "Undersize Frames Received Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ufr](ufr) module"]
pub type UFR = crate::Reg<u32, _UFR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UFR;
#[doc = "`read()` method returns [ufr::R](ufr::R) reader structure"]
impl crate::Readable for UFR {}
#[doc = "Undersize Frames Received Register"]
pub mod ufr;
#[doc = "Oversize Frames Received Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ofr](ofr) module"]
pub type OFR = crate::Reg<u32, _OFR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OFR;
#[doc = "`read()` method returns [ofr::R](ofr::R) reader structure"]
impl crate::Readable for OFR {}
#[doc = "Oversize Frames Received Register"]
pub mod ofr;
#[doc = "Jabbers Received Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [jr](jr) module"]
pub type JR = crate::Reg<u32, _JR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _JR;
#[doc = "`read()` method returns [jr::R](jr::R) reader structure"]
impl crate::Readable for JR {}
#[doc = "Jabbers Received Register"]
pub mod jr;
#[doc = "Frame Check Sequence Errors Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fcse](fcse) module"]
pub type FCSE = crate::Reg<u32, _FCSE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FCSE;
#[doc = "`read()` method returns [fcse::R](fcse::R) reader structure"]
impl crate::Readable for FCSE {}
#[doc = "Frame Check Sequence Errors Register"]
pub mod fcse;
#[doc = "Length Field Frame Errors Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lffe](lffe) module"]
pub type LFFE = crate::Reg<u32, _LFFE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LFFE;
#[doc = "`read()` method returns [lffe::R](lffe::R) reader structure"]
impl crate::Readable for LFFE {}
#[doc = "Length Field Frame Errors Register"]
pub mod lffe;
#[doc = "Receive Symbol Errors Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rse](rse) module"]
pub type RSE = crate::Reg<u32, _RSE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RSE;
#[doc = "`read()` method returns [rse::R](rse::R) reader structure"]
impl crate::Readable for RSE {}
#[doc = "Receive Symbol Errors Register"]
pub mod rse;
#[doc = "Alignment Errors Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ae](ae) module"]
pub type AE = crate::Reg<u32, _AE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AE;
#[doc = "`read()` method returns [ae::R](ae::R) reader structure"]
impl crate::Readable for AE {}
#[doc = "Alignment Errors Register"]
pub mod ae;
#[doc = "Receive Resource Errors Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rre](rre) module"]
pub type RRE = crate::Reg<u32, _RRE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RRE;
#[doc = "`read()` method returns [rre::R](rre::R) reader structure"]
impl crate::Readable for RRE {}
#[doc = "Receive Resource Errors Register"]
pub mod rre;
#[doc = "Receive Overrun Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [roe](roe) module"]
pub type ROE = crate::Reg<u32, _ROE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ROE;
#[doc = "`read()` method returns [roe::R](roe::R) reader structure"]
impl crate::Readable for ROE {}
#[doc = "Receive Overrun Register"]
pub mod roe;
#[doc = "IP Header Checksum Errors Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ihce](ihce) module"]
pub type IHCE = crate::Reg<u32, _IHCE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IHCE;
#[doc = "`read()` method returns [ihce::R](ihce::R) reader structure"]
impl crate::Readable for IHCE {}
#[doc = "IP Header Checksum Errors Register"]
pub mod ihce;
#[doc = "TCP Checksum Errors Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tce](tce) module"]
pub type TCE = crate::Reg<u32, _TCE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCE;
#[doc = "`read()` method returns [tce::R](tce::R) reader structure"]
impl crate::Readable for TCE {}
#[doc = "TCP Checksum Errors Register"]
pub mod tce;
#[doc = "UDP Checksum Errors Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uce](uce) module"]
pub type UCE = crate::Reg<u32, _UCE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCE;
#[doc = "`read()` method returns [uce::R](uce::R) reader structure"]
impl crate::Readable for UCE {}
#[doc = "UDP Checksum Errors Register"]
pub mod uce;
#[doc = "1588 Timer Increment \\[15:0\\]
Sub-Nanoseconds Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tisubn](tisubn) module"]
pub type TISUBN = crate::Reg<u32, _TISUBN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TISUBN;
#[doc = "`read()` method returns [tisubn::R](tisubn::R) reader structure"]
impl crate::Readable for TISUBN {}
#[doc = "`write(|w| ..)` method takes [tisubn::W](tisubn::W) writer structure"]
impl crate::Writable for TISUBN {}
#[doc = "1588 Timer Increment \\[15:0\\]
Sub-Nanoseconds Register"]
pub mod tisubn;
#[doc = "1588 Timer Seconds High \\[15:0\\]
Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tsh](tsh) module"]
pub type TSH = crate::Reg<u32, _TSH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TSH;
#[doc = "`read()` method returns [tsh::R](tsh::R) reader structure"]
impl crate::Readable for TSH {}
#[doc = "`write(|w| ..)` method takes [tsh::W](tsh::W) writer structure"]
impl crate::Writable for TSH {}
#[doc = "1588 Timer Seconds High \\[15:0\\]
Register"]
pub mod tsh;
#[doc = "1588 Timer Sync Strobe Seconds \\[31:0\\]
Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tsssl](tsssl) module"]
pub type TSSSL = crate::Reg<u32, _TSSSL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TSSSL;
#[doc = "`read()` method returns [tsssl::R](tsssl::R) reader structure"]
impl crate::Readable for TSSSL {}
#[doc = "`write(|w| ..)` method takes [tsssl::W](tsssl::W) writer structure"]
impl crate::Writable for TSSSL {}
#[doc = "1588 Timer Sync Strobe Seconds \\[31:0\\]
Register"]
pub mod tsssl;
#[doc = "1588 Timer Sync Strobe Nanoseconds Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tssn](tssn) module"]
pub type TSSN = crate::Reg<u32, _TSSN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TSSN;
#[doc = "`read()` method returns [tssn::R](tssn::R) reader structure"]
impl crate::Readable for TSSN {}
#[doc = "`write(|w| ..)` method takes [tssn::W](tssn::W) writer structure"]
impl crate::Writable for TSSN {}
#[doc = "1588 Timer Sync Strobe Nanoseconds Register"]
pub mod tssn;
#[doc = "1588 Timer Seconds \\[31:0\\]
Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tsl](tsl) module"]
pub type TSL = crate::Reg<u32, _TSL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TSL;
#[doc = "`read()` method returns [tsl::R](tsl::R) reader structure"]
impl crate::Readable for TSL {}
#[doc = "`write(|w| ..)` method takes [tsl::W](tsl::W) writer structure"]
impl crate::Writable for TSL {}
#[doc = "1588 Timer Seconds \\[31:0\\]
Register"]
pub mod tsl;
#[doc = "1588 Timer Nanoseconds Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tn](tn) module"]
pub type TN = crate::Reg<u32, _TN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TN;
#[doc = "`read()` method returns [tn::R](tn::R) reader structure"]
impl crate::Readable for TN {}
#[doc = "`write(|w| ..)` method takes [tn::W](tn::W) writer structure"]
impl crate::Writable for TN {}
#[doc = "1588 Timer Nanoseconds Register"]
pub mod tn;
#[doc = "1588 Timer Adjust Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ta](ta) module"]
pub type TA = crate::Reg<u32, _TA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TA;
#[doc = "`write(|w| ..)` method takes [ta::W](ta::W) writer structure"]
impl crate::Writable for TA {}
#[doc = "1588 Timer Adjust Register"]
pub mod ta;
#[doc = "1588 Timer Increment Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ti](ti) module"]
pub type TI = crate::Reg<u32, _TI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TI;
#[doc = "`read()` method returns [ti::R](ti::R) reader structure"]
impl crate::Readable for TI {}
#[doc = "`write(|w| ..)` method takes [ti::W](ti::W) writer structure"]
impl crate::Writable for TI {}
#[doc = "1588 Timer Increment Register"]
pub mod ti;
#[doc = "PTP Event Frame Transmitted Seconds Low Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eftsl](eftsl) module"]
pub type EFTSL = crate::Reg<u32, _EFTSL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFTSL;
#[doc = "`read()` method returns [eftsl::R](eftsl::R) reader structure"]
impl crate::Readable for EFTSL {}
#[doc = "PTP Event Frame Transmitted Seconds Low Register"]
pub mod eftsl;
#[doc = "PTP Event Frame Transmitted Nanoseconds\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eftn](eftn) module"]
pub type EFTN = crate::Reg<u32, _EFTN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFTN;
#[doc = "`read()` method returns [eftn::R](eftn::R) reader structure"]
impl crate::Readable for EFTN {}
#[doc = "PTP Event Frame Transmitted Nanoseconds"]
pub mod eftn;
#[doc = "PTP Event Frame Received Seconds Low Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [efrsl](efrsl) module"]
pub type EFRSL = crate::Reg<u32, _EFRSL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFRSL;
#[doc = "`read()` method returns [efrsl::R](efrsl::R) reader structure"]
impl crate::Readable for EFRSL {}
#[doc = "PTP Event Frame Received Seconds Low Register"]
pub mod efrsl;
#[doc = "PTP Event Frame Received Nanoseconds\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [efrn](efrn) module"]
pub type EFRN = crate::Reg<u32, _EFRN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFRN;
#[doc = "`read()` method returns [efrn::R](efrn::R) reader structure"]
impl crate::Readable for EFRN {}
#[doc = "PTP Event Frame Received Nanoseconds"]
pub mod efrn;
#[doc = "PTP Peer Event Frame Transmitted Seconds Low Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [peftsl](peftsl) module"]
pub type PEFTSL = crate::Reg<u32, _PEFTSL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PEFTSL;
#[doc = "`read()` method returns [peftsl::R](peftsl::R) reader structure"]
impl crate::Readable for PEFTSL {}
#[doc = "PTP Peer Event Frame Transmitted Seconds Low Register"]
pub mod peftsl;
#[doc = "PTP Peer Event Frame Transmitted Nanoseconds\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [peftn](peftn) module"]
pub type PEFTN = crate::Reg<u32, _PEFTN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PEFTN;
#[doc = "`read()` method returns [peftn::R](peftn::R) reader structure"]
impl crate::Readable for PEFTN {}
#[doc = "PTP Peer Event Frame Transmitted Nanoseconds"]
pub mod peftn;
#[doc = "PTP Peer Event Frame Received Seconds Low Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pefrsl](pefrsl) module"]
pub type PEFRSL = crate::Reg<u32, _PEFRSL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PEFRSL;
#[doc = "`read()` method returns [pefrsl::R](pefrsl::R) reader structure"]
impl crate::Readable for PEFRSL {}
#[doc = "PTP Peer Event Frame Received Seconds Low Register"]
pub mod pefrsl;
#[doc = "PTP Peer Event Frame Received Nanoseconds\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pefrn](pefrn) module"]
pub type PEFRN = crate::Reg<u32, _PEFRN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PEFRN;
#[doc = "`read()` method returns [pefrn::R](pefrn::R) reader structure"]
impl crate::Readable for PEFRN {}
#[doc = "PTP Peer Event Frame Received Nanoseconds"]
pub mod pefrn;
#[doc = "Receive LPI transition Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rlpitr](rlpitr) module"]
pub type RLPITR = crate::Reg<u32, _RLPITR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RLPITR;
#[doc = "`read()` method returns [rlpitr::R](rlpitr::R) reader structure"]
impl crate::Readable for RLPITR {}
#[doc = "Receive LPI transition Register"]
pub mod rlpitr;
#[doc = "Receive LPI Time Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rlpiti](rlpiti) module"]
pub type RLPITI = crate::Reg<u32, _RLPITI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RLPITI;
#[doc = "`read()` method returns [rlpiti::R](rlpiti::R) reader structure"]
impl crate::Readable for RLPITI {}
#[doc = "Receive LPI Time Register"]
pub mod rlpiti;
#[doc = "Receive LPI transition Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tlpitr](tlpitr) module"]
pub type TLPITR = crate::Reg<u32, _TLPITR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TLPITR;
#[doc = "`read()` method returns [tlpitr::R](tlpitr::R) reader structure"]
impl crate::Readable for TLPITR {}
#[doc = "Receive LPI transition Register"]
pub mod tlpitr;
#[doc = "Receive LPI Time Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tlpiti](tlpiti) module"]
pub type TLPITI = crate::Reg<u32, _TLPITI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TLPITI;
#[doc = "`read()` method returns [tlpiti::R](tlpiti::R) reader structure"]
impl crate::Readable for TLPITI {}
#[doc = "Receive LPI Time Register"]
pub mod tlpiti;
