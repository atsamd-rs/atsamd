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
#[doc = "Field `WAKEUPEN8` reader - External Interrupt 8 Wake-up Enable"]
pub type WAKEUPEN8_R = crate::BitReader;
#[doc = "Field `WAKEUPEN8` writer - External Interrupt 8 Wake-up Enable"]
pub type WAKEUPEN8_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `WAKEUPEN9` reader - External Interrupt 9 Wake-up Enable"]
pub type WAKEUPEN9_R = crate::BitReader;
#[doc = "Field `WAKEUPEN9` writer - External Interrupt 9 Wake-up Enable"]
pub type WAKEUPEN9_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `WAKEUPEN10` reader - External Interrupt 10 Wake-up Enable"]
pub type WAKEUPEN10_R = crate::BitReader;
#[doc = "Field `WAKEUPEN10` writer - External Interrupt 10 Wake-up Enable"]
pub type WAKEUPEN10_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `WAKEUPEN11` reader - External Interrupt 11 Wake-up Enable"]
pub type WAKEUPEN11_R = crate::BitReader;
#[doc = "Field `WAKEUPEN11` writer - External Interrupt 11 Wake-up Enable"]
pub type WAKEUPEN11_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `WAKEUPEN12` reader - External Interrupt 12 Wake-up Enable"]
pub type WAKEUPEN12_R = crate::BitReader;
#[doc = "Field `WAKEUPEN12` writer - External Interrupt 12 Wake-up Enable"]
pub type WAKEUPEN12_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `WAKEUPEN13` reader - External Interrupt 13 Wake-up Enable"]
pub type WAKEUPEN13_R = crate::BitReader;
#[doc = "Field `WAKEUPEN13` writer - External Interrupt 13 Wake-up Enable"]
pub type WAKEUPEN13_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `WAKEUPEN14` reader - External Interrupt 14 Wake-up Enable"]
pub type WAKEUPEN14_R = crate::BitReader;
#[doc = "Field `WAKEUPEN14` writer - External Interrupt 14 Wake-up Enable"]
pub type WAKEUPEN14_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `WAKEUPEN15` reader - External Interrupt 15 Wake-up Enable"]
pub type WAKEUPEN15_R = crate::BitReader;
#[doc = "Field `WAKEUPEN15` writer - External Interrupt 15 Wake-up Enable"]
pub type WAKEUPEN15_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
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
    #[doc = "Bit 8 - External Interrupt 8 Wake-up Enable"]
    #[inline(always)]
    pub fn wakeupen8(&self) -> WAKEUPEN8_R {
        WAKEUPEN8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - External Interrupt 9 Wake-up Enable"]
    #[inline(always)]
    pub fn wakeupen9(&self) -> WAKEUPEN9_R {
        WAKEUPEN9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - External Interrupt 10 Wake-up Enable"]
    #[inline(always)]
    pub fn wakeupen10(&self) -> WAKEUPEN10_R {
        WAKEUPEN10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - External Interrupt 11 Wake-up Enable"]
    #[inline(always)]
    pub fn wakeupen11(&self) -> WAKEUPEN11_R {
        WAKEUPEN11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - External Interrupt 12 Wake-up Enable"]
    #[inline(always)]
    pub fn wakeupen12(&self) -> WAKEUPEN12_R {
        WAKEUPEN12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - External Interrupt 13 Wake-up Enable"]
    #[inline(always)]
    pub fn wakeupen13(&self) -> WAKEUPEN13_R {
        WAKEUPEN13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - External Interrupt 14 Wake-up Enable"]
    #[inline(always)]
    pub fn wakeupen14(&self) -> WAKEUPEN14_R {
        WAKEUPEN14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - External Interrupt 15 Wake-up Enable"]
    #[inline(always)]
    pub fn wakeupen15(&self) -> WAKEUPEN15_R {
        WAKEUPEN15_R::new(((self.bits >> 15) & 1) != 0)
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
    #[doc = "Bit 8 - External Interrupt 8 Wake-up Enable"]
    #[inline(always)]
    #[must_use]
    pub fn wakeupen8(&mut self) -> WAKEUPEN8_W<WAKEUP_SPEC, 8> {
        WAKEUPEN8_W::new(self)
    }
    #[doc = "Bit 9 - External Interrupt 9 Wake-up Enable"]
    #[inline(always)]
    #[must_use]
    pub fn wakeupen9(&mut self) -> WAKEUPEN9_W<WAKEUP_SPEC, 9> {
        WAKEUPEN9_W::new(self)
    }
    #[doc = "Bit 10 - External Interrupt 10 Wake-up Enable"]
    #[inline(always)]
    #[must_use]
    pub fn wakeupen10(&mut self) -> WAKEUPEN10_W<WAKEUP_SPEC, 10> {
        WAKEUPEN10_W::new(self)
    }
    #[doc = "Bit 11 - External Interrupt 11 Wake-up Enable"]
    #[inline(always)]
    #[must_use]
    pub fn wakeupen11(&mut self) -> WAKEUPEN11_W<WAKEUP_SPEC, 11> {
        WAKEUPEN11_W::new(self)
    }
    #[doc = "Bit 12 - External Interrupt 12 Wake-up Enable"]
    #[inline(always)]
    #[must_use]
    pub fn wakeupen12(&mut self) -> WAKEUPEN12_W<WAKEUP_SPEC, 12> {
        WAKEUPEN12_W::new(self)
    }
    #[doc = "Bit 13 - External Interrupt 13 Wake-up Enable"]
    #[inline(always)]
    #[must_use]
    pub fn wakeupen13(&mut self) -> WAKEUPEN13_W<WAKEUP_SPEC, 13> {
        WAKEUPEN13_W::new(self)
    }
    #[doc = "Bit 14 - External Interrupt 14 Wake-up Enable"]
    #[inline(always)]
    #[must_use]
    pub fn wakeupen14(&mut self) -> WAKEUPEN14_W<WAKEUP_SPEC, 14> {
        WAKEUPEN14_W::new(self)
    }
    #[doc = "Bit 15 - External Interrupt 15 Wake-up Enable"]
    #[inline(always)]
    #[must_use]
    pub fn wakeupen15(&mut self) -> WAKEUPEN15_W<WAKEUP_SPEC, 15> {
        WAKEUPEN15_W::new(self)
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
