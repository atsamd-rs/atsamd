#[doc = "Register `NSC` reader"]
pub type R = crate::R<NSC_SPEC>;
#[doc = "Register `NSC` writer"]
pub type W = crate::W<NSC_SPEC>;
#[doc = "Field `NANOSEC` reader - 1588 Timer Nanosecond comparison value"]
pub type NANOSEC_R = crate::FieldReader<u32>;
#[doc = "Field `NANOSEC` writer - 1588 Timer Nanosecond comparison value"]
pub type NANOSEC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 21, O, u32>;
impl R {
    #[doc = "Bits 0:20 - 1588 Timer Nanosecond comparison value"]
    #[inline(always)]
    pub fn nanosec(&self) -> NANOSEC_R {
        NANOSEC_R::new(self.bits & 0x001f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:20 - 1588 Timer Nanosecond comparison value"]
    #[inline(always)]
    #[must_use]
    pub fn nanosec(&mut self) -> NANOSEC_W<NSC_SPEC, 0> {
        NANOSEC_W::new(self)
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
#[doc = "Tsu timer comparison nanoseconds Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nsc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nsc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NSC_SPEC;
impl crate::RegisterSpec for NSC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`nsc::R`](R) reader structure"]
impl crate::Readable for NSC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`nsc::W`](W) writer structure"]
impl crate::Writable for NSC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets NSC to value 0"]
impl crate::Resettable for NSC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
