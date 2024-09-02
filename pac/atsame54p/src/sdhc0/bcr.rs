#[doc = "Register `BCR` reader"]
pub type R = crate::R<BcrSpec>;
#[doc = "Register `BCR` writer"]
pub type W = crate::W<BcrSpec>;
#[doc = "Field `BCNT` reader - Blocks Count for Current Transfer"]
pub type BcntR = crate::FieldReader<u16>;
#[doc = "Field `BCNT` writer - Blocks Count for Current Transfer"]
pub type BcntW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Blocks Count for Current Transfer"]
    #[inline(always)]
    pub fn bcnt(&self) -> BcntR {
        BcntR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - Blocks Count for Current Transfer"]
    #[inline(always)]
    #[must_use]
    pub fn bcnt(&mut self) -> BcntW<BcrSpec> {
        BcntW::new(self, 0)
    }
}
#[doc = "Block Count\n\nYou can [`read`](crate::Reg::read) this register and get [`bcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BcrSpec;
impl crate::RegisterSpec for BcrSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`bcr::R`](R) reader structure"]
impl crate::Readable for BcrSpec {}
#[doc = "`write(|w| ..)` method takes [`bcr::W`](W) writer structure"]
impl crate::Writable for BcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets BCR to value 0"]
impl crate::Resettable for BcrSpec {
    const RESET_VALUE: u16 = 0;
}
