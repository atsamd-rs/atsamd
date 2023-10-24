#[doc = "Register `TSSSL` reader"]
pub type R = crate::R<TSSSL_SPEC>;
#[doc = "Register `TSSSL` writer"]
pub type W = crate::W<TSSSL_SPEC>;
#[doc = "Field `VTS` reader - Value of Timer Seconds Register Capture"]
pub type VTS_R = crate::FieldReader<u32>;
#[doc = "Field `VTS` writer - Value of Timer Seconds Register Capture"]
pub type VTS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Value of Timer Seconds Register Capture"]
    #[inline(always)]
    pub fn vts(&self) -> VTS_R {
        VTS_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Value of Timer Seconds Register Capture"]
    #[inline(always)]
    #[must_use]
    pub fn vts(&mut self) -> VTS_W<TSSSL_SPEC, 0> {
        VTS_W::new(self)
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
#[doc = "1588 Timer Sync Strobe Seconds \\[31:0\\]
Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tsssl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tsssl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TSSSL_SPEC;
impl crate::RegisterSpec for TSSSL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tsssl::R`](R) reader structure"]
impl crate::Readable for TSSSL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tsssl::W`](W) writer structure"]
impl crate::Writable for TSSSL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TSSSL to value 0"]
impl crate::Resettable for TSSSL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
