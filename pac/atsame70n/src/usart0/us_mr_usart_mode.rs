#[doc = "Register `US_MR_USART_MODE` reader"]
pub struct R(crate::R<US_MR_USART_MODE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<US_MR_USART_MODE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<US_MR_USART_MODE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<US_MR_USART_MODE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `US_MR_USART_MODE` writer"]
pub struct W(crate::W<US_MR_USART_MODE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<US_MR_USART_MODE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<US_MR_USART_MODE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<US_MR_USART_MODE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "USART Mode of Operation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum USART_MODE_A {
    #[doc = "0: Normal mode"]
    NORMAL = 0,
    #[doc = "1: RS485"]
    RS485 = 1,
    #[doc = "2: Hardware handshaking"]
    HW_HANDSHAKING = 2,
    #[doc = "3: Modem"]
    MODEM = 3,
    #[doc = "4: IS07816 Protocol: T = 0"]
    IS07816_T_0 = 4,
    #[doc = "6: IS07816 Protocol: T = 1"]
    IS07816_T_1 = 6,
    #[doc = "8: IrDA"]
    IRDA = 8,
    #[doc = "9: LON"]
    LON = 9,
    #[doc = "10: LIN Master mode"]
    LIN_MASTER = 10,
    #[doc = "11: LIN Slave mode"]
    LIN_SLAVE = 11,
    #[doc = "14: SPI Master mode (CLKO must be written to 1 and USCLKS = 0, 1 or 2)"]
    SPI_MASTER = 14,
    #[doc = "15: SPI Slave mode"]
    SPI_SLAVE = 15,
}
impl From<USART_MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: USART_MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `USART_MODE` reader - USART Mode of Operation"]
pub struct USART_MODE_R(crate::FieldReader<u8, USART_MODE_A>);
impl USART_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        USART_MODE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<USART_MODE_A> {
        match self.bits {
            0 => Some(USART_MODE_A::NORMAL),
            1 => Some(USART_MODE_A::RS485),
            2 => Some(USART_MODE_A::HW_HANDSHAKING),
            3 => Some(USART_MODE_A::MODEM),
            4 => Some(USART_MODE_A::IS07816_T_0),
            6 => Some(USART_MODE_A::IS07816_T_1),
            8 => Some(USART_MODE_A::IRDA),
            9 => Some(USART_MODE_A::LON),
            10 => Some(USART_MODE_A::LIN_MASTER),
            11 => Some(USART_MODE_A::LIN_SLAVE),
            14 => Some(USART_MODE_A::SPI_MASTER),
            15 => Some(USART_MODE_A::SPI_SLAVE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        **self == USART_MODE_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `RS485`"]
    #[inline(always)]
    pub fn is_rs485(&self) -> bool {
        **self == USART_MODE_A::RS485
    }
    #[doc = "Checks if the value of the field is `HW_HANDSHAKING`"]
    #[inline(always)]
    pub fn is_hw_handshaking(&self) -> bool {
        **self == USART_MODE_A::HW_HANDSHAKING
    }
    #[doc = "Checks if the value of the field is `MODEM`"]
    #[inline(always)]
    pub fn is_modem(&self) -> bool {
        **self == USART_MODE_A::MODEM
    }
    #[doc = "Checks if the value of the field is `IS07816_T_0`"]
    #[inline(always)]
    pub fn is_is07816_t_0(&self) -> bool {
        **self == USART_MODE_A::IS07816_T_0
    }
    #[doc = "Checks if the value of the field is `IS07816_T_1`"]
    #[inline(always)]
    pub fn is_is07816_t_1(&self) -> bool {
        **self == USART_MODE_A::IS07816_T_1
    }
    #[doc = "Checks if the value of the field is `IRDA`"]
    #[inline(always)]
    pub fn is_irda(&self) -> bool {
        **self == USART_MODE_A::IRDA
    }
    #[doc = "Checks if the value of the field is `LON`"]
    #[inline(always)]
    pub fn is_lon(&self) -> bool {
        **self == USART_MODE_A::LON
    }
    #[doc = "Checks if the value of the field is `LIN_MASTER`"]
    #[inline(always)]
    pub fn is_lin_master(&self) -> bool {
        **self == USART_MODE_A::LIN_MASTER
    }
    #[doc = "Checks if the value of the field is `LIN_SLAVE`"]
    #[inline(always)]
    pub fn is_lin_slave(&self) -> bool {
        **self == USART_MODE_A::LIN_SLAVE
    }
    #[doc = "Checks if the value of the field is `SPI_MASTER`"]
    #[inline(always)]
    pub fn is_spi_master(&self) -> bool {
        **self == USART_MODE_A::SPI_MASTER
    }
    #[doc = "Checks if the value of the field is `SPI_SLAVE`"]
    #[inline(always)]
    pub fn is_spi_slave(&self) -> bool {
        **self == USART_MODE_A::SPI_SLAVE
    }
}
impl core::ops::Deref for USART_MODE_R {
    type Target = crate::FieldReader<u8, USART_MODE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USART_MODE` writer - USART Mode of Operation"]
pub struct USART_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> USART_MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USART_MODE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Normal mode"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(USART_MODE_A::NORMAL)
    }
    #[doc = "RS485"]
    #[inline(always)]
    pub fn rs485(self) -> &'a mut W {
        self.variant(USART_MODE_A::RS485)
    }
    #[doc = "Hardware handshaking"]
    #[inline(always)]
    pub fn hw_handshaking(self) -> &'a mut W {
        self.variant(USART_MODE_A::HW_HANDSHAKING)
    }
    #[doc = "Modem"]
    #[inline(always)]
    pub fn modem(self) -> &'a mut W {
        self.variant(USART_MODE_A::MODEM)
    }
    #[doc = "IS07816 Protocol: T = 0"]
    #[inline(always)]
    pub fn is07816_t_0(self) -> &'a mut W {
        self.variant(USART_MODE_A::IS07816_T_0)
    }
    #[doc = "IS07816 Protocol: T = 1"]
    #[inline(always)]
    pub fn is07816_t_1(self) -> &'a mut W {
        self.variant(USART_MODE_A::IS07816_T_1)
    }
    #[doc = "IrDA"]
    #[inline(always)]
    pub fn irda(self) -> &'a mut W {
        self.variant(USART_MODE_A::IRDA)
    }
    #[doc = "LON"]
    #[inline(always)]
    pub fn lon(self) -> &'a mut W {
        self.variant(USART_MODE_A::LON)
    }
    #[doc = "LIN Master mode"]
    #[inline(always)]
    pub fn lin_master(self) -> &'a mut W {
        self.variant(USART_MODE_A::LIN_MASTER)
    }
    #[doc = "LIN Slave mode"]
    #[inline(always)]
    pub fn lin_slave(self) -> &'a mut W {
        self.variant(USART_MODE_A::LIN_SLAVE)
    }
    #[doc = "SPI Master mode (CLKO must be written to 1 and USCLKS = 0, 1 or 2)"]
    #[inline(always)]
    pub fn spi_master(self) -> &'a mut W {
        self.variant(USART_MODE_A::SPI_MASTER)
    }
    #[doc = "SPI Slave mode"]
    #[inline(always)]
    pub fn spi_slave(self) -> &'a mut W {
        self.variant(USART_MODE_A::SPI_SLAVE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
#[doc = "Clock Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum USCLKS_A {
    #[doc = "0: Peripheral clock is selected"]
    MCK = 0,
    #[doc = "1: Peripheral clock divided (DIV = 8) is selected"]
    DIV = 1,
    #[doc = "2: PMC programmable clock (PCK) is selected. If the SCK pin is driven (CLKO = 1), the CD field must be greater than 1."]
    PCK = 2,
    #[doc = "3: Serial clock (SCK) is selected"]
    SCK = 3,
}
impl From<USCLKS_A> for u8 {
    #[inline(always)]
    fn from(variant: USCLKS_A) -> Self {
        variant as _
    }
}
#[doc = "Field `USCLKS` reader - Clock Selection"]
pub struct USCLKS_R(crate::FieldReader<u8, USCLKS_A>);
impl USCLKS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        USCLKS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USCLKS_A {
        match self.bits {
            0 => USCLKS_A::MCK,
            1 => USCLKS_A::DIV,
            2 => USCLKS_A::PCK,
            3 => USCLKS_A::SCK,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `MCK`"]
    #[inline(always)]
    pub fn is_mck(&self) -> bool {
        **self == USCLKS_A::MCK
    }
    #[doc = "Checks if the value of the field is `DIV`"]
    #[inline(always)]
    pub fn is_div(&self) -> bool {
        **self == USCLKS_A::DIV
    }
    #[doc = "Checks if the value of the field is `PCK`"]
    #[inline(always)]
    pub fn is_pck(&self) -> bool {
        **self == USCLKS_A::PCK
    }
    #[doc = "Checks if the value of the field is `SCK`"]
    #[inline(always)]
    pub fn is_sck(&self) -> bool {
        **self == USCLKS_A::SCK
    }
}
impl core::ops::Deref for USCLKS_R {
    type Target = crate::FieldReader<u8, USCLKS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USCLKS` writer - Clock Selection"]
pub struct USCLKS_W<'a> {
    w: &'a mut W,
}
impl<'a> USCLKS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USCLKS_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Peripheral clock is selected"]
    #[inline(always)]
    pub fn mck(self) -> &'a mut W {
        self.variant(USCLKS_A::MCK)
    }
    #[doc = "Peripheral clock divided (DIV = 8) is selected"]
    #[inline(always)]
    pub fn div(self) -> &'a mut W {
        self.variant(USCLKS_A::DIV)
    }
    #[doc = "PMC programmable clock (PCK) is selected. If the SCK pin is driven (CLKO = 1), the CD field must be greater than 1."]
    #[inline(always)]
    pub fn pck(self) -> &'a mut W {
        self.variant(USCLKS_A::PCK)
    }
    #[doc = "Serial clock (SCK) is selected"]
    #[inline(always)]
    pub fn sck(self) -> &'a mut W {
        self.variant(USCLKS_A::SCK)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
#[doc = "Character Length\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CHRL_A {
    #[doc = "0: Character length is 5 bits"]
    _5_BIT = 0,
    #[doc = "1: Character length is 6 bits"]
    _6_BIT = 1,
    #[doc = "2: Character length is 7 bits"]
    _7_BIT = 2,
    #[doc = "3: Character length is 8 bits"]
    _8_BIT = 3,
}
impl From<CHRL_A> for u8 {
    #[inline(always)]
    fn from(variant: CHRL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CHRL` reader - Character Length"]
pub struct CHRL_R(crate::FieldReader<u8, CHRL_A>);
impl CHRL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CHRL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHRL_A {
        match self.bits {
            0 => CHRL_A::_5_BIT,
            1 => CHRL_A::_6_BIT,
            2 => CHRL_A::_7_BIT,
            3 => CHRL_A::_8_BIT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_5_BIT`"]
    #[inline(always)]
    pub fn is_5_bit(&self) -> bool {
        **self == CHRL_A::_5_BIT
    }
    #[doc = "Checks if the value of the field is `_6_BIT`"]
    #[inline(always)]
    pub fn is_6_bit(&self) -> bool {
        **self == CHRL_A::_6_BIT
    }
    #[doc = "Checks if the value of the field is `_7_BIT`"]
    #[inline(always)]
    pub fn is_7_bit(&self) -> bool {
        **self == CHRL_A::_7_BIT
    }
    #[doc = "Checks if the value of the field is `_8_BIT`"]
    #[inline(always)]
    pub fn is_8_bit(&self) -> bool {
        **self == CHRL_A::_8_BIT
    }
}
impl core::ops::Deref for CHRL_R {
    type Target = crate::FieldReader<u8, CHRL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHRL` writer - Character Length"]
pub struct CHRL_W<'a> {
    w: &'a mut W,
}
impl<'a> CHRL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHRL_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Character length is 5 bits"]
    #[inline(always)]
    pub fn _5_bit(self) -> &'a mut W {
        self.variant(CHRL_A::_5_BIT)
    }
    #[doc = "Character length is 6 bits"]
    #[inline(always)]
    pub fn _6_bit(self) -> &'a mut W {
        self.variant(CHRL_A::_6_BIT)
    }
    #[doc = "Character length is 7 bits"]
    #[inline(always)]
    pub fn _7_bit(self) -> &'a mut W {
        self.variant(CHRL_A::_7_BIT)
    }
    #[doc = "Character length is 8 bits"]
    #[inline(always)]
    pub fn _8_bit(self) -> &'a mut W {
        self.variant(CHRL_A::_8_BIT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | ((value as u32 & 0x03) << 6);
        self.w
    }
}
#[doc = "Field `SYNC` reader - Synchronous Mode Select"]
pub struct SYNC_R(crate::FieldReader<bool, bool>);
impl SYNC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SYNC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SYNC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SYNC` writer - Synchronous Mode Select"]
pub struct SYNC_W<'a> {
    w: &'a mut W,
}
impl<'a> SYNC_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Parity Type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PAR_A {
    #[doc = "0: Even parity"]
    EVEN = 0,
    #[doc = "1: Odd parity"]
    ODD = 1,
    #[doc = "2: Parity forced to 0 (Space)"]
    SPACE = 2,
    #[doc = "3: Parity forced to 1 (Mark)"]
    MARK = 3,
    #[doc = "4: No parity"]
    NO = 4,
    #[doc = "6: Multidrop mode"]
    MULTIDROP = 6,
}
impl From<PAR_A> for u8 {
    #[inline(always)]
    fn from(variant: PAR_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PAR` reader - Parity Type"]
pub struct PAR_R(crate::FieldReader<u8, PAR_A>);
impl PAR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PAR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PAR_A> {
        match self.bits {
            0 => Some(PAR_A::EVEN),
            1 => Some(PAR_A::ODD),
            2 => Some(PAR_A::SPACE),
            3 => Some(PAR_A::MARK),
            4 => Some(PAR_A::NO),
            6 => Some(PAR_A::MULTIDROP),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `EVEN`"]
    #[inline(always)]
    pub fn is_even(&self) -> bool {
        **self == PAR_A::EVEN
    }
    #[doc = "Checks if the value of the field is `ODD`"]
    #[inline(always)]
    pub fn is_odd(&self) -> bool {
        **self == PAR_A::ODD
    }
    #[doc = "Checks if the value of the field is `SPACE`"]
    #[inline(always)]
    pub fn is_space(&self) -> bool {
        **self == PAR_A::SPACE
    }
    #[doc = "Checks if the value of the field is `MARK`"]
    #[inline(always)]
    pub fn is_mark(&self) -> bool {
        **self == PAR_A::MARK
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        **self == PAR_A::NO
    }
    #[doc = "Checks if the value of the field is `MULTIDROP`"]
    #[inline(always)]
    pub fn is_multidrop(&self) -> bool {
        **self == PAR_A::MULTIDROP
    }
}
impl core::ops::Deref for PAR_R {
    type Target = crate::FieldReader<u8, PAR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAR` writer - Parity Type"]
pub struct PAR_W<'a> {
    w: &'a mut W,
}
impl<'a> PAR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAR_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Even parity"]
    #[inline(always)]
    pub fn even(self) -> &'a mut W {
        self.variant(PAR_A::EVEN)
    }
    #[doc = "Odd parity"]
    #[inline(always)]
    pub fn odd(self) -> &'a mut W {
        self.variant(PAR_A::ODD)
    }
    #[doc = "Parity forced to 0 (Space)"]
    #[inline(always)]
    pub fn space(self) -> &'a mut W {
        self.variant(PAR_A::SPACE)
    }
    #[doc = "Parity forced to 1 (Mark)"]
    #[inline(always)]
    pub fn mark(self) -> &'a mut W {
        self.variant(PAR_A::MARK)
    }
    #[doc = "No parity"]
    #[inline(always)]
    pub fn no(self) -> &'a mut W {
        self.variant(PAR_A::NO)
    }
    #[doc = "Multidrop mode"]
    #[inline(always)]
    pub fn multidrop(self) -> &'a mut W {
        self.variant(PAR_A::MULTIDROP)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 9)) | ((value as u32 & 0x07) << 9);
        self.w
    }
}
#[doc = "Number of Stop Bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum NBSTOP_A {
    #[doc = "0: 1 stop bit"]
    _1_BIT = 0,
    #[doc = "1: 1.5 stop bit (SYNC = 0) or reserved (SYNC = 1)"]
    _1_5_BIT = 1,
    #[doc = "2: 2 stop bits"]
    _2_BIT = 2,
}
impl From<NBSTOP_A> for u8 {
    #[inline(always)]
    fn from(variant: NBSTOP_A) -> Self {
        variant as _
    }
}
#[doc = "Field `NBSTOP` reader - Number of Stop Bits"]
pub struct NBSTOP_R(crate::FieldReader<u8, NBSTOP_A>);
impl NBSTOP_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        NBSTOP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<NBSTOP_A> {
        match self.bits {
            0 => Some(NBSTOP_A::_1_BIT),
            1 => Some(NBSTOP_A::_1_5_BIT),
            2 => Some(NBSTOP_A::_2_BIT),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_1_BIT`"]
    #[inline(always)]
    pub fn is_1_bit(&self) -> bool {
        **self == NBSTOP_A::_1_BIT
    }
    #[doc = "Checks if the value of the field is `_1_5_BIT`"]
    #[inline(always)]
    pub fn is_1_5_bit(&self) -> bool {
        **self == NBSTOP_A::_1_5_BIT
    }
    #[doc = "Checks if the value of the field is `_2_BIT`"]
    #[inline(always)]
    pub fn is_2_bit(&self) -> bool {
        **self == NBSTOP_A::_2_BIT
    }
}
impl core::ops::Deref for NBSTOP_R {
    type Target = crate::FieldReader<u8, NBSTOP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NBSTOP` writer - Number of Stop Bits"]
pub struct NBSTOP_W<'a> {
    w: &'a mut W,
}
impl<'a> NBSTOP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: NBSTOP_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "1 stop bit"]
    #[inline(always)]
    pub fn _1_bit(self) -> &'a mut W {
        self.variant(NBSTOP_A::_1_BIT)
    }
    #[doc = "1.5 stop bit (SYNC = 0) or reserved (SYNC = 1)"]
    #[inline(always)]
    pub fn _1_5_bit(self) -> &'a mut W {
        self.variant(NBSTOP_A::_1_5_BIT)
    }
    #[doc = "2 stop bits"]
    #[inline(always)]
    pub fn _2_bit(self) -> &'a mut W {
        self.variant(NBSTOP_A::_2_BIT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | ((value as u32 & 0x03) << 12);
        self.w
    }
}
#[doc = "Channel Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CHMODE_A {
    #[doc = "0: Normal mode"]
    NORMAL = 0,
    #[doc = "1: Automatic Echo. Receiver input is connected to the TXD pin."]
    AUTOMATIC = 1,
    #[doc = "2: Local Loopback. Transmitter output is connected to the Receiver Input."]
    LOCAL_LOOPBACK = 2,
    #[doc = "3: Remote Loopback. RXD pin is internally connected to the TXD pin."]
    REMOTE_LOOPBACK = 3,
}
impl From<CHMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: CHMODE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CHMODE` reader - Channel Mode"]
pub struct CHMODE_R(crate::FieldReader<u8, CHMODE_A>);
impl CHMODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CHMODE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHMODE_A {
        match self.bits {
            0 => CHMODE_A::NORMAL,
            1 => CHMODE_A::AUTOMATIC,
            2 => CHMODE_A::LOCAL_LOOPBACK,
            3 => CHMODE_A::REMOTE_LOOPBACK,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        **self == CHMODE_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `AUTOMATIC`"]
    #[inline(always)]
    pub fn is_automatic(&self) -> bool {
        **self == CHMODE_A::AUTOMATIC
    }
    #[doc = "Checks if the value of the field is `LOCAL_LOOPBACK`"]
    #[inline(always)]
    pub fn is_local_loopback(&self) -> bool {
        **self == CHMODE_A::LOCAL_LOOPBACK
    }
    #[doc = "Checks if the value of the field is `REMOTE_LOOPBACK`"]
    #[inline(always)]
    pub fn is_remote_loopback(&self) -> bool {
        **self == CHMODE_A::REMOTE_LOOPBACK
    }
}
impl core::ops::Deref for CHMODE_R {
    type Target = crate::FieldReader<u8, CHMODE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHMODE` writer - Channel Mode"]
pub struct CHMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> CHMODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHMODE_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Normal mode"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(CHMODE_A::NORMAL)
    }
    #[doc = "Automatic Echo. Receiver input is connected to the TXD pin."]
    #[inline(always)]
    pub fn automatic(self) -> &'a mut W {
        self.variant(CHMODE_A::AUTOMATIC)
    }
    #[doc = "Local Loopback. Transmitter output is connected to the Receiver Input."]
    #[inline(always)]
    pub fn local_loopback(self) -> &'a mut W {
        self.variant(CHMODE_A::LOCAL_LOOPBACK)
    }
    #[doc = "Remote Loopback. RXD pin is internally connected to the TXD pin."]
    #[inline(always)]
    pub fn remote_loopback(self) -> &'a mut W {
        self.variant(CHMODE_A::REMOTE_LOOPBACK)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | ((value as u32 & 0x03) << 14);
        self.w
    }
}
#[doc = "Field `MSBF` reader - Bit Order"]
pub struct MSBF_R(crate::FieldReader<bool, bool>);
impl MSBF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MSBF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MSBF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MSBF` writer - Bit Order"]
pub struct MSBF_W<'a> {
    w: &'a mut W,
}
impl<'a> MSBF_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Field `MODE9` reader - 9-bit Character Length"]
pub struct MODE9_R(crate::FieldReader<bool, bool>);
impl MODE9_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MODE9_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MODE9_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MODE9` writer - 9-bit Character Length"]
pub struct MODE9_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE9_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "Field `CLKO` reader - Clock Output Select"]
pub struct CLKO_R(crate::FieldReader<bool, bool>);
impl CLKO_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CLKO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLKO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLKO` writer - Clock Output Select"]
pub struct CLKO_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKO_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
#[doc = "Field `OVER` reader - Oversampling Mode"]
pub struct OVER_R(crate::FieldReader<bool, bool>);
impl OVER_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OVER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVER_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVER` writer - Oversampling Mode"]
pub struct OVER_W<'a> {
    w: &'a mut W,
}
impl<'a> OVER_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
#[doc = "Field `INACK` reader - Inhibit Non Acknowledge"]
pub struct INACK_R(crate::FieldReader<bool, bool>);
impl INACK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INACK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INACK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INACK` writer - Inhibit Non Acknowledge"]
pub struct INACK_W<'a> {
    w: &'a mut W,
}
impl<'a> INACK_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
#[doc = "Field `DSNACK` reader - Disable Successive NACK"]
pub struct DSNACK_R(crate::FieldReader<bool, bool>);
impl DSNACK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DSNACK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DSNACK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DSNACK` writer - Disable Successive NACK"]
pub struct DSNACK_W<'a> {
    w: &'a mut W,
}
impl<'a> DSNACK_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 21)) | ((value as u32 & 0x01) << 21);
        self.w
    }
}
#[doc = "Field `VAR_SYNC` reader - Variable Synchronization of Command/Data Sync Start Frame Delimiter"]
pub struct VAR_SYNC_R(crate::FieldReader<bool, bool>);
impl VAR_SYNC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        VAR_SYNC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VAR_SYNC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VAR_SYNC` writer - Variable Synchronization of Command/Data Sync Start Frame Delimiter"]
pub struct VAR_SYNC_W<'a> {
    w: &'a mut W,
}
impl<'a> VAR_SYNC_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 22)) | ((value as u32 & 0x01) << 22);
        self.w
    }
}
#[doc = "Field `INVDATA` reader - Inverted Data"]
pub struct INVDATA_R(crate::FieldReader<bool, bool>);
impl INVDATA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INVDATA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INVDATA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INVDATA` writer - Inverted Data"]
pub struct INVDATA_W<'a> {
    w: &'a mut W,
}
impl<'a> INVDATA_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 23)) | ((value as u32 & 0x01) << 23);
        self.w
    }
}
#[doc = "Field `MAX_ITERATION` reader - Maximum Number of Automatic Iteration"]
pub struct MAX_ITERATION_R(crate::FieldReader<u8, u8>);
impl MAX_ITERATION_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        MAX_ITERATION_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MAX_ITERATION_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MAX_ITERATION` writer - Maximum Number of Automatic Iteration"]
pub struct MAX_ITERATION_W<'a> {
    w: &'a mut W,
}
impl<'a> MAX_ITERATION_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 24)) | ((value as u32 & 0x07) << 24);
        self.w
    }
}
#[doc = "Field `FILTER` reader - Receive Line Filter"]
pub struct FILTER_R(crate::FieldReader<bool, bool>);
impl FILTER_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FILTER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FILTER_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FILTER` writer - Receive Line Filter"]
pub struct FILTER_W<'a> {
    w: &'a mut W,
}
impl<'a> FILTER_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 28)) | ((value as u32 & 0x01) << 28);
        self.w
    }
}
#[doc = "Field `MAN` reader - Manchester Encoder/Decoder Enable"]
pub struct MAN_R(crate::FieldReader<bool, bool>);
impl MAN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MAN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MAN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MAN` writer - Manchester Encoder/Decoder Enable"]
pub struct MAN_W<'a> {
    w: &'a mut W,
}
impl<'a> MAN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 29)) | ((value as u32 & 0x01) << 29);
        self.w
    }
}
#[doc = "Field `MODSYNC` reader - Manchester Synchronization Mode"]
pub struct MODSYNC_R(crate::FieldReader<bool, bool>);
impl MODSYNC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MODSYNC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MODSYNC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MODSYNC` writer - Manchester Synchronization Mode"]
pub struct MODSYNC_W<'a> {
    w: &'a mut W,
}
impl<'a> MODSYNC_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 30)) | ((value as u32 & 0x01) << 30);
        self.w
    }
}
#[doc = "Field `ONEBIT` reader - Start Frame Delimiter Selector"]
pub struct ONEBIT_R(crate::FieldReader<bool, bool>);
impl ONEBIT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ONEBIT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ONEBIT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ONEBIT` writer - Start Frame Delimiter Selector"]
pub struct ONEBIT_W<'a> {
    w: &'a mut W,
}
impl<'a> ONEBIT_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - USART Mode of Operation"]
    #[inline(always)]
    pub fn usart_mode(&self) -> USART_MODE_R {
        USART_MODE_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:5 - Clock Selection"]
    #[inline(always)]
    pub fn usclks(&self) -> USCLKS_R {
        USCLKS_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - Character Length"]
    #[inline(always)]
    pub fn chrl(&self) -> CHRL_R {
        CHRL_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bit 8 - Synchronous Mode Select"]
    #[inline(always)]
    pub fn sync(&self) -> SYNC_R {
        SYNC_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 9:11 - Parity Type"]
    #[inline(always)]
    pub fn par(&self) -> PAR_R {
        PAR_R::new(((self.bits >> 9) & 0x07) as u8)
    }
    #[doc = "Bits 12:13 - Number of Stop Bits"]
    #[inline(always)]
    pub fn nbstop(&self) -> NBSTOP_R {
        NBSTOP_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 14:15 - Channel Mode"]
    #[inline(always)]
    pub fn chmode(&self) -> CHMODE_R {
        CHMODE_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bit 16 - Bit Order"]
    #[inline(always)]
    pub fn msbf(&self) -> MSBF_R {
        MSBF_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - 9-bit Character Length"]
    #[inline(always)]
    pub fn mode9(&self) -> MODE9_R {
        MODE9_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Clock Output Select"]
    #[inline(always)]
    pub fn clko(&self) -> CLKO_R {
        CLKO_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Oversampling Mode"]
    #[inline(always)]
    pub fn over(&self) -> OVER_R {
        OVER_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Inhibit Non Acknowledge"]
    #[inline(always)]
    pub fn inack(&self) -> INACK_R {
        INACK_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Disable Successive NACK"]
    #[inline(always)]
    pub fn dsnack(&self) -> DSNACK_R {
        DSNACK_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Variable Synchronization of Command/Data Sync Start Frame Delimiter"]
    #[inline(always)]
    pub fn var_sync(&self) -> VAR_SYNC_R {
        VAR_SYNC_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Inverted Data"]
    #[inline(always)]
    pub fn invdata(&self) -> INVDATA_R {
        INVDATA_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bits 24:26 - Maximum Number of Automatic Iteration"]
    #[inline(always)]
    pub fn max_iteration(&self) -> MAX_ITERATION_R {
        MAX_ITERATION_R::new(((self.bits >> 24) & 0x07) as u8)
    }
    #[doc = "Bit 28 - Receive Line Filter"]
    #[inline(always)]
    pub fn filter(&self) -> FILTER_R {
        FILTER_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Manchester Encoder/Decoder Enable"]
    #[inline(always)]
    pub fn man(&self) -> MAN_R {
        MAN_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Manchester Synchronization Mode"]
    #[inline(always)]
    pub fn modsync(&self) -> MODSYNC_R {
        MODSYNC_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Start Frame Delimiter Selector"]
    #[inline(always)]
    pub fn onebit(&self) -> ONEBIT_R {
        ONEBIT_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - USART Mode of Operation"]
    #[inline(always)]
    pub fn usart_mode(&mut self) -> USART_MODE_W {
        USART_MODE_W { w: self }
    }
    #[doc = "Bits 4:5 - Clock Selection"]
    #[inline(always)]
    pub fn usclks(&mut self) -> USCLKS_W {
        USCLKS_W { w: self }
    }
    #[doc = "Bits 6:7 - Character Length"]
    #[inline(always)]
    pub fn chrl(&mut self) -> CHRL_W {
        CHRL_W { w: self }
    }
    #[doc = "Bit 8 - Synchronous Mode Select"]
    #[inline(always)]
    pub fn sync(&mut self) -> SYNC_W {
        SYNC_W { w: self }
    }
    #[doc = "Bits 9:11 - Parity Type"]
    #[inline(always)]
    pub fn par(&mut self) -> PAR_W {
        PAR_W { w: self }
    }
    #[doc = "Bits 12:13 - Number of Stop Bits"]
    #[inline(always)]
    pub fn nbstop(&mut self) -> NBSTOP_W {
        NBSTOP_W { w: self }
    }
    #[doc = "Bits 14:15 - Channel Mode"]
    #[inline(always)]
    pub fn chmode(&mut self) -> CHMODE_W {
        CHMODE_W { w: self }
    }
    #[doc = "Bit 16 - Bit Order"]
    #[inline(always)]
    pub fn msbf(&mut self) -> MSBF_W {
        MSBF_W { w: self }
    }
    #[doc = "Bit 17 - 9-bit Character Length"]
    #[inline(always)]
    pub fn mode9(&mut self) -> MODE9_W {
        MODE9_W { w: self }
    }
    #[doc = "Bit 18 - Clock Output Select"]
    #[inline(always)]
    pub fn clko(&mut self) -> CLKO_W {
        CLKO_W { w: self }
    }
    #[doc = "Bit 19 - Oversampling Mode"]
    #[inline(always)]
    pub fn over(&mut self) -> OVER_W {
        OVER_W { w: self }
    }
    #[doc = "Bit 20 - Inhibit Non Acknowledge"]
    #[inline(always)]
    pub fn inack(&mut self) -> INACK_W {
        INACK_W { w: self }
    }
    #[doc = "Bit 21 - Disable Successive NACK"]
    #[inline(always)]
    pub fn dsnack(&mut self) -> DSNACK_W {
        DSNACK_W { w: self }
    }
    #[doc = "Bit 22 - Variable Synchronization of Command/Data Sync Start Frame Delimiter"]
    #[inline(always)]
    pub fn var_sync(&mut self) -> VAR_SYNC_W {
        VAR_SYNC_W { w: self }
    }
    #[doc = "Bit 23 - Inverted Data"]
    #[inline(always)]
    pub fn invdata(&mut self) -> INVDATA_W {
        INVDATA_W { w: self }
    }
    #[doc = "Bits 24:26 - Maximum Number of Automatic Iteration"]
    #[inline(always)]
    pub fn max_iteration(&mut self) -> MAX_ITERATION_W {
        MAX_ITERATION_W { w: self }
    }
    #[doc = "Bit 28 - Receive Line Filter"]
    #[inline(always)]
    pub fn filter(&mut self) -> FILTER_W {
        FILTER_W { w: self }
    }
    #[doc = "Bit 29 - Manchester Encoder/Decoder Enable"]
    #[inline(always)]
    pub fn man(&mut self) -> MAN_W {
        MAN_W { w: self }
    }
    #[doc = "Bit 30 - Manchester Synchronization Mode"]
    #[inline(always)]
    pub fn modsync(&mut self) -> MODSYNC_W {
        MODSYNC_W { w: self }
    }
    #[doc = "Bit 31 - Start Frame Delimiter Selector"]
    #[inline(always)]
    pub fn onebit(&mut self) -> ONEBIT_W {
        ONEBIT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [us_mr_usart_mode](index.html) module"]
pub struct US_MR_USART_MODE_SPEC;
impl crate::RegisterSpec for US_MR_USART_MODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [us_mr_usart_mode::R](R) reader structure"]
impl crate::Readable for US_MR_USART_MODE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [us_mr_usart_mode::W](W) writer structure"]
impl crate::Writable for US_MR_USART_MODE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets US_MR_USART_MODE to value 0"]
impl crate::Resettable for US_MR_USART_MODE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
