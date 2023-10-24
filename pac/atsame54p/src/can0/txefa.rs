#[doc = "Register `TXEFA` reader"]
pub type R = crate::R<TXEFA_SPEC>;
#[doc = "Register `TXEFA` writer"]
pub type W = crate::W<TXEFA_SPEC>;
#[doc = "Field `EFAI` reader - Event FIFO Acknowledge Index"]
pub type EFAI_R = crate::FieldReader;
#[doc = "Field `EFAI` writer - Event FIFO Acknowledge Index"]
pub type EFAI_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
impl R {
    #[doc = "Bits 0:4 - Event FIFO Acknowledge Index"]
    #[inline(always)]
    pub fn efai(&self) -> EFAI_R {
        EFAI_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Event FIFO Acknowledge Index"]
    #[inline(always)]
    #[must_use]
    pub fn efai(&mut self) -> EFAI_W<TXEFA_SPEC, 0> {
        EFAI_W::new(self)
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
#[doc = "Tx Event FIFO Acknowledge\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txefa::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txefa::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TXEFA_SPEC;
impl crate::RegisterSpec for TXEFA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txefa::R`](R) reader structure"]
impl crate::Readable for TXEFA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`txefa::W`](W) writer structure"]
impl crate::Writable for TXEFA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TXEFA to value 0"]
impl crate::Resettable for TXEFA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
