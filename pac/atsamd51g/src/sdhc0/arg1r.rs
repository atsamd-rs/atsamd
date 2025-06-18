#[doc = "Register `ARG1R` reader"]
pub type R = crate::R<Arg1rSpec>;
#[doc = "Register `ARG1R` writer"]
pub type W = crate::W<Arg1rSpec>;
#[doc = "Field `ARG` reader - Argument 1"]
pub type ArgR = crate::FieldReader<u32>;
#[doc = "Field `ARG` writer - Argument 1"]
pub type ArgW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Argument 1"]
    #[inline(always)]
    pub fn arg(&self) -> ArgR {
        ArgR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Argument 1"]
    #[inline(always)]
    pub fn arg(&mut self) -> ArgW<Arg1rSpec> {
        ArgW::new(self, 0)
    }
}
#[doc = "Argument 1\n\nYou can [`read`](crate::Reg::read) this register and get [`arg1r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`arg1r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Arg1rSpec;
impl crate::RegisterSpec for Arg1rSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`arg1r::R`](R) reader structure"]
impl crate::Readable for Arg1rSpec {}
#[doc = "`write(|w| ..)` method takes [`arg1r::W`](W) writer structure"]
impl crate::Writable for Arg1rSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ARG1R to value 0"]
impl crate::Resettable for Arg1rSpec {}
