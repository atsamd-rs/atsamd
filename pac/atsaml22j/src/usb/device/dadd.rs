#[doc = "Register `DADD` reader"]
pub struct R(crate::R<DADD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DADD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DADD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DADD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DADD` writer"]
pub struct W(crate::W<DADD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DADD_SPEC>;
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
impl From<crate::W<DADD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DADD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DADD` reader - Device Address"]
pub type DADD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DADD` writer - Device Address"]
pub type DADD_W<'a, const O: u8> = crate::FieldWriter<'a, u8, DADD_SPEC, u8, u8, 7, O>;
#[doc = "Field `ADDEN` reader - Device Address Enable"]
pub type ADDEN_R = crate::BitReader<bool>;
#[doc = "Field `ADDEN` writer - Device Address Enable"]
pub type ADDEN_W<'a, const O: u8> = crate::BitWriter<'a, u8, DADD_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:6 - Device Address"]
    #[inline(always)]
    pub fn dadd(&self) -> DADD_R {
        DADD_R::new(self.bits & 0x7f)
    }
    #[doc = "Bit 7 - Device Address Enable"]
    #[inline(always)]
    pub fn adden(&self) -> ADDEN_R {
        ADDEN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - Device Address"]
    #[inline(always)]
    #[must_use]
    pub fn dadd(&mut self) -> DADD_W<0> {
        DADD_W::new(self)
    }
    #[doc = "Bit 7 - Device Address Enable"]
    #[inline(always)]
    #[must_use]
    pub fn adden(&mut self) -> ADDEN_W<7> {
        ADDEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DEVICE Device Address\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dadd](index.html) module"]
pub struct DADD_SPEC;
impl crate::RegisterSpec for DADD_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [dadd::R](R) reader structure"]
impl crate::Readable for DADD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dadd::W](W) writer structure"]
impl crate::Writable for DADD_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DADD to value 0"]
impl crate::Resettable for DADD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
