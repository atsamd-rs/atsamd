#[doc = "Register `INTFLAG` reader"]
pub type R = crate::R<IntflagSpec>;
#[doc = "Register `INTFLAG` writer"]
pub type W = crate::W<IntflagSpec>;
#[doc = "Field `OVF` reader - OVF Interrupt Flag"]
pub type OvfR = crate::BitReader;
#[doc = "Field `OVF` writer - OVF Interrupt Flag"]
pub type OvfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERR` reader - ERR Interrupt Flag"]
pub type ErrR = crate::BitReader;
#[doc = "Field `ERR` writer - ERR Interrupt Flag"]
pub type ErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MC0` reader - MC Interrupt Flag 0"]
pub type Mc0R = crate::BitReader;
#[doc = "Field `MC0` writer - MC Interrupt Flag 0"]
pub type Mc0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MC1` reader - MC Interrupt Flag 1"]
pub type Mc1R = crate::BitReader;
#[doc = "Field `MC1` writer - MC Interrupt Flag 1"]
pub type Mc1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - OVF Interrupt Flag"]
    #[inline(always)]
    pub fn ovf(&self) -> OvfR {
        OvfR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ERR Interrupt Flag"]
    #[inline(always)]
    pub fn err(&self) -> ErrR {
        ErrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - MC Interrupt Flag 0"]
    #[inline(always)]
    pub fn mc0(&self) -> Mc0R {
        Mc0R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - MC Interrupt Flag 1"]
    #[inline(always)]
    pub fn mc1(&self) -> Mc1R {
        Mc1R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - OVF Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ovf(&mut self) -> OvfW<IntflagSpec> {
        OvfW::new(self, 0)
    }
    #[doc = "Bit 1 - ERR Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn err(&mut self) -> ErrW<IntflagSpec> {
        ErrW::new(self, 1)
    }
    #[doc = "Bit 4 - MC Interrupt Flag 0"]
    #[inline(always)]
    #[must_use]
    pub fn mc0(&mut self) -> Mc0W<IntflagSpec> {
        Mc0W::new(self, 4)
    }
    #[doc = "Bit 5 - MC Interrupt Flag 1"]
    #[inline(always)]
    #[must_use]
    pub fn mc1(&mut self) -> Mc1W<IntflagSpec> {
        Mc1W::new(self, 5)
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets INTFLAG to value 0"]
impl crate::Resettable for IntflagSpec {
    const RESET_VALUE: u8 = 0;
}
