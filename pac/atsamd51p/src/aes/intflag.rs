#[doc = "Register `INTFLAG` reader"]
pub type R = crate::R<IntflagSpec>;
#[doc = "Register `INTFLAG` writer"]
pub type W = crate::W<IntflagSpec>;
#[doc = "Field `ENCCMP` reader - Encryption Complete"]
pub type EnccmpR = crate::BitReader;
#[doc = "Field `ENCCMP` writer - Encryption Complete"]
pub type EnccmpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GFMCMP` reader - GF Multiplication Complete"]
pub type GfmcmpR = crate::BitReader;
#[doc = "Field `GFMCMP` writer - GF Multiplication Complete"]
pub type GfmcmpW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Encryption Complete"]
    #[inline(always)]
    pub fn enccmp(&self) -> EnccmpR {
        EnccmpR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - GF Multiplication Complete"]
    #[inline(always)]
    pub fn gfmcmp(&self) -> GfmcmpR {
        GfmcmpR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Encryption Complete"]
    #[inline(always)]
    pub fn enccmp(&mut self) -> EnccmpW<IntflagSpec> {
        EnccmpW::new(self, 0)
    }
    #[doc = "Bit 1 - GF Multiplication Complete"]
    #[inline(always)]
    pub fn gfmcmp(&mut self) -> GfmcmpW<IntflagSpec> {
        GfmcmpW::new(self, 1)
    }
}
#[doc = "Interrupt Flag Status\n\nYou can [`read`](crate::Reg::read) this register and get [`intflag::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intflag::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntflagSpec;
impl crate::RegisterSpec for IntflagSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`intflag::R`](R) reader structure"]
impl crate::Readable for IntflagSpec {}
#[doc = "`write(|w| ..)` method takes [`intflag::W`](W) writer structure"]
impl crate::Writable for IntflagSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INTFLAG to value 0"]
impl crate::Resettable for IntflagSpec {}
