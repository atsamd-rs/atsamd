#[doc = "Register `DCFGR` reader"]
pub type R = crate::R<DcfgrSpec>;
#[doc = "Register `DCFGR` writer"]
pub type W = crate::W<DcfgrSpec>;
#[doc = "Fixed Burst Length for DMA Data Operations:\n\nValue on reset: 4"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Fbldoselect {
    #[doc = "1: 00001: Always use SINGLE AHB bursts"]
    Single = 1,
    #[doc = "4: 001xx: Attempt to use INCR4 AHB bursts (Default)"]
    Incr4 = 4,
    #[doc = "8: 01xxx: Attempt to use INCR8 AHB bursts"]
    Incr8 = 8,
    #[doc = "16: 1xxxx: Attempt to use INCR16 AHB bursts"]
    Incr16 = 16,
}
impl From<Fbldoselect> for u8 {
    #[inline(always)]
    fn from(variant: Fbldoselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Fbldoselect {
    type Ux = u8;
}
impl crate::IsEnum for Fbldoselect {}
#[doc = "Field `FBLDO` reader - Fixed Burst Length for DMA Data Operations:"]
pub type FbldoR = crate::FieldReader<Fbldoselect>;
impl FbldoR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Fbldoselect> {
        match self.bits {
            1 => Some(Fbldoselect::Single),
            4 => Some(Fbldoselect::Incr4),
            8 => Some(Fbldoselect::Incr8),
            16 => Some(Fbldoselect::Incr16),
            _ => None,
        }
    }
    #[doc = "00001: Always use SINGLE AHB bursts"]
    #[inline(always)]
    pub fn is_single(&self) -> bool {
        *self == Fbldoselect::Single
    }
    #[doc = "001xx: Attempt to use INCR4 AHB bursts (Default)"]
    #[inline(always)]
    pub fn is_incr4(&self) -> bool {
        *self == Fbldoselect::Incr4
    }
    #[doc = "01xxx: Attempt to use INCR8 AHB bursts"]
    #[inline(always)]
    pub fn is_incr8(&self) -> bool {
        *self == Fbldoselect::Incr8
    }
    #[doc = "1xxxx: Attempt to use INCR16 AHB bursts"]
    #[inline(always)]
    pub fn is_incr16(&self) -> bool {
        *self == Fbldoselect::Incr16
    }
}
#[doc = "Field `FBLDO` writer - Fixed Burst Length for DMA Data Operations:"]
pub type FbldoW<'a, REG> = crate::FieldWriter<'a, REG, 5, Fbldoselect>;
impl<'a, REG> FbldoW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "00001: Always use SINGLE AHB bursts"]
    #[inline(always)]
    pub fn single(self) -> &'a mut crate::W<REG> {
        self.variant(Fbldoselect::Single)
    }
    #[doc = "001xx: Attempt to use INCR4 AHB bursts (Default)"]
    #[inline(always)]
    pub fn incr4(self) -> &'a mut crate::W<REG> {
        self.variant(Fbldoselect::Incr4)
    }
    #[doc = "01xxx: Attempt to use INCR8 AHB bursts"]
    #[inline(always)]
    pub fn incr8(self) -> &'a mut crate::W<REG> {
        self.variant(Fbldoselect::Incr8)
    }
    #[doc = "1xxxx: Attempt to use INCR16 AHB bursts"]
    #[inline(always)]
    pub fn incr16(self) -> &'a mut crate::W<REG> {
        self.variant(Fbldoselect::Incr16)
    }
}
#[doc = "Field `ESMA` reader - Endian Swap Mode Enable for Management Descriptor Accesses"]
pub type EsmaR = crate::BitReader;
#[doc = "Field `ESMA` writer - Endian Swap Mode Enable for Management Descriptor Accesses"]
pub type EsmaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ESPA` reader - Endian Swap Mode Enable for Packet Data Accesses"]
pub type EspaR = crate::BitReader;
#[doc = "Field `ESPA` writer - Endian Swap Mode Enable for Packet Data Accesses"]
pub type EspaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Receiver Packet Buffer Memory Size Select\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Rxbmsselect {
    #[doc = "0: RECEIVE_BUFFER_SIZE/8 Kbyte Memory Size"]
    Eighth = 0,
    #[doc = "1: RECEIVE_BUFFER_SIZE/4 Kbytes Memory Size"]
    Quarter = 1,
    #[doc = "2: RECEIVE_BUFFER_SIZE/2 Kbytes Memory Size"]
    Half = 2,
    #[doc = "3: RECEIVE_BUFFER_SIZE Kbytes Memory Size"]
    Full = 3,
}
impl From<Rxbmsselect> for u8 {
    #[inline(always)]
    fn from(variant: Rxbmsselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Rxbmsselect {
    type Ux = u8;
}
impl crate::IsEnum for Rxbmsselect {}
#[doc = "Field `RXBMS` reader - Receiver Packet Buffer Memory Size Select"]
pub type RxbmsR = crate::FieldReader<Rxbmsselect>;
impl RxbmsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rxbmsselect {
        match self.bits {
            0 => Rxbmsselect::Eighth,
            1 => Rxbmsselect::Quarter,
            2 => Rxbmsselect::Half,
            3 => Rxbmsselect::Full,
            _ => unreachable!(),
        }
    }
    #[doc = "RECEIVE_BUFFER_SIZE/8 Kbyte Memory Size"]
    #[inline(always)]
    pub fn is_eighth(&self) -> bool {
        *self == Rxbmsselect::Eighth
    }
    #[doc = "RECEIVE_BUFFER_SIZE/4 Kbytes Memory Size"]
    #[inline(always)]
    pub fn is_quarter(&self) -> bool {
        *self == Rxbmsselect::Quarter
    }
    #[doc = "RECEIVE_BUFFER_SIZE/2 Kbytes Memory Size"]
    #[inline(always)]
    pub fn is_half(&self) -> bool {
        *self == Rxbmsselect::Half
    }
    #[doc = "RECEIVE_BUFFER_SIZE Kbytes Memory Size"]
    #[inline(always)]
    pub fn is_full(&self) -> bool {
        *self == Rxbmsselect::Full
    }
}
#[doc = "Field `RXBMS` writer - Receiver Packet Buffer Memory Size Select"]
pub type RxbmsW<'a, REG> = crate::FieldWriter<'a, REG, 2, Rxbmsselect, crate::Safe>;
impl<'a, REG> RxbmsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "RECEIVE_BUFFER_SIZE/8 Kbyte Memory Size"]
    #[inline(always)]
    pub fn eighth(self) -> &'a mut crate::W<REG> {
        self.variant(Rxbmsselect::Eighth)
    }
    #[doc = "RECEIVE_BUFFER_SIZE/4 Kbytes Memory Size"]
    #[inline(always)]
    pub fn quarter(self) -> &'a mut crate::W<REG> {
        self.variant(Rxbmsselect::Quarter)
    }
    #[doc = "RECEIVE_BUFFER_SIZE/2 Kbytes Memory Size"]
    #[inline(always)]
    pub fn half(self) -> &'a mut crate::W<REG> {
        self.variant(Rxbmsselect::Half)
    }
    #[doc = "RECEIVE_BUFFER_SIZE Kbytes Memory Size"]
    #[inline(always)]
    pub fn full(self) -> &'a mut crate::W<REG> {
        self.variant(Rxbmsselect::Full)
    }
}
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
    pub fn fbldo(&mut self) -> FbldoW<DcfgrSpec> {
        FbldoW::new(self, 0)
    }
    #[doc = "Bit 6 - Endian Swap Mode Enable for Management Descriptor Accesses"]
    #[inline(always)]
    pub fn esma(&mut self) -> EsmaW<DcfgrSpec> {
        EsmaW::new(self, 6)
    }
    #[doc = "Bit 7 - Endian Swap Mode Enable for Packet Data Accesses"]
    #[inline(always)]
    pub fn espa(&mut self) -> EspaW<DcfgrSpec> {
        EspaW::new(self, 7)
    }
    #[doc = "Bits 8:9 - Receiver Packet Buffer Memory Size Select"]
    #[inline(always)]
    pub fn rxbms(&mut self) -> RxbmsW<DcfgrSpec> {
        RxbmsW::new(self, 8)
    }
    #[doc = "Bit 10 - Transmitter Packet Buffer Memory Size Select"]
    #[inline(always)]
    pub fn txpbms(&mut self) -> TxpbmsW<DcfgrSpec> {
        TxpbmsW::new(self, 10)
    }
    #[doc = "Bit 11 - Transmitter Checksum Generation Offload Enable"]
    #[inline(always)]
    pub fn txcoen(&mut self) -> TxcoenW<DcfgrSpec> {
        TxcoenW::new(self, 11)
    }
    #[doc = "Bits 16:23 - DMA Receive Buffer Size"]
    #[inline(always)]
    pub fn drbs(&mut self) -> DrbsW<DcfgrSpec> {
        DrbsW::new(self, 16)
    }
    #[doc = "Bit 24 - DMA Discard Receive Packets"]
    #[inline(always)]
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
}
#[doc = "`reset()` method sets DCFGR to value 0x0002_0704"]
impl crate::Resettable for DcfgrSpec {
    const RESET_VALUE: u32 = 0x0002_0704;
}
