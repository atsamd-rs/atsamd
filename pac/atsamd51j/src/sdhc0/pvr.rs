#[doc = "Register `PVR[%s]` reader"]
pub struct R(crate::R<PVR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PVR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PVR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PVR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PVR[%s]` writer"]
pub struct W(crate::W<PVR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PVR_SPEC>;
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
impl From<crate::W<PVR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PVR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SDCLKFSEL` reader - SDCLK Frequency Select Value for Initialization"]
pub struct SDCLKFSEL_R(crate::FieldReader<u16, u16>);
impl SDCLKFSEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        SDCLKFSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SDCLKFSEL_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SDCLKFSEL` writer - SDCLK Frequency Select Value for Initialization"]
pub struct SDCLKFSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SDCLKFSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | (value as u16 & 0x03ff);
        self.w
    }
}
#[doc = "Clock Generator Select Value for Initialization\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLKGSEL_A {
    #[doc = "0: Host Controller Ver2.00 Compatible Clock Generator (Divider)"]
    DIV = 0,
    #[doc = "1: Programmable Clock Generator"]
    PROG = 1,
}
impl From<CLKGSEL_A> for bool {
    #[inline(always)]
    fn from(variant: CLKGSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLKGSEL` reader - Clock Generator Select Value for Initialization"]
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
#[doc = "Field `CLKGSEL` writer - Clock Generator Select Value for Initialization"]
pub struct CLKGSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKGSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLKGSEL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Host Controller Ver2.00 Compatible Clock Generator (Divider)"]
    #[inline(always)]
    pub fn div(self) -> &'a mut W {
        self.variant(CLKGSEL_A::DIV)
    }
    #[doc = "Programmable Clock Generator"]
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u16 & 0x01) << 10);
        self.w
    }
}
#[doc = "Driver Strength Select Value for Initialization\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DRVSEL_A {
    #[doc = "0: Driver Type B is Selected"]
    B = 0,
    #[doc = "1: Driver Type A is Selected"]
    A = 1,
    #[doc = "2: Driver Type C is Selected"]
    C = 2,
    #[doc = "3: Driver Type D is Selected"]
    D = 3,
}
impl From<DRVSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: DRVSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `DRVSEL` reader - Driver Strength Select Value for Initialization"]
pub struct DRVSEL_R(crate::FieldReader<u8, DRVSEL_A>);
impl DRVSEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DRVSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DRVSEL_A {
        match self.bits {
            0 => DRVSEL_A::B,
            1 => DRVSEL_A::A,
            2 => DRVSEL_A::C,
            3 => DRVSEL_A::D,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `B`"]
    #[inline(always)]
    pub fn is_b(&self) -> bool {
        **self == DRVSEL_A::B
    }
    #[doc = "Checks if the value of the field is `A`"]
    #[inline(always)]
    pub fn is_a(&self) -> bool {
        **self == DRVSEL_A::A
    }
    #[doc = "Checks if the value of the field is `C`"]
    #[inline(always)]
    pub fn is_c(&self) -> bool {
        **self == DRVSEL_A::C
    }
    #[doc = "Checks if the value of the field is `D`"]
    #[inline(always)]
    pub fn is_d(&self) -> bool {
        **self == DRVSEL_A::D
    }
}
impl core::ops::Deref for DRVSEL_R {
    type Target = crate::FieldReader<u8, DRVSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DRVSEL` writer - Driver Strength Select Value for Initialization"]
pub struct DRVSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> DRVSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DRVSEL_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Driver Type B is Selected"]
    #[inline(always)]
    pub fn b(self) -> &'a mut W {
        self.variant(DRVSEL_A::B)
    }
    #[doc = "Driver Type A is Selected"]
    #[inline(always)]
    pub fn a(self) -> &'a mut W {
        self.variant(DRVSEL_A::A)
    }
    #[doc = "Driver Type C is Selected"]
    #[inline(always)]
    pub fn c(self) -> &'a mut W {
        self.variant(DRVSEL_A::C)
    }
    #[doc = "Driver Type D is Selected"]
    #[inline(always)]
    pub fn d(self) -> &'a mut W {
        self.variant(DRVSEL_A::D)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | ((value as u16 & 0x03) << 14);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:9 - SDCLK Frequency Select Value for Initialization"]
    #[inline(always)]
    pub fn sdclkfsel(&self) -> SDCLKFSEL_R {
        SDCLKFSEL_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 10 - Clock Generator Select Value for Initialization"]
    #[inline(always)]
    pub fn clkgsel(&self) -> CLKGSEL_R {
        CLKGSEL_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bits 14:15 - Driver Strength Select Value for Initialization"]
    #[inline(always)]
    pub fn drvsel(&self) -> DRVSEL_R {
        DRVSEL_R::new(((self.bits >> 14) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:9 - SDCLK Frequency Select Value for Initialization"]
    #[inline(always)]
    pub fn sdclkfsel(&mut self) -> SDCLKFSEL_W {
        SDCLKFSEL_W { w: self }
    }
    #[doc = "Bit 10 - Clock Generator Select Value for Initialization"]
    #[inline(always)]
    pub fn clkgsel(&mut self) -> CLKGSEL_W {
        CLKGSEL_W { w: self }
    }
    #[doc = "Bits 14:15 - Driver Strength Select Value for Initialization"]
    #[inline(always)]
    pub fn drvsel(&mut self) -> DRVSEL_W {
        DRVSEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Preset Value n\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pvr](index.html) module"]
pub struct PVR_SPEC;
impl crate::RegisterSpec for PVR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [pvr::R](R) reader structure"]
impl crate::Readable for PVR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pvr::W](W) writer structure"]
impl crate::Writable for PVR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PVR[%s]
to value 0"]
impl crate::Resettable for PVR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
