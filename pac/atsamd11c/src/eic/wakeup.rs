#[doc = "Register `WAKEUP` reader"]
pub type R = crate::R<WAKEUP_SPEC>;
#[doc = "Register `WAKEUP` writer"]
pub type W = crate::W<WAKEUP_SPEC>;
#[doc = "Field `WAKEUPEN0` reader - External Interrupt 0 Wake-up Enable"]
pub type WAKEUPEN0_R = crate::BitReader;
#[doc = "Field `WAKEUPEN0` writer - External Interrupt 0 Wake-up Enable"]
pub type WAKEUPEN0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `WAKEUPEN1` reader - External Interrupt 1 Wake-up Enable"]
pub type WAKEUPEN1_R = crate::BitReader;
#[doc = "Field `WAKEUPEN1` writer - External Interrupt 1 Wake-up Enable"]
pub type WAKEUPEN1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `WAKEUPEN2` reader - External Interrupt 2 Wake-up Enable"]
pub type WAKEUPEN2_R = crate::BitReader;
#[doc = "Field `WAKEUPEN2` writer - External Interrupt 2 Wake-up Enable"]
pub type WAKEUPEN2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `WAKEUPEN3` reader - External Interrupt 3 Wake-up Enable"]
pub type WAKEUPEN3_R = crate::BitReader;
#[doc = "Field `WAKEUPEN3` writer - External Interrupt 3 Wake-up Enable"]
pub type WAKEUPEN3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `WAKEUPEN4` reader - External Interrupt 4 Wake-up Enable"]
pub type WAKEUPEN4_R = crate::BitReader;
#[doc = "Field `WAKEUPEN4` writer - External Interrupt 4 Wake-up Enable"]
pub type WAKEUPEN4_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `WAKEUPEN5` reader - External Interrupt 5 Wake-up Enable"]
pub type WAKEUPEN5_R = crate::BitReader;
#[doc = "Field `WAKEUPEN5` writer - External Interrupt 5 Wake-up Enable"]
pub type WAKEUPEN5_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `WAKEUPEN6` reader - External Interrupt 6 Wake-up Enable"]
pub type WAKEUPEN6_R = crate::BitReader;
#[doc = "Field `WAKEUPEN6` writer - External Interrupt 6 Wake-up Enable"]
pub type WAKEUPEN6_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `WAKEUPEN7` reader - External Interrupt 7 Wake-up Enable"]
pub type WAKEUPEN7_R = crate::BitReader;
#[doc = "Field `WAKEUPEN7` writer - External Interrupt 7 Wake-up Enable"]
pub type WAKEUPEN7_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - External Interrupt 0 Wake-up Enable"]
    #[inline(always)]
    pub fn wakeupen0(&self) -> WAKEUPEN0_R {
        WAKEUPEN0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - External Interrupt 1 Wake-up Enable"]
    #[inline(always)]
    pub fn wakeupen1(&self) -> WAKEUPEN1_R {
        WAKEUPEN1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - External Interrupt 2 Wake-up Enable"]
    #[inline(always)]
    pub fn wakeupen2(&self) -> WAKEUPEN2_R {
        WAKEUPEN2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - External Interrupt 3 Wake-up Enable"]
    #[inline(always)]
    pub fn wakeupen3(&self) -> WAKEUPEN3_R {
        WAKEUPEN3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - External Interrupt 4 Wake-up Enable"]
    #[inline(always)]
    pub fn wakeupen4(&self) -> WAKEUPEN4_R {
        WAKEUPEN4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - External Interrupt 5 Wake-up Enable"]
    #[inline(always)]
    pub fn wakeupen5(&self) -> WAKEUPEN5_R {
        WAKEUPEN5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - External Interrupt 6 Wake-up Enable"]
    #[inline(always)]
    pub fn wakeupen6(&self) -> WAKEUPEN6_R {
        WAKEUPEN6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - External Interrupt 7 Wake-up Enable"]
    #[inline(always)]
    pub fn wakeupen7(&self) -> WAKEUPEN7_R {
        WAKEUPEN7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - External Interrupt 0 Wake-up Enable"]
    #[inline(always)]
    #[must_use]
    pub fn wakeupen0(&mut self) -> WAKEUPEN0_W<WAKEUP_SPEC, 0> {
        WAKEUPEN0_W::new(self)
    }
    #[doc = "Bit 1 - External Interrupt 1 Wake-up Enable"]
    #[inline(always)]
    #[must_use]
    pub fn wakeupen1(&mut self) -> WAKEUPEN1_W<WAKEUP_SPEC, 1> {
        WAKEUPEN1_W::new(self)
    }
    #[doc = "Bit 2 - External Interrupt 2 Wake-up Enable"]
    #[inline(always)]
    #[must_use]
    pub fn wakeupen2(&mut self) -> WAKEUPEN2_W<WAKEUP_SPEC, 2> {
        WAKEUPEN2_W::new(self)
    }
    #[doc = "Bit 3 - External Interrupt 3 Wake-up Enable"]
    #[inline(always)]
    #[must_use]
    pub fn wakeupen3(&mut self) -> WAKEUPEN3_W<WAKEUP_SPEC, 3> {
        WAKEUPEN3_W::new(self)
    }
    #[doc = "Bit 4 - External Interrupt 4 Wake-up Enable"]
    #[inline(always)]
    #[must_use]
    pub fn wakeupen4(&mut self) -> WAKEUPEN4_W<WAKEUP_SPEC, 4> {
        WAKEUPEN4_W::new(self)
    }
    #[doc = "Bit 5 - External Interrupt 5 Wake-up Enable"]
    #[inline(always)]
    #[must_use]
    pub fn wakeupen5(&mut self) -> WAKEUPEN5_W<WAKEUP_SPEC, 5> {
        WAKEUPEN5_W::new(self)
    }
    #[doc = "Bit 6 - External Interrupt 6 Wake-up Enable"]
    #[inline(always)]
    #[must_use]
    pub fn wakeupen6(&mut self) -> WAKEUPEN6_W<WAKEUP_SPEC, 6> {
        WAKEUPEN6_W::new(self)
    }
    #[doc = "Bit 7 - External Interrupt 7 Wake-up Enable"]
    #[inline(always)]
    #[must_use]
    pub fn wakeupen7(&mut self) -> WAKEUPEN7_W<WAKEUP_SPEC, 7> {
        WAKEUPEN7_W::new(self)
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
#[doc = "Wake-Up Enable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wakeup::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wakeup::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WAKEUP_SPEC;
impl crate::RegisterSpec for WAKEUP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wakeup::R`](R) reader structure"]
impl crate::Readable for WAKEUP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`wakeup::W`](W) writer structure"]
impl crate::Writable for WAKEUP_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WAKEUP to value 0"]
impl crate::Resettable for WAKEUP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
