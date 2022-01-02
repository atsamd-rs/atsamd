#[doc = "Register `DPLLRATIO` reader"]
pub struct R(crate::R<DPLLRATIO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DPLLRATIO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DPLLRATIO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DPLLRATIO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DPLLRATIO` writer"]
pub struct W(crate::W<DPLLRATIO_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DPLLRATIO_SPEC>;
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
impl From<crate::W<DPLLRATIO_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DPLLRATIO_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LDR` reader - Loop Divider Ratio"]
pub struct LDR_R(crate::FieldReader<u16, u16>);
impl LDR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        LDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LDR_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LDR` writer - Loop Divider Ratio"]
pub struct LDR_W<'a> {
    w: &'a mut W,
}
impl<'a> LDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1fff) | (value as u32 & 0x1fff);
        self.w
    }
}
#[doc = "Field `LDRFRAC` reader - Loop Divider Ratio Fractional Part"]
pub struct LDRFRAC_R(crate::FieldReader<u8, u8>);
impl LDRFRAC_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        LDRFRAC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LDRFRAC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LDRFRAC` writer - Loop Divider Ratio Fractional Part"]
pub struct LDRFRAC_W<'a> {
    w: &'a mut W,
}
impl<'a> LDRFRAC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 16)) | ((value as u32 & 0x1f) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:12 - Loop Divider Ratio"]
    #[inline(always)]
    pub fn ldr(&self) -> LDR_R {
        LDR_R::new((self.bits & 0x1fff) as u16)
    }
    #[doc = "Bits 16:20 - Loop Divider Ratio Fractional Part"]
    #[inline(always)]
    pub fn ldrfrac(&self) -> LDRFRAC_R {
        LDRFRAC_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:12 - Loop Divider Ratio"]
    #[inline(always)]
    pub fn ldr(&mut self) -> LDR_W {
        LDR_W { w: self }
    }
    #[doc = "Bits 16:20 - Loop Divider Ratio Fractional Part"]
    #[inline(always)]
    pub fn ldrfrac(&mut self) -> LDRFRAC_W {
        LDRFRAC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DPLL Ratio Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dpllratio](index.html) module"]
pub struct DPLLRATIO_SPEC;
impl crate::RegisterSpec for DPLLRATIO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dpllratio::R](R) reader structure"]
impl crate::Readable for DPLLRATIO_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dpllratio::W](W) writer structure"]
impl crate::Writable for DPLLRATIO_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DPLLRATIO to value 0"]
impl crate::Resettable for DPLLRATIO_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
