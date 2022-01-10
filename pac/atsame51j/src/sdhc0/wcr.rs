#[doc = "Register `WCR` reader"]
pub struct R(crate::R<WCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WCR` writer"]
pub struct W(crate::W<WCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WCR_SPEC>;
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
impl From<crate::W<WCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Wakeup Event Enable on Card Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKENCINT_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Enable"]
    ENABLE = 1,
}
impl From<WKENCINT_A> for bool {
    #[inline(always)]
    fn from(variant: WKENCINT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKENCINT` reader - Wakeup Event Enable on Card Interrupt"]
pub struct WKENCINT_R(crate::FieldReader<bool, WKENCINT_A>);
impl WKENCINT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WKENCINT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WKENCINT_A {
        match self.bits {
            false => WKENCINT_A::DISABLE,
            true => WKENCINT_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == WKENCINT_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == WKENCINT_A::ENABLE
    }
}
impl core::ops::Deref for WKENCINT_R {
    type Target = crate::FieldReader<bool, WKENCINT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WKENCINT` writer - Wakeup Event Enable on Card Interrupt"]
pub struct WKENCINT_W<'a> {
    w: &'a mut W,
}
impl<'a> WKENCINT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WKENCINT_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(WKENCINT_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(WKENCINT_A::ENABLE)
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
#[doc = "Wakeup Event Enable on Card Insertion\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKENCINS_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Enable"]
    ENABLE = 1,
}
impl From<WKENCINS_A> for bool {
    #[inline(always)]
    fn from(variant: WKENCINS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKENCINS` reader - Wakeup Event Enable on Card Insertion"]
pub struct WKENCINS_R(crate::FieldReader<bool, WKENCINS_A>);
impl WKENCINS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WKENCINS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WKENCINS_A {
        match self.bits {
            false => WKENCINS_A::DISABLE,
            true => WKENCINS_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == WKENCINS_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == WKENCINS_A::ENABLE
    }
}
impl core::ops::Deref for WKENCINS_R {
    type Target = crate::FieldReader<bool, WKENCINS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WKENCINS` writer - Wakeup Event Enable on Card Insertion"]
pub struct WKENCINS_W<'a> {
    w: &'a mut W,
}
impl<'a> WKENCINS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WKENCINS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(WKENCINS_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(WKENCINS_A::ENABLE)
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
#[doc = "Wakeup Event Enable on Card Removal\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKENCREM_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Enable"]
    ENABLE = 1,
}
impl From<WKENCREM_A> for bool {
    #[inline(always)]
    fn from(variant: WKENCREM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKENCREM` reader - Wakeup Event Enable on Card Removal"]
pub struct WKENCREM_R(crate::FieldReader<bool, WKENCREM_A>);
impl WKENCREM_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WKENCREM_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WKENCREM_A {
        match self.bits {
            false => WKENCREM_A::DISABLE,
            true => WKENCREM_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == WKENCREM_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == WKENCREM_A::ENABLE
    }
}
impl core::ops::Deref for WKENCREM_R {
    type Target = crate::FieldReader<bool, WKENCREM_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WKENCREM` writer - Wakeup Event Enable on Card Removal"]
pub struct WKENCREM_W<'a> {
    w: &'a mut W,
}
impl<'a> WKENCREM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WKENCREM_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(WKENCREM_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(WKENCREM_A::ENABLE)
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
    #[doc = "Bit 0 - Wakeup Event Enable on Card Interrupt"]
    #[inline(always)]
    pub fn wkencint(&self) -> WKENCINT_R {
        WKENCINT_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Wakeup Event Enable on Card Insertion"]
    #[inline(always)]
    pub fn wkencins(&self) -> WKENCINS_R {
        WKENCINS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Wakeup Event Enable on Card Removal"]
    #[inline(always)]
    pub fn wkencrem(&self) -> WKENCREM_R {
        WKENCREM_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Wakeup Event Enable on Card Interrupt"]
    #[inline(always)]
    pub fn wkencint(&mut self) -> WKENCINT_W {
        WKENCINT_W { w: self }
    }
    #[doc = "Bit 1 - Wakeup Event Enable on Card Insertion"]
    #[inline(always)]
    pub fn wkencins(&mut self) -> WKENCINS_W {
        WKENCINS_W { w: self }
    }
    #[doc = "Bit 2 - Wakeup Event Enable on Card Removal"]
    #[inline(always)]
    pub fn wkencrem(&mut self) -> WKENCREM_W {
        WKENCREM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Wakeup Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wcr](index.html) module"]
pub struct WCR_SPEC;
impl crate::RegisterSpec for WCR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [wcr::R](R) reader structure"]
impl crate::Readable for WCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wcr::W](W) writer structure"]
impl crate::Writable for WCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WCR to value 0"]
impl crate::Resettable for WCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
