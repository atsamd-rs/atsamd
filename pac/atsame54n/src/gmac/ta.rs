#[doc = "Register `TA` writer"]
pub struct W(crate::W<TA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TA_SPEC>;
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
impl From<crate::W<TA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ITDT` writer - Increment/Decrement"]
pub struct ITDT_W<'a> {
    w: &'a mut W,
}
impl<'a> ITDT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3fff_ffff) | (value as u32 & 0x3fff_ffff);
        self.w
    }
}
#[doc = "Field `ADJ` writer - Adjust 1588 Timer"]
pub struct ADJ_W<'a> {
    w: &'a mut W,
}
impl<'a> ADJ_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:29 - Increment/Decrement"]
    #[inline(always)]
    pub fn itdt(&mut self) -> ITDT_W {
        ITDT_W { w: self }
    }
    #[doc = "Bit 31 - Adjust 1588 Timer"]
    #[inline(always)]
    pub fn adj(&mut self) -> ADJ_W {
        ADJ_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "1588 Timer Adjust Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ta](index.html) module"]
pub struct TA_SPEC;
impl crate::RegisterSpec for TA_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [ta::W](W) writer structure"]
impl crate::Writable for TA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TA to value 0"]
impl crate::Resettable for TA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
