#[doc = "Register `SLEEP` reader"]
pub type R = crate::R<SleepSpec>;
#[doc = "Register `SLEEP` writer"]
pub type W = crate::W<SleepSpec>;
#[doc = "Idle Mode Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Idleselect {
    #[doc = "0: The CPU clock domain is stopped"]
    Cpu = 0,
    #[doc = "1: The CPU and AHB clock domains are stopped"]
    Ahb = 1,
    #[doc = "2: The CPU, AHB and APB clock domains are stopped"]
    Apb = 2,
}
impl From<Idleselect> for u8 {
    #[inline(always)]
    fn from(variant: Idleselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Idleselect {
    type Ux = u8;
}
impl crate::IsEnum for Idleselect {}
#[doc = "Field `IDLE` reader - Idle Mode Configuration"]
pub type IdleR = crate::FieldReader<Idleselect>;
impl IdleR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Idleselect> {
        match self.bits {
            0 => Some(Idleselect::Cpu),
            1 => Some(Idleselect::Ahb),
            2 => Some(Idleselect::Apb),
            _ => None,
        }
    }
    #[doc = "The CPU clock domain is stopped"]
    #[inline(always)]
    pub fn is_cpu(&self) -> bool {
        *self == Idleselect::Cpu
    }
    #[doc = "The CPU and AHB clock domains are stopped"]
    #[inline(always)]
    pub fn is_ahb(&self) -> bool {
        *self == Idleselect::Ahb
    }
    #[doc = "The CPU, AHB and APB clock domains are stopped"]
    #[inline(always)]
    pub fn is_apb(&self) -> bool {
        *self == Idleselect::Apb
    }
}
#[doc = "Field `IDLE` writer - Idle Mode Configuration"]
pub type IdleW<'a, REG> = crate::FieldWriter<'a, REG, 2, Idleselect>;
impl<'a, REG> IdleW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "The CPU clock domain is stopped"]
    #[inline(always)]
    pub fn cpu(self) -> &'a mut crate::W<REG> {
        self.variant(Idleselect::Cpu)
    }
    #[doc = "The CPU and AHB clock domains are stopped"]
    #[inline(always)]
    pub fn ahb(self) -> &'a mut crate::W<REG> {
        self.variant(Idleselect::Ahb)
    }
    #[doc = "The CPU, AHB and APB clock domains are stopped"]
    #[inline(always)]
    pub fn apb(self) -> &'a mut crate::W<REG> {
        self.variant(Idleselect::Apb)
    }
}
impl R {
    #[doc = "Bits 0:1 - Idle Mode Configuration"]
    #[inline(always)]
    pub fn idle(&self) -> IdleR {
        IdleR::new(self.bits & 3)
    }
}
impl W {
    #[doc = "Bits 0:1 - Idle Mode Configuration"]
    #[inline(always)]
    #[must_use]
    pub fn idle(&mut self) -> IdleW<SleepSpec> {
        IdleW::new(self, 0)
    }
}
#[doc = "Sleep Mode\n\nYou can [`read`](crate::Reg::read) this register and get [`sleep::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sleep::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SleepSpec;
impl crate::RegisterSpec for SleepSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`sleep::R`](R) reader structure"]
impl crate::Readable for SleepSpec {}
#[doc = "`write(|w| ..)` method takes [`sleep::W`](W) writer structure"]
impl crate::Writable for SleepSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets SLEEP to value 0"]
impl crate::Resettable for SleepSpec {
    const RESET_VALUE: u8 = 0;
}
