#[doc = "Register `DPLLRATIO` reader"]
pub type R = crate::R<DPLLRATIO_SPEC>;
#[doc = "Register `DPLLRATIO` writer"]
pub type W = crate::W<DPLLRATIO_SPEC>;
#[doc = "Field `LDR` reader - Loop Divider Ratio"]
pub type LDR_R = crate::FieldReader<u16>;
#[doc = "Field `LDR` writer - Loop Divider Ratio"]
pub type LDR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 12, O, u16>;
#[doc = "Field `LDRFRAC` reader - Loop Divider Ratio Fractional Part"]
pub type LDRFRAC_R = crate::FieldReader;
#[doc = "Field `LDRFRAC` writer - Loop Divider Ratio Fractional Part"]
pub type LDRFRAC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
impl R {
    #[doc = "Bits 0:11 - Loop Divider Ratio"]
    #[inline(always)]
    pub fn ldr(&self) -> LDR_R {
        LDR_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:19 - Loop Divider Ratio Fractional Part"]
    #[inline(always)]
    pub fn ldrfrac(&self) -> LDRFRAC_R {
        LDRFRAC_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:11 - Loop Divider Ratio"]
    #[inline(always)]
    #[must_use]
    pub fn ldr(&mut self) -> LDR_W<DPLLRATIO_SPEC, 0> {
        LDR_W::new(self)
    }
    #[doc = "Bits 16:19 - Loop Divider Ratio Fractional Part"]
    #[inline(always)]
    #[must_use]
    pub fn ldrfrac(&mut self) -> LDRFRAC_W<DPLLRATIO_SPEC, 16> {
        LDRFRAC_W::new(self)
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
#[doc = "DPLL Ratio Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dpllratio::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dpllratio::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DPLLRATIO_SPEC;
impl crate::RegisterSpec for DPLLRATIO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dpllratio::R`](R) reader structure"]
impl crate::Readable for DPLLRATIO_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dpllratio::W`](W) writer structure"]
impl crate::Writable for DPLLRATIO_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DPLLRATIO to value 0"]
impl crate::Resettable for DPLLRATIO_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
