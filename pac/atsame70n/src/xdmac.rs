#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Global Type Register"]
    pub xdmac_gtype: crate::Reg<xdmac_gtype::XDMAC_GTYPE_SPEC>,
    #[doc = "0x04 - Global Configuration Register"]
    pub xdmac_gcfg: crate::Reg<xdmac_gcfg::XDMAC_GCFG_SPEC>,
    #[doc = "0x08 - Global Weighted Arbiter Configuration Register"]
    pub xdmac_gwac: crate::Reg<xdmac_gwac::XDMAC_GWAC_SPEC>,
    #[doc = "0x0c - Global Interrupt Enable Register"]
    pub xdmac_gie: crate::Reg<xdmac_gie::XDMAC_GIE_SPEC>,
    #[doc = "0x10 - Global Interrupt Disable Register"]
    pub xdmac_gid: crate::Reg<xdmac_gid::XDMAC_GID_SPEC>,
    #[doc = "0x14 - Global Interrupt Mask Register"]
    pub xdmac_gim: crate::Reg<xdmac_gim::XDMAC_GIM_SPEC>,
    #[doc = "0x18 - Global Interrupt Status Register"]
    pub xdmac_gis: crate::Reg<xdmac_gis::XDMAC_GIS_SPEC>,
    #[doc = "0x1c - Global Channel Enable Register"]
    pub xdmac_ge: crate::Reg<xdmac_ge::XDMAC_GE_SPEC>,
    #[doc = "0x20 - Global Channel Disable Register"]
    pub xdmac_gd: crate::Reg<xdmac_gd::XDMAC_GD_SPEC>,
    #[doc = "0x24 - Global Channel Status Register"]
    pub xdmac_gs: crate::Reg<xdmac_gs::XDMAC_GS_SPEC>,
    #[doc = "0x28 - Global Channel Read Suspend Register"]
    pub xdmac_grs: crate::Reg<xdmac_grs::XDMAC_GRS_SPEC>,
    #[doc = "0x2c - Global Channel Write Suspend Register"]
    pub xdmac_gws: crate::Reg<xdmac_gws::XDMAC_GWS_SPEC>,
    #[doc = "0x30 - Global Channel Read Write Suspend Register"]
    pub xdmac_grws: crate::Reg<xdmac_grws::XDMAC_GRWS_SPEC>,
    #[doc = "0x34 - Global Channel Read Write Resume Register"]
    pub xdmac_grwr: crate::Reg<xdmac_grwr::XDMAC_GRWR_SPEC>,
    #[doc = "0x38 - Global Channel Software Request Register"]
    pub xdmac_gswr: crate::Reg<xdmac_gswr::XDMAC_GSWR_SPEC>,
    #[doc = "0x3c - Global Channel Software Request Status Register"]
    pub xdmac_gsws: crate::Reg<xdmac_gsws::XDMAC_GSWS_SPEC>,
    #[doc = "0x40 - Global Channel Software Flush Request Register"]
    pub xdmac_gswf: crate::Reg<xdmac_gswf::XDMAC_GSWF_SPEC>,
    _reserved17: [u8; 0x0c],
    #[doc = "0x50..0x88 - Channel Interrupt Enable Register"]
    pub xdmac_chid0: XDMAC_CHID,
    _reserved18: [u8; 0x08],
    #[doc = "0x90..0xc8 - Channel Interrupt Enable Register"]
    pub xdmac_chid1: XDMAC_CHID,
    _reserved19: [u8; 0x08],
    #[doc = "0xd0..0x108 - Channel Interrupt Enable Register"]
    pub xdmac_chid2: XDMAC_CHID,
    _reserved20: [u8; 0x08],
    #[doc = "0x110..0x148 - Channel Interrupt Enable Register"]
    pub xdmac_chid3: XDMAC_CHID,
    _reserved21: [u8; 0x08],
    #[doc = "0x150..0x188 - Channel Interrupt Enable Register"]
    pub xdmac_chid4: XDMAC_CHID,
    _reserved22: [u8; 0x08],
    #[doc = "0x190..0x1c8 - Channel Interrupt Enable Register"]
    pub xdmac_chid5: XDMAC_CHID,
    _reserved23: [u8; 0x08],
    #[doc = "0x1d0..0x208 - Channel Interrupt Enable Register"]
    pub xdmac_chid6: XDMAC_CHID,
    _reserved24: [u8; 0x08],
    #[doc = "0x210..0x248 - Channel Interrupt Enable Register"]
    pub xdmac_chid7: XDMAC_CHID,
    _reserved25: [u8; 0x08],
    #[doc = "0x250..0x288 - Channel Interrupt Enable Register"]
    pub xdmac_chid8: XDMAC_CHID,
    _reserved26: [u8; 0x08],
    #[doc = "0x290..0x2c8 - Channel Interrupt Enable Register"]
    pub xdmac_chid9: XDMAC_CHID,
    _reserved27: [u8; 0x08],
    #[doc = "0x2d0..0x308 - Channel Interrupt Enable Register"]
    pub xdmac_chid10: XDMAC_CHID,
    _reserved28: [u8; 0x08],
    #[doc = "0x310..0x348 - Channel Interrupt Enable Register"]
    pub xdmac_chid11: XDMAC_CHID,
    _reserved29: [u8; 0x08],
    #[doc = "0x350..0x388 - Channel Interrupt Enable Register"]
    pub xdmac_chid12: XDMAC_CHID,
    _reserved30: [u8; 0x08],
    #[doc = "0x390..0x3c8 - Channel Interrupt Enable Register"]
    pub xdmac_chid13: XDMAC_CHID,
    _reserved31: [u8; 0x08],
    #[doc = "0x3d0..0x408 - Channel Interrupt Enable Register"]
    pub xdmac_chid14: XDMAC_CHID,
    _reserved32: [u8; 0x08],
    #[doc = "0x410..0x448 - Channel Interrupt Enable Register"]
    pub xdmac_chid15: XDMAC_CHID,
    _reserved33: [u8; 0x08],
    #[doc = "0x450..0x488 - Channel Interrupt Enable Register"]
    pub xdmac_chid16: XDMAC_CHID,
    _reserved34: [u8; 0x08],
    #[doc = "0x490..0x4c8 - Channel Interrupt Enable Register"]
    pub xdmac_chid17: XDMAC_CHID,
    _reserved35: [u8; 0x08],
    #[doc = "0x4d0..0x508 - Channel Interrupt Enable Register"]
    pub xdmac_chid18: XDMAC_CHID,
    _reserved36: [u8; 0x08],
    #[doc = "0x510..0x548 - Channel Interrupt Enable Register"]
    pub xdmac_chid19: XDMAC_CHID,
    _reserved37: [u8; 0x08],
    #[doc = "0x550..0x588 - Channel Interrupt Enable Register"]
    pub xdmac_chid20: XDMAC_CHID,
    _reserved38: [u8; 0x08],
    #[doc = "0x590..0x5c8 - Channel Interrupt Enable Register"]
    pub xdmac_chid21: XDMAC_CHID,
    _reserved39: [u8; 0x08],
    #[doc = "0x5d0..0x608 - Channel Interrupt Enable Register"]
    pub xdmac_chid22: XDMAC_CHID,
    _reserved40: [u8; 0x08],
    #[doc = "0x610..0x648 - Channel Interrupt Enable Register"]
    pub xdmac_chid23: XDMAC_CHID,
}
#[doc = r"Register block"]
#[repr(C)]
pub struct XDMAC_CHID {
    #[doc = "0x00 - Channel Interrupt Enable Register"]
    pub xdmac_cie: crate::Reg<self::xdmac_chid::xdmac_cie::XDMAC_CIE_SPEC>,
    #[doc = "0x04 - Channel Interrupt Disable Register"]
    pub xdmac_cid: crate::Reg<self::xdmac_chid::xdmac_cid::XDMAC_CID_SPEC>,
    #[doc = "0x08 - Channel Interrupt Mask Register"]
    pub xdmac_cim: crate::Reg<self::xdmac_chid::xdmac_cim::XDMAC_CIM_SPEC>,
    #[doc = "0x0c - Channel Interrupt Status Register"]
    pub xdmac_cis: crate::Reg<self::xdmac_chid::xdmac_cis::XDMAC_CIS_SPEC>,
    #[doc = "0x10 - Channel Source Address Register"]
    pub xdmac_csa: crate::Reg<self::xdmac_chid::xdmac_csa::XDMAC_CSA_SPEC>,
    #[doc = "0x14 - Channel Destination Address Register"]
    pub xdmac_cda: crate::Reg<self::xdmac_chid::xdmac_cda::XDMAC_CDA_SPEC>,
    #[doc = "0x18 - Channel Next Descriptor Address Register"]
    pub xdmac_cnda: crate::Reg<self::xdmac_chid::xdmac_cnda::XDMAC_CNDA_SPEC>,
    #[doc = "0x1c - Channel Next Descriptor Control Register"]
    pub xdmac_cndc: crate::Reg<self::xdmac_chid::xdmac_cndc::XDMAC_CNDC_SPEC>,
    #[doc = "0x20 - Channel Microblock Control Register"]
    pub xdmac_cubc: crate::Reg<self::xdmac_chid::xdmac_cubc::XDMAC_CUBC_SPEC>,
    #[doc = "0x24 - Channel Block Control Register"]
    pub xdmac_cbc: crate::Reg<self::xdmac_chid::xdmac_cbc::XDMAC_CBC_SPEC>,
    #[doc = "0x28 - Channel Configuration Register"]
    pub xdmac_cc: crate::Reg<self::xdmac_chid::xdmac_cc::XDMAC_CC_SPEC>,
    #[doc = "0x2c - Channel Data Stride Memory Set Pattern"]
    pub xdmac_cds_msp: crate::Reg<self::xdmac_chid::xdmac_cds_msp::XDMAC_CDS_MSP_SPEC>,
    #[doc = "0x30 - Channel Source Microblock Stride"]
    pub xdmac_csus: crate::Reg<self::xdmac_chid::xdmac_csus::XDMAC_CSUS_SPEC>,
    #[doc = "0x34 - Channel Destination Microblock Stride"]
    pub xdmac_cdus: crate::Reg<self::xdmac_chid::xdmac_cdus::XDMAC_CDUS_SPEC>,
}
#[doc = r"Register block"]
#[doc = "Channel Interrupt Enable Register"]
pub mod xdmac_chid;
#[doc = "XDMAC_GTYPE register accessor: an alias for `Reg<XDMAC_GTYPE_SPEC>`"]
pub type XDMAC_GTYPE = crate::Reg<xdmac_gtype::XDMAC_GTYPE_SPEC>;
#[doc = "Global Type Register"]
pub mod xdmac_gtype;
#[doc = "XDMAC_GCFG register accessor: an alias for `Reg<XDMAC_GCFG_SPEC>`"]
pub type XDMAC_GCFG = crate::Reg<xdmac_gcfg::XDMAC_GCFG_SPEC>;
#[doc = "Global Configuration Register"]
pub mod xdmac_gcfg;
#[doc = "XDMAC_GWAC register accessor: an alias for `Reg<XDMAC_GWAC_SPEC>`"]
pub type XDMAC_GWAC = crate::Reg<xdmac_gwac::XDMAC_GWAC_SPEC>;
#[doc = "Global Weighted Arbiter Configuration Register"]
pub mod xdmac_gwac;
#[doc = "XDMAC_GIE register accessor: an alias for `Reg<XDMAC_GIE_SPEC>`"]
pub type XDMAC_GIE = crate::Reg<xdmac_gie::XDMAC_GIE_SPEC>;
#[doc = "Global Interrupt Enable Register"]
pub mod xdmac_gie;
#[doc = "XDMAC_GID register accessor: an alias for `Reg<XDMAC_GID_SPEC>`"]
pub type XDMAC_GID = crate::Reg<xdmac_gid::XDMAC_GID_SPEC>;
#[doc = "Global Interrupt Disable Register"]
pub mod xdmac_gid;
#[doc = "XDMAC_GIM register accessor: an alias for `Reg<XDMAC_GIM_SPEC>`"]
pub type XDMAC_GIM = crate::Reg<xdmac_gim::XDMAC_GIM_SPEC>;
#[doc = "Global Interrupt Mask Register"]
pub mod xdmac_gim;
#[doc = "XDMAC_GIS register accessor: an alias for `Reg<XDMAC_GIS_SPEC>`"]
pub type XDMAC_GIS = crate::Reg<xdmac_gis::XDMAC_GIS_SPEC>;
#[doc = "Global Interrupt Status Register"]
pub mod xdmac_gis;
#[doc = "XDMAC_GE register accessor: an alias for `Reg<XDMAC_GE_SPEC>`"]
pub type XDMAC_GE = crate::Reg<xdmac_ge::XDMAC_GE_SPEC>;
#[doc = "Global Channel Enable Register"]
pub mod xdmac_ge;
#[doc = "XDMAC_GD register accessor: an alias for `Reg<XDMAC_GD_SPEC>`"]
pub type XDMAC_GD = crate::Reg<xdmac_gd::XDMAC_GD_SPEC>;
#[doc = "Global Channel Disable Register"]
pub mod xdmac_gd;
#[doc = "XDMAC_GS register accessor: an alias for `Reg<XDMAC_GS_SPEC>`"]
pub type XDMAC_GS = crate::Reg<xdmac_gs::XDMAC_GS_SPEC>;
#[doc = "Global Channel Status Register"]
pub mod xdmac_gs;
#[doc = "XDMAC_GRS register accessor: an alias for `Reg<XDMAC_GRS_SPEC>`"]
pub type XDMAC_GRS = crate::Reg<xdmac_grs::XDMAC_GRS_SPEC>;
#[doc = "Global Channel Read Suspend Register"]
pub mod xdmac_grs;
#[doc = "XDMAC_GWS register accessor: an alias for `Reg<XDMAC_GWS_SPEC>`"]
pub type XDMAC_GWS = crate::Reg<xdmac_gws::XDMAC_GWS_SPEC>;
#[doc = "Global Channel Write Suspend Register"]
pub mod xdmac_gws;
#[doc = "XDMAC_GRWS register accessor: an alias for `Reg<XDMAC_GRWS_SPEC>`"]
pub type XDMAC_GRWS = crate::Reg<xdmac_grws::XDMAC_GRWS_SPEC>;
#[doc = "Global Channel Read Write Suspend Register"]
pub mod xdmac_grws;
#[doc = "XDMAC_GRWR register accessor: an alias for `Reg<XDMAC_GRWR_SPEC>`"]
pub type XDMAC_GRWR = crate::Reg<xdmac_grwr::XDMAC_GRWR_SPEC>;
#[doc = "Global Channel Read Write Resume Register"]
pub mod xdmac_grwr;
#[doc = "XDMAC_GSWR register accessor: an alias for `Reg<XDMAC_GSWR_SPEC>`"]
pub type XDMAC_GSWR = crate::Reg<xdmac_gswr::XDMAC_GSWR_SPEC>;
#[doc = "Global Channel Software Request Register"]
pub mod xdmac_gswr;
#[doc = "XDMAC_GSWS register accessor: an alias for `Reg<XDMAC_GSWS_SPEC>`"]
pub type XDMAC_GSWS = crate::Reg<xdmac_gsws::XDMAC_GSWS_SPEC>;
#[doc = "Global Channel Software Request Status Register"]
pub mod xdmac_gsws;
#[doc = "XDMAC_GSWF register accessor: an alias for `Reg<XDMAC_GSWF_SPEC>`"]
pub type XDMAC_GSWF = crate::Reg<xdmac_gswf::XDMAC_GSWF_SPEC>;
#[doc = "Global Channel Software Flush Request Register"]
pub mod xdmac_gswf;
