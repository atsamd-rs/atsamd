#[doc = "Register `QOSCTRL` reader"]
pub type R = crate::R<QOSCTRL_SPEC>;
#[doc = "Register `QOSCTRL` writer"]
pub type W = crate::W<QOSCTRL_SPEC>;
#[doc = "Field `CQOS` reader - Configuration Quality of Service"]
pub type CQOS_R = crate::FieldReader;
#[doc = "Field `CQOS` writer - Configuration Quality of Service"]
pub type CQOS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `DQOS` reader - Data Quality of Service"]
pub type DQOS_R = crate::FieldReader;
#[doc = "Field `DQOS` writer - Data Quality of Service"]
pub type DQOS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
impl R {
    #[doc = "Bits 0:1 - Configuration Quality of Service"]
    #[inline(always)]
    pub fn cqos(&self) -> CQOS_R {
        CQOS_R::new(self.bits & 3)
    }
    #[doc = "Bits 2:3 - Data Quality of Service"]
    #[inline(always)]
    pub fn dqos(&self) -> DQOS_R {
        DQOS_R::new((self.bits >> 2) & 3)
    }
}
impl W {
    #[doc = "Bits 0:1 - Configuration Quality of Service"]
    #[inline(always)]
    #[must_use]
    pub fn cqos(&mut self) -> CQOS_W<QOSCTRL_SPEC, 0> {
        CQOS_W::new(self)
    }
    #[doc = "Bits 2:3 - Data Quality of Service"]
    #[inline(always)]
    #[must_use]
    pub fn dqos(&mut self) -> DQOS_W<QOSCTRL_SPEC, 2> {
        DQOS_W::new(self)
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
#[doc = "USB Quality Of Service\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`qosctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`qosctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct QOSCTRL_SPEC;
impl crate::RegisterSpec for QOSCTRL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`qosctrl::R`](R) reader structure"]
impl crate::Readable for QOSCTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`qosctrl::W`](W) writer structure"]
impl crate::Writable for QOSCTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets QOSCTRL to value 0x0f"]
impl crate::Resettable for QOSCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x0f;
}
