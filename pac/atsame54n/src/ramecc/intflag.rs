#[doc = "Register `INTFLAG` reader"]
pub type R = crate::R<IntflagSpec>;
#[doc = "Register `INTFLAG` writer"]
pub type W = crate::W<IntflagSpec>;
#[doc = "Field `SINGLEE` reader - Single Bit ECC Error Interrupt"]
pub type SingleeR = crate::BitReader;
#[doc = "Field `SINGLEE` writer - Single Bit ECC Error Interrupt"]
pub type SingleeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DUALE` reader - Dual Bit ECC Error Interrupt"]
pub type DualeR = crate::BitReader;
#[doc = "Field `DUALE` writer - Dual Bit ECC Error Interrupt"]
pub type DualeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Single Bit ECC Error Interrupt"]
    #[inline(always)]
    pub fn singlee(&self) -> SingleeR {
        SingleeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Dual Bit ECC Error Interrupt"]
    #[inline(always)]
    pub fn duale(&self) -> DualeR {
        DualeR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Single Bit ECC Error Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn singlee(&mut self) -> SingleeW<IntflagSpec> {
        SingleeW::new(self, 0)
    }
    #[doc = "Bit 1 - Dual Bit ECC Error Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn duale(&mut self) -> DualeW<IntflagSpec> {
        DualeW::new(self, 1)
    }
}
#[doc = "Interrupt Flag\n\nYou can [`read`](crate::Reg::read) this register and get [`intflag::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intflag::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntflagSpec;
impl crate::RegisterSpec for IntflagSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`intflag::R`](R) reader structure"]
impl crate::Readable for IntflagSpec {}
#[doc = "`write(|w| ..)` method takes [`intflag::W`](W) writer structure"]
impl crate::Writable for IntflagSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets INTFLAG to value 0"]
impl crate::Resettable for IntflagSpec {
    const RESET_VALUE: u8 = 0;
}
