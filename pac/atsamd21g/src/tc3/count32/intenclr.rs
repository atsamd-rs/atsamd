#[doc = "Register `INTENCLR` reader"]
pub struct R(crate::R<INTENCLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTENCLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTENCLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTENCLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTENCLR` writer"]
pub struct W(crate::W<INTENCLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTENCLR_SPEC>;
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
impl From<crate::W<INTENCLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTENCLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OVF` reader - Overflow Interrupt Enable"]
pub type OVF_R = crate::BitReader<bool>;
#[doc = "Field `OVF` writer - Overflow Interrupt Enable"]
pub type OVF_W<'a, const O: u8> = crate::BitWriter<'a, u8, INTENCLR_SPEC, bool, O>;
#[doc = "Field `ERR` reader - Error Interrupt Enable"]
pub type ERR_R = crate::BitReader<bool>;
#[doc = "Field `ERR` writer - Error Interrupt Enable"]
pub type ERR_W<'a, const O: u8> = crate::BitWriter<'a, u8, INTENCLR_SPEC, bool, O>;
#[doc = "Field `SYNCRDY` reader - Synchronization Ready Interrupt Enable"]
pub type SYNCRDY_R = crate::BitReader<bool>;
#[doc = "Field `SYNCRDY` writer - Synchronization Ready Interrupt Enable"]
pub type SYNCRDY_W<'a, const O: u8> = crate::BitWriter<'a, u8, INTENCLR_SPEC, bool, O>;
#[doc = "Field `MC0` reader - Match or Capture Channel 0 Interrupt Enable"]
pub type MC0_R = crate::BitReader<bool>;
#[doc = "Field `MC0` writer - Match or Capture Channel 0 Interrupt Enable"]
pub type MC0_W<'a, const O: u8> = crate::BitWriter<'a, u8, INTENCLR_SPEC, bool, O>;
#[doc = "Field `MC1` reader - Match or Capture Channel 1 Interrupt Enable"]
pub type MC1_R = crate::BitReader<bool>;
#[doc = "Field `MC1` writer - Match or Capture Channel 1 Interrupt Enable"]
pub type MC1_W<'a, const O: u8> = crate::BitWriter<'a, u8, INTENCLR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn ovf(&self) -> OVF_R {
        OVF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Error Interrupt Enable"]
    #[inline(always)]
    pub fn err(&self) -> ERR_R {
        ERR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Synchronization Ready Interrupt Enable"]
    #[inline(always)]
    pub fn syncrdy(&self) -> SYNCRDY_R {
        SYNCRDY_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Match or Capture Channel 0 Interrupt Enable"]
    #[inline(always)]
    pub fn mc0(&self) -> MC0_R {
        MC0_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Match or Capture Channel 1 Interrupt Enable"]
    #[inline(always)]
    pub fn mc1(&self) -> MC1_R {
        MC1_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Overflow Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ovf(&mut self) -> OVF_W<0> {
        OVF_W::new(self)
    }
    #[doc = "Bit 1 - Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn err(&mut self) -> ERR_W<1> {
        ERR_W::new(self)
    }
    #[doc = "Bit 3 - Synchronization Ready Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn syncrdy(&mut self) -> SYNCRDY_W<3> {
        SYNCRDY_W::new(self)
    }
    #[doc = "Bit 4 - Match or Capture Channel 0 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn mc0(&mut self) -> MC0_W<4> {
        MC0_W::new(self)
    }
    #[doc = "Bit 5 - Match or Capture Channel 1 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn mc1(&mut self) -> MC1_W<5> {
        MC1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Enable Clear\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intenclr](index.html) module"]
pub struct INTENCLR_SPEC;
impl crate::RegisterSpec for INTENCLR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [intenclr::R](R) reader structure"]
impl crate::Readable for INTENCLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intenclr::W](W) writer structure"]
impl crate::Writable for INTENCLR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTENCLR to value 0"]
impl crate::Resettable for INTENCLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
