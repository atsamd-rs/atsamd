#[doc = "Register `RXCTRL` reader"]
pub type R = crate::R<RXCTRL_SPEC>;
#[doc = "Register `RXCTRL` writer"]
pub type W = crate::W<RXCTRL_SPEC>;
#[doc = "Field `SERMODE` reader - Serializer Mode"]
pub type SERMODE_R = crate::FieldReader<SERMODESELECT_A>;
#[doc = "Serializer Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SERMODESELECT_A {
    #[doc = "0: Receive"]
    RX = 0,
    #[doc = "2: Receive one PDM data on each serial clock edge"]
    PDM2 = 2,
}
impl From<SERMODESELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: SERMODESELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SERMODESELECT_A {
    type Ux = u8;
}
impl SERMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SERMODESELECT_A> {
        match self.bits {
            0 => Some(SERMODESELECT_A::RX),
            2 => Some(SERMODESELECT_A::PDM2),
            _ => None,
        }
    }
    #[doc = "Receive"]
    #[inline(always)]
    pub fn is_rx(&self) -> bool {
        *self == SERMODESELECT_A::RX
    }
    #[doc = "Receive one PDM data on each serial clock edge"]
    #[inline(always)]
    pub fn is_pdm2(&self) -> bool {
        *self == SERMODESELECT_A::PDM2
    }
}
#[doc = "Field `SERMODE` writer - Serializer Mode"]
pub type SERMODE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O, SERMODESELECT_A>;
impl<'a, REG, const O: u8> SERMODE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Receive"]
    #[inline(always)]
    pub fn rx(self) -> &'a mut crate::W<REG> {
        self.variant(SERMODESELECT_A::RX)
    }
    #[doc = "Receive one PDM data on each serial clock edge"]
    #[inline(always)]
    pub fn pdm2(self) -> &'a mut crate::W<REG> {
        self.variant(SERMODESELECT_A::PDM2)
    }
}
#[doc = "Field `CLKSEL` reader - Clock Unit Selection"]
pub type CLKSEL_R = crate::BitReader<CLKSELSELECT_A>;
#[doc = "Clock Unit Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLKSELSELECT_A {
    #[doc = "0: Use Clock Unit 0"]
    CLK0 = 0,
    #[doc = "1: Use Clock Unit 1"]
    CLK1 = 1,
}
impl From<CLKSELSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: CLKSELSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl CLKSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CLKSELSELECT_A {
        match self.bits {
            false => CLKSELSELECT_A::CLK0,
            true => CLKSELSELECT_A::CLK1,
        }
    }
    #[doc = "Use Clock Unit 0"]
    #[inline(always)]
    pub fn is_clk0(&self) -> bool {
        *self == CLKSELSELECT_A::CLK0
    }
    #[doc = "Use Clock Unit 1"]
    #[inline(always)]
    pub fn is_clk1(&self) -> bool {
        *self == CLKSELSELECT_A::CLK1
    }
}
#[doc = "Field `CLKSEL` writer - Clock Unit Selection"]
pub type CLKSEL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CLKSELSELECT_A>;
impl<'a, REG, const O: u8> CLKSEL_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Use Clock Unit 0"]
    #[inline(always)]
    pub fn clk0(self) -> &'a mut crate::W<REG> {
        self.variant(CLKSELSELECT_A::CLK0)
    }
    #[doc = "Use Clock Unit 1"]
    #[inline(always)]
    pub fn clk1(self) -> &'a mut crate::W<REG> {
        self.variant(CLKSELSELECT_A::CLK1)
    }
}
#[doc = "Field `SLOTADJ` reader - Data Slot Formatting Adjust"]
pub type SLOTADJ_R = crate::BitReader<SLOTADJSELECT_A>;
#[doc = "Data Slot Formatting Adjust\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SLOTADJSELECT_A {
    #[doc = "0: Data is right adjusted in slot"]
    RIGHT = 0,
    #[doc = "1: Data is left adjusted in slot"]
    LEFT = 1,
}
impl From<SLOTADJSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: SLOTADJSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl SLOTADJ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SLOTADJSELECT_A {
        match self.bits {
            false => SLOTADJSELECT_A::RIGHT,
            true => SLOTADJSELECT_A::LEFT,
        }
    }
    #[doc = "Data is right adjusted in slot"]
    #[inline(always)]
    pub fn is_right(&self) -> bool {
        *self == SLOTADJSELECT_A::RIGHT
    }
    #[doc = "Data is left adjusted in slot"]
    #[inline(always)]
    pub fn is_left(&self) -> bool {
        *self == SLOTADJSELECT_A::LEFT
    }
}
#[doc = "Field `SLOTADJ` writer - Data Slot Formatting Adjust"]
pub type SLOTADJ_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, SLOTADJSELECT_A>;
impl<'a, REG, const O: u8> SLOTADJ_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Data is right adjusted in slot"]
    #[inline(always)]
    pub fn right(self) -> &'a mut crate::W<REG> {
        self.variant(SLOTADJSELECT_A::RIGHT)
    }
    #[doc = "Data is left adjusted in slot"]
    #[inline(always)]
    pub fn left(self) -> &'a mut crate::W<REG> {
        self.variant(SLOTADJSELECT_A::LEFT)
    }
}
#[doc = "Field `DATASIZE` reader - Data Word Size"]
pub type DATASIZE_R = crate::FieldReader<DATASIZESELECT_A>;
#[doc = "Data Word Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DATASIZESELECT_A {
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
    _16C = 5,
    #[doc = "6: 8 bits"]
    _8 = 6,
    #[doc = "7: 8 bits compact stereo"]
    _8C = 7,
}
impl From<DATASIZESELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: DATASIZESELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DATASIZESELECT_A {
    type Ux = u8;
}
impl DATASIZE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DATASIZESELECT_A {
        match self.bits {
            0 => DATASIZESELECT_A::_32,
            1 => DATASIZESELECT_A::_24,
            2 => DATASIZESELECT_A::_20,
            3 => DATASIZESELECT_A::_18,
            4 => DATASIZESELECT_A::_16,
            5 => DATASIZESELECT_A::_16C,
            6 => DATASIZESELECT_A::_8,
            7 => DATASIZESELECT_A::_8C,
            _ => unreachable!(),
        }
    }
    #[doc = "32 bits"]
    #[inline(always)]
    pub fn is_32(&self) -> bool {
        *self == DATASIZESELECT_A::_32
    }
    #[doc = "24 bits"]
    #[inline(always)]
    pub fn is_24(&self) -> bool {
        *self == DATASIZESELECT_A::_24
    }
    #[doc = "20 bits"]
    #[inline(always)]
    pub fn is_20(&self) -> bool {
        *self == DATASIZESELECT_A::_20
    }
    #[doc = "18 bits"]
    #[inline(always)]
    pub fn is_18(&self) -> bool {
        *self == DATASIZESELECT_A::_18
    }
    #[doc = "16 bits"]
    #[inline(always)]
    pub fn is_16(&self) -> bool {
        *self == DATASIZESELECT_A::_16
    }
    #[doc = "16 bits compact stereo"]
    #[inline(always)]
    pub fn is_16c(&self) -> bool {
        *self == DATASIZESELECT_A::_16C
    }
    #[doc = "8 bits"]
    #[inline(always)]
    pub fn is_8(&self) -> bool {
        *self == DATASIZESELECT_A::_8
    }
    #[doc = "8 bits compact stereo"]
    #[inline(always)]
    pub fn is_8c(&self) -> bool {
        *self == DATASIZESELECT_A::_8C
    }
}
#[doc = "Field `DATASIZE` writer - Data Word Size"]
pub type DATASIZE_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 3, O, DATASIZESELECT_A>;
impl<'a, REG, const O: u8> DATASIZE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "32 bits"]
    #[inline(always)]
    pub fn _32(self) -> &'a mut crate::W<REG> {
        self.variant(DATASIZESELECT_A::_32)
    }
    #[doc = "24 bits"]
    #[inline(always)]
    pub fn _24(self) -> &'a mut crate::W<REG> {
        self.variant(DATASIZESELECT_A::_24)
    }
    #[doc = "20 bits"]
    #[inline(always)]
    pub fn _20(self) -> &'a mut crate::W<REG> {
        self.variant(DATASIZESELECT_A::_20)
    }
    #[doc = "18 bits"]
    #[inline(always)]
    pub fn _18(self) -> &'a mut crate::W<REG> {
        self.variant(DATASIZESELECT_A::_18)
    }
    #[doc = "16 bits"]
    #[inline(always)]
    pub fn _16(self) -> &'a mut crate::W<REG> {
        self.variant(DATASIZESELECT_A::_16)
    }
    #[doc = "16 bits compact stereo"]
    #[inline(always)]
    pub fn _16c(self) -> &'a mut crate::W<REG> {
        self.variant(DATASIZESELECT_A::_16C)
    }
    #[doc = "8 bits"]
    #[inline(always)]
    pub fn _8(self) -> &'a mut crate::W<REG> {
        self.variant(DATASIZESELECT_A::_8)
    }
    #[doc = "8 bits compact stereo"]
    #[inline(always)]
    pub fn _8c(self) -> &'a mut crate::W<REG> {
        self.variant(DATASIZESELECT_A::_8C)
    }
}
#[doc = "Field `WORDADJ` reader - Data Word Formatting Adjust"]
pub type WORDADJ_R = crate::BitReader<WORDADJSELECT_A>;
#[doc = "Data Word Formatting Adjust\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WORDADJSELECT_A {
    #[doc = "0: Data is right adjusted in word"]
    RIGHT = 0,
    #[doc = "1: Data is left adjusted in word"]
    LEFT = 1,
}
impl From<WORDADJSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: WORDADJSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl WORDADJ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WORDADJSELECT_A {
        match self.bits {
            false => WORDADJSELECT_A::RIGHT,
            true => WORDADJSELECT_A::LEFT,
        }
    }
    #[doc = "Data is right adjusted in word"]
    #[inline(always)]
    pub fn is_right(&self) -> bool {
        *self == WORDADJSELECT_A::RIGHT
    }
    #[doc = "Data is left adjusted in word"]
    #[inline(always)]
    pub fn is_left(&self) -> bool {
        *self == WORDADJSELECT_A::LEFT
    }
}
#[doc = "Field `WORDADJ` writer - Data Word Formatting Adjust"]
pub type WORDADJ_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, WORDADJSELECT_A>;
impl<'a, REG, const O: u8> WORDADJ_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Data is right adjusted in word"]
    #[inline(always)]
    pub fn right(self) -> &'a mut crate::W<REG> {
        self.variant(WORDADJSELECT_A::RIGHT)
    }
    #[doc = "Data is left adjusted in word"]
    #[inline(always)]
    pub fn left(self) -> &'a mut crate::W<REG> {
        self.variant(WORDADJSELECT_A::LEFT)
    }
}
#[doc = "Field `EXTEND` reader - Data Formatting Bit Extension"]
pub type EXTEND_R = crate::FieldReader<EXTENDSELECT_A>;
#[doc = "Data Formatting Bit Extension\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTENDSELECT_A {
    #[doc = "0: Extend with zeroes"]
    ZERO = 0,
    #[doc = "1: Extend with ones"]
    ONE = 1,
    #[doc = "2: Extend with Most Significant Bit"]
    MSBIT = 2,
    #[doc = "3: Extend with Least Significant Bit"]
    LSBIT = 3,
}
impl From<EXTENDSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTENDSELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EXTENDSELECT_A {
    type Ux = u8;
}
impl EXTEND_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EXTENDSELECT_A {
        match self.bits {
            0 => EXTENDSELECT_A::ZERO,
            1 => EXTENDSELECT_A::ONE,
            2 => EXTENDSELECT_A::MSBIT,
            3 => EXTENDSELECT_A::LSBIT,
            _ => unreachable!(),
        }
    }
    #[doc = "Extend with zeroes"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == EXTENDSELECT_A::ZERO
    }
    #[doc = "Extend with ones"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == EXTENDSELECT_A::ONE
    }
    #[doc = "Extend with Most Significant Bit"]
    #[inline(always)]
    pub fn is_msbit(&self) -> bool {
        *self == EXTENDSELECT_A::MSBIT
    }
    #[doc = "Extend with Least Significant Bit"]
    #[inline(always)]
    pub fn is_lsbit(&self) -> bool {
        *self == EXTENDSELECT_A::LSBIT
    }
}
#[doc = "Field `EXTEND` writer - Data Formatting Bit Extension"]
pub type EXTEND_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, EXTENDSELECT_A>;
impl<'a, REG, const O: u8> EXTEND_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Extend with zeroes"]
    #[inline(always)]
    pub fn zero(self) -> &'a mut crate::W<REG> {
        self.variant(EXTENDSELECT_A::ZERO)
    }
    #[doc = "Extend with ones"]
    #[inline(always)]
    pub fn one(self) -> &'a mut crate::W<REG> {
        self.variant(EXTENDSELECT_A::ONE)
    }
    #[doc = "Extend with Most Significant Bit"]
    #[inline(always)]
    pub fn msbit(self) -> &'a mut crate::W<REG> {
        self.variant(EXTENDSELECT_A::MSBIT)
    }
    #[doc = "Extend with Least Significant Bit"]
    #[inline(always)]
    pub fn lsbit(self) -> &'a mut crate::W<REG> {
        self.variant(EXTENDSELECT_A::LSBIT)
    }
}
#[doc = "Field `BITREV` reader - Data Formatting Bit Reverse"]
pub type BITREV_R = crate::BitReader<BITREVSELECT_A>;
#[doc = "Data Formatting Bit Reverse\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BITREVSELECT_A {
    #[doc = "0: Transfer Data Most Significant Bit (MSB) first (default for I2S protocol)"]
    MSBIT = 0,
    #[doc = "1: Transfer Data Least Significant Bit (LSB) first"]
    LSBIT = 1,
}
impl From<BITREVSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: BITREVSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl BITREV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BITREVSELECT_A {
        match self.bits {
            false => BITREVSELECT_A::MSBIT,
            true => BITREVSELECT_A::LSBIT,
        }
    }
    #[doc = "Transfer Data Most Significant Bit (MSB) first (default for I2S protocol)"]
    #[inline(always)]
    pub fn is_msbit(&self) -> bool {
        *self == BITREVSELECT_A::MSBIT
    }
    #[doc = "Transfer Data Least Significant Bit (LSB) first"]
    #[inline(always)]
    pub fn is_lsbit(&self) -> bool {
        *self == BITREVSELECT_A::LSBIT
    }
}
#[doc = "Field `BITREV` writer - Data Formatting Bit Reverse"]
pub type BITREV_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, BITREVSELECT_A>;
impl<'a, REG, const O: u8> BITREV_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Transfer Data Most Significant Bit (MSB) first (default for I2S protocol)"]
    #[inline(always)]
    pub fn msbit(self) -> &'a mut crate::W<REG> {
        self.variant(BITREVSELECT_A::MSBIT)
    }
    #[doc = "Transfer Data Least Significant Bit (LSB) first"]
    #[inline(always)]
    pub fn lsbit(self) -> &'a mut crate::W<REG> {
        self.variant(BITREVSELECT_A::LSBIT)
    }
}
#[doc = "Field `SLOTDIS0` reader - Slot 0 Disabled for this Serializer"]
pub type SLOTDIS0_R = crate::BitReader;
#[doc = "Field `SLOTDIS0` writer - Slot 0 Disabled for this Serializer"]
pub type SLOTDIS0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SLOTDIS1` reader - Slot 1 Disabled for this Serializer"]
pub type SLOTDIS1_R = crate::BitReader;
#[doc = "Field `SLOTDIS1` writer - Slot 1 Disabled for this Serializer"]
pub type SLOTDIS1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SLOTDIS2` reader - Slot 2 Disabled for this Serializer"]
pub type SLOTDIS2_R = crate::BitReader;
#[doc = "Field `SLOTDIS2` writer - Slot 2 Disabled for this Serializer"]
pub type SLOTDIS2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SLOTDIS3` reader - Slot 3 Disabled for this Serializer"]
pub type SLOTDIS3_R = crate::BitReader;
#[doc = "Field `SLOTDIS3` writer - Slot 3 Disabled for this Serializer"]
pub type SLOTDIS3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SLOTDIS4` reader - Slot 4 Disabled for this Serializer"]
pub type SLOTDIS4_R = crate::BitReader;
#[doc = "Field `SLOTDIS4` writer - Slot 4 Disabled for this Serializer"]
pub type SLOTDIS4_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SLOTDIS5` reader - Slot 5 Disabled for this Serializer"]
pub type SLOTDIS5_R = crate::BitReader;
#[doc = "Field `SLOTDIS5` writer - Slot 5 Disabled for this Serializer"]
pub type SLOTDIS5_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SLOTDIS6` reader - Slot 6 Disabled for this Serializer"]
pub type SLOTDIS6_R = crate::BitReader;
#[doc = "Field `SLOTDIS6` writer - Slot 6 Disabled for this Serializer"]
pub type SLOTDIS6_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SLOTDIS7` reader - Slot 7 Disabled for this Serializer"]
pub type SLOTDIS7_R = crate::BitReader;
#[doc = "Field `SLOTDIS7` writer - Slot 7 Disabled for this Serializer"]
pub type SLOTDIS7_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MONO` reader - Mono Mode"]
pub type MONO_R = crate::BitReader<MONOSELECT_A>;
#[doc = "Mono Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MONOSELECT_A {
    #[doc = "0: Normal mode"]
    STEREO = 0,
    #[doc = "1: Left channel data is duplicated to right channel"]
    MONO = 1,
}
impl From<MONOSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: MONOSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl MONO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MONOSELECT_A {
        match self.bits {
            false => MONOSELECT_A::STEREO,
            true => MONOSELECT_A::MONO,
        }
    }
    #[doc = "Normal mode"]
    #[inline(always)]
    pub fn is_stereo(&self) -> bool {
        *self == MONOSELECT_A::STEREO
    }
    #[doc = "Left channel data is duplicated to right channel"]
    #[inline(always)]
    pub fn is_mono(&self) -> bool {
        *self == MONOSELECT_A::MONO
    }
}
#[doc = "Field `MONO` writer - Mono Mode"]
pub type MONO_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, MONOSELECT_A>;
impl<'a, REG, const O: u8> MONO_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal mode"]
    #[inline(always)]
    pub fn stereo(self) -> &'a mut crate::W<REG> {
        self.variant(MONOSELECT_A::STEREO)
    }
    #[doc = "Left channel data is duplicated to right channel"]
    #[inline(always)]
    pub fn mono(self) -> &'a mut crate::W<REG> {
        self.variant(MONOSELECT_A::MONO)
    }
}
#[doc = "Field `DMA` reader - Single or Multiple DMA Channels"]
pub type DMA_R = crate::BitReader<DMASELECT_A>;
#[doc = "Single or Multiple DMA Channels\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMASELECT_A {
    #[doc = "0: Single DMA channel"]
    SINGLE = 0,
    #[doc = "1: One DMA channel per data channel"]
    MULTIPLE = 1,
}
impl From<DMASELECT_A> for bool {
    #[inline(always)]
    fn from(variant: DMASELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl DMA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DMASELECT_A {
        match self.bits {
            false => DMASELECT_A::SINGLE,
            true => DMASELECT_A::MULTIPLE,
        }
    }
    #[doc = "Single DMA channel"]
    #[inline(always)]
    pub fn is_single(&self) -> bool {
        *self == DMASELECT_A::SINGLE
    }
    #[doc = "One DMA channel per data channel"]
    #[inline(always)]
    pub fn is_multiple(&self) -> bool {
        *self == DMASELECT_A::MULTIPLE
    }
}
#[doc = "Field `DMA` writer - Single or Multiple DMA Channels"]
pub type DMA_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, DMASELECT_A>;
impl<'a, REG, const O: u8> DMA_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Single DMA channel"]
    #[inline(always)]
    pub fn single(self) -> &'a mut crate::W<REG> {
        self.variant(DMASELECT_A::SINGLE)
    }
    #[doc = "One DMA channel per data channel"]
    #[inline(always)]
    pub fn multiple(self) -> &'a mut crate::W<REG> {
        self.variant(DMASELECT_A::MULTIPLE)
    }
}
#[doc = "Field `RXLOOP` reader - Loop-back Test Mode"]
pub type RXLOOP_R = crate::BitReader;
#[doc = "Field `RXLOOP` writer - Loop-back Test Mode"]
pub type RXLOOP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:1 - Serializer Mode"]
    #[inline(always)]
    pub fn sermode(&self) -> SERMODE_R {
        SERMODE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 5 - Clock Unit Selection"]
    #[inline(always)]
    pub fn clksel(&self) -> CLKSEL_R {
        CLKSEL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - Data Slot Formatting Adjust"]
    #[inline(always)]
    pub fn slotadj(&self) -> SLOTADJ_R {
        SLOTADJ_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - Data Word Size"]
    #[inline(always)]
    pub fn datasize(&self) -> DATASIZE_R {
        DATASIZE_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 12 - Data Word Formatting Adjust"]
    #[inline(always)]
    pub fn wordadj(&self) -> WORDADJ_R {
        WORDADJ_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:14 - Data Formatting Bit Extension"]
    #[inline(always)]
    pub fn extend(&self) -> EXTEND_R {
        EXTEND_R::new(((self.bits >> 13) & 3) as u8)
    }
    #[doc = "Bit 15 - Data Formatting Bit Reverse"]
    #[inline(always)]
    pub fn bitrev(&self) -> BITREV_R {
        BITREV_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Slot 0 Disabled for this Serializer"]
    #[inline(always)]
    pub fn slotdis0(&self) -> SLOTDIS0_R {
        SLOTDIS0_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Slot 1 Disabled for this Serializer"]
    #[inline(always)]
    pub fn slotdis1(&self) -> SLOTDIS1_R {
        SLOTDIS1_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Slot 2 Disabled for this Serializer"]
    #[inline(always)]
    pub fn slotdis2(&self) -> SLOTDIS2_R {
        SLOTDIS2_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Slot 3 Disabled for this Serializer"]
    #[inline(always)]
    pub fn slotdis3(&self) -> SLOTDIS3_R {
        SLOTDIS3_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Slot 4 Disabled for this Serializer"]
    #[inline(always)]
    pub fn slotdis4(&self) -> SLOTDIS4_R {
        SLOTDIS4_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Slot 5 Disabled for this Serializer"]
    #[inline(always)]
    pub fn slotdis5(&self) -> SLOTDIS5_R {
        SLOTDIS5_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Slot 6 Disabled for this Serializer"]
    #[inline(always)]
    pub fn slotdis6(&self) -> SLOTDIS6_R {
        SLOTDIS6_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Slot 7 Disabled for this Serializer"]
    #[inline(always)]
    pub fn slotdis7(&self) -> SLOTDIS7_R {
        SLOTDIS7_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Mono Mode"]
    #[inline(always)]
    pub fn mono(&self) -> MONO_R {
        MONO_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Single or Multiple DMA Channels"]
    #[inline(always)]
    pub fn dma(&self) -> DMA_R {
        DMA_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Loop-back Test Mode"]
    #[inline(always)]
    pub fn rxloop(&self) -> RXLOOP_R {
        RXLOOP_R::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Serializer Mode"]
    #[inline(always)]
    #[must_use]
    pub fn sermode(&mut self) -> SERMODE_W<RXCTRL_SPEC, 0> {
        SERMODE_W::new(self)
    }
    #[doc = "Bit 5 - Clock Unit Selection"]
    #[inline(always)]
    #[must_use]
    pub fn clksel(&mut self) -> CLKSEL_W<RXCTRL_SPEC, 5> {
        CLKSEL_W::new(self)
    }
    #[doc = "Bit 7 - Data Slot Formatting Adjust"]
    #[inline(always)]
    #[must_use]
    pub fn slotadj(&mut self) -> SLOTADJ_W<RXCTRL_SPEC, 7> {
        SLOTADJ_W::new(self)
    }
    #[doc = "Bits 8:10 - Data Word Size"]
    #[inline(always)]
    #[must_use]
    pub fn datasize(&mut self) -> DATASIZE_W<RXCTRL_SPEC, 8> {
        DATASIZE_W::new(self)
    }
    #[doc = "Bit 12 - Data Word Formatting Adjust"]
    #[inline(always)]
    #[must_use]
    pub fn wordadj(&mut self) -> WORDADJ_W<RXCTRL_SPEC, 12> {
        WORDADJ_W::new(self)
    }
    #[doc = "Bits 13:14 - Data Formatting Bit Extension"]
    #[inline(always)]
    #[must_use]
    pub fn extend(&mut self) -> EXTEND_W<RXCTRL_SPEC, 13> {
        EXTEND_W::new(self)
    }
    #[doc = "Bit 15 - Data Formatting Bit Reverse"]
    #[inline(always)]
    #[must_use]
    pub fn bitrev(&mut self) -> BITREV_W<RXCTRL_SPEC, 15> {
        BITREV_W::new(self)
    }
    #[doc = "Bit 16 - Slot 0 Disabled for this Serializer"]
    #[inline(always)]
    #[must_use]
    pub fn slotdis0(&mut self) -> SLOTDIS0_W<RXCTRL_SPEC, 16> {
        SLOTDIS0_W::new(self)
    }
    #[doc = "Bit 17 - Slot 1 Disabled for this Serializer"]
    #[inline(always)]
    #[must_use]
    pub fn slotdis1(&mut self) -> SLOTDIS1_W<RXCTRL_SPEC, 17> {
        SLOTDIS1_W::new(self)
    }
    #[doc = "Bit 18 - Slot 2 Disabled for this Serializer"]
    #[inline(always)]
    #[must_use]
    pub fn slotdis2(&mut self) -> SLOTDIS2_W<RXCTRL_SPEC, 18> {
        SLOTDIS2_W::new(self)
    }
    #[doc = "Bit 19 - Slot 3 Disabled for this Serializer"]
    #[inline(always)]
    #[must_use]
    pub fn slotdis3(&mut self) -> SLOTDIS3_W<RXCTRL_SPEC, 19> {
        SLOTDIS3_W::new(self)
    }
    #[doc = "Bit 20 - Slot 4 Disabled for this Serializer"]
    #[inline(always)]
    #[must_use]
    pub fn slotdis4(&mut self) -> SLOTDIS4_W<RXCTRL_SPEC, 20> {
        SLOTDIS4_W::new(self)
    }
    #[doc = "Bit 21 - Slot 5 Disabled for this Serializer"]
    #[inline(always)]
    #[must_use]
    pub fn slotdis5(&mut self) -> SLOTDIS5_W<RXCTRL_SPEC, 21> {
        SLOTDIS5_W::new(self)
    }
    #[doc = "Bit 22 - Slot 6 Disabled for this Serializer"]
    #[inline(always)]
    #[must_use]
    pub fn slotdis6(&mut self) -> SLOTDIS6_W<RXCTRL_SPEC, 22> {
        SLOTDIS6_W::new(self)
    }
    #[doc = "Bit 23 - Slot 7 Disabled for this Serializer"]
    #[inline(always)]
    #[must_use]
    pub fn slotdis7(&mut self) -> SLOTDIS7_W<RXCTRL_SPEC, 23> {
        SLOTDIS7_W::new(self)
    }
    #[doc = "Bit 24 - Mono Mode"]
    #[inline(always)]
    #[must_use]
    pub fn mono(&mut self) -> MONO_W<RXCTRL_SPEC, 24> {
        MONO_W::new(self)
    }
    #[doc = "Bit 25 - Single or Multiple DMA Channels"]
    #[inline(always)]
    #[must_use]
    pub fn dma(&mut self) -> DMA_W<RXCTRL_SPEC, 25> {
        DMA_W::new(self)
    }
    #[doc = "Bit 26 - Loop-back Test Mode"]
    #[inline(always)]
    #[must_use]
    pub fn rxloop(&mut self) -> RXLOOP_W<RXCTRL_SPEC, 26> {
        RXLOOP_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Rx Serializer Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RXCTRL_SPEC;
impl crate::RegisterSpec for RXCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxctrl::R`](R) reader structure"]
impl crate::Readable for RXCTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rxctrl::W`](W) writer structure"]
impl crate::Writable for RXCTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RXCTRL to value 0"]
impl crate::Resettable for RXCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
