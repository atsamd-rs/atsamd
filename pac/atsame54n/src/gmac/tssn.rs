#[doc = "Register `TSSN` reader"]
pub type R = crate::R<TSSN_SPEC>;
#[doc = "Register `TSSN` writer"]
pub type W = crate::W<TSSN_SPEC>;
#[doc = "Field `VTN` reader - Value Timer Nanoseconds Register Capture"]
pub type VTN_R = crate::FieldReader<u32>;
#[doc = "Field `VTN` writer - Value Timer Nanoseconds Register Capture"]
pub type VTN_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 30, O, u32>;
impl R {
    #[doc = "Bits 0:29 - Value Timer Nanoseconds Register Capture"]
    #[inline(always)]
    pub fn vtn(&self) -> VTN_R {
        VTN_R::new(self.bits & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:29 - Value Timer Nanoseconds Register Capture"]
    #[inline(always)]
    #[must_use]
    pub fn vtn(&mut self) -> VTN_W<TSSN_SPEC, 0> {
        VTN_W::new(self)
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
#[doc = "1588 Timer Sync Strobe Nanoseconds Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tssn::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tssn::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TSSN_SPEC;
impl crate::RegisterSpec for TSSN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tssn::R`](R) reader structure"]
impl crate::Readable for TSSN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tssn::W`](W) writer structure"]
impl crate::Writable for TSSN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TSSN to value 0"]
impl crate::Resettable for TSSN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
