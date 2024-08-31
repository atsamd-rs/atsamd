#[doc = "Register `CHID` reader"]
pub type R = crate::R<ChidSpec>;
#[doc = "Register `CHID` writer"]
pub type W = crate::W<ChidSpec>;
#[doc = "Field `ID` reader - Channel ID"]
pub type IdR = crate::FieldReader;
#[doc = "Field `ID` writer - Channel ID"]
pub type IdW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Channel ID"]
    #[inline(always)]
    pub fn id(&self) -> IdR {
        IdR::new(self.bits & 0x0f)
    }
}
impl W {
    #[doc = "Bits 0:3 - Channel ID"]
    #[inline(always)]
    #[must_use]
    pub fn id(&mut self) -> IdW<ChidSpec> {
        IdW::new(self, 0)
    }
}
#[doc = "Channel ID\n\nYou can [`read`](crate::Reg::read) this register and get [`chid::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chid::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ChidSpec;
impl crate::RegisterSpec for ChidSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`chid::R`](R) reader structure"]
impl crate::Readable for ChidSpec {}
#[doc = "`write(|w| ..)` method takes [`chid::W`](W) writer structure"]
impl crate::Writable for ChidSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets CHID to value 0"]
impl crate::Resettable for ChidSpec {
    const RESET_VALUE: u8 = 0;
}
