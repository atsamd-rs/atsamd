#[doc = "Register `OUTTGL` reader"]
pub type R = crate::R<OuttglSpec>;
#[doc = "Register `OUTTGL` writer"]
pub type W = crate::W<OuttglSpec>;
#[doc = "Field `OUTTGL` reader - PORT Data Output Value Toggle"]
pub type OuttglR = crate::FieldReader<u32>;
#[doc = "Field `OUTTGL` writer - PORT Data Output Value Toggle"]
pub type OuttglW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - PORT Data Output Value Toggle"]
    #[inline(always)]
    pub fn outtgl(&self) -> OuttglR {
        OuttglR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - PORT Data Output Value Toggle"]
    #[inline(always)]
    pub fn outtgl(&mut self) -> OuttglW<OuttglSpec> {
        OuttglW::new(self, 0)
    }
}
#[doc = "Data Output Value Toggle\n\nYou can [`read`](crate::Reg::read) this register and get [`outtgl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`outtgl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OuttglSpec;
impl crate::RegisterSpec for OuttglSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`outtgl::R`](R) reader structure"]
impl crate::Readable for OuttglSpec {}
#[doc = "`write(|w| ..)` method takes [`outtgl::W`](W) writer structure"]
impl crate::Writable for OuttglSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OUTTGL to value 0"]
impl crate::Resettable for OuttglSpec {}
