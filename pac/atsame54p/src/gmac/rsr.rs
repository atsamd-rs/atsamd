#[doc = "Register `RSR` reader"]
pub type R = crate::R<RSR_SPEC>;
#[doc = "Register `RSR` writer"]
pub type W = crate::W<RSR_SPEC>;
#[doc = "Field `BNA` reader - Buffer Not Available"]
pub type BNA_R = crate::BitReader;
#[doc = "Field `BNA` writer - Buffer Not Available"]
pub type BNA_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `REC` reader - Frame Received"]
pub type REC_R = crate::BitReader;
#[doc = "Field `REC` writer - Frame Received"]
pub type REC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RXOVR` reader - Receive Overrun"]
pub type RXOVR_R = crate::BitReader;
#[doc = "Field `RXOVR` writer - Receive Overrun"]
pub type RXOVR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `HNO` reader - HRESP Not OK"]
pub type HNO_R = crate::BitReader;
#[doc = "Field `HNO` writer - HRESP Not OK"]
pub type HNO_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Buffer Not Available"]
    #[inline(always)]
    pub fn bna(&self) -> BNA_R {
        BNA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Frame Received"]
    #[inline(always)]
    pub fn rec(&self) -> REC_R {
        REC_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Receive Overrun"]
    #[inline(always)]
    pub fn rxovr(&self) -> RXOVR_R {
        RXOVR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - HRESP Not OK"]
    #[inline(always)]
    pub fn hno(&self) -> HNO_R {
        HNO_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Buffer Not Available"]
    #[inline(always)]
    #[must_use]
    pub fn bna(&mut self) -> BNA_W<RSR_SPEC, 0> {
        BNA_W::new(self)
    }
    #[doc = "Bit 1 - Frame Received"]
    #[inline(always)]
    #[must_use]
    pub fn rec(&mut self) -> REC_W<RSR_SPEC, 1> {
        REC_W::new(self)
    }
    #[doc = "Bit 2 - Receive Overrun"]
    #[inline(always)]
    #[must_use]
    pub fn rxovr(&mut self) -> RXOVR_W<RSR_SPEC, 2> {
        RXOVR_W::new(self)
    }
    #[doc = "Bit 3 - HRESP Not OK"]
    #[inline(always)]
    #[must_use]
    pub fn hno(&mut self) -> HNO_W<RSR_SPEC, 3> {
        HNO_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Receive Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rsr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rsr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RSR_SPEC;
impl crate::RegisterSpec for RSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rsr::R`](R) reader structure"]
impl crate::Readable for RSR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rsr::W`](W) writer structure"]
impl crate::Writable for RSR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RSR to value 0"]
impl crate::Resettable for RSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
