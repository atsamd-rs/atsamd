#[doc = "Register `CFDPRESC` reader"]
pub struct R(crate::R<CFDPRESC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFDPRESC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFDPRESC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFDPRESC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFDPRESC` writer"]
pub struct W(crate::W<CFDPRESC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFDPRESC_SPEC>;
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
impl From<crate::W<CFDPRESC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFDPRESC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Clock Failure Detector Prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CFDPRESC_A {
    #[doc = "0: 48 MHz"]
    DIV1 = 0,
    #[doc = "1: 24 MHz"]
    DIV2 = 1,
    #[doc = "2: 12 MHz"]
    DIV4 = 2,
    #[doc = "3: 6 MHz"]
    DIV8 = 3,
    #[doc = "4: 3 MHz"]
    DIV16 = 4,
    #[doc = "5: 1.5 MHz"]
    DIV32 = 5,
    #[doc = "6: 0.75 MHz"]
    DIV64 = 6,
    #[doc = "7: 0.3125 MHz"]
    DIV128 = 7,
}
impl From<CFDPRESC_A> for u8 {
    #[inline(always)]
    fn from(variant: CFDPRESC_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CFDPRESC` reader - Clock Failure Detector Prescaler"]
pub struct CFDPRESC_R(crate::FieldReader<u8, CFDPRESC_A>);
impl CFDPRESC_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CFDPRESC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CFDPRESC_A {
        match self.bits {
            0 => CFDPRESC_A::DIV1,
            1 => CFDPRESC_A::DIV2,
            2 => CFDPRESC_A::DIV4,
            3 => CFDPRESC_A::DIV8,
            4 => CFDPRESC_A::DIV16,
            5 => CFDPRESC_A::DIV32,
            6 => CFDPRESC_A::DIV64,
            7 => CFDPRESC_A::DIV128,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIV1`"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        **self == CFDPRESC_A::DIV1
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        **self == CFDPRESC_A::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        **self == CFDPRESC_A::DIV4
    }
    #[doc = "Checks if the value of the field is `DIV8`"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        **self == CFDPRESC_A::DIV8
    }
    #[doc = "Checks if the value of the field is `DIV16`"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        **self == CFDPRESC_A::DIV16
    }
    #[doc = "Checks if the value of the field is `DIV32`"]
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        **self == CFDPRESC_A::DIV32
    }
    #[doc = "Checks if the value of the field is `DIV64`"]
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        **self == CFDPRESC_A::DIV64
    }
    #[doc = "Checks if the value of the field is `DIV128`"]
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        **self == CFDPRESC_A::DIV128
    }
}
impl core::ops::Deref for CFDPRESC_R {
    type Target = crate::FieldReader<u8, CFDPRESC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CFDPRESC` writer - Clock Failure Detector Prescaler"]
pub struct CFDPRESC_W<'a> {
    w: &'a mut W,
}
impl<'a> CFDPRESC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CFDPRESC_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "48 MHz"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(CFDPRESC_A::DIV1)
    }
    #[doc = "24 MHz"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(CFDPRESC_A::DIV2)
    }
    #[doc = "12 MHz"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(CFDPRESC_A::DIV4)
    }
    #[doc = "6 MHz"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(CFDPRESC_A::DIV8)
    }
    #[doc = "3 MHz"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut W {
        self.variant(CFDPRESC_A::DIV16)
    }
    #[doc = "1.5 MHz"]
    #[inline(always)]
    pub fn div32(self) -> &'a mut W {
        self.variant(CFDPRESC_A::DIV32)
    }
    #[doc = "0.75 MHz"]
    #[inline(always)]
    pub fn div64(self) -> &'a mut W {
        self.variant(CFDPRESC_A::DIV64)
    }
    #[doc = "0.3125 MHz"]
    #[inline(always)]
    pub fn div128(self) -> &'a mut W {
        self.variant(CFDPRESC_A::DIV128)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u8 & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Clock Failure Detector Prescaler"]
    #[inline(always)]
    pub fn cfdpresc(&self) -> CFDPRESC_R {
        CFDPRESC_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Clock Failure Detector Prescaler"]
    #[inline(always)]
    pub fn cfdpresc(&mut self) -> CFDPRESC_W {
        CFDPRESC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clock Failure Detector Prescaler\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfdpresc](index.html) module"]
pub struct CFDPRESC_SPEC;
impl crate::RegisterSpec for CFDPRESC_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [cfdpresc::R](R) reader structure"]
impl crate::Readable for CFDPRESC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfdpresc::W](W) writer structure"]
impl crate::Writable for CFDPRESC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CFDPRESC to value 0"]
impl crate::Resettable for CFDPRESC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
