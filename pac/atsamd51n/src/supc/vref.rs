#[doc = "Register `VREF` reader"]
pub struct R(crate::R<VREF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VREF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VREF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VREF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `VREF` writer"]
pub struct W(crate::W<VREF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<VREF_SPEC>;
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
impl From<crate::W<VREF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<VREF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TSEN` reader - Temperature Sensor Output Enable"]
pub struct TSEN_R(crate::FieldReader<bool, bool>);
impl TSEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TSEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSEN` writer - Temperature Sensor Output Enable"]
pub struct TSEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TSEN_W<'a> {
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
#[doc = "Field `VREFOE` reader - Voltage Reference Output Enable"]
pub struct VREFOE_R(crate::FieldReader<bool, bool>);
impl VREFOE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        VREFOE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VREFOE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VREFOE` writer - Voltage Reference Output Enable"]
pub struct VREFOE_W<'a> {
    w: &'a mut W,
}
impl<'a> VREFOE_W<'a> {
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
#[doc = "Field `TSSEL` reader - Temperature Sensor Selection"]
pub struct TSSEL_R(crate::FieldReader<bool, bool>);
impl TSSEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TSSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSSEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSSEL` writer - Temperature Sensor Selection"]
pub struct TSSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> TSSEL_W<'a> {
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
#[doc = "Field `RUNSTDBY` reader - Run during Standby"]
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
#[doc = "Field `RUNSTDBY` writer - Run during Standby"]
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
#[doc = "Field `ONDEMAND` reader - On Demand Contrl"]
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
#[doc = "Field `ONDEMAND` writer - On Demand Contrl"]
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
#[doc = "Voltage Reference Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SEL_A {
    #[doc = "0: 1.0V voltage reference typical value"]
    _1V0 = 0,
    #[doc = "1: 1.1V voltage reference typical value"]
    _1V1 = 1,
    #[doc = "2: 1.2V voltage reference typical value"]
    _1V2 = 2,
    #[doc = "3: 1.25V voltage reference typical value"]
    _1V25 = 3,
    #[doc = "4: 2.0V voltage reference typical value"]
    _2V0 = 4,
    #[doc = "5: 2.2V voltage reference typical value"]
    _2V2 = 5,
    #[doc = "6: 2.4V voltage reference typical value"]
    _2V4 = 6,
    #[doc = "7: 2.5V voltage reference typical value"]
    _2V5 = 7,
}
impl From<SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SEL` reader - Voltage Reference Selection"]
pub struct SEL_R(crate::FieldReader<u8, SEL_A>);
impl SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SEL_A> {
        match self.bits {
            0 => Some(SEL_A::_1V0),
            1 => Some(SEL_A::_1V1),
            2 => Some(SEL_A::_1V2),
            3 => Some(SEL_A::_1V25),
            4 => Some(SEL_A::_2V0),
            5 => Some(SEL_A::_2V2),
            6 => Some(SEL_A::_2V4),
            7 => Some(SEL_A::_2V5),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_1V0`"]
    #[inline(always)]
    pub fn is_1v0(&self) -> bool {
        **self == SEL_A::_1V0
    }
    #[doc = "Checks if the value of the field is `_1V1`"]
    #[inline(always)]
    pub fn is_1v1(&self) -> bool {
        **self == SEL_A::_1V1
    }
    #[doc = "Checks if the value of the field is `_1V2`"]
    #[inline(always)]
    pub fn is_1v2(&self) -> bool {
        **self == SEL_A::_1V2
    }
    #[doc = "Checks if the value of the field is `_1V25`"]
    #[inline(always)]
    pub fn is_1v25(&self) -> bool {
        **self == SEL_A::_1V25
    }
    #[doc = "Checks if the value of the field is `_2V0`"]
    #[inline(always)]
    pub fn is_2v0(&self) -> bool {
        **self == SEL_A::_2V0
    }
    #[doc = "Checks if the value of the field is `_2V2`"]
    #[inline(always)]
    pub fn is_2v2(&self) -> bool {
        **self == SEL_A::_2V2
    }
    #[doc = "Checks if the value of the field is `_2V4`"]
    #[inline(always)]
    pub fn is_2v4(&self) -> bool {
        **self == SEL_A::_2V4
    }
    #[doc = "Checks if the value of the field is `_2V5`"]
    #[inline(always)]
    pub fn is_2v5(&self) -> bool {
        **self == SEL_A::_2V5
    }
}
impl core::ops::Deref for SEL_R {
    type Target = crate::FieldReader<u8, SEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SEL` writer - Voltage Reference Selection"]
pub struct SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "1.0V voltage reference typical value"]
    #[inline(always)]
    pub fn _1v0(self) -> &'a mut W {
        self.variant(SEL_A::_1V0)
    }
    #[doc = "1.1V voltage reference typical value"]
    #[inline(always)]
    pub fn _1v1(self) -> &'a mut W {
        self.variant(SEL_A::_1V1)
    }
    #[doc = "1.2V voltage reference typical value"]
    #[inline(always)]
    pub fn _1v2(self) -> &'a mut W {
        self.variant(SEL_A::_1V2)
    }
    #[doc = "1.25V voltage reference typical value"]
    #[inline(always)]
    pub fn _1v25(self) -> &'a mut W {
        self.variant(SEL_A::_1V25)
    }
    #[doc = "2.0V voltage reference typical value"]
    #[inline(always)]
    pub fn _2v0(self) -> &'a mut W {
        self.variant(SEL_A::_2V0)
    }
    #[doc = "2.2V voltage reference typical value"]
    #[inline(always)]
    pub fn _2v2(self) -> &'a mut W {
        self.variant(SEL_A::_2V2)
    }
    #[doc = "2.4V voltage reference typical value"]
    #[inline(always)]
    pub fn _2v4(self) -> &'a mut W {
        self.variant(SEL_A::_2V4)
    }
    #[doc = "2.5V voltage reference typical value"]
    #[inline(always)]
    pub fn _2v5(self) -> &'a mut W {
        self.variant(SEL_A::_2V5)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | ((value as u32 & 0x0f) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bit 1 - Temperature Sensor Output Enable"]
    #[inline(always)]
    pub fn tsen(&self) -> TSEN_R {
        TSEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Voltage Reference Output Enable"]
    #[inline(always)]
    pub fn vrefoe(&self) -> VREFOE_R {
        VREFOE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Temperature Sensor Selection"]
    #[inline(always)]
    pub fn tssel(&self) -> TSSEL_R {
        TSSEL_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Run during Standby"]
    #[inline(always)]
    pub fn runstdby(&self) -> RUNSTDBY_R {
        RUNSTDBY_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - On Demand Contrl"]
    #[inline(always)]
    pub fn ondemand(&self) -> ONDEMAND_R {
        ONDEMAND_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 16:19 - Voltage Reference Selection"]
    #[inline(always)]
    pub fn sel(&self) -> SEL_R {
        SEL_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 1 - Temperature Sensor Output Enable"]
    #[inline(always)]
    pub fn tsen(&mut self) -> TSEN_W {
        TSEN_W { w: self }
    }
    #[doc = "Bit 2 - Voltage Reference Output Enable"]
    #[inline(always)]
    pub fn vrefoe(&mut self) -> VREFOE_W {
        VREFOE_W { w: self }
    }
    #[doc = "Bit 3 - Temperature Sensor Selection"]
    #[inline(always)]
    pub fn tssel(&mut self) -> TSSEL_W {
        TSSEL_W { w: self }
    }
    #[doc = "Bit 6 - Run during Standby"]
    #[inline(always)]
    pub fn runstdby(&mut self) -> RUNSTDBY_W {
        RUNSTDBY_W { w: self }
    }
    #[doc = "Bit 7 - On Demand Contrl"]
    #[inline(always)]
    pub fn ondemand(&mut self) -> ONDEMAND_W {
        ONDEMAND_W { w: self }
    }
    #[doc = "Bits 16:19 - Voltage Reference Selection"]
    #[inline(always)]
    pub fn sel(&mut self) -> SEL_W {
        SEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "VREF Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vref](index.html) module"]
pub struct VREF_SPEC;
impl crate::RegisterSpec for VREF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [vref::R](R) reader structure"]
impl crate::Readable for VREF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [vref::W](W) writer structure"]
impl crate::Writable for VREF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets VREF to value 0"]
impl crate::Resettable for VREF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
