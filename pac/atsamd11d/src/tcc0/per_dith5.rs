#[doc = "Register `PER_DITH5` reader"]
pub type R = crate::R<PER_DITH5_SPEC>;
#[doc = "Register `PER_DITH5` writer"]
pub type W = crate::W<PER_DITH5_SPEC>;
#[doc = "Field `DITHERCY` reader - Dithering Cycle Number"]
pub type DITHERCY_R = crate::FieldReader;
#[doc = "Field `DITHERCY` writer - Dithering Cycle Number"]
pub type DITHERCY_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
#[doc = "Field `PER` reader - Period Value"]
pub type PER_R = crate::FieldReader<u32>;
#[doc = "Field `PER` writer - Period Value"]
pub type PER_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 19, O, u32>;
impl R {
    #[doc = "Bits 0:4 - Dithering Cycle Number"]
    #[inline(always)]
    pub fn dithercy(&self) -> DITHERCY_R {
        DITHERCY_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:23 - Period Value"]
    #[inline(always)]
    pub fn per(&self) -> PER_R {
        PER_R::new((self.bits >> 5) & 0x0007_ffff)
    }
}
impl W {
    #[doc = "Bits 0:4 - Dithering Cycle Number"]
    #[inline(always)]
    #[must_use]
    pub fn dithercy(&mut self) -> DITHERCY_W<PER_DITH5_SPEC, 0> {
        DITHERCY_W::new(self)
    }
    #[doc = "Bits 5:23 - Period Value"]
    #[inline(always)]
    #[must_use]
    pub fn per(&mut self) -> PER_W<PER_DITH5_SPEC, 5> {
        PER_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Period\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`per_dith5::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`per_dith5::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PER_DITH5_SPEC;
impl crate::RegisterSpec for PER_DITH5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`per_dith5::R`](R) reader structure"]
impl crate::Readable for PER_DITH5_SPEC {}
#[doc = "`write(|w| ..)` method takes [`per_dith5::W`](W) writer structure"]
impl crate::Writable for PER_DITH5_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PER_DITH5 to value 0xffff_ffff"]
impl crate::Resettable for PER_DITH5_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
