#[doc = "Register `DCC%s` reader"]
pub type R = crate::R<DCC_SPEC>;
#[doc = "Register `DCC%s` writer"]
pub type W = crate::W<DCC_SPEC>;
#[doc = "Field `DATA` reader - Data"]
pub type DATA_R = crate::FieldReader<u32>;
#[doc = "Field `DATA` writer - Data"]
pub type DATA_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Data"]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Data"]
    #[inline(always)]
    #[must_use]
    pub fn data(&mut self) -> DATA_W<DCC_SPEC, 0> {
        DATA_W::new(self)
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
#[doc = "Debug Communication Channel n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DCC_SPEC;
impl crate::RegisterSpec for DCC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcc::R`](R) reader structure"]
impl crate::Readable for DCC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dcc::W`](W) writer structure"]
impl crate::Writable for DCC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DCC%s to value 0"]
impl crate::Resettable for DCC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
