#[doc = "Register `WPCLR` reader"]
pub type R = crate::R<WPCLR_SPEC>;
#[doc = "Register `WPCLR` writer"]
pub type W = crate::W<WPCLR_SPEC>;
#[doc = "Field `WP` reader - Write Protection Clear"]
pub type WP_R = crate::FieldReader<u32>;
#[doc = "Field `WP` writer - Write Protection Clear"]
pub type WP_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 31, O, u32>;
impl R {
    #[doc = "Bits 1:31 - Write Protection Clear"]
    #[inline(always)]
    pub fn wp(&self) -> WP_R {
        WP_R::new((self.bits >> 1) & 0x7fff_ffff)
    }
}
impl W {
    #[doc = "Bits 1:31 - Write Protection Clear"]
    #[inline(always)]
    #[must_use]
    pub fn wp(&mut self) -> WP_W<WPCLR_SPEC, 1> {
        WP_W::new(self)
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
#[doc = "Write Protection Clear\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wpclr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wpclr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WPCLR_SPEC;
impl crate::RegisterSpec for WPCLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wpclr::R`](R) reader structure"]
impl crate::Readable for WPCLR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`wpclr::W`](W) writer structure"]
impl crate::Writable for WPCLR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WPCLR to value 0"]
impl crate::Resettable for WPCLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
