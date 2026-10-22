#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control A"]
    pub ctrla: CTRLA,
    #[doc = "0x04 - Control B"]
    pub ctrlb: CTRLB,
    #[doc = "0x06 - Control C"]
    pub ctrlc: CTRLC,
    #[doc = "0x08 - Control D"]
    pub ctrld: CTRLD,
    _reserved4: [u8; 0x03],
    #[doc = "0x0c - Event Control"]
    pub evctrl: EVCTRL,
    #[doc = "0x0d - Interrupt Enable Clear"]
    pub intenclr: INTENCLR,
    #[doc = "0x0e - Interrupt Enable Set"]
    pub intenset: INTENSET,
    #[doc = "0x0f - Interrupt Flag Status and Clear"]
    pub intflag: INTFLAG,
    #[doc = "0x10 - Status"]
    pub status: STATUS,
    _reserved9: [u8; 0x03],
    #[doc = "0x14 - Synchronization Busy"]
    pub syncbusy: SYNCBUSY,
    #[doc = "0x18 - Frame Counter 0 Configuration"]
    pub fc0: FC0,
    #[doc = "0x19 - Frame Counter 1 Configuration"]
    pub fc1: FC1,
    #[doc = "0x1a - Frame Counter 2 Configuration"]
    pub fc2: FC2,
    _reserved13: [u8; 0x01],
    #[doc = "0x1c - LCD Pin Enable Low"]
    pub lpenl: LPENL,
    #[doc = "0x20 - LCD Pin Enable High"]
    pub lpenh: LPENH,
    #[doc = "0x24 - Segments Data Low for COM0 Line"]
    pub sdatal0: SDATAL0,
    #[doc = "0x28 - Segments Data High for COM0 Line"]
    pub sdatah0: SDATAH0,
    #[doc = "0x2c - Segments Data Low for COM1 Line"]
    pub sdatal1: SDATAL1,
    #[doc = "0x30 - Segments Data High for COM1 Line"]
    pub sdatah1: SDATAH1,
    #[doc = "0x34 - Segments Data Low for COM2 Line"]
    pub sdatal2: SDATAL2,
    #[doc = "0x38 - Segments Data High for COM2 Line"]
    pub sdatah2: SDATAH2,
    #[doc = "0x3c - Segments Data Low for COM3 Line"]
    pub sdatal3: SDATAL3,
    #[doc = "0x40 - Segments Data High for COM3 Line"]
    pub sdatah3: SDATAH3,
    #[doc = "0x44 - Segments Data Low for COM4 Line"]
    pub sdatal4: SDATAL4,
    #[doc = "0x48 - Segments Data High for COM4 Line"]
    pub sdatah4: SDATAH4,
    #[doc = "0x4c - Segments Data Low for COM5 Line"]
    pub sdatal5: SDATAL5,
    #[doc = "0x50 - Segments Data High for COM5 Line"]
    pub sdatah5: SDATAH5,
    #[doc = "0x54 - Segments Data Low for COM6 Line"]
    pub sdatal6: SDATAL6,
    #[doc = "0x58 - Segments Data High for COM6 Line"]
    pub sdatah6: SDATAH6,
    #[doc = "0x5c - Segments Data Low for COM7 Line"]
    pub sdatal7: SDATAL7,
    #[doc = "0x60 - Segments Data High for COM7 Line"]
    pub sdatah7: SDATAH7,
    #[doc = "0x64 - Indirect Segments Data Access"]
    pub isdata: ISDATA,
    #[doc = "0x68 - Blink Configuration"]
    pub bcfg: BCFG,
    #[doc = "0x6c - Circular Shift Register Configuration"]
    pub csrcfg: CSRCFG,
    #[doc = "0x70 - Character Mapping Configuration"]
    pub cmcfg: CMCFG,
    _reserved35: [u8; 0x03],
    #[doc = "0x74 - Automated Character Mapping Configuration"]
    pub acmcfg: ACMCFG,
    #[doc = "0x78 - Automated Bit Mapping Configuration"]
    pub abmcfg: ABMCFG,
    _reserved37: [u8; 0x03],
    #[doc = "0x7c - Character Mapping Segments Data"]
    pub cmdata: CMDATA,
    #[doc = "0x80 - Character Mapping Segments Data Mask"]
    pub cmdmask: CMDMASK,
    #[doc = "0x84 - Character Mapping SEG/COM Index"]
    pub cmindex: CMINDEX,
}
#[doc = "CTRLA (rw) register accessor: an alias for `Reg<CTRLA_SPEC>`"]
pub type CTRLA = crate::Reg<ctrla::CTRLA_SPEC>;
#[doc = "Control A"]
pub mod ctrla;
#[doc = "CTRLB (rw) register accessor: an alias for `Reg<CTRLB_SPEC>`"]
pub type CTRLB = crate::Reg<ctrlb::CTRLB_SPEC>;
#[doc = "Control B"]
pub mod ctrlb;
#[doc = "CTRLC (rw) register accessor: an alias for `Reg<CTRLC_SPEC>`"]
pub type CTRLC = crate::Reg<ctrlc::CTRLC_SPEC>;
#[doc = "Control C"]
pub mod ctrlc;
#[doc = "CTRLD (rw) register accessor: an alias for `Reg<CTRLD_SPEC>`"]
pub type CTRLD = crate::Reg<ctrld::CTRLD_SPEC>;
#[doc = "Control D"]
pub mod ctrld;
#[doc = "EVCTRL (rw) register accessor: an alias for `Reg<EVCTRL_SPEC>`"]
pub type EVCTRL = crate::Reg<evctrl::EVCTRL_SPEC>;
#[doc = "Event Control"]
pub mod evctrl;
#[doc = "INTENCLR (rw) register accessor: an alias for `Reg<INTENCLR_SPEC>`"]
pub type INTENCLR = crate::Reg<intenclr::INTENCLR_SPEC>;
#[doc = "Interrupt Enable Clear"]
pub mod intenclr;
#[doc = "INTENSET (rw) register accessor: an alias for `Reg<INTENSET_SPEC>`"]
pub type INTENSET = crate::Reg<intenset::INTENSET_SPEC>;
#[doc = "Interrupt Enable Set"]
pub mod intenset;
#[doc = "INTFLAG (rw) register accessor: an alias for `Reg<INTFLAG_SPEC>`"]
pub type INTFLAG = crate::Reg<intflag::INTFLAG_SPEC>;
#[doc = "Interrupt Flag Status and Clear"]
pub mod intflag;
#[doc = "STATUS (r) register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "Status"]
pub mod status;
#[doc = "SYNCBUSY (r) register accessor: an alias for `Reg<SYNCBUSY_SPEC>`"]
pub type SYNCBUSY = crate::Reg<syncbusy::SYNCBUSY_SPEC>;
#[doc = "Synchronization Busy"]
pub mod syncbusy;
#[doc = "FC0 (rw) register accessor: an alias for `Reg<FC0_SPEC>`"]
pub type FC0 = crate::Reg<fc0::FC0_SPEC>;
#[doc = "Frame Counter 0 Configuration"]
pub mod fc0;
#[doc = "FC1 (rw) register accessor: an alias for `Reg<FC1_SPEC>`"]
pub type FC1 = crate::Reg<fc1::FC1_SPEC>;
#[doc = "Frame Counter 1 Configuration"]
pub mod fc1;
#[doc = "FC2 (rw) register accessor: an alias for `Reg<FC2_SPEC>`"]
pub type FC2 = crate::Reg<fc2::FC2_SPEC>;
#[doc = "Frame Counter 2 Configuration"]
pub mod fc2;
#[doc = "LPENL (rw) register accessor: an alias for `Reg<LPENL_SPEC>`"]
pub type LPENL = crate::Reg<lpenl::LPENL_SPEC>;
#[doc = "LCD Pin Enable Low"]
pub mod lpenl;
#[doc = "LPENH (rw) register accessor: an alias for `Reg<LPENH_SPEC>`"]
pub type LPENH = crate::Reg<lpenh::LPENH_SPEC>;
#[doc = "LCD Pin Enable High"]
pub mod lpenh;
#[doc = "SDATAL0 (rw) register accessor: an alias for `Reg<SDATAL0_SPEC>`"]
pub type SDATAL0 = crate::Reg<sdatal0::SDATAL0_SPEC>;
#[doc = "Segments Data Low for COM0 Line"]
pub mod sdatal0;
#[doc = "SDATAH0 (rw) register accessor: an alias for `Reg<SDATAH0_SPEC>`"]
pub type SDATAH0 = crate::Reg<sdatah0::SDATAH0_SPEC>;
#[doc = "Segments Data High for COM0 Line"]
pub mod sdatah0;
#[doc = "SDATAL1 (rw) register accessor: an alias for `Reg<SDATAL1_SPEC>`"]
pub type SDATAL1 = crate::Reg<sdatal1::SDATAL1_SPEC>;
#[doc = "Segments Data Low for COM1 Line"]
pub mod sdatal1;
#[doc = "SDATAH1 (rw) register accessor: an alias for `Reg<SDATAH1_SPEC>`"]
pub type SDATAH1 = crate::Reg<sdatah1::SDATAH1_SPEC>;
#[doc = "Segments Data High for COM1 Line"]
pub mod sdatah1;
#[doc = "SDATAL2 (rw) register accessor: an alias for `Reg<SDATAL2_SPEC>`"]
pub type SDATAL2 = crate::Reg<sdatal2::SDATAL2_SPEC>;
#[doc = "Segments Data Low for COM2 Line"]
pub mod sdatal2;
#[doc = "SDATAH2 (rw) register accessor: an alias for `Reg<SDATAH2_SPEC>`"]
pub type SDATAH2 = crate::Reg<sdatah2::SDATAH2_SPEC>;
#[doc = "Segments Data High for COM2 Line"]
pub mod sdatah2;
#[doc = "SDATAL3 (rw) register accessor: an alias for `Reg<SDATAL3_SPEC>`"]
pub type SDATAL3 = crate::Reg<sdatal3::SDATAL3_SPEC>;
#[doc = "Segments Data Low for COM3 Line"]
pub mod sdatal3;
#[doc = "SDATAH3 (rw) register accessor: an alias for `Reg<SDATAH3_SPEC>`"]
pub type SDATAH3 = crate::Reg<sdatah3::SDATAH3_SPEC>;
#[doc = "Segments Data High for COM3 Line"]
pub mod sdatah3;
#[doc = "SDATAL4 (rw) register accessor: an alias for `Reg<SDATAL4_SPEC>`"]
pub type SDATAL4 = crate::Reg<sdatal4::SDATAL4_SPEC>;
#[doc = "Segments Data Low for COM4 Line"]
pub mod sdatal4;
#[doc = "SDATAH4 (rw) register accessor: an alias for `Reg<SDATAH4_SPEC>`"]
pub type SDATAH4 = crate::Reg<sdatah4::SDATAH4_SPEC>;
#[doc = "Segments Data High for COM4 Line"]
pub mod sdatah4;
#[doc = "SDATAL5 (rw) register accessor: an alias for `Reg<SDATAL5_SPEC>`"]
pub type SDATAL5 = crate::Reg<sdatal5::SDATAL5_SPEC>;
#[doc = "Segments Data Low for COM5 Line"]
pub mod sdatal5;
#[doc = "SDATAH5 (rw) register accessor: an alias for `Reg<SDATAH5_SPEC>`"]
pub type SDATAH5 = crate::Reg<sdatah5::SDATAH5_SPEC>;
#[doc = "Segments Data High for COM5 Line"]
pub mod sdatah5;
#[doc = "SDATAL6 (rw) register accessor: an alias for `Reg<SDATAL6_SPEC>`"]
pub type SDATAL6 = crate::Reg<sdatal6::SDATAL6_SPEC>;
#[doc = "Segments Data Low for COM6 Line"]
pub mod sdatal6;
#[doc = "SDATAH6 (rw) register accessor: an alias for `Reg<SDATAH6_SPEC>`"]
pub type SDATAH6 = crate::Reg<sdatah6::SDATAH6_SPEC>;
#[doc = "Segments Data High for COM6 Line"]
pub mod sdatah6;
#[doc = "SDATAL7 (rw) register accessor: an alias for `Reg<SDATAL7_SPEC>`"]
pub type SDATAL7 = crate::Reg<sdatal7::SDATAL7_SPEC>;
#[doc = "Segments Data Low for COM7 Line"]
pub mod sdatal7;
#[doc = "SDATAH7 (rw) register accessor: an alias for `Reg<SDATAH7_SPEC>`"]
pub type SDATAH7 = crate::Reg<sdatah7::SDATAH7_SPEC>;
#[doc = "Segments Data High for COM7 Line"]
pub mod sdatah7;
#[doc = "ISDATA (w) register accessor: an alias for `Reg<ISDATA_SPEC>`"]
pub type ISDATA = crate::Reg<isdata::ISDATA_SPEC>;
#[doc = "Indirect Segments Data Access"]
pub mod isdata;
#[doc = "BCFG (rw) register accessor: an alias for `Reg<BCFG_SPEC>`"]
pub type BCFG = crate::Reg<bcfg::BCFG_SPEC>;
#[doc = "Blink Configuration"]
pub mod bcfg;
#[doc = "CSRCFG (rw) register accessor: an alias for `Reg<CSRCFG_SPEC>`"]
pub type CSRCFG = crate::Reg<csrcfg::CSRCFG_SPEC>;
#[doc = "Circular Shift Register Configuration"]
pub mod csrcfg;
#[doc = "CMCFG (rw) register accessor: an alias for `Reg<CMCFG_SPEC>`"]
pub type CMCFG = crate::Reg<cmcfg::CMCFG_SPEC>;
#[doc = "Character Mapping Configuration"]
pub mod cmcfg;
#[doc = "ACMCFG (rw) register accessor: an alias for `Reg<ACMCFG_SPEC>`"]
pub type ACMCFG = crate::Reg<acmcfg::ACMCFG_SPEC>;
#[doc = "Automated Character Mapping Configuration"]
pub mod acmcfg;
#[doc = "ABMCFG (rw) register accessor: an alias for `Reg<ABMCFG_SPEC>`"]
pub type ABMCFG = crate::Reg<abmcfg::ABMCFG_SPEC>;
#[doc = "Automated Bit Mapping Configuration"]
pub mod abmcfg;
#[doc = "CMDATA (w) register accessor: an alias for `Reg<CMDATA_SPEC>`"]
pub type CMDATA = crate::Reg<cmdata::CMDATA_SPEC>;
#[doc = "Character Mapping Segments Data"]
pub mod cmdata;
#[doc = "CMDMASK (rw) register accessor: an alias for `Reg<CMDMASK_SPEC>`"]
pub type CMDMASK = crate::Reg<cmdmask::CMDMASK_SPEC>;
#[doc = "Character Mapping Segments Data Mask"]
pub mod cmdmask;
#[doc = "CMINDEX (rw) register accessor: an alias for `Reg<CMINDEX_SPEC>`"]
pub type CMINDEX = crate::Reg<cmindex::CMINDEX_SPEC>;
#[doc = "Character Mapping SEG/COM Index"]
pub mod cmindex;
