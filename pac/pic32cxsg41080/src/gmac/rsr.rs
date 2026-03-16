#[doc = "Register `RSR` reader"]
pub type R = crate::R<RsrSpec>;
#[doc = "Register `RSR` writer"]
pub type W = crate::W<RsrSpec>;
#[doc = "Field `BNA` reader - Buffer Not Available"]
pub type BnaR = crate::BitReader;
#[doc = "Field `BNA` writer - Buffer Not Available"]
pub type BnaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REC` reader - Frame Received"]
pub type RecR = crate::BitReader;
#[doc = "Field `REC` writer - Frame Received"]
pub type RecW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXOVR` reader - Receive Overrun"]
pub type RxovrR = crate::BitReader;
#[doc = "Field `RXOVR` writer - Receive Overrun"]
pub type RxovrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HNO` reader - HRESP Not OK"]
pub type HnoR = crate::BitReader;
#[doc = "Field `HNO` writer - HRESP Not OK"]
pub type HnoW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Buffer Not Available"]
    #[inline(always)]
    pub fn bna(&self) -> BnaR {
        BnaR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Frame Received"]
    #[inline(always)]
    pub fn rec(&self) -> RecR {
        RecR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Receive Overrun"]
    #[inline(always)]
    pub fn rxovr(&self) -> RxovrR {
        RxovrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - HRESP Not OK"]
    #[inline(always)]
    pub fn hno(&self) -> HnoR {
        HnoR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Buffer Not Available"]
    #[inline(always)]
    #[must_use]
    pub fn bna(&mut self) -> BnaW<RsrSpec> {
        BnaW::new(self, 0)
    }
    #[doc = "Bit 1 - Frame Received"]
    #[inline(always)]
    #[must_use]
    pub fn rec(&mut self) -> RecW<RsrSpec> {
        RecW::new(self, 1)
    }
    #[doc = "Bit 2 - Receive Overrun"]
    #[inline(always)]
    #[must_use]
    pub fn rxovr(&mut self) -> RxovrW<RsrSpec> {
        RxovrW::new(self, 2)
    }
    #[doc = "Bit 3 - HRESP Not OK"]
    #[inline(always)]
    #[must_use]
    pub fn hno(&mut self) -> HnoW<RsrSpec> {
        HnoW::new(self, 3)
    }
}
#[doc = "Receive Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RsrSpec;
impl crate::RegisterSpec for RsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rsr::R`](R) reader structure"]
impl crate::Readable for RsrSpec {}
#[doc = "`write(|w| ..)` method takes [`rsr::W`](W) writer structure"]
impl crate::Writable for RsrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RSR to value 0"]
impl crate::Resettable for RsrSpec {
    const RESET_VALUE: u32 = 0;
}
