#[doc = "Register `MC2R` writer"]
pub type W = crate::W<MC2R_SPEC>;
#[doc = "Field `SRESP` writer - e.MMC Abort Wait IRQ"]
pub type SRESP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ABOOT` writer - e.MMC Abort Boot"]
pub type ABOOT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl W {
    #[doc = "Bit 0 - e.MMC Abort Wait IRQ"]
    #[inline(always)]
    #[must_use]
    pub fn sresp(&mut self) -> SRESP_W<MC2R_SPEC, 0> {
        SRESP_W::new(self)
    }
    #[doc = "Bit 1 - e.MMC Abort Boot"]
    #[inline(always)]
    #[must_use]
    pub fn aboot(&mut self) -> ABOOT_W<MC2R_SPEC, 1> {
        ABOOT_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "MMC Control 2\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mc2r::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MC2R_SPEC;
impl crate::RegisterSpec for MC2R_SPEC {
    type Ux = u8;
}
#[doc = "`write(|w| ..)` method takes [`mc2r::W`](W) writer structure"]
impl crate::Writable for MC2R_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MC2R to value 0"]
impl crate::Resettable for MC2R_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
