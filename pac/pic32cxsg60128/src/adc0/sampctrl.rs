#[doc = "Register `SAMPCTRL` reader"]
pub type R = crate::R<SampctrlSpec>;
#[doc = "Register `SAMPCTRL` writer"]
pub type W = crate::W<SampctrlSpec>;
#[doc = "Field `SAMPLEN` reader - Sampling Time Length"]
pub type SamplenR = crate::FieldReader;
#[doc = "Field `SAMPLEN` writer - Sampling Time Length"]
pub type SamplenW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `OFFCOMP` reader - Comparator Offset Compensation Enable"]
pub type OffcompR = crate::BitReader;
#[doc = "Field `OFFCOMP` writer - Comparator Offset Compensation Enable"]
pub type OffcompW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:5 - Sampling Time Length"]
    #[inline(always)]
    pub fn samplen(&self) -> SamplenR {
        SamplenR::new(self.bits & 0x3f)
    }
    #[doc = "Bit 7 - Comparator Offset Compensation Enable"]
    #[inline(always)]
    pub fn offcomp(&self) -> OffcompR {
        OffcompR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - Sampling Time Length"]
    #[inline(always)]
    #[must_use]
    pub fn samplen(&mut self) -> SamplenW<SampctrlSpec> {
        SamplenW::new(self, 0)
    }
    #[doc = "Bit 7 - Comparator Offset Compensation Enable"]
    #[inline(always)]
    #[must_use]
    pub fn offcomp(&mut self) -> OffcompW<SampctrlSpec> {
        OffcompW::new(self, 7)
    }
}
#[doc = "Sample Time Control\n\nYou can [`read`](crate::Reg::read) this register and get [`sampctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sampctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SampctrlSpec;
impl crate::RegisterSpec for SampctrlSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`sampctrl::R`](R) reader structure"]
impl crate::Readable for SampctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`sampctrl::W`](W) writer structure"]
impl crate::Writable for SampctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets SAMPCTRL to value 0"]
impl crate::Resettable for SampctrlSpec {
    const RESET_VALUE: u8 = 0;
}
