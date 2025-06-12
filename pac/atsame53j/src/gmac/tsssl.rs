#[doc = "Register `TSSSL` reader"]
pub type R = crate::R<TssslSpec>;
#[doc = "Register `TSSSL` writer"]
pub type W = crate::W<TssslSpec>;
#[doc = "Field `VTS` reader - Value of Timer Seconds Register Capture"]
pub type VtsR = crate::FieldReader<u32>;
#[doc = "Field `VTS` writer - Value of Timer Seconds Register Capture"]
pub type VtsW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Value of Timer Seconds Register Capture"]
    #[inline(always)]
    pub fn vts(&self) -> VtsR {
        VtsR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Value of Timer Seconds Register Capture"]
    #[inline(always)]
    pub fn vts(&mut self) -> VtsW<TssslSpec> {
        VtsW::new(self, 0)
    }
}
#[doc = "1588 Timer Sync Strobe Seconds \\[31:0\\] Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tsssl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tsssl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TssslSpec;
impl crate::RegisterSpec for TssslSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tsssl::R`](R) reader structure"]
impl crate::Readable for TssslSpec {}
#[doc = "`write(|w| ..)` method takes [`tsssl::W`](W) writer structure"]
impl crate::Writable for TssslSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TSSSL to value 0"]
impl crate::Resettable for TssslSpec {}
