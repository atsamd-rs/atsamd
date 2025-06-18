#[doc = "Register `INTFLAG` reader"]
pub type R = crate::R<IntflagSpec>;
#[doc = "Register `INTFLAG` writer"]
pub type W = crate::W<IntflagSpec>;
#[doc = "Field `UNDERRUN0` reader - Result 0 Underrun"]
pub type Underrun0R = crate::BitReader;
#[doc = "Field `UNDERRUN0` writer - Result 0 Underrun"]
pub type Underrun0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UNDERRUN1` reader - Result 1 Underrun"]
pub type Underrun1R = crate::BitReader;
#[doc = "Field `UNDERRUN1` writer - Result 1 Underrun"]
pub type Underrun1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EMPTY0` reader - Data Buffer 0 Empty"]
pub type Empty0R = crate::BitReader;
#[doc = "Field `EMPTY0` writer - Data Buffer 0 Empty"]
pub type Empty0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EMPTY1` reader - Data Buffer 1 Empty"]
pub type Empty1R = crate::BitReader;
#[doc = "Field `EMPTY1` writer - Data Buffer 1 Empty"]
pub type Empty1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESRDY0` reader - Result 0 Ready"]
pub type Resrdy0R = crate::BitReader;
#[doc = "Field `RESRDY0` writer - Result 0 Ready"]
pub type Resrdy0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESRDY1` reader - Result 1 Ready"]
pub type Resrdy1R = crate::BitReader;
#[doc = "Field `RESRDY1` writer - Result 1 Ready"]
pub type Resrdy1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVERRUN0` reader - Result 0 Overrun"]
pub type Overrun0R = crate::BitReader;
#[doc = "Field `OVERRUN0` writer - Result 0 Overrun"]
pub type Overrun0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVERRUN1` reader - Result 1 Overrun"]
pub type Overrun1R = crate::BitReader;
#[doc = "Field `OVERRUN1` writer - Result 1 Overrun"]
pub type Overrun1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Result 0 Underrun"]
    #[inline(always)]
    pub fn underrun0(&self) -> Underrun0R {
        Underrun0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Result 1 Underrun"]
    #[inline(always)]
    pub fn underrun1(&self) -> Underrun1R {
        Underrun1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Data Buffer 0 Empty"]
    #[inline(always)]
    pub fn empty0(&self) -> Empty0R {
        Empty0R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Data Buffer 1 Empty"]
    #[inline(always)]
    pub fn empty1(&self) -> Empty1R {
        Empty1R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Result 0 Ready"]
    #[inline(always)]
    pub fn resrdy0(&self) -> Resrdy0R {
        Resrdy0R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Result 1 Ready"]
    #[inline(always)]
    pub fn resrdy1(&self) -> Resrdy1R {
        Resrdy1R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Result 0 Overrun"]
    #[inline(always)]
    pub fn overrun0(&self) -> Overrun0R {
        Overrun0R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Result 1 Overrun"]
    #[inline(always)]
    pub fn overrun1(&self) -> Overrun1R {
        Overrun1R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Result 0 Underrun"]
    #[inline(always)]
    pub fn underrun0(&mut self) -> Underrun0W<IntflagSpec> {
        Underrun0W::new(self, 0)
    }
    #[doc = "Bit 1 - Result 1 Underrun"]
    #[inline(always)]
    pub fn underrun1(&mut self) -> Underrun1W<IntflagSpec> {
        Underrun1W::new(self, 1)
    }
    #[doc = "Bit 2 - Data Buffer 0 Empty"]
    #[inline(always)]
    pub fn empty0(&mut self) -> Empty0W<IntflagSpec> {
        Empty0W::new(self, 2)
    }
    #[doc = "Bit 3 - Data Buffer 1 Empty"]
    #[inline(always)]
    pub fn empty1(&mut self) -> Empty1W<IntflagSpec> {
        Empty1W::new(self, 3)
    }
    #[doc = "Bit 4 - Result 0 Ready"]
    #[inline(always)]
    pub fn resrdy0(&mut self) -> Resrdy0W<IntflagSpec> {
        Resrdy0W::new(self, 4)
    }
    #[doc = "Bit 5 - Result 1 Ready"]
    #[inline(always)]
    pub fn resrdy1(&mut self) -> Resrdy1W<IntflagSpec> {
        Resrdy1W::new(self, 5)
    }
    #[doc = "Bit 6 - Result 0 Overrun"]
    #[inline(always)]
    pub fn overrun0(&mut self) -> Overrun0W<IntflagSpec> {
        Overrun0W::new(self, 6)
    }
    #[doc = "Bit 7 - Result 1 Overrun"]
    #[inline(always)]
    pub fn overrun1(&mut self) -> Overrun1W<IntflagSpec> {
        Overrun1W::new(self, 7)
    }
}
#[doc = "Interrupt Flag Status and Clear\n\nYou can [`read`](crate::Reg::read) this register and get [`intflag::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intflag::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntflagSpec;
impl crate::RegisterSpec for IntflagSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`intflag::R`](R) reader structure"]
impl crate::Readable for IntflagSpec {}
#[doc = "`write(|w| ..)` method takes [`intflag::W`](W) writer structure"]
impl crate::Writable for IntflagSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INTFLAG to value 0"]
impl crate::Resettable for IntflagSpec {}
