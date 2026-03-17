#[doc = "Register `INTFLAG` reader"]
pub type R = crate::R<IntflagSpec>;
#[doc = "Register `INTFLAG` writer"]
pub type W = crate::W<IntflagSpec>;
#[doc = "Field `BOD33RDY` reader - BOD33 Ready"]
pub type Bod33rdyR = crate::BitReader;
#[doc = "Field `BOD33RDY` writer - BOD33 Ready"]
pub type Bod33rdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BOD33DET` reader - BOD33 Detection"]
pub type Bod33detR = crate::BitReader;
#[doc = "Field `BOD33DET` writer - BOD33 Detection"]
pub type Bod33detW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `B33SRDY` reader - BOD33 Synchronization Ready"]
pub type B33srdyR = crate::BitReader;
#[doc = "Field `B33SRDY` writer - BOD33 Synchronization Ready"]
pub type B33srdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VREGRDY` reader - Voltage Regulator Ready"]
pub type VregrdyR = crate::BitReader;
#[doc = "Field `VREGRDY` writer - Voltage Regulator Ready"]
pub type VregrdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VCORERDY` reader - VDDCORE Ready"]
pub type VcorerdyR = crate::BitReader;
#[doc = "Field `VCORERDY` writer - VDDCORE Ready"]
pub type VcorerdyW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - BOD33 Ready"]
    #[inline(always)]
    pub fn bod33rdy(&self) -> Bod33rdyR {
        Bod33rdyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - BOD33 Detection"]
    #[inline(always)]
    pub fn bod33det(&self) -> Bod33detR {
        Bod33detR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - BOD33 Synchronization Ready"]
    #[inline(always)]
    pub fn b33srdy(&self) -> B33srdyR {
        B33srdyR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - Voltage Regulator Ready"]
    #[inline(always)]
    pub fn vregrdy(&self) -> VregrdyR {
        VregrdyR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - VDDCORE Ready"]
    #[inline(always)]
    pub fn vcorerdy(&self) -> VcorerdyR {
        VcorerdyR::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - BOD33 Ready"]
    #[inline(always)]
    #[must_use]
    pub fn bod33rdy(&mut self) -> Bod33rdyW<IntflagSpec> {
        Bod33rdyW::new(self, 0)
    }
    #[doc = "Bit 1 - BOD33 Detection"]
    #[inline(always)]
    #[must_use]
    pub fn bod33det(&mut self) -> Bod33detW<IntflagSpec> {
        Bod33detW::new(self, 1)
    }
    #[doc = "Bit 2 - BOD33 Synchronization Ready"]
    #[inline(always)]
    #[must_use]
    pub fn b33srdy(&mut self) -> B33srdyW<IntflagSpec> {
        B33srdyW::new(self, 2)
    }
    #[doc = "Bit 8 - Voltage Regulator Ready"]
    #[inline(always)]
    #[must_use]
    pub fn vregrdy(&mut self) -> VregrdyW<IntflagSpec> {
        VregrdyW::new(self, 8)
    }
    #[doc = "Bit 10 - VDDCORE Ready"]
    #[inline(always)]
    #[must_use]
    pub fn vcorerdy(&mut self) -> VcorerdyW<IntflagSpec> {
        VcorerdyW::new(self, 10)
    }
}
#[doc = "Interrupt Flag Status and Clear\n\nYou can [`read`](crate::Reg::read) this register and get [`intflag::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intflag::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntflagSpec;
impl crate::RegisterSpec for IntflagSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intflag::R`](R) reader structure"]
impl crate::Readable for IntflagSpec {}
#[doc = "`write(|w| ..)` method takes [`intflag::W`](W) writer structure"]
impl crate::Writable for IntflagSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTFLAG to value 0"]
impl crate::Resettable for IntflagSpec {
    const RESET_VALUE: u32 = 0;
}
