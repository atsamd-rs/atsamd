#[doc = "Register `QOSCTRL` reader"]
pub type R = crate::R<QosctrlSpec>;
#[doc = "Register `QOSCTRL` writer"]
pub type W = crate::W<QosctrlSpec>;
#[doc = "Field `CQOS` reader - Configuration Quality of Service"]
pub type CqosR = crate::FieldReader;
#[doc = "Field `CQOS` writer - Configuration Quality of Service"]
pub type CqosW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DQOS` reader - Data Quality of Service"]
pub type DqosR = crate::FieldReader;
#[doc = "Field `DQOS` writer - Data Quality of Service"]
pub type DqosW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
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
#[doc = "`reset()` method sets QOSCTRL to value 0x0f"]
impl crate::Resettable for QosctrlSpec {
    const RESET_VALUE: u8 = 0x0f;
}
