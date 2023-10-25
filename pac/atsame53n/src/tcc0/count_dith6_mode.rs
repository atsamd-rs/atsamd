#[doc = "Register `COUNT_DITH6_MODE` reader"]
pub type R = crate::R<COUNT_DITH6_MODE_SPEC>;
#[doc = "Register `COUNT_DITH6_MODE` writer"]
pub type W = crate::W<COUNT_DITH6_MODE_SPEC>;
#[doc = "Field `COUNT` reader - Counter Value"]
pub type COUNT_R = crate::FieldReader<u32>;
#[doc = "Field `COUNT` writer - Counter Value"]
pub type COUNT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 18, O, u32>;
impl R {
    #[doc = "Bits 6:23 - Counter Value"]
    #[inline(always)]
    pub fn count(&self) -> COUNT_R {
        COUNT_R::new((self.bits >> 6) & 0x0003_ffff)
    }
}
impl W {
    #[doc = "Bits 6:23 - Counter Value"]
    #[inline(always)]
    #[must_use]
    pub fn count(&mut self) -> COUNT_W<COUNT_DITH6_MODE_SPEC, 6> {
        COUNT_W::new(self)
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
#[doc = "Count\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`count_dith6_mode::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`count_dith6_mode::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct COUNT_DITH6_MODE_SPEC;
impl crate::RegisterSpec for COUNT_DITH6_MODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`count_dith6_mode::R`](R) reader structure"]
impl crate::Readable for COUNT_DITH6_MODE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`count_dith6_mode::W`](W) writer structure"]
impl crate::Writable for COUNT_DITH6_MODE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets COUNT_DITH6_MODE to value 0"]
impl crate::Resettable for COUNT_DITH6_MODE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
