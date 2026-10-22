#[doc = "Register `SLCDCTRL` reader"]
pub struct R(crate::R<SLCDCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SLCDCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SLCDCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SLCDCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SLCDCTRL` writer"]
pub struct W(crate::W<SLCDCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SLCDCTRL_SPEC>;
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
impl From<crate::W<SLCDCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SLCDCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SLCDSEL` reader - SLCD Clock Selection"]
pub type SLCDSEL_R = crate::BitReader<bool>;
#[doc = "Field `SLCDSEL` writer - SLCD Clock Selection"]
pub type SLCDSEL_W<'a, const O: u8> = crate::BitWriter<'a, u8, SLCDCTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - SLCD Clock Selection"]
    #[inline(always)]
    pub fn slcdsel(&self) -> SLCDSEL_R {
        SLCDSEL_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SLCD Clock Selection"]
    #[inline(always)]
    #[must_use]
    pub fn slcdsel(&mut self) -> SLCDSEL_W<0> {
        SLCDSEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SLCD Clock Selection\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [slcdctrl](index.html) module"]
pub struct SLCDCTRL_SPEC;
impl crate::RegisterSpec for SLCDCTRL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [slcdctrl::R](R) reader structure"]
impl crate::Readable for SLCDCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [slcdctrl::W](W) writer structure"]
impl crate::Writable for SLCDCTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SLCDCTRL to value 0"]
impl crate::Resettable for SLCDCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
