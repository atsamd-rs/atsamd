#[doc = "Register `PER_DITH4` reader"]
pub struct R(crate::R<PER_DITH4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PER_DITH4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PER_DITH4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PER_DITH4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PER_DITH4` writer"]
pub struct W(crate::W<PER_DITH4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PER_DITH4_SPEC>;
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
impl From<crate::W<PER_DITH4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PER_DITH4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DITHERCY` reader - Dithering Cycle Number"]
pub type DITHERCY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DITHERCY` writer - Dithering Cycle Number"]
pub type DITHERCY_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PER_DITH4_SPEC, u8, u8, 4, O>;
#[doc = "Field `PER` reader - Period Value"]
pub type PER_R = crate::FieldReader<u32, u32>;
#[doc = "Field `PER` writer - Period Value"]
pub type PER_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PER_DITH4_SPEC, u32, u32, 20, O>;
impl R {
    #[doc = "Bits 0:3 - Dithering Cycle Number"]
    #[inline(always)]
    pub fn dithercy(&self) -> DITHERCY_R {
        DITHERCY_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:23 - Period Value"]
    #[inline(always)]
    pub fn per(&self) -> PER_R {
        PER_R::new((self.bits >> 4) & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:3 - Dithering Cycle Number"]
    #[inline(always)]
    #[must_use]
    pub fn dithercy(&mut self) -> DITHERCY_W<0> {
        DITHERCY_W::new(self)
    }
    #[doc = "Bits 4:23 - Period Value"]
    #[inline(always)]
    #[must_use]
    pub fn per(&mut self) -> PER_W<4> {
        PER_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Period\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [per_dith4](index.html) module"]
pub struct PER_DITH4_SPEC;
impl crate::RegisterSpec for PER_DITH4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [per_dith4::R](R) reader structure"]
impl crate::Readable for PER_DITH4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [per_dith4::W](W) writer structure"]
impl crate::Writable for PER_DITH4_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PER_DITH4 to value 0xffff_ffff"]
impl crate::Resettable for PER_DITH4_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
