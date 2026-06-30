#[doc = "Register `FREQCORR` reader"]
pub type R = crate::R<FreqcorrSpec>;
#[doc = "Register `FREQCORR` writer"]
pub type W = crate::W<FreqcorrSpec>;
#[doc = "Field `VALUE` reader - Correction Value"]
pub type ValueR = crate::FieldReader;
#[doc = "Field `VALUE` writer - Correction Value"]
pub type ValueW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `SIGN` reader - Correction Sign"]
pub type SignR = crate::BitReader;
#[doc = "Field `SIGN` writer - Correction Sign"]
pub type SignW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:6 - Correction Value"]
    #[inline(always)]
    pub fn value(&self) -> ValueR {
        ValueR::new(self.bits & 0x7f)
    }
    #[doc = "Bit 7 - Correction Sign"]
    #[inline(always)]
    pub fn sign(&self) -> SignR {
        SignR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - Correction Value"]
    #[inline(always)]
    #[must_use]
    pub fn value(&mut self) -> ValueW<FreqcorrSpec> {
        ValueW::new(self, 0)
    }
    #[doc = "Bit 7 - Correction Sign"]
    #[inline(always)]
    #[must_use]
    pub fn sign(&mut self) -> SignW<FreqcorrSpec> {
        SignW::new(self, 7)
    }
}
#[doc = "Frequency Correction\n\nYou can [`read`](crate::Reg::read) this register and get [`freqcorr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`freqcorr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FreqcorrSpec;
impl crate::RegisterSpec for FreqcorrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`freqcorr::R`](R) reader structure"]
impl crate::Readable for FreqcorrSpec {}
#[doc = "`write(|w| ..)` method takes [`freqcorr::W`](W) writer structure"]
impl crate::Writable for FreqcorrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets FREQCORR to value 0"]
impl crate::Resettable for FreqcorrSpec {
    const RESET_VALUE: u8 = 0;
}
