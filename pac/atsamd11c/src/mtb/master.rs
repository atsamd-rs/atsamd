#[doc = "Register `MASTER` reader"]
pub struct R(crate::R<MASTER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MASTER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MASTER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MASTER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MASTER` writer"]
pub struct W(crate::W<MASTER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MASTER_SPEC>;
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
impl From<crate::W<MASTER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MASTER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MASK` reader - Maximum Value of the Trace Buffer in SRAM"]
pub struct MASK_R(crate::FieldReader<u8, u8>);
impl MASK_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        MASK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MASK_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MASK` writer - Maximum Value of the Trace Buffer in SRAM"]
pub struct MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> MASK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | (value as u32 & 0x1f);
        self.w
    }
}
#[doc = "Field `TSTARTEN` reader - Trace Start Input Enable"]
pub struct TSTARTEN_R(crate::FieldReader<bool, bool>);
impl TSTARTEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TSTARTEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSTARTEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSTARTEN` writer - Trace Start Input Enable"]
pub struct TSTARTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TSTARTEN_W<'a> {
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
#[doc = "Field `TSTOPEN` reader - Trace Stop Input Enable"]
pub struct TSTOPEN_R(crate::FieldReader<bool, bool>);
impl TSTOPEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TSTOPEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSTOPEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSTOPEN` writer - Trace Stop Input Enable"]
pub struct TSTOPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TSTOPEN_W<'a> {
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
#[doc = "Field `SFRWPRIV` reader - Special Function Register Write Privilege"]
pub struct SFRWPRIV_R(crate::FieldReader<bool, bool>);
impl SFRWPRIV_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SFRWPRIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SFRWPRIV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SFRWPRIV` writer - Special Function Register Write Privilege"]
pub struct SFRWPRIV_W<'a> {
    w: &'a mut W,
}
impl<'a> SFRWPRIV_W<'a> {
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
#[doc = "Field `RAMPRIV` reader - SRAM Privilege"]
pub struct RAMPRIV_R(crate::FieldReader<bool, bool>);
impl RAMPRIV_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RAMPRIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RAMPRIV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RAMPRIV` writer - SRAM Privilege"]
pub struct RAMPRIV_W<'a> {
    w: &'a mut W,
}
impl<'a> RAMPRIV_W<'a> {
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
#[doc = "Field `HALTREQ` reader - Halt Request"]
pub struct HALTREQ_R(crate::FieldReader<bool, bool>);
impl HALTREQ_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        HALTREQ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HALTREQ_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HALTREQ` writer - Halt Request"]
pub struct HALTREQ_W<'a> {
    w: &'a mut W,
}
impl<'a> HALTREQ_W<'a> {
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
#[doc = "Field `EN` reader - Main Trace Enable"]
pub struct EN_R(crate::FieldReader<bool, bool>);
impl EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EN` writer - Main Trace Enable"]
pub struct EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - Maximum Value of the Trace Buffer in SRAM"]
    #[inline(always)]
    pub fn mask(&self) -> MASK_R {
        MASK_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 5 - Trace Start Input Enable"]
    #[inline(always)]
    pub fn tstarten(&self) -> TSTARTEN_R {
        TSTARTEN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Trace Stop Input Enable"]
    #[inline(always)]
    pub fn tstopen(&self) -> TSTOPEN_R {
        TSTOPEN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Special Function Register Write Privilege"]
    #[inline(always)]
    pub fn sfrwpriv(&self) -> SFRWPRIV_R {
        SFRWPRIV_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - SRAM Privilege"]
    #[inline(always)]
    pub fn rampriv(&self) -> RAMPRIV_R {
        RAMPRIV_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Halt Request"]
    #[inline(always)]
    pub fn haltreq(&self) -> HALTREQ_R {
        HALTREQ_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Main Trace Enable"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Maximum Value of the Trace Buffer in SRAM"]
    #[inline(always)]
    pub fn mask(&mut self) -> MASK_W {
        MASK_W { w: self }
    }
    #[doc = "Bit 5 - Trace Start Input Enable"]
    #[inline(always)]
    pub fn tstarten(&mut self) -> TSTARTEN_W {
        TSTARTEN_W { w: self }
    }
    #[doc = "Bit 6 - Trace Stop Input Enable"]
    #[inline(always)]
    pub fn tstopen(&mut self) -> TSTOPEN_W {
        TSTOPEN_W { w: self }
    }
    #[doc = "Bit 7 - Special Function Register Write Privilege"]
    #[inline(always)]
    pub fn sfrwpriv(&mut self) -> SFRWPRIV_W {
        SFRWPRIV_W { w: self }
    }
    #[doc = "Bit 8 - SRAM Privilege"]
    #[inline(always)]
    pub fn rampriv(&mut self) -> RAMPRIV_W {
        RAMPRIV_W { w: self }
    }
    #[doc = "Bit 9 - Halt Request"]
    #[inline(always)]
    pub fn haltreq(&mut self) -> HALTREQ_W {
        HALTREQ_W { w: self }
    }
    #[doc = "Bit 31 - Main Trace Enable"]
    #[inline(always)]
    pub fn en(&mut self) -> EN_W {
        EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MTB Master\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [master](index.html) module"]
pub struct MASTER_SPEC;
impl crate::RegisterSpec for MASTER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [master::R](R) reader structure"]
impl crate::Readable for MASTER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [master::W](W) writer structure"]
impl crate::Writable for MASTER_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MASTER to value 0"]
impl crate::Resettable for MASTER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
