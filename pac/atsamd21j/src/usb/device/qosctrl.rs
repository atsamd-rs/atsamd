#[doc = "Register `QOSCTRL` reader"]
pub type R = crate::R<QosctrlSpec>;
#[doc = "Register `QOSCTRL` writer"]
pub type W = crate::W<QosctrlSpec>;
#[doc = "Configuration Quality of Service\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cqosselect {
    #[doc = "0: Background (no sensitive operation)"]
    Disable = 0,
    #[doc = "1: Sensitive Bandwidth"]
    Low = 1,
    #[doc = "2: Sensitive Latency"]
    Medium = 2,
    #[doc = "3: Critical Latency"]
    High = 3,
}
impl From<Cqosselect> for u8 {
    #[inline(always)]
    fn from(variant: Cqosselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cqosselect {
    type Ux = u8;
}
impl crate::IsEnum for Cqosselect {}
#[doc = "Field `CQOS` reader - Configuration Quality of Service"]
pub type CqosR = crate::FieldReader<Cqosselect>;
impl CqosR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cqosselect {
        match self.bits {
            0 => Cqosselect::Disable,
            1 => Cqosselect::Low,
            2 => Cqosselect::Medium,
            3 => Cqosselect::High,
            _ => unreachable!(),
        }
    }
    #[doc = "Background (no sensitive operation)"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Cqosselect::Disable
    }
    #[doc = "Sensitive Bandwidth"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Cqosselect::Low
    }
    #[doc = "Sensitive Latency"]
    #[inline(always)]
    pub fn is_medium(&self) -> bool {
        *self == Cqosselect::Medium
    }
    #[doc = "Critical Latency"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Cqosselect::High
    }
}
#[doc = "Field `CQOS` writer - Configuration Quality of Service"]
pub type CqosW<'a, REG> = crate::FieldWriter<'a, REG, 2, Cqosselect, crate::Safe>;
impl<'a, REG> CqosW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Background (no sensitive operation)"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Cqosselect::Disable)
    }
    #[doc = "Sensitive Bandwidth"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Cqosselect::Low)
    }
    #[doc = "Sensitive Latency"]
    #[inline(always)]
    pub fn medium(self) -> &'a mut crate::W<REG> {
        self.variant(Cqosselect::Medium)
    }
    #[doc = "Critical Latency"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Cqosselect::High)
    }
}
#[doc = "Data Quality of Service\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dqosselect {
    #[doc = "0: Background (no sensitive operation)"]
    Disable = 0,
    #[doc = "1: Sensitive Bandwidth"]
    Low = 1,
    #[doc = "2: Sensitive Latency"]
    Medium = 2,
    #[doc = "3: Critical Latency"]
    High = 3,
}
impl From<Dqosselect> for u8 {
    #[inline(always)]
    fn from(variant: Dqosselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dqosselect {
    type Ux = u8;
}
impl crate::IsEnum for Dqosselect {}
#[doc = "Field `DQOS` reader - Data Quality of Service"]
pub type DqosR = crate::FieldReader<Dqosselect>;
impl DqosR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dqosselect {
        match self.bits {
            0 => Dqosselect::Disable,
            1 => Dqosselect::Low,
            2 => Dqosselect::Medium,
            3 => Dqosselect::High,
            _ => unreachable!(),
        }
    }
    #[doc = "Background (no sensitive operation)"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Dqosselect::Disable
    }
    #[doc = "Sensitive Bandwidth"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Dqosselect::Low
    }
    #[doc = "Sensitive Latency"]
    #[inline(always)]
    pub fn is_medium(&self) -> bool {
        *self == Dqosselect::Medium
    }
    #[doc = "Critical Latency"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Dqosselect::High
    }
}
#[doc = "Field `DQOS` writer - Data Quality of Service"]
pub type DqosW<'a, REG> = crate::FieldWriter<'a, REG, 2, Dqosselect, crate::Safe>;
impl<'a, REG> DqosW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Background (no sensitive operation)"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Dqosselect::Disable)
    }
    #[doc = "Sensitive Bandwidth"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Dqosselect::Low)
    }
    #[doc = "Sensitive Latency"]
    #[inline(always)]
    pub fn medium(self) -> &'a mut crate::W<REG> {
        self.variant(Dqosselect::Medium)
    }
    #[doc = "Critical Latency"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Dqosselect::High)
    }
}
impl R {
    #[doc = "Bits 0:1 - Configuration Quality of Service"]
    #[inline(always)]
    pub fn cqos(&self) -> CqosR {
        CqosR::new(self.bits & 3)
    }
    #[doc = "Bits 2:3 - Data Quality of Service"]
    #[inline(always)]
    pub fn dqos(&self) -> DqosR {
        DqosR::new((self.bits >> 2) & 3)
    }
}
impl W {
    #[doc = "Bits 0:1 - Configuration Quality of Service"]
    #[inline(always)]
    #[must_use]
    pub fn cqos(&mut self) -> CqosW<QosctrlSpec> {
        CqosW::new(self, 0)
    }
    #[doc = "Bits 2:3 - Data Quality of Service"]
    #[inline(always)]
    #[must_use]
    pub fn dqos(&mut self) -> DqosW<QosctrlSpec> {
        DqosW::new(self, 2)
    }
}
#[doc = "USB Quality Of Service\n\nYou can [`read`](crate::Reg::read) this register and get [`qosctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qosctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct QosctrlSpec;
impl crate::RegisterSpec for QosctrlSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`qosctrl::R`](R) reader structure"]
impl crate::Readable for QosctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`qosctrl::W`](W) writer structure"]
impl crate::Writable for QosctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets QOSCTRL to value 0x05"]
impl crate::Resettable for QosctrlSpec {
    const RESET_VALUE: u8 = 0x05;
}
