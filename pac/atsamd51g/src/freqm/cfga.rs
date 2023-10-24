#[doc = "Register `CFGA` reader"]
pub type R = crate::R<CFGA_SPEC>;
#[doc = "Register `CFGA` writer"]
pub type W = crate::W<CFGA_SPEC>;
#[doc = "Field `REFNUM` reader - Number of Reference Clock Cycles"]
pub type REFNUM_R = crate::FieldReader;
#[doc = "Field `REFNUM` writer - Number of Reference Clock Cycles"]
pub type REFNUM_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Number of Reference Clock Cycles"]
    #[inline(always)]
    pub fn refnum(&self) -> REFNUM_R {
        REFNUM_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Number of Reference Clock Cycles"]
    #[inline(always)]
    #[must_use]
    pub fn refnum(&mut self) -> REFNUM_W<CFGA_SPEC, 0> {
        REFNUM_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Config A register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfga::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfga::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFGA_SPEC;
impl crate::RegisterSpec for CFGA_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`cfga::R`](R) reader structure"]
impl crate::Readable for CFGA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cfga::W`](W) writer structure"]
impl crate::Writable for CFGA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CFGA to value 0"]
impl crate::Resettable for CFGA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
