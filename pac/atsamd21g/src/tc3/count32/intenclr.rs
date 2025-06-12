#[doc = "Register `INTENCLR` reader"]
pub type R = crate::R<IntenclrSpec>;
#[doc = "Register `INTENCLR` writer"]
pub type W = crate::W<IntenclrSpec>;
#[doc = "Field `OVF` reader - Overflow Interrupt Enable"]
pub type OvfR = crate::BitReader;
#[doc = "Field `OVF` writer - Overflow Interrupt Enable"]
pub type OvfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERR` reader - Error Interrupt Enable"]
pub type ErrR = crate::BitReader;
#[doc = "Field `ERR` writer - Error Interrupt Enable"]
pub type ErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYNCRDY` reader - Synchronization Ready Interrupt Enable"]
pub type SyncrdyR = crate::BitReader;
#[doc = "Field `SYNCRDY` writer - Synchronization Ready Interrupt Enable"]
pub type SyncrdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MC0` reader - Match or Capture Channel 0 Interrupt Enable"]
pub type Mc0R = crate::BitReader;
#[doc = "Field `MC0` writer - Match or Capture Channel 0 Interrupt Enable"]
pub type Mc0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MC1` reader - Match or Capture Channel 1 Interrupt Enable"]
pub type Mc1R = crate::BitReader;
#[doc = "Field `MC1` writer - Match or Capture Channel 1 Interrupt Enable"]
pub type Mc1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn ovf(&self) -> OvfR {
        OvfR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Error Interrupt Enable"]
    #[inline(always)]
    pub fn err(&self) -> ErrR {
        ErrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Synchronization Ready Interrupt Enable"]
    #[inline(always)]
    pub fn syncrdy(&self) -> SyncrdyR {
        SyncrdyR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Match or Capture Channel 0 Interrupt Enable"]
    #[inline(always)]
    pub fn mc0(&self) -> Mc0R {
        Mc0R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Match or Capture Channel 1 Interrupt Enable"]
    #[inline(always)]
    pub fn mc1(&self) -> Mc1R {
        Mc1R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn ovf(&mut self) -> OvfW<IntenclrSpec> {
        OvfW::new(self, 0)
    }
    #[doc = "Bit 1 - Error Interrupt Enable"]
    #[inline(always)]
    pub fn err(&mut self) -> ErrW<IntenclrSpec> {
        ErrW::new(self, 1)
    }
    #[doc = "Bit 3 - Synchronization Ready Interrupt Enable"]
    #[inline(always)]
    pub fn syncrdy(&mut self) -> SyncrdyW<IntenclrSpec> {
        SyncrdyW::new(self, 3)
    }
    #[doc = "Bit 4 - Match or Capture Channel 0 Interrupt Enable"]
    #[inline(always)]
    pub fn mc0(&mut self) -> Mc0W<IntenclrSpec> {
        Mc0W::new(self, 4)
    }
    #[doc = "Bit 5 - Match or Capture Channel 1 Interrupt Enable"]
    #[inline(always)]
    pub fn mc1(&mut self) -> Mc1W<IntenclrSpec> {
        Mc1W::new(self, 5)
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
}
#[doc = "`reset()` method sets INTENCLR to value 0"]
impl crate::Resettable for IntenclrSpec {}
