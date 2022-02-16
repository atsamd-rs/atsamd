#[doc = "Register `AES_KEYWR[%s]` writer"]
pub struct W(crate::W<AES_KEYWR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AES_KEYWR_SPEC>;
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
impl From<crate::W<AES_KEYWR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AES_KEYWR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `KEYW` writer - Key Word"]
pub struct KEYW_W<'a> {
    w: &'a mut W,
}
impl<'a> KEYW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value as u32;
        self.w
    }
}
impl W {
    #[doc = "Bits 0:31 - Key Word"]
    #[inline(always)]
    pub fn keyw(&mut self) -> KEYW_W {
        KEYW_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Key Word Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aes_keywr](index.html) module"]
pub struct AES_KEYWR_SPEC;
impl crate::RegisterSpec for AES_KEYWR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [aes_keywr::W](W) writer structure"]
impl crate::Writable for AES_KEYWR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AES_KEYWR[%s]
to value 0"]
impl crate::Resettable for AES_KEYWR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
