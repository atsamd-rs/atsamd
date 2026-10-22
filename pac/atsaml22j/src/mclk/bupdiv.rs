#[doc = "Register `BUPDIV` reader"]
pub struct R(crate::R<BUPDIV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BUPDIV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BUPDIV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BUPDIV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BUPDIV` writer"]
pub struct W(crate::W<BUPDIV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BUPDIV_SPEC>;
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
impl From<crate::W<BUPDIV_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BUPDIV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BUPDIV` reader - Backup Clock Division Factor"]
pub type BUPDIV_R = crate::FieldReader<u8, BUPDIVSELECT_A>;
#[doc = "Backup Clock Division Factor\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BUPDIVSELECT_A {
    #[doc = "1: Divide by 1"]
    DIV1 = 1,
    #[doc = "2: Divide by 2"]
    DIV2 = 2,
    #[doc = "4: Divide by 4"]
    DIV4 = 4,
    #[doc = "8: Divide by 8"]
    DIV8 = 8,
    #[doc = "16: Divide by 16"]
    DIV16 = 16,
    #[doc = "32: Divide by 32"]
    DIV32 = 32,
    #[doc = "64: Divide by 64"]
    DIV64 = 64,
    #[doc = "128: Divide by 128"]
    DIV128 = 128,
}
impl From<BUPDIVSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: BUPDIVSELECT_A) -> Self {
        variant as _
    }
}
impl BUPDIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<BUPDIVSELECT_A> {
        match self.bits {
            1 => Some(BUPDIVSELECT_A::DIV1),
            2 => Some(BUPDIVSELECT_A::DIV2),
            4 => Some(BUPDIVSELECT_A::DIV4),
            8 => Some(BUPDIVSELECT_A::DIV8),
            16 => Some(BUPDIVSELECT_A::DIV16),
            32 => Some(BUPDIVSELECT_A::DIV32),
            64 => Some(BUPDIVSELECT_A::DIV64),
            128 => Some(BUPDIVSELECT_A::DIV128),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DIV1`"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == BUPDIVSELECT_A::DIV1
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == BUPDIVSELECT_A::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == BUPDIVSELECT_A::DIV4
    }
    #[doc = "Checks if the value of the field is `DIV8`"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == BUPDIVSELECT_A::DIV8
    }
    #[doc = "Checks if the value of the field is `DIV16`"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == BUPDIVSELECT_A::DIV16
    }
    #[doc = "Checks if the value of the field is `DIV32`"]
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == BUPDIVSELECT_A::DIV32
    }
    #[doc = "Checks if the value of the field is `DIV64`"]
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == BUPDIVSELECT_A::DIV64
    }
    #[doc = "Checks if the value of the field is `DIV128`"]
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        *self == BUPDIVSELECT_A::DIV128
    }
}
#[doc = "Field `BUPDIV` writer - Backup Clock Division Factor"]
pub type BUPDIV_W<'a, const O: u8> =
    crate::FieldWriter<'a, u8, BUPDIV_SPEC, u8, BUPDIVSELECT_A, 8, O>;
impl<'a, const O: u8> BUPDIV_W<'a, O> {
    #[doc = "Divide by 1"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(BUPDIVSELECT_A::DIV1)
    }
    #[doc = "Divide by 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(BUPDIVSELECT_A::DIV2)
    }
    #[doc = "Divide by 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(BUPDIVSELECT_A::DIV4)
    }
    #[doc = "Divide by 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(BUPDIVSELECT_A::DIV8)
    }
    #[doc = "Divide by 16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut W {
        self.variant(BUPDIVSELECT_A::DIV16)
    }
    #[doc = "Divide by 32"]
    #[inline(always)]
    pub fn div32(self) -> &'a mut W {
        self.variant(BUPDIVSELECT_A::DIV32)
    }
    #[doc = "Divide by 64"]
    #[inline(always)]
    pub fn div64(self) -> &'a mut W {
        self.variant(BUPDIVSELECT_A::DIV64)
    }
    #[doc = "Divide by 128"]
    #[inline(always)]
    pub fn div128(self) -> &'a mut W {
        self.variant(BUPDIVSELECT_A::DIV128)
    }
}
impl R {
    #[doc = "Bits 0:7 - Backup Clock Division Factor"]
    #[inline(always)]
    pub fn bupdiv(&self) -> BUPDIV_R {
        BUPDIV_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Backup Clock Division Factor"]
    #[inline(always)]
    #[must_use]
    pub fn bupdiv(&mut self) -> BUPDIV_W<0> {
        BUPDIV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Backup Clock Division\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bupdiv](index.html) module"]
pub struct BUPDIV_SPEC;
impl crate::RegisterSpec for BUPDIV_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [bupdiv::R](R) reader structure"]
impl crate::Readable for BUPDIV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bupdiv::W](W) writer structure"]
impl crate::Writable for BUPDIV_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BUPDIV to value 0x01"]
impl crate::Resettable for BUPDIV_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
