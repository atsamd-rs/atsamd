#[doc = "Register `INTENCLR` reader"]
pub type R = crate::R<IntenclrSpec>;
#[doc = "Register `INTENCLR` writer"]
pub type W = crate::W<IntenclrSpec>;
#[doc = "Field `UNDERRUN0` reader - Underrun 0 Interrupt Enable"]
pub type Underrun0R = crate::BitReader;
#[doc = "Field `UNDERRUN0` writer - Underrun 0 Interrupt Enable"]
pub type Underrun0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UNDERRUN1` reader - Underrun 1 Interrupt Enable"]
pub type Underrun1R = crate::BitReader;
#[doc = "Field `UNDERRUN1` writer - Underrun 1 Interrupt Enable"]
pub type Underrun1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EMPTY0` reader - Data Buffer 0 Empty Interrupt Enable"]
pub type Empty0R = crate::BitReader;
#[doc = "Field `EMPTY0` writer - Data Buffer 0 Empty Interrupt Enable"]
pub type Empty0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EMPTY1` reader - Data Buffer 1 Empty Interrupt Enable"]
pub type Empty1R = crate::BitReader;
#[doc = "Field `EMPTY1` writer - Data Buffer 1 Empty Interrupt Enable"]
pub type Empty1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Underrun 0 Interrupt Enable"]
    #[inline(always)]
    pub fn underrun0(&self) -> Underrun0R {
        Underrun0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Underrun 1 Interrupt Enable"]
    #[inline(always)]
    pub fn underrun1(&self) -> Underrun1R {
        Underrun1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Data Buffer 0 Empty Interrupt Enable"]
    #[inline(always)]
    pub fn empty0(&self) -> Empty0R {
        Empty0R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Data Buffer 1 Empty Interrupt Enable"]
    #[inline(always)]
    pub fn empty1(&self) -> Empty1R {
        Empty1R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Underrun 0 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn underrun0(&mut self) -> Underrun0W<IntenclrSpec> {
        Underrun0W::new(self, 0)
    }
    #[doc = "Bit 1 - Underrun 1 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn underrun1(&mut self) -> Underrun1W<IntenclrSpec> {
        Underrun1W::new(self, 1)
    }
    #[doc = "Bit 2 - Data Buffer 0 Empty Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn empty0(&mut self) -> Empty0W<IntenclrSpec> {
        Empty0W::new(self, 2)
    }
    #[doc = "Bit 3 - Data Buffer 1 Empty Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn empty1(&mut self) -> Empty1W<IntenclrSpec> {
        Empty1W::new(self, 3)
    }
}
#[doc = "Interrupt Enable Clear\n\nYou can [`read`](crate::Reg::read) this register and get [`intenclr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenclr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntenclrSpec;
impl crate::RegisterSpec for IntenclrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`intenclr::R`](R) reader structure"]
impl crate::Readable for IntenclrSpec {}
#[doc = "`write(|w| ..)` method takes [`intenclr::W`](W) writer structure"]
impl crate::Writable for IntenclrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets INTENCLR to value 0"]
impl crate::Resettable for IntenclrSpec {
    const RESET_VALUE: u8 = 0;
}
