#[doc = "Register `SHCSR` reader"]
pub struct R(crate::R<SHCSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SHCSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SHCSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SHCSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SHCSR` writer"]
pub struct W(crate::W<SHCSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SHCSR_SPEC>;
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
impl From<crate::W<SHCSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SHCSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MEMFAULTACT` reader - MemManage exception active bit"]
pub struct MEMFAULTACT_R(crate::FieldReader<bool, bool>);
impl MEMFAULTACT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MEMFAULTACT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MEMFAULTACT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MEMFAULTACT` writer - MemManage exception active bit"]
pub struct MEMFAULTACT_W<'a> {
    w: &'a mut W,
}
impl<'a> MEMFAULTACT_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Field `BUSFAULTACT` reader - BusFault exception active bit"]
pub struct BUSFAULTACT_R(crate::FieldReader<bool, bool>);
impl BUSFAULTACT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BUSFAULTACT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BUSFAULTACT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BUSFAULTACT` writer - BusFault exception active bit"]
pub struct BUSFAULTACT_W<'a> {
    w: &'a mut W,
}
impl<'a> BUSFAULTACT_W<'a> {
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
#[doc = "Field `USGFAULTACT` reader - UsageFault exception active bit"]
pub struct USGFAULTACT_R(crate::FieldReader<bool, bool>);
impl USGFAULTACT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        USGFAULTACT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USGFAULTACT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USGFAULTACT` writer - UsageFault exception active bit"]
pub struct USGFAULTACT_W<'a> {
    w: &'a mut W,
}
impl<'a> USGFAULTACT_W<'a> {
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
#[doc = "Field `SVCALLACT` reader - SVCall active bit"]
pub struct SVCALLACT_R(crate::FieldReader<bool, bool>);
impl SVCALLACT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SVCALLACT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SVCALLACT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SVCALLACT` writer - SVCall active bit"]
pub struct SVCALLACT_W<'a> {
    w: &'a mut W,
}
impl<'a> SVCALLACT_W<'a> {
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
#[doc = "Field `MONITORACT` reader - DebugMonitor exception active bit"]
pub struct MONITORACT_R(crate::FieldReader<bool, bool>);
impl MONITORACT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MONITORACT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MONITORACT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MONITORACT` writer - DebugMonitor exception active bit"]
pub struct MONITORACT_W<'a> {
    w: &'a mut W,
}
impl<'a> MONITORACT_W<'a> {
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
#[doc = "Field `PENDSVACT` reader - PendSV exception active bit"]
pub struct PENDSVACT_R(crate::FieldReader<bool, bool>);
impl PENDSVACT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PENDSVACT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PENDSVACT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PENDSVACT` writer - PendSV exception active bit"]
pub struct PENDSVACT_W<'a> {
    w: &'a mut W,
}
impl<'a> PENDSVACT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "Field `SYSTICKACT` reader - SysTick exception active bit"]
pub struct SYSTICKACT_R(crate::FieldReader<bool, bool>);
impl SYSTICKACT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SYSTICKACT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SYSTICKACT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SYSTICKACT` writer - SysTick exception active bit"]
pub struct SYSTICKACT_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSTICKACT_W<'a> {
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
#[doc = "Field `USGFAULTPENDED` reader - UsageFault exception pending bit"]
pub struct USGFAULTPENDED_R(crate::FieldReader<bool, bool>);
impl USGFAULTPENDED_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        USGFAULTPENDED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USGFAULTPENDED_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USGFAULTPENDED` writer - UsageFault exception pending bit"]
pub struct USGFAULTPENDED_W<'a> {
    w: &'a mut W,
}
impl<'a> USGFAULTPENDED_W<'a> {
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
#[doc = "Field `MEMFAULTPENDED` reader - MemManage exception pending bit"]
pub struct MEMFAULTPENDED_R(crate::FieldReader<bool, bool>);
impl MEMFAULTPENDED_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MEMFAULTPENDED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MEMFAULTPENDED_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MEMFAULTPENDED` writer - MemManage exception pending bit"]
pub struct MEMFAULTPENDED_W<'a> {
    w: &'a mut W,
}
impl<'a> MEMFAULTPENDED_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "Field `BUSFAULTPENDED` reader - BusFault exception pending bit"]
pub struct BUSFAULTPENDED_R(crate::FieldReader<bool, bool>);
impl BUSFAULTPENDED_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BUSFAULTPENDED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BUSFAULTPENDED_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BUSFAULTPENDED` writer - BusFault exception pending bit"]
pub struct BUSFAULTPENDED_W<'a> {
    w: &'a mut W,
}
impl<'a> BUSFAULTPENDED_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
#[doc = "Field `SVCALLPENDED` reader - SVCall pending bit"]
pub struct SVCALLPENDED_R(crate::FieldReader<bool, bool>);
impl SVCALLPENDED_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SVCALLPENDED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SVCALLPENDED_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SVCALLPENDED` writer - SVCall pending bit"]
pub struct SVCALLPENDED_W<'a> {
    w: &'a mut W,
}
impl<'a> SVCALLPENDED_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
#[doc = "Field `MEMFAULTENA` reader - MemManage enable bit"]
pub struct MEMFAULTENA_R(crate::FieldReader<bool, bool>);
impl MEMFAULTENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MEMFAULTENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MEMFAULTENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MEMFAULTENA` writer - MemManage enable bit"]
pub struct MEMFAULTENA_W<'a> {
    w: &'a mut W,
}
impl<'a> MEMFAULTENA_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Field `BUSFAULTENA` reader - BusFault enable bit"]
pub struct BUSFAULTENA_R(crate::FieldReader<bool, bool>);
impl BUSFAULTENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BUSFAULTENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BUSFAULTENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BUSFAULTENA` writer - BusFault enable bit"]
pub struct BUSFAULTENA_W<'a> {
    w: &'a mut W,
}
impl<'a> BUSFAULTENA_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "Field `USGFAULTENA` reader - UsageFault enable bit"]
pub struct USGFAULTENA_R(crate::FieldReader<bool, bool>);
impl USGFAULTENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        USGFAULTENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USGFAULTENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USGFAULTENA` writer - UsageFault enable bit"]
pub struct USGFAULTENA_W<'a> {
    w: &'a mut W,
}
impl<'a> USGFAULTENA_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - MemManage exception active bit"]
    #[inline(always)]
    pub fn memfaultact(&self) -> MEMFAULTACT_R {
        MEMFAULTACT_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - BusFault exception active bit"]
    #[inline(always)]
    pub fn busfaultact(&self) -> BUSFAULTACT_R {
        BUSFAULTACT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 3 - UsageFault exception active bit"]
    #[inline(always)]
    pub fn usgfaultact(&self) -> USGFAULTACT_R {
        USGFAULTACT_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 7 - SVCall active bit"]
    #[inline(always)]
    pub fn svcallact(&self) -> SVCALLACT_R {
        SVCALLACT_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - DebugMonitor exception active bit"]
    #[inline(always)]
    pub fn monitoract(&self) -> MONITORACT_R {
        MONITORACT_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 10 - PendSV exception active bit"]
    #[inline(always)]
    pub fn pendsvact(&self) -> PENDSVACT_R {
        PENDSVACT_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - SysTick exception active bit"]
    #[inline(always)]
    pub fn systickact(&self) -> SYSTICKACT_R {
        SYSTICKACT_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - UsageFault exception pending bit"]
    #[inline(always)]
    pub fn usgfaultpended(&self) -> USGFAULTPENDED_R {
        USGFAULTPENDED_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - MemManage exception pending bit"]
    #[inline(always)]
    pub fn memfaultpended(&self) -> MEMFAULTPENDED_R {
        MEMFAULTPENDED_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - BusFault exception pending bit"]
    #[inline(always)]
    pub fn busfaultpended(&self) -> BUSFAULTPENDED_R {
        BUSFAULTPENDED_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - SVCall pending bit"]
    #[inline(always)]
    pub fn svcallpended(&self) -> SVCALLPENDED_R {
        SVCALLPENDED_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - MemManage enable bit"]
    #[inline(always)]
    pub fn memfaultena(&self) -> MEMFAULTENA_R {
        MEMFAULTENA_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - BusFault enable bit"]
    #[inline(always)]
    pub fn busfaultena(&self) -> BUSFAULTENA_R {
        BUSFAULTENA_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - UsageFault enable bit"]
    #[inline(always)]
    pub fn usgfaultena(&self) -> USGFAULTENA_R {
        USGFAULTENA_R::new(((self.bits >> 18) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - MemManage exception active bit"]
    #[inline(always)]
    pub fn memfaultact(&mut self) -> MEMFAULTACT_W {
        MEMFAULTACT_W { w: self }
    }
    #[doc = "Bit 1 - BusFault exception active bit"]
    #[inline(always)]
    pub fn busfaultact(&mut self) -> BUSFAULTACT_W {
        BUSFAULTACT_W { w: self }
    }
    #[doc = "Bit 3 - UsageFault exception active bit"]
    #[inline(always)]
    pub fn usgfaultact(&mut self) -> USGFAULTACT_W {
        USGFAULTACT_W { w: self }
    }
    #[doc = "Bit 7 - SVCall active bit"]
    #[inline(always)]
    pub fn svcallact(&mut self) -> SVCALLACT_W {
        SVCALLACT_W { w: self }
    }
    #[doc = "Bit 8 - DebugMonitor exception active bit"]
    #[inline(always)]
    pub fn monitoract(&mut self) -> MONITORACT_W {
        MONITORACT_W { w: self }
    }
    #[doc = "Bit 10 - PendSV exception active bit"]
    #[inline(always)]
    pub fn pendsvact(&mut self) -> PENDSVACT_W {
        PENDSVACT_W { w: self }
    }
    #[doc = "Bit 11 - SysTick exception active bit"]
    #[inline(always)]
    pub fn systickact(&mut self) -> SYSTICKACT_W {
        SYSTICKACT_W { w: self }
    }
    #[doc = "Bit 12 - UsageFault exception pending bit"]
    #[inline(always)]
    pub fn usgfaultpended(&mut self) -> USGFAULTPENDED_W {
        USGFAULTPENDED_W { w: self }
    }
    #[doc = "Bit 13 - MemManage exception pending bit"]
    #[inline(always)]
    pub fn memfaultpended(&mut self) -> MEMFAULTPENDED_W {
        MEMFAULTPENDED_W { w: self }
    }
    #[doc = "Bit 14 - BusFault exception pending bit"]
    #[inline(always)]
    pub fn busfaultpended(&mut self) -> BUSFAULTPENDED_W {
        BUSFAULTPENDED_W { w: self }
    }
    #[doc = "Bit 15 - SVCall pending bit"]
    #[inline(always)]
    pub fn svcallpended(&mut self) -> SVCALLPENDED_W {
        SVCALLPENDED_W { w: self }
    }
    #[doc = "Bit 16 - MemManage enable bit"]
    #[inline(always)]
    pub fn memfaultena(&mut self) -> MEMFAULTENA_W {
        MEMFAULTENA_W { w: self }
    }
    #[doc = "Bit 17 - BusFault enable bit"]
    #[inline(always)]
    pub fn busfaultena(&mut self) -> BUSFAULTENA_W {
        BUSFAULTENA_W { w: self }
    }
    #[doc = "Bit 18 - UsageFault enable bit"]
    #[inline(always)]
    pub fn usgfaultena(&mut self) -> USGFAULTENA_W {
        USGFAULTENA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "System Handler Control and State Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [shcsr](index.html) module"]
pub struct SHCSR_SPEC;
impl crate::RegisterSpec for SHCSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [shcsr::R](R) reader structure"]
impl crate::Readable for SHCSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [shcsr::W](W) writer structure"]
impl crate::Writable for SHCSR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SHCSR to value 0"]
impl crate::Resettable for SHCSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
