#[doc = "Register `QOSCTRL` reader"]
pub type R = crate::R<QosctrlSpec>;
#[doc = "Register `QOSCTRL` writer"]
pub type W = crate::W<QosctrlSpec>;
#[doc = "Write-Back Quality of Service\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Wrbqosselect {
    #[doc = "0: Background (no sensitive operation)"]
    Disable = 0,
    #[doc = "1: Sensitive Bandwidth"]
    Low = 1,
    #[doc = "2: Sensitive Latency"]
    Medium = 2,
    #[doc = "3: Critical Latency"]
    High = 3,
}
impl From<Wrbqosselect> for u8 {
    #[inline(always)]
    fn from(variant: Wrbqosselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Wrbqosselect {
    type Ux = u8;
}
impl crate::IsEnum for Wrbqosselect {}
#[doc = "Field `WRBQOS` reader - Write-Back Quality of Service"]
pub type WrbqosR = crate::FieldReader<Wrbqosselect>;
impl WrbqosR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wrbqosselect {
        match self.bits {
            0 => Wrbqosselect::Disable,
            1 => Wrbqosselect::Low,
            2 => Wrbqosselect::Medium,
            3 => Wrbqosselect::High,
            _ => unreachable!(),
        }
    }
    #[doc = "Background (no sensitive operation)"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Wrbqosselect::Disable
    }
    #[doc = "Sensitive Bandwidth"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Wrbqosselect::Low
    }
    #[doc = "Sensitive Latency"]
    #[inline(always)]
    pub fn is_medium(&self) -> bool {
        *self == Wrbqosselect::Medium
    }
    #[doc = "Critical Latency"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Wrbqosselect::High
    }
}
#[doc = "Field `WRBQOS` writer - Write-Back Quality of Service"]
pub type WrbqosW<'a, REG> = crate::FieldWriter<'a, REG, 2, Wrbqosselect, crate::Safe>;
impl<'a, REG> WrbqosW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Background (no sensitive operation)"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Wrbqosselect::Disable)
    }
    #[doc = "Sensitive Bandwidth"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Wrbqosselect::Low)
    }
    #[doc = "Sensitive Latency"]
    #[inline(always)]
    pub fn medium(self) -> &'a mut crate::W<REG> {
        self.variant(Wrbqosselect::Medium)
    }
    #[doc = "Critical Latency"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Wrbqosselect::High)
    }
}
#[doc = "Fetch Quality of Service\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Fqosselect {
    #[doc = "0: Background (no sensitive operation)"]
    Disable = 0,
    #[doc = "1: Sensitive Bandwidth"]
    Low = 1,
    #[doc = "2: Sensitive Latency"]
    Medium = 2,
    #[doc = "3: Critical Latency"]
    High = 3,
}
impl From<Fqosselect> for u8 {
    #[inline(always)]
    fn from(variant: Fqosselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Fqosselect {
    type Ux = u8;
}
impl crate::IsEnum for Fqosselect {}
#[doc = "Field `FQOS` reader - Fetch Quality of Service"]
pub type FqosR = crate::FieldReader<Fqosselect>;
impl FqosR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fqosselect {
        match self.bits {
            0 => Fqosselect::Disable,
            1 => Fqosselect::Low,
            2 => Fqosselect::Medium,
            3 => Fqosselect::High,
            _ => unreachable!(),
        }
    }
    #[doc = "Background (no sensitive operation)"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Fqosselect::Disable
    }
    #[doc = "Sensitive Bandwidth"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Fqosselect::Low
    }
    #[doc = "Sensitive Latency"]
    #[inline(always)]
    pub fn is_medium(&self) -> bool {
        *self == Fqosselect::Medium
    }
    #[doc = "Critical Latency"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Fqosselect::High
    }
}
#[doc = "Field `FQOS` writer - Fetch Quality of Service"]
pub type FqosW<'a, REG> = crate::FieldWriter<'a, REG, 2, Fqosselect, crate::Safe>;
impl<'a, REG> FqosW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Background (no sensitive operation)"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Fqosselect::Disable)
    }
    #[doc = "Sensitive Bandwidth"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Fqosselect::Low)
    }
    #[doc = "Sensitive Latency"]
    #[inline(always)]
    pub fn medium(self) -> &'a mut crate::W<REG> {
        self.variant(Fqosselect::Medium)
    }
    #[doc = "Critical Latency"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Fqosselect::High)
    }
}
#[doc = "Data Transfer Quality of Service\n\nValue on reset: 1"]
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
#[doc = "Field `DQOS` reader - Data Transfer Quality of Service"]
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
#[doc = "Field `DQOS` writer - Data Transfer Quality of Service"]
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
    #[doc = "Bits 0:1 - Write-Back Quality of Service"]
    #[inline(always)]
    pub fn wrbqos(&self) -> WrbqosR {
        WrbqosR::new(self.bits & 3)
    }
    #[doc = "Bits 2:3 - Fetch Quality of Service"]
    #[inline(always)]
    pub fn fqos(&self) -> FqosR {
        FqosR::new((self.bits >> 2) & 3)
    }
    #[doc = "Bits 4:5 - Data Transfer Quality of Service"]
    #[inline(always)]
    pub fn dqos(&self) -> DqosR {
        DqosR::new((self.bits >> 4) & 3)
    }
}
impl W {
    #[doc = "Bits 0:1 - Write-Back Quality of Service"]
    #[inline(always)]
    pub fn wrbqos(&mut self) -> WrbqosW<QosctrlSpec> {
        WrbqosW::new(self, 0)
    }
    #[doc = "Bits 2:3 - Fetch Quality of Service"]
    #[inline(always)]
    pub fn fqos(&mut self) -> FqosW<QosctrlSpec> {
        FqosW::new(self, 2)
    }
    #[doc = "Bits 4:5 - Data Transfer Quality of Service"]
    #[inline(always)]
    pub fn dqos(&mut self) -> DqosW<QosctrlSpec> {
        DqosW::new(self, 4)
    }
}
#[doc = "QOS Control\n\nYou can [`read`](crate::Reg::read) this register and get [`qosctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qosctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct QosctrlSpec;
impl crate::RegisterSpec for QosctrlSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`qosctrl::R`](R) reader structure"]
impl crate::Readable for QosctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`qosctrl::W`](W) writer structure"]
impl crate::Writable for QosctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets QOSCTRL to value 0x15"]
impl crate::Resettable for QosctrlSpec {
    const RESET_VALUE: u8 = 0x15;
}
