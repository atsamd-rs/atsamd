#[doc = "Register `PERB_DITH5` reader"]
pub type R = crate::R<PERB_DITH5_SPEC>;
#[doc = "Register `PERB_DITH5` writer"]
pub type W = crate::W<PERB_DITH5_SPEC>;
#[doc = "Field `DITHERCYB` reader - Dithering Buffer Cycle Number"]
pub type DITHERCYB_R = crate::FieldReader;
#[doc = "Field `DITHERCYB` writer - Dithering Buffer Cycle Number"]
pub type DITHERCYB_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
#[doc = "Field `PERB` reader - Period Buffer Value"]
pub type PERB_R = crate::FieldReader<u32>;
#[doc = "Field `PERB` writer - Period Buffer Value"]
pub type PERB_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 19, O, u32>;
impl R {
    #[doc = "Bits 0:4 - Dithering Buffer Cycle Number"]
    #[inline(always)]
    pub fn dithercyb(&self) -> DITHERCYB_R {
        DITHERCYB_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:23 - Period Buffer Value"]
    #[inline(always)]
    pub fn perb(&self) -> PERB_R {
        PERB_R::new((self.bits >> 5) & 0x0007_ffff)
    }
}
impl W {
    #[doc = "Bits 0:4 - Dithering Buffer Cycle Number"]
    #[inline(always)]
    #[must_use]
    pub fn dithercyb(&mut self) -> DITHERCYB_W<PERB_DITH5_SPEC, 0> {
        DITHERCYB_W::new(self)
    }
    #[doc = "Bits 5:23 - Period Buffer Value"]
    #[inline(always)]
    #[must_use]
    pub fn perb(&mut self) -> PERB_W<PERB_DITH5_SPEC, 5> {
        PERB_W::new(self)
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
#[doc = "Period Buffer\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`perb_dith5::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`perb_dith5::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PERB_DITH5_SPEC;
impl crate::RegisterSpec for PERB_DITH5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`perb_dith5::R`](R) reader structure"]
impl crate::Readable for PERB_DITH5_SPEC {}
#[doc = "`write(|w| ..)` method takes [`perb_dith5::W`](W) writer structure"]
impl crate::Writable for PERB_DITH5_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PERB_DITH5 to value 0xffff_ffff"]
impl crate::Resettable for PERB_DITH5_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
