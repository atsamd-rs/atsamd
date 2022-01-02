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
#[doc = "Reference Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum REFSEL_A {
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
impl From<REFSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: REFSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `REFSEL` reader - Reference Selection"]
pub struct REFSEL_R(crate::FieldReader<u8, REFSEL_A>);
impl REFSEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        REFSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<REFSEL_A> {
        match self.bits {
            0 => Some(REFSEL_A::INT1V),
            1 => Some(REFSEL_A::INTVCC0),
            2 => Some(REFSEL_A::INTVCC1),
            3 => Some(REFSEL_A::AREFA),
            4 => Some(REFSEL_A::AREFB),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `INT1V`"]
    #[inline(always)]
    pub fn is_int1v(&self) -> bool {
        **self == REFSEL_A::INT1V
    }
    #[doc = "Checks if the value of the field is `INTVCC0`"]
    #[inline(always)]
    pub fn is_intvcc0(&self) -> bool {
        **self == REFSEL_A::INTVCC0
    }
    #[doc = "Checks if the value of the field is `INTVCC1`"]
    #[inline(always)]
    pub fn is_intvcc1(&self) -> bool {
        **self == REFSEL_A::INTVCC1
    }
    #[doc = "Checks if the value of the field is `AREFA`"]
    #[inline(always)]
    pub fn is_arefa(&self) -> bool {
        **self == REFSEL_A::AREFA
    }
    #[doc = "Checks if the value of the field is `AREFB`"]
    #[inline(always)]
    pub fn is_arefb(&self) -> bool {
        **self == REFSEL_A::AREFB
    }
}
impl core::ops::Deref for REFSEL_R {
    type Target = crate::FieldReader<u8, REFSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REFSEL` writer - Reference Selection"]
pub struct REFSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> REFSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REFSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "1.0V voltage reference"]
    #[inline(always)]
    pub fn int1v(self) -> &'a mut W {
        self.variant(REFSEL_A::INT1V)
    }
    #[doc = "1/1.48 VDDANA"]
    #[inline(always)]
    pub fn intvcc0(self) -> &'a mut W {
        self.variant(REFSEL_A::INTVCC0)
    }
    #[doc = "1/2 VDDANA (only for VDDANA > 2.0V)"]
    #[inline(always)]
    pub fn intvcc1(self) -> &'a mut W {
        self.variant(REFSEL_A::INTVCC1)
    }
    #[doc = "External reference"]
    #[inline(always)]
    pub fn arefa(self) -> &'a mut W {
        self.variant(REFSEL_A::AREFA)
    }
    #[doc = "External reference"]
    #[inline(always)]
    pub fn arefb(self) -> &'a mut W {
        self.variant(REFSEL_A::AREFB)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u8 & 0x0f);
        self.w
    }
}
#[doc = "Field `REFCOMP` reader - Reference Buffer Offset Compensation Enable"]
pub struct REFCOMP_R(crate::FieldReader<bool, bool>);
impl REFCOMP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        REFCOMP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REFCOMP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REFCOMP` writer - Reference Buffer Offset Compensation Enable"]
pub struct REFCOMP_W<'a> {
    w: &'a mut W,
}
impl<'a> REFCOMP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u8 & 0x01) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Reference Selection"]
    #[inline(always)]
    pub fn refsel(&self) -> REFSEL_R {
        REFSEL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 7 - Reference Buffer Offset Compensation Enable"]
    #[inline(always)]
    pub fn refcomp(&self) -> REFCOMP_R {
        REFCOMP_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Reference Selection"]
    #[inline(always)]
    pub fn refsel(&mut self) -> REFSEL_W {
        REFSEL_W { w: self }
    }
    #[doc = "Bit 7 - Reference Buffer Offset Compensation Enable"]
    #[inline(always)]
    pub fn refcomp(&mut self) -> REFCOMP_W {
        REFCOMP_W { w: self }
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
}
#[doc = "`reset()` method sets REFCTRL to value 0"]
impl crate::Resettable for REFCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
