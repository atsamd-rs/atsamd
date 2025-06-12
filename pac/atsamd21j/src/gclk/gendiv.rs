#[doc = "Register `GENDIV` reader"]
pub type R = crate::R<GendivSpec>;
#[doc = "Register `GENDIV` writer"]
pub type W = crate::W<GendivSpec>;
#[doc = "Field `ID` reader - Generic Clock Generator Selection"]
pub type IdR = crate::FieldReader;
#[doc = "Field `ID` writer - Generic Clock Generator Selection"]
pub type IdW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DIV` reader - Division Factor"]
pub type DivR = crate::FieldReader<u16>;
#[doc = "Field `DIV` writer - Division Factor"]
pub type DivW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:3 - Generic Clock Generator Selection"]
    #[inline(always)]
    pub fn id(&self) -> IdR {
        IdR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:23 - Division Factor"]
    #[inline(always)]
    pub fn div(&self) -> DivR {
        DivR::new(((self.bits >> 8) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:3 - Generic Clock Generator Selection"]
    #[inline(always)]
    pub fn id(&mut self) -> IdW<GendivSpec> {
        IdW::new(self, 0)
    }
    #[doc = "Bits 8:23 - Division Factor"]
    #[inline(always)]
    pub fn div(&mut self) -> DivW<GendivSpec> {
        DivW::new(self, 8)
    }
}
#[doc = "Generic Clock Generator Division\n\nYou can [`read`](crate::Reg::read) this register and get [`gendiv::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gendiv::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GendivSpec;
impl crate::RegisterSpec for GendivSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gendiv::R`](R) reader structure"]
impl crate::Readable for GendivSpec {}
#[doc = "`write(|w| ..)` method takes [`gendiv::W`](W) writer structure"]
impl crate::Writable for GendivSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GENDIV to value 0"]
impl crate::Resettable for GendivSpec {}
