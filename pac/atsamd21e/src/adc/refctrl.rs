#[doc = "Register `REFCTRL` reader"]
pub struct R(crate::R<REFCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<REFCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<REFCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<REFCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `REFCTRL` writer"]
pub struct W(crate::W<REFCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<REFCTRL_SPEC>;
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
impl From<crate::W<REFCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<REFCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `REFSEL` reader - Reference Selection"]
pub type REFSEL_R = crate::FieldReader<u8, REFSELSELECT_A>;
#[doc = "Reference Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum REFSELSELECT_A {
    #[doc = "0: 1.0V voltage reference"]
    INT1V = 0,
    #[doc = "1: 1/1.48 VDDANA"]
    INTVCC0 = 1,
    #[doc = "2: 1/2 VDDANA (only for VDDANA > 2.0V)"]
    INTVCC1 = 2,
    #[doc = "3: External reference"]
    AREFA = 3,
    #[doc = "4: External reference"]
    AREFB = 4,
}
impl From<REFSELSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: REFSELSELECT_A) -> Self {
        variant as _
    }
}
impl REFSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<REFSELSELECT_A> {
        match self.bits {
            0 => Some(REFSELSELECT_A::INT1V),
            1 => Some(REFSELSELECT_A::INTVCC0),
            2 => Some(REFSELSELECT_A::INTVCC1),
            3 => Some(REFSELSELECT_A::AREFA),
            4 => Some(REFSELSELECT_A::AREFB),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `INT1V`"]
    #[inline(always)]
    pub fn is_int1v(&self) -> bool {
        *self == REFSELSELECT_A::INT1V
    }
    #[doc = "Checks if the value of the field is `INTVCC0`"]
    #[inline(always)]
    pub fn is_intvcc0(&self) -> bool {
        *self == REFSELSELECT_A::INTVCC0
    }
    #[doc = "Checks if the value of the field is `INTVCC1`"]
    #[inline(always)]
    pub fn is_intvcc1(&self) -> bool {
        *self == REFSELSELECT_A::INTVCC1
    }
    #[doc = "Checks if the value of the field is `AREFA`"]
    #[inline(always)]
    pub fn is_arefa(&self) -> bool {
        *self == REFSELSELECT_A::AREFA
    }
    #[doc = "Checks if the value of the field is `AREFB`"]
    #[inline(always)]
    pub fn is_arefb(&self) -> bool {
        *self == REFSELSELECT_A::AREFB
    }
}
#[doc = "Field `REFSEL` writer - Reference Selection"]
pub type REFSEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u8, REFCTRL_SPEC, u8, REFSELSELECT_A, 4, O>;
impl<'a, const O: u8> REFSEL_W<'a, O> {
    #[doc = "1.0V voltage reference"]
    #[inline(always)]
    pub fn int1v(self) -> &'a mut W {
        self.variant(REFSELSELECT_A::INT1V)
    }
    #[doc = "1/1.48 VDDANA"]
    #[inline(always)]
    pub fn intvcc0(self) -> &'a mut W {
        self.variant(REFSELSELECT_A::INTVCC0)
    }
    #[doc = "1/2 VDDANA (only for VDDANA > 2.0V)"]
    #[inline(always)]
    pub fn intvcc1(self) -> &'a mut W {
        self.variant(REFSELSELECT_A::INTVCC1)
    }
    #[doc = "External reference"]
    #[inline(always)]
    pub fn arefa(self) -> &'a mut W {
        self.variant(REFSELSELECT_A::AREFA)
    }
    #[doc = "External reference"]
    #[inline(always)]
    pub fn arefb(self) -> &'a mut W {
        self.variant(REFSELSELECT_A::AREFB)
    }
}
#[doc = "Field `REFCOMP` reader - Reference Buffer Offset Compensation Enable"]
pub type REFCOMP_R = crate::BitReader<bool>;
#[doc = "Field `REFCOMP` writer - Reference Buffer Offset Compensation Enable"]
pub type REFCOMP_W<'a, const O: u8> = crate::BitWriter<'a, u8, REFCTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:3 - Reference Selection"]
    #[inline(always)]
    pub fn refsel(&self) -> REFSEL_R {
        REFSEL_R::new(self.bits & 0x0f)
    }
    #[doc = "Bit 7 - Reference Buffer Offset Compensation Enable"]
    #[inline(always)]
    pub fn refcomp(&self) -> REFCOMP_R {
        REFCOMP_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Reference Selection"]
    #[inline(always)]
    #[must_use]
    pub fn refsel(&mut self) -> REFSEL_W<0> {
        REFSEL_W::new(self)
    }
    #[doc = "Bit 7 - Reference Buffer Offset Compensation Enable"]
    #[inline(always)]
    #[must_use]
    pub fn refcomp(&mut self) -> REFCOMP_W<7> {
        REFCOMP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Reference Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [refctrl](index.html) module"]
pub struct REFCTRL_SPEC;
impl crate::RegisterSpec for REFCTRL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [refctrl::R](R) reader structure"]
impl crate::Readable for REFCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [refctrl::W](W) writer structure"]
impl crate::Writable for REFCTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets REFCTRL to value 0"]
impl crate::Resettable for REFCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
