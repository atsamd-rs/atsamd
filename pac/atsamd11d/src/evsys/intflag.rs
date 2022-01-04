#[doc = "Register `INTFLAG` reader"]
pub struct R(crate::R<INTFLAG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTFLAG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTFLAG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTFLAG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTFLAG` writer"]
pub struct W(crate::W<INTFLAG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTFLAG_SPEC>;
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
impl From<crate::W<INTFLAG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTFLAG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OVR0` reader - Channel 0 Overrun"]
pub struct OVR0_R(crate::FieldReader<bool, bool>);
impl OVR0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OVR0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVR0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVR0` writer - Channel 0 Overrun"]
pub struct OVR0_W<'a> {
    w: &'a mut W,
}
impl<'a> OVR0_W<'a> {
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
#[doc = "Field `OVR1` reader - Channel 1 Overrun"]
pub struct OVR1_R(crate::FieldReader<bool, bool>);
impl OVR1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OVR1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVR1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVR1` writer - Channel 1 Overrun"]
pub struct OVR1_W<'a> {
    w: &'a mut W,
}
impl<'a> OVR1_W<'a> {
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
#[doc = "Field `OVR2` reader - Channel 2 Overrun"]
pub struct OVR2_R(crate::FieldReader<bool, bool>);
impl OVR2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OVR2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVR2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVR2` writer - Channel 2 Overrun"]
pub struct OVR2_W<'a> {
    w: &'a mut W,
}
impl<'a> OVR2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `OVR3` reader - Channel 3 Overrun"]
pub struct OVR3_R(crate::FieldReader<bool, bool>);
impl OVR3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OVR3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVR3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVR3` writer - Channel 3 Overrun"]
pub struct OVR3_W<'a> {
    w: &'a mut W,
}
impl<'a> OVR3_W<'a> {
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
#[doc = "Field `OVR4` reader - Channel 4 Overrun"]
pub struct OVR4_R(crate::FieldReader<bool, bool>);
impl OVR4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OVR4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVR4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVR4` writer - Channel 4 Overrun"]
pub struct OVR4_W<'a> {
    w: &'a mut W,
}
impl<'a> OVR4_W<'a> {
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
#[doc = "Field `OVR5` reader - Channel 5 Overrun"]
pub struct OVR5_R(crate::FieldReader<bool, bool>);
impl OVR5_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OVR5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVR5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVR5` writer - Channel 5 Overrun"]
pub struct OVR5_W<'a> {
    w: &'a mut W,
}
impl<'a> OVR5_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `EVD0` reader - Channel 0 Event Detection"]
pub struct EVD0_R(crate::FieldReader<bool, bool>);
impl EVD0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EVD0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EVD0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EVD0` writer - Channel 0 Event Detection"]
pub struct EVD0_W<'a> {
    w: &'a mut W,
}
impl<'a> EVD0_W<'a> {
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
#[doc = "Field `EVD1` reader - Channel 1 Event Detection"]
pub struct EVD1_R(crate::FieldReader<bool, bool>);
impl EVD1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EVD1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EVD1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EVD1` writer - Channel 1 Event Detection"]
pub struct EVD1_W<'a> {
    w: &'a mut W,
}
impl<'a> EVD1_W<'a> {
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
#[doc = "Field `EVD2` reader - Channel 2 Event Detection"]
pub struct EVD2_R(crate::FieldReader<bool, bool>);
impl EVD2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EVD2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EVD2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EVD2` writer - Channel 2 Event Detection"]
pub struct EVD2_W<'a> {
    w: &'a mut W,
}
impl<'a> EVD2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "Field `EVD3` reader - Channel 3 Event Detection"]
pub struct EVD3_R(crate::FieldReader<bool, bool>);
impl EVD3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EVD3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EVD3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EVD3` writer - Channel 3 Event Detection"]
pub struct EVD3_W<'a> {
    w: &'a mut W,
}
impl<'a> EVD3_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "Field `EVD4` reader - Channel 4 Event Detection"]
pub struct EVD4_R(crate::FieldReader<bool, bool>);
impl EVD4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EVD4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EVD4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EVD4` writer - Channel 4 Event Detection"]
pub struct EVD4_W<'a> {
    w: &'a mut W,
}
impl<'a> EVD4_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "Field `EVD5` reader - Channel 5 Event Detection"]
pub struct EVD5_R(crate::FieldReader<bool, bool>);
impl EVD5_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EVD5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EVD5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EVD5` writer - Channel 5 Event Detection"]
pub struct EVD5_W<'a> {
    w: &'a mut W,
}
impl<'a> EVD5_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Channel 0 Overrun"]
    #[inline(always)]
    pub fn ovr0(&self) -> OVR0_R {
        OVR0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Channel 1 Overrun"]
    #[inline(always)]
    pub fn ovr1(&self) -> OVR1_R {
        OVR1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Channel 2 Overrun"]
    #[inline(always)]
    pub fn ovr2(&self) -> OVR2_R {
        OVR2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Channel 3 Overrun"]
    #[inline(always)]
    pub fn ovr3(&self) -> OVR3_R {
        OVR3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Channel 4 Overrun"]
    #[inline(always)]
    pub fn ovr4(&self) -> OVR4_R {
        OVR4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Channel 5 Overrun"]
    #[inline(always)]
    pub fn ovr5(&self) -> OVR5_R {
        OVR5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Channel 0 Event Detection"]
    #[inline(always)]
    pub fn evd0(&self) -> EVD0_R {
        EVD0_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Channel 1 Event Detection"]
    #[inline(always)]
    pub fn evd1(&self) -> EVD1_R {
        EVD1_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Channel 2 Event Detection"]
    #[inline(always)]
    pub fn evd2(&self) -> EVD2_R {
        EVD2_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Channel 3 Event Detection"]
    #[inline(always)]
    pub fn evd3(&self) -> EVD3_R {
        EVD3_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Channel 4 Event Detection"]
    #[inline(always)]
    pub fn evd4(&self) -> EVD4_R {
        EVD4_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Channel 5 Event Detection"]
    #[inline(always)]
    pub fn evd5(&self) -> EVD5_R {
        EVD5_R::new(((self.bits >> 13) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Channel 0 Overrun"]
    #[inline(always)]
    pub fn ovr0(&mut self) -> OVR0_W {
        OVR0_W { w: self }
    }
    #[doc = "Bit 1 - Channel 1 Overrun"]
    #[inline(always)]
    pub fn ovr1(&mut self) -> OVR1_W {
        OVR1_W { w: self }
    }
    #[doc = "Bit 2 - Channel 2 Overrun"]
    #[inline(always)]
    pub fn ovr2(&mut self) -> OVR2_W {
        OVR2_W { w: self }
    }
    #[doc = "Bit 3 - Channel 3 Overrun"]
    #[inline(always)]
    pub fn ovr3(&mut self) -> OVR3_W {
        OVR3_W { w: self }
    }
    #[doc = "Bit 4 - Channel 4 Overrun"]
    #[inline(always)]
    pub fn ovr4(&mut self) -> OVR4_W {
        OVR4_W { w: self }
    }
    #[doc = "Bit 5 - Channel 5 Overrun"]
    #[inline(always)]
    pub fn ovr5(&mut self) -> OVR5_W {
        OVR5_W { w: self }
    }
    #[doc = "Bit 8 - Channel 0 Event Detection"]
    #[inline(always)]
    pub fn evd0(&mut self) -> EVD0_W {
        EVD0_W { w: self }
    }
    #[doc = "Bit 9 - Channel 1 Event Detection"]
    #[inline(always)]
    pub fn evd1(&mut self) -> EVD1_W {
        EVD1_W { w: self }
    }
    #[doc = "Bit 10 - Channel 2 Event Detection"]
    #[inline(always)]
    pub fn evd2(&mut self) -> EVD2_W {
        EVD2_W { w: self }
    }
    #[doc = "Bit 11 - Channel 3 Event Detection"]
    #[inline(always)]
    pub fn evd3(&mut self) -> EVD3_W {
        EVD3_W { w: self }
    }
    #[doc = "Bit 12 - Channel 4 Event Detection"]
    #[inline(always)]
    pub fn evd4(&mut self) -> EVD4_W {
        EVD4_W { w: self }
    }
    #[doc = "Bit 13 - Channel 5 Event Detection"]
    #[inline(always)]
    pub fn evd5(&mut self) -> EVD5_W {
        EVD5_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Flag Status and Clear\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intflag](index.html) module"]
pub struct INTFLAG_SPEC;
impl crate::RegisterSpec for INTFLAG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intflag::R](R) reader structure"]
impl crate::Readable for INTFLAG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intflag::W](W) writer structure"]
impl crate::Writable for INTFLAG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTFLAG to value 0"]
impl crate::Resettable for INTFLAG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
