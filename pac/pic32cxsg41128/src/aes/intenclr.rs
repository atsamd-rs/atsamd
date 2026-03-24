#[doc = "Register `INTENCLR` reader"]
pub type R = crate::R<IntenclrSpec>;
#[doc = "Register `INTENCLR` writer"]
pub type W = crate::W<IntenclrSpec>;
#[doc = "Field `ENCCMP` reader - Encryption Complete Interrupt Enable"]
pub type EnccmpR = crate::BitReader;
#[doc = "Field `ENCCMP` writer - Encryption Complete Interrupt Enable"]
pub type EnccmpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GFMCMP` reader - GF Multiplication Complete Interrupt Enable"]
pub type GfmcmpR = crate::BitReader;
#[doc = "Field `GFMCMP` writer - GF Multiplication Complete Interrupt Enable"]
pub type GfmcmpW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Encryption Complete Interrupt Enable"]
    #[inline(always)]
    pub fn enccmp(&self) -> EnccmpR {
        EnccmpR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - GF Multiplication Complete Interrupt Enable"]
    #[inline(always)]
    pub fn gfmcmp(&self) -> GfmcmpR {
        GfmcmpR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Encryption Complete Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn enccmp(&mut self) -> EnccmpW<IntenclrSpec> {
        EnccmpW::new(self, 0)
    }
    #[doc = "Bit 1 - GF Multiplication Complete Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn gfmcmp(&mut self) -> GfmcmpW<IntenclrSpec> {
        GfmcmpW::new(self, 1)
    }
}
#[doc = "Interrupt Enable Clear\n\nYou can [`read`](crate::Reg::read) this register and get [`intenclr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenclr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntenclrSpec;
impl crate::RegisterSpec for IntenclrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`intenclr::R`](R) reader structure"]
impl crate::Readable for IntenclrSpec {}
#[doc = "`write(|w| ..)` method takes [`intenclr::W`](W) writer structure"]
impl crate::Writable for IntenclrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets INTENCLR to value 0"]
impl crate::Resettable for IntenclrSpec {
    const RESET_VALUE: u8 = 0;
}
