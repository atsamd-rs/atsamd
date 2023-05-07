#[doc = "Register `HSOFC` reader"]
pub struct R(crate::R<HSOFC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HSOFC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HSOFC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HSOFC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HSOFC` writer"]
pub struct W(crate::W<HSOFC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HSOFC_SPEC>;
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
impl From<crate::W<HSOFC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HSOFC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FLENC` reader - Frame Length Control"]
pub type FLENC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FLENC` writer - Frame Length Control"]
pub type FLENC_W<'a, const O: u8> = crate::FieldWriter<'a, u8, HSOFC_SPEC, u8, u8, 4, O>;
#[doc = "Field `FLENCE` reader - Frame Length Control Enable"]
pub type FLENCE_R = crate::BitReader<bool>;
#[doc = "Field `FLENCE` writer - Frame Length Control Enable"]
pub type FLENCE_W<'a, const O: u8> = crate::BitWriter<'a, u8, HSOFC_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:3 - Frame Length Control"]
    #[inline(always)]
    pub fn flenc(&self) -> FLENC_R {
        FLENC_R::new(self.bits & 0x0f)
    }
    #[doc = "Bit 7 - Frame Length Control Enable"]
    #[inline(always)]
    pub fn flence(&self) -> FLENCE_R {
        FLENCE_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Frame Length Control"]
    #[inline(always)]
    #[must_use]
    pub fn flenc(&mut self) -> FLENC_W<0> {
        FLENC_W::new(self)
    }
    #[doc = "Bit 7 - Frame Length Control Enable"]
    #[inline(always)]
    #[must_use]
    pub fn flence(&mut self) -> FLENCE_W<7> {
        FLENCE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "HOST Host Start Of Frame Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hsofc](index.html) module"]
pub struct HSOFC_SPEC;
impl crate::RegisterSpec for HSOFC_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [hsofc::R](R) reader structure"]
impl crate::Readable for HSOFC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hsofc::W](W) writer structure"]
impl crate::Writable for HSOFC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HSOFC to value 0"]
impl crate::Resettable for HSOFC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
