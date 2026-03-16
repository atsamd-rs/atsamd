#[doc = "Register `TDCR` reader"]
pub type R = crate::R<TdcrSpec>;
#[doc = "Register `TDCR` writer"]
pub type W = crate::W<TdcrSpec>;
#[doc = "Field `TDCF` reader - Transmitter Delay Compensation Filter Length"]
pub type TdcfR = crate::FieldReader;
#[doc = "Field `TDCF` writer - Transmitter Delay Compensation Filter Length"]
pub type TdcfW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `TDCO` reader - Transmitter Delay Compensation Offset"]
pub type TdcoR = crate::FieldReader;
#[doc = "Field `TDCO` writer - Transmitter Delay Compensation Offset"]
pub type TdcoW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - Transmitter Delay Compensation Filter Length"]
    #[inline(always)]
    pub fn tdcf(&self) -> TdcfR {
        TdcfR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - Transmitter Delay Compensation Offset"]
    #[inline(always)]
    pub fn tdco(&self) -> TdcoR {
        TdcoR::new(((self.bits >> 8) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Transmitter Delay Compensation Filter Length"]
    #[inline(always)]
    #[must_use]
    pub fn tdcf(&mut self) -> TdcfW<TdcrSpec> {
        TdcfW::new(self, 0)
    }
    #[doc = "Bits 8:14 - Transmitter Delay Compensation Offset"]
    #[inline(always)]
    #[must_use]
    pub fn tdco(&mut self) -> TdcoW<TdcrSpec> {
        TdcoW::new(self, 8)
    }
}
#[doc = "Extended ID Filter Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`tdcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tdcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TdcrSpec;
impl crate::RegisterSpec for TdcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tdcr::R`](R) reader structure"]
impl crate::Readable for TdcrSpec {}
#[doc = "`write(|w| ..)` method takes [`tdcr::W`](W) writer structure"]
impl crate::Writable for TdcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TDCR to value 0"]
impl crate::Resettable for TdcrSpec {
    const RESET_VALUE: u32 = 0;
}
