#[doc = "Register `SLEEP` reader"]
pub type R = crate::R<SLEEP_SPEC>;
#[doc = "Register `SLEEP` writer"]
pub type W = crate::W<SLEEP_SPEC>;
#[doc = "Field `IDLE` reader - Idle Mode Configuration"]
pub type IDLE_R = crate::FieldReader<IDLESELECT_A>;
#[doc = "Idle Mode Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IDLESELECT_A {
    #[doc = "0: The CPU clock domain is stopped"]
    CPU = 0,
    #[doc = "1: The CPU and AHB clock domains are stopped"]
    AHB = 1,
    #[doc = "2: The CPU, AHB and APB clock domains are stopped"]
    APB = 2,
}
impl From<IDLESELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: IDLESELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for IDLESELECT_A {
    type Ux = u8;
}
impl IDLE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<IDLESELECT_A> {
        match self.bits {
            0 => Some(IDLESELECT_A::CPU),
            1 => Some(IDLESELECT_A::AHB),
            2 => Some(IDLESELECT_A::APB),
            _ => None,
        }
    }
    #[doc = "The CPU clock domain is stopped"]
    #[inline(always)]
    pub fn is_cpu(&self) -> bool {
        *self == IDLESELECT_A::CPU
    }
    #[doc = "The CPU and AHB clock domains are stopped"]
    #[inline(always)]
    pub fn is_ahb(&self) -> bool {
        *self == IDLESELECT_A::AHB
    }
    #[doc = "The CPU, AHB and APB clock domains are stopped"]
    #[inline(always)]
    pub fn is_apb(&self) -> bool {
        *self == IDLESELECT_A::APB
    }
}
#[doc = "Field `IDLE` writer - Idle Mode Configuration"]
pub type IDLE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O, IDLESELECT_A>;
impl<'a, REG, const O: u8> IDLE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "The CPU clock domain is stopped"]
    #[inline(always)]
    pub fn cpu(self) -> &'a mut crate::W<REG> {
        self.variant(IDLESELECT_A::CPU)
    }
    #[doc = "The CPU and AHB clock domains are stopped"]
    #[inline(always)]
    pub fn ahb(self) -> &'a mut crate::W<REG> {
        self.variant(IDLESELECT_A::AHB)
    }
    #[doc = "The CPU, AHB and APB clock domains are stopped"]
    #[inline(always)]
    pub fn apb(self) -> &'a mut crate::W<REG> {
        self.variant(IDLESELECT_A::APB)
    }
}
impl R {
    #[doc = "Bits 0:1 - Idle Mode Configuration"]
    #[inline(always)]
    pub fn idle(&self) -> IDLE_R {
        IDLE_R::new(self.bits & 3)
    }
}
impl W {
    #[doc = "Bits 0:1 - Idle Mode Configuration"]
    #[inline(always)]
    #[must_use]
    pub fn idle(&mut self) -> IDLE_W<SLEEP_SPEC, 0> {
        IDLE_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Sleep Mode\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sleep::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sleep::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SLEEP_SPEC;
impl crate::RegisterSpec for SLEEP_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`sleep::R`](R) reader structure"]
impl crate::Readable for SLEEP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sleep::W`](W) writer structure"]
impl crate::Writable for SLEEP_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SLEEP to value 0"]
impl crate::Resettable for SLEEP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
