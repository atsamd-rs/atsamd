#[doc = "Register `DACC_CDR[%s]` writer"]
pub struct W(crate::W<DACC_CDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DACC_CDR_SPEC>;
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
impl From<crate::W<DACC_CDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DACC_CDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATA0` writer - Data to Convert for channel 0"]
pub struct DATA0_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
#[doc = "Field `DATA1` writer - Data to Convert for channel 1"]
pub struct DATA1_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | ((value as u32 & 0xffff) << 16);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:15 - Data to Convert for channel 0"]
    #[inline(always)]
    pub fn data0(&mut self) -> DATA0_W {
        DATA0_W { w: self }
    }
    #[doc = "Bits 16:31 - Data to Convert for channel 1"]
    #[inline(always)]
    pub fn data1(&mut self) -> DATA1_W {
        DATA1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Conversion Data Register 0\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dacc_cdr](index.html) module"]
pub struct DACC_CDR_SPEC;
impl crate::RegisterSpec for DACC_CDR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [dacc_cdr::W](W) writer structure"]
impl crate::Writable for DACC_CDR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DACC_CDR[%s]
to value 0"]
impl crate::Resettable for DACC_CDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
