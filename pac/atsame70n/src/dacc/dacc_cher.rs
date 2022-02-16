#[doc = "Register `DACC_CHER` writer"]
pub struct W(crate::W<DACC_CHER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DACC_CHER_SPEC>;
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
impl From<crate::W<DACC_CHER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DACC_CHER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CH0` writer - Channel 0 Enable"]
pub struct CH0_W<'a> {
    w: &'a mut W,
}
impl<'a> CH0_W<'a> {
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
#[doc = "Field `CH1` writer - Channel 1 Enable"]
pub struct CH1_W<'a> {
    w: &'a mut W,
}
impl<'a> CH1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
impl W {
    #[doc = "Bit 0 - Channel 0 Enable"]
    #[inline(always)]
    pub fn ch0(&mut self) -> CH0_W {
        CH0_W { w: self }
    }
    #[doc = "Bit 1 - Channel 1 Enable"]
    #[inline(always)]
    pub fn ch1(&mut self) -> CH1_W {
        CH1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel Enable Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dacc_cher](index.html) module"]
pub struct DACC_CHER_SPEC;
impl crate::RegisterSpec for DACC_CHER_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [dacc_cher::W](W) writer structure"]
impl crate::Writable for DACC_CHER_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DACC_CHER to value 0"]
impl crate::Resettable for DACC_CHER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
