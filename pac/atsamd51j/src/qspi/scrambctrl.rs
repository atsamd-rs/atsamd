#[doc = "Register `SCRAMBCTRL` reader"]
pub struct R(crate::R<SCRAMBCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCRAMBCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCRAMBCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCRAMBCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCRAMBCTRL` writer"]
pub struct W(crate::W<SCRAMBCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCRAMBCTRL_SPEC>;
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
impl From<crate::W<SCRAMBCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCRAMBCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENABLE` reader - Scrambling/Unscrambling Enable"]
pub struct ENABLE_R(crate::FieldReader<bool, bool>);
impl ENABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ENABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENABLE` writer - Scrambling/Unscrambling Enable"]
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
#[doc = "Field `RANDOMDIS` reader - Scrambling/Unscrambling Random Value Disable"]
pub struct RANDOMDIS_R(crate::FieldReader<bool, bool>);
impl RANDOMDIS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RANDOMDIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RANDOMDIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RANDOMDIS` writer - Scrambling/Unscrambling Random Value Disable"]
pub struct RANDOMDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> RANDOMDIS_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Scrambling/Unscrambling Enable"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Scrambling/Unscrambling Random Value Disable"]
    #[inline(always)]
    pub fn randomdis(&self) -> RANDOMDIS_R {
        RANDOMDIS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Scrambling/Unscrambling Enable"]
    #[inline(always)]
    pub fn enable(&mut self) -> ENABLE_W {
        ENABLE_W { w: self }
    }
    #[doc = "Bit 1 - Scrambling/Unscrambling Random Value Disable"]
    #[inline(always)]
    pub fn randomdis(&mut self) -> RANDOMDIS_W {
        RANDOMDIS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Scrambling Mode\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scrambctrl](index.html) module"]
pub struct SCRAMBCTRL_SPEC;
impl crate::RegisterSpec for SCRAMBCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [scrambctrl::R](R) reader structure"]
impl crate::Readable for SCRAMBCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [scrambctrl::W](W) writer structure"]
impl crate::Writable for SCRAMBCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SCRAMBCTRL to value 0"]
impl crate::Resettable for SCRAMBCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
