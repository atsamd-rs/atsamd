#[doc = "Register `QSPI_SMR` reader"]
pub struct R(crate::R<QSPI_SMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<QSPI_SMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<QSPI_SMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<QSPI_SMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `QSPI_SMR` writer"]
pub struct W(crate::W<QSPI_SMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<QSPI_SMR_SPEC>;
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
impl From<crate::W<QSPI_SMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<QSPI_SMR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Scrambling/Unscrambling Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SCREN_A {
    #[doc = "0: The scrambling/unscrambling is disabled."]
    DISABLED = 0,
    #[doc = "1: The scrambling/unscrambling is enabled."]
    ENABLED = 1,
}
impl From<SCREN_A> for bool {
    #[inline(always)]
    fn from(variant: SCREN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SCREN` reader - Scrambling/Unscrambling Enable"]
pub struct SCREN_R(crate::FieldReader<bool, SCREN_A>);
impl SCREN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SCREN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SCREN_A {
        match self.bits {
            false => SCREN_A::DISABLED,
            true => SCREN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == SCREN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == SCREN_A::ENABLED
    }
}
impl core::ops::Deref for SCREN_R {
    type Target = crate::FieldReader<bool, SCREN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCREN` writer - Scrambling/Unscrambling Enable"]
pub struct SCREN_W<'a> {
    w: &'a mut W,
}
impl<'a> SCREN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SCREN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The scrambling/unscrambling is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SCREN_A::DISABLED)
    }
    #[doc = "The scrambling/unscrambling is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SCREN_A::ENABLED)
    }
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
#[doc = "Field `RVDIS` reader - Scrambling/Unscrambling Random Value Disable"]
pub struct RVDIS_R(crate::FieldReader<bool, bool>);
impl RVDIS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RVDIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RVDIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RVDIS` writer - Scrambling/Unscrambling Random Value Disable"]
pub struct RVDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> RVDIS_W<'a> {
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
    pub fn scren(&self) -> SCREN_R {
        SCREN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Scrambling/Unscrambling Random Value Disable"]
    #[inline(always)]
    pub fn rvdis(&self) -> RVDIS_R {
        RVDIS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Scrambling/Unscrambling Enable"]
    #[inline(always)]
    pub fn scren(&mut self) -> SCREN_W {
        SCREN_W { w: self }
    }
    #[doc = "Bit 1 - Scrambling/Unscrambling Random Value Disable"]
    #[inline(always)]
    pub fn rvdis(&mut self) -> RVDIS_W {
        RVDIS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Scrambling Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [qspi_smr](index.html) module"]
pub struct QSPI_SMR_SPEC;
impl crate::RegisterSpec for QSPI_SMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [qspi_smr::R](R) reader structure"]
impl crate::Readable for QSPI_SMR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [qspi_smr::W](W) writer structure"]
impl crate::Writable for QSPI_SMR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets QSPI_SMR to value 0"]
impl crate::Resettable for QSPI_SMR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
