#[doc = "Register `TC_EMR` reader"]
pub struct R(crate::R<TC_EMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TC_EMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TC_EMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TC_EMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TC_EMR` writer"]
pub struct W(crate::W<TC_EMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TC_EMR_SPEC>;
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
impl From<crate::W<TC_EMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TC_EMR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Trigger Source for Input A\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TRIGSRCA_A {
    #[doc = "0: The trigger/capture input A is driven by external pin TIOAx"]
    EXTERNAL_TIOAX = 0,
    #[doc = "1: The trigger/capture input A is driven internally by PWMx"]
    PWMX = 1,
}
impl From<TRIGSRCA_A> for u8 {
    #[inline(always)]
    fn from(variant: TRIGSRCA_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TRIGSRCA` reader - Trigger Source for Input A"]
pub struct TRIGSRCA_R(crate::FieldReader<u8, TRIGSRCA_A>);
impl TRIGSRCA_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TRIGSRCA_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TRIGSRCA_A> {
        match self.bits {
            0 => Some(TRIGSRCA_A::EXTERNAL_TIOAX),
            1 => Some(TRIGSRCA_A::PWMX),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `EXTERNAL_TIOAX`"]
    #[inline(always)]
    pub fn is_external_tioax(&self) -> bool {
        **self == TRIGSRCA_A::EXTERNAL_TIOAX
    }
    #[doc = "Checks if the value of the field is `PWMX`"]
    #[inline(always)]
    pub fn is_pwmx(&self) -> bool {
        **self == TRIGSRCA_A::PWMX
    }
}
impl core::ops::Deref for TRIGSRCA_R {
    type Target = crate::FieldReader<u8, TRIGSRCA_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRIGSRCA` writer - Trigger Source for Input A"]
pub struct TRIGSRCA_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIGSRCA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TRIGSRCA_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "The trigger/capture input A is driven by external pin TIOAx"]
    #[inline(always)]
    pub fn external_tioax(self) -> &'a mut W {
        self.variant(TRIGSRCA_A::EXTERNAL_TIOAX)
    }
    #[doc = "The trigger/capture input A is driven internally by PWMx"]
    #[inline(always)]
    pub fn pwmx(self) -> &'a mut W {
        self.variant(TRIGSRCA_A::PWMX)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "Trigger Source for Input B\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TRIGSRCB_A {
    #[doc = "0: The trigger/capture input B is driven by external pin TIOBx"]
    EXTERNAL_TIOBX = 0,
    #[doc = "1: For TC0 to TC10: The trigger/capture input B is driven internally by the comparator output (see Figure 7-16) of the PWMx.For TC11: The trigger/capture input B is driven internally by the GTSUCOMP signal of the Ethernet MAC (GMAC)."]
    PWMX = 1,
}
impl From<TRIGSRCB_A> for u8 {
    #[inline(always)]
    fn from(variant: TRIGSRCB_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TRIGSRCB` reader - Trigger Source for Input B"]
pub struct TRIGSRCB_R(crate::FieldReader<u8, TRIGSRCB_A>);
impl TRIGSRCB_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TRIGSRCB_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TRIGSRCB_A> {
        match self.bits {
            0 => Some(TRIGSRCB_A::EXTERNAL_TIOBX),
            1 => Some(TRIGSRCB_A::PWMX),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `EXTERNAL_TIOBX`"]
    #[inline(always)]
    pub fn is_external_tiobx(&self) -> bool {
        **self == TRIGSRCB_A::EXTERNAL_TIOBX
    }
    #[doc = "Checks if the value of the field is `PWMX`"]
    #[inline(always)]
    pub fn is_pwmx(&self) -> bool {
        **self == TRIGSRCB_A::PWMX
    }
}
impl core::ops::Deref for TRIGSRCB_R {
    type Target = crate::FieldReader<u8, TRIGSRCB_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRIGSRCB` writer - Trigger Source for Input B"]
pub struct TRIGSRCB_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIGSRCB_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TRIGSRCB_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "The trigger/capture input B is driven by external pin TIOBx"]
    #[inline(always)]
    pub fn external_tiobx(self) -> &'a mut W {
        self.variant(TRIGSRCB_A::EXTERNAL_TIOBX)
    }
    #[doc = "For TC0 to TC10: The trigger/capture input B is driven internally by the comparator output (see Figure 7-16) of the PWMx.For TC11: The trigger/capture input B is driven internally by the GTSUCOMP signal of the Ethernet MAC (GMAC)."]
    #[inline(always)]
    pub fn pwmx(self) -> &'a mut W {
        self.variant(TRIGSRCB_A::PWMX)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
#[doc = "Field `NODIVCLK` reader - No Divided Clock"]
pub struct NODIVCLK_R(crate::FieldReader<bool, bool>);
impl NODIVCLK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        NODIVCLK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NODIVCLK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NODIVCLK` writer - No Divided Clock"]
pub struct NODIVCLK_W<'a> {
    w: &'a mut W,
}
impl<'a> NODIVCLK_W<'a> {
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
impl R {
    #[doc = "Bits 0:1 - Trigger Source for Input A"]
    #[inline(always)]
    pub fn trigsrca(&self) -> TRIGSRCA_R {
        TRIGSRCA_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - Trigger Source for Input B"]
    #[inline(always)]
    pub fn trigsrcb(&self) -> TRIGSRCB_R {
        TRIGSRCB_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bit 8 - No Divided Clock"]
    #[inline(always)]
    pub fn nodivclk(&self) -> NODIVCLK_R {
        NODIVCLK_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Trigger Source for Input A"]
    #[inline(always)]
    pub fn trigsrca(&mut self) -> TRIGSRCA_W {
        TRIGSRCA_W { w: self }
    }
    #[doc = "Bits 4:5 - Trigger Source for Input B"]
    #[inline(always)]
    pub fn trigsrcb(&mut self) -> TRIGSRCB_W {
        TRIGSRCB_W { w: self }
    }
    #[doc = "Bit 8 - No Divided Clock"]
    #[inline(always)]
    pub fn nodivclk(&mut self) -> NODIVCLK_W {
        NODIVCLK_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Extended Mode Register (channel = 0)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tc_emr](index.html) module"]
pub struct TC_EMR_SPEC;
impl crate::RegisterSpec for TC_EMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tc_emr::R](R) reader structure"]
impl crate::Readable for TC_EMR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tc_emr::W](W) writer structure"]
impl crate::Writable for TC_EMR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TC_EMR to value 0"]
impl crate::Resettable for TC_EMR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
