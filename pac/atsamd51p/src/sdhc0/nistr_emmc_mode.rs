#[doc = "Register `NISTR_EMMC_MODE` reader"]
pub type R = crate::R<NistrEmmcModeSpec>;
#[doc = "Register `NISTR_EMMC_MODE` writer"]
pub type W = crate::W<NistrEmmcModeSpec>;
#[doc = "Command Complete\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmdcselect {
    #[doc = "0: No command complete"]
    No = 0,
    #[doc = "1: Command complete"]
    Yes = 1,
}
impl From<Cmdcselect> for bool {
    #[inline(always)]
    fn from(variant: Cmdcselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMDC` reader - Command Complete"]
pub type CmdcR = crate::BitReader<Cmdcselect>;
impl CmdcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cmdcselect {
        match self.bits {
            false => Cmdcselect::No,
            true => Cmdcselect::Yes,
        }
    }
    #[doc = "No command complete"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == Cmdcselect::No
    }
    #[doc = "Command complete"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        *self == Cmdcselect::Yes
    }
}
#[doc = "Field `CMDC` writer - Command Complete"]
pub type CmdcW<'a, REG> = crate::BitWriter<'a, REG, Cmdcselect>;
impl<'a, REG> CmdcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No command complete"]
    #[inline(always)]
    pub fn no(self) -> &'a mut crate::W<REG> {
        self.variant(Cmdcselect::No)
    }
    #[doc = "Command complete"]
    #[inline(always)]
    pub fn yes(self) -> &'a mut crate::W<REG> {
        self.variant(Cmdcselect::Yes)
    }
}
#[doc = "Transfer Complete\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Trfcselect {
    #[doc = "0: Not complete"]
    No = 0,
    #[doc = "1: Command execution is completed"]
    Yes = 1,
}
impl From<Trfcselect> for bool {
    #[inline(always)]
    fn from(variant: Trfcselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRFC` reader - Transfer Complete"]
pub type TrfcR = crate::BitReader<Trfcselect>;
impl TrfcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Trfcselect {
        match self.bits {
            false => Trfcselect::No,
            true => Trfcselect::Yes,
        }
    }
    #[doc = "Not complete"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == Trfcselect::No
    }
    #[doc = "Command execution is completed"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        *self == Trfcselect::Yes
    }
}
#[doc = "Field `TRFC` writer - Transfer Complete"]
pub type TrfcW<'a, REG> = crate::BitWriter<'a, REG, Trfcselect>;
impl<'a, REG> TrfcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not complete"]
    #[inline(always)]
    pub fn no(self) -> &'a mut crate::W<REG> {
        self.variant(Trfcselect::No)
    }
    #[doc = "Command execution is completed"]
    #[inline(always)]
    pub fn yes(self) -> &'a mut crate::W<REG> {
        self.variant(Trfcselect::Yes)
    }
}
#[doc = "Block Gap Event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Blkgeselect {
    #[doc = "0: No Block Gap Event"]
    No = 0,
    #[doc = "1: Transaction stopped at block gap"]
    Stop = 1,
}
impl From<Blkgeselect> for bool {
    #[inline(always)]
    fn from(variant: Blkgeselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BLKGE` reader - Block Gap Event"]
pub type BlkgeR = crate::BitReader<Blkgeselect>;
impl BlkgeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Blkgeselect {
        match self.bits {
            false => Blkgeselect::No,
            true => Blkgeselect::Stop,
        }
    }
    #[doc = "No Block Gap Event"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == Blkgeselect::No
    }
    #[doc = "Transaction stopped at block gap"]
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        *self == Blkgeselect::Stop
    }
}
#[doc = "Field `BLKGE` writer - Block Gap Event"]
pub type BlkgeW<'a, REG> = crate::BitWriter<'a, REG, Blkgeselect>;
impl<'a, REG> BlkgeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Block Gap Event"]
    #[inline(always)]
    pub fn no(self) -> &'a mut crate::W<REG> {
        self.variant(Blkgeselect::No)
    }
    #[doc = "Transaction stopped at block gap"]
    #[inline(always)]
    pub fn stop(self) -> &'a mut crate::W<REG> {
        self.variant(Blkgeselect::Stop)
    }
}
#[doc = "DMA Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmaintselect {
    #[doc = "0: No DMA Interrupt"]
    No = 0,
    #[doc = "1: DMA Interrupt is generated"]
    Yes = 1,
}
impl From<Dmaintselect> for bool {
    #[inline(always)]
    fn from(variant: Dmaintselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAINT` reader - DMA Interrupt"]
pub type DmaintR = crate::BitReader<Dmaintselect>;
impl DmaintR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmaintselect {
        match self.bits {
            false => Dmaintselect::No,
            true => Dmaintselect::Yes,
        }
    }
    #[doc = "No DMA Interrupt"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == Dmaintselect::No
    }
    #[doc = "DMA Interrupt is generated"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        *self == Dmaintselect::Yes
    }
}
#[doc = "Field `DMAINT` writer - DMA Interrupt"]
pub type DmaintW<'a, REG> = crate::BitWriter<'a, REG, Dmaintselect>;
impl<'a, REG> DmaintW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No DMA Interrupt"]
    #[inline(always)]
    pub fn no(self) -> &'a mut crate::W<REG> {
        self.variant(Dmaintselect::No)
    }
    #[doc = "DMA Interrupt is generated"]
    #[inline(always)]
    pub fn yes(self) -> &'a mut crate::W<REG> {
        self.variant(Dmaintselect::Yes)
    }
}
#[doc = "Buffer Write Ready\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bwrrdyselect {
    #[doc = "0: Not ready to write buffer"]
    No = 0,
    #[doc = "1: Ready to write buffer"]
    Yes = 1,
}
impl From<Bwrrdyselect> for bool {
    #[inline(always)]
    fn from(variant: Bwrrdyselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BWRRDY` reader - Buffer Write Ready"]
pub type BwrrdyR = crate::BitReader<Bwrrdyselect>;
impl BwrrdyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bwrrdyselect {
        match self.bits {
            false => Bwrrdyselect::No,
            true => Bwrrdyselect::Yes,
        }
    }
    #[doc = "Not ready to write buffer"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == Bwrrdyselect::No
    }
    #[doc = "Ready to write buffer"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        *self == Bwrrdyselect::Yes
    }
}
#[doc = "Field `BWRRDY` writer - Buffer Write Ready"]
pub type BwrrdyW<'a, REG> = crate::BitWriter<'a, REG, Bwrrdyselect>;
impl<'a, REG> BwrrdyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not ready to write buffer"]
    #[inline(always)]
    pub fn no(self) -> &'a mut crate::W<REG> {
        self.variant(Bwrrdyselect::No)
    }
    #[doc = "Ready to write buffer"]
    #[inline(always)]
    pub fn yes(self) -> &'a mut crate::W<REG> {
        self.variant(Bwrrdyselect::Yes)
    }
}
#[doc = "Buffer Read Ready\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Brdrdyselect {
    #[doc = "0: Not ready to read buffer"]
    No = 0,
    #[doc = "1: Ready to read buffer"]
    Yes = 1,
}
impl From<Brdrdyselect> for bool {
    #[inline(always)]
    fn from(variant: Brdrdyselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BRDRDY` reader - Buffer Read Ready"]
pub type BrdrdyR = crate::BitReader<Brdrdyselect>;
impl BrdrdyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Brdrdyselect {
        match self.bits {
            false => Brdrdyselect::No,
            true => Brdrdyselect::Yes,
        }
    }
    #[doc = "Not ready to read buffer"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == Brdrdyselect::No
    }
    #[doc = "Ready to read buffer"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        *self == Brdrdyselect::Yes
    }
}
#[doc = "Field `BRDRDY` writer - Buffer Read Ready"]
pub type BrdrdyW<'a, REG> = crate::BitWriter<'a, REG, Brdrdyselect>;
impl<'a, REG> BrdrdyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not ready to read buffer"]
    #[inline(always)]
    pub fn no(self) -> &'a mut crate::W<REG> {
        self.variant(Brdrdyselect::No)
    }
    #[doc = "Ready to read buffer"]
    #[inline(always)]
    pub fn yes(self) -> &'a mut crate::W<REG> {
        self.variant(Brdrdyselect::Yes)
    }
}
#[doc = "Field `BOOTAR` reader - Boot Acknowledge Received"]
pub type BootarR = crate::BitReader;
#[doc = "Field `BOOTAR` writer - Boot Acknowledge Received"]
pub type BootarW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Error Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Errintselect {
    #[doc = "0: No Error"]
    No = 0,
    #[doc = "1: Error"]
    Yes = 1,
}
impl From<Errintselect> for bool {
    #[inline(always)]
    fn from(variant: Errintselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERRINT` reader - Error Interrupt"]
pub type ErrintR = crate::BitReader<Errintselect>;
impl ErrintR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Errintselect {
        match self.bits {
            false => Errintselect::No,
            true => Errintselect::Yes,
        }
    }
    #[doc = "No Error"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == Errintselect::No
    }
    #[doc = "Error"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        *self == Errintselect::Yes
    }
}
#[doc = "Field `ERRINT` writer - Error Interrupt"]
pub type ErrintW<'a, REG> = crate::BitWriter<'a, REG, Errintselect>;
impl<'a, REG> ErrintW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Error"]
    #[inline(always)]
    pub fn no(self) -> &'a mut crate::W<REG> {
        self.variant(Errintselect::No)
    }
    #[doc = "Error"]
    #[inline(always)]
    pub fn yes(self) -> &'a mut crate::W<REG> {
        self.variant(Errintselect::Yes)
    }
}
impl R {
    #[doc = "Bit 0 - Command Complete"]
    #[inline(always)]
    pub fn cmdc(&self) -> CmdcR {
        CmdcR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transfer Complete"]
    #[inline(always)]
    pub fn trfc(&self) -> TrfcR {
        TrfcR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Block Gap Event"]
    #[inline(always)]
    pub fn blkge(&self) -> BlkgeR {
        BlkgeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DMA Interrupt"]
    #[inline(always)]
    pub fn dmaint(&self) -> DmaintR {
        DmaintR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Buffer Write Ready"]
    #[inline(always)]
    pub fn bwrrdy(&self) -> BwrrdyR {
        BwrrdyR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Buffer Read Ready"]
    #[inline(always)]
    pub fn brdrdy(&self) -> BrdrdyR {
        BrdrdyR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 14 - Boot Acknowledge Received"]
    #[inline(always)]
    pub fn bootar(&self) -> BootarR {
        BootarR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Error Interrupt"]
    #[inline(always)]
    pub fn errint(&self) -> ErrintR {
        ErrintR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Command Complete"]
    #[inline(always)]
    pub fn cmdc(&mut self) -> CmdcW<NistrEmmcModeSpec> {
        CmdcW::new(self, 0)
    }
    #[doc = "Bit 1 - Transfer Complete"]
    #[inline(always)]
    pub fn trfc(&mut self) -> TrfcW<NistrEmmcModeSpec> {
        TrfcW::new(self, 1)
    }
    #[doc = "Bit 2 - Block Gap Event"]
    #[inline(always)]
    pub fn blkge(&mut self) -> BlkgeW<NistrEmmcModeSpec> {
        BlkgeW::new(self, 2)
    }
    #[doc = "Bit 3 - DMA Interrupt"]
    #[inline(always)]
    pub fn dmaint(&mut self) -> DmaintW<NistrEmmcModeSpec> {
        DmaintW::new(self, 3)
    }
    #[doc = "Bit 4 - Buffer Write Ready"]
    #[inline(always)]
    pub fn bwrrdy(&mut self) -> BwrrdyW<NistrEmmcModeSpec> {
        BwrrdyW::new(self, 4)
    }
    #[doc = "Bit 5 - Buffer Read Ready"]
    #[inline(always)]
    pub fn brdrdy(&mut self) -> BrdrdyW<NistrEmmcModeSpec> {
        BrdrdyW::new(self, 5)
    }
    #[doc = "Bit 14 - Boot Acknowledge Received"]
    #[inline(always)]
    pub fn bootar(&mut self) -> BootarW<NistrEmmcModeSpec> {
        BootarW::new(self, 14)
    }
    #[doc = "Bit 15 - Error Interrupt"]
    #[inline(always)]
    pub fn errint(&mut self) -> ErrintW<NistrEmmcModeSpec> {
        ErrintW::new(self, 15)
    }
}
#[doc = "Normal Interrupt Status\n\nYou can [`read`](crate::Reg::read) this register and get [`nistr_emmc_mode::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nistr_emmc_mode::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NistrEmmcModeSpec;
impl crate::RegisterSpec for NistrEmmcModeSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`nistr_emmc_mode::R`](R) reader structure"]
impl crate::Readable for NistrEmmcModeSpec {}
#[doc = "`write(|w| ..)` method takes [`nistr_emmc_mode::W`](W) writer structure"]
impl crate::Writable for NistrEmmcModeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets NISTR_EMMC_MODE to value 0"]
impl crate::Resettable for NistrEmmcModeSpec {}
