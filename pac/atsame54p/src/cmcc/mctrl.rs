#[doc = "Register `MCTRL` writer"]
pub struct W(crate::W<MCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MCTRL_SPEC>;
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
impl From<crate::W<MCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SWRST` writer - Cache Controller Software Reset"]
pub struct SWRST_W<'a> {
    w: &'a mut W,
}
impl<'a> SWRST_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
impl W {
    #[doc = "Bit 0 - Cache Controller Software Reset"]
    #[inline(always)]
    pub fn swrst(&mut self) -> SWRST_W {
        SWRST_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Cache Monitor Control Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mctrl](index.html) module"]
pub struct MCTRL_SPEC;
impl crate::RegisterSpec for MCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [mctrl::W](W) writer structure"]
impl crate::Writable for MCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MCTRL to value 0"]
impl crate::Resettable for MCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
