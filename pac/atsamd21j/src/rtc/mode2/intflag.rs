#[doc = "Register `INTFLAG` reader"]
pub type R = crate::R<IntflagSpec>;
#[doc = "Register `INTFLAG` writer"]
pub type W = crate::W<IntflagSpec>;
#[doc = "Field `ALARM0` reader - Alarm 0"]
pub type Alarm0R = crate::BitReader;
#[doc = "Field `ALARM0` writer - Alarm 0"]
pub type Alarm0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYNCRDY` reader - Synchronization Ready"]
pub type SyncrdyR = crate::BitReader;
#[doc = "Field `SYNCRDY` writer - Synchronization Ready"]
pub type SyncrdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVF` reader - Overflow"]
pub type OvfR = crate::BitReader;
#[doc = "Field `OVF` writer - Overflow"]
pub type OvfW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Alarm 0"]
    #[inline(always)]
    pub fn alarm0(&self) -> Alarm0R {
        Alarm0R::new((self.bits & 1) != 0)
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
    #[doc = "Bit 0 - Alarm 0"]
    #[inline(always)]
    #[must_use]
    pub fn alarm0(&mut self) -> Alarm0W<IntflagSpec> {
        Alarm0W::new(self, 0)
    }
    #[doc = "Bit 6 - Synchronization Ready"]
    #[inline(always)]
    #[must_use]
    pub fn syncrdy(&mut self) -> SyncrdyW<IntflagSpec> {
        SyncrdyW::new(self, 6)
    }
    #[doc = "Bit 7 - Overflow"]
    #[inline(always)]
    #[must_use]
    pub fn ovf(&mut self) -> OvfW<IntflagSpec> {
        OvfW::new(self, 7)
    }
}
#[doc = "MODE2 Interrupt Flag Status and Clear\n\nYou can [`read`](crate::Reg::read) this register and get [`intflag::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intflag::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
