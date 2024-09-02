#[doc = "Register `CTRLB` reader"]
pub type R = crate::R<CtrlbSpec>;
#[doc = "Register `CTRLB` writer"]
pub type W = crate::W<CtrlbSpec>;
#[doc = "Serial Memory Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Modeselect {
    #[doc = "0: SPI operating mode"]
    Spi = 0,
    #[doc = "1: Serial Memory operating mode"]
    Memory = 1,
}
impl From<Modeselect> for bool {
    #[inline(always)]
    fn from(variant: Modeselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MODE` reader - Serial Memory Mode"]
pub type ModeR = crate::BitReader<Modeselect>;
impl ModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Modeselect {
        match self.bits {
            false => Modeselect::Spi,
            true => Modeselect::Memory,
        }
    }
    #[doc = "SPI operating mode"]
    #[inline(always)]
    pub fn is_spi(&self) -> bool {
        *self == Modeselect::Spi
    }
    #[doc = "Serial Memory operating mode"]
    #[inline(always)]
    pub fn is_memory(&self) -> bool {
        *self == Modeselect::Memory
    }
}
#[doc = "Field `MODE` writer - Serial Memory Mode"]
pub type ModeW<'a, REG> = crate::BitWriter<'a, REG, Modeselect>;
impl<'a, REG> ModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPI operating mode"]
    #[inline(always)]
    pub fn spi(self) -> &'a mut crate::W<REG> {
        self.variant(Modeselect::Spi)
    }
    #[doc = "Serial Memory operating mode"]
    #[inline(always)]
    pub fn memory(self) -> &'a mut crate::W<REG> {
        self.variant(Modeselect::Memory)
    }
}
#[doc = "Local Loopback Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Loopenselect {
    #[doc = "0: Local Loopback is disabled"]
    Disabled = 0,
    #[doc = "1: Local Loopback is enabled"]
    Enabled = 1,
}
impl From<Loopenselect> for bool {
    #[inline(always)]
    fn from(variant: Loopenselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LOOPEN` reader - Local Loopback Enable"]
pub type LoopenR = crate::BitReader<Loopenselect>;
impl LoopenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Loopenselect {
        match self.bits {
            false => Loopenselect::Disabled,
            true => Loopenselect::Enabled,
        }
    }
    #[doc = "Local Loopback is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Loopenselect::Disabled
    }
    #[doc = "Local Loopback is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Loopenselect::Enabled
    }
}
#[doc = "Field `LOOPEN` writer - Local Loopback Enable"]
pub type LoopenW<'a, REG> = crate::BitWriter<'a, REG, Loopenselect>;
impl<'a, REG> LoopenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Local Loopback is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Loopenselect::Disabled)
    }
    #[doc = "Local Loopback is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Loopenselect::Enabled)
    }
}
#[doc = "Field `WDRBT` reader - Wait Data Read Before Transfer"]
pub type WdrbtR = crate::BitReader;
#[doc = "Field `WDRBT` writer - Wait Data Read Before Transfer"]
pub type WdrbtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SMEMREG` reader - Serial Memory reg"]
pub type SmemregR = crate::BitReader;
#[doc = "Field `SMEMREG` writer - Serial Memory reg"]
pub type SmemregW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Chip Select Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Csmodeselect {
    #[doc = "0: The chip select is deasserted if TD has not been reloaded before the end of the current transfer."]
    Noreload = 0,
    #[doc = "1: The chip select is deasserted when the bit LASTXFER is written at 1 and the character written in TD has been transferred."]
    Lastxfer = 1,
    #[doc = "2: The chip select is deasserted systematically after each transfer."]
    Systematically = 2,
}
impl From<Csmodeselect> for u8 {
    #[inline(always)]
    fn from(variant: Csmodeselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Csmodeselect {
    type Ux = u8;
}
impl crate::IsEnum for Csmodeselect {}
#[doc = "Field `CSMODE` reader - Chip Select Mode"]
pub type CsmodeR = crate::FieldReader<Csmodeselect>;
impl CsmodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Csmodeselect> {
        match self.bits {
            0 => Some(Csmodeselect::Noreload),
            1 => Some(Csmodeselect::Lastxfer),
            2 => Some(Csmodeselect::Systematically),
            _ => None,
        }
    }
    #[doc = "The chip select is deasserted if TD has not been reloaded before the end of the current transfer."]
    #[inline(always)]
    pub fn is_noreload(&self) -> bool {
        *self == Csmodeselect::Noreload
    }
    #[doc = "The chip select is deasserted when the bit LASTXFER is written at 1 and the character written in TD has been transferred."]
    #[inline(always)]
    pub fn is_lastxfer(&self) -> bool {
        *self == Csmodeselect::Lastxfer
    }
    #[doc = "The chip select is deasserted systematically after each transfer."]
    #[inline(always)]
    pub fn is_systematically(&self) -> bool {
        *self == Csmodeselect::Systematically
    }
}
#[doc = "Field `CSMODE` writer - Chip Select Mode"]
pub type CsmodeW<'a, REG> = crate::FieldWriter<'a, REG, 2, Csmodeselect>;
impl<'a, REG> CsmodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "The chip select is deasserted if TD has not been reloaded before the end of the current transfer."]
    #[inline(always)]
    pub fn noreload(self) -> &'a mut crate::W<REG> {
        self.variant(Csmodeselect::Noreload)
    }
    #[doc = "The chip select is deasserted when the bit LASTXFER is written at 1 and the character written in TD has been transferred."]
    #[inline(always)]
    pub fn lastxfer(self) -> &'a mut crate::W<REG> {
        self.variant(Csmodeselect::Lastxfer)
    }
    #[doc = "The chip select is deasserted systematically after each transfer."]
    #[inline(always)]
    pub fn systematically(self) -> &'a mut crate::W<REG> {
        self.variant(Csmodeselect::Systematically)
    }
}
#[doc = "Data Length\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Datalenselect {
    #[doc = "0: 8-bits transfer"]
    _8bits = 0,
    #[doc = "1: 9 bits transfer"]
    _9bits = 1,
    #[doc = "2: 10-bits transfer"]
    _10bits = 2,
    #[doc = "3: 11-bits transfer"]
    _11bits = 3,
    #[doc = "4: 12-bits transfer"]
    _12bits = 4,
    #[doc = "5: 13-bits transfer"]
    _13bits = 5,
    #[doc = "6: 14-bits transfer"]
    _14bits = 6,
    #[doc = "7: 15-bits transfer"]
    _15bits = 7,
    #[doc = "8: 16-bits transfer"]
    _16bits = 8,
}
impl From<Datalenselect> for u8 {
    #[inline(always)]
    fn from(variant: Datalenselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Datalenselect {
    type Ux = u8;
}
impl crate::IsEnum for Datalenselect {}
#[doc = "Field `DATALEN` reader - Data Length"]
pub type DatalenR = crate::FieldReader<Datalenselect>;
impl DatalenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Datalenselect> {
        match self.bits {
            0 => Some(Datalenselect::_8bits),
            1 => Some(Datalenselect::_9bits),
            2 => Some(Datalenselect::_10bits),
            3 => Some(Datalenselect::_11bits),
            4 => Some(Datalenselect::_12bits),
            5 => Some(Datalenselect::_13bits),
            6 => Some(Datalenselect::_14bits),
            7 => Some(Datalenselect::_15bits),
            8 => Some(Datalenselect::_16bits),
            _ => None,
        }
    }
    #[doc = "8-bits transfer"]
    #[inline(always)]
    pub fn is_8bits(&self) -> bool {
        *self == Datalenselect::_8bits
    }
    #[doc = "9 bits transfer"]
    #[inline(always)]
    pub fn is_9bits(&self) -> bool {
        *self == Datalenselect::_9bits
    }
    #[doc = "10-bits transfer"]
    #[inline(always)]
    pub fn is_10bits(&self) -> bool {
        *self == Datalenselect::_10bits
    }
    #[doc = "11-bits transfer"]
    #[inline(always)]
    pub fn is_11bits(&self) -> bool {
        *self == Datalenselect::_11bits
    }
    #[doc = "12-bits transfer"]
    #[inline(always)]
    pub fn is_12bits(&self) -> bool {
        *self == Datalenselect::_12bits
    }
    #[doc = "13-bits transfer"]
    #[inline(always)]
    pub fn is_13bits(&self) -> bool {
        *self == Datalenselect::_13bits
    }
    #[doc = "14-bits transfer"]
    #[inline(always)]
    pub fn is_14bits(&self) -> bool {
        *self == Datalenselect::_14bits
    }
    #[doc = "15-bits transfer"]
    #[inline(always)]
    pub fn is_15bits(&self) -> bool {
        *self == Datalenselect::_15bits
    }
    #[doc = "16-bits transfer"]
    #[inline(always)]
    pub fn is_16bits(&self) -> bool {
        *self == Datalenselect::_16bits
    }
}
#[doc = "Field `DATALEN` writer - Data Length"]
pub type DatalenW<'a, REG> = crate::FieldWriter<'a, REG, 4, Datalenselect>;
impl<'a, REG> DatalenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "8-bits transfer"]
    #[inline(always)]
    pub fn _8bits(self) -> &'a mut crate::W<REG> {
        self.variant(Datalenselect::_8bits)
    }
    #[doc = "9 bits transfer"]
    #[inline(always)]
    pub fn _9bits(self) -> &'a mut crate::W<REG> {
        self.variant(Datalenselect::_9bits)
    }
    #[doc = "10-bits transfer"]
    #[inline(always)]
    pub fn _10bits(self) -> &'a mut crate::W<REG> {
        self.variant(Datalenselect::_10bits)
    }
    #[doc = "11-bits transfer"]
    #[inline(always)]
    pub fn _11bits(self) -> &'a mut crate::W<REG> {
        self.variant(Datalenselect::_11bits)
    }
    #[doc = "12-bits transfer"]
    #[inline(always)]
    pub fn _12bits(self) -> &'a mut crate::W<REG> {
        self.variant(Datalenselect::_12bits)
    }
    #[doc = "13-bits transfer"]
    #[inline(always)]
    pub fn _13bits(self) -> &'a mut crate::W<REG> {
        self.variant(Datalenselect::_13bits)
    }
    #[doc = "14-bits transfer"]
    #[inline(always)]
    pub fn _14bits(self) -> &'a mut crate::W<REG> {
        self.variant(Datalenselect::_14bits)
    }
    #[doc = "15-bits transfer"]
    #[inline(always)]
    pub fn _15bits(self) -> &'a mut crate::W<REG> {
        self.variant(Datalenselect::_15bits)
    }
    #[doc = "16-bits transfer"]
    #[inline(always)]
    pub fn _16bits(self) -> &'a mut crate::W<REG> {
        self.variant(Datalenselect::_16bits)
    }
}
#[doc = "Field `DLYBCT` reader - Delay Between Consecutive Transfers"]
pub type DlybctR = crate::FieldReader;
#[doc = "Field `DLYBCT` writer - Delay Between Consecutive Transfers"]
pub type DlybctW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DLYCS` reader - Minimum Inactive CS Delay"]
pub type DlycsR = crate::FieldReader;
#[doc = "Field `DLYCS` writer - Minimum Inactive CS Delay"]
pub type DlycsW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0 - Serial Memory Mode"]
    #[inline(always)]
    pub fn mode(&self) -> ModeR {
        ModeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Local Loopback Enable"]
    #[inline(always)]
    pub fn loopen(&self) -> LoopenR {
        LoopenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Wait Data Read Before Transfer"]
    #[inline(always)]
    pub fn wdrbt(&self) -> WdrbtR {
        WdrbtR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Serial Memory reg"]
    #[inline(always)]
    pub fn smemreg(&self) -> SmemregR {
        SmemregR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Chip Select Mode"]
    #[inline(always)]
    pub fn csmode(&self) -> CsmodeR {
        CsmodeR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:11 - Data Length"]
    #[inline(always)]
    pub fn datalen(&self) -> DatalenR {
        DatalenR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:23 - Delay Between Consecutive Transfers"]
    #[inline(always)]
    pub fn dlybct(&self) -> DlybctR {
        DlybctR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Minimum Inactive CS Delay"]
    #[inline(always)]
    pub fn dlycs(&self) -> DlycsR {
        DlycsR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Serial Memory Mode"]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> ModeW<CtrlbSpec> {
        ModeW::new(self, 0)
    }
    #[doc = "Bit 1 - Local Loopback Enable"]
    #[inline(always)]
    #[must_use]
    pub fn loopen(&mut self) -> LoopenW<CtrlbSpec> {
        LoopenW::new(self, 1)
    }
    #[doc = "Bit 2 - Wait Data Read Before Transfer"]
    #[inline(always)]
    #[must_use]
    pub fn wdrbt(&mut self) -> WdrbtW<CtrlbSpec> {
        WdrbtW::new(self, 2)
    }
    #[doc = "Bit 3 - Serial Memory reg"]
    #[inline(always)]
    #[must_use]
    pub fn smemreg(&mut self) -> SmemregW<CtrlbSpec> {
        SmemregW::new(self, 3)
    }
    #[doc = "Bits 4:5 - Chip Select Mode"]
    #[inline(always)]
    #[must_use]
    pub fn csmode(&mut self) -> CsmodeW<CtrlbSpec> {
        CsmodeW::new(self, 4)
    }
    #[doc = "Bits 8:11 - Data Length"]
    #[inline(always)]
    #[must_use]
    pub fn datalen(&mut self) -> DatalenW<CtrlbSpec> {
        DatalenW::new(self, 8)
    }
    #[doc = "Bits 16:23 - Delay Between Consecutive Transfers"]
    #[inline(always)]
    #[must_use]
    pub fn dlybct(&mut self) -> DlybctW<CtrlbSpec> {
        DlybctW::new(self, 16)
    }
    #[doc = "Bits 24:31 - Minimum Inactive CS Delay"]
    #[inline(always)]
    #[must_use]
    pub fn dlycs(&mut self) -> DlycsW<CtrlbSpec> {
        DlycsW::new(self, 24)
    }
}
#[doc = "Control B\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrlb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrlb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlbSpec;
impl crate::RegisterSpec for CtrlbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrlb::R`](R) reader structure"]
impl crate::Readable for CtrlbSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrlb::W`](W) writer structure"]
impl crate::Writable for CtrlbSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTRLB to value 0"]
impl crate::Resettable for CtrlbSpec {
    const RESET_VALUE: u32 = 0;
}
