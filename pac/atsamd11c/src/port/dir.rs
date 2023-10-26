#[doc = "Register `DIR%s` reader"]
pub type R = crate::R<DIR_SPEC>;
#[doc = "Register `DIR%s` writer"]
pub type W = crate::W<DIR_SPEC>;
#[doc = "Field `DIR` reader - Port Data Direction"]
pub type DIR_R = crate::FieldReader<u32>;
#[doc = "Field `DIR` writer - Port Data Direction"]
pub type DIR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Port Data Direction"]
    #[inline(always)]
    pub fn dir(&self) -> DIR_R {
        DIR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Port Data Direction"]
    #[inline(always)]
    #[must_use]
    pub fn dir(&mut self) -> DIR_W<DIR_SPEC, 0> {
        DIR_W::new(self)
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
#[doc = "Data Direction\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dir::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dir::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DIR_SPEC;
impl crate::RegisterSpec for DIR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dir::R`](R) reader structure"]
impl crate::Readable for DIR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dir::W`](W) writer structure"]
impl crate::Writable for DIR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DIR%s to value 0"]
impl crate::Resettable for DIR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
