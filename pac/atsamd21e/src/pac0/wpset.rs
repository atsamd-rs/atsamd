#[doc = "Register `WPSET` reader"]
pub type R = crate::R<WpsetSpec>;
#[doc = "Register `WPSET` writer"]
pub type W = crate::W<WpsetSpec>;
#[doc = "Field `WP` reader - Write Protection Set"]
pub type WpR = crate::FieldReader<u32>;
#[doc = "Field `WP` writer - Write Protection Set"]
pub type WpW<'a, REG> = crate::FieldWriter<'a, REG, 31, u32>;
impl R {
    #[doc = "Bits 1:31 - Write Protection Set"]
    #[inline(always)]
    pub fn wp(&self) -> WpR {
        WpR::new((self.bits >> 1) & 0x7fff_ffff)
    }
}
impl W {
    #[doc = "Bits 1:31 - Write Protection Set"]
    #[inline(always)]
    #[must_use]
    pub fn wp(&mut self) -> WpW<WpsetSpec> {
        WpW::new(self, 1)
    }
}
#[doc = "Write Protection Set\n\nYou can [`read`](crate::Reg::read) this register and get [`wpset::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wpset::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WpsetSpec;
impl crate::RegisterSpec for WpsetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wpset::R`](R) reader structure"]
impl crate::Readable for WpsetSpec {}
#[doc = "`write(|w| ..)` method takes [`wpset::W`](W) writer structure"]
impl crate::Writable for WpsetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WPSET to value 0"]
impl crate::Resettable for WpsetSpec {
    const RESET_VALUE: u32 = 0;
}
