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
#[doc = "Field `EOEN` reader - External Output Enable"]
pub struct EOEN_R(crate::FieldReader<bool, bool>);
impl EOEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EOEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EOEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EOEN` writer - External Output Enable"]
pub struct EOEN_W<'a> {
    w: &'a mut W,
}
impl<'a> EOEN_W<'a> {
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
#[doc = "Field `IOEN` reader - Internal Output Enable"]
pub struct IOEN_R(crate::FieldReader<bool, bool>);
impl IOEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IOEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IOEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IOEN` writer - Internal Output Enable"]
pub struct IOEN_W<'a> {
    w: &'a mut W,
}
impl<'a> IOEN_W<'a> {
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
#[doc = "Field `LEFTADJ` reader - Left Adjusted Data"]
pub struct LEFTADJ_R(crate::FieldReader<bool, bool>);
impl LEFTADJ_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LEFTADJ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LEFTADJ_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LEFTADJ` writer - Left Adjusted Data"]
pub struct LEFTADJ_W<'a> {
    w: &'a mut W,
}
impl<'a> LEFTADJ_W<'a> {
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
#[doc = "Field `VPD` reader - Voltage Pump Disable"]
pub struct VPD_R(crate::FieldReader<bool, bool>);
impl VPD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        VPD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VPD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VPD` writer - Voltage Pump Disable"]
pub struct VPD_W<'a> {
    w: &'a mut W,
}
impl<'a> VPD_W<'a> {
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
#[doc = "Field `BDWP` reader - Bypass DATABUF Write Protection"]
pub struct BDWP_R(crate::FieldReader<bool, bool>);
impl BDWP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BDWP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BDWP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BDWP` writer - Bypass DATABUF Write Protection"]
pub struct BDWP_W<'a> {
    w: &'a mut W,
}
impl<'a> BDWP_W<'a> {
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
#[doc = "Reference Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum REFSEL_A {
    #[doc = "0: Internal 1.0V reference"]
    INT1V = 0,
    #[doc = "1: AVCC"]
    AVCC = 1,
    #[doc = "2: External reference"]
    VREFP = 2,
}
impl From<REFSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: REFSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `REFSEL` reader - Reference Selection"]
pub struct REFSEL_R(crate::FieldReader<u8, REFSEL_A>);
impl REFSEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        REFSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<REFSEL_A> {
        match self.bits {
            0 => Some(REFSEL_A::INT1V),
            1 => Some(REFSEL_A::AVCC),
            2 => Some(REFSEL_A::VREFP),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `INT1V`"]
    #[inline(always)]
    pub fn is_int1v(&self) -> bool {
        **self == REFSEL_A::INT1V
    }
    #[doc = "Checks if the value of the field is `AVCC`"]
    #[inline(always)]
    pub fn is_avcc(&self) -> bool {
        **self == REFSEL_A::AVCC
    }
    #[doc = "Checks if the value of the field is `VREFP`"]
    #[inline(always)]
    pub fn is_vrefp(&self) -> bool {
        **self == REFSEL_A::VREFP
    }
}
impl core::ops::Deref for REFSEL_R {
    type Target = crate::FieldReader<u8, REFSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REFSEL` writer - Reference Selection"]
pub struct REFSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> REFSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REFSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Internal 1.0V reference"]
    #[inline(always)]
    pub fn int1v(self) -> &'a mut W {
        self.variant(REFSEL_A::INT1V)
    }
    #[doc = "AVCC"]
    #[inline(always)]
    pub fn avcc(self) -> &'a mut W {
        self.variant(REFSEL_A::AVCC)
    }
    #[doc = "External reference"]
    #[inline(always)]
    pub fn vrefp(self) -> &'a mut W {
        self.variant(REFSEL_A::VREFP)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | ((value as u8 & 0x03) << 6);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - External Output Enable"]
    #[inline(always)]
    pub fn eoen(&self) -> EOEN_R {
        EOEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Internal Output Enable"]
    #[inline(always)]
    pub fn ioen(&self) -> IOEN_R {
        IOEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Left Adjusted Data"]
    #[inline(always)]
    pub fn leftadj(&self) -> LEFTADJ_R {
        LEFTADJ_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Voltage Pump Disable"]
    #[inline(always)]
    pub fn vpd(&self) -> VPD_R {
        VPD_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Bypass DATABUF Write Protection"]
    #[inline(always)]
    pub fn bdwp(&self) -> BDWP_R {
        BDWP_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 6:7 - Reference Selection"]
    #[inline(always)]
    pub fn refsel(&self) -> REFSEL_R {
        REFSEL_R::new(((self.bits >> 6) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - External Output Enable"]
    #[inline(always)]
    pub fn eoen(&mut self) -> EOEN_W {
        EOEN_W { w: self }
    }
    #[doc = "Bit 1 - Internal Output Enable"]
    #[inline(always)]
    pub fn ioen(&mut self) -> IOEN_W {
        IOEN_W { w: self }
    }
    #[doc = "Bit 2 - Left Adjusted Data"]
    #[inline(always)]
    pub fn leftadj(&mut self) -> LEFTADJ_W {
        LEFTADJ_W { w: self }
    }
    #[doc = "Bit 3 - Voltage Pump Disable"]
    #[inline(always)]
    pub fn vpd(&mut self) -> VPD_W {
        VPD_W { w: self }
    }
    #[doc = "Bit 4 - Bypass DATABUF Write Protection"]
    #[inline(always)]
    pub fn bdwp(&mut self) -> BDWP_W {
        BDWP_W { w: self }
    }
    #[doc = "Bits 6:7 - Reference Selection"]
    #[inline(always)]
    pub fn refsel(&mut self) -> REFSEL_W {
        REFSEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control B\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrlb](index.html) module"]
pub struct CTRLB_SPEC;
impl crate::RegisterSpec for CTRLB_SPEC {
    type Ux = u8;
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
