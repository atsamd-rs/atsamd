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
pub struct BKUP_R(crate::FieldReader<u32, u32>);
impl BKUP_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        BKUP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BKUP_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BKUP` writer - Backup"]
pub struct BKUP_W<'a> {
    w: &'a mut W,
}
impl<'a> BKUP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value as u32;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Backup"]
    #[inline(always)]
    pub fn bkup(&self) -> BKUP_R {
        BKUP_R::new(self.bits as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Backup"]
    #[inline(always)]
    pub fn bkup(&mut self) -> BKUP_W {
        BKUP_W { w: self }
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
}
#[doc = "`reset()` method sets BKUP[%s]
to value 0"]
impl crate::Resettable for BKUP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
