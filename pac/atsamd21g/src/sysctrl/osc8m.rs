#[doc = "Register `OSC8M` reader"]
pub struct R(crate::R<OSC8M_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OSC8M_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OSC8M_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OSC8M_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OSC8M` writer"]
pub struct W(crate::W<OSC8M_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OSC8M_SPEC>;
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
impl From<crate::W<OSC8M_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OSC8M_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENABLE` reader - Oscillator Enable"]
pub struct ENABLE_R(crate::FieldReader<bool, bool>);
impl ENABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ENABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENABLE` writer - Oscillator Enable"]
pub struct ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE_W<'a> {
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
#[doc = "Field `RUNSTDBY` reader - Run in Standby"]
pub struct RUNSTDBY_R(crate::FieldReader<bool, bool>);
impl RUNSTDBY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RUNSTDBY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RUNSTDBY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RUNSTDBY` writer - Run in Standby"]
pub struct RUNSTDBY_W<'a> {
    w: &'a mut W,
}
impl<'a> RUNSTDBY_W<'a> {
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
#[doc = "Field `ONDEMAND` reader - On Demand Control"]
pub struct ONDEMAND_R(crate::FieldReader<bool, bool>);
impl ONDEMAND_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ONDEMAND_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ONDEMAND_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ONDEMAND` writer - On Demand Control"]
pub struct ONDEMAND_W<'a> {
    w: &'a mut W,
}
impl<'a> ONDEMAND_W<'a> {
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
#[doc = "Oscillator Prescaler\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PRESC_A {
    #[doc = "0: 1"]
    _0 = 0,
    #[doc = "1: 2"]
    _1 = 1,
    #[doc = "2: 4"]
    _2 = 2,
    #[doc = "3: 8"]
    _3 = 3,
}
impl From<PRESC_A> for u8 {
    #[inline(always)]
    fn from(variant: PRESC_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PRESC` reader - Oscillator Prescaler"]
pub struct PRESC_R(crate::FieldReader<u8, PRESC_A>);
impl PRESC_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PRESC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PRESC_A {
        match self.bits {
            0 => PRESC_A::_0,
            1 => PRESC_A::_1,
            2 => PRESC_A::_2,
            3 => PRESC_A::_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PRESC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PRESC_A::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        **self == PRESC_A::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline(always)]
    pub fn is_3(&self) -> bool {
        **self == PRESC_A::_3
    }
}
impl core::ops::Deref for PRESC_R {
    type Target = crate::FieldReader<u8, PRESC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRESC` writer - Oscillator Prescaler"]
pub struct PRESC_W<'a> {
    w: &'a mut W,
}
impl<'a> PRESC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PRESC_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "1"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PRESC_A::_0)
    }
    #[doc = "2"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PRESC_A::_1)
    }
    #[doc = "4"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut W {
        self.variant(PRESC_A::_2)
    }
    #[doc = "8"]
    #[inline(always)]
    pub fn _3(self) -> &'a mut W {
        self.variant(PRESC_A::_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | ((value as u32 & 0x03) << 8);
        self.w
    }
}
#[doc = "Field `CALIB` reader - Oscillator Calibration"]
pub struct CALIB_R(crate::FieldReader<u16, u16>);
impl CALIB_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        CALIB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CALIB_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CALIB` writer - Oscillator Calibration"]
pub struct CALIB_W<'a> {
    w: &'a mut W,
}
impl<'a> CALIB_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff << 16)) | ((value as u32 & 0x0fff) << 16);
        self.w
    }
}
#[doc = "Oscillator Frequency Range\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FRANGE_A {
    #[doc = "0: 4 to 6MHz"]
    _0 = 0,
    #[doc = "1: 6 to 8MHz"]
    _1 = 1,
    #[doc = "2: 8 to 11MHz"]
    _2 = 2,
    #[doc = "3: 11 to 15MHz"]
    _3 = 3,
}
impl From<FRANGE_A> for u8 {
    #[inline(always)]
    fn from(variant: FRANGE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `FRANGE` reader - Oscillator Frequency Range"]
pub struct FRANGE_R(crate::FieldReader<u8, FRANGE_A>);
impl FRANGE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        FRANGE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FRANGE_A {
        match self.bits {
            0 => FRANGE_A::_0,
            1 => FRANGE_A::_1,
            2 => FRANGE_A::_2,
            3 => FRANGE_A::_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == FRANGE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == FRANGE_A::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        **self == FRANGE_A::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline(always)]
    pub fn is_3(&self) -> bool {
        **self == FRANGE_A::_3
    }
}
impl core::ops::Deref for FRANGE_R {
    type Target = crate::FieldReader<u8, FRANGE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FRANGE` writer - Oscillator Frequency Range"]
pub struct FRANGE_W<'a> {
    w: &'a mut W,
}
impl<'a> FRANGE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FRANGE_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "4 to 6MHz"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FRANGE_A::_0)
    }
    #[doc = "6 to 8MHz"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FRANGE_A::_1)
    }
    #[doc = "8 to 11MHz"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut W {
        self.variant(FRANGE_A::_2)
    }
    #[doc = "11 to 15MHz"]
    #[inline(always)]
    pub fn _3(self) -> &'a mut W {
        self.variant(FRANGE_A::_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | ((value as u32 & 0x03) << 30);
        self.w
    }
}
impl R {
    #[doc = "Bit 1 - Oscillator Enable"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Run in Standby"]
    #[inline(always)]
    pub fn runstdby(&self) -> RUNSTDBY_R {
        RUNSTDBY_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - On Demand Control"]
    #[inline(always)]
    pub fn ondemand(&self) -> ONDEMAND_R {
        ONDEMAND_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:9 - Oscillator Prescaler"]
    #[inline(always)]
    pub fn presc(&self) -> PRESC_R {
        PRESC_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 16:27 - Oscillator Calibration"]
    #[inline(always)]
    pub fn calib(&self) -> CALIB_R {
        CALIB_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    #[doc = "Bits 30:31 - Oscillator Frequency Range"]
    #[inline(always)]
    pub fn frange(&self) -> FRANGE_R {
        FRANGE_R::new(((self.bits >> 30) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 1 - Oscillator Enable"]
    #[inline(always)]
    pub fn enable(&mut self) -> ENABLE_W {
        ENABLE_W { w: self }
    }
    #[doc = "Bit 6 - Run in Standby"]
    #[inline(always)]
    pub fn runstdby(&mut self) -> RUNSTDBY_W {
        RUNSTDBY_W { w: self }
    }
    #[doc = "Bit 7 - On Demand Control"]
    #[inline(always)]
    pub fn ondemand(&mut self) -> ONDEMAND_W {
        ONDEMAND_W { w: self }
    }
    #[doc = "Bits 8:9 - Oscillator Prescaler"]
    #[inline(always)]
    pub fn presc(&mut self) -> PRESC_W {
        PRESC_W { w: self }
    }
    #[doc = "Bits 16:27 - Oscillator Calibration"]
    #[inline(always)]
    pub fn calib(&mut self) -> CALIB_W {
        CALIB_W { w: self }
    }
    #[doc = "Bits 30:31 - Oscillator Frequency Range"]
    #[inline(always)]
    pub fn frange(&mut self) -> FRANGE_W {
        FRANGE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "8MHz Internal Oscillator (OSC8M) Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [osc8m](index.html) module"]
pub struct OSC8M_SPEC;
impl crate::RegisterSpec for OSC8M_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [osc8m::R](R) reader structure"]
impl crate::Readable for OSC8M_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [osc8m::W](W) writer structure"]
impl crate::Writable for OSC8M_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OSC8M to value 0x8707_0382"]
impl crate::Resettable for OSC8M_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x8707_0382
    }
}
