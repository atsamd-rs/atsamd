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
pub type VECTACTIVE_R = crate::FieldReader<u16, u16>;
#[doc = "Field `VECTACTIVE` writer - Active exception number"]
pub type VECTACTIVE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ICSR_SPEC, u16, u16, 9, O>;
#[doc = "Field `RETTOBASE` reader - No preempted active exceptions to execute"]
pub type RETTOBASE_R = crate::BitReader<bool>;
#[doc = "Field `RETTOBASE` writer - No preempted active exceptions to execute"]
pub type RETTOBASE_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICSR_SPEC, bool, O>;
#[doc = "Field `VECTPENDING` reader - Exception number of the highest priority pending enabled exception"]
pub type VECTPENDING_R = crate::FieldReader<u8, u8>;
#[doc = "Field `VECTPENDING` writer - Exception number of the highest priority pending enabled exception"]
pub type VECTPENDING_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ICSR_SPEC, u8, u8, 6, O>;
#[doc = "Field `ISRPENDING` reader - Interrupt pending flag"]
pub type ISRPENDING_R = crate::BitReader<bool>;
#[doc = "Field `ISRPENDING` writer - Interrupt pending flag"]
pub type ISRPENDING_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICSR_SPEC, bool, O>;
#[doc = "Field `ISRPREEMPT` reader - Debug only"]
pub type ISRPREEMPT_R = crate::BitReader<bool>;
#[doc = "Field `ISRPREEMPT` writer - Debug only"]
pub type ISRPREEMPT_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICSR_SPEC, bool, O>;
#[doc = "Field `PENDSTCLR` reader - SysTick clear-pending bit"]
pub type PENDSTCLR_R = crate::BitReader<PENDSTCLRSELECT_A>;
#[doc = "SysTick clear-pending bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PENDSTCLRSELECT_A {
    #[doc = "0: No effect"]
    VALUE_0 = 0,
    #[doc = "1: Removes the pending state from the SysTick exception"]
    VALUE_1 = 1,
}
impl From<PENDSTCLRSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: PENDSTCLRSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl PENDSTCLR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PENDSTCLRSELECT_A {
        match self.bits {
            false => PENDSTCLRSELECT_A::VALUE_0,
            true => PENDSTCLRSELECT_A::VALUE_1,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE_0`"]
    #[inline(always)]
    pub fn is_value_0(&self) -> bool {
        *self == PENDSTCLRSELECT_A::VALUE_0
    }
    #[doc = "Checks if the value of the field is `VALUE_1`"]
    #[inline(always)]
    pub fn is_value_1(&self) -> bool {
        *self == PENDSTCLRSELECT_A::VALUE_1
    }
}
#[doc = "Field `PENDSTCLR` writer - SysTick clear-pending bit"]
pub type PENDSTCLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICSR_SPEC, PENDSTCLRSELECT_A, O>;
impl<'a, const O: u8> PENDSTCLR_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value_0(self) -> &'a mut W {
        self.variant(PENDSTCLRSELECT_A::VALUE_0)
    }
    #[doc = "Removes the pending state from the SysTick exception"]
    #[inline(always)]
    pub fn value_1(self) -> &'a mut W {
        self.variant(PENDSTCLRSELECT_A::VALUE_1)
    }
}
#[doc = "Field `PENDSTSET` reader - SysTick set-pending bit"]
pub type PENDSTSET_R = crate::BitReader<PENDSTSETSELECT_A>;
#[doc = "SysTick set-pending bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PENDSTSETSELECT_A {
    #[doc = "0: Write: no effect; read: SysTick exception is not pending"]
    VALUE_0 = 0,
    #[doc = "1: Write: changes SysTick exception state to pending; read: SysTick exception is pending"]
    VALUE_1 = 1,
}
impl From<PENDSTSETSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: PENDSTSETSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl PENDSTSET_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PENDSTSETSELECT_A {
        match self.bits {
            false => PENDSTSETSELECT_A::VALUE_0,
            true => PENDSTSETSELECT_A::VALUE_1,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE_0`"]
    #[inline(always)]
    pub fn is_value_0(&self) -> bool {
        *self == PENDSTSETSELECT_A::VALUE_0
    }
    #[doc = "Checks if the value of the field is `VALUE_1`"]
    #[inline(always)]
    pub fn is_value_1(&self) -> bool {
        *self == PENDSTSETSELECT_A::VALUE_1
    }
}
#[doc = "Field `PENDSTSET` writer - SysTick set-pending bit"]
pub type PENDSTSET_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICSR_SPEC, PENDSTSETSELECT_A, O>;
impl<'a, const O: u8> PENDSTSET_W<'a, O> {
    #[doc = "Write: no effect; read: SysTick exception is not pending"]
    #[inline(always)]
    pub fn value_0(self) -> &'a mut W {
        self.variant(PENDSTSETSELECT_A::VALUE_0)
    }
    #[doc = "Write: changes SysTick exception state to pending; read: SysTick exception is pending"]
    #[inline(always)]
    pub fn value_1(self) -> &'a mut W {
        self.variant(PENDSTSETSELECT_A::VALUE_1)
    }
}
#[doc = "Field `PENDSVCLR` reader - PendSV clear-pending bit"]
pub type PENDSVCLR_R = crate::BitReader<PENDSVCLRSELECT_A>;
#[doc = "PendSV clear-pending bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PENDSVCLRSELECT_A {
    #[doc = "0: No effect"]
    VALUE_0 = 0,
    #[doc = "1: Removes the pending state from the PendSV exception"]
    VALUE_1 = 1,
}
impl From<PENDSVCLRSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: PENDSVCLRSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl PENDSVCLR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PENDSVCLRSELECT_A {
        match self.bits {
            false => PENDSVCLRSELECT_A::VALUE_0,
            true => PENDSVCLRSELECT_A::VALUE_1,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE_0`"]
    #[inline(always)]
    pub fn is_value_0(&self) -> bool {
        *self == PENDSVCLRSELECT_A::VALUE_0
    }
    #[doc = "Checks if the value of the field is `VALUE_1`"]
    #[inline(always)]
    pub fn is_value_1(&self) -> bool {
        *self == PENDSVCLRSELECT_A::VALUE_1
    }
}
#[doc = "Field `PENDSVCLR` writer - PendSV clear-pending bit"]
pub type PENDSVCLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICSR_SPEC, PENDSVCLRSELECT_A, O>;
impl<'a, const O: u8> PENDSVCLR_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value_0(self) -> &'a mut W {
        self.variant(PENDSVCLRSELECT_A::VALUE_0)
    }
    #[doc = "Removes the pending state from the PendSV exception"]
    #[inline(always)]
    pub fn value_1(self) -> &'a mut W {
        self.variant(PENDSVCLRSELECT_A::VALUE_1)
    }
}
#[doc = "Field `PENDSVSET` reader - PendSV set-pending bit"]
pub type PENDSVSET_R = crate::BitReader<PENDSVSETSELECT_A>;
#[doc = "PendSV set-pending bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PENDSVSETSELECT_A {
    #[doc = "0: Write: no effect; read: PendSV exception is not pending"]
    VALUE_0 = 0,
    #[doc = "1: Write: changes PendSV exception state to pending; read: PendSV exception is pending"]
    VALUE_1 = 1,
}
impl From<PENDSVSETSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: PENDSVSETSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl PENDSVSET_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PENDSVSETSELECT_A {
        match self.bits {
            false => PENDSVSETSELECT_A::VALUE_0,
            true => PENDSVSETSELECT_A::VALUE_1,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE_0`"]
    #[inline(always)]
    pub fn is_value_0(&self) -> bool {
        *self == PENDSVSETSELECT_A::VALUE_0
    }
    #[doc = "Checks if the value of the field is `VALUE_1`"]
    #[inline(always)]
    pub fn is_value_1(&self) -> bool {
        *self == PENDSVSETSELECT_A::VALUE_1
    }
}
#[doc = "Field `PENDSVSET` writer - PendSV set-pending bit"]
pub type PENDSVSET_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICSR_SPEC, PENDSVSETSELECT_A, O>;
impl<'a, const O: u8> PENDSVSET_W<'a, O> {
    #[doc = "Write: no effect; read: PendSV exception is not pending"]
    #[inline(always)]
    pub fn value_0(self) -> &'a mut W {
        self.variant(PENDSVSETSELECT_A::VALUE_0)
    }
    #[doc = "Write: changes PendSV exception state to pending; read: PendSV exception is pending"]
    #[inline(always)]
    pub fn value_1(self) -> &'a mut W {
        self.variant(PENDSVSETSELECT_A::VALUE_1)
    }
}
#[doc = "Field `NMIPENDSET` reader - NMI set-pending bit"]
pub type NMIPENDSET_R = crate::BitReader<NMIPENDSETSELECT_A>;
#[doc = "NMI set-pending bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NMIPENDSETSELECT_A {
    #[doc = "0: Write: no effect; read: NMI exception is not pending"]
    VALUE_0 = 0,
    #[doc = "1: Write: changes NMI exception state to pending; read: NMI exception is pending"]
    VALUE_1 = 1,
}
impl From<NMIPENDSETSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: NMIPENDSETSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl NMIPENDSET_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NMIPENDSETSELECT_A {
        match self.bits {
            false => NMIPENDSETSELECT_A::VALUE_0,
            true => NMIPENDSETSELECT_A::VALUE_1,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE_0`"]
    #[inline(always)]
    pub fn is_value_0(&self) -> bool {
        *self == NMIPENDSETSELECT_A::VALUE_0
    }
    #[doc = "Checks if the value of the field is `VALUE_1`"]
    #[inline(always)]
    pub fn is_value_1(&self) -> bool {
        *self == NMIPENDSETSELECT_A::VALUE_1
    }
}
#[doc = "Field `NMIPENDSET` writer - NMI set-pending bit"]
pub type NMIPENDSET_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ICSR_SPEC, NMIPENDSETSELECT_A, O>;
impl<'a, const O: u8> NMIPENDSET_W<'a, O> {
    #[doc = "Write: no effect; read: NMI exception is not pending"]
    #[inline(always)]
    pub fn value_0(self) -> &'a mut W {
        self.variant(NMIPENDSETSELECT_A::VALUE_0)
    }
    #[doc = "Write: changes NMI exception state to pending; read: NMI exception is pending"]
    #[inline(always)]
    pub fn value_1(self) -> &'a mut W {
        self.variant(NMIPENDSETSELECT_A::VALUE_1)
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
        RETTOBASE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:17 - Exception number of the highest priority pending enabled exception"]
    #[inline(always)]
    pub fn vectpending(&self) -> VECTPENDING_R {
        VECTPENDING_R::new(((self.bits >> 12) & 0x3f) as u8)
    }
    #[doc = "Bit 22 - Interrupt pending flag"]
    #[inline(always)]
    pub fn isrpending(&self) -> ISRPENDING_R {
        ISRPENDING_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Debug only"]
    #[inline(always)]
    pub fn isrpreempt(&self) -> ISRPREEMPT_R {
        ISRPREEMPT_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 25 - SysTick clear-pending bit"]
    #[inline(always)]
    pub fn pendstclr(&self) -> PENDSTCLR_R {
        PENDSTCLR_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - SysTick set-pending bit"]
    #[inline(always)]
    pub fn pendstset(&self) -> PENDSTSET_R {
        PENDSTSET_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - PendSV clear-pending bit"]
    #[inline(always)]
    pub fn pendsvclr(&self) -> PENDSVCLR_R {
        PENDSVCLR_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - PendSV set-pending bit"]
    #[inline(always)]
    pub fn pendsvset(&self) -> PENDSVSET_R {
        PENDSVSET_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 31 - NMI set-pending bit"]
    #[inline(always)]
    pub fn nmipendset(&self) -> NMIPENDSET_R {
        NMIPENDSET_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:8 - Active exception number"]
    #[inline(always)]
    #[must_use]
    pub fn vectactive(&mut self) -> VECTACTIVE_W<0> {
        VECTACTIVE_W::new(self)
    }
    #[doc = "Bit 11 - No preempted active exceptions to execute"]
    #[inline(always)]
    #[must_use]
    pub fn rettobase(&mut self) -> RETTOBASE_W<11> {
        RETTOBASE_W::new(self)
    }
    #[doc = "Bits 12:17 - Exception number of the highest priority pending enabled exception"]
    #[inline(always)]
    #[must_use]
    pub fn vectpending(&mut self) -> VECTPENDING_W<12> {
        VECTPENDING_W::new(self)
    }
    #[doc = "Bit 22 - Interrupt pending flag"]
    #[inline(always)]
    #[must_use]
    pub fn isrpending(&mut self) -> ISRPENDING_W<22> {
        ISRPENDING_W::new(self)
    }
    #[doc = "Bit 23 - Debug only"]
    #[inline(always)]
    #[must_use]
    pub fn isrpreempt(&mut self) -> ISRPREEMPT_W<23> {
        ISRPREEMPT_W::new(self)
    }
    #[doc = "Bit 25 - SysTick clear-pending bit"]
    #[inline(always)]
    #[must_use]
    pub fn pendstclr(&mut self) -> PENDSTCLR_W<25> {
        PENDSTCLR_W::new(self)
    }
    #[doc = "Bit 26 - SysTick set-pending bit"]
    #[inline(always)]
    #[must_use]
    pub fn pendstset(&mut self) -> PENDSTSET_W<26> {
        PENDSTSET_W::new(self)
    }
    #[doc = "Bit 27 - PendSV clear-pending bit"]
    #[inline(always)]
    #[must_use]
    pub fn pendsvclr(&mut self) -> PENDSVCLR_W<27> {
        PENDSVCLR_W::new(self)
    }
    #[doc = "Bit 28 - PendSV set-pending bit"]
    #[inline(always)]
    #[must_use]
    pub fn pendsvset(&mut self) -> PENDSVSET_W<28> {
        PENDSVSET_W::new(self)
    }
    #[doc = "Bit 31 - NMI set-pending bit"]
    #[inline(always)]
    #[must_use]
    pub fn nmipendset(&mut self) -> NMIPENDSET_W<31> {
        NMIPENDSET_W::new(self)
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ICSR to value 0"]
impl crate::Resettable for ICSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
