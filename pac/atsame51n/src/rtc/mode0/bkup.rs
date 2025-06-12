#[doc = "Register `BKUP[%s]` reader"]
pub type R = crate::R<BkupSpec>;
#[doc = "Register `BKUP[%s]` writer"]
pub type W = crate::W<BkupSpec>;
#[doc = "Field `BKUP` reader - Backup"]
pub type BkupR = crate::FieldReader<u32>;
#[doc = "Field `BKUP` writer - Backup"]
pub type BkupW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Backup"]
    #[inline(always)]
    pub fn bkup(&self) -> BkupR {
        BkupR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Backup"]
    #[inline(always)]
    pub fn bkup(&mut self) -> BkupW<BkupSpec> {
        BkupW::new(self, 0)
    }
}
#[doc = "Backup\n\nYou can [`read`](crate::Reg::read) this register and get [`bkup::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bkup::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BkupSpec;
impl crate::RegisterSpec for BkupSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bkup::R`](R) reader structure"]
impl crate::Readable for BkupSpec {}
#[doc = "`write(|w| ..)` method takes [`bkup::W`](W) writer structure"]
impl crate::Writable for BkupSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BKUP[%s] to value 0"]
impl crate::Resettable for BkupSpec {}
