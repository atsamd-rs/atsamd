#[doc = "Register `BKUP[%s]` reader"]
pub struct R(crate::R<BKUP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BKUP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BKUP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BKUP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BKUP[%s]` writer"]
pub struct W(crate::W<BKUP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BKUP_SPEC>;
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
impl From<crate::W<BKUP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BKUP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BKUP` reader - Backup"]
pub type BKUP_R = crate::FieldReader<u32, u32>;
#[doc = "Field `BKUP` writer - Backup"]
pub type BKUP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BKUP_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Backup"]
    #[inline(always)]
    pub fn bkup(&self) -> BKUP_R {
        BKUP_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Backup"]
    #[inline(always)]
    #[must_use]
    pub fn bkup(&mut self) -> BKUP_W<0> {
        BKUP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Backup\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bkup](index.html) module"]
pub struct BKUP_SPEC;
impl crate::RegisterSpec for BKUP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bkup::R](R) reader structure"]
impl crate::Readable for BKUP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bkup::W](W) writer structure"]
impl crate::Writable for BKUP_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BKUP[%s]
to value 0"]
impl crate::Resettable for BKUP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
