#[doc = "Register `SPPR` reader"]
pub type R = crate::R<SpprSpec>;
#[doc = "Register `SPPR` writer"]
pub type W = crate::W<SpprSpec>;
#[doc = "Field `TXMODE` reader - "]
pub type TxmodeR = crate::FieldReader;
#[doc = "Field `TXMODE` writer - "]
pub type TxmodeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn txmode(&self) -> TxmodeR {
        TxmodeR::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    #[must_use]
    pub fn txmode(&mut self) -> TxmodeW<SpprSpec> {
        TxmodeW::new(self, 0)
    }
}
#[doc = "Selected Pin Protocol Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sppr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sppr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SpprSpec;
impl crate::RegisterSpec for SpprSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sppr::R`](R) reader structure"]
impl crate::Readable for SpprSpec {}
#[doc = "`write(|w| ..)` method takes [`sppr::W`](W) writer structure"]
impl crate::Writable for SpprSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SPPR to value 0"]
impl crate::Resettable for SpprSpec {
    const RESET_VALUE: u32 = 0;
}
