#[doc = "Register `TRNG_CR` writer"]
pub struct W(crate::W<TRNG_CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TRNG_CR_SPEC>;
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
impl From<crate::W<TRNG_CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TRNG_CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENABLE` writer - Enables the TRNG to Provide Random Values"]
pub struct ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE_W<'a> {
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
#[doc = "Security Key\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum KEY_AW {
    #[doc = "5393991: Writing any other value in this field aborts the write operation."]
    PASSWD = 5393991,
}
impl From<KEY_AW> for u32 {
    #[inline(always)]
    fn from(variant: KEY_AW) -> Self {
        variant as _
    }
}
#[doc = "Field `KEY` writer - Security Key"]
pub struct KEY_W<'a> {
    w: &'a mut W,
}
impl<'a> KEY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: KEY_AW) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Writing any other value in this field aborts the write operation."]
    #[inline(always)]
    pub fn passwd(self) -> &'a mut W {
        self.variant(KEY_AW::PASSWD)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x00ff_ffff << 8)) | ((value as u32 & 0x00ff_ffff) << 8);
        self.w
    }
}
impl W {
    #[doc = "Bit 0 - Enables the TRNG to Provide Random Values"]
    #[inline(always)]
    pub fn enable(&mut self) -> ENABLE_W {
        ENABLE_W { w: self }
    }
    #[doc = "Bits 8:31 - Security Key"]
    #[inline(always)]
    pub fn key(&mut self) -> KEY_W {
        KEY_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trng_cr](index.html) module"]
pub struct TRNG_CR_SPEC;
impl crate::RegisterSpec for TRNG_CR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [trng_cr::W](W) writer structure"]
impl crate::Writable for TRNG_CR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TRNG_CR to value 0"]
impl crate::Resettable for TRNG_CR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
