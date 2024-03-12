#[doc = "Register `CMCFG` reader"]
pub struct R(crate::R<CMCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CMCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CMCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CMCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CMCFG` writer"]
pub struct W(crate::W<CMCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CMCFG_SPEC>;
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
impl From<crate::W<CMCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CMCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `NSEG` reader - Number of SEG lines"]
pub type NSEG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `NSEG` writer - Number of SEG lines"]
pub type NSEG_W<'a, const O: u8> = crate::FieldWriter<'a, u8, CMCFG_SPEC, u8, u8, 3, O>;
#[doc = "Field `DEC` reader - Decrement SEG Line Index"]
pub type DEC_R = crate::BitReader<bool>;
#[doc = "Field `DEC` writer - Decrement SEG Line Index"]
pub type DEC_W<'a, const O: u8> = crate::BitWriter<'a, u8, CMCFG_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:2 - Number of SEG lines"]
    #[inline(always)]
    pub fn nseg(&self) -> NSEG_R {
        NSEG_R::new(self.bits & 7)
    }
    #[doc = "Bit 3 - Decrement SEG Line Index"]
    #[inline(always)]
    pub fn dec(&self) -> DEC_R {
        DEC_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Number of SEG lines"]
    #[inline(always)]
    #[must_use]
    pub fn nseg(&mut self) -> NSEG_W<0> {
        NSEG_W::new(self)
    }
    #[doc = "Bit 3 - Decrement SEG Line Index"]
    #[inline(always)]
    #[must_use]
    pub fn dec(&mut self) -> DEC_W<3> {
        DEC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Character Mapping Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmcfg](index.html) module"]
pub struct CMCFG_SPEC;
impl crate::RegisterSpec for CMCFG_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [cmcfg::R](R) reader structure"]
impl crate::Readable for CMCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cmcfg::W](W) writer structure"]
impl crate::Writable for CMCFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CMCFG to value 0"]
impl crate::Resettable for CMCFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
