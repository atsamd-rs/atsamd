#[doc = "Register `OSC48MSTUP` reader"]
pub struct R(crate::R<OSC48MSTUP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OSC48MSTUP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OSC48MSTUP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OSC48MSTUP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OSC48MSTUP` writer"]
pub struct W(crate::W<OSC48MSTUP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OSC48MSTUP_SPEC>;
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
impl From<crate::W<OSC48MSTUP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OSC48MSTUP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Startup Time\n\nValue on reset: 7"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum STARTUP_A {
    #[doc = "0: 166 ns"]
    CYCLE8 = 0,
    #[doc = "1: 333 ns"]
    CYCLE16 = 1,
    #[doc = "2: 667 ns"]
    CYCLE32 = 2,
    #[doc = "3: 1.333 us"]
    CYCLE64 = 3,
    #[doc = "4: 2.667 us"]
    CYCLE128 = 4,
    #[doc = "5: 5.333 us"]
    CYCLE256 = 5,
    #[doc = "6: 10.667 us"]
    CYCLE512 = 6,
    #[doc = "7: 21.333 us"]
    CYCLE1024 = 7,
}
impl From<STARTUP_A> for u8 {
    #[inline(always)]
    fn from(variant: STARTUP_A) -> Self {
        variant as _
    }
}
#[doc = "Field `STARTUP` reader - Startup Time"]
pub struct STARTUP_R(crate::FieldReader<u8, STARTUP_A>);
impl STARTUP_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        STARTUP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STARTUP_A {
        match self.bits {
            0 => STARTUP_A::CYCLE8,
            1 => STARTUP_A::CYCLE16,
            2 => STARTUP_A::CYCLE32,
            3 => STARTUP_A::CYCLE64,
            4 => STARTUP_A::CYCLE128,
            5 => STARTUP_A::CYCLE256,
            6 => STARTUP_A::CYCLE512,
            7 => STARTUP_A::CYCLE1024,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CYCLE8`"]
    #[inline(always)]
    pub fn is_cycle8(&self) -> bool {
        **self == STARTUP_A::CYCLE8
    }
    #[doc = "Checks if the value of the field is `CYCLE16`"]
    #[inline(always)]
    pub fn is_cycle16(&self) -> bool {
        **self == STARTUP_A::CYCLE16
    }
    #[doc = "Checks if the value of the field is `CYCLE32`"]
    #[inline(always)]
    pub fn is_cycle32(&self) -> bool {
        **self == STARTUP_A::CYCLE32
    }
    #[doc = "Checks if the value of the field is `CYCLE64`"]
    #[inline(always)]
    pub fn is_cycle64(&self) -> bool {
        **self == STARTUP_A::CYCLE64
    }
    #[doc = "Checks if the value of the field is `CYCLE128`"]
    #[inline(always)]
    pub fn is_cycle128(&self) -> bool {
        **self == STARTUP_A::CYCLE128
    }
    #[doc = "Checks if the value of the field is `CYCLE256`"]
    #[inline(always)]
    pub fn is_cycle256(&self) -> bool {
        **self == STARTUP_A::CYCLE256
    }
    #[doc = "Checks if the value of the field is `CYCLE512`"]
    #[inline(always)]
    pub fn is_cycle512(&self) -> bool {
        **self == STARTUP_A::CYCLE512
    }
    #[doc = "Checks if the value of the field is `CYCLE1024`"]
    #[inline(always)]
    pub fn is_cycle1024(&self) -> bool {
        **self == STARTUP_A::CYCLE1024
    }
}
impl core::ops::Deref for STARTUP_R {
    type Target = crate::FieldReader<u8, STARTUP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STARTUP` writer - Startup Time"]
pub struct STARTUP_W<'a> {
    w: &'a mut W,
}
impl<'a> STARTUP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STARTUP_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "166 ns"]
    #[inline(always)]
    pub fn cycle8(self) -> &'a mut W {
        self.variant(STARTUP_A::CYCLE8)
    }
    #[doc = "333 ns"]
    #[inline(always)]
    pub fn cycle16(self) -> &'a mut W {
        self.variant(STARTUP_A::CYCLE16)
    }
    #[doc = "667 ns"]
    #[inline(always)]
    pub fn cycle32(self) -> &'a mut W {
        self.variant(STARTUP_A::CYCLE32)
    }
    #[doc = "1.333 us"]
    #[inline(always)]
    pub fn cycle64(self) -> &'a mut W {
        self.variant(STARTUP_A::CYCLE64)
    }
    #[doc = "2.667 us"]
    #[inline(always)]
    pub fn cycle128(self) -> &'a mut W {
        self.variant(STARTUP_A::CYCLE128)
    }
    #[doc = "5.333 us"]
    #[inline(always)]
    pub fn cycle256(self) -> &'a mut W {
        self.variant(STARTUP_A::CYCLE256)
    }
    #[doc = "10.667 us"]
    #[inline(always)]
    pub fn cycle512(self) -> &'a mut W {
        self.variant(STARTUP_A::CYCLE512)
    }
    #[doc = "21.333 us"]
    #[inline(always)]
    pub fn cycle1024(self) -> &'a mut W {
        self.variant(STARTUP_A::CYCLE1024)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u8 & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Startup Time"]
    #[inline(always)]
    pub fn startup(&self) -> STARTUP_R {
        STARTUP_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Startup Time"]
    #[inline(always)]
    pub fn startup(&mut self) -> STARTUP_W {
        STARTUP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OSC48M Startup Time\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [osc48mstup](index.html) module"]
pub struct OSC48MSTUP_SPEC;
impl crate::RegisterSpec for OSC48MSTUP_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [osc48mstup::R](R) reader structure"]
impl crate::Readable for OSC48MSTUP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [osc48mstup::W](W) writer structure"]
impl crate::Writable for OSC48MSTUP_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OSC48MSTUP to value 0x07"]
impl crate::Resettable for OSC48MSTUP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x07
    }
}
