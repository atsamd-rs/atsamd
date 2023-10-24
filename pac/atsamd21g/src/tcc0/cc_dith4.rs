#[doc = "Register `CC%s_DITH4` reader"]
pub type R = crate::R<CC_DITH4_SPEC>;
#[doc = "Register `CC%s_DITH4` writer"]
pub type W = crate::W<CC_DITH4_SPEC>;
#[doc = "Field `DITHERCY` reader - Dithering Cycle Number"]
pub type DITHERCY_R = crate::FieldReader;
#[doc = "Field `DITHERCY` writer - Dithering Cycle Number"]
pub type DITHERCY_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `CC` reader - Channel Compare/Capture Value"]
pub type CC_R = crate::FieldReader<u32>;
#[doc = "Field `CC` writer - Channel Compare/Capture Value"]
pub type CC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 20, O, u32>;
impl R {
    #[doc = "Bits 0:3 - Dithering Cycle Number"]
    #[inline(always)]
    pub fn dithercy(&self) -> DITHERCY_R {
        DITHERCY_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:23 - Channel Compare/Capture Value"]
    #[inline(always)]
    pub fn cc(&self) -> CC_R {
        CC_R::new((self.bits >> 4) & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:3 - Dithering Cycle Number"]
    #[inline(always)]
    #[must_use]
    pub fn dithercy(&mut self) -> DITHERCY_W<CC_DITH4_SPEC, 0> {
        DITHERCY_W::new(self)
    }
    #[doc = "Bits 4:23 - Channel Compare/Capture Value"]
    #[inline(always)]
    #[must_use]
    pub fn cc(&mut self) -> CC_W<CC_DITH4_SPEC, 4> {
        CC_W::new(self)
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
#[doc = "Compare and Capture\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cc_dith4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cc_dith4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CC_DITH4_SPEC;
impl crate::RegisterSpec for CC_DITH4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cc_dith4::R`](R) reader structure"]
impl crate::Readable for CC_DITH4_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cc_dith4::W`](W) writer structure"]
impl crate::Writable for CC_DITH4_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CC%s_DITH4 to value 0"]
impl crate::Resettable for CC_DITH4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
