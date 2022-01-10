#[doc = "Register `CTRLA` reader"]
pub struct R(crate::R<CTRLA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRLA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRLA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRLA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRLA` writer"]
pub struct W(crate::W<CTRLA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRLA_SPEC>;
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
impl From<crate::W<CTRLA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRLA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SWRST` reader - Software Reset"]
pub struct SWRST_R(crate::FieldReader<bool, bool>);
impl SWRST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SWRST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWRST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWRST` writer - Software Reset"]
pub struct SWRST_W<'a> {
    w: &'a mut W,
}
impl<'a> SWRST_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Field `ENABLE` reader - Enable"]
pub struct ENABLE_R(crate::FieldReader<bool, bool>);
impl ENABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ENABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENABLE` writer - Enable"]
pub struct ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Operating Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MODE_A {
    #[doc = "0: USART with external clock"]
    USART_EXT_CLK = 0,
    #[doc = "1: USART with internal clock"]
    USART_INT_CLK = 1,
    #[doc = "2: SPI in slave operation"]
    SPI_SLAVE = 2,
    #[doc = "3: SPI in master operation"]
    SPI_MASTER = 3,
    #[doc = "4: I2C slave operation"]
    I2C_SLAVE = 4,
    #[doc = "5: I2C master operation"]
    I2C_MASTER = 5,
}
impl From<MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `MODE` reader - Operating Mode"]
pub struct MODE_R(crate::FieldReader<u8, MODE_A>);
impl MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        MODE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<MODE_A> {
        match self.bits {
            0 => Some(MODE_A::USART_EXT_CLK),
            1 => Some(MODE_A::USART_INT_CLK),
            2 => Some(MODE_A::SPI_SLAVE),
            3 => Some(MODE_A::SPI_MASTER),
            4 => Some(MODE_A::I2C_SLAVE),
            5 => Some(MODE_A::I2C_MASTER),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `USART_EXT_CLK`"]
    #[inline(always)]
    pub fn is_usart_ext_clk(&self) -> bool {
        **self == MODE_A::USART_EXT_CLK
    }
    #[doc = "Checks if the value of the field is `USART_INT_CLK`"]
    #[inline(always)]
    pub fn is_usart_int_clk(&self) -> bool {
        **self == MODE_A::USART_INT_CLK
    }
    #[doc = "Checks if the value of the field is `SPI_SLAVE`"]
    #[inline(always)]
    pub fn is_spi_slave(&self) -> bool {
        **self == MODE_A::SPI_SLAVE
    }
    #[doc = "Checks if the value of the field is `SPI_MASTER`"]
    #[inline(always)]
    pub fn is_spi_master(&self) -> bool {
        **self == MODE_A::SPI_MASTER
    }
    #[doc = "Checks if the value of the field is `I2C_SLAVE`"]
    #[inline(always)]
    pub fn is_i2c_slave(&self) -> bool {
        **self == MODE_A::I2C_SLAVE
    }
    #[doc = "Checks if the value of the field is `I2C_MASTER`"]
    #[inline(always)]
    pub fn is_i2c_master(&self) -> bool {
        **self == MODE_A::I2C_MASTER
    }
}
impl core::ops::Deref for MODE_R {
    type Target = crate::FieldReader<u8, MODE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MODE` writer - Operating Mode"]
pub struct MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MODE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "USART with external clock"]
    #[inline(always)]
    pub fn usart_ext_clk(self) -> &'a mut W {
        self.variant(MODE_A::USART_EXT_CLK)
    }
    #[doc = "USART with internal clock"]
    #[inline(always)]
    pub fn usart_int_clk(self) -> &'a mut W {
        self.variant(MODE_A::USART_INT_CLK)
    }
    #[doc = "SPI in slave operation"]
    #[inline(always)]
    pub fn spi_slave(self) -> &'a mut W {
        self.variant(MODE_A::SPI_SLAVE)
    }
    #[doc = "SPI in master operation"]
    #[inline(always)]
    pub fn spi_master(self) -> &'a mut W {
        self.variant(MODE_A::SPI_MASTER)
    }
    #[doc = "I2C slave operation"]
    #[inline(always)]
    pub fn i2c_slave(self) -> &'a mut W {
        self.variant(MODE_A::I2C_SLAVE)
    }
    #[doc = "I2C master operation"]
    #[inline(always)]
    pub fn i2c_master(self) -> &'a mut W {
        self.variant(MODE_A::I2C_MASTER)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 2)) | ((value as u32 & 0x07) << 2);
        self.w
    }
}
#[doc = "Field `RUNSTDBY` reader - Run during Standby"]
pub struct RUNSTDBY_R(crate::FieldReader<bool, bool>);
impl RUNSTDBY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RUNSTDBY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RUNSTDBY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RUNSTDBY` writer - Run during Standby"]
pub struct RUNSTDBY_W<'a> {
    w: &'a mut W,
}
impl<'a> RUNSTDBY_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Field `IBON` reader - Immediate Buffer Overflow Notification"]
pub struct IBON_R(crate::FieldReader<bool, bool>);
impl IBON_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IBON_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IBON_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IBON` writer - Immediate Buffer Overflow Notification"]
pub struct IBON_W<'a> {
    w: &'a mut W,
}
impl<'a> IBON_W<'a> {
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
#[doc = "Data Out Pinout\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DOPO_A {
    #[doc = "0: DO on PAD\\[0\\], SCK on PAD\\[1\\]
and SS on PAD\\[2\\]"]
    PAD0 = 0,
    #[doc = "2: DO on PAD\\[3\\], SCK on PAD\\[1\\]
and SS on PAD\\[2\\]"]
    PAD2 = 2,
}
impl From<DOPO_A> for u8 {
    #[inline(always)]
    fn from(variant: DOPO_A) -> Self {
        variant as _
    }
}
#[doc = "Field `DOPO` reader - Data Out Pinout"]
pub struct DOPO_R(crate::FieldReader<u8, DOPO_A>);
impl DOPO_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DOPO_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DOPO_A> {
        match self.bits {
            0 => Some(DOPO_A::PAD0),
            2 => Some(DOPO_A::PAD2),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `PAD0`"]
    #[inline(always)]
    pub fn is_pad0(&self) -> bool {
        **self == DOPO_A::PAD0
    }
    #[doc = "Checks if the value of the field is `PAD2`"]
    #[inline(always)]
    pub fn is_pad2(&self) -> bool {
        **self == DOPO_A::PAD2
    }
}
impl core::ops::Deref for DOPO_R {
    type Target = crate::FieldReader<u8, DOPO_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DOPO` writer - Data Out Pinout"]
pub struct DOPO_W<'a> {
    w: &'a mut W,
}
impl<'a> DOPO_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DOPO_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "DO on PAD\\[0\\], SCK on PAD\\[1\\]
and SS on PAD\\[2\\]"]
    #[inline(always)]
    pub fn pad0(self) -> &'a mut W {
        self.variant(DOPO_A::PAD0)
    }
    #[doc = "DO on PAD\\[3\\], SCK on PAD\\[1\\]
and SS on PAD\\[2\\]"]
    #[inline(always)]
    pub fn pad2(self) -> &'a mut W {
        self.variant(DOPO_A::PAD2)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | ((value as u32 & 0x03) << 16);
        self.w
    }
}
#[doc = "Data In Pinout\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DIPO_A {
    #[doc = "0: SERCOM PAD\\[0\\]
is used as data input"]
    PAD0 = 0,
    #[doc = "1: SERCOM PAD\\[1\\]
is used as data input"]
    PAD1 = 1,
    #[doc = "2: SERCOM PAD\\[2\\]
is used as data input"]
    PAD2 = 2,
    #[doc = "3: SERCOM PAD\\[3\\]
is used as data input"]
    PAD3 = 3,
}
impl From<DIPO_A> for u8 {
    #[inline(always)]
    fn from(variant: DIPO_A) -> Self {
        variant as _
    }
}
#[doc = "Field `DIPO` reader - Data In Pinout"]
pub struct DIPO_R(crate::FieldReader<u8, DIPO_A>);
impl DIPO_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DIPO_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIPO_A {
        match self.bits {
            0 => DIPO_A::PAD0,
            1 => DIPO_A::PAD1,
            2 => DIPO_A::PAD2,
            3 => DIPO_A::PAD3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PAD0`"]
    #[inline(always)]
    pub fn is_pad0(&self) -> bool {
        **self == DIPO_A::PAD0
    }
    #[doc = "Checks if the value of the field is `PAD1`"]
    #[inline(always)]
    pub fn is_pad1(&self) -> bool {
        **self == DIPO_A::PAD1
    }
    #[doc = "Checks if the value of the field is `PAD2`"]
    #[inline(always)]
    pub fn is_pad2(&self) -> bool {
        **self == DIPO_A::PAD2
    }
    #[doc = "Checks if the value of the field is `PAD3`"]
    #[inline(always)]
    pub fn is_pad3(&self) -> bool {
        **self == DIPO_A::PAD3
    }
}
impl core::ops::Deref for DIPO_R {
    type Target = crate::FieldReader<u8, DIPO_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIPO` writer - Data In Pinout"]
pub struct DIPO_W<'a> {
    w: &'a mut W,
}
impl<'a> DIPO_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIPO_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "SERCOM PAD\\[0\\]
is used as data input"]
    #[inline(always)]
    pub fn pad0(self) -> &'a mut W {
        self.variant(DIPO_A::PAD0)
    }
    #[doc = "SERCOM PAD\\[1\\]
is used as data input"]
    #[inline(always)]
    pub fn pad1(self) -> &'a mut W {
        self.variant(DIPO_A::PAD1)
    }
    #[doc = "SERCOM PAD\\[2\\]
is used as data input"]
    #[inline(always)]
    pub fn pad2(self) -> &'a mut W {
        self.variant(DIPO_A::PAD2)
    }
    #[doc = "SERCOM PAD\\[3\\]
is used as data input"]
    #[inline(always)]
    pub fn pad3(self) -> &'a mut W {
        self.variant(DIPO_A::PAD3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | ((value as u32 & 0x03) << 20);
        self.w
    }
}
#[doc = "Frame Format\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FORM_A {
    #[doc = "0: SPI Frame"]
    SPI_FRAME = 0,
    #[doc = "2: SPI Frame with Addr"]
    SPI_FRAME_WITH_ADDR = 2,
}
impl From<FORM_A> for u8 {
    #[inline(always)]
    fn from(variant: FORM_A) -> Self {
        variant as _
    }
}
#[doc = "Field `FORM` reader - Frame Format"]
pub struct FORM_R(crate::FieldReader<u8, FORM_A>);
impl FORM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        FORM_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<FORM_A> {
        match self.bits {
            0 => Some(FORM_A::SPI_FRAME),
            2 => Some(FORM_A::SPI_FRAME_WITH_ADDR),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SPI_FRAME`"]
    #[inline(always)]
    pub fn is_spi_frame(&self) -> bool {
        **self == FORM_A::SPI_FRAME
    }
    #[doc = "Checks if the value of the field is `SPI_FRAME_WITH_ADDR`"]
    #[inline(always)]
    pub fn is_spi_frame_with_addr(&self) -> bool {
        **self == FORM_A::SPI_FRAME_WITH_ADDR
    }
}
impl core::ops::Deref for FORM_R {
    type Target = crate::FieldReader<u8, FORM_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FORM` writer - Frame Format"]
pub struct FORM_W<'a> {
    w: &'a mut W,
}
impl<'a> FORM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FORM_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "SPI Frame"]
    #[inline(always)]
    pub fn spi_frame(self) -> &'a mut W {
        self.variant(FORM_A::SPI_FRAME)
    }
    #[doc = "SPI Frame with Addr"]
    #[inline(always)]
    pub fn spi_frame_with_addr(self) -> &'a mut W {
        self.variant(FORM_A::SPI_FRAME_WITH_ADDR)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | ((value as u32 & 0x0f) << 24);
        self.w
    }
}
#[doc = "Clock Phase\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPHA_A {
    #[doc = "0: The data is sampled on a leading SCK edge and changed on a trailing SCK edge"]
    LEADING_EDGE = 0,
    #[doc = "1: The data is sampled on a trailing SCK edge and changed on a leading SCK edge"]
    TRAILING_EDGE = 1,
}
impl From<CPHA_A> for bool {
    #[inline(always)]
    fn from(variant: CPHA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CPHA` reader - Clock Phase"]
pub struct CPHA_R(crate::FieldReader<bool, CPHA_A>);
impl CPHA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CPHA_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CPHA_A {
        match self.bits {
            false => CPHA_A::LEADING_EDGE,
            true => CPHA_A::TRAILING_EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `LEADING_EDGE`"]
    #[inline(always)]
    pub fn is_leading_edge(&self) -> bool {
        **self == CPHA_A::LEADING_EDGE
    }
    #[doc = "Checks if the value of the field is `TRAILING_EDGE`"]
    #[inline(always)]
    pub fn is_trailing_edge(&self) -> bool {
        **self == CPHA_A::TRAILING_EDGE
    }
}
impl core::ops::Deref for CPHA_R {
    type Target = crate::FieldReader<bool, CPHA_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CPHA` writer - Clock Phase"]
pub struct CPHA_W<'a> {
    w: &'a mut W,
}
impl<'a> CPHA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CPHA_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The data is sampled on a leading SCK edge and changed on a trailing SCK edge"]
    #[inline(always)]
    pub fn leading_edge(self) -> &'a mut W {
        self.variant(CPHA_A::LEADING_EDGE)
    }
    #[doc = "The data is sampled on a trailing SCK edge and changed on a leading SCK edge"]
    #[inline(always)]
    pub fn trailing_edge(self) -> &'a mut W {
        self.variant(CPHA_A::TRAILING_EDGE)
    }
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
#[doc = "Clock Polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPOL_A {
    #[doc = "0: SCK is low when idle"]
    IDLE_LOW = 0,
    #[doc = "1: SCK is high when idle"]
    IDLE_HIGH = 1,
}
impl From<CPOL_A> for bool {
    #[inline(always)]
    fn from(variant: CPOL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CPOL` reader - Clock Polarity"]
pub struct CPOL_R(crate::FieldReader<bool, CPOL_A>);
impl CPOL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CPOL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CPOL_A {
        match self.bits {
            false => CPOL_A::IDLE_LOW,
            true => CPOL_A::IDLE_HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `IDLE_LOW`"]
    #[inline(always)]
    pub fn is_idle_low(&self) -> bool {
        **self == CPOL_A::IDLE_LOW
    }
    #[doc = "Checks if the value of the field is `IDLE_HIGH`"]
    #[inline(always)]
    pub fn is_idle_high(&self) -> bool {
        **self == CPOL_A::IDLE_HIGH
    }
}
impl core::ops::Deref for CPOL_R {
    type Target = crate::FieldReader<bool, CPOL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CPOL` writer - Clock Polarity"]
pub struct CPOL_W<'a> {
    w: &'a mut W,
}
impl<'a> CPOL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CPOL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "SCK is low when idle"]
    #[inline(always)]
    pub fn idle_low(self) -> &'a mut W {
        self.variant(CPOL_A::IDLE_LOW)
    }
    #[doc = "SCK is high when idle"]
    #[inline(always)]
    pub fn idle_high(self) -> &'a mut W {
        self.variant(CPOL_A::IDLE_HIGH)
    }
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
#[doc = "Data Order\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DORD_A {
    #[doc = "0: MSB is transferred first"]
    MSB = 0,
    #[doc = "1: LSB is transferred first"]
    LSB = 1,
}
impl From<DORD_A> for bool {
    #[inline(always)]
    fn from(variant: DORD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DORD` reader - Data Order"]
pub struct DORD_R(crate::FieldReader<bool, DORD_A>);
impl DORD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DORD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DORD_A {
        match self.bits {
            false => DORD_A::MSB,
            true => DORD_A::LSB,
        }
    }
    #[doc = "Checks if the value of the field is `MSB`"]
    #[inline(always)]
    pub fn is_msb(&self) -> bool {
        **self == DORD_A::MSB
    }
    #[doc = "Checks if the value of the field is `LSB`"]
    #[inline(always)]
    pub fn is_lsb(&self) -> bool {
        **self == DORD_A::LSB
    }
}
impl core::ops::Deref for DORD_R {
    type Target = crate::FieldReader<bool, DORD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DORD` writer - Data Order"]
pub struct DORD_W<'a> {
    w: &'a mut W,
}
impl<'a> DORD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DORD_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "MSB is transferred first"]
    #[inline(always)]
    pub fn msb(self) -> &'a mut W {
        self.variant(DORD_A::MSB)
    }
    #[doc = "LSB is transferred first"]
    #[inline(always)]
    pub fn lsb(self) -> &'a mut W {
        self.variant(DORD_A::LSB)
    }
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
impl R {
    #[doc = "Bit 0 - Software Reset"]
    #[inline(always)]
    pub fn swrst(&self) -> SWRST_R {
        SWRST_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Enable"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 2:4 - Operating Mode"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 2) & 0x07) as u8)
    }
    #[doc = "Bit 7 - Run during Standby"]
    #[inline(always)]
    pub fn runstdby(&self) -> RUNSTDBY_R {
        RUNSTDBY_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Immediate Buffer Overflow Notification"]
    #[inline(always)]
    pub fn ibon(&self) -> IBON_R {
        IBON_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 16:17 - Data Out Pinout"]
    #[inline(always)]
    pub fn dopo(&self) -> DOPO_R {
        DOPO_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 20:21 - Data In Pinout"]
    #[inline(always)]
    pub fn dipo(&self) -> DIPO_R {
        DIPO_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bits 24:27 - Frame Format"]
    #[inline(always)]
    pub fn form(&self) -> FORM_R {
        FORM_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 28 - Clock Phase"]
    #[inline(always)]
    pub fn cpha(&self) -> CPHA_R {
        CPHA_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Clock Polarity"]
    #[inline(always)]
    pub fn cpol(&self) -> CPOL_R {
        CPOL_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Data Order"]
    #[inline(always)]
    pub fn dord(&self) -> DORD_R {
        DORD_R::new(((self.bits >> 30) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Software Reset"]
    #[inline(always)]
    pub fn swrst(&mut self) -> SWRST_W {
        SWRST_W { w: self }
    }
    #[doc = "Bit 1 - Enable"]
    #[inline(always)]
    pub fn enable(&mut self) -> ENABLE_W {
        ENABLE_W { w: self }
    }
    #[doc = "Bits 2:4 - Operating Mode"]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W {
        MODE_W { w: self }
    }
    #[doc = "Bit 7 - Run during Standby"]
    #[inline(always)]
    pub fn runstdby(&mut self) -> RUNSTDBY_W {
        RUNSTDBY_W { w: self }
    }
    #[doc = "Bit 8 - Immediate Buffer Overflow Notification"]
    #[inline(always)]
    pub fn ibon(&mut self) -> IBON_W {
        IBON_W { w: self }
    }
    #[doc = "Bits 16:17 - Data Out Pinout"]
    #[inline(always)]
    pub fn dopo(&mut self) -> DOPO_W {
        DOPO_W { w: self }
    }
    #[doc = "Bits 20:21 - Data In Pinout"]
    #[inline(always)]
    pub fn dipo(&mut self) -> DIPO_W {
        DIPO_W { w: self }
    }
    #[doc = "Bits 24:27 - Frame Format"]
    #[inline(always)]
    pub fn form(&mut self) -> FORM_W {
        FORM_W { w: self }
    }
    #[doc = "Bit 28 - Clock Phase"]
    #[inline(always)]
    pub fn cpha(&mut self) -> CPHA_W {
        CPHA_W { w: self }
    }
    #[doc = "Bit 29 - Clock Polarity"]
    #[inline(always)]
    pub fn cpol(&mut self) -> CPOL_W {
        CPOL_W { w: self }
    }
    #[doc = "Bit 30 - Data Order"]
    #[inline(always)]
    pub fn dord(&mut self) -> DORD_W {
        DORD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPIS Control A\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrla](index.html) module"]
pub struct CTRLA_SPEC;
impl crate::RegisterSpec for CTRLA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrla::R](R) reader structure"]
impl crate::Readable for CTRLA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrla::W](W) writer structure"]
impl crate::Writable for CTRLA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTRLA to value 0"]
impl crate::Resettable for CTRLA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
