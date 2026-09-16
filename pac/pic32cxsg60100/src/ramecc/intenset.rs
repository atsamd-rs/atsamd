#[doc = "Register `INTENSET` reader"]
pub type R = crate::R<IntensetSpec>;
#[doc = "Register `INTENSET` writer"]
pub type W = crate::W<IntensetSpec>;
#[doc = "Field `SINGLEE` reader - Single Bit ECC Error Interrupt Enable Set"]
pub type SingleeR = crate::BitReader;
#[doc = "Field `SINGLEE` writer - Single Bit ECC Error Interrupt Enable Set"]
pub type SingleeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DUALE` reader - Dual Bit ECC Error Interrupt Enable Set"]
pub type DualeR = crate::BitReader;
#[doc = "Field `DUALE` writer - Dual Bit ECC Error Interrupt Enable Set"]
pub type DualeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Single Bit ECC Error Interrupt Enable Set"]
    #[inline(always)]
    pub fn singlee(&self) -> SingleeR {
        SingleeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Dual Bit ECC Error Interrupt Enable Set"]
    #[inline(always)]
    pub fn duale(&self) -> DualeR {
        DualeR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Single Bit ECC Error Interrupt Enable Set"]
    #[inline(always)]
    #[must_use]
    pub fn singlee(&mut self) -> SingleeW<IntensetSpec> {
        SingleeW::new(self, 0)
    }
    #[doc = "Bit 1 - Dual Bit ECC Error Interrupt Enable Set"]
    #[inline(always)]
    #[must_use]
    pub fn duale(&mut self) -> DualeW<IntensetSpec> {
        DualeW::new(self, 1)
    }
}
#[doc = "Interrupt Enable Set\n\nYou can [`read`](crate::Reg::read) this register and get [`intenset::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenset::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntensetSpec;
impl crate::RegisterSpec for IntensetSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`intenset::R`](R) reader structure"]
impl crate::Readable for IntensetSpec {}
#[doc = "`write(|w| ..)` method takes [`intenset::W`](W) writer structure"]
impl crate::Writable for IntensetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets INTENSET to value 0"]
impl crate::Resettable for IntensetSpec {
    const RESET_VALUE: u8 = 0;
}
