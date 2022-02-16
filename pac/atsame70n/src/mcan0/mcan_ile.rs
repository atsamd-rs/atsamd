#[doc = "Register `MCAN_ILE` reader"]
pub struct R(crate::R<MCAN_ILE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MCAN_ILE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MCAN_ILE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MCAN_ILE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MCAN_ILE` writer"]
pub struct W(crate::W<MCAN_ILE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MCAN_ILE_SPEC>;
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
impl From<crate::W<MCAN_ILE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MCAN_ILE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EINT0` reader - Enable Interrupt Line 0"]
pub struct EINT0_R(crate::FieldReader<bool, bool>);
impl EINT0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EINT0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EINT0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EINT0` writer - Enable Interrupt Line 0"]
pub struct EINT0_W<'a> {
    w: &'a mut W,
}
impl<'a> EINT0_W<'a> {
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
#[doc = "Field `EINT1` reader - Enable Interrupt Line 1"]
pub struct EINT1_R(crate::FieldReader<bool, bool>);
impl EINT1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EINT1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EINT1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EINT1` writer - Enable Interrupt Line 1"]
pub struct EINT1_W<'a> {
    w: &'a mut W,
}
impl<'a> EINT1_W<'a> {
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
    #[doc = "Bit 0 - Enable Interrupt Line 0"]
    #[inline(always)]
    pub fn eint0(&self) -> EINT0_R {
        EINT0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Enable Interrupt Line 1"]
    #[inline(always)]
    pub fn eint1(&self) -> EINT1_R {
        EINT1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable Interrupt Line 0"]
    #[inline(always)]
    pub fn eint0(&mut self) -> EINT0_W {
        EINT0_W { w: self }
    }
    #[doc = "Bit 1 - Enable Interrupt Line 1"]
    #[inline(always)]
    pub fn eint1(&mut self) -> EINT1_W {
        EINT1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Line Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcan_ile](index.html) module"]
pub struct MCAN_ILE_SPEC;
impl crate::RegisterSpec for MCAN_ILE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mcan_ile::R](R) reader structure"]
impl crate::Readable for MCAN_ILE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mcan_ile::W](W) writer structure"]
impl crate::Writable for MCAN_ILE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MCAN_ILE to value 0"]
impl crate::Resettable for MCAN_ILE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
