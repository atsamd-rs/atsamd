#[doc = "Register `PWSAKDLY` reader"]
pub struct R(crate::R<PWSAKDLY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWSAKDLY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWSAKDLY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWSAKDLY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PWSAKDLY` writer"]
pub struct W(crate::W<PWSAKDLY_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWSAKDLY_SPEC>;
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
impl From<crate::W<PWSAKDLY_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PWSAKDLY_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DLYVAL` reader - Delay Value"]
pub type DLYVAL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DLYVAL` writer - Delay Value"]
pub type DLYVAL_W<'a, const O: u8> = crate::FieldWriter<'a, u8, PWSAKDLY_SPEC, u8, u8, 7, O>;
#[doc = "Field `IGNACK` reader - Ignore Acknowledge"]
pub type IGNACK_R = crate::BitReader<bool>;
#[doc = "Field `IGNACK` writer - Ignore Acknowledge"]
pub type IGNACK_W<'a, const O: u8> = crate::BitWriter<'a, u8, PWSAKDLY_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:6 - Delay Value"]
    #[inline(always)]
    pub fn dlyval(&self) -> DLYVAL_R {
        DLYVAL_R::new(self.bits & 0x7f)
    }
    #[doc = "Bit 7 - Ignore Acknowledge"]
    #[inline(always)]
    pub fn ignack(&self) -> IGNACK_R {
        IGNACK_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - Delay Value"]
    #[inline(always)]
    #[must_use]
    pub fn dlyval(&mut self) -> DLYVAL_W<0> {
        DLYVAL_W::new(self)
    }
    #[doc = "Bit 7 - Ignore Acknowledge"]
    #[inline(always)]
    #[must_use]
    pub fn ignack(&mut self) -> IGNACK_W<7> {
        IGNACK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Power Switch Acknowledge Delay\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwsakdly](index.html) module"]
pub struct PWSAKDLY_SPEC;
impl crate::RegisterSpec for PWSAKDLY_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [pwsakdly::R](R) reader structure"]
impl crate::Readable for PWSAKDLY_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwsakdly::W](W) writer structure"]
impl crate::Writable for PWSAKDLY_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PWSAKDLY to value 0"]
impl crate::Resettable for PWSAKDLY_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
