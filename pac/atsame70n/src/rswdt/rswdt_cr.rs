#[doc = "Register `RSWDT_CR` writer"]
pub struct W(crate::W<RSWDT_CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RSWDT_CR_SPEC>;
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
impl From<crate::W<RSWDT_CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RSWDT_CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WDRSTT` writer - Watchdog Restart"]
pub struct WDRSTT_W<'a> {
    w: &'a mut W,
}
impl<'a> WDRSTT_W<'a> {
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
#[doc = "Password\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum KEY_AW {
    #[doc = "196: Writing any other value in this field aborts the write operation."]
    PASSWD = 196,
}
impl From<KEY_AW> for u8 {
    #[inline(always)]
    fn from(variant: KEY_AW) -> Self {
        variant as _
    }
}
#[doc = "Field `KEY` writer - Password"]
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
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | ((value as u32 & 0xff) << 24);
        self.w
    }
}
impl W {
    #[doc = "Bit 0 - Watchdog Restart"]
    #[inline(always)]
    pub fn wdrstt(&mut self) -> WDRSTT_W {
        WDRSTT_W { w: self }
    }
    #[doc = "Bits 24:31 - Password"]
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
#[doc = "Control Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rswdt_cr](index.html) module"]
pub struct RSWDT_CR_SPEC;
impl crate::RegisterSpec for RSWDT_CR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [rswdt_cr::W](W) writer structure"]
impl crate::Writable for RSWDT_CR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RSWDT_CR to value 0"]
impl crate::Resettable for RSWDT_CR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
