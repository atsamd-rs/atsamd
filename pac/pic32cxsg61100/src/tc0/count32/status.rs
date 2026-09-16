#[doc = "Register `STATUS` reader"]
pub type R = crate::R<StatusSpec>;
#[doc = "Register `STATUS` writer"]
pub type W = crate::W<StatusSpec>;
#[doc = "Field `STOP` reader - Stop Status Flag"]
pub type StopR = crate::BitReader;
#[doc = "Field `STOP` writer - Stop Status Flag"]
pub type StopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLAVE` reader - Slave Status Flag"]
pub type SlaveR = crate::BitReader;
#[doc = "Field `SLAVE` writer - Slave Status Flag"]
pub type SlaveW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PERBUFV` reader - Synchronization Busy Status"]
pub type PerbufvR = crate::BitReader;
#[doc = "Field `PERBUFV` writer - Synchronization Busy Status"]
pub type PerbufvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCBUFV0` reader - Compare channel buffer 0 valid"]
pub type Ccbufv0R = crate::BitReader;
#[doc = "Field `CCBUFV0` writer - Compare channel buffer 0 valid"]
pub type Ccbufv0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCBUFV1` reader - Compare channel buffer 1 valid"]
pub type Ccbufv1R = crate::BitReader;
#[doc = "Field `CCBUFV1` writer - Compare channel buffer 1 valid"]
pub type Ccbufv1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Stop Status Flag"]
    #[inline(always)]
    pub fn stop(&self) -> StopR {
        StopR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Slave Status Flag"]
    #[inline(always)]
    pub fn slave(&self) -> SlaveR {
        SlaveR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Synchronization Busy Status"]
    #[inline(always)]
    pub fn perbufv(&self) -> PerbufvR {
        PerbufvR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Compare channel buffer 0 valid"]
    #[inline(always)]
    pub fn ccbufv0(&self) -> Ccbufv0R {
        Ccbufv0R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Compare channel buffer 1 valid"]
    #[inline(always)]
    pub fn ccbufv1(&self) -> Ccbufv1R {
        Ccbufv1R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Stop Status Flag"]
    #[inline(always)]
    #[must_use]
    pub fn stop(&mut self) -> StopW<StatusSpec> {
        StopW::new(self, 0)
    }
    #[doc = "Bit 1 - Slave Status Flag"]
    #[inline(always)]
    #[must_use]
    pub fn slave(&mut self) -> SlaveW<StatusSpec> {
        SlaveW::new(self, 1)
    }
    #[doc = "Bit 3 - Synchronization Busy Status"]
    #[inline(always)]
    #[must_use]
    pub fn perbufv(&mut self) -> PerbufvW<StatusSpec> {
        PerbufvW::new(self, 3)
    }
    #[doc = "Bit 4 - Compare channel buffer 0 valid"]
    #[inline(always)]
    #[must_use]
    pub fn ccbufv0(&mut self) -> Ccbufv0W<StatusSpec> {
        Ccbufv0W::new(self, 4)
    }
    #[doc = "Bit 5 - Compare channel buffer 1 valid"]
    #[inline(always)]
    #[must_use]
    pub fn ccbufv1(&mut self) -> Ccbufv1W<StatusSpec> {
        Ccbufv1W::new(self, 5)
    }
}
#[doc = "Status\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatusSpec;
impl crate::RegisterSpec for StatusSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for StatusSpec {}
#[doc = "`write(|w| ..)` method takes [`status::W`](W) writer structure"]
impl crate::Writable for StatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets STATUS to value 0x01"]
impl crate::Resettable for StatusSpec {
    const RESET_VALUE: u8 = 0x01;
}
