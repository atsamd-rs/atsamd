#[doc = "Register `FC1` reader"]
pub struct R(crate::R<FC1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FC1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FC1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FC1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FC1` writer"]
pub struct W(crate::W<FC1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FC1_SPEC>;
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
impl From<crate::W<FC1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FC1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OVF` reader - Frame Counter Overflow Value"]
pub type OVF_R = crate::FieldReader<u8, u8>;
#[doc = "Field `OVF` writer - Frame Counter Overflow Value"]
pub type OVF_W<'a, const O: u8> = crate::FieldWriter<'a, u8, FC1_SPEC, u8, u8, 5, O>;
#[doc = "Field `PB` reader - Prescaler Bypass"]
pub type PB_R = crate::BitReader<bool>;
#[doc = "Field `PB` writer - Prescaler Bypass"]
pub type PB_W<'a, const O: u8> = crate::BitWriter<'a, u8, FC1_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:4 - Frame Counter Overflow Value"]
    #[inline(always)]
    pub fn ovf(&self) -> OVF_R {
        OVF_R::new(self.bits & 0x1f)
    }
    #[doc = "Bit 7 - Prescaler Bypass"]
    #[inline(always)]
    pub fn pb(&self) -> PB_R {
        PB_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Frame Counter Overflow Value"]
    #[inline(always)]
    #[must_use]
    pub fn ovf(&mut self) -> OVF_W<0> {
        OVF_W::new(self)
    }
    #[doc = "Bit 7 - Prescaler Bypass"]
    #[inline(always)]
    #[must_use]
    pub fn pb(&mut self) -> PB_W<7> {
        PB_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Frame Counter 1 Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fc1](index.html) module"]
pub struct FC1_SPEC;
impl crate::RegisterSpec for FC1_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [fc1::R](R) reader structure"]
impl crate::Readable for FC1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fc1::W](W) writer structure"]
impl crate::Writable for FC1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FC1 to value 0"]
impl crate::Resettable for FC1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
