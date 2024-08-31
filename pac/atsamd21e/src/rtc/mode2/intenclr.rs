#[doc = "Register `INTENCLR` reader"]
pub type R = crate::R<IntenclrSpec>;
#[doc = "Register `INTENCLR` writer"]
pub type W = crate::W<IntenclrSpec>;
#[doc = "Field `ALARM0` reader - Alarm 0 Interrupt Enable"]
pub type Alarm0R = crate::BitReader;
#[doc = "Field `ALARM0` writer - Alarm 0 Interrupt Enable"]
pub type Alarm0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYNCRDY` reader - Synchronization Ready Interrupt Enable"]
pub type SyncrdyR = crate::BitReader;
#[doc = "Field `SYNCRDY` writer - Synchronization Ready Interrupt Enable"]
pub type SyncrdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVF` reader - Overflow Interrupt Enable"]
pub type OvfR = crate::BitReader;
#[doc = "Field `OVF` writer - Overflow Interrupt Enable"]
pub type OvfW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Alarm 0 Interrupt Enable"]
    #[inline(always)]
    pub fn alarm0(&self) -> Alarm0R {
        Alarm0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 6 - Synchronization Ready Interrupt Enable"]
    #[inline(always)]
    pub fn syncrdy(&self) -> SyncrdyR {
        SyncrdyR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn ovf(&self) -> OvfR {
        OvfR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Alarm 0 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn alarm0(&mut self) -> Alarm0W<IntenclrSpec> {
        Alarm0W::new(self, 0)
    }
    #[doc = "Bit 6 - Synchronization Ready Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn syncrdy(&mut self) -> SyncrdyW<IntenclrSpec> {
        SyncrdyW::new(self, 6)
    }
    #[doc = "Bit 7 - Overflow Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ovf(&mut self) -> OvfW<IntenclrSpec> {
        OvfW::new(self, 7)
    }
}
#[doc = "MODE2 Interrupt Enable Clear\n\nYou can [`read`](crate::Reg::read) this register and get [`intenclr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenclr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
