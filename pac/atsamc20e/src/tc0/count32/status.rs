#[doc = "Register `STATUS` reader"]
pub struct R(crate::R<STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STATUS` writer"]
pub struct W(crate::W<STATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STATUS_SPEC>;
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
impl From<crate::W<STATUS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `STOP` reader - Stop Status Flag"]
pub struct STOP_R(crate::FieldReader<bool, bool>);
impl STOP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        STOP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STOP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STOP` writer - Stop Status Flag"]
pub struct STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> STOP_W<'a> {
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
#[doc = "Field `SLAVE` reader - Slave Status Flag"]
pub struct SLAVE_R(crate::FieldReader<bool, bool>);
impl SLAVE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLAVE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLAVE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLAVE` writer - Slave Status Flag"]
pub struct SLAVE_W<'a> {
    w: &'a mut W,
}
impl<'a> SLAVE_W<'a> {
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
#[doc = "Field `PERBUFV` reader - Synchronization Busy Status"]
pub struct PERBUFV_R(crate::FieldReader<bool, bool>);
impl PERBUFV_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PERBUFV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PERBUFV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PERBUFV` writer - Synchronization Busy Status"]
pub struct PERBUFV_W<'a> {
    w: &'a mut W,
}
impl<'a> PERBUFV_W<'a> {
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
#[doc = "Field `CCBUFV0` reader - Compare channel buffer 0 valid"]
pub struct CCBUFV0_R(crate::FieldReader<bool, bool>);
impl CCBUFV0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CCBUFV0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CCBUFV0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CCBUFV0` writer - Compare channel buffer 0 valid"]
pub struct CCBUFV0_W<'a> {
    w: &'a mut W,
}
impl<'a> CCBUFV0_W<'a> {
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
#[doc = "Field `CCBUFV1` reader - Compare channel buffer 1 valid"]
pub struct CCBUFV1_R(crate::FieldReader<bool, bool>);
impl CCBUFV1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CCBUFV1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CCBUFV1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CCBUFV1` writer - Compare channel buffer 1 valid"]
pub struct CCBUFV1_W<'a> {
    w: &'a mut W,
}
impl<'a> CCBUFV1_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Stop Status Flag"]
    #[inline(always)]
    pub fn stop(&self) -> STOP_R {
        STOP_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Slave Status Flag"]
    #[inline(always)]
    pub fn slave(&self) -> SLAVE_R {
        SLAVE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Synchronization Busy Status"]
    #[inline(always)]
    pub fn perbufv(&self) -> PERBUFV_R {
        PERBUFV_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Compare channel buffer 0 valid"]
    #[inline(always)]
    pub fn ccbufv0(&self) -> CCBUFV0_R {
        CCBUFV0_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Compare channel buffer 1 valid"]
    #[inline(always)]
    pub fn ccbufv1(&self) -> CCBUFV1_R {
        CCBUFV1_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Stop Status Flag"]
    #[inline(always)]
    pub fn stop(&mut self) -> STOP_W {
        STOP_W { w: self }
    }
    #[doc = "Bit 1 - Slave Status Flag"]
    #[inline(always)]
    pub fn slave(&mut self) -> SLAVE_W {
        SLAVE_W { w: self }
    }
    #[doc = "Bit 3 - Synchronization Busy Status"]
    #[inline(always)]
    pub fn perbufv(&mut self) -> PERBUFV_W {
        PERBUFV_W { w: self }
    }
    #[doc = "Bit 4 - Compare channel buffer 0 valid"]
    #[inline(always)]
    pub fn ccbufv0(&mut self) -> CCBUFV0_W {
        CCBUFV0_W { w: self }
    }
    #[doc = "Bit 5 - Compare channel buffer 1 valid"]
    #[inline(always)]
    pub fn ccbufv1(&mut self) -> CCBUFV1_W {
        CCBUFV1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status](index.html) module"]
pub struct STATUS_SPEC;
impl crate::RegisterSpec for STATUS_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [status::R](R) reader structure"]
impl crate::Readable for STATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [status::W](W) writer structure"]
impl crate::Writable for STATUS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets STATUS to value 0x01"]
impl crate::Resettable for STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
