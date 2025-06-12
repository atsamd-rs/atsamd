#[doc = "Register `NISIER_EMMC_MODE` reader"]
pub type R = crate::R<NisierEmmcModeSpec>;
#[doc = "Register `NISIER_EMMC_MODE` writer"]
pub type W = crate::W<NisierEmmcModeSpec>;
#[doc = "Command Complete Signal Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmdcselect {
    #[doc = "0: Masked"]
    Masked = 0,
    #[doc = "1: Enabled"]
    Enabled = 1,
}
impl From<Cmdcselect> for bool {
    #[inline(always)]
    fn from(variant: Cmdcselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMDC` reader - Command Complete Signal Enable"]
pub type CmdcR = crate::BitReader<Cmdcselect>;
impl CmdcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cmdcselect {
        match self.bits {
            false => Cmdcselect::Masked,
            true => Cmdcselect::Enabled,
        }
    }
    #[doc = "Masked"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == Cmdcselect::Masked
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Cmdcselect::Enabled
    }
}
#[doc = "Field `CMDC` writer - Command Complete Signal Enable"]
pub type CmdcW<'a, REG> = crate::BitWriter<'a, REG, Cmdcselect>;
impl<'a, REG> CmdcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut crate::W<REG> {
        self.variant(Cmdcselect::Masked)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Cmdcselect::Enabled)
    }
}
#[doc = "Transfer Complete Signal Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Trfcselect {
    #[doc = "0: Masked"]
    Masked = 0,
    #[doc = "1: Enabled"]
    Enabled = 1,
}
impl From<Trfcselect> for bool {
    #[inline(always)]
    fn from(variant: Trfcselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRFC` reader - Transfer Complete Signal Enable"]
pub type TrfcR = crate::BitReader<Trfcselect>;
impl TrfcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Trfcselect {
        match self.bits {
            false => Trfcselect::Masked,
            true => Trfcselect::Enabled,
        }
    }
    #[doc = "Masked"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == Trfcselect::Masked
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Trfcselect::Enabled
    }
}
#[doc = "Field `TRFC` writer - Transfer Complete Signal Enable"]
pub type TrfcW<'a, REG> = crate::BitWriter<'a, REG, Trfcselect>;
impl<'a, REG> TrfcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut crate::W<REG> {
        self.variant(Trfcselect::Masked)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Trfcselect::Enabled)
    }
}
#[doc = "Block Gap Event Signal Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Blkgeselect {
    #[doc = "0: Masked"]
    Masked = 0,
    #[doc = "1: Enabled"]
    Enabled = 1,
}
impl From<Blkgeselect> for bool {
    #[inline(always)]
    fn from(variant: Blkgeselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BLKGE` reader - Block Gap Event Signal Enable"]
pub type BlkgeR = crate::BitReader<Blkgeselect>;
impl BlkgeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Blkgeselect {
        match self.bits {
            false => Blkgeselect::Masked,
            true => Blkgeselect::Enabled,
        }
    }
    #[doc = "Masked"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == Blkgeselect::Masked
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Blkgeselect::Enabled
    }
}
#[doc = "Field `BLKGE` writer - Block Gap Event Signal Enable"]
pub type BlkgeW<'a, REG> = crate::BitWriter<'a, REG, Blkgeselect>;
impl<'a, REG> BlkgeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut crate::W<REG> {
        self.variant(Blkgeselect::Masked)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Blkgeselect::Enabled)
    }
}
#[doc = "DMA Interrupt Signal Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmaintselect {
    #[doc = "0: Masked"]
    Masked = 0,
    #[doc = "1: Enabled"]
    Enabled = 1,
}
impl From<Dmaintselect> for bool {
    #[inline(always)]
    fn from(variant: Dmaintselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAINT` reader - DMA Interrupt Signal Enable"]
pub type DmaintR = crate::BitReader<Dmaintselect>;
impl DmaintR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmaintselect {
        match self.bits {
            false => Dmaintselect::Masked,
            true => Dmaintselect::Enabled,
        }
    }
    #[doc = "Masked"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == Dmaintselect::Masked
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Dmaintselect::Enabled
    }
}
#[doc = "Field `DMAINT` writer - DMA Interrupt Signal Enable"]
pub type DmaintW<'a, REG> = crate::BitWriter<'a, REG, Dmaintselect>;
impl<'a, REG> DmaintW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut crate::W<REG> {
        self.variant(Dmaintselect::Masked)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dmaintselect::Enabled)
    }
}
#[doc = "Buffer Write Ready Signal Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bwrrdyselect {
    #[doc = "0: Masked"]
    Masked = 0,
    #[doc = "1: Enabled"]
    Enabled = 1,
}
impl From<Bwrrdyselect> for bool {
    #[inline(always)]
    fn from(variant: Bwrrdyselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BWRRDY` reader - Buffer Write Ready Signal Enable"]
pub type BwrrdyR = crate::BitReader<Bwrrdyselect>;
impl BwrrdyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bwrrdyselect {
        match self.bits {
            false => Bwrrdyselect::Masked,
            true => Bwrrdyselect::Enabled,
        }
    }
    #[doc = "Masked"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == Bwrrdyselect::Masked
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Bwrrdyselect::Enabled
    }
}
#[doc = "Field `BWRRDY` writer - Buffer Write Ready Signal Enable"]
pub type BwrrdyW<'a, REG> = crate::BitWriter<'a, REG, Bwrrdyselect>;
impl<'a, REG> BwrrdyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut crate::W<REG> {
        self.variant(Bwrrdyselect::Masked)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Bwrrdyselect::Enabled)
    }
}
#[doc = "Buffer Read Ready Signal Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Brdrdyselect {
    #[doc = "0: Masked"]
    Masked = 0,
    #[doc = "1: Enabled"]
    Enabled = 1,
}
impl From<Brdrdyselect> for bool {
    #[inline(always)]
    fn from(variant: Brdrdyselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BRDRDY` reader - Buffer Read Ready Signal Enable"]
pub type BrdrdyR = crate::BitReader<Brdrdyselect>;
impl BrdrdyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Brdrdyselect {
        match self.bits {
            false => Brdrdyselect::Masked,
            true => Brdrdyselect::Enabled,
        }
    }
    #[doc = "Masked"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == Brdrdyselect::Masked
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Brdrdyselect::Enabled
    }
}
#[doc = "Field `BRDRDY` writer - Buffer Read Ready Signal Enable"]
pub type BrdrdyW<'a, REG> = crate::BitWriter<'a, REG, Brdrdyselect>;
impl<'a, REG> BrdrdyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut crate::W<REG> {
        self.variant(Brdrdyselect::Masked)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Brdrdyselect::Enabled)
    }
}
#[doc = "Field `BOOTAR` reader - Boot Acknowledge Received Signal Enable"]
pub type BootarR = crate::BitReader;
#[doc = "Field `BOOTAR` writer - Boot Acknowledge Received Signal Enable"]
pub type BootarW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Command Complete Signal Enable"]
    #[inline(always)]
    pub fn cmdc(&self) -> CmdcR {
        CmdcR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transfer Complete Signal Enable"]
    #[inline(always)]
    pub fn trfc(&self) -> TrfcR {
        TrfcR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Block Gap Event Signal Enable"]
    #[inline(always)]
    pub fn blkge(&self) -> BlkgeR {
        BlkgeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DMA Interrupt Signal Enable"]
    #[inline(always)]
    pub fn dmaint(&self) -> DmaintR {
        DmaintR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Buffer Write Ready Signal Enable"]
    #[inline(always)]
    pub fn bwrrdy(&self) -> BwrrdyR {
        BwrrdyR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Buffer Read Ready Signal Enable"]
    #[inline(always)]
    pub fn brdrdy(&self) -> BrdrdyR {
        BrdrdyR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 14 - Boot Acknowledge Received Signal Enable"]
    #[inline(always)]
    pub fn bootar(&self) -> BootarR {
        BootarR::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Command Complete Signal Enable"]
    #[inline(always)]
    pub fn cmdc(&mut self) -> CmdcW<NisierEmmcModeSpec> {
        CmdcW::new(self, 0)
    }
    #[doc = "Bit 1 - Transfer Complete Signal Enable"]
    #[inline(always)]
    pub fn trfc(&mut self) -> TrfcW<NisierEmmcModeSpec> {
        TrfcW::new(self, 1)
    }
    #[doc = "Bit 2 - Block Gap Event Signal Enable"]
    #[inline(always)]
    pub fn blkge(&mut self) -> BlkgeW<NisierEmmcModeSpec> {
        BlkgeW::new(self, 2)
    }
    #[doc = "Bit 3 - DMA Interrupt Signal Enable"]
    #[inline(always)]
    pub fn dmaint(&mut self) -> DmaintW<NisierEmmcModeSpec> {
        DmaintW::new(self, 3)
    }
    #[doc = "Bit 4 - Buffer Write Ready Signal Enable"]
    #[inline(always)]
    pub fn bwrrdy(&mut self) -> BwrrdyW<NisierEmmcModeSpec> {
        BwrrdyW::new(self, 4)
    }
    #[doc = "Bit 5 - Buffer Read Ready Signal Enable"]
    #[inline(always)]
    pub fn brdrdy(&mut self) -> BrdrdyW<NisierEmmcModeSpec> {
        BrdrdyW::new(self, 5)
    }
    #[doc = "Bit 14 - Boot Acknowledge Received Signal Enable"]
    #[inline(always)]
    pub fn bootar(&mut self) -> BootarW<NisierEmmcModeSpec> {
        BootarW::new(self, 14)
    }
}
#[doc = "Normal Interrupt Signal Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`nisier_emmc_mode::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nisier_emmc_mode::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NisierEmmcModeSpec;
impl crate::RegisterSpec for NisierEmmcModeSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`nisier_emmc_mode::R`](R) reader structure"]
impl crate::Readable for NisierEmmcModeSpec {}
#[doc = "`write(|w| ..)` method takes [`nisier_emmc_mode::W`](W) writer structure"]
impl crate::Writable for NisierEmmcModeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets NISIER_EMMC_MODE to value 0"]
impl crate::Resettable for NisierEmmcModeSpec {}
