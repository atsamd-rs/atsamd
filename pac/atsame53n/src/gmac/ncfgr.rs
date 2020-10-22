#[doc = "Reader of register NCFGR"]
pub type R = crate::R<u32, super::NCFGR>;
#[doc = "Writer for register NCFGR"]
pub type W = crate::W<u32, super::NCFGR>;
#[doc = "Register NCFGR `reset()`'s with value 0x0008_0000"]
impl crate::ResetValue for super::NCFGR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0008_0000
    }
}
#[doc = "Reader of field `SPD`"]
pub type SPD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPD`"]
pub struct SPD_W<'a> {
    w: &'a mut W,
}
impl<'a> SPD_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `FD`"]
pub type FD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FD`"]
pub struct FD_W<'a> {
    w: &'a mut W,
}
impl<'a> FD_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `DNVLAN`"]
pub type DNVLAN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DNVLAN`"]
pub struct DNVLAN_W<'a> {
    w: &'a mut W,
}
impl<'a> DNVLAN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `JFRAME`"]
pub type JFRAME_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `JFRAME`"]
pub struct JFRAME_W<'a> {
    w: &'a mut W,
}
impl<'a> JFRAME_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `CAF`"]
pub type CAF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CAF`"]
pub struct CAF_W<'a> {
    w: &'a mut W,
}
impl<'a> CAF_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `NBC`"]
pub type NBC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `NBC`"]
pub struct NBC_W<'a> {
    w: &'a mut W,
}
impl<'a> NBC_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `MTIHEN`"]
pub type MTIHEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MTIHEN`"]
pub struct MTIHEN_W<'a> {
    w: &'a mut W,
}
impl<'a> MTIHEN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `UNIHEN`"]
pub type UNIHEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UNIHEN`"]
pub struct UNIHEN_W<'a> {
    w: &'a mut W,
}
impl<'a> UNIHEN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `MAXFS`"]
pub type MAXFS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MAXFS`"]
pub struct MAXFS_W<'a> {
    w: &'a mut W,
}
impl<'a> MAXFS_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `RTY`"]
pub type RTY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTY`"]
pub struct RTY_W<'a> {
    w: &'a mut W,
}
impl<'a> RTY_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `PEN`"]
pub type PEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PEN`"]
pub struct PEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PEN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Reader of field `RXBUFO`"]
pub type RXBUFO_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RXBUFO`"]
pub struct RXBUFO_W<'a> {
    w: &'a mut W,
}
impl<'a> RXBUFO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u32) & 0x03) << 14);
        self.w
    }
}
#[doc = "Reader of field `LFERD`"]
pub type LFERD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LFERD`"]
pub struct LFERD_W<'a> {
    w: &'a mut W,
}
impl<'a> LFERD_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `RFCS`"]
pub type RFCS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RFCS`"]
pub struct RFCS_W<'a> {
    w: &'a mut W,
}
impl<'a> RFCS_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Reader of field `CLK`"]
pub type CLK_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CLK`"]
pub struct CLK_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 18)) | (((value as u32) & 0x07) << 18);
        self.w
    }
}
#[doc = "Reader of field `DBW`"]
pub type DBW_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DBW`"]
pub struct DBW_W<'a> {
    w: &'a mut W,
}
impl<'a> DBW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 21)) | (((value as u32) & 0x03) << 21);
        self.w
    }
}
#[doc = "Reader of field `DCPF`"]
pub type DCPF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DCPF`"]
pub struct DCPF_W<'a> {
    w: &'a mut W,
}
impl<'a> DCPF_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = "Reader of field `RXCOEN`"]
pub type RXCOEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXCOEN`"]
pub struct RXCOEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RXCOEN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Reader of field `EFRHD`"]
pub type EFRHD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EFRHD`"]
pub struct EFRHD_W<'a> {
    w: &'a mut W,
}
impl<'a> EFRHD_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "Reader of field `IRXFCS`"]
pub type IRXFCS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IRXFCS`"]
pub struct IRXFCS_W<'a> {
    w: &'a mut W,
}
impl<'a> IRXFCS_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "Reader of field `IPGSEN`"]
pub type IPGSEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IPGSEN`"]
pub struct IPGSEN_W<'a> {
    w: &'a mut W,
}
impl<'a> IPGSEN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "Reader of field `RXBP`"]
pub type RXBP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXBP`"]
pub struct RXBP_W<'a> {
    w: &'a mut W,
}
impl<'a> RXBP_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
#[doc = "Reader of field `IRXER`"]
pub type IRXER_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IRXER`"]
pub struct IRXER_W<'a> {
    w: &'a mut W,
}
impl<'a> IRXER_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Speed"]
    #[inline(always)]
    pub fn spd(&self) -> SPD_R {
        SPD_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Full Duplex"]
    #[inline(always)]
    pub fn fd(&self) -> FD_R {
        FD_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Discard Non-VLAN FRAMES"]
    #[inline(always)]
    pub fn dnvlan(&self) -> DNVLAN_R {
        DNVLAN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Jumbo Frame Size"]
    #[inline(always)]
    pub fn jframe(&self) -> JFRAME_R {
        JFRAME_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Copy All Frames"]
    #[inline(always)]
    pub fn caf(&self) -> CAF_R {
        CAF_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - No Broadcast"]
    #[inline(always)]
    pub fn nbc(&self) -> NBC_R {
        NBC_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Multicast Hash Enable"]
    #[inline(always)]
    pub fn mtihen(&self) -> MTIHEN_R {
        MTIHEN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Unicast Hash Enable"]
    #[inline(always)]
    pub fn unihen(&self) -> UNIHEN_R {
        UNIHEN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - 1536 Maximum Frame Size"]
    #[inline(always)]
    pub fn maxfs(&self) -> MAXFS_R {
        MAXFS_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Retry Test"]
    #[inline(always)]
    pub fn rty(&self) -> RTY_R {
        RTY_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Pause Enable"]
    #[inline(always)]
    pub fn pen(&self) -> PEN_R {
        PEN_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bits 14:15 - Receive Buffer Offset"]
    #[inline(always)]
    pub fn rxbufo(&self) -> RXBUFO_R {
        RXBUFO_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bit 16 - Length Field Error Frame Discard"]
    #[inline(always)]
    pub fn lferd(&self) -> LFERD_R {
        LFERD_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Remove FCS"]
    #[inline(always)]
    pub fn rfcs(&self) -> RFCS_R {
        RFCS_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bits 18:20 - MDC CLock Division"]
    #[inline(always)]
    pub fn clk(&self) -> CLK_R {
        CLK_R::new(((self.bits >> 18) & 0x07) as u8)
    }
    #[doc = "Bits 21:22 - Data Bus Width"]
    #[inline(always)]
    pub fn dbw(&self) -> DBW_R {
        DBW_R::new(((self.bits >> 21) & 0x03) as u8)
    }
    #[doc = "Bit 23 - Disable Copy of Pause Frames"]
    #[inline(always)]
    pub fn dcpf(&self) -> DCPF_R {
        DCPF_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Receive Checksum Offload Enable"]
    #[inline(always)]
    pub fn rxcoen(&self) -> RXCOEN_R {
        RXCOEN_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Enable Frames Received in Half Duplex"]
    #[inline(always)]
    pub fn efrhd(&self) -> EFRHD_R {
        EFRHD_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Ignore RX FCS"]
    #[inline(always)]
    pub fn irxfcs(&self) -> IRXFCS_R {
        IRXFCS_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 28 - IP Stretch Enable"]
    #[inline(always)]
    pub fn ipgsen(&self) -> IPGSEN_R {
        IPGSEN_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Receive Bad Preamble"]
    #[inline(always)]
    pub fn rxbp(&self) -> RXBP_R {
        RXBP_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Ignore IPG GRXER"]
    #[inline(always)]
    pub fn irxer(&self) -> IRXER_R {
        IRXER_R::new(((self.bits >> 30) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Speed"]
    #[inline(always)]
    pub fn spd(&mut self) -> SPD_W {
        SPD_W { w: self }
    }
    #[doc = "Bit 1 - Full Duplex"]
    #[inline(always)]
    pub fn fd(&mut self) -> FD_W {
        FD_W { w: self }
    }
    #[doc = "Bit 2 - Discard Non-VLAN FRAMES"]
    #[inline(always)]
    pub fn dnvlan(&mut self) -> DNVLAN_W {
        DNVLAN_W { w: self }
    }
    #[doc = "Bit 3 - Jumbo Frame Size"]
    #[inline(always)]
    pub fn jframe(&mut self) -> JFRAME_W {
        JFRAME_W { w: self }
    }
    #[doc = "Bit 4 - Copy All Frames"]
    #[inline(always)]
    pub fn caf(&mut self) -> CAF_W {
        CAF_W { w: self }
    }
    #[doc = "Bit 5 - No Broadcast"]
    #[inline(always)]
    pub fn nbc(&mut self) -> NBC_W {
        NBC_W { w: self }
    }
    #[doc = "Bit 6 - Multicast Hash Enable"]
    #[inline(always)]
    pub fn mtihen(&mut self) -> MTIHEN_W {
        MTIHEN_W { w: self }
    }
    #[doc = "Bit 7 - Unicast Hash Enable"]
    #[inline(always)]
    pub fn unihen(&mut self) -> UNIHEN_W {
        UNIHEN_W { w: self }
    }
    #[doc = "Bit 8 - 1536 Maximum Frame Size"]
    #[inline(always)]
    pub fn maxfs(&mut self) -> MAXFS_W {
        MAXFS_W { w: self }
    }
    #[doc = "Bit 12 - Retry Test"]
    #[inline(always)]
    pub fn rty(&mut self) -> RTY_W {
        RTY_W { w: self }
    }
    #[doc = "Bit 13 - Pause Enable"]
    #[inline(always)]
    pub fn pen(&mut self) -> PEN_W {
        PEN_W { w: self }
    }
    #[doc = "Bits 14:15 - Receive Buffer Offset"]
    #[inline(always)]
    pub fn rxbufo(&mut self) -> RXBUFO_W {
        RXBUFO_W { w: self }
    }
    #[doc = "Bit 16 - Length Field Error Frame Discard"]
    #[inline(always)]
    pub fn lferd(&mut self) -> LFERD_W {
        LFERD_W { w: self }
    }
    #[doc = "Bit 17 - Remove FCS"]
    #[inline(always)]
    pub fn rfcs(&mut self) -> RFCS_W {
        RFCS_W { w: self }
    }
    #[doc = "Bits 18:20 - MDC CLock Division"]
    #[inline(always)]
    pub fn clk(&mut self) -> CLK_W {
        CLK_W { w: self }
    }
    #[doc = "Bits 21:22 - Data Bus Width"]
    #[inline(always)]
    pub fn dbw(&mut self) -> DBW_W {
        DBW_W { w: self }
    }
    #[doc = "Bit 23 - Disable Copy of Pause Frames"]
    #[inline(always)]
    pub fn dcpf(&mut self) -> DCPF_W {
        DCPF_W { w: self }
    }
    #[doc = "Bit 24 - Receive Checksum Offload Enable"]
    #[inline(always)]
    pub fn rxcoen(&mut self) -> RXCOEN_W {
        RXCOEN_W { w: self }
    }
    #[doc = "Bit 25 - Enable Frames Received in Half Duplex"]
    #[inline(always)]
    pub fn efrhd(&mut self) -> EFRHD_W {
        EFRHD_W { w: self }
    }
    #[doc = "Bit 26 - Ignore RX FCS"]
    #[inline(always)]
    pub fn irxfcs(&mut self) -> IRXFCS_W {
        IRXFCS_W { w: self }
    }
    #[doc = "Bit 28 - IP Stretch Enable"]
    #[inline(always)]
    pub fn ipgsen(&mut self) -> IPGSEN_W {
        IPGSEN_W { w: self }
    }
    #[doc = "Bit 29 - Receive Bad Preamble"]
    #[inline(always)]
    pub fn rxbp(&mut self) -> RXBP_W {
        RXBP_W { w: self }
    }
    #[doc = "Bit 30 - Ignore IPG GRXER"]
    #[inline(always)]
    pub fn irxer(&mut self) -> IRXER_W {
        IRXER_W { w: self }
    }
}
