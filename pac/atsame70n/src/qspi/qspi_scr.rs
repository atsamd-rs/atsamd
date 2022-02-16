#[doc = "Register `QSPI_SCR` reader"]
pub struct R(crate::R<QSPI_SCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<QSPI_SCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<QSPI_SCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<QSPI_SCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `QSPI_SCR` writer"]
pub struct W(crate::W<QSPI_SCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<QSPI_SCR_SPEC>;
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
impl From<crate::W<QSPI_SCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<QSPI_SCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CPOL` reader - Clock Polarity"]
pub struct CPOL_R(crate::FieldReader<bool, bool>);
impl CPOL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CPOL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CPOL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CPOL` writer - Clock Polarity"]
pub struct CPOL_W<'a> {
    w: &'a mut W,
}
impl<'a> CPOL_W<'a> {
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
#[doc = "Field `CPHA` reader - Clock Phase"]
pub struct CPHA_R(crate::FieldReader<bool, bool>);
impl CPHA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CPHA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CPHA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CPHA` writer - Clock Phase"]
pub struct CPHA_W<'a> {
    w: &'a mut W,
}
impl<'a> CPHA_W<'a> {
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
#[doc = "Field `SCBR` reader - Serial Clock Baud Rate"]
pub struct SCBR_R(crate::FieldReader<u8, u8>);
impl SCBR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SCBR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCBR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCBR` writer - Serial Clock Baud Rate"]
pub struct SCBR_W<'a> {
    w: &'a mut W,
}
impl<'a> SCBR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `DLYBS` reader - Delay Before QSCK"]
pub struct DLYBS_R(crate::FieldReader<u8, u8>);
impl DLYBS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DLYBS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DLYBS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DLYBS` writer - Delay Before QSCK"]
pub struct DLYBS_W<'a> {
    w: &'a mut W,
}
impl<'a> DLYBS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Clock Polarity"]
    #[inline(always)]
    pub fn cpol(&self) -> CPOL_R {
        CPOL_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Clock Phase"]
    #[inline(always)]
    pub fn cpha(&self) -> CPHA_R {
        CPHA_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 8:15 - Serial Clock Baud Rate"]
    #[inline(always)]
    pub fn scbr(&self) -> SCBR_R {
        SCBR_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Delay Before QSCK"]
    #[inline(always)]
    pub fn dlybs(&self) -> DLYBS_R {
        DLYBS_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Clock Polarity"]
    #[inline(always)]
    pub fn cpol(&mut self) -> CPOL_W {
        CPOL_W { w: self }
    }
    #[doc = "Bit 1 - Clock Phase"]
    #[inline(always)]
    pub fn cpha(&mut self) -> CPHA_W {
        CPHA_W { w: self }
    }
    #[doc = "Bits 8:15 - Serial Clock Baud Rate"]
    #[inline(always)]
    pub fn scbr(&mut self) -> SCBR_W {
        SCBR_W { w: self }
    }
    #[doc = "Bits 16:23 - Delay Before QSCK"]
    #[inline(always)]
    pub fn dlybs(&mut self) -> DLYBS_W {
        DLYBS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Serial Clock Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [qspi_scr](index.html) module"]
pub struct QSPI_SCR_SPEC;
impl crate::RegisterSpec for QSPI_SCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [qspi_scr::R](R) reader structure"]
impl crate::Readable for QSPI_SCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [qspi_scr::W](W) writer structure"]
impl crate::Writable for QSPI_SCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets QSPI_SCR to value 0"]
impl crate::Resettable for QSPI_SCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
