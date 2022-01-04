#[doc = "Register `CCR` reader"]
pub struct R(crate::R<CCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CCR` writer"]
pub struct W(crate::W<CCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CCR_SPEC>;
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
impl From<crate::W<CCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `NONBASETHRDENA` reader - Indicates how processor enters Thread mode"]
pub struct NONBASETHRDENA_R(crate::FieldReader<bool, bool>);
impl NONBASETHRDENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        NONBASETHRDENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NONBASETHRDENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NONBASETHRDENA` writer - Indicates how processor enters Thread mode"]
pub struct NONBASETHRDENA_W<'a> {
    w: &'a mut W,
}
impl<'a> NONBASETHRDENA_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Field `USERSETMPEND` reader - Enables unprivileged software access to STIR register"]
pub struct USERSETMPEND_R(crate::FieldReader<bool, bool>);
impl USERSETMPEND_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        USERSETMPEND_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USERSETMPEND_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USERSETMPEND` writer - Enables unprivileged software access to STIR register"]
pub struct USERSETMPEND_W<'a> {
    w: &'a mut W,
}
impl<'a> USERSETMPEND_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Enables unaligned access traps\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UNALIGN_TRP_A {
    #[doc = "0: Do not trap unaligned halfword and word accesses"]
    VALUE_0 = 0,
    #[doc = "1: Trap unaligned halfword and word accesses"]
    VALUE_1 = 1,
}
impl From<UNALIGN_TRP_A> for bool {
    #[inline(always)]
    fn from(variant: UNALIGN_TRP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UNALIGN_TRP` reader - Enables unaligned access traps"]
pub struct UNALIGN_TRP_R(crate::FieldReader<bool, UNALIGN_TRP_A>);
impl UNALIGN_TRP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        UNALIGN_TRP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UNALIGN_TRP_A {
        match self.bits {
            false => UNALIGN_TRP_A::VALUE_0,
            true => UNALIGN_TRP_A::VALUE_1,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE_0`"]
    #[inline(always)]
    pub fn is_value_0(&self) -> bool {
        **self == UNALIGN_TRP_A::VALUE_0
    }
    #[doc = "Checks if the value of the field is `VALUE_1`"]
    #[inline(always)]
    pub fn is_value_1(&self) -> bool {
        **self == UNALIGN_TRP_A::VALUE_1
    }
}
impl core::ops::Deref for UNALIGN_TRP_R {
    type Target = crate::FieldReader<bool, UNALIGN_TRP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UNALIGN_TRP` writer - Enables unaligned access traps"]
pub struct UNALIGN_TRP_W<'a> {
    w: &'a mut W,
}
impl<'a> UNALIGN_TRP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UNALIGN_TRP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Do not trap unaligned halfword and word accesses"]
    #[inline(always)]
    pub fn value_0(self) -> &'a mut W {
        self.variant(UNALIGN_TRP_A::VALUE_0)
    }
    #[doc = "Trap unaligned halfword and word accesses"]
    #[inline(always)]
    pub fn value_1(self) -> &'a mut W {
        self.variant(UNALIGN_TRP_A::VALUE_1)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `DIV_0_TRP` reader - Enables divide by 0 trap"]
pub struct DIV_0_TRP_R(crate::FieldReader<bool, bool>);
impl DIV_0_TRP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DIV_0_TRP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIV_0_TRP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIV_0_TRP` writer - Enables divide by 0 trap"]
pub struct DIV_0_TRP_W<'a> {
    w: &'a mut W,
}
impl<'a> DIV_0_TRP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `BFHFNMIGN` reader - Ignore LDM/STM BusFault for -1/-2 priority handlers"]
pub struct BFHFNMIGN_R(crate::FieldReader<bool, bool>);
impl BFHFNMIGN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BFHFNMIGN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BFHFNMIGN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BFHFNMIGN` writer - Ignore LDM/STM BusFault for -1/-2 priority handlers"]
pub struct BFHFNMIGN_W<'a> {
    w: &'a mut W,
}
impl<'a> BFHFNMIGN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Indicates stack alignment on exception entry\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STKALIGN_A {
    #[doc = "0: 4-byte aligned"]
    VALUE_0 = 0,
    #[doc = "1: 8-byte aligned"]
    VALUE_1 = 1,
}
impl From<STKALIGN_A> for bool {
    #[inline(always)]
    fn from(variant: STKALIGN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STKALIGN` reader - Indicates stack alignment on exception entry"]
pub struct STKALIGN_R(crate::FieldReader<bool, STKALIGN_A>);
impl STKALIGN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        STKALIGN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STKALIGN_A {
        match self.bits {
            false => STKALIGN_A::VALUE_0,
            true => STKALIGN_A::VALUE_1,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE_0`"]
    #[inline(always)]
    pub fn is_value_0(&self) -> bool {
        **self == STKALIGN_A::VALUE_0
    }
    #[doc = "Checks if the value of the field is `VALUE_1`"]
    #[inline(always)]
    pub fn is_value_1(&self) -> bool {
        **self == STKALIGN_A::VALUE_1
    }
}
impl core::ops::Deref for STKALIGN_R {
    type Target = crate::FieldReader<bool, STKALIGN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STKALIGN` writer - Indicates stack alignment on exception entry"]
pub struct STKALIGN_W<'a> {
    w: &'a mut W,
}
impl<'a> STKALIGN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STKALIGN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "4-byte aligned"]
    #[inline(always)]
    pub fn value_0(self) -> &'a mut W {
        self.variant(STKALIGN_A::VALUE_0)
    }
    #[doc = "8-byte aligned"]
    #[inline(always)]
    pub fn value_1(self) -> &'a mut W {
        self.variant(STKALIGN_A::VALUE_1)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Indicates how processor enters Thread mode"]
    #[inline(always)]
    pub fn nonbasethrdena(&self) -> NONBASETHRDENA_R {
        NONBASETHRDENA_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Enables unprivileged software access to STIR register"]
    #[inline(always)]
    pub fn usersetmpend(&self) -> USERSETMPEND_R {
        USERSETMPEND_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Enables unaligned access traps"]
    #[inline(always)]
    pub fn unalign_trp(&self) -> UNALIGN_TRP_R {
        UNALIGN_TRP_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Enables divide by 0 trap"]
    #[inline(always)]
    pub fn div_0_trp(&self) -> DIV_0_TRP_R {
        DIV_0_TRP_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Ignore LDM/STM BusFault for -1/-2 priority handlers"]
    #[inline(always)]
    pub fn bfhfnmign(&self) -> BFHFNMIGN_R {
        BFHFNMIGN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Indicates stack alignment on exception entry"]
    #[inline(always)]
    pub fn stkalign(&self) -> STKALIGN_R {
        STKALIGN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Indicates how processor enters Thread mode"]
    #[inline(always)]
    pub fn nonbasethrdena(&mut self) -> NONBASETHRDENA_W {
        NONBASETHRDENA_W { w: self }
    }
    #[doc = "Bit 1 - Enables unprivileged software access to STIR register"]
    #[inline(always)]
    pub fn usersetmpend(&mut self) -> USERSETMPEND_W {
        USERSETMPEND_W { w: self }
    }
    #[doc = "Bit 3 - Enables unaligned access traps"]
    #[inline(always)]
    pub fn unalign_trp(&mut self) -> UNALIGN_TRP_W {
        UNALIGN_TRP_W { w: self }
    }
    #[doc = "Bit 4 - Enables divide by 0 trap"]
    #[inline(always)]
    pub fn div_0_trp(&mut self) -> DIV_0_TRP_W {
        DIV_0_TRP_W { w: self }
    }
    #[doc = "Bit 8 - Ignore LDM/STM BusFault for -1/-2 priority handlers"]
    #[inline(always)]
    pub fn bfhfnmign(&mut self) -> BFHFNMIGN_W {
        BFHFNMIGN_W { w: self }
    }
    #[doc = "Bit 9 - Indicates stack alignment on exception entry"]
    #[inline(always)]
    pub fn stkalign(&mut self) -> STKALIGN_W {
        STKALIGN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Configuration and Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccr](index.html) module"]
pub struct CCR_SPEC;
impl crate::RegisterSpec for CCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ccr::R](R) reader structure"]
impl crate::Readable for CCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ccr::W](W) writer structure"]
impl crate::Writable for CCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CCR to value 0x0200"]
impl crate::Resettable for CCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0200
    }
}
