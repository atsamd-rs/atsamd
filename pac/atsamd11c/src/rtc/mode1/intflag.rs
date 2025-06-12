#[doc = "Register `INTFLAG` reader"]
pub type R = crate::R<IntflagSpec>;
#[doc = "Register `INTFLAG` writer"]
pub type W = crate::W<IntflagSpec>;
#[doc = "Field `CMP0` reader - Compare 0"]
pub type Cmp0R = crate::BitReader;
#[doc = "Field `CMP0` writer - Compare 0"]
pub type Cmp0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMP1` reader - Compare 1"]
pub type Cmp1R = crate::BitReader;
#[doc = "Field `CMP1` writer - Compare 1"]
pub type Cmp1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYNCRDY` reader - Synchronization Ready"]
pub type SyncrdyR = crate::BitReader;
#[doc = "Field `SYNCRDY` writer - Synchronization Ready"]
pub type SyncrdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVF` reader - Overflow"]
pub type OvfR = crate::BitReader;
#[doc = "Field `OVF` writer - Overflow"]
pub type OvfW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Compare 0"]
    #[inline(always)]
    pub fn cmp0(&self) -> Cmp0R {
        Cmp0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Compare 1"]
    #[inline(always)]
    pub fn cmp1(&self) -> Cmp1R {
        Cmp1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 6 - Synchronization Ready"]
    #[inline(always)]
    pub fn syncrdy(&self) -> SyncrdyR {
        SyncrdyR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Overflow"]
    #[inline(always)]
    pub fn ovf(&self) -> OvfR {
        OvfR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Compare 0"]
    #[inline(always)]
    pub fn cmp0(&mut self) -> Cmp0W<IntflagSpec> {
        Cmp0W::new(self, 0)
    }
    #[doc = "Bit 1 - Compare 1"]
    #[inline(always)]
    pub fn cmp1(&mut self) -> Cmp1W<IntflagSpec> {
        Cmp1W::new(self, 1)
    }
    #[doc = "Bit 6 - Synchronization Ready"]
    #[inline(always)]
    pub fn syncrdy(&mut self) -> SyncrdyW<IntflagSpec> {
        SyncrdyW::new(self, 6)
    }
    #[doc = "Bit 7 - Overflow"]
    #[inline(always)]
    pub fn ovf(&mut self) -> OvfW<IntflagSpec> {
        OvfW::new(self, 7)
    }
}
#[doc = "MODE1 Interrupt Flag Status and Clear\n\nYou can [`read`](crate::Reg::read) this register and get [`intflag::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intflag::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
