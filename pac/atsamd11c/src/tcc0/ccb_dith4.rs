#[doc = "Register `CCB%s_DITH4` reader"]
pub struct R(crate::R<CCB_DITH4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCB_DITH4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CCB_DITH4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CCB_DITH4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CCB%s_DITH4` writer"]
pub struct W(crate::W<CCB_DITH4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CCB_DITH4_SPEC>;
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
impl From<crate::W<CCB_DITH4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CCB_DITH4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DITHERCYB` reader - Dithering Buffer Cycle Number"]
pub type DITHERCYB_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DITHERCYB` writer - Dithering Buffer Cycle Number"]
pub type DITHERCYB_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCB_DITH4_SPEC, u8, u8, 4, O>;
#[doc = "Field `CCB` reader - Channel Compare/Capture Buffer Value"]
pub type CCB_R = crate::FieldReader<u32, u32>;
#[doc = "Field `CCB` writer - Channel Compare/Capture Buffer Value"]
pub type CCB_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCB_DITH4_SPEC, u32, u32, 20, O>;
impl R {
    #[doc = "Bits 0:3 - Dithering Buffer Cycle Number"]
    #[inline(always)]
    pub fn dithercyb(&self) -> DITHERCYB_R {
        DITHERCYB_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:23 - Channel Compare/Capture Buffer Value"]
    #[inline(always)]
    pub fn ccb(&self) -> CCB_R {
        CCB_R::new((self.bits >> 4) & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:3 - Dithering Buffer Cycle Number"]
    #[inline(always)]
    #[must_use]
    pub fn dithercyb(&mut self) -> DITHERCYB_W<0> {
        DITHERCYB_W::new(self)
    }
    #[doc = "Bits 4:23 - Channel Compare/Capture Buffer Value"]
    #[inline(always)]
    #[must_use]
    pub fn ccb(&mut self) -> CCB_W<4> {
        CCB_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Compare and Capture Buffer\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccb_dith4](index.html) module"]
pub struct CCB_DITH4_SPEC;
impl crate::RegisterSpec for CCB_DITH4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ccb_dith4::R](R) reader structure"]
impl crate::Readable for CCB_DITH4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ccb_dith4::W](W) writer structure"]
impl crate::Writable for CCB_DITH4_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CCB%s_DITH4 to value 0"]
impl crate::Resettable for CCB_DITH4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
