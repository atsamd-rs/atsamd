#[doc = "Register `RJFML` reader"]
pub type R = crate::R<RJFML_SPEC>;
#[doc = "Register `RJFML` writer"]
pub type W = crate::W<RJFML_SPEC>;
#[doc = "Field `FML` reader - Frame Max Length"]
pub type FML_R = crate::FieldReader<u16>;
#[doc = "Field `FML` writer - Frame Max Length"]
pub type FML_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 14, O, u16>;
impl R {
    #[doc = "Bits 0:13 - Frame Max Length"]
    #[inline(always)]
    pub fn fml(&self) -> FML_R {
        FML_R::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - Frame Max Length"]
    #[inline(always)]
    #[must_use]
    pub fn fml(&mut self) -> FML_W<RJFML_SPEC, 0> {
        FML_W::new(self)
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
#[doc = "RX Jumbo Frame Max Length Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rjfml::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rjfml::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RJFML_SPEC;
impl crate::RegisterSpec for RJFML_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rjfml::R`](R) reader structure"]
impl crate::Readable for RJFML_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rjfml::W`](W) writer structure"]
impl crate::Writable for RJFML_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RJFML to value 0x3fff"]
impl crate::Resettable for RJFML_SPEC {
    const RESET_VALUE: Self::Ux = 0x3fff;
}
