#[doc = "Register `INTFLAG` reader"]
pub type R = crate::R<INTFLAG_SPEC>;
#[doc = "Register `INTFLAG` writer"]
pub type W = crate::W<INTFLAG_SPEC>;
#[doc = "Field `UNDERRUN0` reader - Result 0 Underrun"]
pub type UNDERRUN0_R = crate::BitReader;
#[doc = "Field `UNDERRUN0` writer - Result 0 Underrun"]
pub type UNDERRUN0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `UNDERRUN1` reader - Result 1 Underrun"]
pub type UNDERRUN1_R = crate::BitReader;
#[doc = "Field `UNDERRUN1` writer - Result 1 Underrun"]
pub type UNDERRUN1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EMPTY0` reader - Data Buffer 0 Empty"]
pub type EMPTY0_R = crate::BitReader;
#[doc = "Field `EMPTY0` writer - Data Buffer 0 Empty"]
pub type EMPTY0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EMPTY1` reader - Data Buffer 1 Empty"]
pub type EMPTY1_R = crate::BitReader;
#[doc = "Field `EMPTY1` writer - Data Buffer 1 Empty"]
pub type EMPTY1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RESRDY0` reader - Result 0 Ready"]
pub type RESRDY0_R = crate::BitReader;
#[doc = "Field `RESRDY0` writer - Result 0 Ready"]
pub type RESRDY0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RESRDY1` reader - Result 1 Ready"]
pub type RESRDY1_R = crate::BitReader;
#[doc = "Field `RESRDY1` writer - Result 1 Ready"]
pub type RESRDY1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OVERRUN0` reader - Result 0 Overrun"]
pub type OVERRUN0_R = crate::BitReader;
#[doc = "Field `OVERRUN0` writer - Result 0 Overrun"]
pub type OVERRUN0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OVERRUN1` reader - Result 1 Overrun"]
pub type OVERRUN1_R = crate::BitReader;
#[doc = "Field `OVERRUN1` writer - Result 1 Overrun"]
pub type OVERRUN1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Result 0 Underrun"]
    #[inline(always)]
    pub fn underrun0(&self) -> UNDERRUN0_R {
        UNDERRUN0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Result 1 Underrun"]
    #[inline(always)]
    pub fn underrun1(&self) -> UNDERRUN1_R {
        UNDERRUN1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Data Buffer 0 Empty"]
    #[inline(always)]
    pub fn empty0(&self) -> EMPTY0_R {
        EMPTY0_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Data Buffer 1 Empty"]
    #[inline(always)]
    pub fn empty1(&self) -> EMPTY1_R {
        EMPTY1_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Result 0 Ready"]
    #[inline(always)]
    pub fn resrdy0(&self) -> RESRDY0_R {
        RESRDY0_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Result 1 Ready"]
    #[inline(always)]
    pub fn resrdy1(&self) -> RESRDY1_R {
        RESRDY1_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Result 0 Overrun"]
    #[inline(always)]
    pub fn overrun0(&self) -> OVERRUN0_R {
        OVERRUN0_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Result 1 Overrun"]
    #[inline(always)]
    pub fn overrun1(&self) -> OVERRUN1_R {
        OVERRUN1_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Result 0 Underrun"]
    #[inline(always)]
    #[must_use]
    pub fn underrun0(&mut self) -> UNDERRUN0_W<INTFLAG_SPEC, 0> {
        UNDERRUN0_W::new(self)
    }
    #[doc = "Bit 1 - Result 1 Underrun"]
    #[inline(always)]
    #[must_use]
    pub fn underrun1(&mut self) -> UNDERRUN1_W<INTFLAG_SPEC, 1> {
        UNDERRUN1_W::new(self)
    }
    #[doc = "Bit 2 - Data Buffer 0 Empty"]
    #[inline(always)]
    #[must_use]
    pub fn empty0(&mut self) -> EMPTY0_W<INTFLAG_SPEC, 2> {
        EMPTY0_W::new(self)
    }
    #[doc = "Bit 3 - Data Buffer 1 Empty"]
    #[inline(always)]
    #[must_use]
    pub fn empty1(&mut self) -> EMPTY1_W<INTFLAG_SPEC, 3> {
        EMPTY1_W::new(self)
    }
    #[doc = "Bit 4 - Result 0 Ready"]
    #[inline(always)]
    #[must_use]
    pub fn resrdy0(&mut self) -> RESRDY0_W<INTFLAG_SPEC, 4> {
        RESRDY0_W::new(self)
    }
    #[doc = "Bit 5 - Result 1 Ready"]
    #[inline(always)]
    #[must_use]
    pub fn resrdy1(&mut self) -> RESRDY1_W<INTFLAG_SPEC, 5> {
        RESRDY1_W::new(self)
    }
    #[doc = "Bit 6 - Result 0 Overrun"]
    #[inline(always)]
    #[must_use]
    pub fn overrun0(&mut self) -> OVERRUN0_W<INTFLAG_SPEC, 6> {
        OVERRUN0_W::new(self)
    }
    #[doc = "Bit 7 - Result 1 Overrun"]
    #[inline(always)]
    #[must_use]
    pub fn overrun1(&mut self) -> OVERRUN1_W<INTFLAG_SPEC, 7> {
        OVERRUN1_W::new(self)
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
#[doc = "Interrupt Flag Status and Clear\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intflag::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intflag::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTFLAG_SPEC;
impl crate::RegisterSpec for INTFLAG_SPEC {
    type Ux = u8;
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
