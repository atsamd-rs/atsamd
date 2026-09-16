#[doc = "Register `MRCFG` reader"]
pub type R = crate::R<MrcfgSpec>;
#[doc = "Register `MRCFG` writer"]
pub type W = crate::W<MrcfgSpec>;
#[doc = "Quality of Service\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Qosselect {
    #[doc = "0: Background (no sensitive operation)"]
    Disable = 0,
    #[doc = "1: Sensitive Bandwidth"]
    Low = 1,
    #[doc = "2: Sensitive Latency"]
    Medium = 2,
    #[doc = "3: Critical Latency"]
    High = 3,
}
impl From<Qosselect> for u8 {
    #[inline(always)]
    fn from(variant: Qosselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Qosselect {
    type Ux = u8;
}
impl crate::IsEnum for Qosselect {}
#[doc = "Field `QOS` reader - Quality of Service"]
pub type QosR = crate::FieldReader<Qosselect>;
impl QosR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Qosselect {
        match self.bits {
            0 => Qosselect::Disable,
            1 => Qosselect::Low,
            2 => Qosselect::Medium,
            3 => Qosselect::High,
            _ => unreachable!(),
        }
    }
    #[doc = "Background (no sensitive operation)"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Qosselect::Disable
    }
    #[doc = "Sensitive Bandwidth"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Qosselect::Low
    }
    #[doc = "Sensitive Latency"]
    #[inline(always)]
    pub fn is_medium(&self) -> bool {
        *self == Qosselect::Medium
    }
    #[doc = "Critical Latency"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Qosselect::High
    }
}
#[doc = "Field `QOS` writer - Quality of Service"]
pub type QosW<'a, REG> = crate::FieldWriter<'a, REG, 2, Qosselect, crate::Safe>;
impl<'a, REG> QosW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Background (no sensitive operation)"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Qosselect::Disable)
    }
    #[doc = "Sensitive Bandwidth"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Qosselect::Low)
    }
    #[doc = "Sensitive Latency"]
    #[inline(always)]
    pub fn medium(self) -> &'a mut crate::W<REG> {
        self.variant(Qosselect::Medium)
    }
    #[doc = "Critical Latency"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Qosselect::High)
    }
}
impl R {
    #[doc = "Bits 0:1 - Quality of Service"]
    #[inline(always)]
    pub fn qos(&self) -> QosR {
        QosR::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Quality of Service"]
    #[inline(always)]
    #[must_use]
    pub fn qos(&mut self) -> QosW<MrcfgSpec> {
        QosW::new(self, 0)
    }
}
#[doc = "Message RAM Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`mrcfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mrcfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MrcfgSpec;
impl crate::RegisterSpec for MrcfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mrcfg::R`](R) reader structure"]
impl crate::Readable for MrcfgSpec {}
#[doc = "`write(|w| ..)` method takes [`mrcfg::W`](W) writer structure"]
impl crate::Writable for MrcfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MRCFG to value 0x02"]
impl crate::Resettable for MrcfgSpec {
    const RESET_VALUE: u32 = 0x02;
}
