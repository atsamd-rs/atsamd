#[doc = "Register `TMR` reader"]
pub type R = crate::R<TmrSpec>;
#[doc = "Register `TMR` writer"]
pub type W = crate::W<TmrSpec>;
#[doc = "DMA Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmaenselect {
    #[doc = "0: No data transfer or Non DMA data transfer"]
    Disable = 0,
    #[doc = "1: DMA data transfer"]
    Enable = 1,
}
impl From<Dmaenselect> for bool {
    #[inline(always)]
    fn from(variant: Dmaenselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAEN` reader - DMA Enable"]
pub type DmaenR = crate::BitReader<Dmaenselect>;
impl DmaenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmaenselect {
        match self.bits {
            false => Dmaenselect::Disable,
            true => Dmaenselect::Enable,
        }
    }
    #[doc = "No data transfer or Non DMA data transfer"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Dmaenselect::Disable
    }
    #[doc = "DMA data transfer"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Dmaenselect::Enable
    }
}
#[doc = "Field `DMAEN` writer - DMA Enable"]
pub type DmaenW<'a, REG> = crate::BitWriter<'a, REG, Dmaenselect>;
impl<'a, REG> DmaenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No data transfer or Non DMA data transfer"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Dmaenselect::Disable)
    }
    #[doc = "DMA data transfer"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Dmaenselect::Enable)
    }
}
#[doc = "Block Count Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bcenselect {
    #[doc = "0: Disable"]
    Disable = 0,
    #[doc = "1: Enable"]
    Enable = 1,
}
impl From<Bcenselect> for bool {
    #[inline(always)]
    fn from(variant: Bcenselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BCEN` reader - Block Count Enable"]
pub type BcenR = crate::BitReader<Bcenselect>;
impl BcenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bcenselect {
        match self.bits {
            false => Bcenselect::Disable,
            true => Bcenselect::Enable,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Bcenselect::Disable
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Bcenselect::Enable
    }
}
#[doc = "Field `BCEN` writer - Block Count Enable"]
pub type BcenW<'a, REG> = crate::BitWriter<'a, REG, Bcenselect>;
impl<'a, REG> BcenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Bcenselect::Disable)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Bcenselect::Enable)
    }
}
#[doc = "Auto Command Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Acmdenselect {
    #[doc = "0: Auto Command Disabled"]
    Disabled = 0,
    #[doc = "1: Auto CMD12 Enable"]
    Cmd12 = 1,
    #[doc = "2: Auto CMD23 Enable"]
    Cmd23 = 2,
}
impl From<Acmdenselect> for u8 {
    #[inline(always)]
    fn from(variant: Acmdenselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Acmdenselect {
    type Ux = u8;
}
impl crate::IsEnum for Acmdenselect {}
#[doc = "Field `ACMDEN` reader - Auto Command Enable"]
pub type AcmdenR = crate::FieldReader<Acmdenselect>;
impl AcmdenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Acmdenselect> {
        match self.bits {
            0 => Some(Acmdenselect::Disabled),
            1 => Some(Acmdenselect::Cmd12),
            2 => Some(Acmdenselect::Cmd23),
            _ => None,
        }
    }
    #[doc = "Auto Command Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Acmdenselect::Disabled
    }
    #[doc = "Auto CMD12 Enable"]
    #[inline(always)]
    pub fn is_cmd12(&self) -> bool {
        *self == Acmdenselect::Cmd12
    }
    #[doc = "Auto CMD23 Enable"]
    #[inline(always)]
    pub fn is_cmd23(&self) -> bool {
        *self == Acmdenselect::Cmd23
    }
}
#[doc = "Field `ACMDEN` writer - Auto Command Enable"]
pub type AcmdenW<'a, REG> = crate::FieldWriter<'a, REG, 2, Acmdenselect>;
impl<'a, REG> AcmdenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Auto Command Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Acmdenselect::Disabled)
    }
    #[doc = "Auto CMD12 Enable"]
    #[inline(always)]
    pub fn cmd12(self) -> &'a mut crate::W<REG> {
        self.variant(Acmdenselect::Cmd12)
    }
    #[doc = "Auto CMD23 Enable"]
    #[inline(always)]
    pub fn cmd23(self) -> &'a mut crate::W<REG> {
        self.variant(Acmdenselect::Cmd23)
    }
}
#[doc = "Data Transfer Direction Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dtdselselect {
    #[doc = "0: Write (Host to Card)"]
    Write = 0,
    #[doc = "1: Read (Card to Host)"]
    Read = 1,
}
impl From<Dtdselselect> for bool {
    #[inline(always)]
    fn from(variant: Dtdselselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DTDSEL` reader - Data Transfer Direction Selection"]
pub type DtdselR = crate::BitReader<Dtdselselect>;
impl DtdselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dtdselselect {
        match self.bits {
            false => Dtdselselect::Write,
            true => Dtdselselect::Read,
        }
    }
    #[doc = "Write (Host to Card)"]
    #[inline(always)]
    pub fn is_write(&self) -> bool {
        *self == Dtdselselect::Write
    }
    #[doc = "Read (Card to Host)"]
    #[inline(always)]
    pub fn is_read(&self) -> bool {
        *self == Dtdselselect::Read
    }
}
#[doc = "Field `DTDSEL` writer - Data Transfer Direction Selection"]
pub type DtdselW<'a, REG> = crate::BitWriter<'a, REG, Dtdselselect>;
impl<'a, REG> DtdselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Write (Host to Card)"]
    #[inline(always)]
    pub fn write(self) -> &'a mut crate::W<REG> {
        self.variant(Dtdselselect::Write)
    }
    #[doc = "Read (Card to Host)"]
    #[inline(always)]
    pub fn read(self) -> &'a mut crate::W<REG> {
        self.variant(Dtdselselect::Read)
    }
}
#[doc = "Multi/Single Block Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Msbselselect {
    #[doc = "0: Single Block"]
    Single = 0,
    #[doc = "1: Multiple Block"]
    Multiple = 1,
}
impl From<Msbselselect> for bool {
    #[inline(always)]
    fn from(variant: Msbselselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSBSEL` reader - Multi/Single Block Selection"]
pub type MsbselR = crate::BitReader<Msbselselect>;
impl MsbselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Msbselselect {
        match self.bits {
            false => Msbselselect::Single,
            true => Msbselselect::Multiple,
        }
    }
    #[doc = "Single Block"]
    #[inline(always)]
    pub fn is_single(&self) -> bool {
        *self == Msbselselect::Single
    }
    #[doc = "Multiple Block"]
    #[inline(always)]
    pub fn is_multiple(&self) -> bool {
        *self == Msbselselect::Multiple
    }
}
#[doc = "Field `MSBSEL` writer - Multi/Single Block Selection"]
pub type MsbselW<'a, REG> = crate::BitWriter<'a, REG, Msbselselect>;
impl<'a, REG> MsbselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Single Block"]
    #[inline(always)]
    pub fn single(self) -> &'a mut crate::W<REG> {
        self.variant(Msbselselect::Single)
    }
    #[doc = "Multiple Block"]
    #[inline(always)]
    pub fn multiple(self) -> &'a mut crate::W<REG> {
        self.variant(Msbselselect::Multiple)
    }
}
impl R {
    #[doc = "Bit 0 - DMA Enable"]
    #[inline(always)]
    pub fn dmaen(&self) -> DmaenR {
        DmaenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Block Count Enable"]
    #[inline(always)]
    pub fn bcen(&self) -> BcenR {
        BcenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Auto Command Enable"]
    #[inline(always)]
    pub fn acmden(&self) -> AcmdenR {
        AcmdenR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - Data Transfer Direction Selection"]
    #[inline(always)]
    pub fn dtdsel(&self) -> DtdselR {
        DtdselR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Multi/Single Block Selection"]
    #[inline(always)]
    pub fn msbsel(&self) -> MsbselR {
        MsbselR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DMA Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dmaen(&mut self) -> DmaenW<TmrSpec> {
        DmaenW::new(self, 0)
    }
    #[doc = "Bit 1 - Block Count Enable"]
    #[inline(always)]
    #[must_use]
    pub fn bcen(&mut self) -> BcenW<TmrSpec> {
        BcenW::new(self, 1)
    }
    #[doc = "Bits 2:3 - Auto Command Enable"]
    #[inline(always)]
    #[must_use]
    pub fn acmden(&mut self) -> AcmdenW<TmrSpec> {
        AcmdenW::new(self, 2)
    }
    #[doc = "Bit 4 - Data Transfer Direction Selection"]
    #[inline(always)]
    #[must_use]
    pub fn dtdsel(&mut self) -> DtdselW<TmrSpec> {
        DtdselW::new(self, 4)
    }
    #[doc = "Bit 5 - Multi/Single Block Selection"]
    #[inline(always)]
    #[must_use]
    pub fn msbsel(&mut self) -> MsbselW<TmrSpec> {
        MsbselW::new(self, 5)
    }
}
#[doc = "Transfer Mode\n\nYou can [`read`](crate::Reg::read) this register and get [`tmr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tmr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TmrSpec;
impl crate::RegisterSpec for TmrSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`tmr::R`](R) reader structure"]
impl crate::Readable for TmrSpec {}
#[doc = "`write(|w| ..)` method takes [`tmr::W`](W) writer structure"]
impl crate::Writable for TmrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets TMR to value 0"]
impl crate::Resettable for TmrSpec {
    const RESET_VALUE: u16 = 0;
}
