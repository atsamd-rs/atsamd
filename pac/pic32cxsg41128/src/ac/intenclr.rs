#[doc = "Register `INTENCLR` reader"]
pub type R = crate::R<IntenclrSpec>;
#[doc = "Register `INTENCLR` writer"]
pub type W = crate::W<IntenclrSpec>;
#[doc = "Field `COMP0` reader - Comparator 0 Interrupt Enable"]
pub type Comp0R = crate::BitReader;
#[doc = "Field `COMP0` writer - Comparator 0 Interrupt Enable"]
pub type Comp0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMP1` reader - Comparator 1 Interrupt Enable"]
pub type Comp1R = crate::BitReader;
#[doc = "Field `COMP1` writer - Comparator 1 Interrupt Enable"]
pub type Comp1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WIN0` reader - Window 0 Interrupt Enable"]
pub type Win0R = crate::BitReader;
#[doc = "Field `WIN0` writer - Window 0 Interrupt Enable"]
pub type Win0W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Comparator 0 Interrupt Enable"]
    #[inline(always)]
    pub fn comp0(&self) -> Comp0R {
        Comp0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Comparator 1 Interrupt Enable"]
    #[inline(always)]
    pub fn comp1(&self) -> Comp1R {
        Comp1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - Window 0 Interrupt Enable"]
    #[inline(always)]
    pub fn win0(&self) -> Win0R {
        Win0R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Comparator 0 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn comp0(&mut self) -> Comp0W<IntenclrSpec> {
        Comp0W::new(self, 0)
    }
    #[doc = "Bit 1 - Comparator 1 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn comp1(&mut self) -> Comp1W<IntenclrSpec> {
        Comp1W::new(self, 1)
    }
    #[doc = "Bit 4 - Window 0 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn win0(&mut self) -> Win0W<IntenclrSpec> {
        Win0W::new(self, 4)
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
