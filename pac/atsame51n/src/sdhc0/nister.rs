#[doc = "Register `NISTER` reader"]
pub type R = crate::R<NisterSpec>;
#[doc = "Register `NISTER` writer"]
pub type W = crate::W<NisterSpec>;
#[doc = "Command Complete Status Enable\n\nValue on reset: 0"]
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
#[doc = "Field `CMDC` reader - Command Complete Status Enable"]
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
#[doc = "Field `CMDC` writer - Command Complete Status Enable"]
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
#[doc = "Transfer Complete Status Enable\n\nValue on reset: 0"]
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
#[doc = "Field `TRFC` reader - Transfer Complete Status Enable"]
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
#[doc = "Field `TRFC` writer - Transfer Complete Status Enable"]
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
#[doc = "Block Gap Event Status Enable\n\nValue on reset: 0"]
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
#[doc = "Field `BLKGE` reader - Block Gap Event Status Enable"]
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
#[doc = "Field `BLKGE` writer - Block Gap Event Status Enable"]
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
#[doc = "DMA Interrupt Status Enable\n\nValue on reset: 0"]
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
#[doc = "Field `DMAINT` reader - DMA Interrupt Status Enable"]
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
#[doc = "Field `DMAINT` writer - DMA Interrupt Status Enable"]
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
#[doc = "Buffer Write Ready Status Enable\n\nValue on reset: 0"]
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
#[doc = "Field `BWRRDY` reader - Buffer Write Ready Status Enable"]
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
#[doc = "Field `BWRRDY` writer - Buffer Write Ready Status Enable"]
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
#[doc = "Buffer Read Ready Status Enable\n\nValue on reset: 0"]
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
#[doc = "Field `BRDRDY` reader - Buffer Read Ready Status Enable"]
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
#[doc = "Field `BRDRDY` writer - Buffer Read Ready Status Enable"]
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
#[doc = "Card Insertion Status Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cinsselect {
    #[doc = "0: Masked"]
    Masked = 0,
    #[doc = "1: Enabled"]
    Enabled = 1,
}
impl From<Cinsselect> for bool {
    #[inline(always)]
    fn from(variant: Cinsselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CINS` reader - Card Insertion Status Enable"]
pub type CinsR = crate::BitReader<Cinsselect>;
impl CinsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cinsselect {
        match self.bits {
            false => Cinsselect::Masked,
            true => Cinsselect::Enabled,
        }
    }
    #[doc = "Masked"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == Cinsselect::Masked
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Cinsselect::Enabled
    }
}
#[doc = "Field `CINS` writer - Card Insertion Status Enable"]
pub type CinsW<'a, REG> = crate::BitWriter<'a, REG, Cinsselect>;
impl<'a, REG> CinsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut crate::W<REG> {
        self.variant(Cinsselect::Masked)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Cinsselect::Enabled)
    }
}
#[doc = "Card Removal Status Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cremselect {
    #[doc = "0: Masked"]
    Masked = 0,
    #[doc = "1: Enabled"]
    Enabled = 1,
}
impl From<Cremselect> for bool {
    #[inline(always)]
    fn from(variant: Cremselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CREM` reader - Card Removal Status Enable"]
pub type CremR = crate::BitReader<Cremselect>;
impl CremR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cremselect {
        match self.bits {
            false => Cremselect::Masked,
            true => Cremselect::Enabled,
        }
    }
    #[doc = "Masked"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == Cremselect::Masked
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Cremselect::Enabled
    }
}
#[doc = "Field `CREM` writer - Card Removal Status Enable"]
pub type CremW<'a, REG> = crate::BitWriter<'a, REG, Cremselect>;
impl<'a, REG> CremW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut crate::W<REG> {
        self.variant(Cremselect::Masked)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Cremselect::Enabled)
    }
}
#[doc = "Card Interrupt Status Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cintselect {
    #[doc = "0: Masked"]
    Masked = 0,
    #[doc = "1: Enabled"]
    Enabled = 1,
}
impl From<Cintselect> for bool {
    #[inline(always)]
    fn from(variant: Cintselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CINT` reader - Card Interrupt Status Enable"]
pub type CintR = crate::BitReader<Cintselect>;
impl CintR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cintselect {
        match self.bits {
            false => Cintselect::Masked,
            true => Cintselect::Enabled,
        }
    }
    #[doc = "Masked"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == Cintselect::Masked
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Cintselect::Enabled
    }
}
#[doc = "Field `CINT` writer - Card Interrupt Status Enable"]
pub type CintW<'a, REG> = crate::BitWriter<'a, REG, Cintselect>;
impl<'a, REG> CintW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut crate::W<REG> {
        self.variant(Cintselect::Masked)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Cintselect::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - Command Complete Status Enable"]
    #[inline(always)]
    pub fn cmdc(&self) -> CmdcR {
        CmdcR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transfer Complete Status Enable"]
    #[inline(always)]
    pub fn trfc(&self) -> TrfcR {
        TrfcR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Block Gap Event Status Enable"]
    #[inline(always)]
    pub fn blkge(&self) -> BlkgeR {
        BlkgeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DMA Interrupt Status Enable"]
    #[inline(always)]
    pub fn dmaint(&self) -> DmaintR {
        DmaintR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Buffer Write Ready Status Enable"]
    #[inline(always)]
    pub fn bwrrdy(&self) -> BwrrdyR {
        BwrrdyR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Buffer Read Ready Status Enable"]
    #[inline(always)]
    pub fn brdrdy(&self) -> BrdrdyR {
        BrdrdyR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Card Insertion Status Enable"]
    #[inline(always)]
    pub fn cins(&self) -> CinsR {
        CinsR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Card Removal Status Enable"]
    #[inline(always)]
    pub fn crem(&self) -> CremR {
        CremR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Card Interrupt Status Enable"]
    #[inline(always)]
    pub fn cint(&self) -> CintR {
        CintR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Command Complete Status Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmdc(&mut self) -> CmdcW<NisterSpec> {
        CmdcW::new(self, 0)
    }
    #[doc = "Bit 1 - Transfer Complete Status Enable"]
    #[inline(always)]
    #[must_use]
    pub fn trfc(&mut self) -> TrfcW<NisterSpec> {
        TrfcW::new(self, 1)
    }
    #[doc = "Bit 2 - Block Gap Event Status Enable"]
    #[inline(always)]
    #[must_use]
    pub fn blkge(&mut self) -> BlkgeW<NisterSpec> {
        BlkgeW::new(self, 2)
    }
    #[doc = "Bit 3 - DMA Interrupt Status Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dmaint(&mut self) -> DmaintW<NisterSpec> {
        DmaintW::new(self, 3)
    }
    #[doc = "Bit 4 - Buffer Write Ready Status Enable"]
    #[inline(always)]
    #[must_use]
    pub fn bwrrdy(&mut self) -> BwrrdyW<NisterSpec> {
        BwrrdyW::new(self, 4)
    }
    #[doc = "Bit 5 - Buffer Read Ready Status Enable"]
    #[inline(always)]
    #[must_use]
    pub fn brdrdy(&mut self) -> BrdrdyW<NisterSpec> {
        BrdrdyW::new(self, 5)
    }
    #[doc = "Bit 6 - Card Insertion Status Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cins(&mut self) -> CinsW<NisterSpec> {
        CinsW::new(self, 6)
    }
    #[doc = "Bit 7 - Card Removal Status Enable"]
    #[inline(always)]
    #[must_use]
    pub fn crem(&mut self) -> CremW<NisterSpec> {
        CremW::new(self, 7)
    }
    #[doc = "Bit 8 - Card Interrupt Status Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cint(&mut self) -> CintW<NisterSpec> {
        CintW::new(self, 8)
    }
}
#[doc = "Normal Interrupt Status Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`nister::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nister::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NisterSpec;
impl crate::RegisterSpec for NisterSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`nister::R`](R) reader structure"]
impl crate::Readable for NisterSpec {}
#[doc = "`write(|w| ..)` method takes [`nister::W`](W) writer structure"]
impl crate::Writable for NisterSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets NISTER to value 0"]
impl crate::Resettable for NisterSpec {
    const RESET_VALUE: u16 = 0;
}
