#[doc = "Register `WEXCTRL` reader"]
pub type R = crate::R<WexctrlSpec>;
#[doc = "Register `WEXCTRL` writer"]
pub type W = crate::W<WexctrlSpec>;
#[doc = "Field `OTMX` reader - Output Matrix"]
pub type OtmxR = crate::FieldReader;
#[doc = "Field `OTMX` writer - Output Matrix"]
pub type OtmxW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DTIEN0` reader - Dead-time Insertion Generator 0 Enable"]
pub type Dtien0R = crate::BitReader;
#[doc = "Field `DTIEN0` writer - Dead-time Insertion Generator 0 Enable"]
pub type Dtien0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTIEN1` reader - Dead-time Insertion Generator 1 Enable"]
pub type Dtien1R = crate::BitReader;
#[doc = "Field `DTIEN1` writer - Dead-time Insertion Generator 1 Enable"]
pub type Dtien1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTIEN2` reader - Dead-time Insertion Generator 2 Enable"]
pub type Dtien2R = crate::BitReader;
#[doc = "Field `DTIEN2` writer - Dead-time Insertion Generator 2 Enable"]
pub type Dtien2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTIEN3` reader - Dead-time Insertion Generator 3 Enable"]
pub type Dtien3R = crate::BitReader;
#[doc = "Field `DTIEN3` writer - Dead-time Insertion Generator 3 Enable"]
pub type Dtien3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTLS` reader - Dead-time Low Side Outputs Value"]
pub type DtlsR = crate::FieldReader;
#[doc = "Field `DTLS` writer - Dead-time Low Side Outputs Value"]
pub type DtlsW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DTHS` reader - Dead-time High Side Outputs Value"]
pub type DthsR = crate::FieldReader;
#[doc = "Field `DTHS` writer - Dead-time High Side Outputs Value"]
pub type DthsW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:1 - Output Matrix"]
    #[inline(always)]
    pub fn otmx(&self) -> OtmxR {
        OtmxR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 8 - Dead-time Insertion Generator 0 Enable"]
    #[inline(always)]
    pub fn dtien0(&self) -> Dtien0R {
        Dtien0R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Dead-time Insertion Generator 1 Enable"]
    #[inline(always)]
    pub fn dtien1(&self) -> Dtien1R {
        Dtien1R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Dead-time Insertion Generator 2 Enable"]
    #[inline(always)]
    pub fn dtien2(&self) -> Dtien2R {
        Dtien2R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Dead-time Insertion Generator 3 Enable"]
    #[inline(always)]
    pub fn dtien3(&self) -> Dtien3R {
        Dtien3R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 16:23 - Dead-time Low Side Outputs Value"]
    #[inline(always)]
    pub fn dtls(&self) -> DtlsR {
        DtlsR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Dead-time High Side Outputs Value"]
    #[inline(always)]
    pub fn dths(&self) -> DthsR {
        DthsR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Output Matrix"]
    #[inline(always)]
    #[must_use]
    pub fn otmx(&mut self) -> OtmxW<WexctrlSpec> {
        OtmxW::new(self, 0)
    }
    #[doc = "Bit 8 - Dead-time Insertion Generator 0 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dtien0(&mut self) -> Dtien0W<WexctrlSpec> {
        Dtien0W::new(self, 8)
    }
    #[doc = "Bit 9 - Dead-time Insertion Generator 1 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dtien1(&mut self) -> Dtien1W<WexctrlSpec> {
        Dtien1W::new(self, 9)
    }
    #[doc = "Bit 10 - Dead-time Insertion Generator 2 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dtien2(&mut self) -> Dtien2W<WexctrlSpec> {
        Dtien2W::new(self, 10)
    }
    #[doc = "Bit 11 - Dead-time Insertion Generator 3 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dtien3(&mut self) -> Dtien3W<WexctrlSpec> {
        Dtien3W::new(self, 11)
    }
    #[doc = "Bits 16:23 - Dead-time Low Side Outputs Value"]
    #[inline(always)]
    #[must_use]
    pub fn dtls(&mut self) -> DtlsW<WexctrlSpec> {
        DtlsW::new(self, 16)
    }
    #[doc = "Bits 24:31 - Dead-time High Side Outputs Value"]
    #[inline(always)]
    #[must_use]
    pub fn dths(&mut self) -> DthsW<WexctrlSpec> {
        DthsW::new(self, 24)
    }
}
#[doc = "Waveform Extension Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`wexctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wexctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WexctrlSpec;
impl crate::RegisterSpec for WexctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wexctrl::R`](R) reader structure"]
impl crate::Readable for WexctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`wexctrl::W`](W) writer structure"]
impl crate::Writable for WexctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WEXCTRL to value 0"]
impl crate::Resettable for WexctrlSpec {
    const RESET_VALUE: u32 = 0;
}
