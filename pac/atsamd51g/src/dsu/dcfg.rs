#[doc = "Register `DCFG[%s]` reader"]
pub type R = crate::R<DCFG_SPEC>;
#[doc = "Register `DCFG[%s]` writer"]
pub type W = crate::W<DCFG_SPEC>;
#[doc = "Field `DCFG` reader - Device Configuration"]
pub type DCFG_R = crate::FieldReader<u32>;
#[doc = "Field `DCFG` writer - Device Configuration"]
pub type DCFG_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Device Configuration"]
    #[inline(always)]
    pub fn dcfg(&self) -> DCFG_R {
        DCFG_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Device Configuration"]
    #[inline(always)]
    #[must_use]
    pub fn dcfg(&mut self) -> DCFG_W<DCFG_SPEC, 0> {
        DCFG_W::new(self)
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
#[doc = "Device Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DCFG_SPEC;
impl crate::RegisterSpec for DCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcfg::R`](R) reader structure"]
impl crate::Readable for DCFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dcfg::W`](W) writer structure"]
impl crate::Writable for DCFG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DCFG[%s]
to value 0"]
impl crate::Resettable for DCFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
