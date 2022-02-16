#[doc = "Register `GMAC_TIDM2` reader"]
pub struct R(crate::R<GMAC_TIDM2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GMAC_TIDM2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GMAC_TIDM2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GMAC_TIDM2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GMAC_TIDM2` writer"]
pub struct W(crate::W<GMAC_TIDM2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GMAC_TIDM2_SPEC>;
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
impl From<crate::W<GMAC_TIDM2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GMAC_TIDM2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TID` reader - Type ID Match 2"]
pub struct TID_R(crate::FieldReader<u16, u16>);
impl TID_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        TID_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TID_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TID` writer - Type ID Match 2"]
pub struct TID_W<'a> {
    w: &'a mut W,
}
impl<'a> TID_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
#[doc = "Field `ENID2` reader - Enable Copying of TID Matched Frames"]
pub struct ENID2_R(crate::FieldReader<bool, bool>);
impl ENID2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ENID2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENID2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENID2` writer - Enable Copying of TID Matched Frames"]
pub struct ENID2_W<'a> {
    w: &'a mut W,
}
impl<'a> ENID2_W<'a> {
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
    #[doc = "Bits 0:15 - Type ID Match 2"]
    #[inline(always)]
    pub fn tid(&self) -> TID_R {
        TID_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 31 - Enable Copying of TID Matched Frames"]
    #[inline(always)]
    pub fn enid2(&self) -> ENID2_R {
        ENID2_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - Type ID Match 2"]
    #[inline(always)]
    pub fn tid(&mut self) -> TID_W {
        TID_W { w: self }
    }
    #[doc = "Bit 31 - Enable Copying of TID Matched Frames"]
    #[inline(always)]
    pub fn enid2(&mut self) -> ENID2_W {
        ENID2_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Type ID Match 2 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gmac_tidm2](index.html) module"]
pub struct GMAC_TIDM2_SPEC;
impl crate::RegisterSpec for GMAC_TIDM2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gmac_tidm2::R](R) reader structure"]
impl crate::Readable for GMAC_TIDM2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gmac_tidm2::W](W) writer structure"]
impl crate::Writable for GMAC_TIDM2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GMAC_TIDM2 to value 0"]
impl crate::Resettable for GMAC_TIDM2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
