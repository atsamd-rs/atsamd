#[doc = "Register `VTOR` reader"]
pub type R = crate::R<VTOR_SPEC>;
#[doc = "Register `VTOR` writer"]
pub type W = crate::W<VTOR_SPEC>;
#[doc = "Field `TBLOFF` reader - Vector table base offset"]
pub type TBLOFF_R = crate::FieldReader<u32>;
#[doc = "Field `TBLOFF` writer - Vector table base offset"]
pub type TBLOFF_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 25, O, u32>;
impl R {
    #[doc = "Bits 7:31 - Vector table base offset"]
    #[inline(always)]
    pub fn tbloff(&self) -> TBLOFF_R {
        TBLOFF_R::new((self.bits >> 7) & 0x01ff_ffff)
    }
}
impl W {
    #[doc = "Bits 7:31 - Vector table base offset"]
    #[inline(always)]
    #[must_use]
    pub fn tbloff(&mut self) -> TBLOFF_W<VTOR_SPEC, 7> {
        TBLOFF_W::new(self)
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
#[doc = "Vector Table Offset Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vtor::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vtor::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VTOR_SPEC;
impl crate::RegisterSpec for VTOR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vtor::R`](R) reader structure"]
impl crate::Readable for VTOR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`vtor::W`](W) writer structure"]
impl crate::Writable for VTOR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets VTOR to value 0"]
impl crate::Resettable for VTOR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
