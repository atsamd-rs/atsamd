#[doc = "Register `SCRAMBCTRL` reader"]
pub struct R(crate::R<SCRAMBCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCRAMBCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCRAMBCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCRAMBCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCRAMBCTRL` writer"]
pub struct W(crate::W<SCRAMBCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCRAMBCTRL_SPEC>;
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
impl From<crate::W<SCRAMBCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCRAMBCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENABLE` reader - Scrambling/Unscrambling Enable"]
pub type ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `ENABLE` writer - Scrambling/Unscrambling Enable"]
pub type ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCRAMBCTRL_SPEC, bool, O>;
#[doc = "Field `RANDOMDIS` reader - Scrambling/Unscrambling Random Value Disable"]
pub type RANDOMDIS_R = crate::BitReader<bool>;
#[doc = "Field `RANDOMDIS` writer - Scrambling/Unscrambling Random Value Disable"]
pub type RANDOMDIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCRAMBCTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Scrambling/Unscrambling Enable"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Scrambling/Unscrambling Random Value Disable"]
    #[inline(always)]
    pub fn randomdis(&self) -> RANDOMDIS_R {
        RANDOMDIS_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Scrambling/Unscrambling Enable"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> ENABLE_W<0> {
        ENABLE_W::new(self)
    }
    #[doc = "Bit 1 - Scrambling/Unscrambling Random Value Disable"]
    #[inline(always)]
    #[must_use]
    pub fn randomdis(&mut self) -> RANDOMDIS_W<1> {
        RANDOMDIS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Scrambling Mode\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scrambctrl](index.html) module"]
pub struct SCRAMBCTRL_SPEC;
impl crate::RegisterSpec for SCRAMBCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [scrambctrl::R](R) reader structure"]
impl crate::Readable for SCRAMBCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [scrambctrl::W](W) writer structure"]
impl crate::Writable for SCRAMBCTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SCRAMBCTRL to value 0"]
impl crate::Resettable for SCRAMBCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
