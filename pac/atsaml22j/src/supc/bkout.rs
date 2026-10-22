#[doc = "Register `BKOUT` reader"]
pub struct R(crate::R<BKOUT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BKOUT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BKOUT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BKOUT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BKOUT` writer"]
pub struct W(crate::W<BKOUT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BKOUT_SPEC>;
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
impl From<crate::W<BKOUT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BKOUT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EN` reader - Enable Output"]
pub type EN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EN` writer - Enable Output"]
pub type EN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BKOUT_SPEC, u8, u8, 2, O>;
#[doc = "Field `CLR` writer - Clear Output"]
pub type CLR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BKOUT_SPEC, u8, u8, 2, O>;
#[doc = "Field `SET` writer - Set Output"]
pub type SET_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BKOUT_SPEC, u8, u8, 2, O>;
#[doc = "Field `RTCTGL` reader - RTC Toggle Output"]
pub type RTCTGL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RTCTGL` writer - RTC Toggle Output"]
pub type RTCTGL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BKOUT_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:1 - Enable Output"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 24:25 - RTC Toggle Output"]
    #[inline(always)]
    pub fn rtctgl(&self) -> RTCTGL_R {
        RTCTGL_R::new(((self.bits >> 24) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Enable Output"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<0> {
        EN_W::new(self)
    }
    #[doc = "Bits 8:9 - Clear Output"]
    #[inline(always)]
    #[must_use]
    pub fn clr(&mut self) -> CLR_W<8> {
        CLR_W::new(self)
    }
    #[doc = "Bits 16:17 - Set Output"]
    #[inline(always)]
    #[must_use]
    pub fn set(&mut self) -> SET_W<16> {
        SET_W::new(self)
    }
    #[doc = "Bits 24:25 - RTC Toggle Output"]
    #[inline(always)]
    #[must_use]
    pub fn rtctgl(&mut self) -> RTCTGL_W<24> {
        RTCTGL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Backup Output Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bkout](index.html) module"]
pub struct BKOUT_SPEC;
impl crate::RegisterSpec for BKOUT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bkout::R](R) reader structure"]
impl crate::Readable for BKOUT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bkout::W](W) writer structure"]
impl crate::Writable for BKOUT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BKOUT to value 0"]
impl crate::Resettable for BKOUT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
