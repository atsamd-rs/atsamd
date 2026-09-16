#[doc = "Register `NISTR` reader"]
pub type R = crate::R<NistrSpec>;
#[doc = "Register `NISTR` writer"]
pub type W = crate::W<NistrSpec>;
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
#[doc = "Card Insertion\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cinsselect {
    #[doc = "0: Card state stable or Debouncing"]
    No = 0,
    #[doc = "1: Card inserted"]
    Yes = 1,
}
impl From<Cinsselect> for bool {
    #[inline(always)]
    fn from(variant: Cinsselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CINS` reader - Card Insertion"]
pub type CinsR = crate::BitReader<Cinsselect>;
impl CinsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cinsselect {
        match self.bits {
            false => Cinsselect::No,
            true => Cinsselect::Yes,
        }
    }
    #[doc = "Card state stable or Debouncing"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == Cinsselect::No
    }
    #[doc = "Card inserted"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        *self == Cinsselect::Yes
    }
}
#[doc = "Field `CINS` writer - Card Insertion"]
pub type CinsW<'a, REG> = crate::BitWriter<'a, REG, Cinsselect>;
impl<'a, REG> CinsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Card state stable or Debouncing"]
    #[inline(always)]
    pub fn no(self) -> &'a mut crate::W<REG> {
        self.variant(Cinsselect::No)
    }
    #[doc = "Card inserted"]
    #[inline(always)]
    pub fn yes(self) -> &'a mut crate::W<REG> {
        self.variant(Cinsselect::Yes)
    }
}
#[doc = "Card Removal\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cremselect {
    #[doc = "0: Card state stable or Debouncing"]
    No = 0,
    #[doc = "1: Card Removed"]
    Yes = 1,
}
impl From<Cremselect> for bool {
    #[inline(always)]
    fn from(variant: Cremselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CREM` reader - Card Removal"]
pub type CremR = crate::BitReader<Cremselect>;
impl CremR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cremselect {
        match self.bits {
            false => Cremselect::No,
            true => Cremselect::Yes,
        }
    }
    #[doc = "Card state stable or Debouncing"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == Cremselect::No
    }
    #[doc = "Card Removed"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        *self == Cremselect::Yes
    }
}
#[doc = "Field `CREM` writer - Card Removal"]
pub type CremW<'a, REG> = crate::BitWriter<'a, REG, Cremselect>;
impl<'a, REG> CremW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Card state stable or Debouncing"]
    #[inline(always)]
    pub fn no(self) -> &'a mut crate::W<REG> {
        self.variant(Cremselect::No)
    }
    #[doc = "Card Removed"]
    #[inline(always)]
    pub fn yes(self) -> &'a mut crate::W<REG> {
        self.variant(Cremselect::Yes)
    }
}
#[doc = "Card Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cintselect {
    #[doc = "0: No Card Interrupt"]
    No = 0,
    #[doc = "1: Generate Card Interrupt"]
    Yes = 1,
}
impl From<Cintselect> for bool {
    #[inline(always)]
    fn from(variant: Cintselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CINT` reader - Card Interrupt"]
pub type CintR = crate::BitReader<Cintselect>;
impl CintR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cintselect {
        match self.bits {
            false => Cintselect::No,
            true => Cintselect::Yes,
        }
    }
    #[doc = "No Card Interrupt"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == Cintselect::No
    }
    #[doc = "Generate Card Interrupt"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        *self == Cintselect::Yes
    }
}
#[doc = "Field `CINT` writer - Card Interrupt"]
pub type CintW<'a, REG> = crate::BitWriter<'a, REG, Cintselect>;
impl<'a, REG> CintW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Card Interrupt"]
    #[inline(always)]
    pub fn no(self) -> &'a mut crate::W<REG> {
        self.variant(Cintselect::No)
    }
    #[doc = "Generate Card Interrupt"]
    #[inline(always)]
    pub fn yes(self) -> &'a mut crate::W<REG> {
        self.variant(Cintselect::Yes)
    }
}
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
    #[doc = "Bit 6 - Card Insertion"]
    #[inline(always)]
    pub fn cins(&self) -> CinsR {
        CinsR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Card Removal"]
    #[inline(always)]
    pub fn crem(&self) -> CremR {
        CremR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Card Interrupt"]
    #[inline(always)]
    pub fn cint(&self) -> CintR {
        CintR::new(((self.bits >> 8) & 1) != 0)
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
    #[must_use]
    pub fn cmdc(&mut self) -> CmdcW<NistrSpec> {
        CmdcW::new(self, 0)
    }
    #[doc = "Bit 1 - Transfer Complete"]
    #[inline(always)]
    #[must_use]
    pub fn trfc(&mut self) -> TrfcW<NistrSpec> {
        TrfcW::new(self, 1)
    }
    #[doc = "Bit 2 - Block Gap Event"]
    #[inline(always)]
    #[must_use]
    pub fn blkge(&mut self) -> BlkgeW<NistrSpec> {
        BlkgeW::new(self, 2)
    }
    #[doc = "Bit 3 - DMA Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn dmaint(&mut self) -> DmaintW<NistrSpec> {
        DmaintW::new(self, 3)
    }
    #[doc = "Bit 4 - Buffer Write Ready"]
    #[inline(always)]
    #[must_use]
    pub fn bwrrdy(&mut self) -> BwrrdyW<NistrSpec> {
        BwrrdyW::new(self, 4)
    }
    #[doc = "Bit 5 - Buffer Read Ready"]
    #[inline(always)]
    #[must_use]
    pub fn brdrdy(&mut self) -> BrdrdyW<NistrSpec> {
        BrdrdyW::new(self, 5)
    }
    #[doc = "Bit 6 - Card Insertion"]
    #[inline(always)]
    #[must_use]
    pub fn cins(&mut self) -> CinsW<NistrSpec> {
        CinsW::new(self, 6)
    }
    #[doc = "Bit 7 - Card Removal"]
    #[inline(always)]
    #[must_use]
    pub fn crem(&mut self) -> CremW<NistrSpec> {
        CremW::new(self, 7)
    }
    #[doc = "Bit 8 - Card Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn cint(&mut self) -> CintW<NistrSpec> {
        CintW::new(self, 8)
    }
    #[doc = "Bit 15 - Error Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn errint(&mut self) -> ErrintW<NistrSpec> {
        ErrintW::new(self, 15)
    }
}
#[doc = "Normal Interrupt Status\n\nYou can [`read`](crate::Reg::read) this register and get [`nistr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nistr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NistrSpec;
impl crate::RegisterSpec for NistrSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`nistr::R`](R) reader structure"]
impl crate::Readable for NistrSpec {}
#[doc = "`write(|w| ..)` method takes [`nistr::W`](W) writer structure"]
impl crate::Writable for NistrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets NISTR to value 0"]
impl crate::Resettable for NistrSpec {
    const RESET_VALUE: u16 = 0;
}
