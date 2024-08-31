#[doc = "Register `TXCTRL` reader"]
pub type R = crate::R<TxctrlSpec>;
#[doc = "Register `TXCTRL` writer"]
pub type W = crate::W<TxctrlSpec>;
#[doc = "Serializer Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Sermodeselect {
    #[doc = "0: Receive"]
    Rx = 0,
    #[doc = "1: Transmit"]
    Tx = 1,
    #[doc = "2: Receive one PDM data on each serial clock edge"]
    Pdm2 = 2,
}
impl From<Sermodeselect> for u8 {
    #[inline(always)]
    fn from(variant: Sermodeselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Sermodeselect {
    type Ux = u8;
}
impl crate::IsEnum for Sermodeselect {}
#[doc = "Field `SERMODE` reader - Serializer Mode"]
pub type SermodeR = crate::FieldReader<Sermodeselect>;
impl SermodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Sermodeselect> {
        match self.bits {
            0 => Some(Sermodeselect::Rx),
            1 => Some(Sermodeselect::Tx),
            2 => Some(Sermodeselect::Pdm2),
            _ => None,
        }
    }
    #[doc = "Receive"]
    #[inline(always)]
    pub fn is_rx(&self) -> bool {
        *self == Sermodeselect::Rx
    }
    #[doc = "Transmit"]
    #[inline(always)]
    pub fn is_tx(&self) -> bool {
        *self == Sermodeselect::Tx
    }
    #[doc = "Receive one PDM data on each serial clock edge"]
    #[inline(always)]
    pub fn is_pdm2(&self) -> bool {
        *self == Sermodeselect::Pdm2
    }
}
#[doc = "Field `SERMODE` writer - Serializer Mode"]
pub type SermodeW<'a, REG> = crate::FieldWriter<'a, REG, 2, Sermodeselect>;
impl<'a, REG> SermodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Receive"]
    #[inline(always)]
    pub fn rx(self) -> &'a mut crate::W<REG> {
        self.variant(Sermodeselect::Rx)
    }
    #[doc = "Transmit"]
    #[inline(always)]
    pub fn tx(self) -> &'a mut crate::W<REG> {
        self.variant(Sermodeselect::Tx)
    }
    #[doc = "Receive one PDM data on each serial clock edge"]
    #[inline(always)]
    pub fn pdm2(self) -> &'a mut crate::W<REG> {
        self.variant(Sermodeselect::Pdm2)
    }
}
#[doc = "Line Default Line when Slot Disabled\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Txdefaultselect {
    #[doc = "0: Output Default Value is 0"]
    Zero = 0,
    #[doc = "1: Output Default Value is 1"]
    One = 1,
    #[doc = "3: Output Default Value is high impedance"]
    Hiz = 3,
}
impl From<Txdefaultselect> for u8 {
    #[inline(always)]
    fn from(variant: Txdefaultselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Txdefaultselect {
    type Ux = u8;
}
impl crate::IsEnum for Txdefaultselect {}
#[doc = "Field `TXDEFAULT` reader - Line Default Line when Slot Disabled"]
pub type TxdefaultR = crate::FieldReader<Txdefaultselect>;
impl TxdefaultR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Txdefaultselect> {
        match self.bits {
            0 => Some(Txdefaultselect::Zero),
            1 => Some(Txdefaultselect::One),
            3 => Some(Txdefaultselect::Hiz),
            _ => None,
        }
    }
    #[doc = "Output Default Value is 0"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == Txdefaultselect::Zero
    }
    #[doc = "Output Default Value is 1"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == Txdefaultselect::One
    }
    #[doc = "Output Default Value is high impedance"]
    #[inline(always)]
    pub fn is_hiz(&self) -> bool {
        *self == Txdefaultselect::Hiz
    }
}
#[doc = "Field `TXDEFAULT` writer - Line Default Line when Slot Disabled"]
pub type TxdefaultW<'a, REG> = crate::FieldWriter<'a, REG, 2, Txdefaultselect>;
impl<'a, REG> TxdefaultW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Output Default Value is 0"]
    #[inline(always)]
    pub fn zero(self) -> &'a mut crate::W<REG> {
        self.variant(Txdefaultselect::Zero)
    }
    #[doc = "Output Default Value is 1"]
    #[inline(always)]
    pub fn one(self) -> &'a mut crate::W<REG> {
        self.variant(Txdefaultselect::One)
    }
    #[doc = "Output Default Value is high impedance"]
    #[inline(always)]
    pub fn hiz(self) -> &'a mut crate::W<REG> {
        self.variant(Txdefaultselect::Hiz)
    }
}
#[doc = "Transmit Data when Underrun\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Txsameselect {
    #[doc = "0: Zero data transmitted in case of underrun"]
    Zero = 0,
    #[doc = "1: Last data transmitted in case of underrun"]
    Same = 1,
}
impl From<Txsameselect> for bool {
    #[inline(always)]
    fn from(variant: Txsameselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXSAME` reader - Transmit Data when Underrun"]
pub type TxsameR = crate::BitReader<Txsameselect>;
impl TxsameR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Txsameselect {
        match self.bits {
            false => Txsameselect::Zero,
            true => Txsameselect::Same,
        }
    }
    #[doc = "Zero data transmitted in case of underrun"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == Txsameselect::Zero
    }
    #[doc = "Last data transmitted in case of underrun"]
    #[inline(always)]
    pub fn is_same(&self) -> bool {
        *self == Txsameselect::Same
    }
}
#[doc = "Field `TXSAME` writer - Transmit Data when Underrun"]
pub type TxsameW<'a, REG> = crate::BitWriter<'a, REG, Txsameselect>;
impl<'a, REG> TxsameW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Zero data transmitted in case of underrun"]
    #[inline(always)]
    pub fn zero(self) -> &'a mut crate::W<REG> {
        self.variant(Txsameselect::Zero)
    }
    #[doc = "Last data transmitted in case of underrun"]
    #[inline(always)]
    pub fn same(self) -> &'a mut crate::W<REG> {
        self.variant(Txsameselect::Same)
    }
}
#[doc = "Clock Unit Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Clkselselect {
    #[doc = "0: Use Clock Unit 0"]
    Clk0 = 0,
    #[doc = "1: Use Clock Unit 1"]
    Clk1 = 1,
}
impl From<Clkselselect> for bool {
    #[inline(always)]
    fn from(variant: Clkselselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLKSEL` reader - Clock Unit Selection"]
pub type ClkselR = crate::BitReader<Clkselselect>;
impl ClkselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Clkselselect {
        match self.bits {
            false => Clkselselect::Clk0,
            true => Clkselselect::Clk1,
        }
    }
    #[doc = "Use Clock Unit 0"]
    #[inline(always)]
    pub fn is_clk0(&self) -> bool {
        *self == Clkselselect::Clk0
    }
    #[doc = "Use Clock Unit 1"]
    #[inline(always)]
    pub fn is_clk1(&self) -> bool {
        *self == Clkselselect::Clk1
    }
}
#[doc = "Field `CLKSEL` writer - Clock Unit Selection"]
pub type ClkselW<'a, REG> = crate::BitWriter<'a, REG, Clkselselect>;
impl<'a, REG> ClkselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Use Clock Unit 0"]
    #[inline(always)]
    pub fn clk0(self) -> &'a mut crate::W<REG> {
        self.variant(Clkselselect::Clk0)
    }
    #[doc = "Use Clock Unit 1"]
    #[inline(always)]
    pub fn clk1(self) -> &'a mut crate::W<REG> {
        self.variant(Clkselselect::Clk1)
    }
}
#[doc = "Data Slot Formatting Adjust\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Slotadjselect {
    #[doc = "0: Data is right adjusted in slot"]
    Right = 0,
    #[doc = "1: Data is left adjusted in slot"]
    Left = 1,
}
impl From<Slotadjselect> for bool {
    #[inline(always)]
    fn from(variant: Slotadjselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SLOTADJ` reader - Data Slot Formatting Adjust"]
pub type SlotadjR = crate::BitReader<Slotadjselect>;
impl SlotadjR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Slotadjselect {
        match self.bits {
            false => Slotadjselect::Right,
            true => Slotadjselect::Left,
        }
    }
    #[doc = "Data is right adjusted in slot"]
    #[inline(always)]
    pub fn is_right(&self) -> bool {
        *self == Slotadjselect::Right
    }
    #[doc = "Data is left adjusted in slot"]
    #[inline(always)]
    pub fn is_left(&self) -> bool {
        *self == Slotadjselect::Left
    }
}
#[doc = "Field `SLOTADJ` writer - Data Slot Formatting Adjust"]
pub type SlotadjW<'a, REG> = crate::BitWriter<'a, REG, Slotadjselect>;
impl<'a, REG> SlotadjW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Data is right adjusted in slot"]
    #[inline(always)]
    pub fn right(self) -> &'a mut crate::W<REG> {
        self.variant(Slotadjselect::Right)
    }
    #[doc = "Data is left adjusted in slot"]
    #[inline(always)]
    pub fn left(self) -> &'a mut crate::W<REG> {
        self.variant(Slotadjselect::Left)
    }
}
#[doc = "Data Word Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Datasizeselect {
    #[doc = "0: 32 bits"]
    _32 = 0,
    #[doc = "1: 24 bits"]
    _24 = 1,
    #[doc = "2: 20 bits"]
    _20 = 2,
    #[doc = "3: 18 bits"]
    _18 = 3,
    #[doc = "4: 16 bits"]
    _16 = 4,
    #[doc = "5: 16 bits compact stereo"]
    _16c = 5,
    #[doc = "6: 8 bits"]
    _8 = 6,
    #[doc = "7: 8 bits compact stereo"]
    _8c = 7,
}
impl From<Datasizeselect> for u8 {
    #[inline(always)]
    fn from(variant: Datasizeselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Datasizeselect {
    type Ux = u8;
}
impl crate::IsEnum for Datasizeselect {}
#[doc = "Field `DATASIZE` reader - Data Word Size"]
pub type DatasizeR = crate::FieldReader<Datasizeselect>;
impl DatasizeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Datasizeselect {
        match self.bits {
            0 => Datasizeselect::_32,
            1 => Datasizeselect::_24,
            2 => Datasizeselect::_20,
            3 => Datasizeselect::_18,
            4 => Datasizeselect::_16,
            5 => Datasizeselect::_16c,
            6 => Datasizeselect::_8,
            7 => Datasizeselect::_8c,
            _ => unreachable!(),
        }
    }
    #[doc = "32 bits"]
    #[inline(always)]
    pub fn is_32(&self) -> bool {
        *self == Datasizeselect::_32
    }
    #[doc = "24 bits"]
    #[inline(always)]
    pub fn is_24(&self) -> bool {
        *self == Datasizeselect::_24
    }
    #[doc = "20 bits"]
    #[inline(always)]
    pub fn is_20(&self) -> bool {
        *self == Datasizeselect::_20
    }
    #[doc = "18 bits"]
    #[inline(always)]
    pub fn is_18(&self) -> bool {
        *self == Datasizeselect::_18
    }
    #[doc = "16 bits"]
    #[inline(always)]
    pub fn is_16(&self) -> bool {
        *self == Datasizeselect::_16
    }
    #[doc = "16 bits compact stereo"]
    #[inline(always)]
    pub fn is_16c(&self) -> bool {
        *self == Datasizeselect::_16c
    }
    #[doc = "8 bits"]
    #[inline(always)]
    pub fn is_8(&self) -> bool {
        *self == Datasizeselect::_8
    }
    #[doc = "8 bits compact stereo"]
    #[inline(always)]
    pub fn is_8c(&self) -> bool {
        *self == Datasizeselect::_8c
    }
}
#[doc = "Field `DATASIZE` writer - Data Word Size"]
pub type DatasizeW<'a, REG> = crate::FieldWriter<'a, REG, 3, Datasizeselect, crate::Safe>;
impl<'a, REG> DatasizeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "32 bits"]
    #[inline(always)]
    pub fn _32(self) -> &'a mut crate::W<REG> {
        self.variant(Datasizeselect::_32)
    }
    #[doc = "24 bits"]
    #[inline(always)]
    pub fn _24(self) -> &'a mut crate::W<REG> {
        self.variant(Datasizeselect::_24)
    }
    #[doc = "20 bits"]
    #[inline(always)]
    pub fn _20(self) -> &'a mut crate::W<REG> {
        self.variant(Datasizeselect::_20)
    }
    #[doc = "18 bits"]
    #[inline(always)]
    pub fn _18(self) -> &'a mut crate::W<REG> {
        self.variant(Datasizeselect::_18)
    }
    #[doc = "16 bits"]
    #[inline(always)]
    pub fn _16(self) -> &'a mut crate::W<REG> {
        self.variant(Datasizeselect::_16)
    }
    #[doc = "16 bits compact stereo"]
    #[inline(always)]
    pub fn _16c(self) -> &'a mut crate::W<REG> {
        self.variant(Datasizeselect::_16c)
    }
    #[doc = "8 bits"]
    #[inline(always)]
    pub fn _8(self) -> &'a mut crate::W<REG> {
        self.variant(Datasizeselect::_8)
    }
    #[doc = "8 bits compact stereo"]
    #[inline(always)]
    pub fn _8c(self) -> &'a mut crate::W<REG> {
        self.variant(Datasizeselect::_8c)
    }
}
#[doc = "Data Word Formatting Adjust\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wordadjselect {
    #[doc = "0: Data is right adjusted in word"]
    Right = 0,
    #[doc = "1: Data is left adjusted in word"]
    Left = 1,
}
impl From<Wordadjselect> for bool {
    #[inline(always)]
    fn from(variant: Wordadjselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WORDADJ` reader - Data Word Formatting Adjust"]
pub type WordadjR = crate::BitReader<Wordadjselect>;
impl WordadjR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wordadjselect {
        match self.bits {
            false => Wordadjselect::Right,
            true => Wordadjselect::Left,
        }
    }
    #[doc = "Data is right adjusted in word"]
    #[inline(always)]
    pub fn is_right(&self) -> bool {
        *self == Wordadjselect::Right
    }
    #[doc = "Data is left adjusted in word"]
    #[inline(always)]
    pub fn is_left(&self) -> bool {
        *self == Wordadjselect::Left
    }
}
#[doc = "Field `WORDADJ` writer - Data Word Formatting Adjust"]
pub type WordadjW<'a, REG> = crate::BitWriter<'a, REG, Wordadjselect>;
impl<'a, REG> WordadjW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Data is right adjusted in word"]
    #[inline(always)]
    pub fn right(self) -> &'a mut crate::W<REG> {
        self.variant(Wordadjselect::Right)
    }
    #[doc = "Data is left adjusted in word"]
    #[inline(always)]
    pub fn left(self) -> &'a mut crate::W<REG> {
        self.variant(Wordadjselect::Left)
    }
}
#[doc = "Data Formatting Bit Extension\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Extendselect {
    #[doc = "0: Extend with zeroes"]
    Zero = 0,
    #[doc = "1: Extend with ones"]
    One = 1,
    #[doc = "2: Extend with Most Significant Bit"]
    Msbit = 2,
    #[doc = "3: Extend with Least Significant Bit"]
    Lsbit = 3,
}
impl From<Extendselect> for u8 {
    #[inline(always)]
    fn from(variant: Extendselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Extendselect {
    type Ux = u8;
}
impl crate::IsEnum for Extendselect {}
#[doc = "Field `EXTEND` reader - Data Formatting Bit Extension"]
pub type ExtendR = crate::FieldReader<Extendselect>;
impl ExtendR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Extendselect {
        match self.bits {
            0 => Extendselect::Zero,
            1 => Extendselect::One,
            2 => Extendselect::Msbit,
            3 => Extendselect::Lsbit,
            _ => unreachable!(),
        }
    }
    #[doc = "Extend with zeroes"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == Extendselect::Zero
    }
    #[doc = "Extend with ones"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == Extendselect::One
    }
    #[doc = "Extend with Most Significant Bit"]
    #[inline(always)]
    pub fn is_msbit(&self) -> bool {
        *self == Extendselect::Msbit
    }
    #[doc = "Extend with Least Significant Bit"]
    #[inline(always)]
    pub fn is_lsbit(&self) -> bool {
        *self == Extendselect::Lsbit
    }
}
#[doc = "Field `EXTEND` writer - Data Formatting Bit Extension"]
pub type ExtendW<'a, REG> = crate::FieldWriter<'a, REG, 2, Extendselect, crate::Safe>;
impl<'a, REG> ExtendW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Extend with zeroes"]
    #[inline(always)]
    pub fn zero(self) -> &'a mut crate::W<REG> {
        self.variant(Extendselect::Zero)
    }
    #[doc = "Extend with ones"]
    #[inline(always)]
    pub fn one(self) -> &'a mut crate::W<REG> {
        self.variant(Extendselect::One)
    }
    #[doc = "Extend with Most Significant Bit"]
    #[inline(always)]
    pub fn msbit(self) -> &'a mut crate::W<REG> {
        self.variant(Extendselect::Msbit)
    }
    #[doc = "Extend with Least Significant Bit"]
    #[inline(always)]
    pub fn lsbit(self) -> &'a mut crate::W<REG> {
        self.variant(Extendselect::Lsbit)
    }
}
#[doc = "Data Formatting Bit Reverse\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bitrevselect {
    #[doc = "0: Transfer Data Most Significant Bit (MSB) first (default for I2S protocol)"]
    Msbit = 0,
    #[doc = "1: Transfer Data Least Significant Bit (LSB) first"]
    Lsbit = 1,
}
impl From<Bitrevselect> for bool {
    #[inline(always)]
    fn from(variant: Bitrevselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BITREV` reader - Data Formatting Bit Reverse"]
pub type BitrevR = crate::BitReader<Bitrevselect>;
impl BitrevR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bitrevselect {
        match self.bits {
            false => Bitrevselect::Msbit,
            true => Bitrevselect::Lsbit,
        }
    }
    #[doc = "Transfer Data Most Significant Bit (MSB) first (default for I2S protocol)"]
    #[inline(always)]
    pub fn is_msbit(&self) -> bool {
        *self == Bitrevselect::Msbit
    }
    #[doc = "Transfer Data Least Significant Bit (LSB) first"]
    #[inline(always)]
    pub fn is_lsbit(&self) -> bool {
        *self == Bitrevselect::Lsbit
    }
}
#[doc = "Field `BITREV` writer - Data Formatting Bit Reverse"]
pub type BitrevW<'a, REG> = crate::BitWriter<'a, REG, Bitrevselect>;
impl<'a, REG> BitrevW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Transfer Data Most Significant Bit (MSB) first (default for I2S protocol)"]
    #[inline(always)]
    pub fn msbit(self) -> &'a mut crate::W<REG> {
        self.variant(Bitrevselect::Msbit)
    }
    #[doc = "Transfer Data Least Significant Bit (LSB) first"]
    #[inline(always)]
    pub fn lsbit(self) -> &'a mut crate::W<REG> {
        self.variant(Bitrevselect::Lsbit)
    }
}
#[doc = "Field `SLOTDIS0` reader - Slot 0 Disabled for this Serializer"]
pub type Slotdis0R = crate::BitReader;
#[doc = "Field `SLOTDIS0` writer - Slot 0 Disabled for this Serializer"]
pub type Slotdis0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLOTDIS1` reader - Slot 1 Disabled for this Serializer"]
pub type Slotdis1R = crate::BitReader;
#[doc = "Field `SLOTDIS1` writer - Slot 1 Disabled for this Serializer"]
pub type Slotdis1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLOTDIS2` reader - Slot 2 Disabled for this Serializer"]
pub type Slotdis2R = crate::BitReader;
#[doc = "Field `SLOTDIS2` writer - Slot 2 Disabled for this Serializer"]
pub type Slotdis2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLOTDIS3` reader - Slot 3 Disabled for this Serializer"]
pub type Slotdis3R = crate::BitReader;
#[doc = "Field `SLOTDIS3` writer - Slot 3 Disabled for this Serializer"]
pub type Slotdis3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLOTDIS4` reader - Slot 4 Disabled for this Serializer"]
pub type Slotdis4R = crate::BitReader;
#[doc = "Field `SLOTDIS4` writer - Slot 4 Disabled for this Serializer"]
pub type Slotdis4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLOTDIS5` reader - Slot 5 Disabled for this Serializer"]
pub type Slotdis5R = crate::BitReader;
#[doc = "Field `SLOTDIS5` writer - Slot 5 Disabled for this Serializer"]
pub type Slotdis5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLOTDIS6` reader - Slot 6 Disabled for this Serializer"]
pub type Slotdis6R = crate::BitReader;
#[doc = "Field `SLOTDIS6` writer - Slot 6 Disabled for this Serializer"]
pub type Slotdis6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLOTDIS7` reader - Slot 7 Disabled for this Serializer"]
pub type Slotdis7R = crate::BitReader;
#[doc = "Field `SLOTDIS7` writer - Slot 7 Disabled for this Serializer"]
pub type Slotdis7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Mono Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Monoselect {
    #[doc = "0: Normal mode"]
    Stereo = 0,
    #[doc = "1: Left channel data is duplicated to right channel"]
    Mono = 1,
}
impl From<Monoselect> for bool {
    #[inline(always)]
    fn from(variant: Monoselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MONO` reader - Mono Mode"]
pub type MonoR = crate::BitReader<Monoselect>;
impl MonoR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Monoselect {
        match self.bits {
            false => Monoselect::Stereo,
            true => Monoselect::Mono,
        }
    }
    #[doc = "Normal mode"]
    #[inline(always)]
    pub fn is_stereo(&self) -> bool {
        *self == Monoselect::Stereo
    }
    #[doc = "Left channel data is duplicated to right channel"]
    #[inline(always)]
    pub fn is_mono(&self) -> bool {
        *self == Monoselect::Mono
    }
}
#[doc = "Field `MONO` writer - Mono Mode"]
pub type MonoW<'a, REG> = crate::BitWriter<'a, REG, Monoselect>;
impl<'a, REG> MonoW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal mode"]
    #[inline(always)]
    pub fn stereo(self) -> &'a mut crate::W<REG> {
        self.variant(Monoselect::Stereo)
    }
    #[doc = "Left channel data is duplicated to right channel"]
    #[inline(always)]
    pub fn mono(self) -> &'a mut crate::W<REG> {
        self.variant(Monoselect::Mono)
    }
}
#[doc = "Single or Multiple DMA Channels\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmaselect {
    #[doc = "0: Single DMA channel"]
    Single = 0,
    #[doc = "1: One DMA channel per data channel"]
    Multiple = 1,
}
impl From<Dmaselect> for bool {
    #[inline(always)]
    fn from(variant: Dmaselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMA` reader - Single or Multiple DMA Channels"]
pub type DmaR = crate::BitReader<Dmaselect>;
impl DmaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmaselect {
        match self.bits {
            false => Dmaselect::Single,
            true => Dmaselect::Multiple,
        }
    }
    #[doc = "Single DMA channel"]
    #[inline(always)]
    pub fn is_single(&self) -> bool {
        *self == Dmaselect::Single
    }
    #[doc = "One DMA channel per data channel"]
    #[inline(always)]
    pub fn is_multiple(&self) -> bool {
        *self == Dmaselect::Multiple
    }
}
#[doc = "Field `DMA` writer - Single or Multiple DMA Channels"]
pub type DmaW<'a, REG> = crate::BitWriter<'a, REG, Dmaselect>;
impl<'a, REG> DmaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Single DMA channel"]
    #[inline(always)]
    pub fn single(self) -> &'a mut crate::W<REG> {
        self.variant(Dmaselect::Single)
    }
    #[doc = "One DMA channel per data channel"]
    #[inline(always)]
    pub fn multiple(self) -> &'a mut crate::W<REG> {
        self.variant(Dmaselect::Multiple)
    }
}
impl R {
    #[doc = "Bits 0:1 - Serializer Mode"]
    #[inline(always)]
    pub fn sermode(&self) -> SermodeR {
        SermodeR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Line Default Line when Slot Disabled"]
    #[inline(always)]
    pub fn txdefault(&self) -> TxdefaultR {
        TxdefaultR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - Transmit Data when Underrun"]
    #[inline(always)]
    pub fn txsame(&self) -> TxsameR {
        TxsameR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Clock Unit Selection"]
    #[inline(always)]
    pub fn clksel(&self) -> ClkselR {
        ClkselR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - Data Slot Formatting Adjust"]
    #[inline(always)]
    pub fn slotadj(&self) -> SlotadjR {
        SlotadjR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - Data Word Size"]
    #[inline(always)]
    pub fn datasize(&self) -> DatasizeR {
        DatasizeR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 12 - Data Word Formatting Adjust"]
    #[inline(always)]
    pub fn wordadj(&self) -> WordadjR {
        WordadjR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:14 - Data Formatting Bit Extension"]
    #[inline(always)]
    pub fn extend(&self) -> ExtendR {
        ExtendR::new(((self.bits >> 13) & 3) as u8)
    }
    #[doc = "Bit 15 - Data Formatting Bit Reverse"]
    #[inline(always)]
    pub fn bitrev(&self) -> BitrevR {
        BitrevR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Slot 0 Disabled for this Serializer"]
    #[inline(always)]
    pub fn slotdis0(&self) -> Slotdis0R {
        Slotdis0R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Slot 1 Disabled for this Serializer"]
    #[inline(always)]
    pub fn slotdis1(&self) -> Slotdis1R {
        Slotdis1R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Slot 2 Disabled for this Serializer"]
    #[inline(always)]
    pub fn slotdis2(&self) -> Slotdis2R {
        Slotdis2R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Slot 3 Disabled for this Serializer"]
    #[inline(always)]
    pub fn slotdis3(&self) -> Slotdis3R {
        Slotdis3R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Slot 4 Disabled for this Serializer"]
    #[inline(always)]
    pub fn slotdis4(&self) -> Slotdis4R {
        Slotdis4R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Slot 5 Disabled for this Serializer"]
    #[inline(always)]
    pub fn slotdis5(&self) -> Slotdis5R {
        Slotdis5R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Slot 6 Disabled for this Serializer"]
    #[inline(always)]
    pub fn slotdis6(&self) -> Slotdis6R {
        Slotdis6R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Slot 7 Disabled for this Serializer"]
    #[inline(always)]
    pub fn slotdis7(&self) -> Slotdis7R {
        Slotdis7R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Mono Mode"]
    #[inline(always)]
    pub fn mono(&self) -> MonoR {
        MonoR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Single or Multiple DMA Channels"]
    #[inline(always)]
    pub fn dma(&self) -> DmaR {
        DmaR::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Serializer Mode"]
    #[inline(always)]
    #[must_use]
    pub fn sermode(&mut self) -> SermodeW<TxctrlSpec> {
        SermodeW::new(self, 0)
    }
    #[doc = "Bits 2:3 - Line Default Line when Slot Disabled"]
    #[inline(always)]
    #[must_use]
    pub fn txdefault(&mut self) -> TxdefaultW<TxctrlSpec> {
        TxdefaultW::new(self, 2)
    }
    #[doc = "Bit 4 - Transmit Data when Underrun"]
    #[inline(always)]
    #[must_use]
    pub fn txsame(&mut self) -> TxsameW<TxctrlSpec> {
        TxsameW::new(self, 4)
    }
    #[doc = "Bit 5 - Clock Unit Selection"]
    #[inline(always)]
    #[must_use]
    pub fn clksel(&mut self) -> ClkselW<TxctrlSpec> {
        ClkselW::new(self, 5)
    }
    #[doc = "Bit 7 - Data Slot Formatting Adjust"]
    #[inline(always)]
    #[must_use]
    pub fn slotadj(&mut self) -> SlotadjW<TxctrlSpec> {
        SlotadjW::new(self, 7)
    }
    #[doc = "Bits 8:10 - Data Word Size"]
    #[inline(always)]
    #[must_use]
    pub fn datasize(&mut self) -> DatasizeW<TxctrlSpec> {
        DatasizeW::new(self, 8)
    }
    #[doc = "Bit 12 - Data Word Formatting Adjust"]
    #[inline(always)]
    #[must_use]
    pub fn wordadj(&mut self) -> WordadjW<TxctrlSpec> {
        WordadjW::new(self, 12)
    }
    #[doc = "Bits 13:14 - Data Formatting Bit Extension"]
    #[inline(always)]
    #[must_use]
    pub fn extend(&mut self) -> ExtendW<TxctrlSpec> {
        ExtendW::new(self, 13)
    }
    #[doc = "Bit 15 - Data Formatting Bit Reverse"]
    #[inline(always)]
    #[must_use]
    pub fn bitrev(&mut self) -> BitrevW<TxctrlSpec> {
        BitrevW::new(self, 15)
    }
    #[doc = "Bit 16 - Slot 0 Disabled for this Serializer"]
    #[inline(always)]
    #[must_use]
    pub fn slotdis0(&mut self) -> Slotdis0W<TxctrlSpec> {
        Slotdis0W::new(self, 16)
    }
    #[doc = "Bit 17 - Slot 1 Disabled for this Serializer"]
    #[inline(always)]
    #[must_use]
    pub fn slotdis1(&mut self) -> Slotdis1W<TxctrlSpec> {
        Slotdis1W::new(self, 17)
    }
    #[doc = "Bit 18 - Slot 2 Disabled for this Serializer"]
    #[inline(always)]
    #[must_use]
    pub fn slotdis2(&mut self) -> Slotdis2W<TxctrlSpec> {
        Slotdis2W::new(self, 18)
    }
    #[doc = "Bit 19 - Slot 3 Disabled for this Serializer"]
    #[inline(always)]
    #[must_use]
    pub fn slotdis3(&mut self) -> Slotdis3W<TxctrlSpec> {
        Slotdis3W::new(self, 19)
    }
    #[doc = "Bit 20 - Slot 4 Disabled for this Serializer"]
    #[inline(always)]
    #[must_use]
    pub fn slotdis4(&mut self) -> Slotdis4W<TxctrlSpec> {
        Slotdis4W::new(self, 20)
    }
    #[doc = "Bit 21 - Slot 5 Disabled for this Serializer"]
    #[inline(always)]
    #[must_use]
    pub fn slotdis5(&mut self) -> Slotdis5W<TxctrlSpec> {
        Slotdis5W::new(self, 21)
    }
    #[doc = "Bit 22 - Slot 6 Disabled for this Serializer"]
    #[inline(always)]
    #[must_use]
    pub fn slotdis6(&mut self) -> Slotdis6W<TxctrlSpec> {
        Slotdis6W::new(self, 22)
    }
    #[doc = "Bit 23 - Slot 7 Disabled for this Serializer"]
    #[inline(always)]
    #[must_use]
    pub fn slotdis7(&mut self) -> Slotdis7W<TxctrlSpec> {
        Slotdis7W::new(self, 23)
    }
    #[doc = "Bit 24 - Mono Mode"]
    #[inline(always)]
    #[must_use]
    pub fn mono(&mut self) -> MonoW<TxctrlSpec> {
        MonoW::new(self, 24)
    }
    #[doc = "Bit 25 - Single or Multiple DMA Channels"]
    #[inline(always)]
    #[must_use]
    pub fn dma(&mut self) -> DmaW<TxctrlSpec> {
        DmaW::new(self, 25)
    }
}
#[doc = "Tx Serializer Control\n\nYou can [`read`](crate::Reg::read) this register and get [`txctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxctrlSpec;
impl crate::RegisterSpec for TxctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txctrl::R`](R) reader structure"]
impl crate::Readable for TxctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`txctrl::W`](W) writer structure"]
impl crate::Writable for TxctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TXCTRL to value 0"]
impl crate::Resettable for TxctrlSpec {
    const RESET_VALUE: u32 = 0;
}
