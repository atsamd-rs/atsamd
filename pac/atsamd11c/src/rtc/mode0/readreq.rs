#[doc = "Register `READREQ` reader"]
pub struct R(crate::R<READREQ_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<READREQ_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<READREQ_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<READREQ_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `READREQ` writer"]
pub struct W(crate::W<READREQ_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<READREQ_SPEC>;
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
impl From<crate::W<READREQ_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<READREQ_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADDR` reader - Address"]
pub struct ADDR_R(crate::FieldReader<u8, u8>);
impl ADDR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        ADDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADDR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RCONT` reader - Read Continuously"]
pub struct RCONT_R(crate::FieldReader<bool, bool>);
impl RCONT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RCONT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RCONT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RCONT` writer - Read Continuously"]
pub struct RCONT_W<'a> {
    w: &'a mut W,
}
impl<'a> RCONT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u16 & 0x01) << 14);
        self.w
    }
}
#[doc = "Field `RREQ` writer - Read Request"]
pub struct RREQ_W<'a> {
    w: &'a mut W,
}
impl<'a> RREQ_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u16 & 0x01) << 15);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - Address"]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 14 - Read Continuously"]
    #[inline(always)]
    pub fn rcont(&self) -> RCONT_R {
        RCONT_R::new(((self.bits >> 14) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 14 - Read Continuously"]
    #[inline(always)]
    pub fn rcont(&mut self) -> RCONT_W {
        RCONT_W { w: self }
    }
    #[doc = "Bit 15 - Read Request"]
    #[inline(always)]
    pub fn rreq(&mut self) -> RREQ_W {
        RREQ_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Read Request\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [readreq](index.html) module"]
pub struct READREQ_SPEC;
impl crate::RegisterSpec for READREQ_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [readreq::R](R) reader structure"]
impl crate::Readable for READREQ_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [readreq::W](W) writer structure"]
impl crate::Writable for READREQ_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets READREQ to value 0x10"]
impl crate::Resettable for READREQ_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x10
    }
}
