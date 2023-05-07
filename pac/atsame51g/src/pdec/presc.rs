#[doc = "Register `PRESC` reader"]
pub struct R(crate::R<PRESC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRESC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRESC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRESC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PRESC` writer"]
pub struct W(crate::W<PRESC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PRESC_SPEC>;
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
impl From<crate::W<PRESC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PRESC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRESC` reader - Prescaler Value"]
pub type PRESC_R = crate::FieldReader<u8, PRESCSELECT_A>;
#[doc = "Prescaler Value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PRESCSELECT_A {
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
impl From<PRESCSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: PRESCSELECT_A) -> Self {
        variant as _
    }
}
impl PRESC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PRESCSELECT_A> {
        match self.bits {
            0 => Some(PRESCSELECT_A::DIV1),
            1 => Some(PRESCSELECT_A::DIV2),
            2 => Some(PRESCSELECT_A::DIV4),
            3 => Some(PRESCSELECT_A::DIV8),
            4 => Some(PRESCSELECT_A::DIV16),
            5 => Some(PRESCSELECT_A::DIV32),
            6 => Some(PRESCSELECT_A::DIV64),
            7 => Some(PRESCSELECT_A::DIV128),
            8 => Some(PRESCSELECT_A::DIV256),
            9 => Some(PRESCSELECT_A::DIV512),
            10 => Some(PRESCSELECT_A::DIV1024),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DIV1`"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == PRESCSELECT_A::DIV1
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == PRESCSELECT_A::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == PRESCSELECT_A::DIV4
    }
    #[doc = "Checks if the value of the field is `DIV8`"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == PRESCSELECT_A::DIV8
    }
    #[doc = "Checks if the value of the field is `DIV16`"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == PRESCSELECT_A::DIV16
    }
    #[doc = "Checks if the value of the field is `DIV32`"]
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == PRESCSELECT_A::DIV32
    }
    #[doc = "Checks if the value of the field is `DIV64`"]
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == PRESCSELECT_A::DIV64
    }
    #[doc = "Checks if the value of the field is `DIV128`"]
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        *self == PRESCSELECT_A::DIV128
    }
    #[doc = "Checks if the value of the field is `DIV256`"]
    #[inline(always)]
    pub fn is_div256(&self) -> bool {
        *self == PRESCSELECT_A::DIV256
    }
    #[doc = "Checks if the value of the field is `DIV512`"]
    #[inline(always)]
    pub fn is_div512(&self) -> bool {
        *self == PRESCSELECT_A::DIV512
    }
    #[doc = "Checks if the value of the field is `DIV1024`"]
    #[inline(always)]
    pub fn is_div1024(&self) -> bool {
        *self == PRESCSELECT_A::DIV1024
    }
}
#[doc = "Field `PRESC` writer - Prescaler Value"]
pub type PRESC_W<'a, const O: u8> = crate::FieldWriter<'a, u8, PRESC_SPEC, u8, PRESCSELECT_A, 4, O>;
impl<'a, const O: u8> PRESC_W<'a, O> {
    #[doc = "No division"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(PRESCSELECT_A::DIV1)
    }
    #[doc = "Divide by 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(PRESCSELECT_A::DIV2)
    }
    #[doc = "Divide by 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(PRESCSELECT_A::DIV4)
    }
    #[doc = "Divide by 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(PRESCSELECT_A::DIV8)
    }
    #[doc = "Divide by 16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut W {
        self.variant(PRESCSELECT_A::DIV16)
    }
    #[doc = "Divide by 32"]
    #[inline(always)]
    pub fn div32(self) -> &'a mut W {
        self.variant(PRESCSELECT_A::DIV32)
    }
    #[doc = "Divide by 64"]
    #[inline(always)]
    pub fn div64(self) -> &'a mut W {
        self.variant(PRESCSELECT_A::DIV64)
    }
    #[doc = "Divide by 128"]
    #[inline(always)]
    pub fn div128(self) -> &'a mut W {
        self.variant(PRESCSELECT_A::DIV128)
    }
    #[doc = "Divide by 256"]
    #[inline(always)]
    pub fn div256(self) -> &'a mut W {
        self.variant(PRESCSELECT_A::DIV256)
    }
    #[doc = "Divide by 512"]
    #[inline(always)]
    pub fn div512(self) -> &'a mut W {
        self.variant(PRESCSELECT_A::DIV512)
    }
    #[doc = "Divide by 1024"]
    #[inline(always)]
    pub fn div1024(self) -> &'a mut W {
        self.variant(PRESCSELECT_A::DIV1024)
    }
}
impl R {
    #[doc = "Bits 0:3 - Prescaler Value"]
    #[inline(always)]
    pub fn presc(&self) -> PRESC_R {
        PRESC_R::new(self.bits & 0x0f)
    }
}
impl W {
    #[doc = "Bits 0:3 - Prescaler Value"]
    #[inline(always)]
    #[must_use]
    pub fn presc(&mut self) -> PRESC_W<0> {
        PRESC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Prescaler Value\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [presc](index.html) module"]
pub struct PRESC_SPEC;
impl crate::RegisterSpec for PRESC_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [presc::R](R) reader structure"]
impl crate::Readable for PRESC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [presc::W](W) writer structure"]
impl crate::Writable for PRESC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PRESC to value 0"]
impl crate::Resettable for PRESC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
