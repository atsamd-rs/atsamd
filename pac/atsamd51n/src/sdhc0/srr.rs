#[doc = "Register `SRR` reader"]
pub struct R(crate::R<SRR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SRR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SRR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SRR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SRR` writer"]
pub struct W(crate::W<SRR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SRR_SPEC>;
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
impl From<crate::W<SRR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SRR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Software Reset For All\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWRSTALL_A {
    #[doc = "0: Work"]
    WORK = 0,
    #[doc = "1: Reset"]
    RESET = 1,
}
impl From<SWRSTALL_A> for bool {
    #[inline(always)]
    fn from(variant: SWRSTALL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWRSTALL` reader - Software Reset For All"]
pub struct SWRSTALL_R(crate::FieldReader<bool, SWRSTALL_A>);
impl SWRSTALL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SWRSTALL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SWRSTALL_A {
        match self.bits {
            false => SWRSTALL_A::WORK,
            true => SWRSTALL_A::RESET,
        }
    }
    #[doc = "Checks if the value of the field is `WORK`"]
    #[inline(always)]
    pub fn is_work(&self) -> bool {
        **self == SWRSTALL_A::WORK
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        **self == SWRSTALL_A::RESET
    }
}
impl core::ops::Deref for SWRSTALL_R {
    type Target = crate::FieldReader<bool, SWRSTALL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWRSTALL` writer - Software Reset For All"]
pub struct SWRSTALL_W<'a> {
    w: &'a mut W,
}
impl<'a> SWRSTALL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SWRSTALL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Work"]
    #[inline(always)]
    pub fn work(self) -> &'a mut W {
        self.variant(SWRSTALL_A::WORK)
    }
    #[doc = "Reset"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(SWRSTALL_A::RESET)
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
        self.w.bits = (self.w.bits & !0x01) | (value as u8 & 0x01);
        self.w
    }
}
#[doc = "Software Reset For CMD Line\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWRSTCMD_A {
    #[doc = "0: Work"]
    WORK = 0,
    #[doc = "1: Reset"]
    RESET = 1,
}
impl From<SWRSTCMD_A> for bool {
    #[inline(always)]
    fn from(variant: SWRSTCMD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWRSTCMD` reader - Software Reset For CMD Line"]
pub struct SWRSTCMD_R(crate::FieldReader<bool, SWRSTCMD_A>);
impl SWRSTCMD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SWRSTCMD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SWRSTCMD_A {
        match self.bits {
            false => SWRSTCMD_A::WORK,
            true => SWRSTCMD_A::RESET,
        }
    }
    #[doc = "Checks if the value of the field is `WORK`"]
    #[inline(always)]
    pub fn is_work(&self) -> bool {
        **self == SWRSTCMD_A::WORK
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        **self == SWRSTCMD_A::RESET
    }
}
impl core::ops::Deref for SWRSTCMD_R {
    type Target = crate::FieldReader<bool, SWRSTCMD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWRSTCMD` writer - Software Reset For CMD Line"]
pub struct SWRSTCMD_W<'a> {
    w: &'a mut W,
}
impl<'a> SWRSTCMD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SWRSTCMD_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Work"]
    #[inline(always)]
    pub fn work(self) -> &'a mut W {
        self.variant(SWRSTCMD_A::WORK)
    }
    #[doc = "Reset"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(SWRSTCMD_A::RESET)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u8 & 0x01) << 1);
        self.w
    }
}
#[doc = "Software Reset For DAT Line\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWRSTDAT_A {
    #[doc = "0: Work"]
    WORK = 0,
    #[doc = "1: Reset"]
    RESET = 1,
}
impl From<SWRSTDAT_A> for bool {
    #[inline(always)]
    fn from(variant: SWRSTDAT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWRSTDAT` reader - Software Reset For DAT Line"]
pub struct SWRSTDAT_R(crate::FieldReader<bool, SWRSTDAT_A>);
impl SWRSTDAT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SWRSTDAT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SWRSTDAT_A {
        match self.bits {
            false => SWRSTDAT_A::WORK,
            true => SWRSTDAT_A::RESET,
        }
    }
    #[doc = "Checks if the value of the field is `WORK`"]
    #[inline(always)]
    pub fn is_work(&self) -> bool {
        **self == SWRSTDAT_A::WORK
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        **self == SWRSTDAT_A::RESET
    }
}
impl core::ops::Deref for SWRSTDAT_R {
    type Target = crate::FieldReader<bool, SWRSTDAT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWRSTDAT` writer - Software Reset For DAT Line"]
pub struct SWRSTDAT_W<'a> {
    w: &'a mut W,
}
impl<'a> SWRSTDAT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SWRSTDAT_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Work"]
    #[inline(always)]
    pub fn work(self) -> &'a mut W {
        self.variant(SWRSTDAT_A::WORK)
    }
    #[doc = "Reset"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(SWRSTDAT_A::RESET)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u8 & 0x01) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Software Reset For All"]
    #[inline(always)]
    pub fn swrstall(&self) -> SWRSTALL_R {
        SWRSTALL_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Software Reset For CMD Line"]
    #[inline(always)]
    pub fn swrstcmd(&self) -> SWRSTCMD_R {
        SWRSTCMD_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Software Reset For DAT Line"]
    #[inline(always)]
    pub fn swrstdat(&self) -> SWRSTDAT_R {
        SWRSTDAT_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Software Reset For All"]
    #[inline(always)]
    pub fn swrstall(&mut self) -> SWRSTALL_W {
        SWRSTALL_W { w: self }
    }
    #[doc = "Bit 1 - Software Reset For CMD Line"]
    #[inline(always)]
    pub fn swrstcmd(&mut self) -> SWRSTCMD_W {
        SWRSTCMD_W { w: self }
    }
    #[doc = "Bit 2 - Software Reset For DAT Line"]
    #[inline(always)]
    pub fn swrstdat(&mut self) -> SWRSTDAT_W {
        SWRSTDAT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Software Reset\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [srr](index.html) module"]
pub struct SRR_SPEC;
impl crate::RegisterSpec for SRR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [srr::R](R) reader structure"]
impl crate::Readable for SRR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [srr::W](W) writer structure"]
impl crate::Writable for SRR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SRR to value 0"]
impl crate::Resettable for SRR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
