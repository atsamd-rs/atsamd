#[doc = "Register `PRESCBUF` reader"]
pub struct R(crate::R<PRESCBUF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRESCBUF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRESCBUF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRESCBUF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PRESCBUF` writer"]
pub struct W(crate::W<PRESCBUF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PRESCBUF_SPEC>;
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
impl From<crate::W<PRESCBUF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PRESCBUF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Prescaler Buffer Value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PRESCBUF_A {
    #[doc = "0: No division"]
    DIV1 = 0,
    #[doc = "1: Divide by 2"]
    DIV2 = 1,
    #[doc = "2: Divide by 4"]
    DIV4 = 2,
    #[doc = "3: Divide by 8"]
    DIV8 = 3,
    #[doc = "4: Divide by 16"]
    DIV16 = 4,
    #[doc = "5: Divide by 32"]
    DIV32 = 5,
    #[doc = "6: Divide by 64"]
    DIV64 = 6,
    #[doc = "7: Divide by 128"]
    DIV128 = 7,
    #[doc = "8: Divide by 256"]
    DIV256 = 8,
    #[doc = "9: Divide by 512"]
    DIV512 = 9,
    #[doc = "10: Divide by 1024"]
    DIV1024 = 10,
}
impl From<PRESCBUF_A> for u8 {
    #[inline(always)]
    fn from(variant: PRESCBUF_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PRESCBUF` reader - Prescaler Buffer Value"]
pub struct PRESCBUF_R(crate::FieldReader<u8, PRESCBUF_A>);
impl PRESCBUF_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PRESCBUF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PRESCBUF_A> {
        match self.bits {
            0 => Some(PRESCBUF_A::DIV1),
            1 => Some(PRESCBUF_A::DIV2),
            2 => Some(PRESCBUF_A::DIV4),
            3 => Some(PRESCBUF_A::DIV8),
            4 => Some(PRESCBUF_A::DIV16),
            5 => Some(PRESCBUF_A::DIV32),
            6 => Some(PRESCBUF_A::DIV64),
            7 => Some(PRESCBUF_A::DIV128),
            8 => Some(PRESCBUF_A::DIV256),
            9 => Some(PRESCBUF_A::DIV512),
            10 => Some(PRESCBUF_A::DIV1024),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DIV1`"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        **self == PRESCBUF_A::DIV1
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        **self == PRESCBUF_A::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        **self == PRESCBUF_A::DIV4
    }
    #[doc = "Checks if the value of the field is `DIV8`"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        **self == PRESCBUF_A::DIV8
    }
    #[doc = "Checks if the value of the field is `DIV16`"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        **self == PRESCBUF_A::DIV16
    }
    #[doc = "Checks if the value of the field is `DIV32`"]
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        **self == PRESCBUF_A::DIV32
    }
    #[doc = "Checks if the value of the field is `DIV64`"]
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        **self == PRESCBUF_A::DIV64
    }
    #[doc = "Checks if the value of the field is `DIV128`"]
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        **self == PRESCBUF_A::DIV128
    }
    #[doc = "Checks if the value of the field is `DIV256`"]
    #[inline(always)]
    pub fn is_div256(&self) -> bool {
        **self == PRESCBUF_A::DIV256
    }
    #[doc = "Checks if the value of the field is `DIV512`"]
    #[inline(always)]
    pub fn is_div512(&self) -> bool {
        **self == PRESCBUF_A::DIV512
    }
    #[doc = "Checks if the value of the field is `DIV1024`"]
    #[inline(always)]
    pub fn is_div1024(&self) -> bool {
        **self == PRESCBUF_A::DIV1024
    }
}
impl core::ops::Deref for PRESCBUF_R {
    type Target = crate::FieldReader<u8, PRESCBUF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRESCBUF` writer - Prescaler Buffer Value"]
pub struct PRESCBUF_W<'a> {
    w: &'a mut W,
}
impl<'a> PRESCBUF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PRESCBUF_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "No division"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(PRESCBUF_A::DIV1)
    }
    #[doc = "Divide by 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(PRESCBUF_A::DIV2)
    }
    #[doc = "Divide by 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(PRESCBUF_A::DIV4)
    }
    #[doc = "Divide by 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(PRESCBUF_A::DIV8)
    }
    #[doc = "Divide by 16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut W {
        self.variant(PRESCBUF_A::DIV16)
    }
    #[doc = "Divide by 32"]
    #[inline(always)]
    pub fn div32(self) -> &'a mut W {
        self.variant(PRESCBUF_A::DIV32)
    }
    #[doc = "Divide by 64"]
    #[inline(always)]
    pub fn div64(self) -> &'a mut W {
        self.variant(PRESCBUF_A::DIV64)
    }
    #[doc = "Divide by 128"]
    #[inline(always)]
    pub fn div128(self) -> &'a mut W {
        self.variant(PRESCBUF_A::DIV128)
    }
    #[doc = "Divide by 256"]
    #[inline(always)]
    pub fn div256(self) -> &'a mut W {
        self.variant(PRESCBUF_A::DIV256)
    }
    #[doc = "Divide by 512"]
    #[inline(always)]
    pub fn div512(self) -> &'a mut W {
        self.variant(PRESCBUF_A::DIV512)
    }
    #[doc = "Divide by 1024"]
    #[inline(always)]
    pub fn div1024(self) -> &'a mut W {
        self.variant(PRESCBUF_A::DIV1024)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u8 & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Prescaler Buffer Value"]
    #[inline(always)]
    pub fn prescbuf(&self) -> PRESCBUF_R {
        PRESCBUF_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Prescaler Buffer Value"]
    #[inline(always)]
    pub fn prescbuf(&mut self) -> PRESCBUF_W {
        PRESCBUF_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Prescaler Buffer Value\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prescbuf](index.html) module"]
pub struct PRESCBUF_SPEC;
impl crate::RegisterSpec for PRESCBUF_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [prescbuf::R](R) reader structure"]
impl crate::Readable for PRESCBUF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [prescbuf::W](W) writer structure"]
impl crate::Writable for PRESCBUF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PRESCBUF to value 0"]
impl crate::Resettable for PRESCBUF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
