#[doc = "Register `INTFLAG` reader"]
pub type R = crate::R<IntflagSpec>;
#[doc = "Register `INTFLAG` writer"]
pub type W = crate::W<IntflagSpec>;
#[doc = "Field `OVF` reader - Overflow"]
pub type OvfR = crate::BitReader;
#[doc = "Field `OVF` writer - Overflow"]
pub type OvfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERR` reader - Error"]
pub type ErrR = crate::BitReader;
#[doc = "Field `ERR` writer - Error"]
pub type ErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYNCRDY` reader - Synchronization Ready"]
pub type SyncrdyR = crate::BitReader;
#[doc = "Field `SYNCRDY` writer - Synchronization Ready"]
pub type SyncrdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MC0` reader - Match or Capture Channel 0"]
pub type Mc0R = crate::BitReader;
#[doc = "Field `MC0` writer - Match or Capture Channel 0"]
pub type Mc0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MC1` reader - Match or Capture Channel 1"]
pub type Mc1R = crate::BitReader;
#[doc = "Field `MC1` writer - Match or Capture Channel 1"]
pub type Mc1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Overflow"]
    #[inline(always)]
    pub fn ovf(&self) -> OvfR {
        OvfR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Error"]
    #[inline(always)]
    pub fn err(&self) -> ErrR {
        ErrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Synchronization Ready"]
    #[inline(always)]
    pub fn syncrdy(&self) -> SyncrdyR {
        SyncrdyR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Match or Capture Channel 0"]
    #[inline(always)]
    pub fn mc0(&self) -> Mc0R {
        Mc0R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Match or Capture Channel 1"]
    #[inline(always)]
    pub fn mc1(&self) -> Mc1R {
        Mc1R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Overflow"]
    #[inline(always)]
    pub fn ovf(&mut self) -> OvfW<IntflagSpec> {
        OvfW::new(self, 0)
    }
    #[doc = "Bit 1 - Error"]
    #[inline(always)]
    pub fn err(&mut self) -> ErrW<IntflagSpec> {
        ErrW::new(self, 1)
    }
    #[doc = "Bit 3 - Synchronization Ready"]
    #[inline(always)]
    pub fn syncrdy(&mut self) -> SyncrdyW<IntflagSpec> {
        SyncrdyW::new(self, 3)
    }
    #[doc = "Bit 4 - Match or Capture Channel 0"]
    #[inline(always)]
    pub fn mc0(&mut self) -> Mc0W<IntflagSpec> {
        Mc0W::new(self, 4)
    }
    #[doc = "Bit 5 - Match or Capture Channel 1"]
    #[inline(always)]
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
}
#[doc = "`reset()` method sets INTFLAG to value 0"]
impl crate::Resettable for IntflagSpec {}
