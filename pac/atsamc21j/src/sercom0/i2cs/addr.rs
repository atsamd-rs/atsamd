#[doc = "Register `ADDR` reader"]
pub struct R(crate::R<ADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADDR` writer"]
pub struct W(crate::W<ADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADDR_SPEC>;
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
impl From<crate::W<ADDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GENCEN` reader - General Call Address Enable"]
pub struct GENCEN_R(crate::FieldReader<bool, bool>);
impl GENCEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GENCEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GENCEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GENCEN` writer - General Call Address Enable"]
pub struct GENCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> GENCEN_W<'a> {
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
#[doc = "Field `ADDR` reader - Address Value"]
pub struct ADDR_R(crate::FieldReader<u16, u16>);
impl ADDR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        ADDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADDR_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADDR` writer - Address Value"]
pub struct ADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 1)) | ((value as u32 & 0x03ff) << 1);
        self.w
    }
}
#[doc = "Field `TENBITEN` reader - Ten Bit Addressing Enable"]
pub struct TENBITEN_R(crate::FieldReader<bool, bool>);
impl TENBITEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TENBITEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TENBITEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TENBITEN` writer - Ten Bit Addressing Enable"]
pub struct TENBITEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TENBITEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
#[doc = "Field `ADDRMASK` reader - Address Mask"]
pub struct ADDRMASK_R(crate::FieldReader<u16, u16>);
impl ADDRMASK_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        ADDRMASK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADDRMASK_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADDRMASK` writer - Address Mask"]
pub struct ADDRMASK_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDRMASK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 17)) | ((value as u32 & 0x03ff) << 17);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - General Call Address Enable"]
    #[inline(always)]
    pub fn gencen(&self) -> GENCEN_R {
        GENCEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:10 - Address Value"]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new(((self.bits >> 1) & 0x03ff) as u16)
    }
    #[doc = "Bit 15 - Ten Bit Addressing Enable"]
    #[inline(always)]
    pub fn tenbiten(&self) -> TENBITEN_R {
        TENBITEN_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 17:26 - Address Mask"]
    #[inline(always)]
    pub fn addrmask(&self) -> ADDRMASK_R {
        ADDRMASK_R::new(((self.bits >> 17) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - General Call Address Enable"]
    #[inline(always)]
    pub fn gencen(&mut self) -> GENCEN_W {
        GENCEN_W { w: self }
    }
    #[doc = "Bits 1:10 - Address Value"]
    #[inline(always)]
    pub fn addr(&mut self) -> ADDR_W {
        ADDR_W { w: self }
    }
    #[doc = "Bit 15 - Ten Bit Addressing Enable"]
    #[inline(always)]
    pub fn tenbiten(&mut self) -> TENBITEN_W {
        TENBITEN_W { w: self }
    }
    #[doc = "Bits 17:26 - Address Mask"]
    #[inline(always)]
    pub fn addrmask(&mut self) -> ADDRMASK_W {
        ADDRMASK_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2CS Address\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [addr](index.html) module"]
pub struct ADDR_SPEC;
impl crate::RegisterSpec for ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [addr::R](R) reader structure"]
impl crate::Readable for ADDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [addr::W](W) writer structure"]
impl crate::Writable for ADDR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADDR to value 0"]
impl crate::Resettable for ADDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
