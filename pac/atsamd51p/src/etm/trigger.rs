#[doc = "Register `TRIGGER` reader"]
pub type R = crate::R<TriggerSpec>;
#[doc = "Register `TRIGGER` writer"]
pub type W = crate::W<TriggerSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "ETM Trigger Event Register\n\nYou can [`read`](crate::Reg::read) this register and get [`trigger::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trigger::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TriggerSpec;
impl crate::RegisterSpec for TriggerSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`trigger::R`](R) reader structure"]
impl crate::Readable for TriggerSpec {}
#[doc = "`write(|w| ..)` method takes [`trigger::W`](W) writer structure"]
impl crate::Writable for TriggerSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TRIGGER to value 0"]
impl crate::Resettable for TriggerSpec {}
