#[doc = "Register `APBASEL` reader"]
pub struct R(crate::R<APBASEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APBASEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APBASEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APBASEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `APBASEL` writer"]
pub struct W(crate::W<APBASEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APBASEL_SPEC>;
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
impl From<crate::W<APBASEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APBASEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "APBA Prescaler Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum APBADIV_A {
    #[doc = "0: Divide by 1"]
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
}
impl From<APBADIV_A> for u8 {
    #[inline(always)]
    fn from(variant: APBADIV_A) -> Self {
        variant as _
    }
}
#[doc = "Field `APBADIV` reader - APBA Prescaler Selection"]
pub struct APBADIV_R(crate::FieldReader<u8, APBADIV_A>);
impl APBADIV_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        APBADIV_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> APBADIV_A {
        match self.bits {
            0 => APBADIV_A::DIV1,
            1 => APBADIV_A::DIV2,
            2 => APBADIV_A::DIV4,
            3 => APBADIV_A::DIV8,
            4 => APBADIV_A::DIV16,
            5 => APBADIV_A::DIV32,
            6 => APBADIV_A::DIV64,
            7 => APBADIV_A::DIV128,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIV1`"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        **self == APBADIV_A::DIV1
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        **self == APBADIV_A::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        **self == APBADIV_A::DIV4
    }
    #[doc = "Checks if the value of the field is `DIV8`"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        **self == APBADIV_A::DIV8
    }
    #[doc = "Checks if the value of the field is `DIV16`"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        **self == APBADIV_A::DIV16
    }
    #[doc = "Checks if the value of the field is `DIV32`"]
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        **self == APBADIV_A::DIV32
    }
    #[doc = "Checks if the value of the field is `DIV64`"]
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        **self == APBADIV_A::DIV64
    }
    #[doc = "Checks if the value of the field is `DIV128`"]
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        **self == APBADIV_A::DIV128
    }
}
impl core::ops::Deref for APBADIV_R {
    type Target = crate::FieldReader<u8, APBADIV_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `APBADIV` writer - APBA Prescaler Selection"]
pub struct APBADIV_W<'a> {
    w: &'a mut W,
}
impl<'a> APBADIV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: APBADIV_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Divide by 1"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(APBADIV_A::DIV1)
    }
    #[doc = "Divide by 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(APBADIV_A::DIV2)
    }
    #[doc = "Divide by 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(APBADIV_A::DIV4)
    }
    #[doc = "Divide by 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(APBADIV_A::DIV8)
    }
    #[doc = "Divide by 16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut W {
        self.variant(APBADIV_A::DIV16)
    }
    #[doc = "Divide by 32"]
    #[inline(always)]
    pub fn div32(self) -> &'a mut W {
        self.variant(APBADIV_A::DIV32)
    }
    #[doc = "Divide by 64"]
    #[inline(always)]
    pub fn div64(self) -> &'a mut W {
        self.variant(APBADIV_A::DIV64)
    }
    #[doc = "Divide by 128"]
    #[inline(always)]
    pub fn div128(self) -> &'a mut W {
        self.variant(APBADIV_A::DIV128)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u8 & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - APBA Prescaler Selection"]
    #[inline(always)]
    pub fn apbadiv(&self) -> APBADIV_R {
        APBADIV_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - APBA Prescaler Selection"]
    #[inline(always)]
    pub fn apbadiv(&mut self) -> APBADIV_W {
        APBADIV_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "APBA Clock Select\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apbasel](index.html) module"]
pub struct APBASEL_SPEC;
impl crate::RegisterSpec for APBASEL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [apbasel::R](R) reader structure"]
impl crate::Readable for APBASEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [apbasel::W](W) writer structure"]
impl crate::Writable for APBASEL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets APBASEL to value 0"]
impl crate::Resettable for APBASEL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
