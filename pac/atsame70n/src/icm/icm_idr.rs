#[doc = "Register `ICM_IDR` writer"]
pub struct W(crate::W<ICM_IDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ICM_IDR_SPEC>;
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
impl From<crate::W<ICM_IDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ICM_IDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RHC` writer - Region Hash Completed Interrupt Disable"]
pub struct RHC_W<'a> {
    w: &'a mut W,
}
impl<'a> RHC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
#[doc = "Field `RDM` writer - Region Digest Mismatch Interrupt Disable"]
pub struct RDM_W<'a> {
    w: &'a mut W,
}
impl<'a> RDM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | ((value as u32 & 0x0f) << 4);
        self.w
    }
}
#[doc = "Field `RBE` writer - Region Bus Error Interrupt Disable"]
pub struct RBE_W<'a> {
    w: &'a mut W,
}
impl<'a> RBE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
#[doc = "Field `RWC` writer - Region Wrap Condition Detected Interrupt Disable"]
pub struct RWC_W<'a> {
    w: &'a mut W,
}
impl<'a> RWC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | ((value as u32 & 0x0f) << 12);
        self.w
    }
}
#[doc = "Field `REC` writer - Region End bit Condition detected Interrupt Disable"]
pub struct REC_W<'a> {
    w: &'a mut W,
}
impl<'a> REC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | ((value as u32 & 0x0f) << 16);
        self.w
    }
}
#[doc = "Field `RSU` writer - Region Status Updated Interrupt Disable"]
pub struct RSU_W<'a> {
    w: &'a mut W,
}
impl<'a> RSU_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | ((value as u32 & 0x0f) << 20);
        self.w
    }
}
#[doc = "Field `URAD` writer - Undefined Register Access Detection Interrupt Disable"]
pub struct URAD_W<'a> {
    w: &'a mut W,
}
impl<'a> URAD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:3 - Region Hash Completed Interrupt Disable"]
    #[inline(always)]
    pub fn rhc(&mut self) -> RHC_W {
        RHC_W { w: self }
    }
    #[doc = "Bits 4:7 - Region Digest Mismatch Interrupt Disable"]
    #[inline(always)]
    pub fn rdm(&mut self) -> RDM_W {
        RDM_W { w: self }
    }
    #[doc = "Bits 8:11 - Region Bus Error Interrupt Disable"]
    #[inline(always)]
    pub fn rbe(&mut self) -> RBE_W {
        RBE_W { w: self }
    }
    #[doc = "Bits 12:15 - Region Wrap Condition Detected Interrupt Disable"]
    #[inline(always)]
    pub fn rwc(&mut self) -> RWC_W {
        RWC_W { w: self }
    }
    #[doc = "Bits 16:19 - Region End bit Condition detected Interrupt Disable"]
    #[inline(always)]
    pub fn rec(&mut self) -> REC_W {
        REC_W { w: self }
    }
    #[doc = "Bits 20:23 - Region Status Updated Interrupt Disable"]
    #[inline(always)]
    pub fn rsu(&mut self) -> RSU_W {
        RSU_W { w: self }
    }
    #[doc = "Bit 24 - Undefined Register Access Detection Interrupt Disable"]
    #[inline(always)]
    pub fn urad(&mut self) -> URAD_W {
        URAD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Disable Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icm_idr](index.html) module"]
pub struct ICM_IDR_SPEC;
impl crate::RegisterSpec for ICM_IDR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [icm_idr::W](W) writer structure"]
impl crate::Writable for ICM_IDR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ICM_IDR to value 0"]
impl crate::Resettable for ICM_IDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
