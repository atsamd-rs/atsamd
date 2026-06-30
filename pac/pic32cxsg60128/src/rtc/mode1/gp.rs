#[doc = "Register `GP[%s]` reader"]
pub type R = crate::R<GpSpec>;
#[doc = "Register `GP[%s]` writer"]
pub type W = crate::W<GpSpec>;
#[doc = "Field `GP` reader - General Purpose"]
pub type GpR = crate::FieldReader<u32>;
#[doc = "Field `GP` writer - General Purpose"]
pub type GpW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - General Purpose"]
    #[inline(always)]
    pub fn gp(&self) -> GpR {
        GpR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - General Purpose"]
    #[inline(always)]
    #[must_use]
    pub fn gp(&mut self) -> GpW<GpSpec> {
        GpW::new(self, 0)
    }
}
#[doc = "General Purpose\n\nYou can [`read`](crate::Reg::read) this register and get [`gp::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gp::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpSpec;
impl crate::RegisterSpec for GpSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gp::R`](R) reader structure"]
impl crate::Readable for GpSpec {}
#[doc = "`write(|w| ..)` method takes [`gp::W`](W) writer structure"]
impl crate::Writable for GpSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GP[%s]
to value 0"]
impl crate::Resettable for GpSpec {
    const RESET_VALUE: u32 = 0;
}
