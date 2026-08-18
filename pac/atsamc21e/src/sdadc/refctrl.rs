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
    #[doc = "0: Internal Bandgap Reference"]
    INTREF = 0,
    #[doc = "1: External Reference"]
    AREFB = 1,
    #[doc = "2: Internal DAC Output"]
    DAC = 2,
    #[doc = "3: VDDANA"]
    INTVCC = 3,
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
    pub fn variant(&self) -> REFSEL_A {
        match self.bits {
            0 => REFSEL_A::INTREF,
            1 => REFSEL_A::AREFB,
            2 => REFSEL_A::DAC,
            3 => REFSEL_A::INTVCC,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `INTREF`"]
    #[inline(always)]
    pub fn is_intref(&self) -> bool {
        **self == REFSEL_A::INTREF
    }
    #[doc = "Checks if the value of the field is `AREFB`"]
    #[inline(always)]
    pub fn is_arefb(&self) -> bool {
        **self == REFSEL_A::AREFB
    }
    #[doc = "Checks if the value of the field is `DAC`"]
    #[inline(always)]
    pub fn is_dac(&self) -> bool {
        **self == REFSEL_A::DAC
    }
    #[doc = "Checks if the value of the field is `INTVCC`"]
    #[inline(always)]
    pub fn is_intvcc(&self) -> bool {
        **self == REFSEL_A::INTVCC
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
        self.bits(variant.into())
    }
    #[doc = "Internal Bandgap Reference"]
    #[inline(always)]
    pub fn intref(self) -> &'a mut W {
        self.variant(REFSEL_A::INTREF)
    }
    #[doc = "External Reference"]
    #[inline(always)]
    pub fn arefb(self) -> &'a mut W {
        self.variant(REFSEL_A::AREFB)
    }
    #[doc = "Internal DAC Output"]
    #[inline(always)]
    pub fn dac(self) -> &'a mut W {
        self.variant(REFSEL_A::DAC)
    }
    #[doc = "VDDANA"]
    #[inline(always)]
    pub fn intvcc(self) -> &'a mut W {
        self.variant(REFSEL_A::INTVCC)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u8 & 0x03);
        self.w
    }
}
#[doc = "Field `REFRANGE` reader - Reference Range"]
pub struct REFRANGE_R(crate::FieldReader<u8, u8>);
impl REFRANGE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        REFRANGE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REFRANGE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REFRANGE` writer - Reference Range"]
pub struct REFRANGE_W<'a> {
    w: &'a mut W,
}
impl<'a> REFRANGE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u8 & 0x03) << 4);
        self.w
    }
}
#[doc = "Field `ONREFBUF` reader - Reference Buffer"]
pub struct ONREFBUF_R(crate::FieldReader<bool, bool>);
impl ONREFBUF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ONREFBUF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ONREFBUF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ONREFBUF` writer - Reference Buffer"]
pub struct ONREFBUF_W<'a> {
    w: &'a mut W,
}
impl<'a> ONREFBUF_W<'a> {
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
    #[doc = "Bits 0:1 - Reference Selection"]
    #[inline(always)]
    pub fn refsel(&self) -> REFSEL_R {
        REFSEL_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - Reference Range"]
    #[inline(always)]
    pub fn refrange(&self) -> REFRANGE_R {
        REFRANGE_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bit 7 - Reference Buffer"]
    #[inline(always)]
    pub fn onrefbuf(&self) -> ONREFBUF_R {
        ONREFBUF_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Reference Selection"]
    #[inline(always)]
    pub fn refsel(&mut self) -> REFSEL_W {
        REFSEL_W { w: self }
    }
    #[doc = "Bits 4:5 - Reference Range"]
    #[inline(always)]
    pub fn refrange(&mut self) -> REFRANGE_W {
        REFRANGE_W { w: self }
    }
    #[doc = "Bit 7 - Reference Buffer"]
    #[inline(always)]
    pub fn onrefbuf(&mut self) -> ONREFBUF_W {
        ONREFBUF_W { w: self }
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
