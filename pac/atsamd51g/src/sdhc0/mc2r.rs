#[doc = "Register `MC2R` writer"]
pub type W = crate::W<Mc2rSpec>;
#[doc = "Field `SRESP` writer - e.MMC Abort Wait IRQ"]
pub type SrespW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ABOOT` writer - e.MMC Abort Boot"]
pub type AbootW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - e.MMC Abort Wait IRQ"]
    #[inline(always)]
    pub fn sresp(&mut self) -> SrespW<Mc2rSpec> {
        SrespW::new(self, 0)
    }
    #[doc = "Bit 1 - e.MMC Abort Boot"]
    #[inline(always)]
    pub fn aboot(&mut self) -> AbootW<Mc2rSpec> {
        AbootW::new(self, 1)
    }
}
#[doc = "MMC Control 2\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mc2r::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Mc2rSpec;
impl crate::RegisterSpec for Mc2rSpec {
    type Ux = u8;
}
#[doc = "`write(|w| ..)` method takes [`mc2r::W`](W) writer structure"]
impl crate::Writable for Mc2rSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MC2R to value 0"]
impl crate::Resettable for Mc2rSpec {}
