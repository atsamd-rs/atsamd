#[doc = "Register `PRICTRL` reader"]
pub struct R(crate::R<PRICTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRICTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRICTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRICTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PRICTRL` writer"]
pub struct W(crate::W<PRICTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PRICTRL_SPEC>;
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
impl From<crate::W<PRICTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PRICTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRI` reader - Channel Priority Number"]
pub type PRI_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PRI` writer - Channel Priority Number"]
pub type PRI_W<'a, const O: u8> = crate::FieldWriter<'a, u8, PRICTRL_SPEC, u8, u8, 4, O>;
#[doc = "Field `RREN` reader - Round-Robin Scheduling Enable"]
pub type RREN_R = crate::BitReader<bool>;
#[doc = "Field `RREN` writer - Round-Robin Scheduling Enable"]
pub type RREN_W<'a, const O: u8> = crate::BitWriter<'a, u8, PRICTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:3 - Channel Priority Number"]
    #[inline(always)]
    pub fn pri(&self) -> PRI_R {
        PRI_R::new(self.bits & 0x0f)
    }
    #[doc = "Bit 7 - Round-Robin Scheduling Enable"]
    #[inline(always)]
    pub fn rren(&self) -> RREN_R {
        RREN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Channel Priority Number"]
    #[inline(always)]
    #[must_use]
    pub fn pri(&mut self) -> PRI_W<0> {
        PRI_W::new(self)
    }
    #[doc = "Bit 7 - Round-Robin Scheduling Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rren(&mut self) -> RREN_W<7> {
        RREN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Priority Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prictrl](index.html) module"]
pub struct PRICTRL_SPEC;
impl crate::RegisterSpec for PRICTRL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [prictrl::R](R) reader structure"]
impl crate::Readable for PRICTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [prictrl::W](W) writer structure"]
impl crate::Writable for PRICTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PRICTRL to value 0"]
impl crate::Resettable for PRICTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
