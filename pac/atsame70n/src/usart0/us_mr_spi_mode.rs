#[doc = "Register `US_MR_SPI_MODE` reader"]
pub struct R(crate::R<US_MR_SPI_MODE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<US_MR_SPI_MODE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<US_MR_SPI_MODE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<US_MR_SPI_MODE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `US_MR_SPI_MODE` writer"]
pub struct W(crate::W<US_MR_SPI_MODE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<US_MR_SPI_MODE_SPEC>;
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
impl From<crate::W<US_MR_SPI_MODE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<US_MR_SPI_MODE_SPEC>) -> Self {
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
#[doc = "Field `CPHA` reader - SPI Clock Phase"]
pub struct CPHA_R(crate::FieldReader<bool, bool>);
impl CPHA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CPHA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CPHA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CPHA` writer - SPI Clock Phase"]
pub struct CPHA_W<'a> {
    w: &'a mut W,
}
impl<'a> CPHA_W<'a> {
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
#[doc = "Field `CPOL` reader - SPI Clock Polarity"]
pub struct CPOL_R(crate::FieldReader<bool, bool>);
impl CPOL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CPOL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CPOL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CPOL` writer - SPI Clock Polarity"]
pub struct CPOL_W<'a> {
    w: &'a mut W,
}
impl<'a> CPOL_W<'a> {
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
#[doc = "Field `WRDBT` reader - Wait Read Data Before Transfer"]
pub struct WRDBT_R(crate::FieldReader<bool, bool>);
impl WRDBT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WRDBT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WRDBT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WRDBT` writer - Wait Read Data Before Transfer"]
pub struct WRDBT_W<'a> {
    w: &'a mut W,
}
impl<'a> WRDBT_W<'a> {
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
    #[doc = "Bit 18 - Clock Output Select"]
    #[inline(always)]
    pub fn clko(&self) -> CLKO_R {
        CLKO_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 8 - SPI Clock Phase"]
    #[inline(always)]
    pub fn cpha(&self) -> CPHA_R {
        CPHA_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 16 - SPI Clock Polarity"]
    #[inline(always)]
    pub fn cpol(&self) -> CPOL_R {
        CPOL_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Wait Read Data Before Transfer"]
    #[inline(always)]
    pub fn wrdbt(&self) -> WRDBT_R {
        WRDBT_R::new(((self.bits >> 20) & 0x01) != 0)
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
    #[doc = "Bit 18 - Clock Output Select"]
    #[inline(always)]
    pub fn clko(&mut self) -> CLKO_W {
        CLKO_W { w: self }
    }
    #[doc = "Bit 8 - SPI Clock Phase"]
    #[inline(always)]
    pub fn cpha(&mut self) -> CPHA_W {
        CPHA_W { w: self }
    }
    #[doc = "Bit 16 - SPI Clock Polarity"]
    #[inline(always)]
    pub fn cpol(&mut self) -> CPOL_W {
        CPOL_W { w: self }
    }
    #[doc = "Bit 20 - Wait Read Data Before Transfer"]
    #[inline(always)]
    pub fn wrdbt(&mut self) -> WRDBT_W {
        WRDBT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [us_mr_spi_mode](index.html) module"]
pub struct US_MR_SPI_MODE_SPEC;
impl crate::RegisterSpec for US_MR_SPI_MODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [us_mr_spi_mode::R](R) reader structure"]
impl crate::Readable for US_MR_SPI_MODE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [us_mr_spi_mode::W](W) writer structure"]
impl crate::Writable for US_MR_SPI_MODE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets US_MR_SPI_MODE to value 0"]
impl crate::Resettable for US_MR_SPI_MODE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
