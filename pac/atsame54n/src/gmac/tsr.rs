#[doc = "Register `TSR` reader"]
pub struct R(crate::R<TSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TSR` writer"]
pub struct W(crate::W<TSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TSR_SPEC>;
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
impl From<crate::W<TSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UBR` reader - Used Bit Read"]
pub type UBR_R = crate::BitReader<bool>;
#[doc = "Field `UBR` writer - Used Bit Read"]
pub type UBR_W<'a, const O: u8> = crate::BitWriter<'a, u32, TSR_SPEC, bool, O>;
#[doc = "Field `COL` reader - Collision Occurred"]
pub type COL_R = crate::BitReader<bool>;
#[doc = "Field `COL` writer - Collision Occurred"]
pub type COL_W<'a, const O: u8> = crate::BitWriter<'a, u32, TSR_SPEC, bool, O>;
#[doc = "Field `RLE` reader - Retry Limit Exceeded"]
pub type RLE_R = crate::BitReader<bool>;
#[doc = "Field `RLE` writer - Retry Limit Exceeded"]
pub type RLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, TSR_SPEC, bool, O>;
#[doc = "Field `TXGO` reader - Transmit Go"]
pub type TXGO_R = crate::BitReader<bool>;
#[doc = "Field `TXGO` writer - Transmit Go"]
pub type TXGO_W<'a, const O: u8> = crate::BitWriter<'a, u32, TSR_SPEC, bool, O>;
#[doc = "Field `TFC` reader - Transmit Frame Corruption Due to AHB Error"]
pub type TFC_R = crate::BitReader<bool>;
#[doc = "Field `TFC` writer - Transmit Frame Corruption Due to AHB Error"]
pub type TFC_W<'a, const O: u8> = crate::BitWriter<'a, u32, TSR_SPEC, bool, O>;
#[doc = "Field `TXCOMP` reader - Transmit Complete"]
pub type TXCOMP_R = crate::BitReader<bool>;
#[doc = "Field `TXCOMP` writer - Transmit Complete"]
pub type TXCOMP_W<'a, const O: u8> = crate::BitWriter<'a, u32, TSR_SPEC, bool, O>;
#[doc = "Field `UND` reader - Transmit Underrun"]
pub type UND_R = crate::BitReader<bool>;
#[doc = "Field `UND` writer - Transmit Underrun"]
pub type UND_W<'a, const O: u8> = crate::BitWriter<'a, u32, TSR_SPEC, bool, O>;
#[doc = "Field `HRESP` reader - HRESP Not OK"]
pub type HRESP_R = crate::BitReader<bool>;
#[doc = "Field `HRESP` writer - HRESP Not OK"]
pub type HRESP_W<'a, const O: u8> = crate::BitWriter<'a, u32, TSR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Used Bit Read"]
    #[inline(always)]
    pub fn ubr(&self) -> UBR_R {
        UBR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Collision Occurred"]
    #[inline(always)]
    pub fn col(&self) -> COL_R {
        COL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Retry Limit Exceeded"]
    #[inline(always)]
    pub fn rle(&self) -> RLE_R {
        RLE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Transmit Go"]
    #[inline(always)]
    pub fn txgo(&self) -> TXGO_R {
        TXGO_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Transmit Frame Corruption Due to AHB Error"]
    #[inline(always)]
    pub fn tfc(&self) -> TFC_R {
        TFC_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Transmit Complete"]
    #[inline(always)]
    pub fn txcomp(&self) -> TXCOMP_R {
        TXCOMP_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Transmit Underrun"]
    #[inline(always)]
    pub fn und(&self) -> UND_R {
        UND_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - HRESP Not OK"]
    #[inline(always)]
    pub fn hresp(&self) -> HRESP_R {
        HRESP_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Used Bit Read"]
    #[inline(always)]
    #[must_use]
    pub fn ubr(&mut self) -> UBR_W<0> {
        UBR_W::new(self)
    }
    #[doc = "Bit 1 - Collision Occurred"]
    #[inline(always)]
    #[must_use]
    pub fn col(&mut self) -> COL_W<1> {
        COL_W::new(self)
    }
    #[doc = "Bit 2 - Retry Limit Exceeded"]
    #[inline(always)]
    #[must_use]
    pub fn rle(&mut self) -> RLE_W<2> {
        RLE_W::new(self)
    }
    #[doc = "Bit 3 - Transmit Go"]
    #[inline(always)]
    #[must_use]
    pub fn txgo(&mut self) -> TXGO_W<3> {
        TXGO_W::new(self)
    }
    #[doc = "Bit 4 - Transmit Frame Corruption Due to AHB Error"]
    #[inline(always)]
    #[must_use]
    pub fn tfc(&mut self) -> TFC_W<4> {
        TFC_W::new(self)
    }
    #[doc = "Bit 5 - Transmit Complete"]
    #[inline(always)]
    #[must_use]
    pub fn txcomp(&mut self) -> TXCOMP_W<5> {
        TXCOMP_W::new(self)
    }
    #[doc = "Bit 6 - Transmit Underrun"]
    #[inline(always)]
    #[must_use]
    pub fn und(&mut self) -> UND_W<6> {
        UND_W::new(self)
    }
    #[doc = "Bit 8 - HRESP Not OK"]
    #[inline(always)]
    #[must_use]
    pub fn hresp(&mut self) -> HRESP_W<8> {
        HRESP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Transmit Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tsr](index.html) module"]
pub struct TSR_SPEC;
impl crate::RegisterSpec for TSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tsr::R](R) reader structure"]
impl crate::Readable for TSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tsr::W](W) writer structure"]
impl crate::Writable for TSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TSR to value 0"]
impl crate::Resettable for TSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
