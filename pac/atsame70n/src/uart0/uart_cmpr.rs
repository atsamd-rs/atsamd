#[doc = "Register `UART_CMPR` reader"]
pub struct R(crate::R<UART_CMPR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UART_CMPR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UART_CMPR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UART_CMPR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UART_CMPR` writer"]
pub struct W(crate::W<UART_CMPR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UART_CMPR_SPEC>;
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
impl From<crate::W<UART_CMPR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UART_CMPR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VAL1` reader - First Comparison Value for Received Character"]
pub struct VAL1_R(crate::FieldReader<u8, u8>);
impl VAL1_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        VAL1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VAL1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VAL1` writer - First Comparison Value for Received Character"]
pub struct VAL1_W<'a> {
    w: &'a mut W,
}
impl<'a> VAL1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Comparison Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMPMODE_A {
    #[doc = "0: Any character is received and comparison function drives CMP flag."]
    FLAG_ONLY = 0,
    #[doc = "1: Comparison condition must be met to start reception."]
    START_CONDITION = 1,
}
impl From<CMPMODE_A> for bool {
    #[inline(always)]
    fn from(variant: CMPMODE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMPMODE` reader - Comparison Mode"]
pub struct CMPMODE_R(crate::FieldReader<bool, CMPMODE_A>);
impl CMPMODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CMPMODE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPMODE_A {
        match self.bits {
            false => CMPMODE_A::FLAG_ONLY,
            true => CMPMODE_A::START_CONDITION,
        }
    }
    #[doc = "Checks if the value of the field is `FLAG_ONLY`"]
    #[inline(always)]
    pub fn is_flag_only(&self) -> bool {
        **self == CMPMODE_A::FLAG_ONLY
    }
    #[doc = "Checks if the value of the field is `START_CONDITION`"]
    #[inline(always)]
    pub fn is_start_condition(&self) -> bool {
        **self == CMPMODE_A::START_CONDITION
    }
}
impl core::ops::Deref for CMPMODE_R {
    type Target = crate::FieldReader<bool, CMPMODE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMPMODE` writer - Comparison Mode"]
pub struct CMPMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPMODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CMPMODE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Any character is received and comparison function drives CMP flag."]
    #[inline(always)]
    pub fn flag_only(self) -> &'a mut W {
        self.variant(CMPMODE_A::FLAG_ONLY)
    }
    #[doc = "Comparison condition must be met to start reception."]
    #[inline(always)]
    pub fn start_condition(self) -> &'a mut W {
        self.variant(CMPMODE_A::START_CONDITION)
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "Field `CMPPAR` reader - Compare Parity"]
pub struct CMPPAR_R(crate::FieldReader<bool, bool>);
impl CMPPAR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CMPPAR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMPPAR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMPPAR` writer - Compare Parity"]
pub struct CMPPAR_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPPAR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
#[doc = "Field `VAL2` reader - Second Comparison Value for Received Character"]
pub struct VAL2_R(crate::FieldReader<u8, u8>);
impl VAL2_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        VAL2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VAL2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VAL2` writer - Second Comparison Value for Received Character"]
pub struct VAL2_W<'a> {
    w: &'a mut W,
}
impl<'a> VAL2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - First Comparison Value for Received Character"]
    #[inline(always)]
    pub fn val1(&self) -> VAL1_R {
        VAL1_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 12 - Comparison Mode"]
    #[inline(always)]
    pub fn cmpmode(&self) -> CMPMODE_R {
        CMPMODE_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Compare Parity"]
    #[inline(always)]
    pub fn cmppar(&self) -> CMPPAR_R {
        CMPPAR_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bits 16:23 - Second Comparison Value for Received Character"]
    #[inline(always)]
    pub fn val2(&self) -> VAL2_R {
        VAL2_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - First Comparison Value for Received Character"]
    #[inline(always)]
    pub fn val1(&mut self) -> VAL1_W {
        VAL1_W { w: self }
    }
    #[doc = "Bit 12 - Comparison Mode"]
    #[inline(always)]
    pub fn cmpmode(&mut self) -> CMPMODE_W {
        CMPMODE_W { w: self }
    }
    #[doc = "Bit 14 - Compare Parity"]
    #[inline(always)]
    pub fn cmppar(&mut self) -> CMPPAR_W {
        CMPPAR_W { w: self }
    }
    #[doc = "Bits 16:23 - Second Comparison Value for Received Character"]
    #[inline(always)]
    pub fn val2(&mut self) -> VAL2_W {
        VAL2_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Comparison Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart_cmpr](index.html) module"]
pub struct UART_CMPR_SPEC;
impl crate::RegisterSpec for UART_CMPR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uart_cmpr::R](R) reader structure"]
impl crate::Readable for UART_CMPR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uart_cmpr::W](W) writer structure"]
impl crate::Writable for UART_CMPR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UART_CMPR to value 0"]
impl crate::Resettable for UART_CMPR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
