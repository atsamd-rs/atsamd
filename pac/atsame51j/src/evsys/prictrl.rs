#[doc = "Register `PRICTRL` reader"]
pub type R = crate::R<PRICTRL_SPEC>;
#[doc = "Register `PRICTRL` writer"]
pub type W = crate::W<PRICTRL_SPEC>;
#[doc = "Field `PRI` reader - Channel Priority Number"]
pub type PRI_R = crate::FieldReader;
#[doc = "Field `PRI` writer - Channel Priority Number"]
pub type PRI_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `RREN` reader - Round-Robin Scheduling Enable"]
pub type RREN_R = crate::BitReader;
#[doc = "Field `RREN` writer - Round-Robin Scheduling Enable"]
pub type RREN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:3 - Channel Priority Number"]
    #[inline(always)]
    pub fn pri(&self) -> PRI_R {
        PRI_R::new(self.bits & 0x0f)
    }
    #[doc = "Bit 7 - Round-Robin Scheduling Enable"]
    #[inline(always)]
    pub fn rren(&self) -> RREN_R {
        RREN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Channel Priority Number"]
    #[inline(always)]
    #[must_use]
    pub fn pri(&mut self) -> PRI_W<PRICTRL_SPEC, 0> {
        PRI_W::new(self)
    }
    #[doc = "Bit 7 - Round-Robin Scheduling Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rren(&mut self) -> RREN_W<PRICTRL_SPEC, 7> {
        RREN_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Priority Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`prictrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`prictrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PRICTRL_SPEC;
impl crate::RegisterSpec for PRICTRL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`prictrl::R`](R) reader structure"]
impl crate::Readable for PRICTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`prictrl::W`](W) writer structure"]
impl crate::Writable for PRICTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PRICTRL to value 0"]
impl crate::Resettable for PRICTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
