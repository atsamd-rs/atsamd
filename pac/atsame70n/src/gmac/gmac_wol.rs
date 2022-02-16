#[doc = "Register `GMAC_WOL` reader"]
pub struct R(crate::R<GMAC_WOL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GMAC_WOL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GMAC_WOL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GMAC_WOL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GMAC_WOL` writer"]
pub struct W(crate::W<GMAC_WOL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GMAC_WOL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<GMAC_WOL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GMAC_WOL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IP` reader - ARP Request IP Address"]
pub struct IP_R(crate::FieldReader<u16, u16>);
impl IP_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        IP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IP_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IP` writer - ARP Request IP Address"]
pub struct IP_W<'a> {
    w: &'a mut W,
}
impl<'a> IP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
#[doc = "Field `MAG` reader - Magic Packet Event Enable"]
pub struct MAG_R(crate::FieldReader<bool, bool>);
impl MAG_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MAG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MAG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MAG` writer - Magic Packet Event Enable"]
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Field `ARP` reader - ARP Request IP Address"]
pub struct ARP_R(crate::FieldReader<bool, bool>);
impl ARP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ARP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ARP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ARP` writer - ARP Request IP Address"]
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "Field `SA1` reader - Specific Address Register 1 Event Enable"]
pub struct SA1_R(crate::FieldReader<bool, bool>);
impl SA1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SA1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SA1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SA1` writer - Specific Address Register 1 Event Enable"]
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
#[doc = "Field `MTI` reader - Multicast Hash Event Enable"]
pub struct MTI_R(crate::FieldReader<bool, bool>);
impl MTI_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MTI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MTI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MTI` writer - Multicast Hash Event Enable"]
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - ARP Request IP Address"]
    #[inline(always)]
    pub fn ip(&self) -> IP_R {
        IP_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - Magic Packet Event Enable"]
    #[inline(always)]
    pub fn mag(&self) -> MAG_R {
        MAG_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - ARP Request IP Address"]
    #[inline(always)]
    pub fn arp(&self) -> ARP_R {
        ARP_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Specific Address Register 1 Event Enable"]
    #[inline(always)]
    pub fn sa1(&self) -> SA1_R {
        SA1_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Multicast Hash Event Enable"]
    #[inline(always)]
    pub fn mti(&self) -> MTI_R {
        MTI_R::new(((self.bits >> 19) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - ARP Request IP Address"]
    #[inline(always)]
    pub fn ip(&mut self) -> IP_W {
        IP_W { w: self }
    }
    #[doc = "Bit 16 - Magic Packet Event Enable"]
    #[inline(always)]
    pub fn mag(&mut self) -> MAG_W {
        MAG_W { w: self }
    }
    #[doc = "Bit 17 - ARP Request IP Address"]
    #[inline(always)]
    pub fn arp(&mut self) -> ARP_W {
        ARP_W { w: self }
    }
    #[doc = "Bit 18 - Specific Address Register 1 Event Enable"]
    #[inline(always)]
    pub fn sa1(&mut self) -> SA1_W {
        SA1_W { w: self }
    }
    #[doc = "Bit 19 - Multicast Hash Event Enable"]
    #[inline(always)]
    pub fn mti(&mut self) -> MTI_W {
        MTI_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Wake on LAN Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gmac_wol](index.html) module"]
pub struct GMAC_WOL_SPEC;
impl crate::RegisterSpec for GMAC_WOL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gmac_wol::R](R) reader structure"]
impl crate::Readable for GMAC_WOL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gmac_wol::W](W) writer structure"]
impl crate::Writable for GMAC_WOL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GMAC_WOL to value 0"]
impl crate::Resettable for GMAC_WOL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
