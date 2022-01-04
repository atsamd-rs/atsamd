#[doc = "Register `CTRLB` reader"]
pub struct R(crate::R<CTRLB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRLB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRLB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRLB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRLB` writer"]
pub struct W(crate::W<CTRLB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRLB_SPEC>;
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
impl From<crate::W<CTRLB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRLB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RESUME` reader - Send USB Resume"]
pub struct RESUME_R(crate::FieldReader<bool, bool>);
impl RESUME_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RESUME_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESUME_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESUME` writer - Send USB Resume"]
pub struct RESUME_W<'a> {
    w: &'a mut W,
}
impl<'a> RESUME_W<'a> {
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
#[doc = "Speed Configuration for Host\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SPDCONF_A {
    #[doc = "0: Normal mode: the host starts in full-speed mode and performs a high-speed reset to switch to the high speed mode if the downstream peripheral is high-speed capable."]
    NORMAL = 0,
    #[doc = "3: Full-speed: the host remains in full-speed mode whatever is the peripheral speed capability. Relevant in UTMI mode only."]
    FS = 3,
}
impl From<SPDCONF_A> for u8 {
    #[inline(always)]
    fn from(variant: SPDCONF_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SPDCONF` reader - Speed Configuration for Host"]
pub struct SPDCONF_R(crate::FieldReader<u8, SPDCONF_A>);
impl SPDCONF_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SPDCONF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SPDCONF_A> {
        match self.bits {
            0 => Some(SPDCONF_A::NORMAL),
            3 => Some(SPDCONF_A::FS),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        **self == SPDCONF_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `FS`"]
    #[inline(always)]
    pub fn is_fs(&self) -> bool {
        **self == SPDCONF_A::FS
    }
}
impl core::ops::Deref for SPDCONF_R {
    type Target = crate::FieldReader<u8, SPDCONF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPDCONF` writer - Speed Configuration for Host"]
pub struct SPDCONF_W<'a> {
    w: &'a mut W,
}
impl<'a> SPDCONF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPDCONF_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Normal mode: the host starts in full-speed mode and performs a high-speed reset to switch to the high speed mode if the downstream peripheral is high-speed capable."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(SPDCONF_A::NORMAL)
    }
    #[doc = "Full-speed: the host remains in full-speed mode whatever is the peripheral speed capability. Relevant in UTMI mode only."]
    #[inline(always)]
    pub fn fs(self) -> &'a mut W {
        self.variant(SPDCONF_A::FS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | ((value as u16 & 0x03) << 2);
        self.w
    }
}
#[doc = "Field `AUTORESUME` reader - Auto Resume Enable"]
pub struct AUTORESUME_R(crate::FieldReader<bool, bool>);
impl AUTORESUME_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        AUTORESUME_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AUTORESUME_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AUTORESUME` writer - Auto Resume Enable"]
pub struct AUTORESUME_W<'a> {
    w: &'a mut W,
}
impl<'a> AUTORESUME_W<'a> {
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
#[doc = "Field `TSTJ` reader - Test mode J"]
pub struct TSTJ_R(crate::FieldReader<bool, bool>);
impl TSTJ_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TSTJ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSTJ_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSTJ` writer - Test mode J"]
pub struct TSTJ_W<'a> {
    w: &'a mut W,
}
impl<'a> TSTJ_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u16 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `TSTK` reader - Test mode K"]
pub struct TSTK_R(crate::FieldReader<bool, bool>);
impl TSTK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TSTK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSTK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSTK` writer - Test mode K"]
pub struct TSTK_W<'a> {
    w: &'a mut W,
}
impl<'a> TSTK_W<'a> {
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
#[doc = "Field `SOFE` reader - Start of Frame Generation Enable"]
pub struct SOFE_R(crate::FieldReader<bool, bool>);
impl SOFE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SOFE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SOFE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SOFE` writer - Start of Frame Generation Enable"]
pub struct SOFE_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u16 & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `BUSRESET` reader - Send USB Reset"]
pub struct BUSRESET_R(crate::FieldReader<bool, bool>);
impl BUSRESET_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BUSRESET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BUSRESET_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BUSRESET` writer - Send USB Reset"]
pub struct BUSRESET_W<'a> {
    w: &'a mut W,
}
impl<'a> BUSRESET_W<'a> {
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
#[doc = "Field `VBUSOK` reader - VBUS is OK"]
pub struct VBUSOK_R(crate::FieldReader<bool, bool>);
impl VBUSOK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        VBUSOK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VBUSOK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VBUSOK` writer - VBUS is OK"]
pub struct VBUSOK_W<'a> {
    w: &'a mut W,
}
impl<'a> VBUSOK_W<'a> {
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
#[doc = "Field `L1RESUME` reader - Send L1 Resume"]
pub struct L1RESUME_R(crate::FieldReader<bool, bool>);
impl L1RESUME_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        L1RESUME_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for L1RESUME_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `L1RESUME` writer - Send L1 Resume"]
pub struct L1RESUME_W<'a> {
    w: &'a mut W,
}
impl<'a> L1RESUME_W<'a> {
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
    #[doc = "Bit 1 - Send USB Resume"]
    #[inline(always)]
    pub fn resume(&self) -> RESUME_R {
        RESUME_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 2:3 - Speed Configuration for Host"]
    #[inline(always)]
    pub fn spdconf(&self) -> SPDCONF_R {
        SPDCONF_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bit 4 - Auto Resume Enable"]
    #[inline(always)]
    pub fn autoresume(&self) -> AUTORESUME_R {
        AUTORESUME_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Test mode J"]
    #[inline(always)]
    pub fn tstj(&self) -> TSTJ_R {
        TSTJ_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Test mode K"]
    #[inline(always)]
    pub fn tstk(&self) -> TSTK_R {
        TSTK_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Start of Frame Generation Enable"]
    #[inline(always)]
    pub fn sofe(&self) -> SOFE_R {
        SOFE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Send USB Reset"]
    #[inline(always)]
    pub fn busreset(&self) -> BUSRESET_R {
        BUSRESET_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - VBUS is OK"]
    #[inline(always)]
    pub fn vbusok(&self) -> VBUSOK_R {
        VBUSOK_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Send L1 Resume"]
    #[inline(always)]
    pub fn l1resume(&self) -> L1RESUME_R {
        L1RESUME_R::new(((self.bits >> 11) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Send USB Resume"]
    #[inline(always)]
    pub fn resume(&mut self) -> RESUME_W {
        RESUME_W { w: self }
    }
    #[doc = "Bits 2:3 - Speed Configuration for Host"]
    #[inline(always)]
    pub fn spdconf(&mut self) -> SPDCONF_W {
        SPDCONF_W { w: self }
    }
    #[doc = "Bit 4 - Auto Resume Enable"]
    #[inline(always)]
    pub fn autoresume(&mut self) -> AUTORESUME_W {
        AUTORESUME_W { w: self }
    }
    #[doc = "Bit 5 - Test mode J"]
    #[inline(always)]
    pub fn tstj(&mut self) -> TSTJ_W {
        TSTJ_W { w: self }
    }
    #[doc = "Bit 6 - Test mode K"]
    #[inline(always)]
    pub fn tstk(&mut self) -> TSTK_W {
        TSTK_W { w: self }
    }
    #[doc = "Bit 8 - Start of Frame Generation Enable"]
    #[inline(always)]
    pub fn sofe(&mut self) -> SOFE_W {
        SOFE_W { w: self }
    }
    #[doc = "Bit 9 - Send USB Reset"]
    #[inline(always)]
    pub fn busreset(&mut self) -> BUSRESET_W {
        BUSRESET_W { w: self }
    }
    #[doc = "Bit 10 - VBUS is OK"]
    #[inline(always)]
    pub fn vbusok(&mut self) -> VBUSOK_W {
        VBUSOK_W { w: self }
    }
    #[doc = "Bit 11 - Send L1 Resume"]
    #[inline(always)]
    pub fn l1resume(&mut self) -> L1RESUME_W {
        L1RESUME_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "HOST Control B\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrlb](index.html) module"]
pub struct CTRLB_SPEC;
impl crate::RegisterSpec for CTRLB_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [ctrlb::R](R) reader structure"]
impl crate::Readable for CTRLB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrlb::W](W) writer structure"]
impl crate::Writable for CTRLB_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTRLB to value 0"]
impl crate::Resettable for CTRLB_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
