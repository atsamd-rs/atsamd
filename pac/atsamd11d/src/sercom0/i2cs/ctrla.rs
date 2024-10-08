#[doc = "Register `CTRLA` reader"]
pub type R = crate::R<CtrlaSpec>;
#[doc = "Register `CTRLA` writer"]
pub type W = crate::W<CtrlaSpec>;
#[doc = "Field `SWRST` reader - Software Reset"]
pub type SwrstR = crate::BitReader;
#[doc = "Field `SWRST` writer - Software Reset"]
pub type SwrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENABLE` reader - Enable"]
pub type EnableR = crate::BitReader;
#[doc = "Field `ENABLE` writer - Enable"]
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Operating Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Modeselect {
    #[doc = "0: USART mode with external clock"]
    UsartExtClk = 0,
    #[doc = "1: USART mode with internal clock"]
    UsartIntClk = 1,
    #[doc = "2: SPI mode with external clock"]
    SpiSlave = 2,
    #[doc = "3: SPI mode with internal clock"]
    SpiMaster = 3,
    #[doc = "4: I2C mode with external clock"]
    I2cSlave = 4,
    #[doc = "5: I2C mode with internal clock"]
    I2cMaster = 5,
}
impl From<Modeselect> for u8 {
    #[inline(always)]
    fn from(variant: Modeselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Modeselect {
    type Ux = u8;
}
impl crate::IsEnum for Modeselect {}
#[doc = "Field `MODE` reader - Operating Mode"]
pub type ModeR = crate::FieldReader<Modeselect>;
impl ModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Modeselect> {
        match self.bits {
            0 => Some(Modeselect::UsartExtClk),
            1 => Some(Modeselect::UsartIntClk),
            2 => Some(Modeselect::SpiSlave),
            3 => Some(Modeselect::SpiMaster),
            4 => Some(Modeselect::I2cSlave),
            5 => Some(Modeselect::I2cMaster),
            _ => None,
        }
    }
    #[doc = "USART mode with external clock"]
    #[inline(always)]
    pub fn is_usart_ext_clk(&self) -> bool {
        *self == Modeselect::UsartExtClk
    }
    #[doc = "USART mode with internal clock"]
    #[inline(always)]
    pub fn is_usart_int_clk(&self) -> bool {
        *self == Modeselect::UsartIntClk
    }
    #[doc = "SPI mode with external clock"]
    #[inline(always)]
    pub fn is_spi_slave(&self) -> bool {
        *self == Modeselect::SpiSlave
    }
    #[doc = "SPI mode with internal clock"]
    #[inline(always)]
    pub fn is_spi_master(&self) -> bool {
        *self == Modeselect::SpiMaster
    }
    #[doc = "I2C mode with external clock"]
    #[inline(always)]
    pub fn is_i2c_slave(&self) -> bool {
        *self == Modeselect::I2cSlave
    }
    #[doc = "I2C mode with internal clock"]
    #[inline(always)]
    pub fn is_i2c_master(&self) -> bool {
        *self == Modeselect::I2cMaster
    }
}
#[doc = "Field `MODE` writer - Operating Mode"]
pub type ModeW<'a, REG> = crate::FieldWriter<'a, REG, 3, Modeselect>;
impl<'a, REG> ModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "USART mode with external clock"]
    #[inline(always)]
    pub fn usart_ext_clk(self) -> &'a mut crate::W<REG> {
        self.variant(Modeselect::UsartExtClk)
    }
    #[doc = "USART mode with internal clock"]
    #[inline(always)]
    pub fn usart_int_clk(self) -> &'a mut crate::W<REG> {
        self.variant(Modeselect::UsartIntClk)
    }
    #[doc = "SPI mode with external clock"]
    #[inline(always)]
    pub fn spi_slave(self) -> &'a mut crate::W<REG> {
        self.variant(Modeselect::SpiSlave)
    }
    #[doc = "SPI mode with internal clock"]
    #[inline(always)]
    pub fn spi_master(self) -> &'a mut crate::W<REG> {
        self.variant(Modeselect::SpiMaster)
    }
    #[doc = "I2C mode with external clock"]
    #[inline(always)]
    pub fn i2c_slave(self) -> &'a mut crate::W<REG> {
        self.variant(Modeselect::I2cSlave)
    }
    #[doc = "I2C mode with internal clock"]
    #[inline(always)]
    pub fn i2c_master(self) -> &'a mut crate::W<REG> {
        self.variant(Modeselect::I2cMaster)
    }
}
#[doc = "Field `RUNSTDBY` reader - Run during Standby"]
pub type RunstdbyR = crate::BitReader;
#[doc = "Field `RUNSTDBY` writer - Run during Standby"]
pub type RunstdbyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PINOUT` reader - Pin Usage"]
pub type PinoutR = crate::BitReader;
#[doc = "Field `PINOUT` writer - Pin Usage"]
pub type PinoutW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDAHOLD` reader - SDA Hold Time"]
pub type SdaholdR = crate::FieldReader;
#[doc = "Field `SDAHOLD` writer - SDA Hold Time"]
pub type SdaholdW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SEXTTOEN` reader - Slave SCL Low Extend Timeout"]
pub type SexttoenR = crate::BitReader;
#[doc = "Field `SEXTTOEN` writer - Slave SCL Low Extend Timeout"]
pub type SexttoenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPEED` reader - Transfer Speed"]
pub type SpeedR = crate::FieldReader;
#[doc = "Field `SPEED` writer - Transfer Speed"]
pub type SpeedW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SCLSM` reader - SCL Clock Stretch Mode"]
pub type SclsmR = crate::BitReader;
#[doc = "Field `SCLSM` writer - SCL Clock Stretch Mode"]
pub type SclsmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOWTOUTEN` reader - SCL Low Timeout Enable"]
pub type LowtoutenR = crate::BitReader;
#[doc = "Field `LOWTOUTEN` writer - SCL Low Timeout Enable"]
pub type LowtoutenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Software Reset"]
    #[inline(always)]
    pub fn swrst(&self) -> SwrstR {
        SwrstR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable"]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:4 - Operating Mode"]
    #[inline(always)]
    pub fn mode(&self) -> ModeR {
        ModeR::new(((self.bits >> 2) & 7) as u8)
    }
    #[doc = "Bit 7 - Run during Standby"]
    #[inline(always)]
    pub fn runstdby(&self) -> RunstdbyR {
        RunstdbyR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 16 - Pin Usage"]
    #[inline(always)]
    pub fn pinout(&self) -> PinoutR {
        PinoutR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 20:21 - SDA Hold Time"]
    #[inline(always)]
    pub fn sdahold(&self) -> SdaholdR {
        SdaholdR::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bit 23 - Slave SCL Low Extend Timeout"]
    #[inline(always)]
    pub fn sexttoen(&self) -> SexttoenR {
        SexttoenR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:25 - Transfer Speed"]
    #[inline(always)]
    pub fn speed(&self) -> SpeedR {
        SpeedR::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 27 - SCL Clock Stretch Mode"]
    #[inline(always)]
    pub fn sclsm(&self) -> SclsmR {
        SclsmR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 30 - SCL Low Timeout Enable"]
    #[inline(always)]
    pub fn lowtouten(&self) -> LowtoutenR {
        LowtoutenR::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Software Reset"]
    #[inline(always)]
    #[must_use]
    pub fn swrst(&mut self) -> SwrstW<CtrlaSpec> {
        SwrstW::new(self, 0)
    }
    #[doc = "Bit 1 - Enable"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> EnableW<CtrlaSpec> {
        EnableW::new(self, 1)
    }
    #[doc = "Bits 2:4 - Operating Mode"]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> ModeW<CtrlaSpec> {
        ModeW::new(self, 2)
    }
    #[doc = "Bit 7 - Run during Standby"]
    #[inline(always)]
    #[must_use]
    pub fn runstdby(&mut self) -> RunstdbyW<CtrlaSpec> {
        RunstdbyW::new(self, 7)
    }
    #[doc = "Bit 16 - Pin Usage"]
    #[inline(always)]
    #[must_use]
    pub fn pinout(&mut self) -> PinoutW<CtrlaSpec> {
        PinoutW::new(self, 16)
    }
    #[doc = "Bits 20:21 - SDA Hold Time"]
    #[inline(always)]
    #[must_use]
    pub fn sdahold(&mut self) -> SdaholdW<CtrlaSpec> {
        SdaholdW::new(self, 20)
    }
    #[doc = "Bit 23 - Slave SCL Low Extend Timeout"]
    #[inline(always)]
    #[must_use]
    pub fn sexttoen(&mut self) -> SexttoenW<CtrlaSpec> {
        SexttoenW::new(self, 23)
    }
    #[doc = "Bits 24:25 - Transfer Speed"]
    #[inline(always)]
    #[must_use]
    pub fn speed(&mut self) -> SpeedW<CtrlaSpec> {
        SpeedW::new(self, 24)
    }
    #[doc = "Bit 27 - SCL Clock Stretch Mode"]
    #[inline(always)]
    #[must_use]
    pub fn sclsm(&mut self) -> SclsmW<CtrlaSpec> {
        SclsmW::new(self, 27)
    }
    #[doc = "Bit 30 - SCL Low Timeout Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lowtouten(&mut self) -> LowtoutenW<CtrlaSpec> {
        LowtoutenW::new(self, 30)
    }
}
#[doc = "I2CS Control A\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrla::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrla::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlaSpec;
impl crate::RegisterSpec for CtrlaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrla::R`](R) reader structure"]
impl crate::Readable for CtrlaSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrla::W`](W) writer structure"]
impl crate::Writable for CtrlaSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTRLA to value 0"]
impl crate::Resettable for CtrlaSpec {
    const RESET_VALUE: u32 = 0;
}
