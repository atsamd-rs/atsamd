#[doc = "Register `US_THR` writer"]
pub struct W(crate::W<US_THR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<US_THR_SPEC>;
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
impl From<crate::W<US_THR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<US_THR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TXCHR` writer - Character to be Transmitted"]
pub struct TXCHR_W<'a> {
    w: &'a mut W,
}
impl<'a> TXCHR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01ff) | (value as u32 & 0x01ff);
        self.w
    }
}
#[doc = "Field `TXSYNH` writer - Sync Field to be Transmitted"]
pub struct TXSYNH_W<'a> {
    w: &'a mut W,
}
impl<'a> TXSYNH_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:8 - Character to be Transmitted"]
    #[inline(always)]
    pub fn txchr(&mut self) -> TXCHR_W {
        TXCHR_W { w: self }
    }
    #[doc = "Bit 15 - Sync Field to be Transmitted"]
    #[inline(always)]
    pub fn txsynh(&mut self) -> TXSYNH_W {
        TXSYNH_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Transmit Holding Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [us_thr](index.html) module"]
pub struct US_THR_SPEC;
impl crate::RegisterSpec for US_THR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [us_thr::W](W) writer structure"]
impl crate::Writable for US_THR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets US_THR to value 0"]
impl crate::Resettable for US_THR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
