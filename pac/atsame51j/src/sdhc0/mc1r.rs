#[doc = "Register `MC1R` reader"]
pub type R = crate::R<Mc1rSpec>;
#[doc = "Register `MC1R` writer"]
pub type W = crate::W<Mc1rSpec>;
#[doc = "e.MMC Command Type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cmdtypselect {
    #[doc = "0: Not a MMC specific command"]
    Normal = 0,
    #[doc = "1: Wait IRQ Command"]
    Waitirq = 1,
    #[doc = "2: Stream Command"]
    Stream = 2,
    #[doc = "3: Boot Command"]
    Boot = 3,
}
impl From<Cmdtypselect> for u8 {
    #[inline(always)]
    fn from(variant: Cmdtypselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cmdtypselect {
    type Ux = u8;
}
impl crate::IsEnum for Cmdtypselect {}
#[doc = "Field `CMDTYP` reader - e.MMC Command Type"]
pub type CmdtypR = crate::FieldReader<Cmdtypselect>;
impl CmdtypR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cmdtypselect {
        match self.bits {
            0 => Cmdtypselect::Normal,
            1 => Cmdtypselect::Waitirq,
            2 => Cmdtypselect::Stream,
            3 => Cmdtypselect::Boot,
            _ => unreachable!(),
        }
    }
    #[doc = "Not a MMC specific command"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == Cmdtypselect::Normal
    }
    #[doc = "Wait IRQ Command"]
    #[inline(always)]
    pub fn is_waitirq(&self) -> bool {
        *self == Cmdtypselect::Waitirq
    }
    #[doc = "Stream Command"]
    #[inline(always)]
    pub fn is_stream(&self) -> bool {
        *self == Cmdtypselect::Stream
    }
    #[doc = "Boot Command"]
    #[inline(always)]
    pub fn is_boot(&self) -> bool {
        *self == Cmdtypselect::Boot
    }
}
#[doc = "Field `CMDTYP` writer - e.MMC Command Type"]
pub type CmdtypW<'a, REG> = crate::FieldWriter<'a, REG, 2, Cmdtypselect, crate::Safe>;
impl<'a, REG> CmdtypW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Not a MMC specific command"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(Cmdtypselect::Normal)
    }
    #[doc = "Wait IRQ Command"]
    #[inline(always)]
    pub fn waitirq(self) -> &'a mut crate::W<REG> {
        self.variant(Cmdtypselect::Waitirq)
    }
    #[doc = "Stream Command"]
    #[inline(always)]
    pub fn stream(self) -> &'a mut crate::W<REG> {
        self.variant(Cmdtypselect::Stream)
    }
    #[doc = "Boot Command"]
    #[inline(always)]
    pub fn boot(self) -> &'a mut crate::W<REG> {
        self.variant(Cmdtypselect::Boot)
    }
}
#[doc = "Field `DDR` reader - e.MMC HSDDR Mode"]
pub type DdrR = crate::BitReader;
#[doc = "Field `DDR` writer - e.MMC HSDDR Mode"]
pub type DdrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OPD` reader - e.MMC Open Drain Mode"]
pub type OpdR = crate::BitReader;
#[doc = "Field `OPD` writer - e.MMC Open Drain Mode"]
pub type OpdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BOOTA` reader - e.MMC Boot Acknowledge Enable"]
pub type BootaR = crate::BitReader;
#[doc = "Field `BOOTA` writer - e.MMC Boot Acknowledge Enable"]
pub type BootaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSTN` reader - e.MMC Reset Signal"]
pub type RstnR = crate::BitReader;
#[doc = "Field `RSTN` writer - e.MMC Reset Signal"]
pub type RstnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FCD` reader - e.MMC Force Card Detect"]
pub type FcdR = crate::BitReader;
#[doc = "Field `FCD` writer - e.MMC Force Card Detect"]
pub type FcdW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - e.MMC Command Type"]
    #[inline(always)]
    pub fn cmdtyp(&self) -> CmdtypR {
        CmdtypR::new(self.bits & 3)
    }
    #[doc = "Bit 3 - e.MMC HSDDR Mode"]
    #[inline(always)]
    pub fn ddr(&self) -> DdrR {
        DdrR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - e.MMC Open Drain Mode"]
    #[inline(always)]
    pub fn opd(&self) -> OpdR {
        OpdR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - e.MMC Boot Acknowledge Enable"]
    #[inline(always)]
    pub fn boota(&self) -> BootaR {
        BootaR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - e.MMC Reset Signal"]
    #[inline(always)]
    pub fn rstn(&self) -> RstnR {
        RstnR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - e.MMC Force Card Detect"]
    #[inline(always)]
    pub fn fcd(&self) -> FcdR {
        FcdR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - e.MMC Command Type"]
    #[inline(always)]
    pub fn cmdtyp(&mut self) -> CmdtypW<Mc1rSpec> {
        CmdtypW::new(self, 0)
    }
    #[doc = "Bit 3 - e.MMC HSDDR Mode"]
    #[inline(always)]
    pub fn ddr(&mut self) -> DdrW<Mc1rSpec> {
        DdrW::new(self, 3)
    }
    #[doc = "Bit 4 - e.MMC Open Drain Mode"]
    #[inline(always)]
    pub fn opd(&mut self) -> OpdW<Mc1rSpec> {
        OpdW::new(self, 4)
    }
    #[doc = "Bit 5 - e.MMC Boot Acknowledge Enable"]
    #[inline(always)]
    pub fn boota(&mut self) -> BootaW<Mc1rSpec> {
        BootaW::new(self, 5)
    }
    #[doc = "Bit 6 - e.MMC Reset Signal"]
    #[inline(always)]
    pub fn rstn(&mut self) -> RstnW<Mc1rSpec> {
        RstnW::new(self, 6)
    }
    #[doc = "Bit 7 - e.MMC Force Card Detect"]
    #[inline(always)]
    pub fn fcd(&mut self) -> FcdW<Mc1rSpec> {
        FcdW::new(self, 7)
    }
}
#[doc = "MMC Control 1\n\nYou can [`read`](crate::Reg::read) this register and get [`mc1r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mc1r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Mc1rSpec;
impl crate::RegisterSpec for Mc1rSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`mc1r::R`](R) reader structure"]
impl crate::Readable for Mc1rSpec {}
#[doc = "`write(|w| ..)` method takes [`mc1r::W`](W) writer structure"]
impl crate::Writable for Mc1rSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MC1R to value 0"]
impl crate::Resettable for Mc1rSpec {}
