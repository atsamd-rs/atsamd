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
    #[doc = "0: USART mode with external clock"]
    USART_EXT_CLK = 0,
    #[doc = "1: USART mode with internal clock"]
    USART_INT_CLK = 1,
    #[doc = "2: SPI mode with external clock"]
    SPI_SLAVE = 2,
    #[doc = "3: SPI mode with internal clock"]
    SPI_MASTER = 3,
    #[doc = "4: I2C mode with external clock"]
    I2C_SLAVE = 4,
    #[doc = "5: I2C mode with internal clock"]
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
    #[doc = "USART mode with external clock"]
    #[inline(always)]
    pub fn usart_ext_clk(self) -> &'a mut W {
        self.variant(MODESELECT_A::USART_EXT_CLK)
    }
    #[doc = "USART mode with internal clock"]
    #[inline(always)]
    pub fn usart_int_clk(self) -> &'a mut W {
        self.variant(MODESELECT_A::USART_INT_CLK)
    }
    #[doc = "SPI mode with external clock"]
    #[inline(always)]
    pub fn spi_slave(self) -> &'a mut W {
        self.variant(MODESELECT_A::SPI_SLAVE)
    }
    #[doc = "SPI mode with internal clock"]
    #[inline(always)]
    pub fn spi_master(self) -> &'a mut W {
        self.variant(MODESELECT_A::SPI_MASTER)
    }
    #[doc = "I2C mode with external clock"]
    #[inline(always)]
    pub fn i2c_slave(self) -> &'a mut W {
        self.variant(MODESELECT_A::I2C_SLAVE)
    }
    #[doc = "I2C mode with internal clock"]
    #[inline(always)]
    pub fn i2c_master(self) -> &'a mut W {
        self.variant(MODESELECT_A::I2C_MASTER)
    }
}
#[doc = "Field `RUNSTDBY` reader - Run during Standby"]
pub type RUNSTDBY_R = crate::BitReader<bool>;
#[doc = "Field `RUNSTDBY` writer - Run during Standby"]
pub type RUNSTDBY_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRLA_SPEC, bool, O>;
#[doc = "Field `PINOUT` reader - Pin Usage"]
pub type PINOUT_R = crate::BitReader<bool>;
#[doc = "Field `PINOUT` writer - Pin Usage"]
pub type PINOUT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRLA_SPEC, bool, O>;
#[doc = "Field `SDAHOLD` reader - SDA Hold Time"]
pub type SDAHOLD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SDAHOLD` writer - SDA Hold Time"]
pub type SDAHOLD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRLA_SPEC, u8, u8, 2, O>;
#[doc = "Field `SEXTTOEN` reader - Slave SCL Low Extend Timeout"]
pub type SEXTTOEN_R = crate::BitReader<bool>;
#[doc = "Field `SEXTTOEN` writer - Slave SCL Low Extend Timeout"]
pub type SEXTTOEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRLA_SPEC, bool, O>;
#[doc = "Field `SPEED` reader - Transfer Speed"]
pub type SPEED_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SPEED` writer - Transfer Speed"]
pub type SPEED_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRLA_SPEC, u8, u8, 2, O>;
#[doc = "Field `SCLSM` reader - SCL Clock Stretch Mode"]
pub type SCLSM_R = crate::BitReader<bool>;
#[doc = "Field `SCLSM` writer - SCL Clock Stretch Mode"]
pub type SCLSM_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRLA_SPEC, bool, O>;
#[doc = "Field `LOWTOUTEN` reader - SCL Low Timeout Enable"]
pub type LOWTOUTEN_R = crate::BitReader<bool>;
#[doc = "Field `LOWTOUTEN` writer - SCL Low Timeout Enable"]
pub type LOWTOUTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRLA_SPEC, bool, O>;
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
    #[doc = "Bit 16 - Pin Usage"]
    #[inline(always)]
    #[must_use]
    pub fn pinout(&mut self) -> PINOUT_W<16> {
        PINOUT_W::new(self)
    }
    #[doc = "Bits 20:21 - SDA Hold Time"]
    #[inline(always)]
    #[must_use]
    pub fn sdahold(&mut self) -> SDAHOLD_W<20> {
        SDAHOLD_W::new(self)
    }
    #[doc = "Bit 23 - Slave SCL Low Extend Timeout"]
    #[inline(always)]
    #[must_use]
    pub fn sexttoen(&mut self) -> SEXTTOEN_W<23> {
        SEXTTOEN_W::new(self)
    }
    #[doc = "Bits 24:25 - Transfer Speed"]
    #[inline(always)]
    #[must_use]
    pub fn speed(&mut self) -> SPEED_W<24> {
        SPEED_W::new(self)
    }
    #[doc = "Bit 27 - SCL Clock Stretch Mode"]
    #[inline(always)]
    #[must_use]
    pub fn sclsm(&mut self) -> SCLSM_W<27> {
        SCLSM_W::new(self)
    }
    #[doc = "Bit 30 - SCL Low Timeout Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lowtouten(&mut self) -> LOWTOUTEN_W<30> {
        LOWTOUTEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2CS Control A\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrla](index.html) module"]
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
