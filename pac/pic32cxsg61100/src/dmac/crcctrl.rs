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
    #[doc = "32: DMA Channel 0"]
    Chn0 = 32,
    #[doc = "33: DMA Channel 1"]
    Chn1 = 33,
    #[doc = "34: DMA Channel 2"]
    Chn2 = 34,
    #[doc = "35: DMA Channel 3"]
    Chn3 = 35,
    #[doc = "36: DMA Channel 4"]
    Chn4 = 36,
    #[doc = "37: DMA Channel 5"]
    Chn5 = 37,
    #[doc = "38: DMA Channel 6"]
    Chn6 = 38,
    #[doc = "39: DMA Channel 7"]
    Chn7 = 39,
    #[doc = "40: DMA Channel 8"]
    Chn8 = 40,
    #[doc = "41: DMA Channel 9"]
    Chn9 = 41,
    #[doc = "42: DMA Channel 10"]
    Chn10 = 42,
    #[doc = "43: DMA Channel 11"]
    Chn11 = 43,
    #[doc = "44: DMA Channel 12"]
    Chn12 = 44,
    #[doc = "45: DMA Channel 13"]
    Chn13 = 45,
    #[doc = "46: DMA Channel 14"]
    Chn14 = 46,
    #[doc = "47: DMA Channel 15"]
    Chn15 = 47,
    #[doc = "48: DMA Channel 16"]
    Chn16 = 48,
    #[doc = "49: DMA Channel 17"]
    Chn17 = 49,
    #[doc = "50: DMA Channel 18"]
    Chn18 = 50,
    #[doc = "51: DMA Channel 19"]
    Chn19 = 51,
    #[doc = "52: DMA Channel 20"]
    Chn20 = 52,
    #[doc = "53: DMA Channel 21"]
    Chn21 = 53,
    #[doc = "54: DMA Channel 22"]
    Chn22 = 54,
    #[doc = "55: DMA Channel 23"]
    Chn23 = 55,
    #[doc = "56: DMA Channel 24"]
    Chn24 = 56,
    #[doc = "57: DMA Channel 25"]
    Chn25 = 57,
    #[doc = "58: DMA Channel 26"]
    Chn26 = 58,
    #[doc = "59: DMA Channel 27"]
    Chn27 = 59,
    #[doc = "60: DMA Channel 28"]
    Chn28 = 60,
    #[doc = "61: DMA Channel 29"]
    Chn29 = 61,
    #[doc = "62: DMA Channel 30"]
    Chn30 = 62,
    #[doc = "63: DMA Channel 31"]
    Chn31 = 63,
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
            32 => Some(Crcsrcselect::Chn0),
            33 => Some(Crcsrcselect::Chn1),
            34 => Some(Crcsrcselect::Chn2),
            35 => Some(Crcsrcselect::Chn3),
            36 => Some(Crcsrcselect::Chn4),
            37 => Some(Crcsrcselect::Chn5),
            38 => Some(Crcsrcselect::Chn6),
            39 => Some(Crcsrcselect::Chn7),
            40 => Some(Crcsrcselect::Chn8),
            41 => Some(Crcsrcselect::Chn9),
            42 => Some(Crcsrcselect::Chn10),
            43 => Some(Crcsrcselect::Chn11),
            44 => Some(Crcsrcselect::Chn12),
            45 => Some(Crcsrcselect::Chn13),
            46 => Some(Crcsrcselect::Chn14),
            47 => Some(Crcsrcselect::Chn15),
            48 => Some(Crcsrcselect::Chn16),
            49 => Some(Crcsrcselect::Chn17),
            50 => Some(Crcsrcselect::Chn18),
            51 => Some(Crcsrcselect::Chn19),
            52 => Some(Crcsrcselect::Chn20),
            53 => Some(Crcsrcselect::Chn21),
            54 => Some(Crcsrcselect::Chn22),
            55 => Some(Crcsrcselect::Chn23),
            56 => Some(Crcsrcselect::Chn24),
            57 => Some(Crcsrcselect::Chn25),
            58 => Some(Crcsrcselect::Chn26),
            59 => Some(Crcsrcselect::Chn27),
            60 => Some(Crcsrcselect::Chn28),
            61 => Some(Crcsrcselect::Chn29),
            62 => Some(Crcsrcselect::Chn30),
            63 => Some(Crcsrcselect::Chn31),
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
    #[doc = "DMA Channel 0"]
    #[inline(always)]
    pub fn is_chn0(&self) -> bool {
        *self == Crcsrcselect::Chn0
    }
    #[doc = "DMA Channel 1"]
    #[inline(always)]
    pub fn is_chn1(&self) -> bool {
        *self == Crcsrcselect::Chn1
    }
    #[doc = "DMA Channel 2"]
    #[inline(always)]
    pub fn is_chn2(&self) -> bool {
        *self == Crcsrcselect::Chn2
    }
    #[doc = "DMA Channel 3"]
    #[inline(always)]
    pub fn is_chn3(&self) -> bool {
        *self == Crcsrcselect::Chn3
    }
    #[doc = "DMA Channel 4"]
    #[inline(always)]
    pub fn is_chn4(&self) -> bool {
        *self == Crcsrcselect::Chn4
    }
    #[doc = "DMA Channel 5"]
    #[inline(always)]
    pub fn is_chn5(&self) -> bool {
        *self == Crcsrcselect::Chn5
    }
    #[doc = "DMA Channel 6"]
    #[inline(always)]
    pub fn is_chn6(&self) -> bool {
        *self == Crcsrcselect::Chn6
    }
    #[doc = "DMA Channel 7"]
    #[inline(always)]
    pub fn is_chn7(&self) -> bool {
        *self == Crcsrcselect::Chn7
    }
    #[doc = "DMA Channel 8"]
    #[inline(always)]
    pub fn is_chn8(&self) -> bool {
        *self == Crcsrcselect::Chn8
    }
    #[doc = "DMA Channel 9"]
    #[inline(always)]
    pub fn is_chn9(&self) -> bool {
        *self == Crcsrcselect::Chn9
    }
    #[doc = "DMA Channel 10"]
    #[inline(always)]
    pub fn is_chn10(&self) -> bool {
        *self == Crcsrcselect::Chn10
    }
    #[doc = "DMA Channel 11"]
    #[inline(always)]
    pub fn is_chn11(&self) -> bool {
        *self == Crcsrcselect::Chn11
    }
    #[doc = "DMA Channel 12"]
    #[inline(always)]
    pub fn is_chn12(&self) -> bool {
        *self == Crcsrcselect::Chn12
    }
    #[doc = "DMA Channel 13"]
    #[inline(always)]
    pub fn is_chn13(&self) -> bool {
        *self == Crcsrcselect::Chn13
    }
    #[doc = "DMA Channel 14"]
    #[inline(always)]
    pub fn is_chn14(&self) -> bool {
        *self == Crcsrcselect::Chn14
    }
    #[doc = "DMA Channel 15"]
    #[inline(always)]
    pub fn is_chn15(&self) -> bool {
        *self == Crcsrcselect::Chn15
    }
    #[doc = "DMA Channel 16"]
    #[inline(always)]
    pub fn is_chn16(&self) -> bool {
        *self == Crcsrcselect::Chn16
    }
    #[doc = "DMA Channel 17"]
    #[inline(always)]
    pub fn is_chn17(&self) -> bool {
        *self == Crcsrcselect::Chn17
    }
    #[doc = "DMA Channel 18"]
    #[inline(always)]
    pub fn is_chn18(&self) -> bool {
        *self == Crcsrcselect::Chn18
    }
    #[doc = "DMA Channel 19"]
    #[inline(always)]
    pub fn is_chn19(&self) -> bool {
        *self == Crcsrcselect::Chn19
    }
    #[doc = "DMA Channel 20"]
    #[inline(always)]
    pub fn is_chn20(&self) -> bool {
        *self == Crcsrcselect::Chn20
    }
    #[doc = "DMA Channel 21"]
    #[inline(always)]
    pub fn is_chn21(&self) -> bool {
        *self == Crcsrcselect::Chn21
    }
    #[doc = "DMA Channel 22"]
    #[inline(always)]
    pub fn is_chn22(&self) -> bool {
        *self == Crcsrcselect::Chn22
    }
    #[doc = "DMA Channel 23"]
    #[inline(always)]
    pub fn is_chn23(&self) -> bool {
        *self == Crcsrcselect::Chn23
    }
    #[doc = "DMA Channel 24"]
    #[inline(always)]
    pub fn is_chn24(&self) -> bool {
        *self == Crcsrcselect::Chn24
    }
    #[doc = "DMA Channel 25"]
    #[inline(always)]
    pub fn is_chn25(&self) -> bool {
        *self == Crcsrcselect::Chn25
    }
    #[doc = "DMA Channel 26"]
    #[inline(always)]
    pub fn is_chn26(&self) -> bool {
        *self == Crcsrcselect::Chn26
    }
    #[doc = "DMA Channel 27"]
    #[inline(always)]
    pub fn is_chn27(&self) -> bool {
        *self == Crcsrcselect::Chn27
    }
    #[doc = "DMA Channel 28"]
    #[inline(always)]
    pub fn is_chn28(&self) -> bool {
        *self == Crcsrcselect::Chn28
    }
    #[doc = "DMA Channel 29"]
    #[inline(always)]
    pub fn is_chn29(&self) -> bool {
        *self == Crcsrcselect::Chn29
    }
    #[doc = "DMA Channel 30"]
    #[inline(always)]
    pub fn is_chn30(&self) -> bool {
        *self == Crcsrcselect::Chn30
    }
    #[doc = "DMA Channel 31"]
    #[inline(always)]
    pub fn is_chn31(&self) -> bool {
        *self == Crcsrcselect::Chn31
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
    #[doc = "DMA Channel 0"]
    #[inline(always)]
    pub fn chn0(self) -> &'a mut crate::W<REG> {
        self.variant(Crcsrcselect::Chn0)
    }
    #[doc = "DMA Channel 1"]
    #[inline(always)]
    pub fn chn1(self) -> &'a mut crate::W<REG> {
        self.variant(Crcsrcselect::Chn1)
    }
    #[doc = "DMA Channel 2"]
    #[inline(always)]
    pub fn chn2(self) -> &'a mut crate::W<REG> {
        self.variant(Crcsrcselect::Chn2)
    }
    #[doc = "DMA Channel 3"]
    #[inline(always)]
    pub fn chn3(self) -> &'a mut crate::W<REG> {
        self.variant(Crcsrcselect::Chn3)
    }
    #[doc = "DMA Channel 4"]
    #[inline(always)]
    pub fn chn4(self) -> &'a mut crate::W<REG> {
        self.variant(Crcsrcselect::Chn4)
    }
    #[doc = "DMA Channel 5"]
    #[inline(always)]
    pub fn chn5(self) -> &'a mut crate::W<REG> {
        self.variant(Crcsrcselect::Chn5)
    }
    #[doc = "DMA Channel 6"]
    #[inline(always)]
    pub fn chn6(self) -> &'a mut crate::W<REG> {
        self.variant(Crcsrcselect::Chn6)
    }
    #[doc = "DMA Channel 7"]
    #[inline(always)]
    pub fn chn7(self) -> &'a mut crate::W<REG> {
        self.variant(Crcsrcselect::Chn7)
    }
    #[doc = "DMA Channel 8"]
    #[inline(always)]
    pub fn chn8(self) -> &'a mut crate::W<REG> {
        self.variant(Crcsrcselect::Chn8)
    }
    #[doc = "DMA Channel 9"]
    #[inline(always)]
    pub fn chn9(self) -> &'a mut crate::W<REG> {
        self.variant(Crcsrcselect::Chn9)
    }
    #[doc = "DMA Channel 10"]
    #[inline(always)]
    pub fn chn10(self) -> &'a mut crate::W<REG> {
        self.variant(Crcsrcselect::Chn10)
    }
    #[doc = "DMA Channel 11"]
    #[inline(always)]
    pub fn chn11(self) -> &'a mut crate::W<REG> {
        self.variant(Crcsrcselect::Chn11)
    }
    #[doc = "DMA Channel 12"]
    #[inline(always)]
    pub fn chn12(self) -> &'a mut crate::W<REG> {
        self.variant(Crcsrcselect::Chn12)
    }
    #[doc = "DMA Channel 13"]
    #[inline(always)]
    pub fn chn13(self) -> &'a mut crate::W<REG> {
        self.variant(Crcsrcselect::Chn13)
    }
    #[doc = "DMA Channel 14"]
    #[inline(always)]
    pub fn chn14(self) -> &'a mut crate::W<REG> {
        self.variant(Crcsrcselect::Chn14)
    }
    #[doc = "DMA Channel 15"]
    #[inline(always)]
    pub fn chn15(self) -> &'a mut crate::W<REG> {
        self.variant(Crcsrcselect::Chn15)
    }
    #[doc = "DMA Channel 16"]
    #[inline(always)]
    pub fn chn16(self) -> &'a mut crate::W<REG> {
        self.variant(Crcsrcselect::Chn16)
    }
    #[doc = "DMA Channel 17"]
    #[inline(always)]
    pub fn chn17(self) -> &'a mut crate::W<REG> {
        self.variant(Crcsrcselect::Chn17)
    }
    #[doc = "DMA Channel 18"]
    #[inline(always)]
    pub fn chn18(self) -> &'a mut crate::W<REG> {
        self.variant(Crcsrcselect::Chn18)
    }
    #[doc = "DMA Channel 19"]
    #[inline(always)]
    pub fn chn19(self) -> &'a mut crate::W<REG> {
        self.variant(Crcsrcselect::Chn19)
    }
    #[doc = "DMA Channel 20"]
    #[inline(always)]
    pub fn chn20(self) -> &'a mut crate::W<REG> {
        self.variant(Crcsrcselect::Chn20)
    }
    #[doc = "DMA Channel 21"]
    #[inline(always)]
    pub fn chn21(self) -> &'a mut crate::W<REG> {
        self.variant(Crcsrcselect::Chn21)
    }
    #[doc = "DMA Channel 22"]
    #[inline(always)]
    pub fn chn22(self) -> &'a mut crate::W<REG> {
        self.variant(Crcsrcselect::Chn22)
    }
    #[doc = "DMA Channel 23"]
    #[inline(always)]
    pub fn chn23(self) -> &'a mut crate::W<REG> {
        self.variant(Crcsrcselect::Chn23)
    }
    #[doc = "DMA Channel 24"]
    #[inline(always)]
    pub fn chn24(self) -> &'a mut crate::W<REG> {
        self.variant(Crcsrcselect::Chn24)
    }
    #[doc = "DMA Channel 25"]
    #[inline(always)]
    pub fn chn25(self) -> &'a mut crate::W<REG> {
        self.variant(Crcsrcselect::Chn25)
    }
    #[doc = "DMA Channel 26"]
    #[inline(always)]
    pub fn chn26(self) -> &'a mut crate::W<REG> {
        self.variant(Crcsrcselect::Chn26)
    }
    #[doc = "DMA Channel 27"]
    #[inline(always)]
    pub fn chn27(self) -> &'a mut crate::W<REG> {
        self.variant(Crcsrcselect::Chn27)
    }
    #[doc = "DMA Channel 28"]
    #[inline(always)]
    pub fn chn28(self) -> &'a mut crate::W<REG> {
        self.variant(Crcsrcselect::Chn28)
    }
    #[doc = "DMA Channel 29"]
    #[inline(always)]
    pub fn chn29(self) -> &'a mut crate::W<REG> {
        self.variant(Crcsrcselect::Chn29)
    }
    #[doc = "DMA Channel 30"]
    #[inline(always)]
    pub fn chn30(self) -> &'a mut crate::W<REG> {
        self.variant(Crcsrcselect::Chn30)
    }
    #[doc = "DMA Channel 31"]
    #[inline(always)]
    pub fn chn31(self) -> &'a mut crate::W<REG> {
        self.variant(Crcsrcselect::Chn31)
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
