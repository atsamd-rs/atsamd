#[doc = "Register `CTRLC` reader"]
pub struct R(crate::R<CTRLC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRLC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRLC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRLC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRLC` writer"]
pub struct W(crate::W<CTRLC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRLC_SPEC>;
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
impl From<crate::W<CTRLC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRLC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GTIME` reader - Guard Time"]
pub struct GTIME_R(crate::FieldReader<u8, u8>);
impl GTIME_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        GTIME_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GTIME_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GTIME` writer - Guard Time"]
pub struct GTIME_W<'a> {
    w: &'a mut W,
}
impl<'a> GTIME_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
#[doc = "Field `BRKLEN` reader - LIN Master Break Length"]
pub struct BRKLEN_R(crate::FieldReader<u8, u8>);
impl BRKLEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        BRKLEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BRKLEN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BRKLEN` writer - LIN Master Break Length"]
pub struct BRKLEN_W<'a> {
    w: &'a mut W,
}
impl<'a> BRKLEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | ((value as u32 & 0x03) << 8);
        self.w
    }
}
#[doc = "Field `HDRDLY` reader - LIN Master Header Delay"]
pub struct HDRDLY_R(crate::FieldReader<u8, u8>);
impl HDRDLY_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        HDRDLY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HDRDLY_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HDRDLY` writer - LIN Master Header Delay"]
pub struct HDRDLY_W<'a> {
    w: &'a mut W,
}
impl<'a> HDRDLY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | ((value as u32 & 0x03) << 10);
        self.w
    }
}
#[doc = "Field `INACK` reader - Inhibit Not Acknowledge"]
pub struct INACK_R(crate::FieldReader<bool, bool>);
impl INACK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INACK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INACK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INACK` writer - Inhibit Not Acknowledge"]
pub struct INACK_W<'a> {
    w: &'a mut W,
}
impl<'a> INACK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Field `DSNACK` reader - Disable Successive NACK"]
pub struct DSNACK_R(crate::FieldReader<bool, bool>);
impl DSNACK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DSNACK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DSNACK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DSNACK` writer - Disable Successive NACK"]
pub struct DSNACK_W<'a> {
    w: &'a mut W,
}
impl<'a> DSNACK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "Field `MAXITER` reader - Maximum Iterations"]
pub struct MAXITER_R(crate::FieldReader<u8, u8>);
impl MAXITER_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        MAXITER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MAXITER_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MAXITER` writer - Maximum Iterations"]
pub struct MAXITER_W<'a> {
    w: &'a mut W,
}
impl<'a> MAXITER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 20)) | ((value as u32 & 0x07) << 20);
        self.w
    }
}
#[doc = "Data 32 Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DATA32B_A {
    #[doc = "0: Data reads and writes according CTRLB.CHSIZE"]
    DATA_READ_WRITE_CHSIZE = 0,
    #[doc = "1: Data reads according CTRLB.CHSIZE and writes according 32-bit extension"]
    DATA_READ_CHSIZE_WRITE_32BIT = 1,
    #[doc = "2: Data reads according 32-bit extension and writes according CTRLB.CHSIZE"]
    DATA_READ_32BIT_WRITE_CHSIZE = 2,
    #[doc = "3: Data reads and writes according 32-bit extension"]
    DATA_READ_WRITE_32BIT = 3,
}
impl From<DATA32B_A> for u8 {
    #[inline(always)]
    fn from(variant: DATA32B_A) -> Self {
        variant as _
    }
}
#[doc = "Field `DATA32B` reader - Data 32 Bit"]
pub struct DATA32B_R(crate::FieldReader<u8, DATA32B_A>);
impl DATA32B_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DATA32B_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DATA32B_A {
        match self.bits {
            0 => DATA32B_A::DATA_READ_WRITE_CHSIZE,
            1 => DATA32B_A::DATA_READ_CHSIZE_WRITE_32BIT,
            2 => DATA32B_A::DATA_READ_32BIT_WRITE_CHSIZE,
            3 => DATA32B_A::DATA_READ_WRITE_32BIT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DATA_READ_WRITE_CHSIZE`"]
    #[inline(always)]
    pub fn is_data_read_write_chsize(&self) -> bool {
        **self == DATA32B_A::DATA_READ_WRITE_CHSIZE
    }
    #[doc = "Checks if the value of the field is `DATA_READ_CHSIZE_WRITE_32BIT`"]
    #[inline(always)]
    pub fn is_data_read_chsize_write_32bit(&self) -> bool {
        **self == DATA32B_A::DATA_READ_CHSIZE_WRITE_32BIT
    }
    #[doc = "Checks if the value of the field is `DATA_READ_32BIT_WRITE_CHSIZE`"]
    #[inline(always)]
    pub fn is_data_read_32bit_write_chsize(&self) -> bool {
        **self == DATA32B_A::DATA_READ_32BIT_WRITE_CHSIZE
    }
    #[doc = "Checks if the value of the field is `DATA_READ_WRITE_32BIT`"]
    #[inline(always)]
    pub fn is_data_read_write_32bit(&self) -> bool {
        **self == DATA32B_A::DATA_READ_WRITE_32BIT
    }
}
impl core::ops::Deref for DATA32B_R {
    type Target = crate::FieldReader<u8, DATA32B_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DATA32B` writer - Data 32 Bit"]
pub struct DATA32B_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA32B_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DATA32B_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Data reads and writes according CTRLB.CHSIZE"]
    #[inline(always)]
    pub fn data_read_write_chsize(self) -> &'a mut W {
        self.variant(DATA32B_A::DATA_READ_WRITE_CHSIZE)
    }
    #[doc = "Data reads according CTRLB.CHSIZE and writes according 32-bit extension"]
    #[inline(always)]
    pub fn data_read_chsize_write_32bit(self) -> &'a mut W {
        self.variant(DATA32B_A::DATA_READ_CHSIZE_WRITE_32BIT)
    }
    #[doc = "Data reads according 32-bit extension and writes according CTRLB.CHSIZE"]
    #[inline(always)]
    pub fn data_read_32bit_write_chsize(self) -> &'a mut W {
        self.variant(DATA32B_A::DATA_READ_32BIT_WRITE_CHSIZE)
    }
    #[doc = "Data reads and writes according 32-bit extension"]
    #[inline(always)]
    pub fn data_read_write_32bit(self) -> &'a mut W {
        self.variant(DATA32B_A::DATA_READ_WRITE_32BIT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | ((value as u32 & 0x03) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Guard Time"]
    #[inline(always)]
    pub fn gtime(&self) -> GTIME_R {
        GTIME_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 8:9 - LIN Master Break Length"]
    #[inline(always)]
    pub fn brklen(&self) -> BRKLEN_R {
        BRKLEN_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 10:11 - LIN Master Header Delay"]
    #[inline(always)]
    pub fn hdrdly(&self) -> HDRDLY_R {
        HDRDLY_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bit 16 - Inhibit Not Acknowledge"]
    #[inline(always)]
    pub fn inack(&self) -> INACK_R {
        INACK_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Disable Successive NACK"]
    #[inline(always)]
    pub fn dsnack(&self) -> DSNACK_R {
        DSNACK_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bits 20:22 - Maximum Iterations"]
    #[inline(always)]
    pub fn maxiter(&self) -> MAXITER_R {
        MAXITER_R::new(((self.bits >> 20) & 0x07) as u8)
    }
    #[doc = "Bits 24:25 - Data 32 Bit"]
    #[inline(always)]
    pub fn data32b(&self) -> DATA32B_R {
        DATA32B_R::new(((self.bits >> 24) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Guard Time"]
    #[inline(always)]
    pub fn gtime(&mut self) -> GTIME_W {
        GTIME_W { w: self }
    }
    #[doc = "Bits 8:9 - LIN Master Break Length"]
    #[inline(always)]
    pub fn brklen(&mut self) -> BRKLEN_W {
        BRKLEN_W { w: self }
    }
    #[doc = "Bits 10:11 - LIN Master Header Delay"]
    #[inline(always)]
    pub fn hdrdly(&mut self) -> HDRDLY_W {
        HDRDLY_W { w: self }
    }
    #[doc = "Bit 16 - Inhibit Not Acknowledge"]
    #[inline(always)]
    pub fn inack(&mut self) -> INACK_W {
        INACK_W { w: self }
    }
    #[doc = "Bit 17 - Disable Successive NACK"]
    #[inline(always)]
    pub fn dsnack(&mut self) -> DSNACK_W {
        DSNACK_W { w: self }
    }
    #[doc = "Bits 20:22 - Maximum Iterations"]
    #[inline(always)]
    pub fn maxiter(&mut self) -> MAXITER_W {
        MAXITER_W { w: self }
    }
    #[doc = "Bits 24:25 - Data 32 Bit"]
    #[inline(always)]
    pub fn data32b(&mut self) -> DATA32B_W {
        DATA32B_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USART_EXT Control C\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrlc](index.html) module"]
pub struct CTRLC_SPEC;
impl crate::RegisterSpec for CTRLC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrlc::R](R) reader structure"]
impl crate::Readable for CTRLC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrlc::W](W) writer structure"]
impl crate::Writable for CTRLC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTRLC to value 0"]
impl crate::Resettable for CTRLC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
