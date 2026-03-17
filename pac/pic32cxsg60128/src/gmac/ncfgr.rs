#[doc = "Register `NCFGR` reader"]
pub type R = crate::R<NcfgrSpec>;
#[doc = "Register `NCFGR` writer"]
pub type W = crate::W<NcfgrSpec>;
#[doc = "Field `SPD` reader - Speed"]
pub type SpdR = crate::BitReader;
#[doc = "Field `SPD` writer - Speed"]
pub type SpdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FD` reader - Full Duplex"]
pub type FdR = crate::BitReader;
#[doc = "Field `FD` writer - Full Duplex"]
pub type FdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DNVLAN` reader - Discard Non-VLAN FRAMES"]
pub type DnvlanR = crate::BitReader;
#[doc = "Field `DNVLAN` writer - Discard Non-VLAN FRAMES"]
pub type DnvlanW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `JFRAME` reader - Jumbo Frame Size"]
pub type JframeR = crate::BitReader;
#[doc = "Field `JFRAME` writer - Jumbo Frame Size"]
pub type JframeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAF` reader - Copy All Frames"]
pub type CafR = crate::BitReader;
#[doc = "Field `CAF` writer - Copy All Frames"]
pub type CafW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NBC` reader - No Broadcast"]
pub type NbcR = crate::BitReader;
#[doc = "Field `NBC` writer - No Broadcast"]
pub type NbcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MTIHEN` reader - Multicast Hash Enable"]
pub type MtihenR = crate::BitReader;
#[doc = "Field `MTIHEN` writer - Multicast Hash Enable"]
pub type MtihenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UNIHEN` reader - Unicast Hash Enable"]
pub type UnihenR = crate::BitReader;
#[doc = "Field `UNIHEN` writer - Unicast Hash Enable"]
pub type UnihenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MAXFS` reader - 1536 Maximum Frame Size"]
pub type MaxfsR = crate::BitReader;
#[doc = "Field `MAXFS` writer - 1536 Maximum Frame Size"]
pub type MaxfsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTY` reader - Retry Test"]
pub type RtyR = crate::BitReader;
#[doc = "Field `RTY` writer - Retry Test"]
pub type RtyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PEN` reader - Pause Enable"]
pub type PenR = crate::BitReader;
#[doc = "Field `PEN` writer - Pause Enable"]
pub type PenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXBUFO` reader - Receive Buffer Offset"]
pub type RxbufoR = crate::FieldReader;
#[doc = "Field `RXBUFO` writer - Receive Buffer Offset"]
pub type RxbufoW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `LFERD` reader - Length Field Error Frame Discard"]
pub type LferdR = crate::BitReader;
#[doc = "Field `LFERD` writer - Length Field Error Frame Discard"]
pub type LferdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RFCS` reader - Remove FCS"]
pub type RfcsR = crate::BitReader;
#[doc = "Field `RFCS` writer - Remove FCS"]
pub type RfcsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "MDC CLock Division\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Clkselect {
    #[doc = "0: MCK divided by 8"]
    Mck8 = 0,
    #[doc = "1: MCK divided by 16"]
    Mck16 = 1,
    #[doc = "2: MCK divided by 32"]
    Mck32 = 2,
    #[doc = "3: MCK divided by 48"]
    Mck48 = 3,
    #[doc = "4: MCK divided by 64"]
    Mck64 = 4,
    #[doc = "5: MCK divided by 96"]
    Mck96 = 5,
}
impl From<Clkselect> for u8 {
    #[inline(always)]
    fn from(variant: Clkselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Clkselect {
    type Ux = u8;
}
impl crate::IsEnum for Clkselect {}
#[doc = "Field `CLK` reader - MDC CLock Division"]
pub type ClkR = crate::FieldReader<Clkselect>;
impl ClkR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Clkselect> {
        match self.bits {
            0 => Some(Clkselect::Mck8),
            1 => Some(Clkselect::Mck16),
            2 => Some(Clkselect::Mck32),
            3 => Some(Clkselect::Mck48),
            4 => Some(Clkselect::Mck64),
            5 => Some(Clkselect::Mck96),
            _ => None,
        }
    }
    #[doc = "MCK divided by 8"]
    #[inline(always)]
    pub fn is_mck8(&self) -> bool {
        *self == Clkselect::Mck8
    }
    #[doc = "MCK divided by 16"]
    #[inline(always)]
    pub fn is_mck16(&self) -> bool {
        *self == Clkselect::Mck16
    }
    #[doc = "MCK divided by 32"]
    #[inline(always)]
    pub fn is_mck32(&self) -> bool {
        *self == Clkselect::Mck32
    }
    #[doc = "MCK divided by 48"]
    #[inline(always)]
    pub fn is_mck48(&self) -> bool {
        *self == Clkselect::Mck48
    }
    #[doc = "MCK divided by 64"]
    #[inline(always)]
    pub fn is_mck64(&self) -> bool {
        *self == Clkselect::Mck64
    }
    #[doc = "MCK divided by 96"]
    #[inline(always)]
    pub fn is_mck96(&self) -> bool {
        *self == Clkselect::Mck96
    }
}
#[doc = "Field `CLK` writer - MDC CLock Division"]
pub type ClkW<'a, REG> = crate::FieldWriter<'a, REG, 3, Clkselect>;
impl<'a, REG> ClkW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "MCK divided by 8"]
    #[inline(always)]
    pub fn mck8(self) -> &'a mut crate::W<REG> {
        self.variant(Clkselect::Mck8)
    }
    #[doc = "MCK divided by 16"]
    #[inline(always)]
    pub fn mck16(self) -> &'a mut crate::W<REG> {
        self.variant(Clkselect::Mck16)
    }
    #[doc = "MCK divided by 32"]
    #[inline(always)]
    pub fn mck32(self) -> &'a mut crate::W<REG> {
        self.variant(Clkselect::Mck32)
    }
    #[doc = "MCK divided by 48"]
    #[inline(always)]
    pub fn mck48(self) -> &'a mut crate::W<REG> {
        self.variant(Clkselect::Mck48)
    }
    #[doc = "MCK divided by 64"]
    #[inline(always)]
    pub fn mck64(self) -> &'a mut crate::W<REG> {
        self.variant(Clkselect::Mck64)
    }
    #[doc = "MCK divided by 96"]
    #[inline(always)]
    pub fn mck96(self) -> &'a mut crate::W<REG> {
        self.variant(Clkselect::Mck96)
    }
}
#[doc = "Field `DBW` reader - Data Bus Width"]
pub type DbwR = crate::FieldReader;
#[doc = "Field `DBW` writer - Data Bus Width"]
pub type DbwW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DCPF` reader - Disable Copy of Pause Frames"]
pub type DcpfR = crate::BitReader;
#[doc = "Field `DCPF` writer - Disable Copy of Pause Frames"]
pub type DcpfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXCOEN` reader - Receive Checksum Offload Enable"]
pub type RxcoenR = crate::BitReader;
#[doc = "Field `RXCOEN` writer - Receive Checksum Offload Enable"]
pub type RxcoenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EFRHD` reader - Enable Frames Received in Half Duplex"]
pub type EfrhdR = crate::BitReader;
#[doc = "Field `EFRHD` writer - Enable Frames Received in Half Duplex"]
pub type EfrhdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IRXFCS` reader - Ignore RX FCS"]
pub type IrxfcsR = crate::BitReader;
#[doc = "Field `IRXFCS` writer - Ignore RX FCS"]
pub type IrxfcsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IPGSEN` reader - IP Stretch Enable"]
pub type IpgsenR = crate::BitReader;
#[doc = "Field `IPGSEN` writer - IP Stretch Enable"]
pub type IpgsenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXBP` reader - Receive Bad Preamble"]
pub type RxbpR = crate::BitReader;
#[doc = "Field `RXBP` writer - Receive Bad Preamble"]
pub type RxbpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IRXER` reader - Ignore IPG GRXER"]
pub type IrxerR = crate::BitReader;
#[doc = "Field `IRXER` writer - Ignore IPG GRXER"]
pub type IrxerW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Speed"]
    #[inline(always)]
    pub fn spd(&self) -> SpdR {
        SpdR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Full Duplex"]
    #[inline(always)]
    pub fn fd(&self) -> FdR {
        FdR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Discard Non-VLAN FRAMES"]
    #[inline(always)]
    pub fn dnvlan(&self) -> DnvlanR {
        DnvlanR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Jumbo Frame Size"]
    #[inline(always)]
    pub fn jframe(&self) -> JframeR {
        JframeR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Copy All Frames"]
    #[inline(always)]
    pub fn caf(&self) -> CafR {
        CafR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - No Broadcast"]
    #[inline(always)]
    pub fn nbc(&self) -> NbcR {
        NbcR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Multicast Hash Enable"]
    #[inline(always)]
    pub fn mtihen(&self) -> MtihenR {
        MtihenR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Unicast Hash Enable"]
    #[inline(always)]
    pub fn unihen(&self) -> UnihenR {
        UnihenR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 1536 Maximum Frame Size"]
    #[inline(always)]
    pub fn maxfs(&self) -> MaxfsR {
        MaxfsR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - Retry Test"]
    #[inline(always)]
    pub fn rty(&self) -> RtyR {
        RtyR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Pause Enable"]
    #[inline(always)]
    pub fn pen(&self) -> PenR {
        PenR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:15 - Receive Buffer Offset"]
    #[inline(always)]
    pub fn rxbufo(&self) -> RxbufoR {
        RxbufoR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bit 16 - Length Field Error Frame Discard"]
    #[inline(always)]
    pub fn lferd(&self) -> LferdR {
        LferdR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Remove FCS"]
    #[inline(always)]
    pub fn rfcs(&self) -> RfcsR {
        RfcsR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:20 - MDC CLock Division"]
    #[inline(always)]
    pub fn clk(&self) -> ClkR {
        ClkR::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bits 21:22 - Data Bus Width"]
    #[inline(always)]
    pub fn dbw(&self) -> DbwR {
        DbwR::new(((self.bits >> 21) & 3) as u8)
    }
    #[doc = "Bit 23 - Disable Copy of Pause Frames"]
    #[inline(always)]
    pub fn dcpf(&self) -> DcpfR {
        DcpfR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Receive Checksum Offload Enable"]
    #[inline(always)]
    pub fn rxcoen(&self) -> RxcoenR {
        RxcoenR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Enable Frames Received in Half Duplex"]
    #[inline(always)]
    pub fn efrhd(&self) -> EfrhdR {
        EfrhdR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Ignore RX FCS"]
    #[inline(always)]
    pub fn irxfcs(&self) -> IrxfcsR {
        IrxfcsR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 28 - IP Stretch Enable"]
    #[inline(always)]
    pub fn ipgsen(&self) -> IpgsenR {
        IpgsenR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Receive Bad Preamble"]
    #[inline(always)]
    pub fn rxbp(&self) -> RxbpR {
        RxbpR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Ignore IPG GRXER"]
    #[inline(always)]
    pub fn irxer(&self) -> IrxerR {
        IrxerR::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Speed"]
    #[inline(always)]
    #[must_use]
    pub fn spd(&mut self) -> SpdW<NcfgrSpec> {
        SpdW::new(self, 0)
    }
    #[doc = "Bit 1 - Full Duplex"]
    #[inline(always)]
    #[must_use]
    pub fn fd(&mut self) -> FdW<NcfgrSpec> {
        FdW::new(self, 1)
    }
    #[doc = "Bit 2 - Discard Non-VLAN FRAMES"]
    #[inline(always)]
    #[must_use]
    pub fn dnvlan(&mut self) -> DnvlanW<NcfgrSpec> {
        DnvlanW::new(self, 2)
    }
    #[doc = "Bit 3 - Jumbo Frame Size"]
    #[inline(always)]
    #[must_use]
    pub fn jframe(&mut self) -> JframeW<NcfgrSpec> {
        JframeW::new(self, 3)
    }
    #[doc = "Bit 4 - Copy All Frames"]
    #[inline(always)]
    #[must_use]
    pub fn caf(&mut self) -> CafW<NcfgrSpec> {
        CafW::new(self, 4)
    }
    #[doc = "Bit 5 - No Broadcast"]
    #[inline(always)]
    #[must_use]
    pub fn nbc(&mut self) -> NbcW<NcfgrSpec> {
        NbcW::new(self, 5)
    }
    #[doc = "Bit 6 - Multicast Hash Enable"]
    #[inline(always)]
    #[must_use]
    pub fn mtihen(&mut self) -> MtihenW<NcfgrSpec> {
        MtihenW::new(self, 6)
    }
    #[doc = "Bit 7 - Unicast Hash Enable"]
    #[inline(always)]
    #[must_use]
    pub fn unihen(&mut self) -> UnihenW<NcfgrSpec> {
        UnihenW::new(self, 7)
    }
    #[doc = "Bit 8 - 1536 Maximum Frame Size"]
    #[inline(always)]
    #[must_use]
    pub fn maxfs(&mut self) -> MaxfsW<NcfgrSpec> {
        MaxfsW::new(self, 8)
    }
    #[doc = "Bit 12 - Retry Test"]
    #[inline(always)]
    #[must_use]
    pub fn rty(&mut self) -> RtyW<NcfgrSpec> {
        RtyW::new(self, 12)
    }
    #[doc = "Bit 13 - Pause Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pen(&mut self) -> PenW<NcfgrSpec> {
        PenW::new(self, 13)
    }
    #[doc = "Bits 14:15 - Receive Buffer Offset"]
    #[inline(always)]
    #[must_use]
    pub fn rxbufo(&mut self) -> RxbufoW<NcfgrSpec> {
        RxbufoW::new(self, 14)
    }
    #[doc = "Bit 16 - Length Field Error Frame Discard"]
    #[inline(always)]
    #[must_use]
    pub fn lferd(&mut self) -> LferdW<NcfgrSpec> {
        LferdW::new(self, 16)
    }
    #[doc = "Bit 17 - Remove FCS"]
    #[inline(always)]
    #[must_use]
    pub fn rfcs(&mut self) -> RfcsW<NcfgrSpec> {
        RfcsW::new(self, 17)
    }
    #[doc = "Bits 18:20 - MDC CLock Division"]
    #[inline(always)]
    #[must_use]
    pub fn clk(&mut self) -> ClkW<NcfgrSpec> {
        ClkW::new(self, 18)
    }
    #[doc = "Bits 21:22 - Data Bus Width"]
    #[inline(always)]
    #[must_use]
    pub fn dbw(&mut self) -> DbwW<NcfgrSpec> {
        DbwW::new(self, 21)
    }
    #[doc = "Bit 23 - Disable Copy of Pause Frames"]
    #[inline(always)]
    #[must_use]
    pub fn dcpf(&mut self) -> DcpfW<NcfgrSpec> {
        DcpfW::new(self, 23)
    }
    #[doc = "Bit 24 - Receive Checksum Offload Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxcoen(&mut self) -> RxcoenW<NcfgrSpec> {
        RxcoenW::new(self, 24)
    }
    #[doc = "Bit 25 - Enable Frames Received in Half Duplex"]
    #[inline(always)]
    #[must_use]
    pub fn efrhd(&mut self) -> EfrhdW<NcfgrSpec> {
        EfrhdW::new(self, 25)
    }
    #[doc = "Bit 26 - Ignore RX FCS"]
    #[inline(always)]
    #[must_use]
    pub fn irxfcs(&mut self) -> IrxfcsW<NcfgrSpec> {
        IrxfcsW::new(self, 26)
    }
    #[doc = "Bit 28 - IP Stretch Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ipgsen(&mut self) -> IpgsenW<NcfgrSpec> {
        IpgsenW::new(self, 28)
    }
    #[doc = "Bit 29 - Receive Bad Preamble"]
    #[inline(always)]
    #[must_use]
    pub fn rxbp(&mut self) -> RxbpW<NcfgrSpec> {
        RxbpW::new(self, 29)
    }
    #[doc = "Bit 30 - Ignore IPG GRXER"]
    #[inline(always)]
    #[must_use]
    pub fn irxer(&mut self) -> IrxerW<NcfgrSpec> {
        IrxerW::new(self, 30)
    }
}
#[doc = "Network Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ncfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ncfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NcfgrSpec;
impl crate::RegisterSpec for NcfgrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ncfgr::R`](R) reader structure"]
impl crate::Readable for NcfgrSpec {}
#[doc = "`write(|w| ..)` method takes [`ncfgr::W`](W) writer structure"]
impl crate::Writable for NcfgrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets NCFGR to value 0x0008_0000"]
impl crate::Resettable for NcfgrSpec {
    const RESET_VALUE: u32 = 0x0008_0000;
}
