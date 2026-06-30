#[doc = "Register `GAINCORR` reader"]
pub type R = crate::R<GaincorrSpec>;
#[doc = "Register `GAINCORR` writer"]
pub type W = crate::W<GaincorrSpec>;
#[doc = "Field `GAINCORR` reader - Gain Correction Value"]
pub type GaincorrR = crate::FieldReader<u16>;
#[doc = "Field `GAINCORR` writer - Gain Correction Value"]
pub type GaincorrW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - Gain Correction Value"]
    #[inline(always)]
    pub fn gaincorr(&self) -> GaincorrR {
        GaincorrR::new(self.bits & 0x0fff)
    }
}
impl W {
    #[doc = "Bits 0:11 - Gain Correction Value"]
    #[inline(always)]
    #[must_use]
    pub fn gaincorr(&mut self) -> GaincorrW<GaincorrSpec> {
        GaincorrW::new(self, 0)
    }
}
#[doc = "Gain Correction\n\nYou can [`read`](crate::Reg::read) this register and get [`gaincorr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gaincorr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GaincorrSpec;
impl crate::RegisterSpec for GaincorrSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`gaincorr::R`](R) reader structure"]
impl crate::Readable for GaincorrSpec {}
#[doc = "`write(|w| ..)` method takes [`gaincorr::W`](W) writer structure"]
impl crate::Writable for GaincorrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets GAINCORR to value 0"]
impl crate::Resettable for GaincorrSpec {
    const RESET_VALUE: u16 = 0;
}
