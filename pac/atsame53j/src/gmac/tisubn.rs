#[doc = "Register `TISUBN` reader"]
pub type R = crate::R<TISUBN_SPEC>;
#[doc = "Register `TISUBN` writer"]
pub type W = crate::W<TISUBN_SPEC>;
#[doc = "Field `LSBTIR` reader - Lower Significant Bits of Timer Increment"]
pub type LSBTIR_R = crate::FieldReader<u16>;
#[doc = "Field `LSBTIR` writer - Lower Significant Bits of Timer Increment"]
pub type LSBTIR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - Lower Significant Bits of Timer Increment"]
    #[inline(always)]
    pub fn lsbtir(&self) -> LSBTIR_R {
        LSBTIR_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Lower Significant Bits of Timer Increment"]
    #[inline(always)]
    #[must_use]
    pub fn lsbtir(&mut self) -> LSBTIR_W<TISUBN_SPEC, 0> {
        LSBTIR_W::new(self)
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
#[doc = "1588 Timer Increment \\[15:0\\]
Sub-Nanoseconds Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tisubn::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tisubn::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TISUBN_SPEC;
impl crate::RegisterSpec for TISUBN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tisubn::R`](R) reader structure"]
impl crate::Readable for TISUBN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tisubn::W`](W) writer structure"]
impl crate::Writable for TISUBN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TISUBN to value 0"]
impl crate::Resettable for TISUBN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
