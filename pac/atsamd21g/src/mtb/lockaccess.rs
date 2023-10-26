#[doc = "Register `LOCKACCESS` reader"]
pub type R = crate::R<LOCKACCESS_SPEC>;
#[doc = "Register `LOCKACCESS` writer"]
pub type W = crate::W<LOCKACCESS_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<LOCKACCESS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
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
#[doc = "MTB Lock Access\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lockaccess::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lockaccess::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LOCKACCESS_SPEC;
impl crate::RegisterSpec for LOCKACCESS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lockaccess::R`](R) reader structure"]
impl crate::Readable for LOCKACCESS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lockaccess::W`](W) writer structure"]
impl crate::Writable for LOCKACCESS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LOCKACCESS to value 0"]
impl crate::Resettable for LOCKACCESS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
