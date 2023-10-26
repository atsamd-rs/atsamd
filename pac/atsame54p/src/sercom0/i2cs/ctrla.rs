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
#[doc = "Field `PINOUT` reader - Pin Usage"]
pub type PINOUT_R = crate::BitReader;
#[doc = "Field `PINOUT` writer - Pin Usage"]
pub type PINOUT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SDAHOLD` reader - SDA Hold Time"]
pub type SDAHOLD_R = crate::FieldReader<SDAHOLDSELECT_A>;
#[doc = "SDA Hold Time\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SDAHOLDSELECT_A {
    #[doc = "0: Disabled"]
    DISABLE = 0,
    #[doc = "1: 50-100ns hold time"]
    _75NS = 1,
    #[doc = "2: 300-600ns hold time"]
    _450NS = 2,
    #[doc = "3: 400-800ns hold time"]
    _600NS = 3,
}
impl From<SDAHOLDSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: SDAHOLDSELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SDAHOLDSELECT_A {
    type Ux = u8;
}
impl SDAHOLD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SDAHOLDSELECT_A {
        match self.bits {
            0 => SDAHOLDSELECT_A::DISABLE,
            1 => SDAHOLDSELECT_A::_75NS,
            2 => SDAHOLDSELECT_A::_450NS,
            3 => SDAHOLDSELECT_A::_600NS,
            _ => unreachable!(),
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == SDAHOLDSELECT_A::DISABLE
    }
    #[doc = "50-100ns hold time"]
    #[inline(always)]
    pub fn is_75ns(&self) -> bool {
        *self == SDAHOLDSELECT_A::_75NS
    }
    #[doc = "300-600ns hold time"]
    #[inline(always)]
    pub fn is_450ns(&self) -> bool {
        *self == SDAHOLDSELECT_A::_450NS
    }
    #[doc = "400-800ns hold time"]
    #[inline(always)]
    pub fn is_600ns(&self) -> bool {
        *self == SDAHOLDSELECT_A::_600NS
    }
}
#[doc = "Field `SDAHOLD` writer - SDA Hold Time"]
pub type SDAHOLD_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, SDAHOLDSELECT_A>;
impl<'a, REG, const O: u8> SDAHOLD_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(SDAHOLDSELECT_A::DISABLE)
    }
    #[doc = "50-100ns hold time"]
    #[inline(always)]
    pub fn _75ns(self) -> &'a mut crate::W<REG> {
        self.variant(SDAHOLDSELECT_A::_75NS)
    }
    #[doc = "300-600ns hold time"]
    #[inline(always)]
    pub fn _450ns(self) -> &'a mut crate::W<REG> {
        self.variant(SDAHOLDSELECT_A::_450NS)
    }
    #[doc = "400-800ns hold time"]
    #[inline(always)]
    pub fn _600ns(self) -> &'a mut crate::W<REG> {
        self.variant(SDAHOLDSELECT_A::_600NS)
    }
}
#[doc = "Field `SEXTTOEN` reader - Slave SCL Low Extend Timeout"]
pub type SEXTTOEN_R = crate::BitReader;
#[doc = "Field `SEXTTOEN` writer - Slave SCL Low Extend Timeout"]
pub type SEXTTOEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SPEED` reader - Transfer Speed"]
pub type SPEED_R = crate::FieldReader<SPEEDSELECT_A>;
#[doc = "Transfer Speed\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SPEEDSELECT_A {
    #[doc = "0: Standard Mode(Sm) Upto 100kHz and Fast Mode(Fm) Upto 400kHz"]
    STANDARD_AND_FAST_MODE = 0,
    #[doc = "1: Fast-mode Plus Upto 1MHz"]
    FASTPLUS_MODE = 1,
    #[doc = "2: High-speed mode Upto 3.4MHz"]
    HIGH_SPEED_MODE = 2,
}
impl From<SPEEDSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: SPEEDSELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SPEEDSELECT_A {
    type Ux = u8;
}
impl SPEED_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SPEEDSELECT_A> {
        match self.bits {
            0 => Some(SPEEDSELECT_A::STANDARD_AND_FAST_MODE),
            1 => Some(SPEEDSELECT_A::FASTPLUS_MODE),
            2 => Some(SPEEDSELECT_A::HIGH_SPEED_MODE),
            _ => None,
        }
    }
    #[doc = "Standard Mode(Sm) Upto 100kHz and Fast Mode(Fm) Upto 400kHz"]
    #[inline(always)]
    pub fn is_standard_and_fast_mode(&self) -> bool {
        *self == SPEEDSELECT_A::STANDARD_AND_FAST_MODE
    }
    #[doc = "Fast-mode Plus Upto 1MHz"]
    #[inline(always)]
    pub fn is_fastplus_mode(&self) -> bool {
        *self == SPEEDSELECT_A::FASTPLUS_MODE
    }
    #[doc = "High-speed mode Upto 3.4MHz"]
    #[inline(always)]
    pub fn is_high_speed_mode(&self) -> bool {
        *self == SPEEDSELECT_A::HIGH_SPEED_MODE
    }
}
#[doc = "Field `SPEED` writer - Transfer Speed"]
pub type SPEED_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O, SPEEDSELECT_A>;
impl<'a, REG, const O: u8> SPEED_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Standard Mode(Sm) Upto 100kHz and Fast Mode(Fm) Upto 400kHz"]
    #[inline(always)]
    pub fn standard_and_fast_mode(self) -> &'a mut crate::W<REG> {
        self.variant(SPEEDSELECT_A::STANDARD_AND_FAST_MODE)
    }
    #[doc = "Fast-mode Plus Upto 1MHz"]
    #[inline(always)]
    pub fn fastplus_mode(self) -> &'a mut crate::W<REG> {
        self.variant(SPEEDSELECT_A::FASTPLUS_MODE)
    }
    #[doc = "High-speed mode Upto 3.4MHz"]
    #[inline(always)]
    pub fn high_speed_mode(self) -> &'a mut crate::W<REG> {
        self.variant(SPEEDSELECT_A::HIGH_SPEED_MODE)
    }
}
#[doc = "Field `SCLSM` reader - SCL Clock Stretch Mode"]
pub type SCLSM_R = crate::BitReader;
#[doc = "Field `SCLSM` writer - SCL Clock Stretch Mode"]
pub type SCLSM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LOWTOUTEN` reader - SCL Low Timeout Enable"]
pub type LOWTOUTEN_R = crate::BitReader;
#[doc = "Field `LOWTOUTEN` writer - SCL Low Timeout Enable"]
pub type LOWTOUTEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
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
    #[doc = "Bit 16 - Pin Usage"]
    #[inline(always)]
    pub fn pinout(&self) -> PINOUT_R {
        PINOUT_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 20:21 - SDA Hold Time"]
    #[inline(always)]
    pub fn sdahold(&self) -> SDAHOLD_R {
        SDAHOLD_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bit 23 - Slave SCL Low Extend Timeout"]
    #[inline(always)]
    pub fn sexttoen(&self) -> SEXTTOEN_R {
        SEXTTOEN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:25 - Transfer Speed"]
    #[inline(always)]
    pub fn speed(&self) -> SPEED_R {
        SPEED_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 27 - SCL Clock Stretch Mode"]
    #[inline(always)]
    pub fn sclsm(&self) -> SCLSM_R {
        SCLSM_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 30 - SCL Low Timeout Enable"]
    #[inline(always)]
    pub fn lowtouten(&self) -> LOWTOUTEN_R {
        LOWTOUTEN_R::new(((self.bits >> 30) & 1) != 0)
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
    #[doc = "Bit 16 - Pin Usage"]
    #[inline(always)]
    #[must_use]
    pub fn pinout(&mut self) -> PINOUT_W<CTRLA_SPEC, 16> {
        PINOUT_W::new(self)
    }
    #[doc = "Bits 20:21 - SDA Hold Time"]
    #[inline(always)]
    #[must_use]
    pub fn sdahold(&mut self) -> SDAHOLD_W<CTRLA_SPEC, 20> {
        SDAHOLD_W::new(self)
    }
    #[doc = "Bit 23 - Slave SCL Low Extend Timeout"]
    #[inline(always)]
    #[must_use]
    pub fn sexttoen(&mut self) -> SEXTTOEN_W<CTRLA_SPEC, 23> {
        SEXTTOEN_W::new(self)
    }
    #[doc = "Bits 24:25 - Transfer Speed"]
    #[inline(always)]
    #[must_use]
    pub fn speed(&mut self) -> SPEED_W<CTRLA_SPEC, 24> {
        SPEED_W::new(self)
    }
    #[doc = "Bit 27 - SCL Clock Stretch Mode"]
    #[inline(always)]
    #[must_use]
    pub fn sclsm(&mut self) -> SCLSM_W<CTRLA_SPEC, 27> {
        SCLSM_W::new(self)
    }
    #[doc = "Bit 30 - SCL Low Timeout Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lowtouten(&mut self) -> LOWTOUTEN_W<CTRLA_SPEC, 30> {
        LOWTOUTEN_W::new(self)
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
#[doc = "I2CS Control A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrla::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrla::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
