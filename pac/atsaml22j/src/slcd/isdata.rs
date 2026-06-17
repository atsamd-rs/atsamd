#[doc = "Register `ISDATA` writer"]
pub struct W(crate::W<ISDATA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ISDATA_SPEC>;
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
impl From<crate::W<ISDATA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ISDATA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SDATA` writer - Segments Data"]
pub type SDATA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ISDATA_SPEC, u8, u8, 8, O>;
#[doc = "Field `SDMASK` writer - Segments Data Mask"]
pub type SDMASK_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ISDATA_SPEC, u8, u8, 8, O>;
#[doc = "Field `OFF` writer - Byte Offset"]
pub type OFF_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ISDATA_SPEC, u8, u8, 6, O>;
impl W {
    #[doc = "Bits 0:7 - Segments Data"]
    #[inline(always)]
    #[must_use]
    pub fn sdata(&mut self) -> SDATA_W<0> {
        SDATA_W::new(self)
    }
    #[doc = "Bits 8:15 - Segments Data Mask"]
    #[inline(always)]
    #[must_use]
    pub fn sdmask(&mut self) -> SDMASK_W<8> {
        SDMASK_W::new(self)
    }
    #[doc = "Bits 16:21 - Byte Offset"]
    #[inline(always)]
    #[must_use]
    pub fn off(&mut self) -> OFF_W<16> {
        OFF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Indirect Segments Data Access\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [isdata](index.html) module"]
pub struct ISDATA_SPEC;
impl crate::RegisterSpec for ISDATA_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [isdata::W](W) writer structure"]
impl crate::Writable for ISDATA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ISDATA to value 0"]
impl crate::Resettable for ISDATA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
