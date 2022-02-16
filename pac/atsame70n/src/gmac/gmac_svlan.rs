#[doc = "Register `GMAC_SVLAN` reader"]
pub struct R(crate::R<GMAC_SVLAN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GMAC_SVLAN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GMAC_SVLAN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GMAC_SVLAN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GMAC_SVLAN` writer"]
pub struct W(crate::W<GMAC_SVLAN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GMAC_SVLAN_SPEC>;
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
impl From<crate::W<GMAC_SVLAN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GMAC_SVLAN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VLAN_TYPE` reader - User Defined VLAN_TYPE Field"]
pub struct VLAN_TYPE_R(crate::FieldReader<u16, u16>);
impl VLAN_TYPE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        VLAN_TYPE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VLAN_TYPE_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VLAN_TYPE` writer - User Defined VLAN_TYPE Field"]
pub struct VLAN_TYPE_W<'a> {
    w: &'a mut W,
}
impl<'a> VLAN_TYPE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
#[doc = "Field `ESVLAN` reader - Enable Stacked VLAN Processing Mode"]
pub struct ESVLAN_R(crate::FieldReader<bool, bool>);
impl ESVLAN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ESVLAN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ESVLAN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ESVLAN` writer - Enable Stacked VLAN Processing Mode"]
pub struct ESVLAN_W<'a> {
    w: &'a mut W,
}
impl<'a> ESVLAN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - User Defined VLAN_TYPE Field"]
    #[inline(always)]
    pub fn vlan_type(&self) -> VLAN_TYPE_R {
        VLAN_TYPE_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 31 - Enable Stacked VLAN Processing Mode"]
    #[inline(always)]
    pub fn esvlan(&self) -> ESVLAN_R {
        ESVLAN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - User Defined VLAN_TYPE Field"]
    #[inline(always)]
    pub fn vlan_type(&mut self) -> VLAN_TYPE_W {
        VLAN_TYPE_W { w: self }
    }
    #[doc = "Bit 31 - Enable Stacked VLAN Processing Mode"]
    #[inline(always)]
    pub fn esvlan(&mut self) -> ESVLAN_W {
        ESVLAN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Stacked VLAN Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gmac_svlan](index.html) module"]
pub struct GMAC_SVLAN_SPEC;
impl crate::RegisterSpec for GMAC_SVLAN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gmac_svlan::R](R) reader structure"]
impl crate::Readable for GMAC_SVLAN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gmac_svlan::W](W) writer structure"]
impl crate::Writable for GMAC_SVLAN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GMAC_SVLAN to value 0"]
impl crate::Resettable for GMAC_SVLAN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
