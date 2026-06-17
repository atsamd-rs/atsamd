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
pub type LDR_R = crate::FieldReader<u16, u16>;
#[doc = "Field `LDR` writer - Loop Divider Ratio"]
pub type LDR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DPLLRATIO_SPEC, u16, u16, 12, O>;
#[doc = "Field `LDRFRAC` reader - Loop Divider Ratio Fractional Part"]
pub type LDRFRAC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LDRFRAC` writer - Loop Divider Ratio Fractional Part"]
pub type LDRFRAC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DPLLRATIO_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:11 - Loop Divider Ratio"]
    #[inline(always)]
    pub fn ldr(&self) -> LDR_R {
        LDR_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:19 - Loop Divider Ratio Fractional Part"]
    #[inline(always)]
    pub fn ldrfrac(&self) -> LDRFRAC_R {
        LDRFRAC_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:11 - Loop Divider Ratio"]
    #[inline(always)]
    #[must_use]
    pub fn ldr(&mut self) -> LDR_W<0> {
        LDR_W::new(self)
    }
    #[doc = "Bits 16:19 - Loop Divider Ratio Fractional Part"]
    #[inline(always)]
    #[must_use]
    pub fn ldrfrac(&mut self) -> LDRFRAC_W<16> {
        LDRFRAC_W::new(self)
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DPLLRATIO to value 0"]
impl crate::Resettable for DPLLRATIO_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
