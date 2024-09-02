#[doc = "Register `WPCLR` reader"]
pub type R = crate::R<WpclrSpec>;
#[doc = "Register `WPCLR` writer"]
pub type W = crate::W<WpclrSpec>;
#[doc = "Field `WP` reader - Write Protection Clear"]
pub type WpR = crate::FieldReader<u32>;
#[doc = "Field `WP` writer - Write Protection Clear"]
pub type WpW<'a, REG> = crate::FieldWriter<'a, REG, 31, u32>;
impl R {
    #[doc = "Bits 1:31 - Write Protection Clear"]
    #[inline(always)]
    pub fn wp(&self) -> WpR {
        WpR::new((self.bits >> 1) & 0x7fff_ffff)
    }
}
impl W {
    #[doc = "Bits 1:31 - Write Protection Clear"]
    #[inline(always)]
    #[must_use]
    pub fn wp(&mut self) -> WpW<WpclrSpec> {
        WpW::new(self, 1)
    }
}
#[doc = "Write Protection Clear\n\nYou can [`read`](crate::Reg::read) this register and get [`wpclr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wpclr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WpclrSpec;
impl crate::RegisterSpec for WpclrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wpclr::R`](R) reader structure"]
impl crate::Readable for WpclrSpec {}
#[doc = "`write(|w| ..)` method takes [`wpclr::W`](W) writer structure"]
impl crate::Writable for WpclrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WPCLR to value 0"]
impl crate::Resettable for WpclrSpec {
    const RESET_VALUE: u32 = 0;
}
