#[doc = "Register `XIDFC` reader"]
pub type R = crate::R<XIDFC_SPEC>;
#[doc = "Register `XIDFC` writer"]
pub type W = crate::W<XIDFC_SPEC>;
#[doc = "Field `FLESA` reader - Filter List Extended Start Address"]
pub type FLESA_R = crate::FieldReader<u16>;
#[doc = "Field `FLESA` writer - Filter List Extended Start Address"]
pub type FLESA_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
#[doc = "Field `LSE` reader - List Size Extended"]
pub type LSE_R = crate::FieldReader;
#[doc = "Field `LSE` writer - List Size Extended"]
pub type LSE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O>;
impl R {
    #[doc = "Bits 0:15 - Filter List Extended Start Address"]
    #[inline(always)]
    pub fn flesa(&self) -> FLESA_R {
        FLESA_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:22 - List Size Extended"]
    #[inline(always)]
    pub fn lse(&self) -> LSE_R {
        LSE_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - Filter List Extended Start Address"]
    #[inline(always)]
    #[must_use]
    pub fn flesa(&mut self) -> FLESA_W<XIDFC_SPEC, 0> {
        FLESA_W::new(self)
    }
    #[doc = "Bits 16:22 - List Size Extended"]
    #[inline(always)]
    #[must_use]
    pub fn lse(&mut self) -> LSE_W<XIDFC_SPEC, 16> {
        LSE_W::new(self)
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
#[doc = "Extended ID Filter Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`xidfc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`xidfc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct XIDFC_SPEC;
impl crate::RegisterSpec for XIDFC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`xidfc::R`](R) reader structure"]
impl crate::Readable for XIDFC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`xidfc::W`](W) writer structure"]
impl crate::Writable for XIDFC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets XIDFC to value 0"]
impl crate::Resettable for XIDFC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
