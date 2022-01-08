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
#[doc = "Field `BUSERR` reader - Bus Error"]
pub struct BUSERR_R(crate::FieldReader<bool, bool>);
impl BUSERR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BUSERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BUSERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BUSERR` writer - Bus Error"]
pub struct BUSERR_W<'a> {
    w: &'a mut W,
}
impl<'a> BUSERR_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | (value as u16 & 0x01);
        self.w
    }
}
#[doc = "Field `COLL` reader - Transmit Collision"]
pub struct COLL_R(crate::FieldReader<bool, bool>);
impl COLL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        COLL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COLL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COLL` writer - Transmit Collision"]
pub struct COLL_W<'a> {
    w: &'a mut W,
}
impl<'a> COLL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u16 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `RXNACK` reader - Received Not Acknowledge"]
pub struct RXNACK_R(crate::FieldReader<bool, bool>);
impl RXNACK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RXNACK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXNACK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXNACK` writer - Received Not Acknowledge"]
pub struct RXNACK_W<'a> {
    w: &'a mut W,
}
impl<'a> RXNACK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u16 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `DIR` reader - Read/Write Direction"]
pub struct DIR_R(crate::FieldReader<bool, bool>);
impl DIR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DIR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIR` writer - Read/Write Direction"]
pub struct DIR_W<'a> {
    w: &'a mut W,
}
impl<'a> DIR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u16 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `SR` reader - Repeated Start"]
pub struct SR_R(crate::FieldReader<bool, bool>);
impl SR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SR` writer - Repeated Start"]
pub struct SR_W<'a> {
    w: &'a mut W,
}
impl<'a> SR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u16 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `LOWTOUT` reader - SCL Low Timeout"]
pub struct LOWTOUT_R(crate::FieldReader<bool, bool>);
impl LOWTOUT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LOWTOUT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOWTOUT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOWTOUT` writer - SCL Low Timeout"]
pub struct LOWTOUT_W<'a> {
    w: &'a mut W,
}
impl<'a> LOWTOUT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u16 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `CLKHOLD` reader - Clock Hold"]
pub struct CLKHOLD_R(crate::FieldReader<bool, bool>);
impl CLKHOLD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CLKHOLD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLKHOLD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLKHOLD` writer - Clock Hold"]
pub struct CLKHOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKHOLD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u16 & 0x01) << 7);
        self.w
    }
}
#[doc = "Field `SEXTTOUT` reader - Slave SCL Low Extend Timeout"]
pub struct SEXTTOUT_R(crate::FieldReader<bool, bool>);
impl SEXTTOUT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SEXTTOUT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SEXTTOUT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SEXTTOUT` writer - Slave SCL Low Extend Timeout"]
pub struct SEXTTOUT_W<'a> {
    w: &'a mut W,
}
impl<'a> SEXTTOUT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u16 & 0x01) << 9);
        self.w
    }
}
#[doc = "Field `HS` reader - High Speed"]
pub struct HS_R(crate::FieldReader<bool, bool>);
impl HS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        HS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HS` writer - High Speed"]
pub struct HS_W<'a> {
    w: &'a mut W,
}
impl<'a> HS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u16 & 0x01) << 10);
        self.w
    }
}
#[doc = "Field `LENERR` reader - Transaction Length Error"]
pub struct LENERR_R(crate::FieldReader<bool, bool>);
impl LENERR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LENERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LENERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LENERR` writer - Transaction Length Error"]
pub struct LENERR_W<'a> {
    w: &'a mut W,
}
impl<'a> LENERR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u16 & 0x01) << 11);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Bus Error"]
    #[inline(always)]
    pub fn buserr(&self) -> BUSERR_R {
        BUSERR_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Transmit Collision"]
    #[inline(always)]
    pub fn coll(&self) -> COLL_R {
        COLL_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Received Not Acknowledge"]
    #[inline(always)]
    pub fn rxnack(&self) -> RXNACK_R {
        RXNACK_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Read/Write Direction"]
    #[inline(always)]
    pub fn dir(&self) -> DIR_R {
        DIR_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Repeated Start"]
    #[inline(always)]
    pub fn sr(&self) -> SR_R {
        SR_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 6 - SCL Low Timeout"]
    #[inline(always)]
    pub fn lowtout(&self) -> LOWTOUT_R {
        LOWTOUT_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Clock Hold"]
    #[inline(always)]
    pub fn clkhold(&self) -> CLKHOLD_R {
        CLKHOLD_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Slave SCL Low Extend Timeout"]
    #[inline(always)]
    pub fn sexttout(&self) -> SEXTTOUT_R {
        SEXTTOUT_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - High Speed"]
    #[inline(always)]
    pub fn hs(&self) -> HS_R {
        HS_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Transaction Length Error"]
    #[inline(always)]
    pub fn lenerr(&self) -> LENERR_R {
        LENERR_R::new(((self.bits >> 11) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Bus Error"]
    #[inline(always)]
    pub fn buserr(&mut self) -> BUSERR_W {
        BUSERR_W { w: self }
    }
    #[doc = "Bit 1 - Transmit Collision"]
    #[inline(always)]
    pub fn coll(&mut self) -> COLL_W {
        COLL_W { w: self }
    }
    #[doc = "Bit 2 - Received Not Acknowledge"]
    #[inline(always)]
    pub fn rxnack(&mut self) -> RXNACK_W {
        RXNACK_W { w: self }
    }
    #[doc = "Bit 3 - Read/Write Direction"]
    #[inline(always)]
    pub fn dir(&mut self) -> DIR_W {
        DIR_W { w: self }
    }
    #[doc = "Bit 4 - Repeated Start"]
    #[inline(always)]
    pub fn sr(&mut self) -> SR_W {
        SR_W { w: self }
    }
    #[doc = "Bit 6 - SCL Low Timeout"]
    #[inline(always)]
    pub fn lowtout(&mut self) -> LOWTOUT_W {
        LOWTOUT_W { w: self }
    }
    #[doc = "Bit 7 - Clock Hold"]
    #[inline(always)]
    pub fn clkhold(&mut self) -> CLKHOLD_W {
        CLKHOLD_W { w: self }
    }
    #[doc = "Bit 9 - Slave SCL Low Extend Timeout"]
    #[inline(always)]
    pub fn sexttout(&mut self) -> SEXTTOUT_W {
        SEXTTOUT_W { w: self }
    }
    #[doc = "Bit 10 - High Speed"]
    #[inline(always)]
    pub fn hs(&mut self) -> HS_W {
        HS_W { w: self }
    }
    #[doc = "Bit 11 - Transaction Length Error"]
    #[inline(always)]
    pub fn lenerr(&mut self) -> LENERR_W {
        LENERR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2CS Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status](index.html) module"]
pub struct STATUS_SPEC;
impl crate::RegisterSpec for STATUS_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [status::R](R) reader structure"]
impl crate::Readable for STATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [status::W](W) writer structure"]
impl crate::Writable for STATUS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
