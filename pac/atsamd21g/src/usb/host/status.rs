#[doc = "Register `STATUS` reader"]
pub type R = crate::R<StatusSpec>;
#[doc = "Register `STATUS` writer"]
pub type W = crate::W<StatusSpec>;
#[doc = "Field `SPEED` reader - Speed Status"]
pub type SpeedR = crate::FieldReader;
#[doc = "Field `SPEED` writer - Speed Status"]
pub type SpeedW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `LINESTATE` reader - USB Line State Status"]
pub type LinestateR = crate::FieldReader;
impl R {
    #[doc = "Bits 2:3 - Speed Status"]
    #[inline(always)]
    pub fn speed(&self) -> SpeedR {
        SpeedR::new((self.bits >> 2) & 3)
    }
    #[doc = "Bits 6:7 - USB Line State Status"]
    #[inline(always)]
    pub fn linestate(&self) -> LinestateR {
        LinestateR::new((self.bits >> 6) & 3)
    }
}
impl W {
    #[doc = "Bits 2:3 - Speed Status"]
    #[inline(always)]
    pub fn speed(&mut self) -> SpeedW<StatusSpec> {
        SpeedW::new(self, 2)
    }
}
#[doc = "HOST Status\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatusSpec;
impl crate::RegisterSpec for StatusSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for StatusSpec {}
#[doc = "`write(|w| ..)` method takes [`status::W`](W) writer structure"]
impl crate::Writable for StatusSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for StatusSpec {}
