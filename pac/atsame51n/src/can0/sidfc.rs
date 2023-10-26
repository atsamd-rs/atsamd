#[doc = "Register `SIDFC` reader"]
pub type R = crate::R<SIDFC_SPEC>;
#[doc = "Register `SIDFC` writer"]
pub type W = crate::W<SIDFC_SPEC>;
#[doc = "Field `FLSSA` reader - Filter List Standard Start Address"]
pub type FLSSA_R = crate::FieldReader<u16>;
#[doc = "Field `FLSSA` writer - Filter List Standard Start Address"]
pub type FLSSA_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
#[doc = "Field `LSS` reader - List Size Standard"]
pub type LSS_R = crate::FieldReader;
#[doc = "Field `LSS` writer - List Size Standard"]
pub type LSS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:15 - Filter List Standard Start Address"]
    #[inline(always)]
    pub fn flssa(&self) -> FLSSA_R {
        FLSSA_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:23 - List Size Standard"]
    #[inline(always)]
    pub fn lss(&self) -> LSS_R {
        LSS_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - Filter List Standard Start Address"]
    #[inline(always)]
    #[must_use]
    pub fn flssa(&mut self) -> FLSSA_W<SIDFC_SPEC, 0> {
        FLSSA_W::new(self)
    }
    #[doc = "Bits 16:23 - List Size Standard"]
    #[inline(always)]
    #[must_use]
    pub fn lss(&mut self) -> LSS_W<SIDFC_SPEC, 16> {
        LSS_W::new(self)
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
#[doc = "Standard ID Filter Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sidfc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sidfc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SIDFC_SPEC;
impl crate::RegisterSpec for SIDFC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sidfc::R`](R) reader structure"]
impl crate::Readable for SIDFC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sidfc::W`](W) writer structure"]
impl crate::Writable for SIDFC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SIDFC to value 0"]
impl crate::Resettable for SIDFC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
