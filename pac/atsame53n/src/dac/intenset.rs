#[doc = "Register `INTENSET` reader"]
pub struct R(crate::R<INTENSET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTENSET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTENSET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTENSET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTENSET` writer"]
pub struct W(crate::W<INTENSET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTENSET_SPEC>;
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
impl From<crate::W<INTENSET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTENSET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UNDERRUN0` reader - Underrun 0 Interrupt Enable"]
pub struct UNDERRUN0_R(crate::FieldReader<bool, bool>);
impl UNDERRUN0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        UNDERRUN0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UNDERRUN0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UNDERRUN0` writer - Underrun 0 Interrupt Enable"]
pub struct UNDERRUN0_W<'a> {
    w: &'a mut W,
}
impl<'a> UNDERRUN0_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | (value as u8 & 0x01);
        self.w
    }
}
#[doc = "Field `UNDERRUN1` reader - Underrun 1 Interrupt Enable"]
pub struct UNDERRUN1_R(crate::FieldReader<bool, bool>);
impl UNDERRUN1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        UNDERRUN1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UNDERRUN1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UNDERRUN1` writer - Underrun 1 Interrupt Enable"]
pub struct UNDERRUN1_W<'a> {
    w: &'a mut W,
}
impl<'a> UNDERRUN1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u8 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `EMPTY0` reader - Data Buffer 0 Empty Interrupt Enable"]
pub struct EMPTY0_R(crate::FieldReader<bool, bool>);
impl EMPTY0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EMPTY0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EMPTY0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EMPTY0` writer - Data Buffer 0 Empty Interrupt Enable"]
pub struct EMPTY0_W<'a> {
    w: &'a mut W,
}
impl<'a> EMPTY0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u8 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `EMPTY1` reader - Data Buffer 1 Empty Interrupt Enable"]
pub struct EMPTY1_R(crate::FieldReader<bool, bool>);
impl EMPTY1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EMPTY1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EMPTY1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EMPTY1` writer - Data Buffer 1 Empty Interrupt Enable"]
pub struct EMPTY1_W<'a> {
    w: &'a mut W,
}
impl<'a> EMPTY1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u8 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `RESRDY0` reader - Result 0 Ready Interrupt Enable"]
pub struct RESRDY0_R(crate::FieldReader<bool, bool>);
impl RESRDY0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RESRDY0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESRDY0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESRDY0` writer - Result 0 Ready Interrupt Enable"]
pub struct RESRDY0_W<'a> {
    w: &'a mut W,
}
impl<'a> RESRDY0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u8 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `RESRDY1` reader - Result 1 Ready Interrupt Enable"]
pub struct RESRDY1_R(crate::FieldReader<bool, bool>);
impl RESRDY1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RESRDY1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESRDY1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESRDY1` writer - Result 1 Ready Interrupt Enable"]
pub struct RESRDY1_W<'a> {
    w: &'a mut W,
}
impl<'a> RESRDY1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u8 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `OVERRUN0` reader - Overrun 0 Interrupt Enable"]
pub struct OVERRUN0_R(crate::FieldReader<bool, bool>);
impl OVERRUN0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OVERRUN0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVERRUN0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVERRUN0` writer - Overrun 0 Interrupt Enable"]
pub struct OVERRUN0_W<'a> {
    w: &'a mut W,
}
impl<'a> OVERRUN0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u8 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `OVERRUN1` reader - Overrun 1 Interrupt Enable"]
pub struct OVERRUN1_R(crate::FieldReader<bool, bool>);
impl OVERRUN1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OVERRUN1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVERRUN1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVERRUN1` writer - Overrun 1 Interrupt Enable"]
pub struct OVERRUN1_W<'a> {
    w: &'a mut W,
}
impl<'a> OVERRUN1_W<'a> {
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
    #[doc = "Bit 0 - Underrun 0 Interrupt Enable"]
    #[inline(always)]
    pub fn underrun0(&self) -> UNDERRUN0_R {
        UNDERRUN0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Underrun 1 Interrupt Enable"]
    #[inline(always)]
    pub fn underrun1(&self) -> UNDERRUN1_R {
        UNDERRUN1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Data Buffer 0 Empty Interrupt Enable"]
    #[inline(always)]
    pub fn empty0(&self) -> EMPTY0_R {
        EMPTY0_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Data Buffer 1 Empty Interrupt Enable"]
    #[inline(always)]
    pub fn empty1(&self) -> EMPTY1_R {
        EMPTY1_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Result 0 Ready Interrupt Enable"]
    #[inline(always)]
    pub fn resrdy0(&self) -> RESRDY0_R {
        RESRDY0_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Result 1 Ready Interrupt Enable"]
    #[inline(always)]
    pub fn resrdy1(&self) -> RESRDY1_R {
        RESRDY1_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Overrun 0 Interrupt Enable"]
    #[inline(always)]
    pub fn overrun0(&self) -> OVERRUN0_R {
        OVERRUN0_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Overrun 1 Interrupt Enable"]
    #[inline(always)]
    pub fn overrun1(&self) -> OVERRUN1_R {
        OVERRUN1_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Underrun 0 Interrupt Enable"]
    #[inline(always)]
    pub fn underrun0(&mut self) -> UNDERRUN0_W {
        UNDERRUN0_W { w: self }
    }
    #[doc = "Bit 1 - Underrun 1 Interrupt Enable"]
    #[inline(always)]
    pub fn underrun1(&mut self) -> UNDERRUN1_W {
        UNDERRUN1_W { w: self }
    }
    #[doc = "Bit 2 - Data Buffer 0 Empty Interrupt Enable"]
    #[inline(always)]
    pub fn empty0(&mut self) -> EMPTY0_W {
        EMPTY0_W { w: self }
    }
    #[doc = "Bit 3 - Data Buffer 1 Empty Interrupt Enable"]
    #[inline(always)]
    pub fn empty1(&mut self) -> EMPTY1_W {
        EMPTY1_W { w: self }
    }
    #[doc = "Bit 4 - Result 0 Ready Interrupt Enable"]
    #[inline(always)]
    pub fn resrdy0(&mut self) -> RESRDY0_W {
        RESRDY0_W { w: self }
    }
    #[doc = "Bit 5 - Result 1 Ready Interrupt Enable"]
    #[inline(always)]
    pub fn resrdy1(&mut self) -> RESRDY1_W {
        RESRDY1_W { w: self }
    }
    #[doc = "Bit 6 - Overrun 0 Interrupt Enable"]
    #[inline(always)]
    pub fn overrun0(&mut self) -> OVERRUN0_W {
        OVERRUN0_W { w: self }
    }
    #[doc = "Bit 7 - Overrun 1 Interrupt Enable"]
    #[inline(always)]
    pub fn overrun1(&mut self) -> OVERRUN1_W {
        OVERRUN1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Enable Set\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intenset](index.html) module"]
pub struct INTENSET_SPEC;
impl crate::RegisterSpec for INTENSET_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [intenset::R](R) reader structure"]
impl crate::Readable for INTENSET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intenset::W](W) writer structure"]
impl crate::Writable for INTENSET_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTENSET to value 0"]
impl crate::Resettable for INTENSET_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
