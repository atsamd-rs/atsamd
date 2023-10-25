#[doc = "Register `INTFLAG` reader"]
pub type R = crate::R<INTFLAG_SPEC>;
#[doc = "Register `INTFLAG` writer"]
pub type W = crate::W<INTFLAG_SPEC>;
#[doc = "Field `EXTINT0` reader - External Interrupt 0"]
pub type EXTINT0_R = crate::BitReader;
#[doc = "Field `EXTINT0` writer - External Interrupt 0"]
pub type EXTINT0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EXTINT1` reader - External Interrupt 1"]
pub type EXTINT1_R = crate::BitReader;
#[doc = "Field `EXTINT1` writer - External Interrupt 1"]
pub type EXTINT1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EXTINT2` reader - External Interrupt 2"]
pub type EXTINT2_R = crate::BitReader;
#[doc = "Field `EXTINT2` writer - External Interrupt 2"]
pub type EXTINT2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EXTINT3` reader - External Interrupt 3"]
pub type EXTINT3_R = crate::BitReader;
#[doc = "Field `EXTINT3` writer - External Interrupt 3"]
pub type EXTINT3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EXTINT4` reader - External Interrupt 4"]
pub type EXTINT4_R = crate::BitReader;
#[doc = "Field `EXTINT4` writer - External Interrupt 4"]
pub type EXTINT4_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EXTINT5` reader - External Interrupt 5"]
pub type EXTINT5_R = crate::BitReader;
#[doc = "Field `EXTINT5` writer - External Interrupt 5"]
pub type EXTINT5_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EXTINT6` reader - External Interrupt 6"]
pub type EXTINT6_R = crate::BitReader;
#[doc = "Field `EXTINT6` writer - External Interrupt 6"]
pub type EXTINT6_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EXTINT7` reader - External Interrupt 7"]
pub type EXTINT7_R = crate::BitReader;
#[doc = "Field `EXTINT7` writer - External Interrupt 7"]
pub type EXTINT7_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - External Interrupt 0"]
    #[inline(always)]
    pub fn extint0(&self) -> EXTINT0_R {
        EXTINT0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - External Interrupt 1"]
    #[inline(always)]
    pub fn extint1(&self) -> EXTINT1_R {
        EXTINT1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - External Interrupt 2"]
    #[inline(always)]
    pub fn extint2(&self) -> EXTINT2_R {
        EXTINT2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - External Interrupt 3"]
    #[inline(always)]
    pub fn extint3(&self) -> EXTINT3_R {
        EXTINT3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - External Interrupt 4"]
    #[inline(always)]
    pub fn extint4(&self) -> EXTINT4_R {
        EXTINT4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - External Interrupt 5"]
    #[inline(always)]
    pub fn extint5(&self) -> EXTINT5_R {
        EXTINT5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - External Interrupt 6"]
    #[inline(always)]
    pub fn extint6(&self) -> EXTINT6_R {
        EXTINT6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - External Interrupt 7"]
    #[inline(always)]
    pub fn extint7(&self) -> EXTINT7_R {
        EXTINT7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - External Interrupt 0"]
    #[inline(always)]
    #[must_use]
    pub fn extint0(&mut self) -> EXTINT0_W<INTFLAG_SPEC, 0> {
        EXTINT0_W::new(self)
    }
    #[doc = "Bit 1 - External Interrupt 1"]
    #[inline(always)]
    #[must_use]
    pub fn extint1(&mut self) -> EXTINT1_W<INTFLAG_SPEC, 1> {
        EXTINT1_W::new(self)
    }
    #[doc = "Bit 2 - External Interrupt 2"]
    #[inline(always)]
    #[must_use]
    pub fn extint2(&mut self) -> EXTINT2_W<INTFLAG_SPEC, 2> {
        EXTINT2_W::new(self)
    }
    #[doc = "Bit 3 - External Interrupt 3"]
    #[inline(always)]
    #[must_use]
    pub fn extint3(&mut self) -> EXTINT3_W<INTFLAG_SPEC, 3> {
        EXTINT3_W::new(self)
    }
    #[doc = "Bit 4 - External Interrupt 4"]
    #[inline(always)]
    #[must_use]
    pub fn extint4(&mut self) -> EXTINT4_W<INTFLAG_SPEC, 4> {
        EXTINT4_W::new(self)
    }
    #[doc = "Bit 5 - External Interrupt 5"]
    #[inline(always)]
    #[must_use]
    pub fn extint5(&mut self) -> EXTINT5_W<INTFLAG_SPEC, 5> {
        EXTINT5_W::new(self)
    }
    #[doc = "Bit 6 - External Interrupt 6"]
    #[inline(always)]
    #[must_use]
    pub fn extint6(&mut self) -> EXTINT6_W<INTFLAG_SPEC, 6> {
        EXTINT6_W::new(self)
    }
    #[doc = "Bit 7 - External Interrupt 7"]
    #[inline(always)]
    #[must_use]
    pub fn extint7(&mut self) -> EXTINT7_W<INTFLAG_SPEC, 7> {
        EXTINT7_W::new(self)
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
#[doc = "Interrupt Flag Status and Clear\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intflag::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intflag::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTFLAG_SPEC;
impl crate::RegisterSpec for INTFLAG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intflag::R`](R) reader structure"]
impl crate::Readable for INTFLAG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`intflag::W`](W) writer structure"]
impl crate::Writable for INTFLAG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTFLAG to value 0"]
impl crate::Resettable for INTFLAG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
