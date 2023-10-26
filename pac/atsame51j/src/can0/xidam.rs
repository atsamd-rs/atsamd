#[doc = "Register `XIDAM` reader"]
pub type R = crate::R<XIDAM_SPEC>;
#[doc = "Register `XIDAM` writer"]
pub type W = crate::W<XIDAM_SPEC>;
#[doc = "Field `EIDM` reader - Extended ID Mask"]
pub type EIDM_R = crate::FieldReader<u32>;
#[doc = "Field `EIDM` writer - Extended ID Mask"]
pub type EIDM_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 29, O, u32>;
impl R {
    #[doc = "Bits 0:28 - Extended ID Mask"]
    #[inline(always)]
    pub fn eidm(&self) -> EIDM_R {
        EIDM_R::new(self.bits & 0x1fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:28 - Extended ID Mask"]
    #[inline(always)]
    #[must_use]
    pub fn eidm(&mut self) -> EIDM_W<XIDAM_SPEC, 0> {
        EIDM_W::new(self)
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
#[doc = "Extended ID AND Mask\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`xidam::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`xidam::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct XIDAM_SPEC;
impl crate::RegisterSpec for XIDAM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`xidam::R`](R) reader structure"]
impl crate::Readable for XIDAM_SPEC {}
#[doc = "`write(|w| ..)` method takes [`xidam::W`](W) writer structure"]
impl crate::Writable for XIDAM_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets XIDAM to value 0x1fff_ffff"]
impl crate::Resettable for XIDAM_SPEC {
    const RESET_VALUE: Self::Ux = 0x1fff_ffff;
}
