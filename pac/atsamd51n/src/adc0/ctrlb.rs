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
#[doc = "Field `LEFTADJ` reader - Left-Adjusted Result"]
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
#[doc = "Field `LEFTADJ` writer - Left-Adjusted Result"]
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
        self.w.bits = (self.w.bits & !0x01) | (value as u16 & 0x01);
        self.w
    }
}
#[doc = "Field `FREERUN` reader - Free Running Mode"]
pub struct FREERUN_R(crate::FieldReader<bool, bool>);
impl FREERUN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FREERUN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FREERUN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FREERUN` writer - Free Running Mode"]
pub struct FREERUN_W<'a> {
    w: &'a mut W,
}
impl<'a> FREERUN_W<'a> {
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
#[doc = "Field `CORREN` reader - Digital Correction Logic Enable"]
pub struct CORREN_R(crate::FieldReader<bool, bool>);
impl CORREN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CORREN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORREN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORREN` writer - Digital Correction Logic Enable"]
pub struct CORREN_W<'a> {
    w: &'a mut W,
}
impl<'a> CORREN_W<'a> {
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
#[doc = "Conversion Result Resolution\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RESSEL_A {
    #[doc = "0: 12-bit result"]
    _12BIT = 0,
    #[doc = "1: For averaging mode output"]
    _16BIT = 1,
    #[doc = "2: 10-bit result"]
    _10BIT = 2,
    #[doc = "3: 8-bit result"]
    _8BIT = 3,
}
impl From<RESSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: RESSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `RESSEL` reader - Conversion Result Resolution"]
pub struct RESSEL_R(crate::FieldReader<u8, RESSEL_A>);
impl RESSEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RESSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RESSEL_A {
        match self.bits {
            0 => RESSEL_A::_12BIT,
            1 => RESSEL_A::_16BIT,
            2 => RESSEL_A::_10BIT,
            3 => RESSEL_A::_8BIT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_12BIT`"]
    #[inline(always)]
    pub fn is_12bit(&self) -> bool {
        **self == RESSEL_A::_12BIT
    }
    #[doc = "Checks if the value of the field is `_16BIT`"]
    #[inline(always)]
    pub fn is_16bit(&self) -> bool {
        **self == RESSEL_A::_16BIT
    }
    #[doc = "Checks if the value of the field is `_10BIT`"]
    #[inline(always)]
    pub fn is_10bit(&self) -> bool {
        **self == RESSEL_A::_10BIT
    }
    #[doc = "Checks if the value of the field is `_8BIT`"]
    #[inline(always)]
    pub fn is_8bit(&self) -> bool {
        **self == RESSEL_A::_8BIT
    }
}
impl core::ops::Deref for RESSEL_R {
    type Target = crate::FieldReader<u8, RESSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESSEL` writer - Conversion Result Resolution"]
pub struct RESSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> RESSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RESSEL_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "12-bit result"]
    #[inline(always)]
    pub fn _12bit(self) -> &'a mut W {
        self.variant(RESSEL_A::_12BIT)
    }
    #[doc = "For averaging mode output"]
    #[inline(always)]
    pub fn _16bit(self) -> &'a mut W {
        self.variant(RESSEL_A::_16BIT)
    }
    #[doc = "10-bit result"]
    #[inline(always)]
    pub fn _10bit(self) -> &'a mut W {
        self.variant(RESSEL_A::_10BIT)
    }
    #[doc = "8-bit result"]
    #[inline(always)]
    pub fn _8bit(self) -> &'a mut W {
        self.variant(RESSEL_A::_8BIT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 3)) | ((value as u16 & 0x03) << 3);
        self.w
    }
}
#[doc = "Window Monitor Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum WINMODE_A {
    #[doc = "0: No window mode (default)"]
    DISABLE = 0,
    #[doc = "1: RESULT > WINLT"]
    MODE1 = 1,
    #[doc = "2: RESULT < WINUT"]
    MODE2 = 2,
    #[doc = "3: WINLT < RESULT < WINUT"]
    MODE3 = 3,
    #[doc = "4: !(WINLT < RESULT < WINUT)"]
    MODE4 = 4,
}
impl From<WINMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: WINMODE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `WINMODE` reader - Window Monitor Mode"]
pub struct WINMODE_R(crate::FieldReader<u8, WINMODE_A>);
impl WINMODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        WINMODE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<WINMODE_A> {
        match self.bits {
            0 => Some(WINMODE_A::DISABLE),
            1 => Some(WINMODE_A::MODE1),
            2 => Some(WINMODE_A::MODE2),
            3 => Some(WINMODE_A::MODE3),
            4 => Some(WINMODE_A::MODE4),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == WINMODE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `MODE1`"]
    #[inline(always)]
    pub fn is_mode1(&self) -> bool {
        **self == WINMODE_A::MODE1
    }
    #[doc = "Checks if the value of the field is `MODE2`"]
    #[inline(always)]
    pub fn is_mode2(&self) -> bool {
        **self == WINMODE_A::MODE2
    }
    #[doc = "Checks if the value of the field is `MODE3`"]
    #[inline(always)]
    pub fn is_mode3(&self) -> bool {
        **self == WINMODE_A::MODE3
    }
    #[doc = "Checks if the value of the field is `MODE4`"]
    #[inline(always)]
    pub fn is_mode4(&self) -> bool {
        **self == WINMODE_A::MODE4
    }
}
impl core::ops::Deref for WINMODE_R {
    type Target = crate::FieldReader<u8, WINMODE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WINMODE` writer - Window Monitor Mode"]
pub struct WINMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> WINMODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WINMODE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "No window mode (default)"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(WINMODE_A::DISABLE)
    }
    #[doc = "RESULT > WINLT"]
    #[inline(always)]
    pub fn mode1(self) -> &'a mut W {
        self.variant(WINMODE_A::MODE1)
    }
    #[doc = "RESULT < WINUT"]
    #[inline(always)]
    pub fn mode2(self) -> &'a mut W {
        self.variant(WINMODE_A::MODE2)
    }
    #[doc = "WINLT < RESULT < WINUT"]
    #[inline(always)]
    pub fn mode3(self) -> &'a mut W {
        self.variant(WINMODE_A::MODE3)
    }
    #[doc = "!(WINLT < RESULT < WINUT)"]
    #[inline(always)]
    pub fn mode4(self) -> &'a mut W {
        self.variant(WINMODE_A::MODE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | ((value as u16 & 0x07) << 8);
        self.w
    }
}
#[doc = "Field `WINSS` reader - Window Single Sample"]
pub struct WINSS_R(crate::FieldReader<bool, bool>);
impl WINSS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WINSS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WINSS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WINSS` writer - Window Single Sample"]
pub struct WINSS_W<'a> {
    w: &'a mut W,
}
impl<'a> WINSS_W<'a> {
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
    #[doc = "Bit 0 - Left-Adjusted Result"]
    #[inline(always)]
    pub fn leftadj(&self) -> LEFTADJ_R {
        LEFTADJ_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Free Running Mode"]
    #[inline(always)]
    pub fn freerun(&self) -> FREERUN_R {
        FREERUN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Digital Correction Logic Enable"]
    #[inline(always)]
    pub fn corren(&self) -> CORREN_R {
        CORREN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 3:4 - Conversion Result Resolution"]
    #[inline(always)]
    pub fn ressel(&self) -> RESSEL_R {
        RESSEL_R::new(((self.bits >> 3) & 0x03) as u8)
    }
    #[doc = "Bits 8:10 - Window Monitor Mode"]
    #[inline(always)]
    pub fn winmode(&self) -> WINMODE_R {
        WINMODE_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bit 11 - Window Single Sample"]
    #[inline(always)]
    pub fn winss(&self) -> WINSS_R {
        WINSS_R::new(((self.bits >> 11) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Left-Adjusted Result"]
    #[inline(always)]
    pub fn leftadj(&mut self) -> LEFTADJ_W {
        LEFTADJ_W { w: self }
    }
    #[doc = "Bit 1 - Free Running Mode"]
    #[inline(always)]
    pub fn freerun(&mut self) -> FREERUN_W {
        FREERUN_W { w: self }
    }
    #[doc = "Bit 2 - Digital Correction Logic Enable"]
    #[inline(always)]
    pub fn corren(&mut self) -> CORREN_W {
        CORREN_W { w: self }
    }
    #[doc = "Bits 3:4 - Conversion Result Resolution"]
    #[inline(always)]
    pub fn ressel(&mut self) -> RESSEL_W {
        RESSEL_W { w: self }
    }
    #[doc = "Bits 8:10 - Window Monitor Mode"]
    #[inline(always)]
    pub fn winmode(&mut self) -> WINMODE_W {
        WINMODE_W { w: self }
    }
    #[doc = "Bit 11 - Window Single Sample"]
    #[inline(always)]
    pub fn winss(&mut self) -> WINSS_W {
        WINSS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control B\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrlb](index.html) module"]
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
