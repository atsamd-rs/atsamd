#[doc = "Register `RSR` reader"]
pub struct R(crate::R<RSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RSR` writer"]
pub struct W(crate::W<RSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<RSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BNA` reader - Buffer Not Available"]
pub type BNA_R = crate::BitReader<bool>;
#[doc = "Field `BNA` writer - Buffer Not Available"]
pub type BNA_W<'a, const O: u8> = crate::BitWriter<'a, u32, RSR_SPEC, bool, O>;
#[doc = "Field `REC` reader - Frame Received"]
pub type REC_R = crate::BitReader<bool>;
#[doc = "Field `REC` writer - Frame Received"]
pub type REC_W<'a, const O: u8> = crate::BitWriter<'a, u32, RSR_SPEC, bool, O>;
#[doc = "Field `RXOVR` reader - Receive Overrun"]
pub type RXOVR_R = crate::BitReader<bool>;
#[doc = "Field `RXOVR` writer - Receive Overrun"]
pub type RXOVR_W<'a, const O: u8> = crate::BitWriter<'a, u32, RSR_SPEC, bool, O>;
#[doc = "Field `HNO` reader - HRESP Not OK"]
pub type HNO_R = crate::BitReader<bool>;
#[doc = "Field `HNO` writer - HRESP Not OK"]
pub type HNO_W<'a, const O: u8> = crate::BitWriter<'a, u32, RSR_SPEC, bool, O>;
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
    pub fn bna(&mut self) -> BNA_W<0> {
        BNA_W::new(self)
    }
    #[doc = "Bit 1 - Frame Received"]
    #[inline(always)]
    #[must_use]
    pub fn rec(&mut self) -> REC_W<1> {
        REC_W::new(self)
    }
    #[doc = "Bit 2 - Receive Overrun"]
    #[inline(always)]
    #[must_use]
    pub fn rxovr(&mut self) -> RXOVR_W<2> {
        RXOVR_W::new(self)
    }
    #[doc = "Bit 3 - HRESP Not OK"]
    #[inline(always)]
    #[must_use]
    pub fn hno(&mut self) -> HNO_W<3> {
        HNO_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Receive Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rsr](index.html) module"]
pub struct RSR_SPEC;
impl crate::RegisterSpec for RSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rsr::R](R) reader structure"]
impl crate::Readable for RSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rsr::W](W) writer structure"]
impl crate::Writable for RSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RSR to value 0"]
impl crate::Resettable for RSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
