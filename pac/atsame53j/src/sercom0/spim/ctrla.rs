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
pub type SWRST_R = crate::BitReader<bool>;
#[doc = "Field `SWRST` writer - Software Reset"]
pub type SWRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRLA_SPEC, bool, O>;
#[doc = "Field `ENABLE` reader - Enable"]
pub type ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `ENABLE` writer - Enable"]
pub type ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRLA_SPEC, bool, O>;
#[doc = "Field `MODE` reader - Operating Mode"]
pub type MODE_R = crate::FieldReader<u8, MODESELECT_A>;
#[doc = "Operating Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MODESELECT_A {
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
impl From<MODESELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: MODESELECT_A) -> Self {
        variant as _
    }
}
impl MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<MODESELECT_A> {
        match self.bits {
            0 => Some(MODESELECT_A::USART_EXT_CLK),
            1 => Some(MODESELECT_A::USART_INT_CLK),
            2 => Some(MODESELECT_A::SPI_SLAVE),
            3 => Some(MODESELECT_A::SPI_MASTER),
            4 => Some(MODESELECT_A::I2C_SLAVE),
            5 => Some(MODESELECT_A::I2C_MASTER),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `USART_EXT_CLK`"]
    #[inline(always)]
    pub fn is_usart_ext_clk(&self) -> bool {
        *self == MODESELECT_A::USART_EXT_CLK
    }
    #[doc = "Checks if the value of the field is `USART_INT_CLK`"]
    #[inline(always)]
    pub fn is_usart_int_clk(&self) -> bool {
        *self == MODESELECT_A::USART_INT_CLK
    }
    #[doc = "Checks if the value of the field is `SPI_SLAVE`"]
    #[inline(always)]
    pub fn is_spi_slave(&self) -> bool {
        *self == MODESELECT_A::SPI_SLAVE
    }
    #[doc = "Checks if the value of the field is `SPI_MASTER`"]
    #[inline(always)]
    pub fn is_spi_master(&self) -> bool {
        *self == MODESELECT_A::SPI_MASTER
    }
    #[doc = "Checks if the value of the field is `I2C_SLAVE`"]
    #[inline(always)]
    pub fn is_i2c_slave(&self) -> bool {
        *self == MODESELECT_A::I2C_SLAVE
    }
    #[doc = "Checks if the value of the field is `I2C_MASTER`"]
    #[inline(always)]
    pub fn is_i2c_master(&self) -> bool {
        *self == MODESELECT_A::I2C_MASTER
    }
}
#[doc = "Field `MODE` writer - Operating Mode"]
pub type MODE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRLA_SPEC, u8, MODESELECT_A, 3, O>;
impl<'a, const O: u8> MODE_W<'a, O> {
    #[doc = "USART with external clock"]
    #[inline(always)]
    pub fn usart_ext_clk(self) -> &'a mut W {
        self.variant(MODESELECT_A::USART_EXT_CLK)
    }
    #[doc = "USART with internal clock"]
    #[inline(always)]
    pub fn usart_int_clk(self) -> &'a mut W {
        self.variant(MODESELECT_A::USART_INT_CLK)
    }
    #[doc = "SPI in slave operation"]
    #[inline(always)]
    pub fn spi_slave(self) -> &'a mut W {
        self.variant(MODESELECT_A::SPI_SLAVE)
    }
    #[doc = "SPI in master operation"]
    #[inline(always)]
    pub fn spi_master(self) -> &'a mut W {
        self.variant(MODESELECT_A::SPI_MASTER)
    }
    #[doc = "I2C slave operation"]
    #[inline(always)]
    pub fn i2c_slave(self) -> &'a mut W {
        self.variant(MODESELECT_A::I2C_SLAVE)
    }
    #[doc = "I2C master operation"]
    #[inline(always)]
    pub fn i2c_master(self) -> &'a mut W {
        self.variant(MODESELECT_A::I2C_MASTER)
    }
}
#[doc = "Field `RUNSTDBY` reader - Run during Standby"]
pub type RUNSTDBY_R = crate::BitReader<bool>;
#[doc = "Field `RUNSTDBY` writer - Run during Standby"]
pub type RUNSTDBY_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRLA_SPEC, bool, O>;
#[doc = "Field `IBON` reader - Immediate Buffer Overflow Notification"]
pub type IBON_R = crate::BitReader<bool>;
#[doc = "Field `IBON` writer - Immediate Buffer Overflow Notification"]
pub type IBON_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRLA_SPEC, bool, O>;
#[doc = "Field `DOPO` reader - Data Out Pinout"]
pub type DOPO_R = crate::FieldReader<u8, DOPOSELECT_A>;
#[doc = "Data Out Pinout\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DOPOSELECT_A {
    #[doc = "0: DO on PAD\\[0\\], SCK on PAD\\[1\\]
and SS on PAD\\[2\\]"]
    PAD0 = 0,
    #[doc = "2: DO on PAD\\[3\\], SCK on PAD\\[1\\]
and SS on PAD\\[2\\]"]
    PAD2 = 2,
}
impl From<DOPOSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: DOPOSELECT_A) -> Self {
        variant as _
    }
}
impl DOPO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DOPOSELECT_A> {
        match self.bits {
            0 => Some(DOPOSELECT_A::PAD0),
            2 => Some(DOPOSELECT_A::PAD2),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `PAD0`"]
    #[inline(always)]
    pub fn is_pad0(&self) -> bool {
        *self == DOPOSELECT_A::PAD0
    }
    #[doc = "Checks if the value of the field is `PAD2`"]
    #[inline(always)]
    pub fn is_pad2(&self) -> bool {
        *self == DOPOSELECT_A::PAD2
    }
}
#[doc = "Field `DOPO` writer - Data Out Pinout"]
pub type DOPO_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRLA_SPEC, u8, DOPOSELECT_A, 2, O>;
impl<'a, const O: u8> DOPO_W<'a, O> {
    #[doc = "DO on PAD\\[0\\], SCK on PAD\\[1\\]
and SS on PAD\\[2\\]"]
    #[inline(always)]
    pub fn pad0(self) -> &'a mut W {
        self.variant(DOPOSELECT_A::PAD0)
    }
    #[doc = "DO on PAD\\[3\\], SCK on PAD\\[1\\]
and SS on PAD\\[2\\]"]
    #[inline(always)]
    pub fn pad2(self) -> &'a mut W {
        self.variant(DOPOSELECT_A::PAD2)
    }
}
#[doc = "Field `DIPO` reader - Data In Pinout"]
pub type DIPO_R = crate::FieldReader<u8, DIPOSELECT_A>;
#[doc = "Data In Pinout\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DIPOSELECT_A {
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
impl From<DIPOSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: DIPOSELECT_A) -> Self {
        variant as _
    }
}
impl DIPO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIPOSELECT_A {
        match self.bits {
            0 => DIPOSELECT_A::PAD0,
            1 => DIPOSELECT_A::PAD1,
            2 => DIPOSELECT_A::PAD2,
            3 => DIPOSELECT_A::PAD3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PAD0`"]
    #[inline(always)]
    pub fn is_pad0(&self) -> bool {
        *self == DIPOSELECT_A::PAD0
    }
    #[doc = "Checks if the value of the field is `PAD1`"]
    #[inline(always)]
    pub fn is_pad1(&self) -> bool {
        *self == DIPOSELECT_A::PAD1
    }
    #[doc = "Checks if the value of the field is `PAD2`"]
    #[inline(always)]
    pub fn is_pad2(&self) -> bool {
        *self == DIPOSELECT_A::PAD2
    }
    #[doc = "Checks if the value of the field is `PAD3`"]
    #[inline(always)]
    pub fn is_pad3(&self) -> bool {
        *self == DIPOSELECT_A::PAD3
    }
}
#[doc = "Field `DIPO` writer - Data In Pinout"]
pub type DIPO_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, CTRLA_SPEC, u8, DIPOSELECT_A, 2, O>;
impl<'a, const O: u8> DIPO_W<'a, O> {
    #[doc = "SERCOM PAD\\[0\\]
is used as data input"]
    #[inline(always)]
    pub fn pad0(self) -> &'a mut W {
        self.variant(DIPOSELECT_A::PAD0)
    }
    #[doc = "SERCOM PAD\\[1\\]
is used as data input"]
    #[inline(always)]
    pub fn pad1(self) -> &'a mut W {
        self.variant(DIPOSELECT_A::PAD1)
    }
    #[doc = "SERCOM PAD\\[2\\]
is used as data input"]
    #[inline(always)]
    pub fn pad2(self) -> &'a mut W {
        self.variant(DIPOSELECT_A::PAD2)
    }
    #[doc = "SERCOM PAD\\[3\\]
is used as data input"]
    #[inline(always)]
    pub fn pad3(self) -> &'a mut W {
        self.variant(DIPOSELECT_A::PAD3)
    }
}
#[doc = "Field `FORM` reader - Frame Format"]
pub type FORM_R = crate::FieldReader<u8, FORMSELECT_A>;
#[doc = "Frame Format\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FORMSELECT_A {
    #[doc = "0: SPI Frame"]
    SPI_FRAME = 0,
    #[doc = "2: SPI Frame with Addr"]
    SPI_FRAME_WITH_ADDR = 2,
}
impl From<FORMSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: FORMSELECT_A) -> Self {
        variant as _
    }
}
impl FORM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<FORMSELECT_A> {
        match self.bits {
            0 => Some(FORMSELECT_A::SPI_FRAME),
            2 => Some(FORMSELECT_A::SPI_FRAME_WITH_ADDR),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SPI_FRAME`"]
    #[inline(always)]
    pub fn is_spi_frame(&self) -> bool {
        *self == FORMSELECT_A::SPI_FRAME
    }
    #[doc = "Checks if the value of the field is `SPI_FRAME_WITH_ADDR`"]
    #[inline(always)]
    pub fn is_spi_frame_with_addr(&self) -> bool {
        *self == FORMSELECT_A::SPI_FRAME_WITH_ADDR
    }
}
#[doc = "Field `FORM` writer - Frame Format"]
pub type FORM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRLA_SPEC, u8, FORMSELECT_A, 4, O>;
impl<'a, const O: u8> FORM_W<'a, O> {
    #[doc = "SPI Frame"]
    #[inline(always)]
    pub fn spi_frame(self) -> &'a mut W {
        self.variant(FORMSELECT_A::SPI_FRAME)
    }
    #[doc = "SPI Frame with Addr"]
    #[inline(always)]
    pub fn spi_frame_with_addr(self) -> &'a mut W {
        self.variant(FORMSELECT_A::SPI_FRAME_WITH_ADDR)
    }
}
#[doc = "Field `CPHA` reader - Clock Phase"]
pub type CPHA_R = crate::BitReader<CPHASELECT_A>;
#[doc = "Clock Phase\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CPHASELECT_A {
    #[doc = "0: The data is sampled on a leading SCK edge and changed on a trailing SCK edge"]
    LEADING_EDGE = 0,
    #[doc = "1: The data is sampled on a trailing SCK edge and changed on a leading SCK edge"]
    TRAILING_EDGE = 1,
}
impl From<CPHASELECT_A> for bool {
    #[inline(always)]
    fn from(variant: CPHASELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl CPHA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CPHASELECT_A {
        match self.bits {
            false => CPHASELECT_A::LEADING_EDGE,
            true => CPHASELECT_A::TRAILING_EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `LEADING_EDGE`"]
    #[inline(always)]
    pub fn is_leading_edge(&self) -> bool {
        *self == CPHASELECT_A::LEADING_EDGE
    }
    #[doc = "Checks if the value of the field is `TRAILING_EDGE`"]
    #[inline(always)]
    pub fn is_trailing_edge(&self) -> bool {
        *self == CPHASELECT_A::TRAILING_EDGE
    }
}
#[doc = "Field `CPHA` writer - Clock Phase"]
pub type CPHA_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRLA_SPEC, CPHASELECT_A, O>;
impl<'a, const O: u8> CPHA_W<'a, O> {
    #[doc = "The data is sampled on a leading SCK edge and changed on a trailing SCK edge"]
    #[inline(always)]
    pub fn leading_edge(self) -> &'a mut W {
        self.variant(CPHASELECT_A::LEADING_EDGE)
    }
    #[doc = "The data is sampled on a trailing SCK edge and changed on a leading SCK edge"]
    #[inline(always)]
    pub fn trailing_edge(self) -> &'a mut W {
        self.variant(CPHASELECT_A::TRAILING_EDGE)
    }
}
#[doc = "Field `CPOL` reader - Clock Polarity"]
pub type CPOL_R = crate::BitReader<CPOLSELECT_A>;
#[doc = "Clock Polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CPOLSELECT_A {
    #[doc = "0: SCK is low when idle"]
    IDLE_LOW = 0,
    #[doc = "1: SCK is high when idle"]
    IDLE_HIGH = 1,
}
impl From<CPOLSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: CPOLSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl CPOL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CPOLSELECT_A {
        match self.bits {
            false => CPOLSELECT_A::IDLE_LOW,
            true => CPOLSELECT_A::IDLE_HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `IDLE_LOW`"]
    #[inline(always)]
    pub fn is_idle_low(&self) -> bool {
        *self == CPOLSELECT_A::IDLE_LOW
    }
    #[doc = "Checks if the value of the field is `IDLE_HIGH`"]
    #[inline(always)]
    pub fn is_idle_high(&self) -> bool {
        *self == CPOLSELECT_A::IDLE_HIGH
    }
}
#[doc = "Field `CPOL` writer - Clock Polarity"]
pub type CPOL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRLA_SPEC, CPOLSELECT_A, O>;
impl<'a, const O: u8> CPOL_W<'a, O> {
    #[doc = "SCK is low when idle"]
    #[inline(always)]
    pub fn idle_low(self) -> &'a mut W {
        self.variant(CPOLSELECT_A::IDLE_LOW)
    }
    #[doc = "SCK is high when idle"]
    #[inline(always)]
    pub fn idle_high(self) -> &'a mut W {
        self.variant(CPOLSELECT_A::IDLE_HIGH)
    }
}
#[doc = "Field `DORD` reader - Data Order"]
pub type DORD_R = crate::BitReader<DORDSELECT_A>;
#[doc = "Data Order\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DORDSELECT_A {
    #[doc = "0: MSB is transferred first"]
    MSB = 0,
    #[doc = "1: LSB is transferred first"]
    LSB = 1,
}
impl From<DORDSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: DORDSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl DORD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DORDSELECT_A {
        match self.bits {
            false => DORDSELECT_A::MSB,
            true => DORDSELECT_A::LSB,
        }
    }
    #[doc = "Checks if the value of the field is `MSB`"]
    #[inline(always)]
    pub fn is_msb(&self) -> bool {
        *self == DORDSELECT_A::MSB
    }
    #[doc = "Checks if the value of the field is `LSB`"]
    #[inline(always)]
    pub fn is_lsb(&self) -> bool {
        *self == DORDSELECT_A::LSB
    }
}
#[doc = "Field `DORD` writer - Data Order"]
pub type DORD_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRLA_SPEC, DORDSELECT_A, O>;
impl<'a, const O: u8> DORD_W<'a, O> {
    #[doc = "MSB is transferred first"]
    #[inline(always)]
    pub fn msb(self) -> &'a mut W {
        self.variant(DORDSELECT_A::MSB)
    }
    #[doc = "LSB is transferred first"]
    #[inline(always)]
    pub fn lsb(self) -> &'a mut W {
        self.variant(DORDSELECT_A::LSB)
    }
}
impl R {
    #[doc = "Bit 0 - Software Reset"]
    #[inline(always)]
    pub fn swrst(&self) -> SWRST_R {
        SWRST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:4 - Operating Mode"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 2) & 7) as u8)
    }
    #[doc = "Bit 7 - Run during Standby"]
    #[inline(always)]
    pub fn runstdby(&self) -> RUNSTDBY_R {
        RUNSTDBY_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Immediate Buffer Overflow Notification"]
    #[inline(always)]
    pub fn ibon(&self) -> IBON_R {
        IBON_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 16:17 - Data Out Pinout"]
    #[inline(always)]
    pub fn dopo(&self) -> DOPO_R {
        DOPO_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 20:21 - Data In Pinout"]
    #[inline(always)]
    pub fn dipo(&self) -> DIPO_R {
        DIPO_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 24:27 - Frame Format"]
    #[inline(always)]
    pub fn form(&self) -> FORM_R {
        FORM_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 28 - Clock Phase"]
    #[inline(always)]
    pub fn cpha(&self) -> CPHA_R {
        CPHA_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Clock Polarity"]
    #[inline(always)]
    pub fn cpol(&self) -> CPOL_R {
        CPOL_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Data Order"]
    #[inline(always)]
    pub fn dord(&self) -> DORD_R {
        DORD_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Software Reset"]
    #[inline(always)]
    #[must_use]
    pub fn swrst(&mut self) -> SWRST_W<0> {
        SWRST_W::new(self)
    }
    #[doc = "Bit 1 - Enable"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> ENABLE_W<1> {
        ENABLE_W::new(self)
    }
    #[doc = "Bits 2:4 - Operating Mode"]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> MODE_W<2> {
        MODE_W::new(self)
    }
    #[doc = "Bit 7 - Run during Standby"]
    #[inline(always)]
    #[must_use]
    pub fn runstdby(&mut self) -> RUNSTDBY_W<7> {
        RUNSTDBY_W::new(self)
    }
    #[doc = "Bit 8 - Immediate Buffer Overflow Notification"]
    #[inline(always)]
    #[must_use]
    pub fn ibon(&mut self) -> IBON_W<8> {
        IBON_W::new(self)
    }
    #[doc = "Bits 16:17 - Data Out Pinout"]
    #[inline(always)]
    #[must_use]
    pub fn dopo(&mut self) -> DOPO_W<16> {
        DOPO_W::new(self)
    }
    #[doc = "Bits 20:21 - Data In Pinout"]
    #[inline(always)]
    #[must_use]
    pub fn dipo(&mut self) -> DIPO_W<20> {
        DIPO_W::new(self)
    }
    #[doc = "Bits 24:27 - Frame Format"]
    #[inline(always)]
    #[must_use]
    pub fn form(&mut self) -> FORM_W<24> {
        FORM_W::new(self)
    }
    #[doc = "Bit 28 - Clock Phase"]
    #[inline(always)]
    #[must_use]
    pub fn cpha(&mut self) -> CPHA_W<28> {
        CPHA_W::new(self)
    }
    #[doc = "Bit 29 - Clock Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn cpol(&mut self) -> CPOL_W<29> {
        CPOL_W::new(self)
    }
    #[doc = "Bit 30 - Data Order"]
    #[inline(always)]
    #[must_use]
    pub fn dord(&mut self) -> DORD_W<30> {
        DORD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPIM Control A\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrla](index.html) module"]
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRLA to value 0"]
impl crate::Resettable for CTRLA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
