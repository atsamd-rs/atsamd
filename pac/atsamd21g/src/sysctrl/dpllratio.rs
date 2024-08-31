#[doc = "Register `DPLLRATIO` reader"]
pub type R = crate::R<DpllratioSpec>;
#[doc = "Register `DPLLRATIO` writer"]
pub type W = crate::W<DpllratioSpec>;
#[doc = "Field `LDR` reader - Loop Divider Ratio"]
pub type LdrR = crate::FieldReader<u16>;
#[doc = "Field `LDR` writer - Loop Divider Ratio"]
pub type LdrW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `LDRFRAC` reader - Loop Divider Ratio Fractional Part"]
pub type LdrfracR = crate::FieldReader;
#[doc = "Field `LDRFRAC` writer - Loop Divider Ratio Fractional Part"]
pub type LdrfracW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:11 - Loop Divider Ratio"]
    #[inline(always)]
    pub fn ldr(&self) -> LdrR {
        LdrR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:19 - Loop Divider Ratio Fractional Part"]
    #[inline(always)]
    pub fn ldrfrac(&self) -> LdrfracR {
        LdrfracR::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:11 - Loop Divider Ratio"]
    #[inline(always)]
    #[must_use]
    pub fn ldr(&mut self) -> LdrW<DpllratioSpec> {
        LdrW::new(self, 0)
    }
    #[doc = "Bits 16:19 - Loop Divider Ratio Fractional Part"]
    #[inline(always)]
    #[must_use]
    pub fn ldrfrac(&mut self) -> LdrfracW<DpllratioSpec> {
        LdrfracW::new(self, 16)
    }
}
#[doc = "DPLL Ratio Control\n\nYou can [`read`](crate::Reg::read) this register and get [`dpllratio::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dpllratio::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DpllratioSpec;
impl crate::RegisterSpec for DpllratioSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dpllratio::R`](R) reader structure"]
impl crate::Readable for DpllratioSpec {}
#[doc = "`write(|w| ..)` method takes [`dpllratio::W`](W) writer structure"]
impl crate::Writable for DpllratioSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DPLLRATIO to value 0"]
impl crate::Resettable for DpllratioSpec {
    const RESET_VALUE: u32 = 0;
}
