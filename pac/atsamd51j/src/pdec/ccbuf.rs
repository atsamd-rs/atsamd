#[doc = "Register `CCBUF[%s]` reader"]
pub type R = crate::R<CCBUF_SPEC>;
#[doc = "Register `CCBUF[%s]` writer"]
pub type W = crate::W<CCBUF_SPEC>;
#[doc = "Field `CCBUF` reader - Channel Compare Buffer Value"]
pub type CCBUF_R = crate::FieldReader<u16>;
#[doc = "Field `CCBUF` writer - Channel Compare Buffer Value"]
pub type CCBUF_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - Channel Compare Buffer Value"]
    #[inline(always)]
    pub fn ccbuf(&self) -> CCBUF_R {
        CCBUF_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Channel Compare Buffer Value"]
    #[inline(always)]
    #[must_use]
    pub fn ccbuf(&mut self) -> CCBUF_W<CCBUF_SPEC, 0> {
        CCBUF_W::new(self)
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
#[doc = "Channel Compare Buffer Value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccbuf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccbuf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CCBUF_SPEC;
impl crate::RegisterSpec for CCBUF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccbuf::R`](R) reader structure"]
impl crate::Readable for CCBUF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ccbuf::W`](W) writer structure"]
impl crate::Writable for CCBUF_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CCBUF[%s]
to value 0"]
impl crate::Resettable for CCBUF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
