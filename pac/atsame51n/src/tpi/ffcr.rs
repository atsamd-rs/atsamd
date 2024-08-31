#[doc = "Register `FFCR` reader"]
pub type R = crate::R<FfcrSpec>;
#[doc = "Register `FFCR` writer"]
pub type W = crate::W<FfcrSpec>;
#[doc = "Field `EnFCont` reader - "]
pub type EnFcontR = crate::BitReader;
#[doc = "Field `EnFCont` writer - "]
pub type EnFcontW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TrigIn` reader - "]
pub type TrigInR = crate::BitReader;
#[doc = "Field `TrigIn` writer - "]
pub type TrigInW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn en_fcont(&self) -> EnFcontR {
        EnFcontR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn trig_in(&self) -> TrigInR {
        TrigInR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn en_fcont(&mut self) -> EnFcontW<FfcrSpec> {
        EnFcontW::new(self, 1)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn trig_in(&mut self) -> TrigInW<FfcrSpec> {
        TrigInW::new(self, 8)
    }
}
#[doc = "Formatter and Flush Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ffcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ffcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FfcrSpec;
impl crate::RegisterSpec for FfcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ffcr::R`](R) reader structure"]
impl crate::Readable for FfcrSpec {}
#[doc = "`write(|w| ..)` method takes [`ffcr::W`](W) writer structure"]
impl crate::Writable for FfcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FFCR to value 0"]
impl crate::Resettable for FfcrSpec {
    const RESET_VALUE: u32 = 0;
}
