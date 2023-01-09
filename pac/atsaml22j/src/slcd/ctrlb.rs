#[doc = "Register `CTRLB` reader"]
pub struct R(crate::R<CTRLB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRLB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRLB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRLB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRLB` writer"]
pub struct W(crate::W<CTRLB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRLB_SPEC>;
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
impl From<crate::W<CTRLB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRLB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BBD` reader - Bias Buffer Enable Duration"]
pub type BBD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BBD` writer - Bias Buffer Enable Duration"]
pub type BBD_W<'a, const O: u8> = crate::FieldWriter<'a, u16, CTRLB_SPEC, u8, u8, 4, O>;
#[doc = "Field `BBEN` reader - Bias Buffer Enable"]
pub type BBEN_R = crate::BitReader<bool>;
#[doc = "Field `BBEN` writer - Bias Buffer Enable"]
pub type BBEN_W<'a, const O: u8> = crate::BitWriter<'a, u16, CTRLB_SPEC, bool, O>;
#[doc = "Field `LRD` reader - Low Resistance Enable Duration"]
pub type LRD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LRD` writer - Low Resistance Enable Duration"]
pub type LRD_W<'a, const O: u8> = crate::FieldWriter<'a, u16, CTRLB_SPEC, u8, u8, 4, O>;
#[doc = "Field `LREN` reader - Low Resistance Enable"]
pub type LREN_R = crate::BitReader<bool>;
#[doc = "Field `LREN` writer - Low Resistance Enable"]
pub type LREN_W<'a, const O: u8> = crate::BitWriter<'a, u16, CTRLB_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:3 - Bias Buffer Enable Duration"]
    #[inline(always)]
    pub fn bbd(&self) -> BBD_R {
        BBD_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 7 - Bias Buffer Enable"]
    #[inline(always)]
    pub fn bben(&self) -> BBEN_R {
        BBEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Low Resistance Enable Duration"]
    #[inline(always)]
    pub fn lrd(&self) -> LRD_R {
        LRD_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 15 - Low Resistance Enable"]
    #[inline(always)]
    pub fn lren(&self) -> LREN_R {
        LREN_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Bias Buffer Enable Duration"]
    #[inline(always)]
    #[must_use]
    pub fn bbd(&mut self) -> BBD_W<0> {
        BBD_W::new(self)
    }
    #[doc = "Bit 7 - Bias Buffer Enable"]
    #[inline(always)]
    #[must_use]
    pub fn bben(&mut self) -> BBEN_W<7> {
        BBEN_W::new(self)
    }
    #[doc = "Bits 8:11 - Low Resistance Enable Duration"]
    #[inline(always)]
    #[must_use]
    pub fn lrd(&mut self) -> LRD_W<8> {
        LRD_W::new(self)
    }
    #[doc = "Bit 15 - Low Resistance Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lren(&mut self) -> LREN_W<15> {
        LREN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control B\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrlb](index.html) module"]
pub struct CTRLB_SPEC;
impl crate::RegisterSpec for CTRLB_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [ctrlb::R](R) reader structure"]
impl crate::Readable for CTRLB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrlb::W](W) writer structure"]
impl crate::Writable for CTRLB_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRLB to value 0"]
impl crate::Resettable for CTRLB_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
