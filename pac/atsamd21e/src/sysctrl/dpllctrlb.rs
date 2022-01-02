#[doc = "Register `DPLLCTRLB` reader"]
pub struct R(crate::R<DPLLCTRLB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DPLLCTRLB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DPLLCTRLB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DPLLCTRLB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DPLLCTRLB` writer"]
pub struct W(crate::W<DPLLCTRLB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DPLLCTRLB_SPEC>;
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
impl From<crate::W<DPLLCTRLB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DPLLCTRLB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Proportional Integral Filter Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FILTER_A {
    #[doc = "0: Default filter mode"]
    DEFAULT = 0,
    #[doc = "1: Low bandwidth filter"]
    LBFILT = 1,
    #[doc = "2: High bandwidth filter"]
    HBFILT = 2,
    #[doc = "3: High damping filter"]
    HDFILT = 3,
}
impl From<FILTER_A> for u8 {
    #[inline(always)]
    fn from(variant: FILTER_A) -> Self {
        variant as _
    }
}
#[doc = "Field `FILTER` reader - Proportional Integral Filter Selection"]
pub struct FILTER_R(crate::FieldReader<u8, FILTER_A>);
impl FILTER_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        FILTER_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FILTER_A {
        match self.bits {
            0 => FILTER_A::DEFAULT,
            1 => FILTER_A::LBFILT,
            2 => FILTER_A::HBFILT,
            3 => FILTER_A::HDFILT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DEFAULT`"]
    #[inline(always)]
    pub fn is_default(&self) -> bool {
        **self == FILTER_A::DEFAULT
    }
    #[doc = "Checks if the value of the field is `LBFILT`"]
    #[inline(always)]
    pub fn is_lbfilt(&self) -> bool {
        **self == FILTER_A::LBFILT
    }
    #[doc = "Checks if the value of the field is `HBFILT`"]
    #[inline(always)]
    pub fn is_hbfilt(&self) -> bool {
        **self == FILTER_A::HBFILT
    }
    #[doc = "Checks if the value of the field is `HDFILT`"]
    #[inline(always)]
    pub fn is_hdfilt(&self) -> bool {
        **self == FILTER_A::HDFILT
    }
}
impl core::ops::Deref for FILTER_R {
    type Target = crate::FieldReader<u8, FILTER_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FILTER` writer - Proportional Integral Filter Selection"]
pub struct FILTER_W<'a> {
    w: &'a mut W,
}
impl<'a> FILTER_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FILTER_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Default filter mode"]
    #[inline(always)]
    pub fn default(self) -> &'a mut W {
        self.variant(FILTER_A::DEFAULT)
    }
    #[doc = "Low bandwidth filter"]
    #[inline(always)]
    pub fn lbfilt(self) -> &'a mut W {
        self.variant(FILTER_A::LBFILT)
    }
    #[doc = "High bandwidth filter"]
    #[inline(always)]
    pub fn hbfilt(self) -> &'a mut W {
        self.variant(FILTER_A::HBFILT)
    }
    #[doc = "High damping filter"]
    #[inline(always)]
    pub fn hdfilt(self) -> &'a mut W {
        self.variant(FILTER_A::HDFILT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "Field `LPEN` reader - Low-Power Enable"]
pub struct LPEN_R(crate::FieldReader<bool, bool>);
impl LPEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LPEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LPEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LPEN` writer - Low-Power Enable"]
pub struct LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> LPEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `WUF` reader - Wake Up Fast"]
pub struct WUF_R(crate::FieldReader<bool, bool>);
impl WUF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WUF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WUF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WUF` writer - Wake Up Fast"]
pub struct WUF_W<'a> {
    w: &'a mut W,
}
impl<'a> WUF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Reference Clock Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum REFCLK_A {
    #[doc = "0: CLK_DPLL_REF0 clock reference"]
    REF0 = 0,
    #[doc = "1: CLK_DPLL_REF1 clock reference"]
    REF1 = 1,
    #[doc = "2: GCLK_DPLL clock reference"]
    GCLK = 2,
}
impl From<REFCLK_A> for u8 {
    #[inline(always)]
    fn from(variant: REFCLK_A) -> Self {
        variant as _
    }
}
#[doc = "Field `REFCLK` reader - Reference Clock Selection"]
pub struct REFCLK_R(crate::FieldReader<u8, REFCLK_A>);
impl REFCLK_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        REFCLK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<REFCLK_A> {
        match self.bits {
            0 => Some(REFCLK_A::REF0),
            1 => Some(REFCLK_A::REF1),
            2 => Some(REFCLK_A::GCLK),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `REF0`"]
    #[inline(always)]
    pub fn is_ref0(&self) -> bool {
        **self == REFCLK_A::REF0
    }
    #[doc = "Checks if the value of the field is `REF1`"]
    #[inline(always)]
    pub fn is_ref1(&self) -> bool {
        **self == REFCLK_A::REF1
    }
    #[doc = "Checks if the value of the field is `GCLK`"]
    #[inline(always)]
    pub fn is_gclk(&self) -> bool {
        **self == REFCLK_A::GCLK
    }
}
impl core::ops::Deref for REFCLK_R {
    type Target = crate::FieldReader<u8, REFCLK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REFCLK` writer - Reference Clock Selection"]
pub struct REFCLK_W<'a> {
    w: &'a mut W,
}
impl<'a> REFCLK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REFCLK_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "CLK_DPLL_REF0 clock reference"]
    #[inline(always)]
    pub fn ref0(self) -> &'a mut W {
        self.variant(REFCLK_A::REF0)
    }
    #[doc = "CLK_DPLL_REF1 clock reference"]
    #[inline(always)]
    pub fn ref1(self) -> &'a mut W {
        self.variant(REFCLK_A::REF1)
    }
    #[doc = "GCLK_DPLL clock reference"]
    #[inline(always)]
    pub fn gclk(self) -> &'a mut W {
        self.variant(REFCLK_A::GCLK)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
#[doc = "Lock Time\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LTIME_A {
    #[doc = "0: No time-out"]
    DEFAULT = 0,
    #[doc = "4: Time-out if no lock within 8 ms"]
    _8MS = 4,
    #[doc = "5: Time-out if no lock within 9 ms"]
    _9MS = 5,
    #[doc = "6: Time-out if no lock within 10 ms"]
    _10MS = 6,
    #[doc = "7: Time-out if no lock within 11 ms"]
    _11MS = 7,
}
impl From<LTIME_A> for u8 {
    #[inline(always)]
    fn from(variant: LTIME_A) -> Self {
        variant as _
    }
}
#[doc = "Field `LTIME` reader - Lock Time"]
pub struct LTIME_R(crate::FieldReader<u8, LTIME_A>);
impl LTIME_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        LTIME_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<LTIME_A> {
        match self.bits {
            0 => Some(LTIME_A::DEFAULT),
            4 => Some(LTIME_A::_8MS),
            5 => Some(LTIME_A::_9MS),
            6 => Some(LTIME_A::_10MS),
            7 => Some(LTIME_A::_11MS),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DEFAULT`"]
    #[inline(always)]
    pub fn is_default(&self) -> bool {
        **self == LTIME_A::DEFAULT
    }
    #[doc = "Checks if the value of the field is `_8MS`"]
    #[inline(always)]
    pub fn is_8ms(&self) -> bool {
        **self == LTIME_A::_8MS
    }
    #[doc = "Checks if the value of the field is `_9MS`"]
    #[inline(always)]
    pub fn is_9ms(&self) -> bool {
        **self == LTIME_A::_9MS
    }
    #[doc = "Checks if the value of the field is `_10MS`"]
    #[inline(always)]
    pub fn is_10ms(&self) -> bool {
        **self == LTIME_A::_10MS
    }
    #[doc = "Checks if the value of the field is `_11MS`"]
    #[inline(always)]
    pub fn is_11ms(&self) -> bool {
        **self == LTIME_A::_11MS
    }
}
impl core::ops::Deref for LTIME_R {
    type Target = crate::FieldReader<u8, LTIME_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LTIME` writer - Lock Time"]
pub struct LTIME_W<'a> {
    w: &'a mut W,
}
impl<'a> LTIME_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LTIME_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "No time-out"]
    #[inline(always)]
    pub fn default(self) -> &'a mut W {
        self.variant(LTIME_A::DEFAULT)
    }
    #[doc = "Time-out if no lock within 8 ms"]
    #[inline(always)]
    pub fn _8ms(self) -> &'a mut W {
        self.variant(LTIME_A::_8MS)
    }
    #[doc = "Time-out if no lock within 9 ms"]
    #[inline(always)]
    pub fn _9ms(self) -> &'a mut W {
        self.variant(LTIME_A::_9MS)
    }
    #[doc = "Time-out if no lock within 10 ms"]
    #[inline(always)]
    pub fn _10ms(self) -> &'a mut W {
        self.variant(LTIME_A::_10MS)
    }
    #[doc = "Time-out if no lock within 11 ms"]
    #[inline(always)]
    pub fn _11ms(self) -> &'a mut W {
        self.variant(LTIME_A::_11MS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | ((value as u32 & 0x07) << 8);
        self.w
    }
}
#[doc = "Field `LBYPASS` reader - Lock Bypass"]
pub struct LBYPASS_R(crate::FieldReader<bool, bool>);
impl LBYPASS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LBYPASS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LBYPASS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LBYPASS` writer - Lock Bypass"]
pub struct LBYPASS_W<'a> {
    w: &'a mut W,
}
impl<'a> LBYPASS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "Field `DIV` reader - Clock Divider"]
pub struct DIV_R(crate::FieldReader<u16, u16>);
impl DIV_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        DIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIV_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIV` writer - Clock Divider"]
pub struct DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> DIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07ff << 16)) | ((value as u32 & 0x07ff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Proportional Integral Filter Selection"]
    #[inline(always)]
    pub fn filter(&self) -> FILTER_R {
        FILTER_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 2 - Low-Power Enable"]
    #[inline(always)]
    pub fn lpen(&self) -> LPEN_R {
        LPEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Wake Up Fast"]
    #[inline(always)]
    pub fn wuf(&self) -> WUF_R {
        WUF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 4:5 - Reference Clock Selection"]
    #[inline(always)]
    pub fn refclk(&self) -> REFCLK_R {
        REFCLK_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 8:10 - Lock Time"]
    #[inline(always)]
    pub fn ltime(&self) -> LTIME_R {
        LTIME_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bit 12 - Lock Bypass"]
    #[inline(always)]
    pub fn lbypass(&self) -> LBYPASS_R {
        LBYPASS_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 16:26 - Clock Divider"]
    #[inline(always)]
    pub fn div(&self) -> DIV_R {
        DIV_R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:1 - Proportional Integral Filter Selection"]
    #[inline(always)]
    pub fn filter(&mut self) -> FILTER_W {
        FILTER_W { w: self }
    }
    #[doc = "Bit 2 - Low-Power Enable"]
    #[inline(always)]
    pub fn lpen(&mut self) -> LPEN_W {
        LPEN_W { w: self }
    }
    #[doc = "Bit 3 - Wake Up Fast"]
    #[inline(always)]
    pub fn wuf(&mut self) -> WUF_W {
        WUF_W { w: self }
    }
    #[doc = "Bits 4:5 - Reference Clock Selection"]
    #[inline(always)]
    pub fn refclk(&mut self) -> REFCLK_W {
        REFCLK_W { w: self }
    }
    #[doc = "Bits 8:10 - Lock Time"]
    #[inline(always)]
    pub fn ltime(&mut self) -> LTIME_W {
        LTIME_W { w: self }
    }
    #[doc = "Bit 12 - Lock Bypass"]
    #[inline(always)]
    pub fn lbypass(&mut self) -> LBYPASS_W {
        LBYPASS_W { w: self }
    }
    #[doc = "Bits 16:26 - Clock Divider"]
    #[inline(always)]
    pub fn div(&mut self) -> DIV_W {
        DIV_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DPLL Control B\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dpllctrlb](index.html) module"]
pub struct DPLLCTRLB_SPEC;
impl crate::RegisterSpec for DPLLCTRLB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dpllctrlb::R](R) reader structure"]
impl crate::Readable for DPLLCTRLB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dpllctrlb::W](W) writer structure"]
impl crate::Writable for DPLLCTRLB_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DPLLCTRLB to value 0"]
impl crate::Resettable for DPLLCTRLB_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
