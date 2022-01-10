#[doc = "Register `VREF` reader"]
pub struct R(crate::R<VREF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VREF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VREF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VREF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `VREF` writer"]
pub struct W(crate::W<VREF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<VREF_SPEC>;
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
impl From<crate::W<VREF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<VREF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TSEN` reader - Temperature Sensor Enable"]
pub struct TSEN_R(crate::FieldReader<bool, bool>);
impl TSEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TSEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSEN` writer - Temperature Sensor Enable"]
pub struct TSEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TSEN_W<'a> {
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
#[doc = "Field `BGOUTEN` reader - Bandgap Output Enable"]
pub struct BGOUTEN_R(crate::FieldReader<bool, bool>);
impl BGOUTEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BGOUTEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BGOUTEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BGOUTEN` writer - Bandgap Output Enable"]
pub struct BGOUTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> BGOUTEN_W<'a> {
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
#[doc = "Field `CALIB` reader - Bandgap Voltage Generator Calibration"]
pub struct CALIB_R(crate::FieldReader<u16, u16>);
impl CALIB_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        CALIB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CALIB_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CALIB` writer - Bandgap Voltage Generator Calibration"]
pub struct CALIB_W<'a> {
    w: &'a mut W,
}
impl<'a> CALIB_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07ff << 16)) | ((value as u32 & 0x07ff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bit 1 - Temperature Sensor Enable"]
    #[inline(always)]
    pub fn tsen(&self) -> TSEN_R {
        TSEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Bandgap Output Enable"]
    #[inline(always)]
    pub fn bgouten(&self) -> BGOUTEN_R {
        BGOUTEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 16:26 - Bandgap Voltage Generator Calibration"]
    #[inline(always)]
    pub fn calib(&self) -> CALIB_R {
        CALIB_R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bit 1 - Temperature Sensor Enable"]
    #[inline(always)]
    pub fn tsen(&mut self) -> TSEN_W {
        TSEN_W { w: self }
    }
    #[doc = "Bit 2 - Bandgap Output Enable"]
    #[inline(always)]
    pub fn bgouten(&mut self) -> BGOUTEN_W {
        BGOUTEN_W { w: self }
    }
    #[doc = "Bits 16:26 - Bandgap Voltage Generator Calibration"]
    #[inline(always)]
    pub fn calib(&mut self) -> CALIB_W {
        CALIB_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Voltage References System (VREF) Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vref](index.html) module"]
pub struct VREF_SPEC;
impl crate::RegisterSpec for VREF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [vref::R](R) reader structure"]
impl crate::Readable for VREF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [vref::W](W) writer structure"]
impl crate::Writable for VREF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets VREF to value 0"]
impl crate::Resettable for VREF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
