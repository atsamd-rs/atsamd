#[doc = "Register `CCR` reader"]
pub struct R(crate::R<CCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CCR` writer"]
pub struct W(crate::W<CCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CCR_SPEC>;
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
impl From<crate::W<CCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Internal Clock Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INTCLKEN_A {
    #[doc = "0: Stop"]
    OFF = 0,
    #[doc = "1: Oscillate"]
    ON = 1,
}
impl From<INTCLKEN_A> for bool {
    #[inline(always)]
    fn from(variant: INTCLKEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INTCLKEN` reader - Internal Clock Enable"]
pub struct INTCLKEN_R(crate::FieldReader<bool, INTCLKEN_A>);
impl INTCLKEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INTCLKEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTCLKEN_A {
        match self.bits {
            false => INTCLKEN_A::OFF,
            true => INTCLKEN_A::ON,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        **self == INTCLKEN_A::OFF
    }
    #[doc = "Checks if the value of the field is `ON`"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        **self == INTCLKEN_A::ON
    }
}
impl core::ops::Deref for INTCLKEN_R {
    type Target = crate::FieldReader<bool, INTCLKEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INTCLKEN` writer - Internal Clock Enable"]
pub struct INTCLKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> INTCLKEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INTCLKEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Stop"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(INTCLKEN_A::OFF)
    }
    #[doc = "Oscillate"]
    #[inline(always)]
    pub fn on(self) -> &'a mut W {
        self.variant(INTCLKEN_A::ON)
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
        self.w.bits = (self.w.bits & !0x01) | (value as u16 & 0x01);
        self.w
    }
}
#[doc = "Internal Clock Stable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INTCLKS_A {
    #[doc = "0: Not Ready"]
    NOT_READY = 0,
    #[doc = "1: Ready"]
    READY = 1,
}
impl From<INTCLKS_A> for bool {
    #[inline(always)]
    fn from(variant: INTCLKS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INTCLKS` reader - Internal Clock Stable"]
pub struct INTCLKS_R(crate::FieldReader<bool, INTCLKS_A>);
impl INTCLKS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INTCLKS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTCLKS_A {
        match self.bits {
            false => INTCLKS_A::NOT_READY,
            true => INTCLKS_A::READY,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_READY`"]
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        **self == INTCLKS_A::NOT_READY
    }
    #[doc = "Checks if the value of the field is `READY`"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        **self == INTCLKS_A::READY
    }
}
impl core::ops::Deref for INTCLKS_R {
    type Target = crate::FieldReader<bool, INTCLKS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INTCLKS` writer - Internal Clock Stable"]
pub struct INTCLKS_W<'a> {
    w: &'a mut W,
}
impl<'a> INTCLKS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INTCLKS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Not Ready"]
    #[inline(always)]
    pub fn not_ready(self) -> &'a mut W {
        self.variant(INTCLKS_A::NOT_READY)
    }
    #[doc = "Ready"]
    #[inline(always)]
    pub fn ready(self) -> &'a mut W {
        self.variant(INTCLKS_A::READY)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u16 & 0x01) << 1);
        self.w
    }
}
#[doc = "SD Clock Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SDCLKEN_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Enable"]
    ENABLE = 1,
}
impl From<SDCLKEN_A> for bool {
    #[inline(always)]
    fn from(variant: SDCLKEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SDCLKEN` reader - SD Clock Enable"]
pub struct SDCLKEN_R(crate::FieldReader<bool, SDCLKEN_A>);
impl SDCLKEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SDCLKEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SDCLKEN_A {
        match self.bits {
            false => SDCLKEN_A::DISABLE,
            true => SDCLKEN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == SDCLKEN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == SDCLKEN_A::ENABLE
    }
}
impl core::ops::Deref for SDCLKEN_R {
    type Target = crate::FieldReader<bool, SDCLKEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SDCLKEN` writer - SD Clock Enable"]
pub struct SDCLKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SDCLKEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SDCLKEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(SDCLKEN_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(SDCLKEN_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u16 & 0x01) << 2);
        self.w
    }
}
#[doc = "Clock Generator Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLKGSEL_A {
    #[doc = "0: Divided Clock Mode"]
    DIV = 0,
    #[doc = "1: Programmable Clock Mode"]
    PROG = 1,
}
impl From<CLKGSEL_A> for bool {
    #[inline(always)]
    fn from(variant: CLKGSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLKGSEL` reader - Clock Generator Select"]
pub struct CLKGSEL_R(crate::FieldReader<bool, CLKGSEL_A>);
impl CLKGSEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CLKGSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLKGSEL_A {
        match self.bits {
            false => CLKGSEL_A::DIV,
            true => CLKGSEL_A::PROG,
        }
    }
    #[doc = "Checks if the value of the field is `DIV`"]
    #[inline(always)]
    pub fn is_div(&self) -> bool {
        **self == CLKGSEL_A::DIV
    }
    #[doc = "Checks if the value of the field is `PROG`"]
    #[inline(always)]
    pub fn is_prog(&self) -> bool {
        **self == CLKGSEL_A::PROG
    }
}
impl core::ops::Deref for CLKGSEL_R {
    type Target = crate::FieldReader<bool, CLKGSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLKGSEL` writer - Clock Generator Select"]
pub struct CLKGSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKGSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLKGSEL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Divided Clock Mode"]
    #[inline(always)]
    pub fn div(self) -> &'a mut W {
        self.variant(CLKGSEL_A::DIV)
    }
    #[doc = "Programmable Clock Mode"]
    #[inline(always)]
    pub fn prog(self) -> &'a mut W {
        self.variant(CLKGSEL_A::PROG)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u16 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `USDCLKFSEL` reader - Upper Bits of SDCLK Frequency Select"]
pub struct USDCLKFSEL_R(crate::FieldReader<u8, u8>);
impl USDCLKFSEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        USDCLKFSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USDCLKFSEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USDCLKFSEL` writer - Upper Bits of SDCLK Frequency Select"]
pub struct USDCLKFSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> USDCLKFSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | ((value as u16 & 0x03) << 6);
        self.w
    }
}
#[doc = "Field `SDCLKFSEL` reader - SDCLK Frequency Select"]
pub struct SDCLKFSEL_R(crate::FieldReader<u8, u8>);
impl SDCLKFSEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SDCLKFSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SDCLKFSEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SDCLKFSEL` writer - SDCLK Frequency Select"]
pub struct SDCLKFSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SDCLKFSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u16 & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Internal Clock Enable"]
    #[inline(always)]
    pub fn intclken(&self) -> INTCLKEN_R {
        INTCLKEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Internal Clock Stable"]
    #[inline(always)]
    pub fn intclks(&self) -> INTCLKS_R {
        INTCLKS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - SD Clock Enable"]
    #[inline(always)]
    pub fn sdclken(&self) -> SDCLKEN_R {
        SDCLKEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Clock Generator Select"]
    #[inline(always)]
    pub fn clkgsel(&self) -> CLKGSEL_R {
        CLKGSEL_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bits 6:7 - Upper Bits of SDCLK Frequency Select"]
    #[inline(always)]
    pub fn usdclkfsel(&self) -> USDCLKFSEL_R {
        USDCLKFSEL_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 8:15 - SDCLK Frequency Select"]
    #[inline(always)]
    pub fn sdclkfsel(&self) -> SDCLKFSEL_R {
        SDCLKFSEL_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Internal Clock Enable"]
    #[inline(always)]
    pub fn intclken(&mut self) -> INTCLKEN_W {
        INTCLKEN_W { w: self }
    }
    #[doc = "Bit 1 - Internal Clock Stable"]
    #[inline(always)]
    pub fn intclks(&mut self) -> INTCLKS_W {
        INTCLKS_W { w: self }
    }
    #[doc = "Bit 2 - SD Clock Enable"]
    #[inline(always)]
    pub fn sdclken(&mut self) -> SDCLKEN_W {
        SDCLKEN_W { w: self }
    }
    #[doc = "Bit 5 - Clock Generator Select"]
    #[inline(always)]
    pub fn clkgsel(&mut self) -> CLKGSEL_W {
        CLKGSEL_W { w: self }
    }
    #[doc = "Bits 6:7 - Upper Bits of SDCLK Frequency Select"]
    #[inline(always)]
    pub fn usdclkfsel(&mut self) -> USDCLKFSEL_W {
        USDCLKFSEL_W { w: self }
    }
    #[doc = "Bits 8:15 - SDCLK Frequency Select"]
    #[inline(always)]
    pub fn sdclkfsel(&mut self) -> SDCLKFSEL_W {
        SDCLKFSEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clock Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccr](index.html) module"]
pub struct CCR_SPEC;
impl crate::RegisterSpec for CCR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [ccr::R](R) reader structure"]
impl crate::Readable for CCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ccr::W](W) writer structure"]
impl crate::Writable for CCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CCR to value 0"]
impl crate::Resettable for CCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
