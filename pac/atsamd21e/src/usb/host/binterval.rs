#[doc = "Register `BINTERVAL%s` reader"]
pub type R = crate::R<BintervalSpec>;
#[doc = "Register `BINTERVAL%s` writer"]
pub type W = crate::W<BintervalSpec>;
#[doc = "Field `BITINTERVAL` reader - Bit Interval"]
pub type BitintervalR = crate::FieldReader;
#[doc = "Field `BITINTERVAL` writer - Bit Interval"]
pub type BitintervalW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Bit Interval"]
    #[inline(always)]
    pub fn bitinterval(&self) -> BitintervalR {
        BitintervalR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Bit Interval"]
    #[inline(always)]
    pub fn bitinterval(&mut self) -> BitintervalW<BintervalSpec> {
        BitintervalW::new(self, 0)
    }
}
#[doc = "HOST Bus Access Period of Pipe\n\nYou can [`read`](crate::Reg::read) this register and get [`binterval::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`binterval::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BintervalSpec;
impl crate::RegisterSpec for BintervalSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`binterval::R`](R) reader structure"]
impl crate::Readable for BintervalSpec {}
#[doc = "`write(|w| ..)` method takes [`binterval::W`](W) writer structure"]
impl crate::Writable for BintervalSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BINTERVAL%s to value 0"]
impl crate::Resettable for BintervalSpec {}
