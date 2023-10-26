#[doc = "Register `PERBUF_DITH5_MODE` reader"]
pub type R = crate::R<PERBUF_DITH5_MODE_SPEC>;
#[doc = "Register `PERBUF_DITH5_MODE` writer"]
pub type W = crate::W<PERBUF_DITH5_MODE_SPEC>;
#[doc = "Field `DITHERBUF` reader - Dithering Buffer Cycle Number"]
pub type DITHERBUF_R = crate::FieldReader;
#[doc = "Field `DITHERBUF` writer - Dithering Buffer Cycle Number"]
pub type DITHERBUF_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
#[doc = "Field `PERBUF` reader - Period Buffer Value"]
pub type PERBUF_R = crate::FieldReader<u32>;
#[doc = "Field `PERBUF` writer - Period Buffer Value"]
pub type PERBUF_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 19, O, u32>;
impl R {
    #[doc = "Bits 0:4 - Dithering Buffer Cycle Number"]
    #[inline(always)]
    pub fn ditherbuf(&self) -> DITHERBUF_R {
        DITHERBUF_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:23 - Period Buffer Value"]
    #[inline(always)]
    pub fn perbuf(&self) -> PERBUF_R {
        PERBUF_R::new((self.bits >> 5) & 0x0007_ffff)
    }
}
impl W {
    #[doc = "Bits 0:4 - Dithering Buffer Cycle Number"]
    #[inline(always)]
    #[must_use]
    pub fn ditherbuf(&mut self) -> DITHERBUF_W<PERBUF_DITH5_MODE_SPEC, 0> {
        DITHERBUF_W::new(self)
    }
    #[doc = "Bits 5:23 - Period Buffer Value"]
    #[inline(always)]
    #[must_use]
    pub fn perbuf(&mut self) -> PERBUF_W<PERBUF_DITH5_MODE_SPEC, 5> {
        PERBUF_W::new(self)
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
#[doc = "Period Buffer\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`perbuf_dith5_mode::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`perbuf_dith5_mode::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PERBUF_DITH5_MODE_SPEC;
impl crate::RegisterSpec for PERBUF_DITH5_MODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`perbuf_dith5_mode::R`](R) reader structure"]
impl crate::Readable for PERBUF_DITH5_MODE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`perbuf_dith5_mode::W`](W) writer structure"]
impl crate::Writable for PERBUF_DITH5_MODE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PERBUF_DITH5_MODE to value 0xffff_ffff"]
impl crate::Resettable for PERBUF_DITH5_MODE_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
