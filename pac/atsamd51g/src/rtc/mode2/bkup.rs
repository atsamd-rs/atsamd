#[doc = "Register `BKUP[%s]` reader"]
pub type R = crate::R<BKUP_SPEC>;
#[doc = "Register `BKUP[%s]` writer"]
pub type W = crate::W<BKUP_SPEC>;
#[doc = "Field `BKUP` reader - Backup"]
pub type BKUP_R = crate::FieldReader<u32>;
#[doc = "Field `BKUP` writer - Backup"]
pub type BKUP_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Backup"]
    #[inline(always)]
    pub fn bkup(&self) -> BKUP_R {
        BKUP_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Backup"]
    #[inline(always)]
    #[must_use]
    pub fn bkup(&mut self) -> BKUP_W<BKUP_SPEC, 0> {
        BKUP_W::new(self)
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
#[doc = "Backup\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bkup::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bkup::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BKUP_SPEC;
impl crate::RegisterSpec for BKUP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bkup::R`](R) reader structure"]
impl crate::Readable for BKUP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`bkup::W`](W) writer structure"]
impl crate::Writable for BKUP_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BKUP[%s]
to value 0"]
impl crate::Resettable for BKUP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
