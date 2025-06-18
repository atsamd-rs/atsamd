#[doc = "Register `FNUM` reader"]
pub type R = crate::R<FnumSpec>;
#[doc = "Register `FNUM` writer"]
pub type W = crate::W<FnumSpec>;
#[doc = "Field `FNUM` reader - Frame Number"]
pub type FnumR = crate::FieldReader<u16>;
#[doc = "Field `FNUM` writer - Frame Number"]
pub type FnumW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bits 3:13 - Frame Number"]
    #[inline(always)]
    pub fn fnum(&self) -> FnumR {
        FnumR::new((self.bits >> 3) & 0x07ff)
    }
}
impl W {
    #[doc = "Bits 3:13 - Frame Number"]
    #[inline(always)]
    pub fn fnum(&mut self) -> FnumW<FnumSpec> {
        FnumW::new(self, 3)
    }
}
#[doc = "HOST Host Frame Number\n\nYou can [`read`](crate::Reg::read) this register and get [`fnum::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fnum::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FnumSpec;
impl crate::RegisterSpec for FnumSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`fnum::R`](R) reader structure"]
impl crate::Readable for FnumSpec {}
#[doc = "`write(|w| ..)` method takes [`fnum::W`](W) writer structure"]
impl crate::Writable for FnumSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FNUM to value 0"]
impl crate::Resettable for FnumSpec {}
