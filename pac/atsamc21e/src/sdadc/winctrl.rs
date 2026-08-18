#[doc = "Register `WINCTRL` reader"]
pub struct R(crate::R<WINCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WINCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WINCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WINCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WINCTRL` writer"]
pub struct W(crate::W<WINCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WINCTRL_SPEC>;
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
impl From<crate::W<WINCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WINCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WINMODE` reader - Window Monitor Mode"]
pub struct WINMODE_R(crate::FieldReader<u8, u8>);
impl WINMODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        WINMODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WINMODE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WINMODE` writer - Window Monitor Mode"]
pub struct WINMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> WINMODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u8 & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Window Monitor Mode"]
    #[inline(always)]
    pub fn winmode(&self) -> WINMODE_R {
        WINMODE_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Window Monitor Mode"]
    #[inline(always)]
    pub fn winmode(&mut self) -> WINMODE_W {
        WINMODE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Window Monitor Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [winctrl](index.html) module"]
pub struct WINCTRL_SPEC;
impl crate::RegisterSpec for WINCTRL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [winctrl::R](R) reader structure"]
impl crate::Readable for WINCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [winctrl::W](W) writer structure"]
impl crate::Writable for WINCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WINCTRL to value 0"]
impl crate::Resettable for WINCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
