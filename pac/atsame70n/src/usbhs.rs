#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Device General Control Register"]
    pub usbhs_devctrl: crate::Reg<usbhs_devctrl::USBHS_DEVCTRL_SPEC>,
    #[doc = "0x04 - Device Global Interrupt Status Register"]
    pub usbhs_devisr: crate::Reg<usbhs_devisr::USBHS_DEVISR_SPEC>,
    #[doc = "0x08 - Device Global Interrupt Clear Register"]
    pub usbhs_devicr: crate::Reg<usbhs_devicr::USBHS_DEVICR_SPEC>,
    #[doc = "0x0c - Device Global Interrupt Set Register"]
    pub usbhs_devifr: crate::Reg<usbhs_devifr::USBHS_DEVIFR_SPEC>,
    #[doc = "0x10 - Device Global Interrupt Mask Register"]
    pub usbhs_devimr: crate::Reg<usbhs_devimr::USBHS_DEVIMR_SPEC>,
    #[doc = "0x14 - Device Global Interrupt Disable Register"]
    pub usbhs_devidr: crate::Reg<usbhs_devidr::USBHS_DEVIDR_SPEC>,
    #[doc = "0x18 - Device Global Interrupt Enable Register"]
    pub usbhs_devier: crate::Reg<usbhs_devier::USBHS_DEVIER_SPEC>,
    #[doc = "0x1c - Device Endpoint Register"]
    pub usbhs_devept: crate::Reg<usbhs_devept::USBHS_DEVEPT_SPEC>,
    #[doc = "0x20 - Device Frame Number Register"]
    pub usbhs_devfnum: crate::Reg<usbhs_devfnum::USBHS_DEVFNUM_SPEC>,
    _reserved9: [u8; 0xdc],
    #[doc = "0x100..0x128 - Device Endpoint Configuration Register"]
    pub usbhs_deveptcfg: [crate::Reg<usbhs_deveptcfg::USBHS_DEVEPTCFG_SPEC>; 10],
    _reserved10: [u8; 0x08],
    _reserved_10_usbhs_deveptisr: [u8; 0x28],
    _reserved11: [u8; 0x08],
    _reserved_11_usbhs_devepticr: [u8; 0x28],
    _reserved12: [u8; 0x08],
    _reserved_12_usbhs_deveptifr: [u8; 0x28],
    _reserved13: [u8; 0x08],
    _reserved_13_usbhs_deveptimr: [u8; 0x28],
    _reserved14: [u8; 0x08],
    _reserved_14_usbhs_deveptier: [u8; 0x28],
    _reserved15: [u8; 0x08],
    _reserved_15_usbhs_deveptidr: [u8; 0x28],
    _reserved16: [u8; 0xc8],
    #[doc = "0x310..0x380 - Device DMA Channel Next Descriptor Address Register"]
    pub usbhs_devdma: [USBHS_DEVDMA; 7],
    _reserved17: [u8; 0x80],
    #[doc = "0x400 - Host General Control Register"]
    pub usbhs_hstctrl: crate::Reg<usbhs_hstctrl::USBHS_HSTCTRL_SPEC>,
    #[doc = "0x404 - Host Global Interrupt Status Register"]
    pub usbhs_hstisr: crate::Reg<usbhs_hstisr::USBHS_HSTISR_SPEC>,
    #[doc = "0x408 - Host Global Interrupt Clear Register"]
    pub usbhs_hsticr: crate::Reg<usbhs_hsticr::USBHS_HSTICR_SPEC>,
    #[doc = "0x40c - Host Global Interrupt Set Register"]
    pub usbhs_hstifr: crate::Reg<usbhs_hstifr::USBHS_HSTIFR_SPEC>,
    #[doc = "0x410 - Host Global Interrupt Mask Register"]
    pub usbhs_hstimr: crate::Reg<usbhs_hstimr::USBHS_HSTIMR_SPEC>,
    #[doc = "0x414 - Host Global Interrupt Disable Register"]
    pub usbhs_hstidr: crate::Reg<usbhs_hstidr::USBHS_HSTIDR_SPEC>,
    #[doc = "0x418 - Host Global Interrupt Enable Register"]
    pub usbhs_hstier: crate::Reg<usbhs_hstier::USBHS_HSTIER_SPEC>,
    #[doc = "0x41c - Host Pipe Register"]
    pub usbhs_hstpip: crate::Reg<usbhs_hstpip::USBHS_HSTPIP_SPEC>,
    #[doc = "0x420 - Host Frame Number Register"]
    pub usbhs_hstfnum: crate::Reg<usbhs_hstfnum::USBHS_HSTFNUM_SPEC>,
    #[doc = "0x424 - Host Address 1 Register"]
    pub usbhs_hstaddr1: crate::Reg<usbhs_hstaddr1::USBHS_HSTADDR1_SPEC>,
    #[doc = "0x428 - Host Address 2 Register"]
    pub usbhs_hstaddr2: crate::Reg<usbhs_hstaddr2::USBHS_HSTADDR2_SPEC>,
    #[doc = "0x42c - Host Address 3 Register"]
    pub usbhs_hstaddr3: crate::Reg<usbhs_hstaddr3::USBHS_HSTADDR3_SPEC>,
    _reserved29: [u8; 0xd0],
    _reserved_29_usbhs_: [u8; 0x28],
    _reserved30: [u8; 0x08],
    _reserved_30_usbhs_hstpipisr: [u8; 0x28],
    _reserved31: [u8; 0x08],
    _reserved_31_usbhs_hstpipicr: [u8; 0x28],
    _reserved32: [u8; 0x08],
    _reserved_32_usbhs_hstpipifr: [u8; 0x28],
    _reserved33: [u8; 0x08],
    _reserved_33_usbhs_hstpipimr: [u8; 0x28],
    _reserved34: [u8; 0x08],
    _reserved_34_usbhs_hstpipier: [u8; 0x28],
    _reserved35: [u8; 0x08],
    _reserved_35_usbhs_hstpipidr: [u8; 0x28],
    _reserved36: [u8; 0x08],
    #[doc = "0x650..0x678 - Host Pipe IN Request Register"]
    pub usbhs_hstpipinrq: [crate::Reg<usbhs_hstpipinrq::USBHS_HSTPIPINRQ_SPEC>; 10],
    _reserved37: [u8; 0x08],
    #[doc = "0x680..0x6a8 - Host Pipe Error Register"]
    pub usbhs_hstpiperr: [crate::Reg<usbhs_hstpiperr::USBHS_HSTPIPERR_SPEC>; 10],
    _reserved38: [u8; 0x68],
    #[doc = "0x710..0x780 - Host DMA Channel Next Descriptor Address Register"]
    pub usbhs_hstdma: [USBHS_HSTDMA; 7],
    _reserved39: [u8; 0x80],
    #[doc = "0x800 - General Control Register"]
    pub usbhs_ctrl: crate::Reg<usbhs_ctrl::USBHS_CTRL_SPEC>,
    #[doc = "0x804 - General Status Register"]
    pub usbhs_sr: crate::Reg<usbhs_sr::USBHS_SR_SPEC>,
    #[doc = "0x808 - General Status Clear Register"]
    pub usbhs_scr: crate::Reg<usbhs_scr::USBHS_SCR_SPEC>,
    #[doc = "0x80c - General Status Set Register"]
    pub usbhs_sfr: crate::Reg<usbhs_sfr::USBHS_SFR_SPEC>,
}
impl RegisterBlock {
    #[doc = "0x130..0x158 - Device Endpoint Interrupt Status Register"]
    #[inline(always)]
    pub fn usbhs_deveptisr_intrpt_mode(
        &self,
    ) -> &[crate::Reg<usbhs_deveptisr_intrpt_mode::USBHS_DEVEPTISR_INTRPT_MODE_SPEC>; 10] {
        unsafe {
            &*(((self as *const Self) as *const u8).add(304usize)
                as *const [crate::Reg<
                    usbhs_deveptisr_intrpt_mode::USBHS_DEVEPTISR_INTRPT_MODE_SPEC,
                >; 10])
        }
    }
    #[doc = "0x130..0x158 - Device Endpoint Interrupt Status Register"]
    #[inline(always)]
    pub fn usbhs_deveptisr_blk_mode(
        &self,
    ) -> &[crate::Reg<usbhs_deveptisr_blk_mode::USBHS_DEVEPTISR_BLK_MODE_SPEC>; 10] {
        unsafe {
            &*(((self as *const Self) as *const u8).add(304usize)
                as *const [crate::Reg<usbhs_deveptisr_blk_mode::USBHS_DEVEPTISR_BLK_MODE_SPEC>; 10])
        }
    }
    #[doc = "0x130..0x158 - Device Endpoint Interrupt Status Register"]
    #[inline(always)]
    pub fn usbhs_deveptisr_iso_mode(
        &self,
    ) -> &[crate::Reg<usbhs_deveptisr_iso_mode::USBHS_DEVEPTISR_ISO_MODE_SPEC>; 10] {
        unsafe {
            &*(((self as *const Self) as *const u8).add(304usize)
                as *const [crate::Reg<usbhs_deveptisr_iso_mode::USBHS_DEVEPTISR_ISO_MODE_SPEC>; 10])
        }
    }
    #[doc = "0x130..0x158 - Device Endpoint Interrupt Status Register"]
    #[inline(always)]
    pub fn usbhs_deveptisr_ctrl_mode(
        &self,
    ) -> &[crate::Reg<usbhs_deveptisr_ctrl_mode::USBHS_DEVEPTISR_CTRL_MODE_SPEC>; 10] {
        unsafe {
            &*(((self as *const Self) as *const u8).add(304usize)
                as *const [crate::Reg<usbhs_deveptisr_ctrl_mode::USBHS_DEVEPTISR_CTRL_MODE_SPEC>;
                    10])
        }
    }
    #[doc = "0x160..0x188 - Device Endpoint Interrupt Clear Register"]
    #[inline(always)]
    pub fn usbhs_devepticr_intrpt_mode(
        &self,
    ) -> &[crate::Reg<usbhs_devepticr_intrpt_mode::USBHS_DEVEPTICR_INTRPT_MODE_SPEC>; 10] {
        unsafe {
            &*(((self as *const Self) as *const u8).add(352usize)
                as *const [crate::Reg<
                    usbhs_devepticr_intrpt_mode::USBHS_DEVEPTICR_INTRPT_MODE_SPEC,
                >; 10])
        }
    }
    #[doc = "0x160..0x188 - Device Endpoint Interrupt Clear Register"]
    #[inline(always)]
    pub fn usbhs_devepticr_blk_mode(
        &self,
    ) -> &[crate::Reg<usbhs_devepticr_blk_mode::USBHS_DEVEPTICR_BLK_MODE_SPEC>; 10] {
        unsafe {
            &*(((self as *const Self) as *const u8).add(352usize)
                as *const [crate::Reg<usbhs_devepticr_blk_mode::USBHS_DEVEPTICR_BLK_MODE_SPEC>; 10])
        }
    }
    #[doc = "0x160..0x188 - Device Endpoint Interrupt Clear Register"]
    #[inline(always)]
    pub fn usbhs_devepticr_iso_mode(
        &self,
    ) -> &[crate::Reg<usbhs_devepticr_iso_mode::USBHS_DEVEPTICR_ISO_MODE_SPEC>; 10] {
        unsafe {
            &*(((self as *const Self) as *const u8).add(352usize)
                as *const [crate::Reg<usbhs_devepticr_iso_mode::USBHS_DEVEPTICR_ISO_MODE_SPEC>; 10])
        }
    }
    #[doc = "0x160..0x188 - Device Endpoint Interrupt Clear Register"]
    #[inline(always)]
    pub fn usbhs_devepticr_ctrl_mode(
        &self,
    ) -> &[crate::Reg<usbhs_devepticr_ctrl_mode::USBHS_DEVEPTICR_CTRL_MODE_SPEC>; 10] {
        unsafe {
            &*(((self as *const Self) as *const u8).add(352usize)
                as *const [crate::Reg<usbhs_devepticr_ctrl_mode::USBHS_DEVEPTICR_CTRL_MODE_SPEC>;
                    10])
        }
    }
    #[doc = "0x190..0x1b8 - Device Endpoint Interrupt Set Register"]
    #[inline(always)]
    pub fn usbhs_deveptifr_intrpt_mode(
        &self,
    ) -> &[crate::Reg<usbhs_deveptifr_intrpt_mode::USBHS_DEVEPTIFR_INTRPT_MODE_SPEC>; 10] {
        unsafe {
            &*(((self as *const Self) as *const u8).add(400usize)
                as *const [crate::Reg<
                    usbhs_deveptifr_intrpt_mode::USBHS_DEVEPTIFR_INTRPT_MODE_SPEC,
                >; 10])
        }
    }
    #[doc = "0x190..0x1b8 - Device Endpoint Interrupt Set Register"]
    #[inline(always)]
    pub fn usbhs_deveptifr_blk_mode(
        &self,
    ) -> &[crate::Reg<usbhs_deveptifr_blk_mode::USBHS_DEVEPTIFR_BLK_MODE_SPEC>; 10] {
        unsafe {
            &*(((self as *const Self) as *const u8).add(400usize)
                as *const [crate::Reg<usbhs_deveptifr_blk_mode::USBHS_DEVEPTIFR_BLK_MODE_SPEC>; 10])
        }
    }
    #[doc = "0x190..0x1b8 - Device Endpoint Interrupt Set Register"]
    #[inline(always)]
    pub fn usbhs_deveptifr_iso_mode(
        &self,
    ) -> &[crate::Reg<usbhs_deveptifr_iso_mode::USBHS_DEVEPTIFR_ISO_MODE_SPEC>; 10] {
        unsafe {
            &*(((self as *const Self) as *const u8).add(400usize)
                as *const [crate::Reg<usbhs_deveptifr_iso_mode::USBHS_DEVEPTIFR_ISO_MODE_SPEC>; 10])
        }
    }
    #[doc = "0x190..0x1b8 - Device Endpoint Interrupt Set Register"]
    #[inline(always)]
    pub fn usbhs_deveptifr_ctrl_mode(
        &self,
    ) -> &[crate::Reg<usbhs_deveptifr_ctrl_mode::USBHS_DEVEPTIFR_CTRL_MODE_SPEC>; 10] {
        unsafe {
            &*(((self as *const Self) as *const u8).add(400usize)
                as *const [crate::Reg<usbhs_deveptifr_ctrl_mode::USBHS_DEVEPTIFR_CTRL_MODE_SPEC>;
                    10])
        }
    }
    #[doc = "0x1c0..0x1e8 - Device Endpoint Interrupt Mask Register"]
    #[inline(always)]
    pub fn usbhs_deveptimr_intrpt_mode(
        &self,
    ) -> &[crate::Reg<usbhs_deveptimr_intrpt_mode::USBHS_DEVEPTIMR_INTRPT_MODE_SPEC>; 10] {
        unsafe {
            &*(((self as *const Self) as *const u8).add(448usize)
                as *const [crate::Reg<
                    usbhs_deveptimr_intrpt_mode::USBHS_DEVEPTIMR_INTRPT_MODE_SPEC,
                >; 10])
        }
    }
    #[doc = "0x1c0..0x1e8 - Device Endpoint Interrupt Mask Register"]
    #[inline(always)]
    pub fn usbhs_deveptimr_blk_mode(
        &self,
    ) -> &[crate::Reg<usbhs_deveptimr_blk_mode::USBHS_DEVEPTIMR_BLK_MODE_SPEC>; 10] {
        unsafe {
            &*(((self as *const Self) as *const u8).add(448usize)
                as *const [crate::Reg<usbhs_deveptimr_blk_mode::USBHS_DEVEPTIMR_BLK_MODE_SPEC>; 10])
        }
    }
    #[doc = "0x1c0..0x1e8 - Device Endpoint Interrupt Mask Register"]
    #[inline(always)]
    pub fn usbhs_deveptimr_iso_mode(
        &self,
    ) -> &[crate::Reg<usbhs_deveptimr_iso_mode::USBHS_DEVEPTIMR_ISO_MODE_SPEC>; 10] {
        unsafe {
            &*(((self as *const Self) as *const u8).add(448usize)
                as *const [crate::Reg<usbhs_deveptimr_iso_mode::USBHS_DEVEPTIMR_ISO_MODE_SPEC>; 10])
        }
    }
    #[doc = "0x1c0..0x1e8 - Device Endpoint Interrupt Mask Register"]
    #[inline(always)]
    pub fn usbhs_deveptimr_ctrl_mode(
        &self,
    ) -> &[crate::Reg<usbhs_deveptimr_ctrl_mode::USBHS_DEVEPTIMR_CTRL_MODE_SPEC>; 10] {
        unsafe {
            &*(((self as *const Self) as *const u8).add(448usize)
                as *const [crate::Reg<usbhs_deveptimr_ctrl_mode::USBHS_DEVEPTIMR_CTRL_MODE_SPEC>;
                    10])
        }
    }
    #[doc = "0x1f0..0x218 - Device Endpoint Interrupt Enable Register"]
    #[inline(always)]
    pub fn usbhs_deveptier_intrpt_mode(
        &self,
    ) -> &[crate::Reg<usbhs_deveptier_intrpt_mode::USBHS_DEVEPTIER_INTRPT_MODE_SPEC>; 10] {
        unsafe {
            &*(((self as *const Self) as *const u8).add(496usize)
                as *const [crate::Reg<
                    usbhs_deveptier_intrpt_mode::USBHS_DEVEPTIER_INTRPT_MODE_SPEC,
                >; 10])
        }
    }
    #[doc = "0x1f0..0x218 - Device Endpoint Interrupt Enable Register"]
    #[inline(always)]
    pub fn usbhs_deveptier_blk_mode(
        &self,
    ) -> &[crate::Reg<usbhs_deveptier_blk_mode::USBHS_DEVEPTIER_BLK_MODE_SPEC>; 10] {
        unsafe {
            &*(((self as *const Self) as *const u8).add(496usize)
                as *const [crate::Reg<usbhs_deveptier_blk_mode::USBHS_DEVEPTIER_BLK_MODE_SPEC>; 10])
        }
    }
    #[doc = "0x1f0..0x218 - Device Endpoint Interrupt Enable Register"]
    #[inline(always)]
    pub fn usbhs_deveptier_iso_mode(
        &self,
    ) -> &[crate::Reg<usbhs_deveptier_iso_mode::USBHS_DEVEPTIER_ISO_MODE_SPEC>; 10] {
        unsafe {
            &*(((self as *const Self) as *const u8).add(496usize)
                as *const [crate::Reg<usbhs_deveptier_iso_mode::USBHS_DEVEPTIER_ISO_MODE_SPEC>; 10])
        }
    }
    #[doc = "0x1f0..0x218 - Device Endpoint Interrupt Enable Register"]
    #[inline(always)]
    pub fn usbhs_deveptier_ctrl_mode(
        &self,
    ) -> &[crate::Reg<usbhs_deveptier_ctrl_mode::USBHS_DEVEPTIER_CTRL_MODE_SPEC>; 10] {
        unsafe {
            &*(((self as *const Self) as *const u8).add(496usize)
                as *const [crate::Reg<usbhs_deveptier_ctrl_mode::USBHS_DEVEPTIER_CTRL_MODE_SPEC>;
                    10])
        }
    }
    #[doc = "0x220..0x248 - Device Endpoint Interrupt Disable Register"]
    #[inline(always)]
    pub fn usbhs_deveptidr_intrpt_mode(
        &self,
    ) -> &[crate::Reg<usbhs_deveptidr_intrpt_mode::USBHS_DEVEPTIDR_INTRPT_MODE_SPEC>; 10] {
        unsafe {
            &*(((self as *const Self) as *const u8).add(544usize)
                as *const [crate::Reg<
                    usbhs_deveptidr_intrpt_mode::USBHS_DEVEPTIDR_INTRPT_MODE_SPEC,
                >; 10])
        }
    }
    #[doc = "0x220..0x248 - Device Endpoint Interrupt Disable Register"]
    #[inline(always)]
    pub fn usbhs_deveptidr_blk_mode(
        &self,
    ) -> &[crate::Reg<usbhs_deveptidr_blk_mode::USBHS_DEVEPTIDR_BLK_MODE_SPEC>; 10] {
        unsafe {
            &*(((self as *const Self) as *const u8).add(544usize)
                as *const [crate::Reg<usbhs_deveptidr_blk_mode::USBHS_DEVEPTIDR_BLK_MODE_SPEC>; 10])
        }
    }
    #[doc = "0x220..0x248 - Device Endpoint Interrupt Disable Register"]
    #[inline(always)]
    pub fn usbhs_deveptidr_iso_mode(
        &self,
    ) -> &[crate::Reg<usbhs_deveptidr_iso_mode::USBHS_DEVEPTIDR_ISO_MODE_SPEC>; 10] {
        unsafe {
            &*(((self as *const Self) as *const u8).add(544usize)
                as *const [crate::Reg<usbhs_deveptidr_iso_mode::USBHS_DEVEPTIDR_ISO_MODE_SPEC>; 10])
        }
    }
    #[doc = "0x220..0x248 - Device Endpoint Interrupt Disable Register"]
    #[inline(always)]
    pub fn usbhs_deveptidr_ctrl_mode(
        &self,
    ) -> &[crate::Reg<usbhs_deveptidr_ctrl_mode::USBHS_DEVEPTIDR_CTRL_MODE_SPEC>; 10] {
        unsafe {
            &*(((self as *const Self) as *const u8).add(544usize)
                as *const [crate::Reg<usbhs_deveptidr_ctrl_mode::USBHS_DEVEPTIDR_CTRL_MODE_SPEC>;
                    10])
        }
    }
    #[doc = "0x500..0x528 - Host Pipe Configuration Register"]
    #[inline(always)]
    pub fn usbhs_hstpipcfg_ctrl_bulk_mode(
        &self,
    ) -> &[crate::Reg<usbhs_hstpipcfg_ctrl_bulk_mode::USBHS_HSTPIPCFG_CTRL_BULK_MODE_SPEC>; 10]
    {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1280usize)
                as *const [crate::Reg<
                    usbhs_hstpipcfg_ctrl_bulk_mode::USBHS_HSTPIPCFG_CTRL_BULK_MODE_SPEC,
                >; 10])
        }
    }
    #[doc = "0x500..0x528 - Host Pipe Configuration Register"]
    #[inline(always)]
    pub fn usbhs_hstpipcfg(&self) -> &[crate::Reg<usbhs_hstpipcfg::USBHS_HSTPIPCFG_SPEC>; 10] {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1280usize)
                as *const [crate::Reg<usbhs_hstpipcfg::USBHS_HSTPIPCFG_SPEC>; 10])
        }
    }
    #[doc = "0x530..0x558 - Host Pipe Status Register"]
    #[inline(always)]
    pub fn usbhs_hstpipisr_intrpt_mode(
        &self,
    ) -> &[crate::Reg<usbhs_hstpipisr_intrpt_mode::USBHS_HSTPIPISR_INTRPT_MODE_SPEC>; 10] {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1328usize)
                as *const [crate::Reg<
                    usbhs_hstpipisr_intrpt_mode::USBHS_HSTPIPISR_INTRPT_MODE_SPEC,
                >; 10])
        }
    }
    #[doc = "0x530..0x558 - Host Pipe Status Register"]
    #[inline(always)]
    pub fn usbhs_hstpipisr_blk_mode(
        &self,
    ) -> &[crate::Reg<usbhs_hstpipisr_blk_mode::USBHS_HSTPIPISR_BLK_MODE_SPEC>; 10] {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1328usize)
                as *const [crate::Reg<usbhs_hstpipisr_blk_mode::USBHS_HSTPIPISR_BLK_MODE_SPEC>; 10])
        }
    }
    #[doc = "0x530..0x558 - Host Pipe Status Register"]
    #[inline(always)]
    pub fn usbhs_hstpipisr_iso_mode(
        &self,
    ) -> &[crate::Reg<usbhs_hstpipisr_iso_mode::USBHS_HSTPIPISR_ISO_MODE_SPEC>; 10] {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1328usize)
                as *const [crate::Reg<usbhs_hstpipisr_iso_mode::USBHS_HSTPIPISR_ISO_MODE_SPEC>; 10])
        }
    }
    #[doc = "0x530..0x558 - Host Pipe Status Register"]
    #[inline(always)]
    pub fn usbhs_hstpipisr_ctrl_mode(
        &self,
    ) -> &[crate::Reg<usbhs_hstpipisr_ctrl_mode::USBHS_HSTPIPISR_CTRL_MODE_SPEC>; 10] {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1328usize)
                as *const [crate::Reg<usbhs_hstpipisr_ctrl_mode::USBHS_HSTPIPISR_CTRL_MODE_SPEC>;
                    10])
        }
    }
    #[doc = "0x560..0x588 - Host Pipe Clear Register"]
    #[inline(always)]
    pub fn usbhs_hstpipicr_intrpt_mode(
        &self,
    ) -> &[crate::Reg<usbhs_hstpipicr_intrpt_mode::USBHS_HSTPIPICR_INTRPT_MODE_SPEC>; 10] {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1376usize)
                as *const [crate::Reg<
                    usbhs_hstpipicr_intrpt_mode::USBHS_HSTPIPICR_INTRPT_MODE_SPEC,
                >; 10])
        }
    }
    #[doc = "0x560..0x588 - Host Pipe Clear Register"]
    #[inline(always)]
    pub fn usbhs_hstpipicr_blk_mode(
        &self,
    ) -> &[crate::Reg<usbhs_hstpipicr_blk_mode::USBHS_HSTPIPICR_BLK_MODE_SPEC>; 10] {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1376usize)
                as *const [crate::Reg<usbhs_hstpipicr_blk_mode::USBHS_HSTPIPICR_BLK_MODE_SPEC>; 10])
        }
    }
    #[doc = "0x560..0x588 - Host Pipe Clear Register"]
    #[inline(always)]
    pub fn usbhs_hstpipicr_iso_mode(
        &self,
    ) -> &[crate::Reg<usbhs_hstpipicr_iso_mode::USBHS_HSTPIPICR_ISO_MODE_SPEC>; 10] {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1376usize)
                as *const [crate::Reg<usbhs_hstpipicr_iso_mode::USBHS_HSTPIPICR_ISO_MODE_SPEC>; 10])
        }
    }
    #[doc = "0x560..0x588 - Host Pipe Clear Register"]
    #[inline(always)]
    pub fn usbhs_hstpipicr_ctrl_mode(
        &self,
    ) -> &[crate::Reg<usbhs_hstpipicr_ctrl_mode::USBHS_HSTPIPICR_CTRL_MODE_SPEC>; 10] {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1376usize)
                as *const [crate::Reg<usbhs_hstpipicr_ctrl_mode::USBHS_HSTPIPICR_CTRL_MODE_SPEC>;
                    10])
        }
    }
    #[doc = "0x590..0x5b8 - Host Pipe Set Register"]
    #[inline(always)]
    pub fn usbhs_hstpipifr_intrpt_mode(
        &self,
    ) -> &[crate::Reg<usbhs_hstpipifr_intrpt_mode::USBHS_HSTPIPIFR_INTRPT_MODE_SPEC>; 10] {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1424usize)
                as *const [crate::Reg<
                    usbhs_hstpipifr_intrpt_mode::USBHS_HSTPIPIFR_INTRPT_MODE_SPEC,
                >; 10])
        }
    }
    #[doc = "0x590..0x5b8 - Host Pipe Set Register"]
    #[inline(always)]
    pub fn usbhs_hstpipifr_blk_mode(
        &self,
    ) -> &[crate::Reg<usbhs_hstpipifr_blk_mode::USBHS_HSTPIPIFR_BLK_MODE_SPEC>; 10] {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1424usize)
                as *const [crate::Reg<usbhs_hstpipifr_blk_mode::USBHS_HSTPIPIFR_BLK_MODE_SPEC>; 10])
        }
    }
    #[doc = "0x590..0x5b8 - Host Pipe Set Register"]
    #[inline(always)]
    pub fn usbhs_hstpipifr_iso_mode(
        &self,
    ) -> &[crate::Reg<usbhs_hstpipifr_iso_mode::USBHS_HSTPIPIFR_ISO_MODE_SPEC>; 10] {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1424usize)
                as *const [crate::Reg<usbhs_hstpipifr_iso_mode::USBHS_HSTPIPIFR_ISO_MODE_SPEC>; 10])
        }
    }
    #[doc = "0x590..0x5b8 - Host Pipe Set Register"]
    #[inline(always)]
    pub fn usbhs_hstpipifr_ctrl_mode(
        &self,
    ) -> &[crate::Reg<usbhs_hstpipifr_ctrl_mode::USBHS_HSTPIPIFR_CTRL_MODE_SPEC>; 10] {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1424usize)
                as *const [crate::Reg<usbhs_hstpipifr_ctrl_mode::USBHS_HSTPIPIFR_CTRL_MODE_SPEC>;
                    10])
        }
    }
    #[doc = "0x5c0..0x5e8 - Host Pipe Mask Register"]
    #[inline(always)]
    pub fn usbhs_hstpipimr_intrpt_mode(
        &self,
    ) -> &[crate::Reg<usbhs_hstpipimr_intrpt_mode::USBHS_HSTPIPIMR_INTRPT_MODE_SPEC>; 10] {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1472usize)
                as *const [crate::Reg<
                    usbhs_hstpipimr_intrpt_mode::USBHS_HSTPIPIMR_INTRPT_MODE_SPEC,
                >; 10])
        }
    }
    #[doc = "0x5c0..0x5e8 - Host Pipe Mask Register"]
    #[inline(always)]
    pub fn usbhs_hstpipimr_blk_mode(
        &self,
    ) -> &[crate::Reg<usbhs_hstpipimr_blk_mode::USBHS_HSTPIPIMR_BLK_MODE_SPEC>; 10] {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1472usize)
                as *const [crate::Reg<usbhs_hstpipimr_blk_mode::USBHS_HSTPIPIMR_BLK_MODE_SPEC>; 10])
        }
    }
    #[doc = "0x5c0..0x5e8 - Host Pipe Mask Register"]
    #[inline(always)]
    pub fn usbhs_hstpipimr_iso_mode(
        &self,
    ) -> &[crate::Reg<usbhs_hstpipimr_iso_mode::USBHS_HSTPIPIMR_ISO_MODE_SPEC>; 10] {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1472usize)
                as *const [crate::Reg<usbhs_hstpipimr_iso_mode::USBHS_HSTPIPIMR_ISO_MODE_SPEC>; 10])
        }
    }
    #[doc = "0x5c0..0x5e8 - Host Pipe Mask Register"]
    #[inline(always)]
    pub fn usbhs_hstpipimr_ctrl_mode(
        &self,
    ) -> &[crate::Reg<usbhs_hstpipimr_ctrl_mode::USBHS_HSTPIPIMR_CTRL_MODE_SPEC>; 10] {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1472usize)
                as *const [crate::Reg<usbhs_hstpipimr_ctrl_mode::USBHS_HSTPIPIMR_CTRL_MODE_SPEC>;
                    10])
        }
    }
    #[doc = "0x5f0..0x618 - Host Pipe Enable Register"]
    #[inline(always)]
    pub fn usbhs_hstpipier_intrpt_mode(
        &self,
    ) -> &[crate::Reg<usbhs_hstpipier_intrpt_mode::USBHS_HSTPIPIER_INTRPT_MODE_SPEC>; 10] {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1520usize)
                as *const [crate::Reg<
                    usbhs_hstpipier_intrpt_mode::USBHS_HSTPIPIER_INTRPT_MODE_SPEC,
                >; 10])
        }
    }
    #[doc = "0x5f0..0x618 - Host Pipe Enable Register"]
    #[inline(always)]
    pub fn usbhs_hstpipier_blk_mode(
        &self,
    ) -> &[crate::Reg<usbhs_hstpipier_blk_mode::USBHS_HSTPIPIER_BLK_MODE_SPEC>; 10] {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1520usize)
                as *const [crate::Reg<usbhs_hstpipier_blk_mode::USBHS_HSTPIPIER_BLK_MODE_SPEC>; 10])
        }
    }
    #[doc = "0x5f0..0x618 - Host Pipe Enable Register"]
    #[inline(always)]
    pub fn usbhs_hstpipier_iso_mode(
        &self,
    ) -> &[crate::Reg<usbhs_hstpipier_iso_mode::USBHS_HSTPIPIER_ISO_MODE_SPEC>; 10] {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1520usize)
                as *const [crate::Reg<usbhs_hstpipier_iso_mode::USBHS_HSTPIPIER_ISO_MODE_SPEC>; 10])
        }
    }
    #[doc = "0x5f0..0x618 - Host Pipe Enable Register"]
    #[inline(always)]
    pub fn usbhs_hstpipier_ctrl_mode(
        &self,
    ) -> &[crate::Reg<usbhs_hstpipier_ctrl_mode::USBHS_HSTPIPIER_CTRL_MODE_SPEC>; 10] {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1520usize)
                as *const [crate::Reg<usbhs_hstpipier_ctrl_mode::USBHS_HSTPIPIER_CTRL_MODE_SPEC>;
                    10])
        }
    }
    #[doc = "0x620..0x648 - Host Pipe Disable Register"]
    #[inline(always)]
    pub fn usbhs_hstpipidr_intrpt_mode(
        &self,
    ) -> &[crate::Reg<usbhs_hstpipidr_intrpt_mode::USBHS_HSTPIPIDR_INTRPT_MODE_SPEC>; 10] {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1568usize)
                as *const [crate::Reg<
                    usbhs_hstpipidr_intrpt_mode::USBHS_HSTPIPIDR_INTRPT_MODE_SPEC,
                >; 10])
        }
    }
    #[doc = "0x620..0x648 - Host Pipe Disable Register"]
    #[inline(always)]
    pub fn usbhs_hstpipidr_blk_mode(
        &self,
    ) -> &[crate::Reg<usbhs_hstpipidr_blk_mode::USBHS_HSTPIPIDR_BLK_MODE_SPEC>; 10] {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1568usize)
                as *const [crate::Reg<usbhs_hstpipidr_blk_mode::USBHS_HSTPIPIDR_BLK_MODE_SPEC>; 10])
        }
    }
    #[doc = "0x620..0x648 - Host Pipe Disable Register"]
    #[inline(always)]
    pub fn usbhs_hstpipidr_iso_mode(
        &self,
    ) -> &[crate::Reg<usbhs_hstpipidr_iso_mode::USBHS_HSTPIPIDR_ISO_MODE_SPEC>; 10] {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1568usize)
                as *const [crate::Reg<usbhs_hstpipidr_iso_mode::USBHS_HSTPIPIDR_ISO_MODE_SPEC>; 10])
        }
    }
    #[doc = "0x620..0x648 - Host Pipe Disable Register"]
    #[inline(always)]
    pub fn usbhs_hstpipidr_ctrl_mode(
        &self,
    ) -> &[crate::Reg<usbhs_hstpipidr_ctrl_mode::USBHS_HSTPIPIDR_CTRL_MODE_SPEC>; 10] {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1568usize)
                as *const [crate::Reg<usbhs_hstpipidr_ctrl_mode::USBHS_HSTPIPIDR_CTRL_MODE_SPEC>;
                    10])
        }
    }
}
#[doc = r"Register block"]
#[repr(C)]
pub struct USBHS_DEVDMA {
    #[doc = "0x00 - Device DMA Channel Next Descriptor Address Register"]
    pub usbhs_devdmanxtdsc:
        crate::Reg<self::usbhs_devdma::usbhs_devdmanxtdsc::USBHS_DEVDMANXTDSC_SPEC>,
    #[doc = "0x04 - Device DMA Channel Address Register"]
    pub usbhs_devdmaaddress:
        crate::Reg<self::usbhs_devdma::usbhs_devdmaaddress::USBHS_DEVDMAADDRESS_SPEC>,
    #[doc = "0x08 - Device DMA Channel Control Register"]
    pub usbhs_devdmacontrol:
        crate::Reg<self::usbhs_devdma::usbhs_devdmacontrol::USBHS_DEVDMACONTROL_SPEC>,
    #[doc = "0x0c - Device DMA Channel Status Register"]
    pub usbhs_devdmastatus:
        crate::Reg<self::usbhs_devdma::usbhs_devdmastatus::USBHS_DEVDMASTATUS_SPEC>,
}
#[doc = r"Register block"]
#[doc = "Device DMA Channel Next Descriptor Address Register"]
pub mod usbhs_devdma;
#[doc = r"Register block"]
#[repr(C)]
pub struct USBHS_HSTDMA {
    #[doc = "0x00 - Host DMA Channel Next Descriptor Address Register"]
    pub usbhs_hstdmanxtdsc:
        crate::Reg<self::usbhs_hstdma::usbhs_hstdmanxtdsc::USBHS_HSTDMANXTDSC_SPEC>,
    #[doc = "0x04 - Host DMA Channel Address Register"]
    pub usbhs_hstdmaaddress:
        crate::Reg<self::usbhs_hstdma::usbhs_hstdmaaddress::USBHS_HSTDMAADDRESS_SPEC>,
    #[doc = "0x08 - Host DMA Channel Control Register"]
    pub usbhs_hstdmacontrol:
        crate::Reg<self::usbhs_hstdma::usbhs_hstdmacontrol::USBHS_HSTDMACONTROL_SPEC>,
    #[doc = "0x0c - Host DMA Channel Status Register"]
    pub usbhs_hstdmastatus:
        crate::Reg<self::usbhs_hstdma::usbhs_hstdmastatus::USBHS_HSTDMASTATUS_SPEC>,
}
#[doc = r"Register block"]
#[doc = "Host DMA Channel Next Descriptor Address Register"]
pub mod usbhs_hstdma;
#[doc = "USBHS_DEVCTRL register accessor: an alias for `Reg<USBHS_DEVCTRL_SPEC>`"]
pub type USBHS_DEVCTRL = crate::Reg<usbhs_devctrl::USBHS_DEVCTRL_SPEC>;
#[doc = "Device General Control Register"]
pub mod usbhs_devctrl;
#[doc = "USBHS_DEVISR register accessor: an alias for `Reg<USBHS_DEVISR_SPEC>`"]
pub type USBHS_DEVISR = crate::Reg<usbhs_devisr::USBHS_DEVISR_SPEC>;
#[doc = "Device Global Interrupt Status Register"]
pub mod usbhs_devisr;
#[doc = "USBHS_DEVICR register accessor: an alias for `Reg<USBHS_DEVICR_SPEC>`"]
pub type USBHS_DEVICR = crate::Reg<usbhs_devicr::USBHS_DEVICR_SPEC>;
#[doc = "Device Global Interrupt Clear Register"]
pub mod usbhs_devicr;
#[doc = "USBHS_DEVIFR register accessor: an alias for `Reg<USBHS_DEVIFR_SPEC>`"]
pub type USBHS_DEVIFR = crate::Reg<usbhs_devifr::USBHS_DEVIFR_SPEC>;
#[doc = "Device Global Interrupt Set Register"]
pub mod usbhs_devifr;
#[doc = "USBHS_DEVIMR register accessor: an alias for `Reg<USBHS_DEVIMR_SPEC>`"]
pub type USBHS_DEVIMR = crate::Reg<usbhs_devimr::USBHS_DEVIMR_SPEC>;
#[doc = "Device Global Interrupt Mask Register"]
pub mod usbhs_devimr;
#[doc = "USBHS_DEVIDR register accessor: an alias for `Reg<USBHS_DEVIDR_SPEC>`"]
pub type USBHS_DEVIDR = crate::Reg<usbhs_devidr::USBHS_DEVIDR_SPEC>;
#[doc = "Device Global Interrupt Disable Register"]
pub mod usbhs_devidr;
#[doc = "USBHS_DEVIER register accessor: an alias for `Reg<USBHS_DEVIER_SPEC>`"]
pub type USBHS_DEVIER = crate::Reg<usbhs_devier::USBHS_DEVIER_SPEC>;
#[doc = "Device Global Interrupt Enable Register"]
pub mod usbhs_devier;
#[doc = "USBHS_DEVEPT register accessor: an alias for `Reg<USBHS_DEVEPT_SPEC>`"]
pub type USBHS_DEVEPT = crate::Reg<usbhs_devept::USBHS_DEVEPT_SPEC>;
#[doc = "Device Endpoint Register"]
pub mod usbhs_devept;
#[doc = "USBHS_DEVFNUM register accessor: an alias for `Reg<USBHS_DEVFNUM_SPEC>`"]
pub type USBHS_DEVFNUM = crate::Reg<usbhs_devfnum::USBHS_DEVFNUM_SPEC>;
#[doc = "Device Frame Number Register"]
pub mod usbhs_devfnum;
#[doc = "USBHS_DEVEPTCFG register accessor: an alias for `Reg<USBHS_DEVEPTCFG_SPEC>`"]
pub type USBHS_DEVEPTCFG = crate::Reg<usbhs_deveptcfg::USBHS_DEVEPTCFG_SPEC>;
#[doc = "Device Endpoint Configuration Register"]
pub mod usbhs_deveptcfg;
#[doc = "USBHS_DEVEPTISR_CTRL_MODE register accessor: an alias for `Reg<USBHS_DEVEPTISR_CTRL_MODE_SPEC>`"]
pub type USBHS_DEVEPTISR_CTRL_MODE =
    crate::Reg<usbhs_deveptisr_ctrl_mode::USBHS_DEVEPTISR_CTRL_MODE_SPEC>;
#[doc = "Device Endpoint Interrupt Status Register"]
pub mod usbhs_deveptisr_ctrl_mode;
#[doc = "USBHS_DEVEPTISR_ISO_MODE register accessor: an alias for `Reg<USBHS_DEVEPTISR_ISO_MODE_SPEC>`"]
pub type USBHS_DEVEPTISR_ISO_MODE =
    crate::Reg<usbhs_deveptisr_iso_mode::USBHS_DEVEPTISR_ISO_MODE_SPEC>;
#[doc = "Device Endpoint Interrupt Status Register"]
pub mod usbhs_deveptisr_iso_mode;
#[doc = "USBHS_DEVEPTISR_BLK_MODE register accessor: an alias for `Reg<USBHS_DEVEPTISR_BLK_MODE_SPEC>`"]
pub type USBHS_DEVEPTISR_BLK_MODE =
    crate::Reg<usbhs_deveptisr_blk_mode::USBHS_DEVEPTISR_BLK_MODE_SPEC>;
#[doc = "Device Endpoint Interrupt Status Register"]
pub mod usbhs_deveptisr_blk_mode;
#[doc = "USBHS_DEVEPTISR_INTRPT_MODE register accessor: an alias for `Reg<USBHS_DEVEPTISR_INTRPT_MODE_SPEC>`"]
pub type USBHS_DEVEPTISR_INTRPT_MODE =
    crate::Reg<usbhs_deveptisr_intrpt_mode::USBHS_DEVEPTISR_INTRPT_MODE_SPEC>;
#[doc = "Device Endpoint Interrupt Status Register"]
pub mod usbhs_deveptisr_intrpt_mode;
#[doc = "USBHS_DEVEPTICR_CTRL_MODE register accessor: an alias for `Reg<USBHS_DEVEPTICR_CTRL_MODE_SPEC>`"]
pub type USBHS_DEVEPTICR_CTRL_MODE =
    crate::Reg<usbhs_devepticr_ctrl_mode::USBHS_DEVEPTICR_CTRL_MODE_SPEC>;
#[doc = "Device Endpoint Interrupt Clear Register"]
pub mod usbhs_devepticr_ctrl_mode;
#[doc = "USBHS_DEVEPTICR_ISO_MODE register accessor: an alias for `Reg<USBHS_DEVEPTICR_ISO_MODE_SPEC>`"]
pub type USBHS_DEVEPTICR_ISO_MODE =
    crate::Reg<usbhs_devepticr_iso_mode::USBHS_DEVEPTICR_ISO_MODE_SPEC>;
#[doc = "Device Endpoint Interrupt Clear Register"]
pub mod usbhs_devepticr_iso_mode;
#[doc = "USBHS_DEVEPTICR_BLK_MODE register accessor: an alias for `Reg<USBHS_DEVEPTICR_BLK_MODE_SPEC>`"]
pub type USBHS_DEVEPTICR_BLK_MODE =
    crate::Reg<usbhs_devepticr_blk_mode::USBHS_DEVEPTICR_BLK_MODE_SPEC>;
#[doc = "Device Endpoint Interrupt Clear Register"]
pub mod usbhs_devepticr_blk_mode;
#[doc = "USBHS_DEVEPTICR_INTRPT_MODE register accessor: an alias for `Reg<USBHS_DEVEPTICR_INTRPT_MODE_SPEC>`"]
pub type USBHS_DEVEPTICR_INTRPT_MODE =
    crate::Reg<usbhs_devepticr_intrpt_mode::USBHS_DEVEPTICR_INTRPT_MODE_SPEC>;
#[doc = "Device Endpoint Interrupt Clear Register"]
pub mod usbhs_devepticr_intrpt_mode;
#[doc = "USBHS_DEVEPTIFR_CTRL_MODE register accessor: an alias for `Reg<USBHS_DEVEPTIFR_CTRL_MODE_SPEC>`"]
pub type USBHS_DEVEPTIFR_CTRL_MODE =
    crate::Reg<usbhs_deveptifr_ctrl_mode::USBHS_DEVEPTIFR_CTRL_MODE_SPEC>;
#[doc = "Device Endpoint Interrupt Set Register"]
pub mod usbhs_deveptifr_ctrl_mode;
#[doc = "USBHS_DEVEPTIFR_ISO_MODE register accessor: an alias for `Reg<USBHS_DEVEPTIFR_ISO_MODE_SPEC>`"]
pub type USBHS_DEVEPTIFR_ISO_MODE =
    crate::Reg<usbhs_deveptifr_iso_mode::USBHS_DEVEPTIFR_ISO_MODE_SPEC>;
#[doc = "Device Endpoint Interrupt Set Register"]
pub mod usbhs_deveptifr_iso_mode;
#[doc = "USBHS_DEVEPTIFR_BLK_MODE register accessor: an alias for `Reg<USBHS_DEVEPTIFR_BLK_MODE_SPEC>`"]
pub type USBHS_DEVEPTIFR_BLK_MODE =
    crate::Reg<usbhs_deveptifr_blk_mode::USBHS_DEVEPTIFR_BLK_MODE_SPEC>;
#[doc = "Device Endpoint Interrupt Set Register"]
pub mod usbhs_deveptifr_blk_mode;
#[doc = "USBHS_DEVEPTIFR_INTRPT_MODE register accessor: an alias for `Reg<USBHS_DEVEPTIFR_INTRPT_MODE_SPEC>`"]
pub type USBHS_DEVEPTIFR_INTRPT_MODE =
    crate::Reg<usbhs_deveptifr_intrpt_mode::USBHS_DEVEPTIFR_INTRPT_MODE_SPEC>;
#[doc = "Device Endpoint Interrupt Set Register"]
pub mod usbhs_deveptifr_intrpt_mode;
#[doc = "USBHS_DEVEPTIMR_CTRL_MODE register accessor: an alias for `Reg<USBHS_DEVEPTIMR_CTRL_MODE_SPEC>`"]
pub type USBHS_DEVEPTIMR_CTRL_MODE =
    crate::Reg<usbhs_deveptimr_ctrl_mode::USBHS_DEVEPTIMR_CTRL_MODE_SPEC>;
#[doc = "Device Endpoint Interrupt Mask Register"]
pub mod usbhs_deveptimr_ctrl_mode;
#[doc = "USBHS_DEVEPTIMR_ISO_MODE register accessor: an alias for `Reg<USBHS_DEVEPTIMR_ISO_MODE_SPEC>`"]
pub type USBHS_DEVEPTIMR_ISO_MODE =
    crate::Reg<usbhs_deveptimr_iso_mode::USBHS_DEVEPTIMR_ISO_MODE_SPEC>;
#[doc = "Device Endpoint Interrupt Mask Register"]
pub mod usbhs_deveptimr_iso_mode;
#[doc = "USBHS_DEVEPTIMR_BLK_MODE register accessor: an alias for `Reg<USBHS_DEVEPTIMR_BLK_MODE_SPEC>`"]
pub type USBHS_DEVEPTIMR_BLK_MODE =
    crate::Reg<usbhs_deveptimr_blk_mode::USBHS_DEVEPTIMR_BLK_MODE_SPEC>;
#[doc = "Device Endpoint Interrupt Mask Register"]
pub mod usbhs_deveptimr_blk_mode;
#[doc = "USBHS_DEVEPTIMR_INTRPT_MODE register accessor: an alias for `Reg<USBHS_DEVEPTIMR_INTRPT_MODE_SPEC>`"]
pub type USBHS_DEVEPTIMR_INTRPT_MODE =
    crate::Reg<usbhs_deveptimr_intrpt_mode::USBHS_DEVEPTIMR_INTRPT_MODE_SPEC>;
#[doc = "Device Endpoint Interrupt Mask Register"]
pub mod usbhs_deveptimr_intrpt_mode;
#[doc = "USBHS_DEVEPTIER_CTRL_MODE register accessor: an alias for `Reg<USBHS_DEVEPTIER_CTRL_MODE_SPEC>`"]
pub type USBHS_DEVEPTIER_CTRL_MODE =
    crate::Reg<usbhs_deveptier_ctrl_mode::USBHS_DEVEPTIER_CTRL_MODE_SPEC>;
#[doc = "Device Endpoint Interrupt Enable Register"]
pub mod usbhs_deveptier_ctrl_mode;
#[doc = "USBHS_DEVEPTIER_ISO_MODE register accessor: an alias for `Reg<USBHS_DEVEPTIER_ISO_MODE_SPEC>`"]
pub type USBHS_DEVEPTIER_ISO_MODE =
    crate::Reg<usbhs_deveptier_iso_mode::USBHS_DEVEPTIER_ISO_MODE_SPEC>;
#[doc = "Device Endpoint Interrupt Enable Register"]
pub mod usbhs_deveptier_iso_mode;
#[doc = "USBHS_DEVEPTIER_BLK_MODE register accessor: an alias for `Reg<USBHS_DEVEPTIER_BLK_MODE_SPEC>`"]
pub type USBHS_DEVEPTIER_BLK_MODE =
    crate::Reg<usbhs_deveptier_blk_mode::USBHS_DEVEPTIER_BLK_MODE_SPEC>;
#[doc = "Device Endpoint Interrupt Enable Register"]
pub mod usbhs_deveptier_blk_mode;
#[doc = "USBHS_DEVEPTIER_INTRPT_MODE register accessor: an alias for `Reg<USBHS_DEVEPTIER_INTRPT_MODE_SPEC>`"]
pub type USBHS_DEVEPTIER_INTRPT_MODE =
    crate::Reg<usbhs_deveptier_intrpt_mode::USBHS_DEVEPTIER_INTRPT_MODE_SPEC>;
#[doc = "Device Endpoint Interrupt Enable Register"]
pub mod usbhs_deveptier_intrpt_mode;
#[doc = "USBHS_DEVEPTIDR_CTRL_MODE register accessor: an alias for `Reg<USBHS_DEVEPTIDR_CTRL_MODE_SPEC>`"]
pub type USBHS_DEVEPTIDR_CTRL_MODE =
    crate::Reg<usbhs_deveptidr_ctrl_mode::USBHS_DEVEPTIDR_CTRL_MODE_SPEC>;
#[doc = "Device Endpoint Interrupt Disable Register"]
pub mod usbhs_deveptidr_ctrl_mode;
#[doc = "USBHS_DEVEPTIDR_ISO_MODE register accessor: an alias for `Reg<USBHS_DEVEPTIDR_ISO_MODE_SPEC>`"]
pub type USBHS_DEVEPTIDR_ISO_MODE =
    crate::Reg<usbhs_deveptidr_iso_mode::USBHS_DEVEPTIDR_ISO_MODE_SPEC>;
#[doc = "Device Endpoint Interrupt Disable Register"]
pub mod usbhs_deveptidr_iso_mode;
#[doc = "USBHS_DEVEPTIDR_BLK_MODE register accessor: an alias for `Reg<USBHS_DEVEPTIDR_BLK_MODE_SPEC>`"]
pub type USBHS_DEVEPTIDR_BLK_MODE =
    crate::Reg<usbhs_deveptidr_blk_mode::USBHS_DEVEPTIDR_BLK_MODE_SPEC>;
#[doc = "Device Endpoint Interrupt Disable Register"]
pub mod usbhs_deveptidr_blk_mode;
#[doc = "USBHS_DEVEPTIDR_INTRPT_MODE register accessor: an alias for `Reg<USBHS_DEVEPTIDR_INTRPT_MODE_SPEC>`"]
pub type USBHS_DEVEPTIDR_INTRPT_MODE =
    crate::Reg<usbhs_deveptidr_intrpt_mode::USBHS_DEVEPTIDR_INTRPT_MODE_SPEC>;
#[doc = "Device Endpoint Interrupt Disable Register"]
pub mod usbhs_deveptidr_intrpt_mode;
#[doc = "USBHS_HSTCTRL register accessor: an alias for `Reg<USBHS_HSTCTRL_SPEC>`"]
pub type USBHS_HSTCTRL = crate::Reg<usbhs_hstctrl::USBHS_HSTCTRL_SPEC>;
#[doc = "Host General Control Register"]
pub mod usbhs_hstctrl;
#[doc = "USBHS_HSTISR register accessor: an alias for `Reg<USBHS_HSTISR_SPEC>`"]
pub type USBHS_HSTISR = crate::Reg<usbhs_hstisr::USBHS_HSTISR_SPEC>;
#[doc = "Host Global Interrupt Status Register"]
pub mod usbhs_hstisr;
#[doc = "USBHS_HSTICR register accessor: an alias for `Reg<USBHS_HSTICR_SPEC>`"]
pub type USBHS_HSTICR = crate::Reg<usbhs_hsticr::USBHS_HSTICR_SPEC>;
#[doc = "Host Global Interrupt Clear Register"]
pub mod usbhs_hsticr;
#[doc = "USBHS_HSTIFR register accessor: an alias for `Reg<USBHS_HSTIFR_SPEC>`"]
pub type USBHS_HSTIFR = crate::Reg<usbhs_hstifr::USBHS_HSTIFR_SPEC>;
#[doc = "Host Global Interrupt Set Register"]
pub mod usbhs_hstifr;
#[doc = "USBHS_HSTIMR register accessor: an alias for `Reg<USBHS_HSTIMR_SPEC>`"]
pub type USBHS_HSTIMR = crate::Reg<usbhs_hstimr::USBHS_HSTIMR_SPEC>;
#[doc = "Host Global Interrupt Mask Register"]
pub mod usbhs_hstimr;
#[doc = "USBHS_HSTIDR register accessor: an alias for `Reg<USBHS_HSTIDR_SPEC>`"]
pub type USBHS_HSTIDR = crate::Reg<usbhs_hstidr::USBHS_HSTIDR_SPEC>;
#[doc = "Host Global Interrupt Disable Register"]
pub mod usbhs_hstidr;
#[doc = "USBHS_HSTIER register accessor: an alias for `Reg<USBHS_HSTIER_SPEC>`"]
pub type USBHS_HSTIER = crate::Reg<usbhs_hstier::USBHS_HSTIER_SPEC>;
#[doc = "Host Global Interrupt Enable Register"]
pub mod usbhs_hstier;
#[doc = "USBHS_HSTPIP register accessor: an alias for `Reg<USBHS_HSTPIP_SPEC>`"]
pub type USBHS_HSTPIP = crate::Reg<usbhs_hstpip::USBHS_HSTPIP_SPEC>;
#[doc = "Host Pipe Register"]
pub mod usbhs_hstpip;
#[doc = "USBHS_HSTFNUM register accessor: an alias for `Reg<USBHS_HSTFNUM_SPEC>`"]
pub type USBHS_HSTFNUM = crate::Reg<usbhs_hstfnum::USBHS_HSTFNUM_SPEC>;
#[doc = "Host Frame Number Register"]
pub mod usbhs_hstfnum;
#[doc = "USBHS_HSTADDR1 register accessor: an alias for `Reg<USBHS_HSTADDR1_SPEC>`"]
pub type USBHS_HSTADDR1 = crate::Reg<usbhs_hstaddr1::USBHS_HSTADDR1_SPEC>;
#[doc = "Host Address 1 Register"]
pub mod usbhs_hstaddr1;
#[doc = "USBHS_HSTADDR2 register accessor: an alias for `Reg<USBHS_HSTADDR2_SPEC>`"]
pub type USBHS_HSTADDR2 = crate::Reg<usbhs_hstaddr2::USBHS_HSTADDR2_SPEC>;
#[doc = "Host Address 2 Register"]
pub mod usbhs_hstaddr2;
#[doc = "USBHS_HSTADDR3 register accessor: an alias for `Reg<USBHS_HSTADDR3_SPEC>`"]
pub type USBHS_HSTADDR3 = crate::Reg<usbhs_hstaddr3::USBHS_HSTADDR3_SPEC>;
#[doc = "Host Address 3 Register"]
pub mod usbhs_hstaddr3;
#[doc = "USBHS_HSTPIPCFG register accessor: an alias for `Reg<USBHS_HSTPIPCFG_SPEC>`"]
pub type USBHS_HSTPIPCFG = crate::Reg<usbhs_hstpipcfg::USBHS_HSTPIPCFG_SPEC>;
#[doc = "Host Pipe Configuration Register"]
pub mod usbhs_hstpipcfg;
#[doc = "USBHS_HSTPIPCFG_CTRL_BULK_MODE register accessor: an alias for `Reg<USBHS_HSTPIPCFG_CTRL_BULK_MODE_SPEC>`"]
pub type USBHS_HSTPIPCFG_CTRL_BULK_MODE =
    crate::Reg<usbhs_hstpipcfg_ctrl_bulk_mode::USBHS_HSTPIPCFG_CTRL_BULK_MODE_SPEC>;
#[doc = "Host Pipe Configuration Register"]
pub mod usbhs_hstpipcfg_ctrl_bulk_mode;
#[doc = "USBHS_HSTPIPISR_CTRL_MODE register accessor: an alias for `Reg<USBHS_HSTPIPISR_CTRL_MODE_SPEC>`"]
pub type USBHS_HSTPIPISR_CTRL_MODE =
    crate::Reg<usbhs_hstpipisr_ctrl_mode::USBHS_HSTPIPISR_CTRL_MODE_SPEC>;
#[doc = "Host Pipe Status Register"]
pub mod usbhs_hstpipisr_ctrl_mode;
#[doc = "USBHS_HSTPIPISR_ISO_MODE register accessor: an alias for `Reg<USBHS_HSTPIPISR_ISO_MODE_SPEC>`"]
pub type USBHS_HSTPIPISR_ISO_MODE =
    crate::Reg<usbhs_hstpipisr_iso_mode::USBHS_HSTPIPISR_ISO_MODE_SPEC>;
#[doc = "Host Pipe Status Register"]
pub mod usbhs_hstpipisr_iso_mode;
#[doc = "USBHS_HSTPIPISR_BLK_MODE register accessor: an alias for `Reg<USBHS_HSTPIPISR_BLK_MODE_SPEC>`"]
pub type USBHS_HSTPIPISR_BLK_MODE =
    crate::Reg<usbhs_hstpipisr_blk_mode::USBHS_HSTPIPISR_BLK_MODE_SPEC>;
#[doc = "Host Pipe Status Register"]
pub mod usbhs_hstpipisr_blk_mode;
#[doc = "USBHS_HSTPIPISR_INTRPT_MODE register accessor: an alias for `Reg<USBHS_HSTPIPISR_INTRPT_MODE_SPEC>`"]
pub type USBHS_HSTPIPISR_INTRPT_MODE =
    crate::Reg<usbhs_hstpipisr_intrpt_mode::USBHS_HSTPIPISR_INTRPT_MODE_SPEC>;
#[doc = "Host Pipe Status Register"]
pub mod usbhs_hstpipisr_intrpt_mode;
#[doc = "USBHS_HSTPIPICR_CTRL_MODE register accessor: an alias for `Reg<USBHS_HSTPIPICR_CTRL_MODE_SPEC>`"]
pub type USBHS_HSTPIPICR_CTRL_MODE =
    crate::Reg<usbhs_hstpipicr_ctrl_mode::USBHS_HSTPIPICR_CTRL_MODE_SPEC>;
#[doc = "Host Pipe Clear Register"]
pub mod usbhs_hstpipicr_ctrl_mode;
#[doc = "USBHS_HSTPIPICR_ISO_MODE register accessor: an alias for `Reg<USBHS_HSTPIPICR_ISO_MODE_SPEC>`"]
pub type USBHS_HSTPIPICR_ISO_MODE =
    crate::Reg<usbhs_hstpipicr_iso_mode::USBHS_HSTPIPICR_ISO_MODE_SPEC>;
#[doc = "Host Pipe Clear Register"]
pub mod usbhs_hstpipicr_iso_mode;
#[doc = "USBHS_HSTPIPICR_BLK_MODE register accessor: an alias for `Reg<USBHS_HSTPIPICR_BLK_MODE_SPEC>`"]
pub type USBHS_HSTPIPICR_BLK_MODE =
    crate::Reg<usbhs_hstpipicr_blk_mode::USBHS_HSTPIPICR_BLK_MODE_SPEC>;
#[doc = "Host Pipe Clear Register"]
pub mod usbhs_hstpipicr_blk_mode;
#[doc = "USBHS_HSTPIPICR_INTRPT_MODE register accessor: an alias for `Reg<USBHS_HSTPIPICR_INTRPT_MODE_SPEC>`"]
pub type USBHS_HSTPIPICR_INTRPT_MODE =
    crate::Reg<usbhs_hstpipicr_intrpt_mode::USBHS_HSTPIPICR_INTRPT_MODE_SPEC>;
#[doc = "Host Pipe Clear Register"]
pub mod usbhs_hstpipicr_intrpt_mode;
#[doc = "USBHS_HSTPIPIFR_CTRL_MODE register accessor: an alias for `Reg<USBHS_HSTPIPIFR_CTRL_MODE_SPEC>`"]
pub type USBHS_HSTPIPIFR_CTRL_MODE =
    crate::Reg<usbhs_hstpipifr_ctrl_mode::USBHS_HSTPIPIFR_CTRL_MODE_SPEC>;
#[doc = "Host Pipe Set Register"]
pub mod usbhs_hstpipifr_ctrl_mode;
#[doc = "USBHS_HSTPIPIFR_ISO_MODE register accessor: an alias for `Reg<USBHS_HSTPIPIFR_ISO_MODE_SPEC>`"]
pub type USBHS_HSTPIPIFR_ISO_MODE =
    crate::Reg<usbhs_hstpipifr_iso_mode::USBHS_HSTPIPIFR_ISO_MODE_SPEC>;
#[doc = "Host Pipe Set Register"]
pub mod usbhs_hstpipifr_iso_mode;
#[doc = "USBHS_HSTPIPIFR_BLK_MODE register accessor: an alias for `Reg<USBHS_HSTPIPIFR_BLK_MODE_SPEC>`"]
pub type USBHS_HSTPIPIFR_BLK_MODE =
    crate::Reg<usbhs_hstpipifr_blk_mode::USBHS_HSTPIPIFR_BLK_MODE_SPEC>;
#[doc = "Host Pipe Set Register"]
pub mod usbhs_hstpipifr_blk_mode;
#[doc = "USBHS_HSTPIPIFR_INTRPT_MODE register accessor: an alias for `Reg<USBHS_HSTPIPIFR_INTRPT_MODE_SPEC>`"]
pub type USBHS_HSTPIPIFR_INTRPT_MODE =
    crate::Reg<usbhs_hstpipifr_intrpt_mode::USBHS_HSTPIPIFR_INTRPT_MODE_SPEC>;
#[doc = "Host Pipe Set Register"]
pub mod usbhs_hstpipifr_intrpt_mode;
#[doc = "USBHS_HSTPIPIMR_CTRL_MODE register accessor: an alias for `Reg<USBHS_HSTPIPIMR_CTRL_MODE_SPEC>`"]
pub type USBHS_HSTPIPIMR_CTRL_MODE =
    crate::Reg<usbhs_hstpipimr_ctrl_mode::USBHS_HSTPIPIMR_CTRL_MODE_SPEC>;
#[doc = "Host Pipe Mask Register"]
pub mod usbhs_hstpipimr_ctrl_mode;
#[doc = "USBHS_HSTPIPIMR_ISO_MODE register accessor: an alias for `Reg<USBHS_HSTPIPIMR_ISO_MODE_SPEC>`"]
pub type USBHS_HSTPIPIMR_ISO_MODE =
    crate::Reg<usbhs_hstpipimr_iso_mode::USBHS_HSTPIPIMR_ISO_MODE_SPEC>;
#[doc = "Host Pipe Mask Register"]
pub mod usbhs_hstpipimr_iso_mode;
#[doc = "USBHS_HSTPIPIMR_BLK_MODE register accessor: an alias for `Reg<USBHS_HSTPIPIMR_BLK_MODE_SPEC>`"]
pub type USBHS_HSTPIPIMR_BLK_MODE =
    crate::Reg<usbhs_hstpipimr_blk_mode::USBHS_HSTPIPIMR_BLK_MODE_SPEC>;
#[doc = "Host Pipe Mask Register"]
pub mod usbhs_hstpipimr_blk_mode;
#[doc = "USBHS_HSTPIPIMR_INTRPT_MODE register accessor: an alias for `Reg<USBHS_HSTPIPIMR_INTRPT_MODE_SPEC>`"]
pub type USBHS_HSTPIPIMR_INTRPT_MODE =
    crate::Reg<usbhs_hstpipimr_intrpt_mode::USBHS_HSTPIPIMR_INTRPT_MODE_SPEC>;
#[doc = "Host Pipe Mask Register"]
pub mod usbhs_hstpipimr_intrpt_mode;
#[doc = "USBHS_HSTPIPIER_CTRL_MODE register accessor: an alias for `Reg<USBHS_HSTPIPIER_CTRL_MODE_SPEC>`"]
pub type USBHS_HSTPIPIER_CTRL_MODE =
    crate::Reg<usbhs_hstpipier_ctrl_mode::USBHS_HSTPIPIER_CTRL_MODE_SPEC>;
#[doc = "Host Pipe Enable Register"]
pub mod usbhs_hstpipier_ctrl_mode;
#[doc = "USBHS_HSTPIPIER_ISO_MODE register accessor: an alias for `Reg<USBHS_HSTPIPIER_ISO_MODE_SPEC>`"]
pub type USBHS_HSTPIPIER_ISO_MODE =
    crate::Reg<usbhs_hstpipier_iso_mode::USBHS_HSTPIPIER_ISO_MODE_SPEC>;
#[doc = "Host Pipe Enable Register"]
pub mod usbhs_hstpipier_iso_mode;
#[doc = "USBHS_HSTPIPIER_BLK_MODE register accessor: an alias for `Reg<USBHS_HSTPIPIER_BLK_MODE_SPEC>`"]
pub type USBHS_HSTPIPIER_BLK_MODE =
    crate::Reg<usbhs_hstpipier_blk_mode::USBHS_HSTPIPIER_BLK_MODE_SPEC>;
#[doc = "Host Pipe Enable Register"]
pub mod usbhs_hstpipier_blk_mode;
#[doc = "USBHS_HSTPIPIER_INTRPT_MODE register accessor: an alias for `Reg<USBHS_HSTPIPIER_INTRPT_MODE_SPEC>`"]
pub type USBHS_HSTPIPIER_INTRPT_MODE =
    crate::Reg<usbhs_hstpipier_intrpt_mode::USBHS_HSTPIPIER_INTRPT_MODE_SPEC>;
#[doc = "Host Pipe Enable Register"]
pub mod usbhs_hstpipier_intrpt_mode;
#[doc = "USBHS_HSTPIPIDR_CTRL_MODE register accessor: an alias for `Reg<USBHS_HSTPIPIDR_CTRL_MODE_SPEC>`"]
pub type USBHS_HSTPIPIDR_CTRL_MODE =
    crate::Reg<usbhs_hstpipidr_ctrl_mode::USBHS_HSTPIPIDR_CTRL_MODE_SPEC>;
#[doc = "Host Pipe Disable Register"]
pub mod usbhs_hstpipidr_ctrl_mode;
#[doc = "USBHS_HSTPIPIDR_ISO_MODE register accessor: an alias for `Reg<USBHS_HSTPIPIDR_ISO_MODE_SPEC>`"]
pub type USBHS_HSTPIPIDR_ISO_MODE =
    crate::Reg<usbhs_hstpipidr_iso_mode::USBHS_HSTPIPIDR_ISO_MODE_SPEC>;
#[doc = "Host Pipe Disable Register"]
pub mod usbhs_hstpipidr_iso_mode;
#[doc = "USBHS_HSTPIPIDR_BLK_MODE register accessor: an alias for `Reg<USBHS_HSTPIPIDR_BLK_MODE_SPEC>`"]
pub type USBHS_HSTPIPIDR_BLK_MODE =
    crate::Reg<usbhs_hstpipidr_blk_mode::USBHS_HSTPIPIDR_BLK_MODE_SPEC>;
#[doc = "Host Pipe Disable Register"]
pub mod usbhs_hstpipidr_blk_mode;
#[doc = "USBHS_HSTPIPIDR_INTRPT_MODE register accessor: an alias for `Reg<USBHS_HSTPIPIDR_INTRPT_MODE_SPEC>`"]
pub type USBHS_HSTPIPIDR_INTRPT_MODE =
    crate::Reg<usbhs_hstpipidr_intrpt_mode::USBHS_HSTPIPIDR_INTRPT_MODE_SPEC>;
#[doc = "Host Pipe Disable Register"]
pub mod usbhs_hstpipidr_intrpt_mode;
#[doc = "USBHS_HSTPIPINRQ register accessor: an alias for `Reg<USBHS_HSTPIPINRQ_SPEC>`"]
pub type USBHS_HSTPIPINRQ = crate::Reg<usbhs_hstpipinrq::USBHS_HSTPIPINRQ_SPEC>;
#[doc = "Host Pipe IN Request Register"]
pub mod usbhs_hstpipinrq;
#[doc = "USBHS_HSTPIPERR register accessor: an alias for `Reg<USBHS_HSTPIPERR_SPEC>`"]
pub type USBHS_HSTPIPERR = crate::Reg<usbhs_hstpiperr::USBHS_HSTPIPERR_SPEC>;
#[doc = "Host Pipe Error Register"]
pub mod usbhs_hstpiperr;
#[doc = "USBHS_CTRL register accessor: an alias for `Reg<USBHS_CTRL_SPEC>`"]
pub type USBHS_CTRL = crate::Reg<usbhs_ctrl::USBHS_CTRL_SPEC>;
#[doc = "General Control Register"]
pub mod usbhs_ctrl;
#[doc = "USBHS_SR register accessor: an alias for `Reg<USBHS_SR_SPEC>`"]
pub type USBHS_SR = crate::Reg<usbhs_sr::USBHS_SR_SPEC>;
#[doc = "General Status Register"]
pub mod usbhs_sr;
#[doc = "USBHS_SCR register accessor: an alias for `Reg<USBHS_SCR_SPEC>`"]
pub type USBHS_SCR = crate::Reg<usbhs_scr::USBHS_SCR_SPEC>;
#[doc = "General Status Clear Register"]
pub mod usbhs_scr;
#[doc = "USBHS_SFR register accessor: an alias for `Reg<USBHS_SFR_SPEC>`"]
pub type USBHS_SFR = crate::Reg<usbhs_sfr::USBHS_SFR_SPEC>;
#[doc = "General Status Set Register"]
pub mod usbhs_sfr;
