#[doc = "Register `ILE` reader"]
pub type R = crate::R<IleSpec>;
#[doc = "Register `ILE` writer"]
pub type W = crate::W<IleSpec>;
#[doc = "Field `EINT0` reader - Enable Interrupt Line 0"]
pub type Eint0R = crate::BitReader;
#[doc = "Field `EINT0` writer - Enable Interrupt Line 0"]
pub type Eint0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EINT1` reader - Enable Interrupt Line 1"]
pub type Eint1R = crate::BitReader;
#[doc = "Field `EINT1` writer - Enable Interrupt Line 1"]
pub type Eint1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable Interrupt Line 0"]
    #[inline(always)]
    pub fn eint0(&self) -> Eint0R {
        Eint0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable Interrupt Line 1"]
    #[inline(always)]
    pub fn eint1(&self) -> Eint1R {
        Eint1R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable Interrupt Line 0"]
    #[inline(always)]
    #[must_use]
    pub fn eint0(&mut self) -> Eint0W<IleSpec> {
        Eint0W::new(self, 0)
    }
    #[doc = "Bit 1 - Enable Interrupt Line 1"]
    #[inline(always)]
    #[must_use]
    pub fn eint1(&mut self) -> Eint1W<IleSpec> {
        Eint1W::new(self, 1)
    }
}
#[doc = "Interrupt Line Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`ile::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ile::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IleSpec;
impl crate::RegisterSpec for IleSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ile::R`](R) reader structure"]
impl crate::Readable for IleSpec {}
#[doc = "`write(|w| ..)` method takes [`ile::W`](W) writer structure"]
impl crate::Writable for IleSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ILE to value 0"]
impl crate::Resettable for IleSpec {
    const RESET_VALUE: u32 = 0;
}
