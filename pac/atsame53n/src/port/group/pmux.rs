#[doc = "Register `PMUX[%s]` reader"]
pub type R = crate::R<PMUX_SPEC>;
#[doc = "Register `PMUX[%s]` writer"]
pub type W = crate::W<PMUX_SPEC>;
#[doc = "Field `PMUXE` reader - Peripheral Multiplexing for Even-Numbered Pin"]
pub type PMUXE_R = crate::FieldReader;
#[doc = "Field `PMUXE` writer - Peripheral Multiplexing for Even-Numbered Pin"]
pub type PMUXE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `PMUXO` reader - Peripheral Multiplexing for Odd-Numbered Pin"]
pub type PMUXO_R = crate::FieldReader;
#[doc = "Field `PMUXO` writer - Peripheral Multiplexing for Odd-Numbered Pin"]
pub type PMUXO_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
impl R {
    #[doc = "Bits 0:3 - Peripheral Multiplexing for Even-Numbered Pin"]
    #[inline(always)]
    pub fn pmuxe(&self) -> PMUXE_R {
        PMUXE_R::new(self.bits & 0x0f)
    }
    #[doc = "Bits 4:7 - Peripheral Multiplexing for Odd-Numbered Pin"]
    #[inline(always)]
    pub fn pmuxo(&self) -> PMUXO_R {
        PMUXO_R::new((self.bits >> 4) & 0x0f)
    }
}
impl W {
    #[doc = "Bits 0:3 - Peripheral Multiplexing for Even-Numbered Pin"]
    #[inline(always)]
    #[must_use]
    pub fn pmuxe(&mut self) -> PMUXE_W<PMUX_SPEC, 0> {
        PMUXE_W::new(self)
    }
    #[doc = "Bits 4:7 - Peripheral Multiplexing for Odd-Numbered Pin"]
    #[inline(always)]
    #[must_use]
    pub fn pmuxo(&mut self) -> PMUXO_W<PMUX_SPEC, 4> {
        PMUXO_W::new(self)
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
#[doc = "Peripheral Multiplexing\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmux::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmux::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PMUX_SPEC;
impl crate::RegisterSpec for PMUX_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`pmux::R`](R) reader structure"]
impl crate::Readable for PMUX_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pmux::W`](W) writer structure"]
impl crate::Writable for PMUX_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PMUX[%s]
to value 0"]
impl crate::Resettable for PMUX_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
