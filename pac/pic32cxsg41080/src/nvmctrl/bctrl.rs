#[doc = "Register `BCTRL` reader"]
pub type R = crate::R<BctrlSpec>;
#[doc = "Register `BCTRL` writer"]
pub type W = crate::W<BctrlSpec>;
#[doc = "Field `BRPE` reader - Boot Read Protection Enable"]
pub type BrpeR = crate::BitReader;
#[doc = "Field `BRPE` writer - Boot Read Protection Enable"]
pub type BrpeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Boot Read Protection Enable"]
    #[inline(always)]
    pub fn brpe(&self) -> BrpeR {
        BrpeR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Boot Read Protection Enable"]
    #[inline(always)]
    #[must_use]
    pub fn brpe(&mut self) -> BrpeW<BctrlSpec> {
        BrpeW::new(self, 0)
    }
}
#[doc = "Boot Control\n\nYou can [`read`](crate::Reg::read) this register and get [`bctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BctrlSpec;
impl crate::RegisterSpec for BctrlSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`bctrl::R`](R) reader structure"]
impl crate::Readable for BctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`bctrl::W`](W) writer structure"]
impl crate::Writable for BctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets BCTRL to value 0"]
impl crate::Resettable for BctrlSpec {
    const RESET_VALUE: u8 = 0;
}
