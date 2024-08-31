#[doc = "Register `WPMR` reader"]
pub type R = crate::R<WpmrSpec>;
#[doc = "Register `WPMR` writer"]
pub type W = crate::W<WpmrSpec>;
#[doc = "Field `WPEN` reader - Write Protection Enable"]
pub type WpenR = crate::BitReader;
#[doc = "Field `WPEN` writer - Write Protection Enable"]
pub type WpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WPKEY` reader - Write Protection Key"]
pub type WpkeyR = crate::FieldReader<u32>;
#[doc = "Field `WPKEY` writer - Write Protection Key"]
pub type WpkeyW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bit 0 - Write Protection Enable"]
    #[inline(always)]
    pub fn wpen(&self) -> WpenR {
        WpenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:31 - Write Protection Key"]
    #[inline(always)]
    pub fn wpkey(&self) -> WpkeyR {
        WpkeyR::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - Write Protection Enable"]
    #[inline(always)]
    #[must_use]
    pub fn wpen(&mut self) -> WpenW<WpmrSpec> {
        WpenW::new(self, 0)
    }
    #[doc = "Bits 8:31 - Write Protection Key"]
    #[inline(always)]
    #[must_use]
    pub fn wpkey(&mut self) -> WpkeyW<WpmrSpec> {
        WpkeyW::new(self, 8)
    }
}
#[doc = "Write Protection Mode Register\n\nYou can [`read`](crate::Reg::read) this register and get [`wpmr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wpmr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WpmrSpec;
impl crate::RegisterSpec for WpmrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wpmr::R`](R) reader structure"]
impl crate::Readable for WpmrSpec {}
#[doc = "`write(|w| ..)` method takes [`wpmr::W`](W) writer structure"]
impl crate::Writable for WpmrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WPMR to value 0"]
impl crate::Resettable for WpmrSpec {
    const RESET_VALUE: u32 = 0;
}
