#[doc = "Register `CHINTFLAG` reader"]
pub struct R(crate::R<CHINTFLAG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CHINTFLAG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CHINTFLAG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CHINTFLAG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CHINTFLAG` writer"]
pub struct W(crate::W<CHINTFLAG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CHINTFLAG_SPEC>;
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
impl From<crate::W<CHINTFLAG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CHINTFLAG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TERR` reader - Channel Transfer Error"]
pub type TERR_R = crate::BitReader<bool>;
#[doc = "Field `TERR` writer - Channel Transfer Error"]
pub type TERR_W<'a, const O: u8> = crate::BitWriter<'a, u8, CHINTFLAG_SPEC, bool, O>;
#[doc = "Field `TCMPL` reader - Channel Transfer Complete"]
pub type TCMPL_R = crate::BitReader<bool>;
#[doc = "Field `TCMPL` writer - Channel Transfer Complete"]
pub type TCMPL_W<'a, const O: u8> = crate::BitWriter<'a, u8, CHINTFLAG_SPEC, bool, O>;
#[doc = "Field `SUSP` reader - Channel Suspend"]
pub type SUSP_R = crate::BitReader<bool>;
#[doc = "Field `SUSP` writer - Channel Suspend"]
pub type SUSP_W<'a, const O: u8> = crate::BitWriter<'a, u8, CHINTFLAG_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Channel Transfer Error"]
    #[inline(always)]
    pub fn terr(&self) -> TERR_R {
        TERR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel Transfer Complete"]
    #[inline(always)]
    pub fn tcmpl(&self) -> TCMPL_R {
        TCMPL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Channel Suspend"]
    #[inline(always)]
    pub fn susp(&self) -> SUSP_R {
        SUSP_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Channel Transfer Error"]
    #[inline(always)]
    #[must_use]
    pub fn terr(&mut self) -> TERR_W<0> {
        TERR_W::new(self)
    }
    #[doc = "Bit 1 - Channel Transfer Complete"]
    #[inline(always)]
    #[must_use]
    pub fn tcmpl(&mut self) -> TCMPL_W<1> {
        TCMPL_W::new(self)
    }
    #[doc = "Bit 2 - Channel Suspend"]
    #[inline(always)]
    #[must_use]
    pub fn susp(&mut self) -> SUSP_W<2> {
        SUSP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel Interrupt Flag Status and Clear\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chintflag](index.html) module"]
pub struct CHINTFLAG_SPEC;
impl crate::RegisterSpec for CHINTFLAG_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [chintflag::R](R) reader structure"]
impl crate::Readable for CHINTFLAG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [chintflag::W](W) writer structure"]
impl crate::Writable for CHINTFLAG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CHINTFLAG to value 0"]
impl crate::Resettable for CHINTFLAG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
