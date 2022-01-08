#[doc = "Register `ICSR` reader"]
pub struct R(crate::R<ICSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ICSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ICSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ICSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ICSR` writer"]
pub struct W(crate::W<ICSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ICSR_SPEC>;
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
impl From<crate::W<ICSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ICSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VECTACTIVE` reader - Active exception number"]
pub struct VECTACTIVE_R(crate::FieldReader<u16, u16>);
impl VECTACTIVE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        VECTACTIVE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VECTACTIVE_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VECTACTIVE` writer - Active exception number"]
pub struct VECTACTIVE_W<'a> {
    w: &'a mut W,
}
impl<'a> VECTACTIVE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01ff) | (value as u32 & 0x01ff);
        self.w
    }
}
#[doc = "Field `RETTOBASE` reader - No preempted active exceptions to execute"]
pub struct RETTOBASE_R(crate::FieldReader<bool, bool>);
impl RETTOBASE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RETTOBASE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RETTOBASE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RETTOBASE` writer - No preempted active exceptions to execute"]
pub struct RETTOBASE_W<'a> {
    w: &'a mut W,
}
impl<'a> RETTOBASE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "Field `VECTPENDING` reader - Exception number of the highest priority pending enabled exception"]
pub struct VECTPENDING_R(crate::FieldReader<u8, u8>);
impl VECTPENDING_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        VECTPENDING_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VECTPENDING_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VECTPENDING` writer - Exception number of the highest priority pending enabled exception"]
pub struct VECTPENDING_W<'a> {
    w: &'a mut W,
}
impl<'a> VECTPENDING_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 12)) | ((value as u32 & 0x3f) << 12);
        self.w
    }
}
#[doc = "Field `ISRPENDING` reader - Interrupt pending flag"]
pub struct ISRPENDING_R(crate::FieldReader<bool, bool>);
impl ISRPENDING_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ISRPENDING_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ISRPENDING_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ISRPENDING` writer - Interrupt pending flag"]
pub struct ISRPENDING_W<'a> {
    w: &'a mut W,
}
impl<'a> ISRPENDING_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | ((value as u32 & 0x01) << 22);
        self.w
    }
}
#[doc = "Field `ISRPREEMPT` reader - Debug only"]
pub struct ISRPREEMPT_R(crate::FieldReader<bool, bool>);
impl ISRPREEMPT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ISRPREEMPT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ISRPREEMPT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ISRPREEMPT` writer - Debug only"]
pub struct ISRPREEMPT_W<'a> {
    w: &'a mut W,
}
impl<'a> ISRPREEMPT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | ((value as u32 & 0x01) << 23);
        self.w
    }
}
#[doc = "SysTick clear-pending bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PENDSTCLR_A {
    #[doc = "0: No effect"]
    VALUE_0 = 0,
    #[doc = "1: Removes the pending state from the SysTick exception"]
    VALUE_1 = 1,
}
impl From<PENDSTCLR_A> for bool {
    #[inline(always)]
    fn from(variant: PENDSTCLR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PENDSTCLR` reader - SysTick clear-pending bit"]
pub struct PENDSTCLR_R(crate::FieldReader<bool, PENDSTCLR_A>);
impl PENDSTCLR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PENDSTCLR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PENDSTCLR_A {
        match self.bits {
            false => PENDSTCLR_A::VALUE_0,
            true => PENDSTCLR_A::VALUE_1,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE_0`"]
    #[inline(always)]
    pub fn is_value_0(&self) -> bool {
        **self == PENDSTCLR_A::VALUE_0
    }
    #[doc = "Checks if the value of the field is `VALUE_1`"]
    #[inline(always)]
    pub fn is_value_1(&self) -> bool {
        **self == PENDSTCLR_A::VALUE_1
    }
}
impl core::ops::Deref for PENDSTCLR_R {
    type Target = crate::FieldReader<bool, PENDSTCLR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PENDSTCLR` writer - SysTick clear-pending bit"]
pub struct PENDSTCLR_W<'a> {
    w: &'a mut W,
}
impl<'a> PENDSTCLR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PENDSTCLR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value_0(self) -> &'a mut W {
        self.variant(PENDSTCLR_A::VALUE_0)
    }
    #[doc = "Removes the pending state from the SysTick exception"]
    #[inline(always)]
    pub fn value_1(self) -> &'a mut W {
        self.variant(PENDSTCLR_A::VALUE_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | ((value as u32 & 0x01) << 25);
        self.w
    }
}
#[doc = "SysTick set-pending bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PENDSTSET_A {
    #[doc = "0: Write: no effect; read: SysTick exception is not pending"]
    VALUE_0 = 0,
    #[doc = "1: Write: changes SysTick exception state to pending; read: SysTick exception is pending"]
    VALUE_1 = 1,
}
impl From<PENDSTSET_A> for bool {
    #[inline(always)]
    fn from(variant: PENDSTSET_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PENDSTSET` reader - SysTick set-pending bit"]
pub struct PENDSTSET_R(crate::FieldReader<bool, PENDSTSET_A>);
impl PENDSTSET_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PENDSTSET_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PENDSTSET_A {
        match self.bits {
            false => PENDSTSET_A::VALUE_0,
            true => PENDSTSET_A::VALUE_1,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE_0`"]
    #[inline(always)]
    pub fn is_value_0(&self) -> bool {
        **self == PENDSTSET_A::VALUE_0
    }
    #[doc = "Checks if the value of the field is `VALUE_1`"]
    #[inline(always)]
    pub fn is_value_1(&self) -> bool {
        **self == PENDSTSET_A::VALUE_1
    }
}
impl core::ops::Deref for PENDSTSET_R {
    type Target = crate::FieldReader<bool, PENDSTSET_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PENDSTSET` writer - SysTick set-pending bit"]
pub struct PENDSTSET_W<'a> {
    w: &'a mut W,
}
impl<'a> PENDSTSET_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PENDSTSET_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Write: no effect; read: SysTick exception is not pending"]
    #[inline(always)]
    pub fn value_0(self) -> &'a mut W {
        self.variant(PENDSTSET_A::VALUE_0)
    }
    #[doc = "Write: changes SysTick exception state to pending; read: SysTick exception is pending"]
    #[inline(always)]
    pub fn value_1(self) -> &'a mut W {
        self.variant(PENDSTSET_A::VALUE_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | ((value as u32 & 0x01) << 26);
        self.w
    }
}
#[doc = "PendSV clear-pending bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PENDSVCLR_A {
    #[doc = "0: No effect"]
    VALUE_0 = 0,
    #[doc = "1: Removes the pending state from the PendSV exception"]
    VALUE_1 = 1,
}
impl From<PENDSVCLR_A> for bool {
    #[inline(always)]
    fn from(variant: PENDSVCLR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PENDSVCLR` reader - PendSV clear-pending bit"]
pub struct PENDSVCLR_R(crate::FieldReader<bool, PENDSVCLR_A>);
impl PENDSVCLR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PENDSVCLR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PENDSVCLR_A {
        match self.bits {
            false => PENDSVCLR_A::VALUE_0,
            true => PENDSVCLR_A::VALUE_1,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE_0`"]
    #[inline(always)]
    pub fn is_value_0(&self) -> bool {
        **self == PENDSVCLR_A::VALUE_0
    }
    #[doc = "Checks if the value of the field is `VALUE_1`"]
    #[inline(always)]
    pub fn is_value_1(&self) -> bool {
        **self == PENDSVCLR_A::VALUE_1
    }
}
impl core::ops::Deref for PENDSVCLR_R {
    type Target = crate::FieldReader<bool, PENDSVCLR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PENDSVCLR` writer - PendSV clear-pending bit"]
pub struct PENDSVCLR_W<'a> {
    w: &'a mut W,
}
impl<'a> PENDSVCLR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PENDSVCLR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value_0(self) -> &'a mut W {
        self.variant(PENDSVCLR_A::VALUE_0)
    }
    #[doc = "Removes the pending state from the PendSV exception"]
    #[inline(always)]
    pub fn value_1(self) -> &'a mut W {
        self.variant(PENDSVCLR_A::VALUE_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | ((value as u32 & 0x01) << 27);
        self.w
    }
}
#[doc = "PendSV set-pending bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PENDSVSET_A {
    #[doc = "0: Write: no effect; read: PendSV exception is not pending"]
    VALUE_0 = 0,
    #[doc = "1: Write: changes PendSV exception state to pending; read: PendSV exception is pending"]
    VALUE_1 = 1,
}
impl From<PENDSVSET_A> for bool {
    #[inline(always)]
    fn from(variant: PENDSVSET_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PENDSVSET` reader - PendSV set-pending bit"]
pub struct PENDSVSET_R(crate::FieldReader<bool, PENDSVSET_A>);
impl PENDSVSET_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PENDSVSET_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PENDSVSET_A {
        match self.bits {
            false => PENDSVSET_A::VALUE_0,
            true => PENDSVSET_A::VALUE_1,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE_0`"]
    #[inline(always)]
    pub fn is_value_0(&self) -> bool {
        **self == PENDSVSET_A::VALUE_0
    }
    #[doc = "Checks if the value of the field is `VALUE_1`"]
    #[inline(always)]
    pub fn is_value_1(&self) -> bool {
        **self == PENDSVSET_A::VALUE_1
    }
}
impl core::ops::Deref for PENDSVSET_R {
    type Target = crate::FieldReader<bool, PENDSVSET_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PENDSVSET` writer - PendSV set-pending bit"]
pub struct PENDSVSET_W<'a> {
    w: &'a mut W,
}
impl<'a> PENDSVSET_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PENDSVSET_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Write: no effect; read: PendSV exception is not pending"]
    #[inline(always)]
    pub fn value_0(self) -> &'a mut W {
        self.variant(PENDSVSET_A::VALUE_0)
    }
    #[doc = "Write: changes PendSV exception state to pending; read: PendSV exception is pending"]
    #[inline(always)]
    pub fn value_1(self) -> &'a mut W {
        self.variant(PENDSVSET_A::VALUE_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | ((value as u32 & 0x01) << 28);
        self.w
    }
}
#[doc = "NMI set-pending bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NMIPENDSET_A {
    #[doc = "0: Write: no effect; read: NMI exception is not pending"]
    VALUE_0 = 0,
    #[doc = "1: Write: changes NMI exception state to pending; read: NMI exception is pending"]
    VALUE_1 = 1,
}
impl From<NMIPENDSET_A> for bool {
    #[inline(always)]
    fn from(variant: NMIPENDSET_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NMIPENDSET` reader - NMI set-pending bit"]
pub struct NMIPENDSET_R(crate::FieldReader<bool, NMIPENDSET_A>);
impl NMIPENDSET_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        NMIPENDSET_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NMIPENDSET_A {
        match self.bits {
            false => NMIPENDSET_A::VALUE_0,
            true => NMIPENDSET_A::VALUE_1,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE_0`"]
    #[inline(always)]
    pub fn is_value_0(&self) -> bool {
        **self == NMIPENDSET_A::VALUE_0
    }
    #[doc = "Checks if the value of the field is `VALUE_1`"]
    #[inline(always)]
    pub fn is_value_1(&self) -> bool {
        **self == NMIPENDSET_A::VALUE_1
    }
}
impl core::ops::Deref for NMIPENDSET_R {
    type Target = crate::FieldReader<bool, NMIPENDSET_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NMIPENDSET` writer - NMI set-pending bit"]
pub struct NMIPENDSET_W<'a> {
    w: &'a mut W,
}
impl<'a> NMIPENDSET_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: NMIPENDSET_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Write: no effect; read: NMI exception is not pending"]
    #[inline(always)]
    pub fn value_0(self) -> &'a mut W {
        self.variant(NMIPENDSET_A::VALUE_0)
    }
    #[doc = "Write: changes NMI exception state to pending; read: NMI exception is pending"]
    #[inline(always)]
    pub fn value_1(self) -> &'a mut W {
        self.variant(NMIPENDSET_A::VALUE_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:8 - Active exception number"]
    #[inline(always)]
    pub fn vectactive(&self) -> VECTACTIVE_R {
        VECTACTIVE_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bit 11 - No preempted active exceptions to execute"]
    #[inline(always)]
    pub fn rettobase(&self) -> RETTOBASE_R {
        RETTOBASE_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bits 12:17 - Exception number of the highest priority pending enabled exception"]
    #[inline(always)]
    pub fn vectpending(&self) -> VECTPENDING_R {
        VECTPENDING_R::new(((self.bits >> 12) & 0x3f) as u8)
    }
    #[doc = "Bit 22 - Interrupt pending flag"]
    #[inline(always)]
    pub fn isrpending(&self) -> ISRPENDING_R {
        ISRPENDING_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Debug only"]
    #[inline(always)]
    pub fn isrpreempt(&self) -> ISRPREEMPT_R {
        ISRPREEMPT_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 25 - SysTick clear-pending bit"]
    #[inline(always)]
    pub fn pendstclr(&self) -> PENDSTCLR_R {
        PENDSTCLR_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - SysTick set-pending bit"]
    #[inline(always)]
    pub fn pendstset(&self) -> PENDSTSET_R {
        PENDSTSET_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - PendSV clear-pending bit"]
    #[inline(always)]
    pub fn pendsvclr(&self) -> PENDSVCLR_R {
        PENDSVCLR_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - PendSV set-pending bit"]
    #[inline(always)]
    pub fn pendsvset(&self) -> PENDSVSET_R {
        PENDSVSET_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 31 - NMI set-pending bit"]
    #[inline(always)]
    pub fn nmipendset(&self) -> NMIPENDSET_R {
        NMIPENDSET_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:8 - Active exception number"]
    #[inline(always)]
    pub fn vectactive(&mut self) -> VECTACTIVE_W {
        VECTACTIVE_W { w: self }
    }
    #[doc = "Bit 11 - No preempted active exceptions to execute"]
    #[inline(always)]
    pub fn rettobase(&mut self) -> RETTOBASE_W {
        RETTOBASE_W { w: self }
    }
    #[doc = "Bits 12:17 - Exception number of the highest priority pending enabled exception"]
    #[inline(always)]
    pub fn vectpending(&mut self) -> VECTPENDING_W {
        VECTPENDING_W { w: self }
    }
    #[doc = "Bit 22 - Interrupt pending flag"]
    #[inline(always)]
    pub fn isrpending(&mut self) -> ISRPENDING_W {
        ISRPENDING_W { w: self }
    }
    #[doc = "Bit 23 - Debug only"]
    #[inline(always)]
    pub fn isrpreempt(&mut self) -> ISRPREEMPT_W {
        ISRPREEMPT_W { w: self }
    }
    #[doc = "Bit 25 - SysTick clear-pending bit"]
    #[inline(always)]
    pub fn pendstclr(&mut self) -> PENDSTCLR_W {
        PENDSTCLR_W { w: self }
    }
    #[doc = "Bit 26 - SysTick set-pending bit"]
    #[inline(always)]
    pub fn pendstset(&mut self) -> PENDSTSET_W {
        PENDSTSET_W { w: self }
    }
    #[doc = "Bit 27 - PendSV clear-pending bit"]
    #[inline(always)]
    pub fn pendsvclr(&mut self) -> PENDSVCLR_W {
        PENDSVCLR_W { w: self }
    }
    #[doc = "Bit 28 - PendSV set-pending bit"]
    #[inline(always)]
    pub fn pendsvset(&mut self) -> PENDSVSET_W {
        PENDSVSET_W { w: self }
    }
    #[doc = "Bit 31 - NMI set-pending bit"]
    #[inline(always)]
    pub fn nmipendset(&mut self) -> NMIPENDSET_W {
        NMIPENDSET_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Control and State Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icsr](index.html) module"]
pub struct ICSR_SPEC;
impl crate::RegisterSpec for ICSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [icsr::R](R) reader structure"]
impl crate::Readable for ICSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [icsr::W](W) writer structure"]
impl crate::Writable for ICSR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ICSR to value 0"]
impl crate::Resettable for ICSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
