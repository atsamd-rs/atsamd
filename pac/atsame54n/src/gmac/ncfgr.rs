#[doc = "Register `NCFGR` reader"]
pub type R = crate::R<NCFGR_SPEC>;
#[doc = "Register `NCFGR` writer"]
pub type W = crate::W<NCFGR_SPEC>;
#[doc = "Field `SPD` reader - Speed"]
pub type SPD_R = crate::BitReader;
#[doc = "Field `SPD` writer - Speed"]
pub type SPD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FD` reader - Full Duplex"]
pub type FD_R = crate::BitReader;
#[doc = "Field `FD` writer - Full Duplex"]
pub type FD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DNVLAN` reader - Discard Non-VLAN FRAMES"]
pub type DNVLAN_R = crate::BitReader;
#[doc = "Field `DNVLAN` writer - Discard Non-VLAN FRAMES"]
pub type DNVLAN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `JFRAME` reader - Jumbo Frame Size"]
pub type JFRAME_R = crate::BitReader;
#[doc = "Field `JFRAME` writer - Jumbo Frame Size"]
pub type JFRAME_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CAF` reader - Copy All Frames"]
pub type CAF_R = crate::BitReader;
#[doc = "Field `CAF` writer - Copy All Frames"]
pub type CAF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `NBC` reader - No Broadcast"]
pub type NBC_R = crate::BitReader;
#[doc = "Field `NBC` writer - No Broadcast"]
pub type NBC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MTIHEN` reader - Multicast Hash Enable"]
pub type MTIHEN_R = crate::BitReader;
#[doc = "Field `MTIHEN` writer - Multicast Hash Enable"]
pub type MTIHEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `UNIHEN` reader - Unicast Hash Enable"]
pub type UNIHEN_R = crate::BitReader;
#[doc = "Field `UNIHEN` writer - Unicast Hash Enable"]
pub type UNIHEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MAXFS` reader - 1536 Maximum Frame Size"]
pub type MAXFS_R = crate::BitReader;
#[doc = "Field `MAXFS` writer - 1536 Maximum Frame Size"]
pub type MAXFS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RTY` reader - Retry Test"]
pub type RTY_R = crate::BitReader;
#[doc = "Field `RTY` writer - Retry Test"]
pub type RTY_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PEN` reader - Pause Enable"]
pub type PEN_R = crate::BitReader;
#[doc = "Field `PEN` writer - Pause Enable"]
pub type PEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RXBUFO` reader - Receive Buffer Offset"]
pub type RXBUFO_R = crate::FieldReader;
#[doc = "Field `RXBUFO` writer - Receive Buffer Offset"]
pub type RXBUFO_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `LFERD` reader - Length Field Error Frame Discard"]
pub type LFERD_R = crate::BitReader;
#[doc = "Field `LFERD` writer - Length Field Error Frame Discard"]
pub type LFERD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RFCS` reader - Remove FCS"]
pub type RFCS_R = crate::BitReader;
#[doc = "Field `RFCS` writer - Remove FCS"]
pub type RFCS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CLK` reader - MDC CLock Division"]
pub type CLK_R = crate::FieldReader;
#[doc = "Field `CLK` writer - MDC CLock Division"]
pub type CLK_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `DBW` reader - Data Bus Width"]
pub type DBW_R = crate::FieldReader;
#[doc = "Field `DBW` writer - Data Bus Width"]
pub type DBW_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `DCPF` reader - Disable Copy of Pause Frames"]
pub type DCPF_R = crate::BitReader;
#[doc = "Field `DCPF` writer - Disable Copy of Pause Frames"]
pub type DCPF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RXCOEN` reader - Receive Checksum Offload Enable"]
pub type RXCOEN_R = crate::BitReader;
#[doc = "Field `RXCOEN` writer - Receive Checksum Offload Enable"]
pub type RXCOEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EFRHD` reader - Enable Frames Received in Half Duplex"]
pub type EFRHD_R = crate::BitReader;
#[doc = "Field `EFRHD` writer - Enable Frames Received in Half Duplex"]
pub type EFRHD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `IRXFCS` reader - Ignore RX FCS"]
pub type IRXFCS_R = crate::BitReader;
#[doc = "Field `IRXFCS` writer - Ignore RX FCS"]
pub type IRXFCS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `IPGSEN` reader - IP Stretch Enable"]
pub type IPGSEN_R = crate::BitReader;
#[doc = "Field `IPGSEN` writer - IP Stretch Enable"]
pub type IPGSEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RXBP` reader - Receive Bad Preamble"]
pub type RXBP_R = crate::BitReader;
#[doc = "Field `RXBP` writer - Receive Bad Preamble"]
pub type RXBP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `IRXER` reader - Ignore IPG GRXER"]
pub type IRXER_R = crate::BitReader;
#[doc = "Field `IRXER` writer - Ignore IPG GRXER"]
pub type IRXER_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Speed"]
    #[inline(always)]
    pub fn spd(&self) -> SPD_R {
        SPD_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Full Duplex"]
    #[inline(always)]
    pub fn fd(&self) -> FD_R {
        FD_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Discard Non-VLAN FRAMES"]
    #[inline(always)]
    pub fn dnvlan(&self) -> DNVLAN_R {
        DNVLAN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Jumbo Frame Size"]
    #[inline(always)]
    pub fn jframe(&self) -> JFRAME_R {
        JFRAME_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Copy All Frames"]
    #[inline(always)]
    pub fn caf(&self) -> CAF_R {
        CAF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - No Broadcast"]
    #[inline(always)]
    pub fn nbc(&self) -> NBC_R {
        NBC_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Multicast Hash Enable"]
    #[inline(always)]
    pub fn mtihen(&self) -> MTIHEN_R {
        MTIHEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Unicast Hash Enable"]
    #[inline(always)]
    pub fn unihen(&self) -> UNIHEN_R {
        UNIHEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 1536 Maximum Frame Size"]
    #[inline(always)]
    pub fn maxfs(&self) -> MAXFS_R {
        MAXFS_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - Retry Test"]
    #[inline(always)]
    pub fn rty(&self) -> RTY_R {
        RTY_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Pause Enable"]
    #[inline(always)]
    pub fn pen(&self) -> PEN_R {
        PEN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:15 - Receive Buffer Offset"]
    #[inline(always)]
    pub fn rxbufo(&self) -> RXBUFO_R {
        RXBUFO_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bit 16 - Length Field Error Frame Discard"]
    #[inline(always)]
    pub fn lferd(&self) -> LFERD_R {
        LFERD_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Remove FCS"]
    #[inline(always)]
    pub fn rfcs(&self) -> RFCS_R {
        RFCS_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:20 - MDC CLock Division"]
    #[inline(always)]
    pub fn clk(&self) -> CLK_R {
        CLK_R::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bits 21:22 - Data Bus Width"]
    #[inline(always)]
    pub fn dbw(&self) -> DBW_R {
        DBW_R::new(((self.bits >> 21) & 3) as u8)
    }
    #[doc = "Bit 23 - Disable Copy of Pause Frames"]
    #[inline(always)]
    pub fn dcpf(&self) -> DCPF_R {
        DCPF_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Receive Checksum Offload Enable"]
    #[inline(always)]
    pub fn rxcoen(&self) -> RXCOEN_R {
        RXCOEN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Enable Frames Received in Half Duplex"]
    #[inline(always)]
    pub fn efrhd(&self) -> EFRHD_R {
        EFRHD_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Ignore RX FCS"]
    #[inline(always)]
    pub fn irxfcs(&self) -> IRXFCS_R {
        IRXFCS_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 28 - IP Stretch Enable"]
    #[inline(always)]
    pub fn ipgsen(&self) -> IPGSEN_R {
        IPGSEN_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Receive Bad Preamble"]
    #[inline(always)]
    pub fn rxbp(&self) -> RXBP_R {
        RXBP_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Ignore IPG GRXER"]
    #[inline(always)]
    pub fn irxer(&self) -> IRXER_R {
        IRXER_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Speed"]
    #[inline(always)]
    #[must_use]
    pub fn spd(&mut self) -> SPD_W<NCFGR_SPEC, 0> {
        SPD_W::new(self)
    }
    #[doc = "Bit 1 - Full Duplex"]
    #[inline(always)]
    #[must_use]
    pub fn fd(&mut self) -> FD_W<NCFGR_SPEC, 1> {
        FD_W::new(self)
    }
    #[doc = "Bit 2 - Discard Non-VLAN FRAMES"]
    #[inline(always)]
    #[must_use]
    pub fn dnvlan(&mut self) -> DNVLAN_W<NCFGR_SPEC, 2> {
        DNVLAN_W::new(self)
    }
    #[doc = "Bit 3 - Jumbo Frame Size"]
    #[inline(always)]
    #[must_use]
    pub fn jframe(&mut self) -> JFRAME_W<NCFGR_SPEC, 3> {
        JFRAME_W::new(self)
    }
    #[doc = "Bit 4 - Copy All Frames"]
    #[inline(always)]
    #[must_use]
    pub fn caf(&mut self) -> CAF_W<NCFGR_SPEC, 4> {
        CAF_W::new(self)
    }
    #[doc = "Bit 5 - No Broadcast"]
    #[inline(always)]
    #[must_use]
    pub fn nbc(&mut self) -> NBC_W<NCFGR_SPEC, 5> {
        NBC_W::new(self)
    }
    #[doc = "Bit 6 - Multicast Hash Enable"]
    #[inline(always)]
    #[must_use]
    pub fn mtihen(&mut self) -> MTIHEN_W<NCFGR_SPEC, 6> {
        MTIHEN_W::new(self)
    }
    #[doc = "Bit 7 - Unicast Hash Enable"]
    #[inline(always)]
    #[must_use]
    pub fn unihen(&mut self) -> UNIHEN_W<NCFGR_SPEC, 7> {
        UNIHEN_W::new(self)
    }
    #[doc = "Bit 8 - 1536 Maximum Frame Size"]
    #[inline(always)]
    #[must_use]
    pub fn maxfs(&mut self) -> MAXFS_W<NCFGR_SPEC, 8> {
        MAXFS_W::new(self)
    }
    #[doc = "Bit 12 - Retry Test"]
    #[inline(always)]
    #[must_use]
    pub fn rty(&mut self) -> RTY_W<NCFGR_SPEC, 12> {
        RTY_W::new(self)
    }
    #[doc = "Bit 13 - Pause Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pen(&mut self) -> PEN_W<NCFGR_SPEC, 13> {
        PEN_W::new(self)
    }
    #[doc = "Bits 14:15 - Receive Buffer Offset"]
    #[inline(always)]
    #[must_use]
    pub fn rxbufo(&mut self) -> RXBUFO_W<NCFGR_SPEC, 14> {
        RXBUFO_W::new(self)
    }
    #[doc = "Bit 16 - Length Field Error Frame Discard"]
    #[inline(always)]
    #[must_use]
    pub fn lferd(&mut self) -> LFERD_W<NCFGR_SPEC, 16> {
        LFERD_W::new(self)
    }
    #[doc = "Bit 17 - Remove FCS"]
    #[inline(always)]
    #[must_use]
    pub fn rfcs(&mut self) -> RFCS_W<NCFGR_SPEC, 17> {
        RFCS_W::new(self)
    }
    #[doc = "Bits 18:20 - MDC CLock Division"]
    #[inline(always)]
    #[must_use]
    pub fn clk(&mut self) -> CLK_W<NCFGR_SPEC, 18> {
        CLK_W::new(self)
    }
    #[doc = "Bits 21:22 - Data Bus Width"]
    #[inline(always)]
    #[must_use]
    pub fn dbw(&mut self) -> DBW_W<NCFGR_SPEC, 21> {
        DBW_W::new(self)
    }
    #[doc = "Bit 23 - Disable Copy of Pause Frames"]
    #[inline(always)]
    #[must_use]
    pub fn dcpf(&mut self) -> DCPF_W<NCFGR_SPEC, 23> {
        DCPF_W::new(self)
    }
    #[doc = "Bit 24 - Receive Checksum Offload Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxcoen(&mut self) -> RXCOEN_W<NCFGR_SPEC, 24> {
        RXCOEN_W::new(self)
    }
    #[doc = "Bit 25 - Enable Frames Received in Half Duplex"]
    #[inline(always)]
    #[must_use]
    pub fn efrhd(&mut self) -> EFRHD_W<NCFGR_SPEC, 25> {
        EFRHD_W::new(self)
    }
    #[doc = "Bit 26 - Ignore RX FCS"]
    #[inline(always)]
    #[must_use]
    pub fn irxfcs(&mut self) -> IRXFCS_W<NCFGR_SPEC, 26> {
        IRXFCS_W::new(self)
    }
    #[doc = "Bit 28 - IP Stretch Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ipgsen(&mut self) -> IPGSEN_W<NCFGR_SPEC, 28> {
        IPGSEN_W::new(self)
    }
    #[doc = "Bit 29 - Receive Bad Preamble"]
    #[inline(always)]
    #[must_use]
    pub fn rxbp(&mut self) -> RXBP_W<NCFGR_SPEC, 29> {
        RXBP_W::new(self)
    }
    #[doc = "Bit 30 - Ignore IPG GRXER"]
    #[inline(always)]
    #[must_use]
    pub fn irxer(&mut self) -> IRXER_W<NCFGR_SPEC, 30> {
        IRXER_W::new(self)
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
#[doc = "Network Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ncfgr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ncfgr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NCFGR_SPEC;
impl crate::RegisterSpec for NCFGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ncfgr::R`](R) reader structure"]
impl crate::Readable for NCFGR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ncfgr::W`](W) writer structure"]
impl crate::Writable for NCFGR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets NCFGR to value 0x0008_0000"]
impl crate::Resettable for NCFGR_SPEC {
    const RESET_VALUE: Self::Ux = 0x0008_0000;
}
