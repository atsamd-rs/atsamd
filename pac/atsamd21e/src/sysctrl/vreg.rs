#[doc = "Register `VREG` reader"]
pub type R = crate::R<VREG_SPEC>;
#[doc = "Register `VREG` writer"]
pub type W = crate::W<VREG_SPEC>;
#[doc = "Field `RUNSTDBY` reader - Run in Standby"]
pub type RUNSTDBY_R = crate::BitReader;
#[doc = "Field `RUNSTDBY` writer - Run in Standby"]
pub type RUNSTDBY_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FORCELDO` reader - Force LDO Voltage Regulator"]
pub type FORCELDO_R = crate::BitReader;
#[doc = "Field `FORCELDO` writer - Force LDO Voltage Regulator"]
pub type FORCELDO_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 6 - Run in Standby"]
    #[inline(always)]
    pub fn runstdby(&self) -> RUNSTDBY_R {
        RUNSTDBY_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 13 - Force LDO Voltage Regulator"]
    #[inline(always)]
    pub fn forceldo(&self) -> FORCELDO_R {
        FORCELDO_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 6 - Run in Standby"]
    #[inline(always)]
    #[must_use]
    pub fn runstdby(&mut self) -> RUNSTDBY_W<VREG_SPEC, 6> {
        RUNSTDBY_W::new(self)
    }
    #[doc = "Bit 13 - Force LDO Voltage Regulator"]
    #[inline(always)]
    #[must_use]
    pub fn forceldo(&mut self) -> FORCELDO_W<VREG_SPEC, 13> {
        FORCELDO_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Voltage Regulator System (VREG) Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vreg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vreg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VREG_SPEC;
impl crate::RegisterSpec for VREG_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`vreg::R`](R) reader structure"]
impl crate::Readable for VREG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`vreg::W`](W) writer structure"]
impl crate::Writable for VREG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets VREG to value 0"]
impl crate::Resettable for VREG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
