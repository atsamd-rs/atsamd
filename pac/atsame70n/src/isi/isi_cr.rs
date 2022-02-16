#[doc = "Register `ISI_CR` writer"]
pub struct W(crate::W<ISI_CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ISI_CR_SPEC>;
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
impl From<crate::W<ISI_CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ISI_CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ISI_EN` writer - ISI Module Enable Request"]
pub struct ISI_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ISI_EN_W<'a> {
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
#[doc = "Field `ISI_DIS` writer - ISI Module Disable Request"]
pub struct ISI_DIS_W<'a> {
    w: &'a mut W,
}
impl<'a> ISI_DIS_W<'a> {
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
#[doc = "Field `ISI_SRST` writer - ISI Software Reset Request"]
pub struct ISI_SRST_W<'a> {
    w: &'a mut W,
}
impl<'a> ISI_SRST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `ISI_CDC` writer - ISI Codec Request"]
pub struct ISI_CDC_W<'a> {
    w: &'a mut W,
}
impl<'a> ISI_CDC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
impl W {
    #[doc = "Bit 0 - ISI Module Enable Request"]
    #[inline(always)]
    pub fn isi_en(&mut self) -> ISI_EN_W {
        ISI_EN_W { w: self }
    }
    #[doc = "Bit 1 - ISI Module Disable Request"]
    #[inline(always)]
    pub fn isi_dis(&mut self) -> ISI_DIS_W {
        ISI_DIS_W { w: self }
    }
    #[doc = "Bit 2 - ISI Software Reset Request"]
    #[inline(always)]
    pub fn isi_srst(&mut self) -> ISI_SRST_W {
        ISI_SRST_W { w: self }
    }
    #[doc = "Bit 8 - ISI Codec Request"]
    #[inline(always)]
    pub fn isi_cdc(&mut self) -> ISI_CDC_W {
        ISI_CDC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ISI Control Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [isi_cr](index.html) module"]
pub struct ISI_CR_SPEC;
impl crate::RegisterSpec for ISI_CR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [isi_cr::W](W) writer structure"]
impl crate::Writable for ISI_CR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ISI_CR to value 0"]
impl crate::Resettable for ISI_CR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
