#[doc = "Register `ACR` reader"]
pub struct R(crate::R<ACR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ACR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ACR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ACR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ACR` writer"]
pub struct W(crate::W<ACR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ACR_SPEC>;
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
impl From<crate::W<ACR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ACR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "AHB Maximum Burst\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum BMAX_A {
    #[doc = "0: `0`"]
    INCR16 = 0,
    #[doc = "1: `1`"]
    INCR8 = 1,
    #[doc = "2: `10`"]
    INCR4 = 2,
    #[doc = "3: `11`"]
    SINGLE = 3,
}
impl From<BMAX_A> for u8 {
    #[inline(always)]
    fn from(variant: BMAX_A) -> Self {
        variant as _
    }
}
#[doc = "Field `BMAX` reader - AHB Maximum Burst"]
pub struct BMAX_R(crate::FieldReader<u8, BMAX_A>);
impl BMAX_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        BMAX_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BMAX_A {
        match self.bits {
            0 => BMAX_A::INCR16,
            1 => BMAX_A::INCR8,
            2 => BMAX_A::INCR4,
            3 => BMAX_A::SINGLE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `INCR16`"]
    #[inline(always)]
    pub fn is_incr16(&self) -> bool {
        **self == BMAX_A::INCR16
    }
    #[doc = "Checks if the value of the field is `INCR8`"]
    #[inline(always)]
    pub fn is_incr8(&self) -> bool {
        **self == BMAX_A::INCR8
    }
    #[doc = "Checks if the value of the field is `INCR4`"]
    #[inline(always)]
    pub fn is_incr4(&self) -> bool {
        **self == BMAX_A::INCR4
    }
    #[doc = "Checks if the value of the field is `SINGLE`"]
    #[inline(always)]
    pub fn is_single(&self) -> bool {
        **self == BMAX_A::SINGLE
    }
}
impl core::ops::Deref for BMAX_R {
    type Target = crate::FieldReader<u8, BMAX_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BMAX` writer - AHB Maximum Burst"]
pub struct BMAX_W<'a> {
    w: &'a mut W,
}
impl<'a> BMAX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BMAX_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn incr16(self) -> &'a mut W {
        self.variant(BMAX_A::INCR16)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn incr8(self) -> &'a mut W {
        self.variant(BMAX_A::INCR8)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn incr4(self) -> &'a mut W {
        self.variant(BMAX_A::INCR4)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn single(self) -> &'a mut W {
        self.variant(BMAX_A::SINGLE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - AHB Maximum Burst"]
    #[inline(always)]
    pub fn bmax(&self) -> BMAX_R {
        BMAX_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - AHB Maximum Burst"]
    #[inline(always)]
    pub fn bmax(&mut self) -> BMAX_W {
        BMAX_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AHB Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [acr](index.html) module"]
pub struct ACR_SPEC;
impl crate::RegisterSpec for ACR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [acr::R](R) reader structure"]
impl crate::Readable for ACR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [acr::W](W) writer structure"]
impl crate::Writable for ACR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ACR to value 0"]
impl crate::Resettable for ACR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
