#[doc = "Register `SCFG[%s]` reader"]
pub struct R(crate::R<SCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCFG[%s]` writer"]
pub struct W(crate::W<SCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCFG_SPEC>;
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
impl From<crate::W<SCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SLOT_CYCLE` reader - Maximum Number of Allowed Cycles for a Burst"]
pub struct SLOT_CYCLE_R(crate::FieldReader<u8, u8>);
impl SLOT_CYCLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SLOT_CYCLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLOT_CYCLE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLOT_CYCLE` writer - Maximum Number of Allowed Cycles for a Burst"]
pub struct SLOT_CYCLE_W<'a> {
    w: &'a mut W,
}
impl<'a> SLOT_CYCLE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Default Master Type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DEFMSTR_TYPE_A {
    #[doc = "0: No Default Master. At the end of current slave access, if no other master request is pending, the slave is deconnected from all masters. This resusts in having a one cycle latency for the first transfer of a burst."]
    NO_DEFAULT = 0,
    #[doc = "1: Last Default Master At the end of current slave access, if no other master request is pending, the slave stay connected with the last master havingaccessed it.This resusts in not having the one cycle latency when the last master re-trying access on the slave."]
    LAST_DEFAULT = 1,
    #[doc = "2: Fixed Default Master At the end of current slave access, if no other master request is pending, the slave connects with fixed master which numberis in FIXED_DEFMSTR register.This resusts in not having the one cycle latency when the fixed master re-trying access on the slave."]
    FIXED_DEFAULT = 2,
}
impl From<DEFMSTR_TYPE_A> for u8 {
    #[inline(always)]
    fn from(variant: DEFMSTR_TYPE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `DEFMSTR_TYPE` reader - Default Master Type"]
pub struct DEFMSTR_TYPE_R(crate::FieldReader<u8, DEFMSTR_TYPE_A>);
impl DEFMSTR_TYPE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DEFMSTR_TYPE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DEFMSTR_TYPE_A> {
        match self.bits {
            0 => Some(DEFMSTR_TYPE_A::NO_DEFAULT),
            1 => Some(DEFMSTR_TYPE_A::LAST_DEFAULT),
            2 => Some(DEFMSTR_TYPE_A::FIXED_DEFAULT),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NO_DEFAULT`"]
    #[inline(always)]
    pub fn is_no_default(&self) -> bool {
        **self == DEFMSTR_TYPE_A::NO_DEFAULT
    }
    #[doc = "Checks if the value of the field is `LAST_DEFAULT`"]
    #[inline(always)]
    pub fn is_last_default(&self) -> bool {
        **self == DEFMSTR_TYPE_A::LAST_DEFAULT
    }
    #[doc = "Checks if the value of the field is `FIXED_DEFAULT`"]
    #[inline(always)]
    pub fn is_fixed_default(&self) -> bool {
        **self == DEFMSTR_TYPE_A::FIXED_DEFAULT
    }
}
impl core::ops::Deref for DEFMSTR_TYPE_R {
    type Target = crate::FieldReader<u8, DEFMSTR_TYPE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DEFMSTR_TYPE` writer - Default Master Type"]
pub struct DEFMSTR_TYPE_W<'a> {
    w: &'a mut W,
}
impl<'a> DEFMSTR_TYPE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DEFMSTR_TYPE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "No Default Master. At the end of current slave access, if no other master request is pending, the slave is deconnected from all masters. This resusts in having a one cycle latency for the first transfer of a burst."]
    #[inline(always)]
    pub fn no_default(self) -> &'a mut W {
        self.variant(DEFMSTR_TYPE_A::NO_DEFAULT)
    }
    #[doc = "Last Default Master At the end of current slave access, if no other master request is pending, the slave stay connected with the last master havingaccessed it.This resusts in not having the one cycle latency when the last master re-trying access on the slave."]
    #[inline(always)]
    pub fn last_default(self) -> &'a mut W {
        self.variant(DEFMSTR_TYPE_A::LAST_DEFAULT)
    }
    #[doc = "Fixed Default Master At the end of current slave access, if no other master request is pending, the slave connects with fixed master which numberis in FIXED_DEFMSTR register.This resusts in not having the one cycle latency when the fixed master re-trying access on the slave."]
    #[inline(always)]
    pub fn fixed_default(self) -> &'a mut W {
        self.variant(DEFMSTR_TYPE_A::FIXED_DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | ((value as u32 & 0x03) << 16);
        self.w
    }
}
#[doc = "Field `FIXED_DEFMSTR` reader - Fixed Index of Default Master"]
pub struct FIXED_DEFMSTR_R(crate::FieldReader<u8, u8>);
impl FIXED_DEFMSTR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        FIXED_DEFMSTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FIXED_DEFMSTR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FIXED_DEFMSTR` writer - Fixed Index of Default Master"]
pub struct FIXED_DEFMSTR_W<'a> {
    w: &'a mut W,
}
impl<'a> FIXED_DEFMSTR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 18)) | ((value as u32 & 0x0f) << 18);
        self.w
    }
}
#[doc = "Arbitration Type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ARBT_A {
    #[doc = "0: Round-Robin Arbitration"]
    ROUND_ROBIN = 0,
    #[doc = "1: Fixed Priority Arbitration"]
    FIXED_PRIORITY = 1,
}
impl From<ARBT_A> for bool {
    #[inline(always)]
    fn from(variant: ARBT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ARBT` reader - Arbitration Type"]
pub struct ARBT_R(crate::FieldReader<bool, ARBT_A>);
impl ARBT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ARBT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ARBT_A {
        match self.bits {
            false => ARBT_A::ROUND_ROBIN,
            true => ARBT_A::FIXED_PRIORITY,
        }
    }
    #[doc = "Checks if the value of the field is `ROUND_ROBIN`"]
    #[inline(always)]
    pub fn is_round_robin(&self) -> bool {
        **self == ARBT_A::ROUND_ROBIN
    }
    #[doc = "Checks if the value of the field is `FIXED_PRIORITY`"]
    #[inline(always)]
    pub fn is_fixed_priority(&self) -> bool {
        **self == ARBT_A::FIXED_PRIORITY
    }
}
impl core::ops::Deref for ARBT_R {
    type Target = crate::FieldReader<bool, ARBT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ARBT` writer - Arbitration Type"]
pub struct ARBT_W<'a> {
    w: &'a mut W,
}
impl<'a> ARBT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ARBT_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Round-Robin Arbitration"]
    #[inline(always)]
    pub fn round_robin(self) -> &'a mut W {
        self.variant(ARBT_A::ROUND_ROBIN)
    }
    #[doc = "Fixed Priority Arbitration"]
    #[inline(always)]
    pub fn fixed_priority(self) -> &'a mut W {
        self.variant(ARBT_A::FIXED_PRIORITY)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Maximum Number of Allowed Cycles for a Burst"]
    #[inline(always)]
    pub fn slot_cycle(&self) -> SLOT_CYCLE_R {
        SLOT_CYCLE_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:17 - Default Master Type"]
    #[inline(always)]
    pub fn defmstr_type(&self) -> DEFMSTR_TYPE_R {
        DEFMSTR_TYPE_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 18:21 - Fixed Index of Default Master"]
    #[inline(always)]
    pub fn fixed_defmstr(&self) -> FIXED_DEFMSTR_R {
        FIXED_DEFMSTR_R::new(((self.bits >> 18) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - Arbitration Type"]
    #[inline(always)]
    pub fn arbt(&self) -> ARBT_R {
        ARBT_R::new(((self.bits >> 24) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Maximum Number of Allowed Cycles for a Burst"]
    #[inline(always)]
    pub fn slot_cycle(&mut self) -> SLOT_CYCLE_W {
        SLOT_CYCLE_W { w: self }
    }
    #[doc = "Bits 16:17 - Default Master Type"]
    #[inline(always)]
    pub fn defmstr_type(&mut self) -> DEFMSTR_TYPE_W {
        DEFMSTR_TYPE_W { w: self }
    }
    #[doc = "Bits 18:21 - Fixed Index of Default Master"]
    #[inline(always)]
    pub fn fixed_defmstr(&mut self) -> FIXED_DEFMSTR_W {
        FIXED_DEFMSTR_W { w: self }
    }
    #[doc = "Bit 24 - Arbitration Type"]
    #[inline(always)]
    pub fn arbt(&mut self) -> ARBT_W {
        ARBT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Slave Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scfg](index.html) module"]
pub struct SCFG_SPEC;
impl crate::RegisterSpec for SCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [scfg::R](R) reader structure"]
impl crate::Readable for SCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [scfg::W](W) writer structure"]
impl crate::Writable for SCFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SCFG[%s]
to value 0x10"]
impl crate::Resettable for SCFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x10
    }
}
