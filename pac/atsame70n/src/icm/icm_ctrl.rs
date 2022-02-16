#[doc = "Register `ICM_CTRL` writer"]
pub struct W(crate::W<ICM_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ICM_CTRL_SPEC>;
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
impl From<crate::W<ICM_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ICM_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENABLE` writer - ICM Enable"]
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
#[doc = "Field `DISABLE` writer - ICM Disable Register"]
pub struct DISABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> DISABLE_W<'a> {
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
#[doc = "Field `SWRST` writer - Software Reset"]
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `REHASH` writer - Recompute Internal Hash"]
pub struct REHASH_W<'a> {
    w: &'a mut W,
}
impl<'a> REHASH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | ((value as u32 & 0x0f) << 4);
        self.w
    }
}
#[doc = "Field `RMDIS` writer - Region Monitoring Disable"]
pub struct RMDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> RMDIS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
#[doc = "Field `RMEN` writer - Region Monitoring Enable"]
pub struct RMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RMEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | ((value as u32 & 0x0f) << 12);
        self.w
    }
}
impl W {
    #[doc = "Bit 0 - ICM Enable"]
    #[inline(always)]
    pub fn enable(&mut self) -> ENABLE_W {
        ENABLE_W { w: self }
    }
    #[doc = "Bit 1 - ICM Disable Register"]
    #[inline(always)]
    pub fn disable(&mut self) -> DISABLE_W {
        DISABLE_W { w: self }
    }
    #[doc = "Bit 2 - Software Reset"]
    #[inline(always)]
    pub fn swrst(&mut self) -> SWRST_W {
        SWRST_W { w: self }
    }
    #[doc = "Bits 4:7 - Recompute Internal Hash"]
    #[inline(always)]
    pub fn rehash(&mut self) -> REHASH_W {
        REHASH_W { w: self }
    }
    #[doc = "Bits 8:11 - Region Monitoring Disable"]
    #[inline(always)]
    pub fn rmdis(&mut self) -> RMDIS_W {
        RMDIS_W { w: self }
    }
    #[doc = "Bits 12:15 - Region Monitoring Enable"]
    #[inline(always)]
    pub fn rmen(&mut self) -> RMEN_W {
        RMEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icm_ctrl](index.html) module"]
pub struct ICM_CTRL_SPEC;
impl crate::RegisterSpec for ICM_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [icm_ctrl::W](W) writer structure"]
impl crate::Writable for ICM_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ICM_CTRL to value 0"]
impl crate::Resettable for ICM_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
