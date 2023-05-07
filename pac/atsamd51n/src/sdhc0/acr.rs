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
#[doc = "Field `BMAX` reader - AHB Maximum Burst"]
pub type BMAX_R = crate::FieldReader<u8, BMAXSELECT_A>;
#[doc = "AHB Maximum Burst\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BMAXSELECT_A {
    #[doc = "0: `0`"]
    INCR16 = 0,
    #[doc = "1: `1`"]
    INCR8 = 1,
    #[doc = "2: `10`"]
    INCR4 = 2,
    #[doc = "3: `11`"]
    SINGLE = 3,
}
impl From<BMAXSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: BMAXSELECT_A) -> Self {
        variant as _
    }
}
impl BMAX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BMAXSELECT_A {
        match self.bits {
            0 => BMAXSELECT_A::INCR16,
            1 => BMAXSELECT_A::INCR8,
            2 => BMAXSELECT_A::INCR4,
            3 => BMAXSELECT_A::SINGLE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `INCR16`"]
    #[inline(always)]
    pub fn is_incr16(&self) -> bool {
        *self == BMAXSELECT_A::INCR16
    }
    #[doc = "Checks if the value of the field is `INCR8`"]
    #[inline(always)]
    pub fn is_incr8(&self) -> bool {
        *self == BMAXSELECT_A::INCR8
    }
    #[doc = "Checks if the value of the field is `INCR4`"]
    #[inline(always)]
    pub fn is_incr4(&self) -> bool {
        *self == BMAXSELECT_A::INCR4
    }
    #[doc = "Checks if the value of the field is `SINGLE`"]
    #[inline(always)]
    pub fn is_single(&self) -> bool {
        *self == BMAXSELECT_A::SINGLE
    }
}
#[doc = "Field `BMAX` writer - AHB Maximum Burst"]
pub type BMAX_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, ACR_SPEC, u8, BMAXSELECT_A, 2, O>;
impl<'a, const O: u8> BMAX_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn incr16(self) -> &'a mut W {
        self.variant(BMAXSELECT_A::INCR16)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn incr8(self) -> &'a mut W {
        self.variant(BMAXSELECT_A::INCR8)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn incr4(self) -> &'a mut W {
        self.variant(BMAXSELECT_A::INCR4)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn single(self) -> &'a mut W {
        self.variant(BMAXSELECT_A::SINGLE)
    }
}
impl R {
    #[doc = "Bits 0:1 - AHB Maximum Burst"]
    #[inline(always)]
    pub fn bmax(&self) -> BMAX_R {
        BMAX_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - AHB Maximum Burst"]
    #[inline(always)]
    #[must_use]
    pub fn bmax(&mut self) -> BMAX_W<0> {
        BMAX_W::new(self)
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ACR to value 0"]
impl crate::Resettable for ACR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
