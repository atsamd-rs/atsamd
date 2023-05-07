#[doc = "Register `GENDIV` reader"]
pub struct R(crate::R<GENDIV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GENDIV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GENDIV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GENDIV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GENDIV` writer"]
pub struct W(crate::W<GENDIV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GENDIV_SPEC>;
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
impl From<crate::W<GENDIV_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GENDIV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ID` reader - Generic Clock Generator Selection"]
pub type ID_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ID` writer - Generic Clock Generator Selection"]
pub type ID_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GENDIV_SPEC, u8, u8, 4, O>;
#[doc = "Field `DIV` reader - Division Factor"]
pub type DIV_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DIV` writer - Division Factor"]
pub type DIV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GENDIV_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:3 - Generic Clock Generator Selection"]
    #[inline(always)]
    pub fn id(&self) -> ID_R {
        ID_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:23 - Division Factor"]
    #[inline(always)]
    pub fn div(&self) -> DIV_R {
        DIV_R::new(((self.bits >> 8) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:3 - Generic Clock Generator Selection"]
    #[inline(always)]
    #[must_use]
    pub fn id(&mut self) -> ID_W<0> {
        ID_W::new(self)
    }
    #[doc = "Bits 8:23 - Division Factor"]
    #[inline(always)]
    #[must_use]
    pub fn div(&mut self) -> DIV_W<8> {
        DIV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Generic Clock Generator Division\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gendiv](index.html) module"]
pub struct GENDIV_SPEC;
impl crate::RegisterSpec for GENDIV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gendiv::R](R) reader structure"]
impl crate::Readable for GENDIV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gendiv::W](W) writer structure"]
impl crate::Writable for GENDIV_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GENDIV to value 0"]
impl crate::Resettable for GENDIV_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
