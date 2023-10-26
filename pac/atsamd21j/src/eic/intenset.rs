#[doc = "Register `INTENSET` reader"]
pub type R = crate::R<INTENSET_SPEC>;
#[doc = "Register `INTENSET` writer"]
pub type W = crate::W<INTENSET_SPEC>;
#[doc = "Field `EXTINT0` reader - External Interrupt 0 Enable"]
pub type EXTINT0_R = crate::BitReader;
#[doc = "Field `EXTINT0` writer - External Interrupt 0 Enable"]
pub type EXTINT0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EXTINT1` reader - External Interrupt 1 Enable"]
pub type EXTINT1_R = crate::BitReader;
#[doc = "Field `EXTINT1` writer - External Interrupt 1 Enable"]
pub type EXTINT1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EXTINT2` reader - External Interrupt 2 Enable"]
pub type EXTINT2_R = crate::BitReader;
#[doc = "Field `EXTINT2` writer - External Interrupt 2 Enable"]
pub type EXTINT2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EXTINT3` reader - External Interrupt 3 Enable"]
pub type EXTINT3_R = crate::BitReader;
#[doc = "Field `EXTINT3` writer - External Interrupt 3 Enable"]
pub type EXTINT3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EXTINT4` reader - External Interrupt 4 Enable"]
pub type EXTINT4_R = crate::BitReader;
#[doc = "Field `EXTINT4` writer - External Interrupt 4 Enable"]
pub type EXTINT4_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EXTINT5` reader - External Interrupt 5 Enable"]
pub type EXTINT5_R = crate::BitReader;
#[doc = "Field `EXTINT5` writer - External Interrupt 5 Enable"]
pub type EXTINT5_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EXTINT6` reader - External Interrupt 6 Enable"]
pub type EXTINT6_R = crate::BitReader;
#[doc = "Field `EXTINT6` writer - External Interrupt 6 Enable"]
pub type EXTINT6_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EXTINT7` reader - External Interrupt 7 Enable"]
pub type EXTINT7_R = crate::BitReader;
#[doc = "Field `EXTINT7` writer - External Interrupt 7 Enable"]
pub type EXTINT7_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EXTINT8` reader - External Interrupt 8 Enable"]
pub type EXTINT8_R = crate::BitReader;
#[doc = "Field `EXTINT8` writer - External Interrupt 8 Enable"]
pub type EXTINT8_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EXTINT9` reader - External Interrupt 9 Enable"]
pub type EXTINT9_R = crate::BitReader;
#[doc = "Field `EXTINT9` writer - External Interrupt 9 Enable"]
pub type EXTINT9_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EXTINT10` reader - External Interrupt 10 Enable"]
pub type EXTINT10_R = crate::BitReader;
#[doc = "Field `EXTINT10` writer - External Interrupt 10 Enable"]
pub type EXTINT10_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EXTINT11` reader - External Interrupt 11 Enable"]
pub type EXTINT11_R = crate::BitReader;
#[doc = "Field `EXTINT11` writer - External Interrupt 11 Enable"]
pub type EXTINT11_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EXTINT12` reader - External Interrupt 12 Enable"]
pub type EXTINT12_R = crate::BitReader;
#[doc = "Field `EXTINT12` writer - External Interrupt 12 Enable"]
pub type EXTINT12_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EXTINT13` reader - External Interrupt 13 Enable"]
pub type EXTINT13_R = crate::BitReader;
#[doc = "Field `EXTINT13` writer - External Interrupt 13 Enable"]
pub type EXTINT13_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EXTINT14` reader - External Interrupt 14 Enable"]
pub type EXTINT14_R = crate::BitReader;
#[doc = "Field `EXTINT14` writer - External Interrupt 14 Enable"]
pub type EXTINT14_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EXTINT15` reader - External Interrupt 15 Enable"]
pub type EXTINT15_R = crate::BitReader;
#[doc = "Field `EXTINT15` writer - External Interrupt 15 Enable"]
pub type EXTINT15_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - External Interrupt 0 Enable"]
    #[inline(always)]
    pub fn extint0(&self) -> EXTINT0_R {
        EXTINT0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - External Interrupt 1 Enable"]
    #[inline(always)]
    pub fn extint1(&self) -> EXTINT1_R {
        EXTINT1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - External Interrupt 2 Enable"]
    #[inline(always)]
    pub fn extint2(&self) -> EXTINT2_R {
        EXTINT2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - External Interrupt 3 Enable"]
    #[inline(always)]
    pub fn extint3(&self) -> EXTINT3_R {
        EXTINT3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - External Interrupt 4 Enable"]
    #[inline(always)]
    pub fn extint4(&self) -> EXTINT4_R {
        EXTINT4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - External Interrupt 5 Enable"]
    #[inline(always)]
    pub fn extint5(&self) -> EXTINT5_R {
        EXTINT5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - External Interrupt 6 Enable"]
    #[inline(always)]
    pub fn extint6(&self) -> EXTINT6_R {
        EXTINT6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - External Interrupt 7 Enable"]
    #[inline(always)]
    pub fn extint7(&self) -> EXTINT7_R {
        EXTINT7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - External Interrupt 8 Enable"]
    #[inline(always)]
    pub fn extint8(&self) -> EXTINT8_R {
        EXTINT8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - External Interrupt 9 Enable"]
    #[inline(always)]
    pub fn extint9(&self) -> EXTINT9_R {
        EXTINT9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - External Interrupt 10 Enable"]
    #[inline(always)]
    pub fn extint10(&self) -> EXTINT10_R {
        EXTINT10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - External Interrupt 11 Enable"]
    #[inline(always)]
    pub fn extint11(&self) -> EXTINT11_R {
        EXTINT11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - External Interrupt 12 Enable"]
    #[inline(always)]
    pub fn extint12(&self) -> EXTINT12_R {
        EXTINT12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - External Interrupt 13 Enable"]
    #[inline(always)]
    pub fn extint13(&self) -> EXTINT13_R {
        EXTINT13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - External Interrupt 14 Enable"]
    #[inline(always)]
    pub fn extint14(&self) -> EXTINT14_R {
        EXTINT14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - External Interrupt 15 Enable"]
    #[inline(always)]
    pub fn extint15(&self) -> EXTINT15_R {
        EXTINT15_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - External Interrupt 0 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn extint0(&mut self) -> EXTINT0_W<INTENSET_SPEC, 0> {
        EXTINT0_W::new(self)
    }
    #[doc = "Bit 1 - External Interrupt 1 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn extint1(&mut self) -> EXTINT1_W<INTENSET_SPEC, 1> {
        EXTINT1_W::new(self)
    }
    #[doc = "Bit 2 - External Interrupt 2 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn extint2(&mut self) -> EXTINT2_W<INTENSET_SPEC, 2> {
        EXTINT2_W::new(self)
    }
    #[doc = "Bit 3 - External Interrupt 3 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn extint3(&mut self) -> EXTINT3_W<INTENSET_SPEC, 3> {
        EXTINT3_W::new(self)
    }
    #[doc = "Bit 4 - External Interrupt 4 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn extint4(&mut self) -> EXTINT4_W<INTENSET_SPEC, 4> {
        EXTINT4_W::new(self)
    }
    #[doc = "Bit 5 - External Interrupt 5 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn extint5(&mut self) -> EXTINT5_W<INTENSET_SPEC, 5> {
        EXTINT5_W::new(self)
    }
    #[doc = "Bit 6 - External Interrupt 6 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn extint6(&mut self) -> EXTINT6_W<INTENSET_SPEC, 6> {
        EXTINT6_W::new(self)
    }
    #[doc = "Bit 7 - External Interrupt 7 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn extint7(&mut self) -> EXTINT7_W<INTENSET_SPEC, 7> {
        EXTINT7_W::new(self)
    }
    #[doc = "Bit 8 - External Interrupt 8 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn extint8(&mut self) -> EXTINT8_W<INTENSET_SPEC, 8> {
        EXTINT8_W::new(self)
    }
    #[doc = "Bit 9 - External Interrupt 9 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn extint9(&mut self) -> EXTINT9_W<INTENSET_SPEC, 9> {
        EXTINT9_W::new(self)
    }
    #[doc = "Bit 10 - External Interrupt 10 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn extint10(&mut self) -> EXTINT10_W<INTENSET_SPEC, 10> {
        EXTINT10_W::new(self)
    }
    #[doc = "Bit 11 - External Interrupt 11 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn extint11(&mut self) -> EXTINT11_W<INTENSET_SPEC, 11> {
        EXTINT11_W::new(self)
    }
    #[doc = "Bit 12 - External Interrupt 12 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn extint12(&mut self) -> EXTINT12_W<INTENSET_SPEC, 12> {
        EXTINT12_W::new(self)
    }
    #[doc = "Bit 13 - External Interrupt 13 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn extint13(&mut self) -> EXTINT13_W<INTENSET_SPEC, 13> {
        EXTINT13_W::new(self)
    }
    #[doc = "Bit 14 - External Interrupt 14 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn extint14(&mut self) -> EXTINT14_W<INTENSET_SPEC, 14> {
        EXTINT14_W::new(self)
    }
    #[doc = "Bit 15 - External Interrupt 15 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn extint15(&mut self) -> EXTINT15_W<INTENSET_SPEC, 15> {
        EXTINT15_W::new(self)
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
#[doc = "Interrupt Enable Set\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intenset::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intenset::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTENSET_SPEC;
impl crate::RegisterSpec for INTENSET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intenset::R`](R) reader structure"]
impl crate::Readable for INTENSET_SPEC {}
#[doc = "`write(|w| ..)` method takes [`intenset::W`](W) writer structure"]
impl crate::Writable for INTENSET_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTENSET to value 0"]
impl crate::Resettable for INTENSET_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
