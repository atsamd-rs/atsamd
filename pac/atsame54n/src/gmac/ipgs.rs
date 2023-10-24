#[doc = "Register `IPGS` reader"]
pub type R = crate::R<IPGS_SPEC>;
#[doc = "Register `IPGS` writer"]
pub type W = crate::W<IPGS_SPEC>;
#[doc = "Field `FL` reader - Frame Length"]
pub type FL_R = crate::FieldReader<u16>;
#[doc = "Field `FL` writer - Frame Length"]
pub type FL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - Frame Length"]
    #[inline(always)]
    pub fn fl(&self) -> FL_R {
        FL_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Frame Length"]
    #[inline(always)]
    #[must_use]
    pub fn fl(&mut self) -> FL_W<IPGS_SPEC, 0> {
        FL_W::new(self)
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
#[doc = "IPG Stretch Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ipgs::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ipgs::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IPGS_SPEC;
impl crate::RegisterSpec for IPGS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ipgs::R`](R) reader structure"]
impl crate::Readable for IPGS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ipgs::W`](W) writer structure"]
impl crate::Writable for IPGS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IPGS to value 0"]
impl crate::Resettable for IPGS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
