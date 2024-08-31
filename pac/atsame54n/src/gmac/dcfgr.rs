#[doc = "Register `DCFGR` reader"]
pub type R = crate::R<DcfgrSpec>;
#[doc = "Register `DCFGR` writer"]
pub type W = crate::W<DcfgrSpec>;
#[doc = "Field `FBLDO` reader - Fixed Burst Length for DMA Data Operations:"]
pub type FbldoR = crate::FieldReader;
#[doc = "Field `FBLDO` writer - Fixed Burst Length for DMA Data Operations:"]
pub type FbldoW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `ESMA` reader - Endian Swap Mode Enable for Management Descriptor Accesses"]
pub type EsmaR = crate::BitReader;
#[doc = "Field `ESMA` writer - Endian Swap Mode Enable for Management Descriptor Accesses"]
pub type EsmaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ESPA` reader - Endian Swap Mode Enable for Packet Data Accesses"]
pub type EspaR = crate::BitReader;
#[doc = "Field `ESPA` writer - Endian Swap Mode Enable for Packet Data Accesses"]
pub type EspaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXBMS` reader - Receiver Packet Buffer Memory Size Select"]
pub type RxbmsR = crate::FieldReader;
#[doc = "Field `RXBMS` writer - Receiver Packet Buffer Memory Size Select"]
pub type RxbmsW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TXPBMS` reader - Transmitter Packet Buffer Memory Size Select"]
pub type TxpbmsR = crate::BitReader;
#[doc = "Field `TXPBMS` writer - Transmitter Packet Buffer Memory Size Select"]
pub type TxpbmsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXCOEN` reader - Transmitter Checksum Generation Offload Enable"]
pub type TxcoenR = crate::BitReader;
#[doc = "Field `TXCOEN` writer - Transmitter Checksum Generation Offload Enable"]
pub type TxcoenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DRBS` reader - DMA Receive Buffer Size"]
pub type DrbsR = crate::FieldReader;
#[doc = "Field `DRBS` writer - DMA Receive Buffer Size"]
pub type DrbsW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DDRP` reader - DMA Discard Receive Packets"]
pub type DdrpR = crate::BitReader;
#[doc = "Field `DDRP` writer - DMA Discard Receive Packets"]
pub type DdrpW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:4 - Fixed Burst Length for DMA Data Operations:"]
    #[inline(always)]
    pub fn fbldo(&self) -> FbldoR {
        FbldoR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 6 - Endian Swap Mode Enable for Management Descriptor Accesses"]
    #[inline(always)]
    pub fn esma(&self) -> EsmaR {
        EsmaR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Endian Swap Mode Enable for Packet Data Accesses"]
    #[inline(always)]
    pub fn espa(&self) -> EspaR {
        EspaR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Receiver Packet Buffer Memory Size Select"]
    #[inline(always)]
    pub fn rxbms(&self) -> RxbmsR {
        RxbmsR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - Transmitter Packet Buffer Memory Size Select"]
    #[inline(always)]
    pub fn txpbms(&self) -> TxpbmsR {
        TxpbmsR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Transmitter Checksum Generation Offload Enable"]
    #[inline(always)]
    pub fn txcoen(&self) -> TxcoenR {
        TxcoenR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 16:23 - DMA Receive Buffer Size"]
    #[inline(always)]
    pub fn drbs(&self) -> DrbsR {
        DrbsR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - DMA Discard Receive Packets"]
    #[inline(always)]
    pub fn ddrp(&self) -> DdrpR {
        DdrpR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Fixed Burst Length for DMA Data Operations:"]
    #[inline(always)]
    #[must_use]
    pub fn fbldo(&mut self) -> FbldoW<DcfgrSpec> {
        FbldoW::new(self, 0)
    }
    #[doc = "Bit 6 - Endian Swap Mode Enable for Management Descriptor Accesses"]
    #[inline(always)]
    #[must_use]
    pub fn esma(&mut self) -> EsmaW<DcfgrSpec> {
        EsmaW::new(self, 6)
    }
    #[doc = "Bit 7 - Endian Swap Mode Enable for Packet Data Accesses"]
    #[inline(always)]
    #[must_use]
    pub fn espa(&mut self) -> EspaW<DcfgrSpec> {
        EspaW::new(self, 7)
    }
    #[doc = "Bits 8:9 - Receiver Packet Buffer Memory Size Select"]
    #[inline(always)]
    #[must_use]
    pub fn rxbms(&mut self) -> RxbmsW<DcfgrSpec> {
        RxbmsW::new(self, 8)
    }
    #[doc = "Bit 10 - Transmitter Packet Buffer Memory Size Select"]
    #[inline(always)]
    #[must_use]
    pub fn txpbms(&mut self) -> TxpbmsW<DcfgrSpec> {
        TxpbmsW::new(self, 10)
    }
    #[doc = "Bit 11 - Transmitter Checksum Generation Offload Enable"]
    #[inline(always)]
    #[must_use]
    pub fn txcoen(&mut self) -> TxcoenW<DcfgrSpec> {
        TxcoenW::new(self, 11)
    }
    #[doc = "Bits 16:23 - DMA Receive Buffer Size"]
    #[inline(always)]
    #[must_use]
    pub fn drbs(&mut self) -> DrbsW<DcfgrSpec> {
        DrbsW::new(self, 16)
    }
    #[doc = "Bit 24 - DMA Discard Receive Packets"]
    #[inline(always)]
    #[must_use]
    pub fn ddrp(&mut self) -> DdrpW<DcfgrSpec> {
        DdrpW::new(self, 24)
    }
}
#[doc = "DMA Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dcfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DcfgrSpec;
impl crate::RegisterSpec for DcfgrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcfgr::R`](R) reader structure"]
impl crate::Readable for DcfgrSpec {}
#[doc = "`write(|w| ..)` method takes [`dcfgr::W`](W) writer structure"]
impl crate::Writable for DcfgrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DCFGR to value 0x0002_0704"]
impl crate::Resettable for DcfgrSpec {
    const RESET_VALUE: u32 = 0x0002_0704;
}
