#[doc = "Register `MC2R` writer"]
pub struct W(crate::W<MC2R_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MC2R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<MC2R_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MC2R_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SRESP` writer - e.MMC Abort Wait IRQ"]
pub type SRESP_W<'a, const O: u8> = crate::BitWriter<'a, u8, MC2R_SPEC, bool, O>;
#[doc = "Field `ABOOT` writer - e.MMC Abort Boot"]
pub type ABOOT_W<'a, const O: u8> = crate::BitWriter<'a, u8, MC2R_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - e.MMC Abort Wait IRQ"]
    #[inline(always)]
    #[must_use]
    pub fn sresp(&mut self) -> SRESP_W<0> {
        SRESP_W::new(self)
    }
    #[doc = "Bit 1 - e.MMC Abort Boot"]
    #[inline(always)]
    #[must_use]
    pub fn aboot(&mut self) -> ABOOT_W<1> {
        ABOOT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MMC Control 2\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mc2r](index.html) module"]
pub struct MC2R_SPEC;
impl crate::RegisterSpec for MC2R_SPEC {
    type Ux = u8;
}
#[doc = "`write(|w| ..)` method takes [mc2r::W](W) writer structure"]
impl crate::Writable for MC2R_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MC2R to value 0"]
impl crate::Resettable for MC2R_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
