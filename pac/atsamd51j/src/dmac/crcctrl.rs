#[doc = "Register `CRCCTRL` reader"]
pub type R = crate::R<CrcctrlSpec>;
#[doc = "Register `CRCCTRL` writer"]
pub type W = crate::W<CrcctrlSpec>;
#[doc = "CRC Beat Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Crcbeatsizeselect {
    #[doc = "0: 8-bit bus transfer"]
    Byte = 0,
    #[doc = "1: 16-bit bus transfer"]
    Hword = 1,
    #[doc = "2: 32-bit bus transfer"]
    Word = 2,
}
impl From<Crcbeatsizeselect> for u8 {
    #[inline(always)]
    fn from(variant: Crcbeatsizeselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Crcbeatsizeselect {
    type Ux = u8;
}
impl crate::IsEnum for Crcbeatsizeselect {}
#[doc = "Field `CRCBEATSIZE` reader - CRC Beat Size"]
pub type CrcbeatsizeR = crate::FieldReader<Crcbeatsizeselect>;
impl CrcbeatsizeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Crcbeatsizeselect> {
        match self.bits {
            0 => Some(Crcbeatsizeselect::Byte),
            1 => Some(Crcbeatsizeselect::Hword),
            2 => Some(Crcbeatsizeselect::Word),
            _ => None,
        }
    }
    #[doc = "8-bit bus transfer"]
    #[inline(always)]
    pub fn is_byte(&self) -> bool {
        *self == Crcbeatsizeselect::Byte
    }
    #[doc = "16-bit bus transfer"]
    #[inline(always)]
    pub fn is_hword(&self) -> bool {
        *self == Crcbeatsizeselect::Hword
    }
    #[doc = "32-bit bus transfer"]
    #[inline(always)]
    pub fn is_word(&self) -> bool {
        *self == Crcbeatsizeselect::Word
    }
}
#[doc = "Field `CRCBEATSIZE` writer - CRC Beat Size"]
pub type CrcbeatsizeW<'a, REG> = crate::FieldWriter<'a, REG, 2, Crcbeatsizeselect>;
impl<'a, REG> CrcbeatsizeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "8-bit bus transfer"]
    #[inline(always)]
    pub fn byte(self) -> &'a mut crate::W<REG> {
        self.variant(Crcbeatsizeselect::Byte)
    }
    #[doc = "16-bit bus transfer"]
    #[inline(always)]
    pub fn hword(self) -> &'a mut crate::W<REG> {
        self.variant(Crcbeatsizeselect::Hword)
    }
    #[doc = "32-bit bus transfer"]
    #[inline(always)]
    pub fn word(self) -> &'a mut crate::W<REG> {
        self.variant(Crcbeatsizeselect::Word)
    }
}
#[doc = "CRC Polynomial Type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Crcpolyselect {
    #[doc = "0: CRC-16 (CRC-CCITT)"]
    Crc16 = 0,
    #[doc = "1: CRC32 (IEEE 802.3)"]
    Crc32 = 1,
}
impl From<Crcpolyselect> for u8 {
    #[inline(always)]
    fn from(variant: Crcpolyselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Crcpolyselect {
    type Ux = u8;
}
impl crate::IsEnum for Crcpolyselect {}
#[doc = "Field `CRCPOLY` reader - CRC Polynomial Type"]
pub type CrcpolyR = crate::FieldReader<Crcpolyselect>;
impl CrcpolyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Crcpolyselect> {
        match self.bits {
            0 => Some(Crcpolyselect::Crc16),
            1 => Some(Crcpolyselect::Crc32),
            _ => None,
        }
    }
    #[doc = "CRC-16 (CRC-CCITT)"]
    #[inline(always)]
    pub fn is_crc16(&self) -> bool {
        *self == Crcpolyselect::Crc16
    }
    #[doc = "CRC32 (IEEE 802.3)"]
    #[inline(always)]
    pub fn is_crc32(&self) -> bool {
        *self == Crcpolyselect::Crc32
    }
}
#[doc = "Field `CRCPOLY` writer - CRC Polynomial Type"]
pub type CrcpolyW<'a, REG> = crate::FieldWriter<'a, REG, 2, Crcpolyselect>;
impl<'a, REG> CrcpolyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CRC-16 (CRC-CCITT)"]
    #[inline(always)]
    pub fn crc16(self) -> &'a mut crate::W<REG> {
        self.variant(Crcpolyselect::Crc16)
    }
    #[doc = "CRC32 (IEEE 802.3)"]
    #[inline(always)]
    pub fn crc32(self) -> &'a mut crate::W<REG> {
        self.variant(Crcpolyselect::Crc32)
    }
}
#[doc = "CRC Input Source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Crcsrcselect {
    #[doc = "0: CRC Disabled"]
    Disable = 0,
    #[doc = "1: I/O interface"]
    Io = 1,
}
impl From<Crcsrcselect> for u8 {
    #[inline(always)]
    fn from(variant: Crcsrcselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Crcsrcselect {
    type Ux = u8;
}
impl crate::IsEnum for Crcsrcselect {}
#[doc = "Field `CRCSRC` reader - CRC Input Source"]
pub type CrcsrcR = crate::FieldReader<Crcsrcselect>;
impl CrcsrcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Crcsrcselect> {
        match self.bits {
            0 => Some(Crcsrcselect::Disable),
            1 => Some(Crcsrcselect::Io),
            _ => None,
        }
    }
    #[doc = "CRC Disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Crcsrcselect::Disable
    }
    #[doc = "I/O interface"]
    #[inline(always)]
    pub fn is_io(&self) -> bool {
        *self == Crcsrcselect::Io
    }
}
#[doc = "Field `CRCSRC` writer - CRC Input Source"]
pub type CrcsrcW<'a, REG> = crate::FieldWriter<'a, REG, 6, Crcsrcselect>;
impl<'a, REG> CrcsrcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CRC Disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Crcsrcselect::Disable)
    }
    #[doc = "I/O interface"]
    #[inline(always)]
    pub fn io(self) -> &'a mut crate::W<REG> {
        self.variant(Crcsrcselect::Io)
    }
}
#[doc = "CRC Operating Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Crcmodeselect {
    #[doc = "0: Default operating mode"]
    Default = 0,
    #[doc = "2: Memory CRC monitor operating mode"]
    Crcmon = 2,
    #[doc = "3: Memory CRC generation operating mode"]
    Crcgen = 3,
}
impl From<Crcmodeselect> for u8 {
    #[inline(always)]
    fn from(variant: Crcmodeselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Crcmodeselect {
    type Ux = u8;
}
impl crate::IsEnum for Crcmodeselect {}
#[doc = "Field `CRCMODE` reader - CRC Operating Mode"]
pub type CrcmodeR = crate::FieldReader<Crcmodeselect>;
impl CrcmodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Crcmodeselect> {
        match self.bits {
            0 => Some(Crcmodeselect::Default),
            2 => Some(Crcmodeselect::Crcmon),
            3 => Some(Crcmodeselect::Crcgen),
            _ => None,
        }
    }
    #[doc = "Default operating mode"]
    #[inline(always)]
    pub fn is_default(&self) -> bool {
        *self == Crcmodeselect::Default
    }
    #[doc = "Memory CRC monitor operating mode"]
    #[inline(always)]
    pub fn is_crcmon(&self) -> bool {
        *self == Crcmodeselect::Crcmon
    }
    #[doc = "Memory CRC generation operating mode"]
    #[inline(always)]
    pub fn is_crcgen(&self) -> bool {
        *self == Crcmodeselect::Crcgen
    }
}
#[doc = "Field `CRCMODE` writer - CRC Operating Mode"]
pub type CrcmodeW<'a, REG> = crate::FieldWriter<'a, REG, 2, Crcmodeselect>;
impl<'a, REG> CrcmodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Default operating mode"]
    #[inline(always)]
    pub fn default(self) -> &'a mut crate::W<REG> {
        self.variant(Crcmodeselect::Default)
    }
    #[doc = "Memory CRC monitor operating mode"]
    #[inline(always)]
    pub fn crcmon(self) -> &'a mut crate::W<REG> {
        self.variant(Crcmodeselect::Crcmon)
    }
    #[doc = "Memory CRC generation operating mode"]
    #[inline(always)]
    pub fn crcgen(self) -> &'a mut crate::W<REG> {
        self.variant(Crcmodeselect::Crcgen)
    }
}
impl R {
    #[doc = "Bits 0:1 - CRC Beat Size"]
    #[inline(always)]
    pub fn crcbeatsize(&self) -> CrcbeatsizeR {
        CrcbeatsizeR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - CRC Polynomial Type"]
    #[inline(always)]
    pub fn crcpoly(&self) -> CrcpolyR {
        CrcpolyR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 8:13 - CRC Input Source"]
    #[inline(always)]
    pub fn crcsrc(&self) -> CrcsrcR {
        CrcsrcR::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 14:15 - CRC Operating Mode"]
    #[inline(always)]
    pub fn crcmode(&self) -> CrcmodeR {
        CrcmodeR::new(((self.bits >> 14) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - CRC Beat Size"]
    #[inline(always)]
    #[must_use]
    pub fn crcbeatsize(&mut self) -> CrcbeatsizeW<CrcctrlSpec> {
        CrcbeatsizeW::new(self, 0)
    }
    #[doc = "Bits 2:3 - CRC Polynomial Type"]
    #[inline(always)]
    #[must_use]
    pub fn crcpoly(&mut self) -> CrcpolyW<CrcctrlSpec> {
        CrcpolyW::new(self, 2)
    }
    #[doc = "Bits 8:13 - CRC Input Source"]
    #[inline(always)]
    #[must_use]
    pub fn crcsrc(&mut self) -> CrcsrcW<CrcctrlSpec> {
        CrcsrcW::new(self, 8)
    }
    #[doc = "Bits 14:15 - CRC Operating Mode"]
    #[inline(always)]
    #[must_use]
    pub fn crcmode(&mut self) -> CrcmodeW<CrcctrlSpec> {
        CrcmodeW::new(self, 14)
    }
}
#[doc = "CRC Control\n\nYou can [`read`](crate::Reg::read) this register and get [`crcctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crcctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrcctrlSpec;
impl crate::RegisterSpec for CrcctrlSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`crcctrl::R`](R) reader structure"]
impl crate::Readable for CrcctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`crcctrl::W`](W) writer structure"]
impl crate::Writable for CrcctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets CRCCTRL to value 0"]
impl crate::Resettable for CrcctrlSpec {
    const RESET_VALUE: u16 = 0;
}
