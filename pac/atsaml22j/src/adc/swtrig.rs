#[doc = "Register `SWTRIG` reader"]
pub struct R(crate::R<SWTRIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SWTRIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SWTRIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SWTRIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SWTRIG` writer"]
pub struct W(crate::W<SWTRIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SWTRIG_SPEC>;
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
impl From<crate::W<SWTRIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SWTRIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FLUSH` reader - ADC Flush"]
pub type FLUSH_R = crate::BitReader<bool>;
#[doc = "Field `FLUSH` writer - ADC Flush"]
pub type FLUSH_W<'a, const O: u8> = crate::BitWriter<'a, u8, SWTRIG_SPEC, bool, O>;
#[doc = "Field `START` reader - Start ADC Conversion"]
pub type START_R = crate::BitReader<bool>;
#[doc = "Field `START` writer - Start ADC Conversion"]
pub type START_W<'a, const O: u8> = crate::BitWriter<'a, u8, SWTRIG_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - ADC Flush"]
    #[inline(always)]
    pub fn flush(&self) -> FLUSH_R {
        FLUSH_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Start ADC Conversion"]
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ADC Flush"]
    #[inline(always)]
    #[must_use]
    pub fn flush(&mut self) -> FLUSH_W<0> {
        FLUSH_W::new(self)
    }
    #[doc = "Bit 1 - Start ADC Conversion"]
    #[inline(always)]
    #[must_use]
    pub fn start(&mut self) -> START_W<1> {
        START_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Software Trigger\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [swtrig](index.html) module"]
pub struct SWTRIG_SPEC;
impl crate::RegisterSpec for SWTRIG_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [swtrig::R](R) reader structure"]
impl crate::Readable for SWTRIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [swtrig::W](W) writer structure"]
impl crate::Writable for SWTRIG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SWTRIG to value 0"]
impl crate::Resettable for SWTRIG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
