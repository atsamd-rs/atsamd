#[doc = "Reader of register WOL"]
pub type R = crate::R<u32, super::WOL>;
#[doc = "Writer for register WOL"]
pub type W = crate::W<u32, super::WOL>;
#[doc = "Register WOL `reset()`'s with value 0"]
impl crate::ResetValue for super::WOL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `IP`"]
pub type IP_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `IP`"]
pub struct IP_W<'a> {
    w: &'a mut W,
}
impl<'a> IP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
#[doc = "Reader of field `MAG`"]
pub type MAG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MAG`"]
pub struct MAG_W<'a> {
    w: &'a mut W,
}
impl<'a> MAG_W<'a> {
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
#[doc = "Reader of field `ARP`"]
pub type ARP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ARP`"]
pub struct ARP_W<'a> {
    w: &'a mut W,
}
impl<'a> ARP_W<'a> {
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
#[doc = "Reader of field `SA1`"]
pub type SA1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SA1`"]
pub struct SA1_W<'a> {
    w: &'a mut W,
}
impl<'a> SA1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Reader of field `MTI`"]
pub type MTI_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MTI`"]
pub struct MTI_W<'a> {
    w: &'a mut W,
}
impl<'a> MTI_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - IP address"]
    #[inline(always)]
    pub fn ip(&self) -> IP_R {
        IP_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - Event enable"]
    #[inline(always)]
    pub fn mag(&self) -> MAG_R {
        MAG_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - LAN ARP req"]
    #[inline(always)]
    pub fn arp(&self) -> ARP_R {
        ARP_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - WOL specific address reg 1"]
    #[inline(always)]
    pub fn sa1(&self) -> SA1_R {
        SA1_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - WOL LAN multicast"]
    #[inline(always)]
    pub fn mti(&self) -> MTI_R {
        MTI_R::new(((self.bits >> 19) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - IP address"]
    #[inline(always)]
    pub fn ip(&mut self) -> IP_W {
        IP_W { w: self }
    }
    #[doc = "Bit 16 - Event enable"]
    #[inline(always)]
    pub fn mag(&mut self) -> MAG_W {
        MAG_W { w: self }
    }
    #[doc = "Bit 17 - LAN ARP req"]
    #[inline(always)]
    pub fn arp(&mut self) -> ARP_W {
        ARP_W { w: self }
    }
    #[doc = "Bit 18 - WOL specific address reg 1"]
    #[inline(always)]
    pub fn sa1(&mut self) -> SA1_W {
        SA1_W { w: self }
    }
    #[doc = "Bit 19 - WOL LAN multicast"]
    #[inline(always)]
    pub fn mti(&mut self) -> MTI_W {
        MTI_W { w: self }
    }
}
