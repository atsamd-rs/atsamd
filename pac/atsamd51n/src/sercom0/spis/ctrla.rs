#[doc = "Register `CTRLA` reader"]
pub type R = crate::R<CTRLA_SPEC>;
#[doc = "Register `CTRLA` writer"]
pub type W = crate::W<CTRLA_SPEC>;
#[doc = "Field `SWRST` reader - Software Reset"]
pub type SWRST_R = crate::BitReader;
#[doc = "Field `SWRST` writer - Software Reset"]
pub type SWRST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ENABLE` reader - Enable"]
pub type ENABLE_R = crate::BitReader;
#[doc = "Field `ENABLE` writer - Enable"]
pub type ENABLE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MODE` reader - Operating Mode"]
pub type MODE_R = crate::FieldReader<MODESELECT_A>;
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
impl crate::FieldSpec for MODESELECT_A {
    type Ux = u8;
}
impl MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<MODESELECT_A> {
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
    #[doc = "USART with external clock"]
    #[inline(always)]
    pub fn is_usart_ext_clk(&self) -> bool {
        *self == MODESELECT_A::USART_EXT_CLK
    }
    #[doc = "USART with internal clock"]
    #[inline(always)]
    pub fn is_usart_int_clk(&self) -> bool {
        *self == MODESELECT_A::USART_INT_CLK
    }
    #[doc = "SPI in slave operation"]
    #[inline(always)]
    pub fn is_spi_slave(&self) -> bool {
        *self == MODESELECT_A::SPI_SLAVE
    }
    #[doc = "SPI in master operation"]
    #[inline(always)]
    pub fn is_spi_master(&self) -> bool {
        *self == MODESELECT_A::SPI_MASTER
    }
    #[doc = "I2C slave operation"]
    #[inline(always)]
    pub fn is_i2c_slave(&self) -> bool {
        *self == MODESELECT_A::I2C_SLAVE
    }
    #[doc = "I2C master operation"]
    #[inline(always)]
    pub fn is_i2c_master(&self) -> bool {
        *self == MODESELECT_A::I2C_MASTER
    }
}
#[doc = "Field `MODE` writer - Operating Mode"]
pub type MODE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O, MODESELECT_A>;
impl<'a, REG, const O: u8> MODE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "USART with external clock"]
    #[inline(always)]
    pub fn usart_ext_clk(self) -> &'a mut crate::W<REG> {
        self.variant(MODESELECT_A::USART_EXT_CLK)
    }
    #[doc = "USART with internal clock"]
    #[inline(always)]
    pub fn usart_int_clk(self) -> &'a mut crate::W<REG> {
        self.variant(MODESELECT_A::USART_INT_CLK)
    }
    #[doc = "SPI in slave operation"]
    #[inline(always)]
    pub fn spi_slave(self) -> &'a mut crate::W<REG> {
        self.variant(MODESELECT_A::SPI_SLAVE)
    }
    #[doc = "SPI in master operation"]
    #[inline(always)]
    pub fn spi_master(self) -> &'a mut crate::W<REG> {
        self.variant(MODESELECT_A::SPI_MASTER)
    }
    #[doc = "I2C slave operation"]
    #[inline(always)]
    pub fn i2c_slave(self) -> &'a mut crate::W<REG> {
        self.variant(MODESELECT_A::I2C_SLAVE)
    }
    #[doc = "I2C master operation"]
    #[inline(always)]
    pub fn i2c_master(self) -> &'a mut crate::W<REG> {
        self.variant(MODESELECT_A::I2C_MASTER)
    }
}
#[doc = "Field `RUNSTDBY` reader - Run during Standby"]
pub type RUNSTDBY_R = crate::BitReader;
#[doc = "Field `RUNSTDBY` writer - Run during Standby"]
pub type RUNSTDBY_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `IBON` reader - Immediate Buffer Overflow Notification"]
pub type IBON_R = crate::BitReader;
#[doc = "Field `IBON` writer - Immediate Buffer Overflow Notification"]
pub type IBON_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DOPO` reader - Data Out Pinout"]
pub type DOPO_R = crate::FieldReader<DOPOSELECT_A>;
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
impl crate::FieldSpec for DOPOSELECT_A {
    type Ux = u8;
}
impl DOPO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<DOPOSELECT_A> {
        match self.bits {
            0 => Some(DOPOSELECT_A::PAD0),
            2 => Some(DOPOSELECT_A::PAD2),
            _ => None,
        }
    }
    #[doc = "DO on PAD\\[0\\], SCK on PAD\\[1\\]
and SS on PAD\\[2\\]"]
    #[inline(always)]
    pub fn is_pad0(&self) -> bool {
        *self == DOPOSELECT_A::PAD0
    }
    #[doc = "DO on PAD\\[3\\], SCK on PAD\\[1\\]
and SS on PAD\\[2\\]"]
    #[inline(always)]
    pub fn is_pad2(&self) -> bool {
        *self == DOPOSELECT_A::PAD2
    }
}
#[doc = "Field `DOPO` writer - Data Out Pinout"]
pub type DOPO_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O, DOPOSELECT_A>;
impl<'a, REG, const O: u8> DOPO_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "DO on PAD\\[0\\], SCK on PAD\\[1\\]
and SS on PAD\\[2\\]"]
    #[inline(always)]
    pub fn pad0(self) -> &'a mut crate::W<REG> {
        self.variant(DOPOSELECT_A::PAD0)
    }
    #[doc = "DO on PAD\\[3\\], SCK on PAD\\[1\\]
and SS on PAD\\[2\\]"]
    #[inline(always)]
    pub fn pad2(self) -> &'a mut crate::W<REG> {
        self.variant(DOPOSELECT_A::PAD2)
    }
}
#[doc = "Field `DIPO` reader - Data In Pinout"]
pub type DIPO_R = crate::FieldReader<DIPOSELECT_A>;
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
impl crate::FieldSpec for DIPOSELECT_A {
    type Ux = u8;
}
impl DIPO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DIPOSELECT_A {
        match self.bits {
            0 => DIPOSELECT_A::PAD0,
            1 => DIPOSELECT_A::PAD1,
            2 => DIPOSELECT_A::PAD2,
            3 => DIPOSELECT_A::PAD3,
            _ => unreachable!(),
        }
    }
    #[doc = "SERCOM PAD\\[0\\]
is used as data input"]
    #[inline(always)]
    pub fn is_pad0(&self) -> bool {
        *self == DIPOSELECT_A::PAD0
    }
    #[doc = "SERCOM PAD\\[1\\]
is used as data input"]
    #[inline(always)]
    pub fn is_pad1(&self) -> bool {
        *self == DIPOSELECT_A::PAD1
    }
    #[doc = "SERCOM PAD\\[2\\]
is used as data input"]
    #[inline(always)]
    pub fn is_pad2(&self) -> bool {
        *self == DIPOSELECT_A::PAD2
    }
    #[doc = "SERCOM PAD\\[3\\]
is used as data input"]
    #[inline(always)]
    pub fn is_pad3(&self) -> bool {
        *self == DIPOSELECT_A::PAD3
    }
}
#[doc = "Field `DIPO` writer - Data In Pinout"]
pub type DIPO_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, DIPOSELECT_A>;
impl<'a, REG, const O: u8> DIPO_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "SERCOM PAD\\[0\\]
is used as data input"]
    #[inline(always)]
    pub fn pad0(self) -> &'a mut crate::W<REG> {
        self.variant(DIPOSELECT_A::PAD0)
    }
    #[doc = "SERCOM PAD\\[1\\]
is used as data input"]
    #[inline(always)]
    pub fn pad1(self) -> &'a mut crate::W<REG> {
        self.variant(DIPOSELECT_A::PAD1)
    }
    #[doc = "SERCOM PAD\\[2\\]
is used as data input"]
    #[inline(always)]
    pub fn pad2(self) -> &'a mut crate::W<REG> {
        self.variant(DIPOSELECT_A::PAD2)
    }
    #[doc = "SERCOM PAD\\[3\\]
is used as data input"]
    #[inline(always)]
    pub fn pad3(self) -> &'a mut crate::W<REG> {
        self.variant(DIPOSELECT_A::PAD3)
    }
}
#[doc = "Field `FORM` reader - Frame Format"]
pub type FORM_R = crate::FieldReader<FORMSELECT_A>;
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
impl crate::FieldSpec for FORMSELECT_A {
    type Ux = u8;
}
impl FORM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<FORMSELECT_A> {
        match self.bits {
            0 => Some(FORMSELECT_A::SPI_FRAME),
            2 => Some(FORMSELECT_A::SPI_FRAME_WITH_ADDR),
            _ => None,
        }
    }
    #[doc = "SPI Frame"]
    #[inline(always)]
    pub fn is_spi_frame(&self) -> bool {
        *self == FORMSELECT_A::SPI_FRAME
    }
    #[doc = "SPI Frame with Addr"]
    #[inline(always)]
    pub fn is_spi_frame_with_addr(&self) -> bool {
        *self == FORMSELECT_A::SPI_FRAME_WITH_ADDR
    }
}
#[doc = "Field `FORM` writer - Frame Format"]
pub type FORM_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O, FORMSELECT_A>;
impl<'a, REG, const O: u8> FORM_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "SPI Frame"]
    #[inline(always)]
    pub fn spi_frame(self) -> &'a mut crate::W<REG> {
        self.variant(FORMSELECT_A::SPI_FRAME)
    }
    #[doc = "SPI Frame with Addr"]
    #[inline(always)]
    pub fn spi_frame_with_addr(self) -> &'a mut crate::W<REG> {
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
    pub const fn variant(&self) -> CPHASELECT_A {
        match self.bits {
            false => CPHASELECT_A::LEADING_EDGE,
            true => CPHASELECT_A::TRAILING_EDGE,
        }
    }
    #[doc = "The data is sampled on a leading SCK edge and changed on a trailing SCK edge"]
    #[inline(always)]
    pub fn is_leading_edge(&self) -> bool {
        *self == CPHASELECT_A::LEADING_EDGE
    }
    #[doc = "The data is sampled on a trailing SCK edge and changed on a leading SCK edge"]
    #[inline(always)]
    pub fn is_trailing_edge(&self) -> bool {
        *self == CPHASELECT_A::TRAILING_EDGE
    }
}
#[doc = "Field `CPHA` writer - Clock Phase"]
pub type CPHA_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CPHASELECT_A>;
impl<'a, REG, const O: u8> CPHA_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The data is sampled on a leading SCK edge and changed on a trailing SCK edge"]
    #[inline(always)]
    pub fn leading_edge(self) -> &'a mut crate::W<REG> {
        self.variant(CPHASELECT_A::LEADING_EDGE)
    }
    #[doc = "The data is sampled on a trailing SCK edge and changed on a leading SCK edge"]
    #[inline(always)]
    pub fn trailing_edge(self) -> &'a mut crate::W<REG> {
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
    pub const fn variant(&self) -> CPOLSELECT_A {
        match self.bits {
            false => CPOLSELECT_A::IDLE_LOW,
            true => CPOLSELECT_A::IDLE_HIGH,
        }
    }
    #[doc = "SCK is low when idle"]
    #[inline(always)]
    pub fn is_idle_low(&self) -> bool {
        *self == CPOLSELECT_A::IDLE_LOW
    }
    #[doc = "SCK is high when idle"]
    #[inline(always)]
    pub fn is_idle_high(&self) -> bool {
        *self == CPOLSELECT_A::IDLE_HIGH
    }
}
#[doc = "Field `CPOL` writer - Clock Polarity"]
pub type CPOL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CPOLSELECT_A>;
impl<'a, REG, const O: u8> CPOL_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SCK is low when idle"]
    #[inline(always)]
    pub fn idle_low(self) -> &'a mut crate::W<REG> {
        self.variant(CPOLSELECT_A::IDLE_LOW)
    }
    #[doc = "SCK is high when idle"]
    #[inline(always)]
    pub fn idle_high(self) -> &'a mut crate::W<REG> {
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
    pub const fn variant(&self) -> DORDSELECT_A {
        match self.bits {
            false => DORDSELECT_A::MSB,
            true => DORDSELECT_A::LSB,
        }
    }
    #[doc = "MSB is transferred first"]
    #[inline(always)]
    pub fn is_msb(&self) -> bool {
        *self == DORDSELECT_A::MSB
    }
    #[doc = "LSB is transferred first"]
    #[inline(always)]
    pub fn is_lsb(&self) -> bool {
        *self == DORDSELECT_A::LSB
    }
}
#[doc = "Field `DORD` writer - Data Order"]
pub type DORD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, DORDSELECT_A>;
impl<'a, REG, const O: u8> DORD_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "MSB is transferred first"]
    #[inline(always)]
    pub fn msb(self) -> &'a mut crate::W<REG> {
        self.variant(DORDSELECT_A::MSB)
    }
    #[doc = "LSB is transferred first"]
    #[inline(always)]
    pub fn lsb(self) -> &'a mut crate::W<REG> {
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
    pub fn swrst(&mut self) -> SWRST_W<CTRLA_SPEC, 0> {
        SWRST_W::new(self)
    }
    #[doc = "Bit 1 - Enable"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> ENABLE_W<CTRLA_SPEC, 1> {
        ENABLE_W::new(self)
    }
    #[doc = "Bits 2:4 - Operating Mode"]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> MODE_W<CTRLA_SPEC, 2> {
        MODE_W::new(self)
    }
    #[doc = "Bit 7 - Run during Standby"]
    #[inline(always)]
    #[must_use]
    pub fn runstdby(&mut self) -> RUNSTDBY_W<CTRLA_SPEC, 7> {
        RUNSTDBY_W::new(self)
    }
    #[doc = "Bit 8 - Immediate Buffer Overflow Notification"]
    #[inline(always)]
    #[must_use]
    pub fn ibon(&mut self) -> IBON_W<CTRLA_SPEC, 8> {
        IBON_W::new(self)
    }
    #[doc = "Bits 16:17 - Data Out Pinout"]
    #[inline(always)]
    #[must_use]
    pub fn dopo(&mut self) -> DOPO_W<CTRLA_SPEC, 16> {
        DOPO_W::new(self)
    }
    #[doc = "Bits 20:21 - Data In Pinout"]
    #[inline(always)]
    #[must_use]
    pub fn dipo(&mut self) -> DIPO_W<CTRLA_SPEC, 20> {
        DIPO_W::new(self)
    }
    #[doc = "Bits 24:27 - Frame Format"]
    #[inline(always)]
    #[must_use]
    pub fn form(&mut self) -> FORM_W<CTRLA_SPEC, 24> {
        FORM_W::new(self)
    }
    #[doc = "Bit 28 - Clock Phase"]
    #[inline(always)]
    #[must_use]
    pub fn cpha(&mut self) -> CPHA_W<CTRLA_SPEC, 28> {
        CPHA_W::new(self)
    }
    #[doc = "Bit 29 - Clock Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn cpol(&mut self) -> CPOL_W<CTRLA_SPEC, 29> {
        CPOL_W::new(self)
    }
    #[doc = "Bit 30 - Data Order"]
    #[inline(always)]
    #[must_use]
    pub fn dord(&mut self) -> DORD_W<CTRLA_SPEC, 30> {
        DORD_W::new(self)
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
#[doc = "SPIS Control A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrla::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrla::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRLA_SPEC;
impl crate::RegisterSpec for CTRLA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrla::R`](R) reader structure"]
impl crate::Readable for CTRLA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctrla::W`](W) writer structure"]
impl crate::Writable for CTRLA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRLA to value 0"]
impl crate::Resettable for CTRLA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
