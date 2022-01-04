#[doc = "Register `PCHCTRL[%s]` reader"]
pub struct R(crate::R<PCHCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PCHCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PCHCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PCHCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PCHCTRL[%s]` writer"]
pub struct W(crate::W<PCHCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PCHCTRL_SPEC>;
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
impl From<crate::W<PCHCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PCHCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Generic Clock Generator\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum GEN_A {
    #[doc = "0: Generic clock generator 0"]
    GCLK0 = 0,
    #[doc = "1: Generic clock generator 1"]
    GCLK1 = 1,
    #[doc = "2: Generic clock generator 2"]
    GCLK2 = 2,
    #[doc = "3: Generic clock generator 3"]
    GCLK3 = 3,
    #[doc = "4: Generic clock generator 4"]
    GCLK4 = 4,
    #[doc = "5: Generic clock generator 5"]
    GCLK5 = 5,
    #[doc = "6: Generic clock generator 6"]
    GCLK6 = 6,
    #[doc = "7: Generic clock generator 7"]
    GCLK7 = 7,
    #[doc = "8: Generic clock generator 8"]
    GCLK8 = 8,
    #[doc = "9: Generic clock generator 9"]
    GCLK9 = 9,
    #[doc = "10: Generic clock generator 10"]
    GCLK10 = 10,
    #[doc = "11: Generic clock generator 11"]
    GCLK11 = 11,
}
impl From<GEN_A> for u8 {
    #[inline(always)]
    fn from(variant: GEN_A) -> Self {
        variant as _
    }
}
#[doc = "Field `GEN` reader - Generic Clock Generator"]
pub struct GEN_R(crate::FieldReader<u8, GEN_A>);
impl GEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        GEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<GEN_A> {
        match self.bits {
            0 => Some(GEN_A::GCLK0),
            1 => Some(GEN_A::GCLK1),
            2 => Some(GEN_A::GCLK2),
            3 => Some(GEN_A::GCLK3),
            4 => Some(GEN_A::GCLK4),
            5 => Some(GEN_A::GCLK5),
            6 => Some(GEN_A::GCLK6),
            7 => Some(GEN_A::GCLK7),
            8 => Some(GEN_A::GCLK8),
            9 => Some(GEN_A::GCLK9),
            10 => Some(GEN_A::GCLK10),
            11 => Some(GEN_A::GCLK11),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `GCLK0`"]
    #[inline(always)]
    pub fn is_gclk0(&self) -> bool {
        **self == GEN_A::GCLK0
    }
    #[doc = "Checks if the value of the field is `GCLK1`"]
    #[inline(always)]
    pub fn is_gclk1(&self) -> bool {
        **self == GEN_A::GCLK1
    }
    #[doc = "Checks if the value of the field is `GCLK2`"]
    #[inline(always)]
    pub fn is_gclk2(&self) -> bool {
        **self == GEN_A::GCLK2
    }
    #[doc = "Checks if the value of the field is `GCLK3`"]
    #[inline(always)]
    pub fn is_gclk3(&self) -> bool {
        **self == GEN_A::GCLK3
    }
    #[doc = "Checks if the value of the field is `GCLK4`"]
    #[inline(always)]
    pub fn is_gclk4(&self) -> bool {
        **self == GEN_A::GCLK4
    }
    #[doc = "Checks if the value of the field is `GCLK5`"]
    #[inline(always)]
    pub fn is_gclk5(&self) -> bool {
        **self == GEN_A::GCLK5
    }
    #[doc = "Checks if the value of the field is `GCLK6`"]
    #[inline(always)]
    pub fn is_gclk6(&self) -> bool {
        **self == GEN_A::GCLK6
    }
    #[doc = "Checks if the value of the field is `GCLK7`"]
    #[inline(always)]
    pub fn is_gclk7(&self) -> bool {
        **self == GEN_A::GCLK7
    }
    #[doc = "Checks if the value of the field is `GCLK8`"]
    #[inline(always)]
    pub fn is_gclk8(&self) -> bool {
        **self == GEN_A::GCLK8
    }
    #[doc = "Checks if the value of the field is `GCLK9`"]
    #[inline(always)]
    pub fn is_gclk9(&self) -> bool {
        **self == GEN_A::GCLK9
    }
    #[doc = "Checks if the value of the field is `GCLK10`"]
    #[inline(always)]
    pub fn is_gclk10(&self) -> bool {
        **self == GEN_A::GCLK10
    }
    #[doc = "Checks if the value of the field is `GCLK11`"]
    #[inline(always)]
    pub fn is_gclk11(&self) -> bool {
        **self == GEN_A::GCLK11
    }
}
impl core::ops::Deref for GEN_R {
    type Target = crate::FieldReader<u8, GEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GEN` writer - Generic Clock Generator"]
pub struct GEN_W<'a> {
    w: &'a mut W,
}
impl<'a> GEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GEN_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Generic clock generator 0"]
    #[inline(always)]
    pub fn gclk0(self) -> &'a mut W {
        self.variant(GEN_A::GCLK0)
    }
    #[doc = "Generic clock generator 1"]
    #[inline(always)]
    pub fn gclk1(self) -> &'a mut W {
        self.variant(GEN_A::GCLK1)
    }
    #[doc = "Generic clock generator 2"]
    #[inline(always)]
    pub fn gclk2(self) -> &'a mut W {
        self.variant(GEN_A::GCLK2)
    }
    #[doc = "Generic clock generator 3"]
    #[inline(always)]
    pub fn gclk3(self) -> &'a mut W {
        self.variant(GEN_A::GCLK3)
    }
    #[doc = "Generic clock generator 4"]
    #[inline(always)]
    pub fn gclk4(self) -> &'a mut W {
        self.variant(GEN_A::GCLK4)
    }
    #[doc = "Generic clock generator 5"]
    #[inline(always)]
    pub fn gclk5(self) -> &'a mut W {
        self.variant(GEN_A::GCLK5)
    }
    #[doc = "Generic clock generator 6"]
    #[inline(always)]
    pub fn gclk6(self) -> &'a mut W {
        self.variant(GEN_A::GCLK6)
    }
    #[doc = "Generic clock generator 7"]
    #[inline(always)]
    pub fn gclk7(self) -> &'a mut W {
        self.variant(GEN_A::GCLK7)
    }
    #[doc = "Generic clock generator 8"]
    #[inline(always)]
    pub fn gclk8(self) -> &'a mut W {
        self.variant(GEN_A::GCLK8)
    }
    #[doc = "Generic clock generator 9"]
    #[inline(always)]
    pub fn gclk9(self) -> &'a mut W {
        self.variant(GEN_A::GCLK9)
    }
    #[doc = "Generic clock generator 10"]
    #[inline(always)]
    pub fn gclk10(self) -> &'a mut W {
        self.variant(GEN_A::GCLK10)
    }
    #[doc = "Generic clock generator 11"]
    #[inline(always)]
    pub fn gclk11(self) -> &'a mut W {
        self.variant(GEN_A::GCLK11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
#[doc = "Field `CHEN` reader - Channel Enable"]
pub struct CHEN_R(crate::FieldReader<bool, bool>);
impl CHEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CHEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHEN` writer - Channel Enable"]
pub struct CHEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CHEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `WRTLOCK` reader - Write Lock"]
pub struct WRTLOCK_R(crate::FieldReader<bool, bool>);
impl WRTLOCK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WRTLOCK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WRTLOCK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WRTLOCK` writer - Write Lock"]
pub struct WRTLOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> WRTLOCK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Generic Clock Generator"]
    #[inline(always)]
    pub fn gen(&self) -> GEN_R {
        GEN_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 6 - Channel Enable"]
    #[inline(always)]
    pub fn chen(&self) -> CHEN_R {
        CHEN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Write Lock"]
    #[inline(always)]
    pub fn wrtlock(&self) -> WRTLOCK_R {
        WRTLOCK_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Generic Clock Generator"]
    #[inline(always)]
    pub fn gen(&mut self) -> GEN_W {
        GEN_W { w: self }
    }
    #[doc = "Bit 6 - Channel Enable"]
    #[inline(always)]
    pub fn chen(&mut self) -> CHEN_W {
        CHEN_W { w: self }
    }
    #[doc = "Bit 7 - Write Lock"]
    #[inline(always)]
    pub fn wrtlock(&mut self) -> WRTLOCK_W {
        WRTLOCK_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Peripheral Clock Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pchctrl](index.html) module"]
pub struct PCHCTRL_SPEC;
impl crate::RegisterSpec for PCHCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pchctrl::R](R) reader structure"]
impl crate::Readable for PCHCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pchctrl::W](W) writer structure"]
impl crate::Writable for PCHCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PCHCTRL[%s]
to value 0"]
impl crate::Resettable for PCHCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
