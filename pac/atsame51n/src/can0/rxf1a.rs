#[doc = "Register `RXF1A` reader"]
pub type R = crate::R<RXF1A_SPEC>;
#[doc = "Register `RXF1A` writer"]
pub type W = crate::W<RXF1A_SPEC>;
#[doc = "Field `F1AI` reader - Rx FIFO 1 Acknowledge Index"]
pub type F1AI_R = crate::FieldReader;
#[doc = "Field `F1AI` writer - Rx FIFO 1 Acknowledge Index"]
pub type F1AI_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 6, O>;
impl R {
    #[doc = "Bits 0:5 - Rx FIFO 1 Acknowledge Index"]
    #[inline(always)]
    pub fn f1ai(&self) -> F1AI_R {
        F1AI_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Rx FIFO 1 Acknowledge Index"]
    #[inline(always)]
    #[must_use]
    pub fn f1ai(&mut self) -> F1AI_W<RXF1A_SPEC, 0> {
        F1AI_W::new(self)
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
#[doc = "Rx FIFO 1 Acknowledge\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxf1a::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxf1a::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RXF1A_SPEC;
impl crate::RegisterSpec for RXF1A_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxf1a::R`](R) reader structure"]
impl crate::Readable for RXF1A_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rxf1a::W`](W) writer structure"]
impl crate::Writable for RXF1A_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RXF1A to value 0"]
impl crate::Resettable for RXF1A_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
