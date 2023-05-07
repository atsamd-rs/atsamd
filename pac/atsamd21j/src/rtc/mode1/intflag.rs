#[doc = "Register `INTFLAG` reader"]
pub struct R(crate::R<INTFLAG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTFLAG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTFLAG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTFLAG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTFLAG` writer"]
pub struct W(crate::W<INTFLAG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTFLAG_SPEC>;
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
impl From<crate::W<INTFLAG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTFLAG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CMP0` reader - Compare 0"]
pub type CMP0_R = crate::BitReader<bool>;
#[doc = "Field `CMP0` writer - Compare 0"]
pub type CMP0_W<'a, const O: u8> = crate::BitWriter<'a, u8, INTFLAG_SPEC, bool, O>;
#[doc = "Field `CMP1` reader - Compare 1"]
pub type CMP1_R = crate::BitReader<bool>;
#[doc = "Field `CMP1` writer - Compare 1"]
pub type CMP1_W<'a, const O: u8> = crate::BitWriter<'a, u8, INTFLAG_SPEC, bool, O>;
#[doc = "Field `SYNCRDY` reader - Synchronization Ready"]
pub type SYNCRDY_R = crate::BitReader<bool>;
#[doc = "Field `SYNCRDY` writer - Synchronization Ready"]
pub type SYNCRDY_W<'a, const O: u8> = crate::BitWriter<'a, u8, INTFLAG_SPEC, bool, O>;
#[doc = "Field `OVF` reader - Overflow"]
pub type OVF_R = crate::BitReader<bool>;
#[doc = "Field `OVF` writer - Overflow"]
pub type OVF_W<'a, const O: u8> = crate::BitWriter<'a, u8, INTFLAG_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Compare 0"]
    #[inline(always)]
    pub fn cmp0(&self) -> CMP0_R {
        CMP0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Compare 1"]
    #[inline(always)]
    pub fn cmp1(&self) -> CMP1_R {
        CMP1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 6 - Synchronization Ready"]
    #[inline(always)]
    pub fn syncrdy(&self) -> SYNCRDY_R {
        SYNCRDY_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Overflow"]
    #[inline(always)]
    pub fn ovf(&self) -> OVF_R {
        OVF_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Compare 0"]
    #[inline(always)]
    #[must_use]
    pub fn cmp0(&mut self) -> CMP0_W<0> {
        CMP0_W::new(self)
    }
    #[doc = "Bit 1 - Compare 1"]
    #[inline(always)]
    #[must_use]
    pub fn cmp1(&mut self) -> CMP1_W<1> {
        CMP1_W::new(self)
    }
    #[doc = "Bit 6 - Synchronization Ready"]
    #[inline(always)]
    #[must_use]
    pub fn syncrdy(&mut self) -> SYNCRDY_W<6> {
        SYNCRDY_W::new(self)
    }
    #[doc = "Bit 7 - Overflow"]
    #[inline(always)]
    #[must_use]
    pub fn ovf(&mut self) -> OVF_W<7> {
        OVF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MODE1 Interrupt Flag Status and Clear\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intflag](index.html) module"]
pub struct INTFLAG_SPEC;
impl crate::RegisterSpec for INTFLAG_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [intflag::R](R) reader structure"]
impl crate::Readable for INTFLAG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intflag::W](W) writer structure"]
impl crate::Writable for INTFLAG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTFLAG to value 0"]
impl crate::Resettable for INTFLAG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
