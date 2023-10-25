#[doc = "Register `PERB` reader"]
pub type R = crate::R<PERB_SPEC>;
#[doc = "Register `PERB` writer"]
pub type W = crate::W<PERB_SPEC>;
#[doc = "Field `PERB` reader - Period Buffer Value"]
pub type PERB_R = crate::FieldReader<u32>;
#[doc = "Field `PERB` writer - Period Buffer Value"]
pub type PERB_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 24, O, u32>;
impl R {
    #[doc = "Bits 0:23 - Period Buffer Value"]
    #[inline(always)]
    pub fn perb(&self) -> PERB_R {
        PERB_R::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - Period Buffer Value"]
    #[inline(always)]
    #[must_use]
    pub fn perb(&mut self) -> PERB_W<PERB_SPEC, 0> {
        PERB_W::new(self)
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
#[doc = "Period Buffer\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`perb::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`perb::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PERB_SPEC;
impl crate::RegisterSpec for PERB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`perb::R`](R) reader structure"]
impl crate::Readable for PERB_SPEC {}
#[doc = "`write(|w| ..)` method takes [`perb::W`](W) writer structure"]
impl crate::Writable for PERB_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PERB to value 0xffff_ffff"]
impl crate::Resettable for PERB_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
