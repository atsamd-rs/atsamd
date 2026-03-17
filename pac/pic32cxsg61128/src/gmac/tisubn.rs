#[doc = "Register `TISUBN` reader"]
pub type R = crate::R<TisubnSpec>;
#[doc = "Register `TISUBN` writer"]
pub type W = crate::W<TisubnSpec>;
#[doc = "Field `LSBTIR` reader - Lower Significant Bits of Timer Increment"]
pub type LsbtirR = crate::FieldReader<u16>;
#[doc = "Field `LSBTIR` writer - Lower Significant Bits of Timer Increment"]
pub type LsbtirW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Lower Significant Bits of Timer Increment"]
    #[inline(always)]
    pub fn lsbtir(&self) -> LsbtirR {
        LsbtirR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Lower Significant Bits of Timer Increment"]
    #[inline(always)]
    #[must_use]
    pub fn lsbtir(&mut self) -> LsbtirW<TisubnSpec> {
        LsbtirW::new(self, 0)
    }
}
#[doc = "1588 Timer Increment \\[15:0\\]
Sub-Nanoseconds Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tisubn::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tisubn::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TisubnSpec;
impl crate::RegisterSpec for TisubnSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tisubn::R`](R) reader structure"]
impl crate::Readable for TisubnSpec {}
#[doc = "`write(|w| ..)` method takes [`tisubn::W`](W) writer structure"]
impl crate::Writable for TisubnSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TISUBN to value 0"]
impl crate::Resettable for TisubnSpec {
    const RESET_VALUE: u32 = 0;
}
