#[doc = "Register `INTENCLR` reader"]
pub type R = crate::R<IntenclrSpec>;
#[doc = "Register `INTENCLR` writer"]
pub type W = crate::W<IntenclrSpec>;
#[doc = "Field `SINGLEE` reader - Single Bit ECC Error Interrupt Enable Clear"]
pub type SingleeR = crate::BitReader;
#[doc = "Field `SINGLEE` writer - Single Bit ECC Error Interrupt Enable Clear"]
pub type SingleeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DUALE` reader - Dual Bit ECC Error Interrupt Enable Clear"]
pub type DualeR = crate::BitReader;
#[doc = "Field `DUALE` writer - Dual Bit ECC Error Interrupt Enable Clear"]
pub type DualeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Single Bit ECC Error Interrupt Enable Clear"]
    #[inline(always)]
    pub fn singlee(&self) -> SingleeR {
        SingleeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Dual Bit ECC Error Interrupt Enable Clear"]
    #[inline(always)]
    pub fn duale(&self) -> DualeR {
        DualeR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Single Bit ECC Error Interrupt Enable Clear"]
    #[inline(always)]
    pub fn singlee(&mut self) -> SingleeW<IntenclrSpec> {
        SingleeW::new(self, 0)
    }
    #[doc = "Bit 1 - Dual Bit ECC Error Interrupt Enable Clear"]
    #[inline(always)]
    pub fn duale(&mut self) -> DualeW<IntenclrSpec> {
        DualeW::new(self, 1)
    }
}
#[doc = "Interrupt Enable Clear\n\nYou can [`read`](crate::Reg::read) this register and get [`intenclr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenclr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntenclrSpec;
impl crate::RegisterSpec for IntenclrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`intenclr::R`](R) reader structure"]
impl crate::Readable for IntenclrSpec {}
#[doc = "`write(|w| ..)` method takes [`intenclr::W`](W) writer structure"]
impl crate::Writable for IntenclrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INTENCLR to value 0"]
impl crate::Resettable for IntenclrSpec {}
