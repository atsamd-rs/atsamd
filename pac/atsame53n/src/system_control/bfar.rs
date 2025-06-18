#[doc = "Register `BFAR` reader"]
pub type R = crate::R<BfarSpec>;
#[doc = "Register `BFAR` writer"]
pub type W = crate::W<BfarSpec>;
#[doc = "Field `ADDRESS` reader - Address that generated the BusFault"]
pub type AddressR = crate::FieldReader<u32>;
#[doc = "Field `ADDRESS` writer - Address that generated the BusFault"]
pub type AddressW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Address that generated the BusFault"]
    #[inline(always)]
    pub fn address(&self) -> AddressR {
        AddressR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Address that generated the BusFault"]
    #[inline(always)]
    pub fn address(&mut self) -> AddressW<BfarSpec> {
        AddressW::new(self, 0)
    }
}
#[doc = "BusFault Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`bfar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bfar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BfarSpec;
impl crate::RegisterSpec for BfarSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bfar::R`](R) reader structure"]
impl crate::Readable for BfarSpec {}
#[doc = "`write(|w| ..)` method takes [`bfar::W`](W) writer structure"]
impl crate::Writable for BfarSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BFAR to value 0"]
impl crate::Resettable for BfarSpec {}
