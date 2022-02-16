#[doc = "Register `GMAC_ST2CW0` reader"]
pub struct R(crate::R<GMAC_ST2CW0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GMAC_ST2CW0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GMAC_ST2CW0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GMAC_ST2CW0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GMAC_ST2CW0` writer"]
pub struct W(crate::W<GMAC_ST2CW0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GMAC_ST2CW0_SPEC>;
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
impl From<crate::W<GMAC_ST2CW0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GMAC_ST2CW0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MASKVAL` reader - Mask Value"]
pub struct MASKVAL_R(crate::FieldReader<u16, u16>);
impl MASKVAL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        MASKVAL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MASKVAL_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MASKVAL` writer - Mask Value"]
pub struct MASKVAL_W<'a> {
    w: &'a mut W,
}
impl<'a> MASKVAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
#[doc = "Field `COMPVAL` reader - Compare Value"]
pub struct COMPVAL_R(crate::FieldReader<u16, u16>);
impl COMPVAL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        COMPVAL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COMPVAL_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMPVAL` writer - Compare Value"]
pub struct COMPVAL_W<'a> {
    w: &'a mut W,
}
impl<'a> COMPVAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | ((value as u32 & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Mask Value"]
    #[inline(always)]
    pub fn maskval(&self) -> MASKVAL_R {
        MASKVAL_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Compare Value"]
    #[inline(always)]
    pub fn compval(&self) -> COMPVAL_R {
        COMPVAL_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Mask Value"]
    #[inline(always)]
    pub fn maskval(&mut self) -> MASKVAL_W {
        MASKVAL_W { w: self }
    }
    #[doc = "Bits 16:31 - Compare Value"]
    #[inline(always)]
    pub fn compval(&mut self) -> COMPVAL_W {
        COMPVAL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Screening Type 2 Compare Word 0 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gmac_st2cw0](index.html) module"]
pub struct GMAC_ST2CW0_SPEC;
impl crate::RegisterSpec for GMAC_ST2CW0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gmac_st2cw0::R](R) reader structure"]
impl crate::Readable for GMAC_ST2CW0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gmac_st2cw0::W](W) writer structure"]
impl crate::Writable for GMAC_ST2CW0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GMAC_ST2CW0 to value 0"]
impl crate::Resettable for GMAC_ST2CW0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
