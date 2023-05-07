#[doc = "Register `DCFGR` reader"]
pub struct R(crate::R<DCFGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DCFGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DCFGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DCFGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DCFGR` writer"]
pub struct W(crate::W<DCFGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DCFGR_SPEC>;
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
impl From<crate::W<DCFGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DCFGR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FBLDO` reader - Fixed Burst Length for DMA Data Operations:"]
pub type FBLDO_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FBLDO` writer - Fixed Burst Length for DMA Data Operations:"]
pub type FBLDO_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DCFGR_SPEC, u8, u8, 5, O>;
#[doc = "Field `ESMA` reader - Endian Swap Mode Enable for Management Descriptor Accesses"]
pub type ESMA_R = crate::BitReader<bool>;
#[doc = "Field `ESMA` writer - Endian Swap Mode Enable for Management Descriptor Accesses"]
pub type ESMA_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCFGR_SPEC, bool, O>;
#[doc = "Field `ESPA` reader - Endian Swap Mode Enable for Packet Data Accesses"]
pub type ESPA_R = crate::BitReader<bool>;
#[doc = "Field `ESPA` writer - Endian Swap Mode Enable for Packet Data Accesses"]
pub type ESPA_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCFGR_SPEC, bool, O>;
#[doc = "Field `RXBMS` reader - Receiver Packet Buffer Memory Size Select"]
pub type RXBMS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RXBMS` writer - Receiver Packet Buffer Memory Size Select"]
pub type RXBMS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DCFGR_SPEC, u8, u8, 2, O>;
#[doc = "Field `TXPBMS` reader - Transmitter Packet Buffer Memory Size Select"]
pub type TXPBMS_R = crate::BitReader<bool>;
#[doc = "Field `TXPBMS` writer - Transmitter Packet Buffer Memory Size Select"]
pub type TXPBMS_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCFGR_SPEC, bool, O>;
#[doc = "Field `TXCOEN` reader - Transmitter Checksum Generation Offload Enable"]
pub type TXCOEN_R = crate::BitReader<bool>;
#[doc = "Field `TXCOEN` writer - Transmitter Checksum Generation Offload Enable"]
pub type TXCOEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCFGR_SPEC, bool, O>;
#[doc = "Field `DRBS` reader - DMA Receive Buffer Size"]
pub type DRBS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DRBS` writer - DMA Receive Buffer Size"]
pub type DRBS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DCFGR_SPEC, u8, u8, 8, O>;
#[doc = "Field `DDRP` reader - DMA Discard Receive Packets"]
pub type DDRP_R = crate::BitReader<bool>;
#[doc = "Field `DDRP` writer - DMA Discard Receive Packets"]
pub type DDRP_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCFGR_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:4 - Fixed Burst Length for DMA Data Operations:"]
    #[inline(always)]
    pub fn fbldo(&self) -> FBLDO_R {
        FBLDO_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 6 - Endian Swap Mode Enable for Management Descriptor Accesses"]
    #[inline(always)]
    pub fn esma(&self) -> ESMA_R {
        ESMA_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Endian Swap Mode Enable for Packet Data Accesses"]
    #[inline(always)]
    pub fn espa(&self) -> ESPA_R {
        ESPA_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Receiver Packet Buffer Memory Size Select"]
    #[inline(always)]
    pub fn rxbms(&self) -> RXBMS_R {
        RXBMS_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - Transmitter Packet Buffer Memory Size Select"]
    #[inline(always)]
    pub fn txpbms(&self) -> TXPBMS_R {
        TXPBMS_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Transmitter Checksum Generation Offload Enable"]
    #[inline(always)]
    pub fn txcoen(&self) -> TXCOEN_R {
        TXCOEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 16:23 - DMA Receive Buffer Size"]
    #[inline(always)]
    pub fn drbs(&self) -> DRBS_R {
        DRBS_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - DMA Discard Receive Packets"]
    #[inline(always)]
    pub fn ddrp(&self) -> DDRP_R {
        DDRP_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Fixed Burst Length for DMA Data Operations:"]
    #[inline(always)]
    #[must_use]
    pub fn fbldo(&mut self) -> FBLDO_W<0> {
        FBLDO_W::new(self)
    }
    #[doc = "Bit 6 - Endian Swap Mode Enable for Management Descriptor Accesses"]
    #[inline(always)]
    #[must_use]
    pub fn esma(&mut self) -> ESMA_W<6> {
        ESMA_W::new(self)
    }
    #[doc = "Bit 7 - Endian Swap Mode Enable for Packet Data Accesses"]
    #[inline(always)]
    #[must_use]
    pub fn espa(&mut self) -> ESPA_W<7> {
        ESPA_W::new(self)
    }
    #[doc = "Bits 8:9 - Receiver Packet Buffer Memory Size Select"]
    #[inline(always)]
    #[must_use]
    pub fn rxbms(&mut self) -> RXBMS_W<8> {
        RXBMS_W::new(self)
    }
    #[doc = "Bit 10 - Transmitter Packet Buffer Memory Size Select"]
    #[inline(always)]
    #[must_use]
    pub fn txpbms(&mut self) -> TXPBMS_W<10> {
        TXPBMS_W::new(self)
    }
    #[doc = "Bit 11 - Transmitter Checksum Generation Offload Enable"]
    #[inline(always)]
    #[must_use]
    pub fn txcoen(&mut self) -> TXCOEN_W<11> {
        TXCOEN_W::new(self)
    }
    #[doc = "Bits 16:23 - DMA Receive Buffer Size"]
    #[inline(always)]
    #[must_use]
    pub fn drbs(&mut self) -> DRBS_W<16> {
        DRBS_W::new(self)
    }
    #[doc = "Bit 24 - DMA Discard Receive Packets"]
    #[inline(always)]
    #[must_use]
    pub fn ddrp(&mut self) -> DDRP_W<24> {
        DDRP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcfgr](index.html) module"]
pub struct DCFGR_SPEC;
impl crate::RegisterSpec for DCFGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dcfgr::R](R) reader structure"]
impl crate::Readable for DCFGR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dcfgr::W](W) writer structure"]
impl crate::Writable for DCFGR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DCFGR to value 0x0002_0704"]
impl crate::Resettable for DCFGR_SPEC {
    const RESET_VALUE: Self::Ux = 0x0002_0704;
}
