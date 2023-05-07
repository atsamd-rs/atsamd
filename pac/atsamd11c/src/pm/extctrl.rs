#[doc = "Register `EXTCTRL` reader"]
pub struct R(crate::R<EXTCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EXTCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EXTCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EXTCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EXTCTRL` writer"]
pub struct W(crate::W<EXTCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EXTCTRL_SPEC>;
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
impl From<crate::W<EXTCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EXTCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SETDIS` reader - External Reset Disable"]
pub type SETDIS_R = crate::BitReader<bool>;
#[doc = "Field `SETDIS` writer - External Reset Disable"]
pub type SETDIS_W<'a, const O: u8> = crate::BitWriter<'a, u8, EXTCTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - External Reset Disable"]
    #[inline(always)]
    pub fn setdis(&self) -> SETDIS_R {
        SETDIS_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - External Reset Disable"]
    #[inline(always)]
    #[must_use]
    pub fn setdis(&mut self) -> SETDIS_W<0> {
        SETDIS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "External Reset Controller\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [extctrl](index.html) module"]
pub struct EXTCTRL_SPEC;
impl crate::RegisterSpec for EXTCTRL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [extctrl::R](R) reader structure"]
impl crate::Readable for EXTCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [extctrl::W](W) writer structure"]
impl crate::Writable for EXTCTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EXTCTRL to value 0"]
impl crate::Resettable for EXTCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
