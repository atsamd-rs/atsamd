#[doc = "Register `EVCTRL` reader"]
pub type R = crate::R<EVCTRL_SPEC>;
#[doc = "Register `EVCTRL` writer"]
pub type W = crate::W<EVCTRL_SPEC>;
#[doc = "Field `EXTINTEO` reader - External Interrupt Event Output Enable"]
pub type EXTINTEO_R = crate::FieldReader<u16>;
#[doc = "Field `EXTINTEO` writer - External Interrupt Event Output Enable"]
pub type EXTINTEO_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - External Interrupt Event Output Enable"]
    #[inline(always)]
    pub fn extinteo(&self) -> EXTINTEO_R {
        EXTINTEO_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - External Interrupt Event Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn extinteo(&mut self) -> EXTINTEO_W<EVCTRL_SPEC, 0> {
        EXTINTEO_W::new(self)
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
#[doc = "Event Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`evctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`evctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EVCTRL_SPEC;
impl crate::RegisterSpec for EVCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`evctrl::R`](R) reader structure"]
impl crate::Readable for EVCTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`evctrl::W`](W) writer structure"]
impl crate::Writable for EVCTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EVCTRL to value 0"]
impl crate::Resettable for EVCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
