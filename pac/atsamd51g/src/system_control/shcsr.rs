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
pub type MEMFAULTACT_R = crate::BitReader<bool>;
#[doc = "Field `MEMFAULTACT` writer - MemManage exception active bit"]
pub type MEMFAULTACT_W<'a, const O: u8> = crate::BitWriter<'a, u32, SHCSR_SPEC, bool, O>;
#[doc = "Field `BUSFAULTACT` reader - BusFault exception active bit"]
pub type BUSFAULTACT_R = crate::BitReader<bool>;
#[doc = "Field `BUSFAULTACT` writer - BusFault exception active bit"]
pub type BUSFAULTACT_W<'a, const O: u8> = crate::BitWriter<'a, u32, SHCSR_SPEC, bool, O>;
#[doc = "Field `USGFAULTACT` reader - UsageFault exception active bit"]
pub type USGFAULTACT_R = crate::BitReader<bool>;
#[doc = "Field `USGFAULTACT` writer - UsageFault exception active bit"]
pub type USGFAULTACT_W<'a, const O: u8> = crate::BitWriter<'a, u32, SHCSR_SPEC, bool, O>;
#[doc = "Field `SVCALLACT` reader - SVCall active bit"]
pub type SVCALLACT_R = crate::BitReader<bool>;
#[doc = "Field `SVCALLACT` writer - SVCall active bit"]
pub type SVCALLACT_W<'a, const O: u8> = crate::BitWriter<'a, u32, SHCSR_SPEC, bool, O>;
#[doc = "Field `MONITORACT` reader - DebugMonitor exception active bit"]
pub type MONITORACT_R = crate::BitReader<bool>;
#[doc = "Field `MONITORACT` writer - DebugMonitor exception active bit"]
pub type MONITORACT_W<'a, const O: u8> = crate::BitWriter<'a, u32, SHCSR_SPEC, bool, O>;
#[doc = "Field `PENDSVACT` reader - PendSV exception active bit"]
pub type PENDSVACT_R = crate::BitReader<bool>;
#[doc = "Field `PENDSVACT` writer - PendSV exception active bit"]
pub type PENDSVACT_W<'a, const O: u8> = crate::BitWriter<'a, u32, SHCSR_SPEC, bool, O>;
#[doc = "Field `SYSTICKACT` reader - SysTick exception active bit"]
pub type SYSTICKACT_R = crate::BitReader<bool>;
#[doc = "Field `SYSTICKACT` writer - SysTick exception active bit"]
pub type SYSTICKACT_W<'a, const O: u8> = crate::BitWriter<'a, u32, SHCSR_SPEC, bool, O>;
#[doc = "Field `USGFAULTPENDED` reader - UsageFault exception pending bit"]
pub type USGFAULTPENDED_R = crate::BitReader<bool>;
#[doc = "Field `USGFAULTPENDED` writer - UsageFault exception pending bit"]
pub type USGFAULTPENDED_W<'a, const O: u8> = crate::BitWriter<'a, u32, SHCSR_SPEC, bool, O>;
#[doc = "Field `MEMFAULTPENDED` reader - MemManage exception pending bit"]
pub type MEMFAULTPENDED_R = crate::BitReader<bool>;
#[doc = "Field `MEMFAULTPENDED` writer - MemManage exception pending bit"]
pub type MEMFAULTPENDED_W<'a, const O: u8> = crate::BitWriter<'a, u32, SHCSR_SPEC, bool, O>;
#[doc = "Field `BUSFAULTPENDED` reader - BusFault exception pending bit"]
pub type BUSFAULTPENDED_R = crate::BitReader<bool>;
#[doc = "Field `BUSFAULTPENDED` writer - BusFault exception pending bit"]
pub type BUSFAULTPENDED_W<'a, const O: u8> = crate::BitWriter<'a, u32, SHCSR_SPEC, bool, O>;
#[doc = "Field `SVCALLPENDED` reader - SVCall pending bit"]
pub type SVCALLPENDED_R = crate::BitReader<bool>;
#[doc = "Field `SVCALLPENDED` writer - SVCall pending bit"]
pub type SVCALLPENDED_W<'a, const O: u8> = crate::BitWriter<'a, u32, SHCSR_SPEC, bool, O>;
#[doc = "Field `MEMFAULTENA` reader - MemManage enable bit"]
pub type MEMFAULTENA_R = crate::BitReader<bool>;
#[doc = "Field `MEMFAULTENA` writer - MemManage enable bit"]
pub type MEMFAULTENA_W<'a, const O: u8> = crate::BitWriter<'a, u32, SHCSR_SPEC, bool, O>;
#[doc = "Field `BUSFAULTENA` reader - BusFault enable bit"]
pub type BUSFAULTENA_R = crate::BitReader<bool>;
#[doc = "Field `BUSFAULTENA` writer - BusFault enable bit"]
pub type BUSFAULTENA_W<'a, const O: u8> = crate::BitWriter<'a, u32, SHCSR_SPEC, bool, O>;
#[doc = "Field `USGFAULTENA` reader - UsageFault enable bit"]
pub type USGFAULTENA_R = crate::BitReader<bool>;
#[doc = "Field `USGFAULTENA` writer - UsageFault enable bit"]
pub type USGFAULTENA_W<'a, const O: u8> = crate::BitWriter<'a, u32, SHCSR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - MemManage exception active bit"]
    #[inline(always)]
    pub fn memfaultact(&self) -> MEMFAULTACT_R {
        MEMFAULTACT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - BusFault exception active bit"]
    #[inline(always)]
    pub fn busfaultact(&self) -> BUSFAULTACT_R {
        BUSFAULTACT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - UsageFault exception active bit"]
    #[inline(always)]
    pub fn usgfaultact(&self) -> USGFAULTACT_R {
        USGFAULTACT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 7 - SVCall active bit"]
    #[inline(always)]
    pub fn svcallact(&self) -> SVCALLACT_R {
        SVCALLACT_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - DebugMonitor exception active bit"]
    #[inline(always)]
    pub fn monitoract(&self) -> MONITORACT_R {
        MONITORACT_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - PendSV exception active bit"]
    #[inline(always)]
    pub fn pendsvact(&self) -> PENDSVACT_R {
        PENDSVACT_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - SysTick exception active bit"]
    #[inline(always)]
    pub fn systickact(&self) -> SYSTICKACT_R {
        SYSTICKACT_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - UsageFault exception pending bit"]
    #[inline(always)]
    pub fn usgfaultpended(&self) -> USGFAULTPENDED_R {
        USGFAULTPENDED_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - MemManage exception pending bit"]
    #[inline(always)]
    pub fn memfaultpended(&self) -> MEMFAULTPENDED_R {
        MEMFAULTPENDED_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - BusFault exception pending bit"]
    #[inline(always)]
    pub fn busfaultpended(&self) -> BUSFAULTPENDED_R {
        BUSFAULTPENDED_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - SVCall pending bit"]
    #[inline(always)]
    pub fn svcallpended(&self) -> SVCALLPENDED_R {
        SVCALLPENDED_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - MemManage enable bit"]
    #[inline(always)]
    pub fn memfaultena(&self) -> MEMFAULTENA_R {
        MEMFAULTENA_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - BusFault enable bit"]
    #[inline(always)]
    pub fn busfaultena(&self) -> BUSFAULTENA_R {
        BUSFAULTENA_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - UsageFault enable bit"]
    #[inline(always)]
    pub fn usgfaultena(&self) -> USGFAULTENA_R {
        USGFAULTENA_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - MemManage exception active bit"]
    #[inline(always)]
    #[must_use]
    pub fn memfaultact(&mut self) -> MEMFAULTACT_W<0> {
        MEMFAULTACT_W::new(self)
    }
    #[doc = "Bit 1 - BusFault exception active bit"]
    #[inline(always)]
    #[must_use]
    pub fn busfaultact(&mut self) -> BUSFAULTACT_W<1> {
        BUSFAULTACT_W::new(self)
    }
    #[doc = "Bit 3 - UsageFault exception active bit"]
    #[inline(always)]
    #[must_use]
    pub fn usgfaultact(&mut self) -> USGFAULTACT_W<3> {
        USGFAULTACT_W::new(self)
    }
    #[doc = "Bit 7 - SVCall active bit"]
    #[inline(always)]
    #[must_use]
    pub fn svcallact(&mut self) -> SVCALLACT_W<7> {
        SVCALLACT_W::new(self)
    }
    #[doc = "Bit 8 - DebugMonitor exception active bit"]
    #[inline(always)]
    #[must_use]
    pub fn monitoract(&mut self) -> MONITORACT_W<8> {
        MONITORACT_W::new(self)
    }
    #[doc = "Bit 10 - PendSV exception active bit"]
    #[inline(always)]
    #[must_use]
    pub fn pendsvact(&mut self) -> PENDSVACT_W<10> {
        PENDSVACT_W::new(self)
    }
    #[doc = "Bit 11 - SysTick exception active bit"]
    #[inline(always)]
    #[must_use]
    pub fn systickact(&mut self) -> SYSTICKACT_W<11> {
        SYSTICKACT_W::new(self)
    }
    #[doc = "Bit 12 - UsageFault exception pending bit"]
    #[inline(always)]
    #[must_use]
    pub fn usgfaultpended(&mut self) -> USGFAULTPENDED_W<12> {
        USGFAULTPENDED_W::new(self)
    }
    #[doc = "Bit 13 - MemManage exception pending bit"]
    #[inline(always)]
    #[must_use]
    pub fn memfaultpended(&mut self) -> MEMFAULTPENDED_W<13> {
        MEMFAULTPENDED_W::new(self)
    }
    #[doc = "Bit 14 - BusFault exception pending bit"]
    #[inline(always)]
    #[must_use]
    pub fn busfaultpended(&mut self) -> BUSFAULTPENDED_W<14> {
        BUSFAULTPENDED_W::new(self)
    }
    #[doc = "Bit 15 - SVCall pending bit"]
    #[inline(always)]
    #[must_use]
    pub fn svcallpended(&mut self) -> SVCALLPENDED_W<15> {
        SVCALLPENDED_W::new(self)
    }
    #[doc = "Bit 16 - MemManage enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn memfaultena(&mut self) -> MEMFAULTENA_W<16> {
        MEMFAULTENA_W::new(self)
    }
    #[doc = "Bit 17 - BusFault enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn busfaultena(&mut self) -> BUSFAULTENA_W<17> {
        BUSFAULTENA_W::new(self)
    }
    #[doc = "Bit 18 - UsageFault enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn usgfaultena(&mut self) -> USGFAULTENA_W<18> {
        USGFAULTENA_W::new(self)
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SHCSR to value 0"]
impl crate::Resettable for SHCSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
