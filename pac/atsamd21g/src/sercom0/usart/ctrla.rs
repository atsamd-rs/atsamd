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
#[doc = "Field `IBON` reader - Immediate Buffer Overflow Notification"]
pub type IbonR = crate::BitReader;
#[doc = "Field `IBON` writer - Immediate Buffer Overflow Notification"]
pub type IbonW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SAMPR` reader - Sample"]
pub type SamprR = crate::FieldReader;
#[doc = "Field `SAMPR` writer - Sample"]
pub type SamprW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `TXPO` reader - Transmit Data Pinout"]
pub type TxpoR = crate::FieldReader;
#[doc = "Field `TXPO` writer - Transmit Data Pinout"]
pub type TxpoW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RXPO` reader - Receive Data Pinout"]
pub type RxpoR = crate::FieldReader;
#[doc = "Field `RXPO` writer - Receive Data Pinout"]
pub type RxpoW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SAMPA` reader - Sample Adjustment"]
pub type SampaR = crate::FieldReader;
#[doc = "Field `SAMPA` writer - Sample Adjustment"]
pub type SampaW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `FORM` reader - Frame Format"]
pub type FormR = crate::FieldReader;
#[doc = "Field `FORM` writer - Frame Format"]
pub type FormW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CMODE` reader - Communication Mode"]
pub type CmodeR = crate::BitReader;
#[doc = "Field `CMODE` writer - Communication Mode"]
pub type CmodeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPOL` reader - Clock Polarity"]
pub type CpolR = crate::BitReader;
#[doc = "Field `CPOL` writer - Clock Polarity"]
pub type CpolW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DORD` reader - Data Order"]
pub type DordR = crate::BitReader;
#[doc = "Field `DORD` writer - Data Order"]
pub type DordW<'a, REG> = crate::BitWriter<'a, REG>;
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
    #[doc = "Bit 8 - Immediate Buffer Overflow Notification"]
    #[inline(always)]
    pub fn ibon(&self) -> IbonR {
        IbonR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 13:15 - Sample"]
    #[inline(always)]
    pub fn sampr(&self) -> SamprR {
        SamprR::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bits 16:17 - Transmit Data Pinout"]
    #[inline(always)]
    pub fn txpo(&self) -> TxpoR {
        TxpoR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 20:21 - Receive Data Pinout"]
    #[inline(always)]
    pub fn rxpo(&self) -> RxpoR {
        RxpoR::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - Sample Adjustment"]
    #[inline(always)]
    pub fn sampa(&self) -> SampaR {
        SampaR::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:27 - Frame Format"]
    #[inline(always)]
    pub fn form(&self) -> FormR {
        FormR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 28 - Communication Mode"]
    #[inline(always)]
    pub fn cmode(&self) -> CmodeR {
        CmodeR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Clock Polarity"]
    #[inline(always)]
    pub fn cpol(&self) -> CpolR {
        CpolR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Data Order"]
    #[inline(always)]
    pub fn dord(&self) -> DordR {
        DordR::new(((self.bits >> 30) & 1) != 0)
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
    #[doc = "Bit 8 - Immediate Buffer Overflow Notification"]
    #[inline(always)]
    #[must_use]
    pub fn ibon(&mut self) -> IbonW<CtrlaSpec> {
        IbonW::new(self, 8)
    }
    #[doc = "Bits 13:15 - Sample"]
    #[inline(always)]
    #[must_use]
    pub fn sampr(&mut self) -> SamprW<CtrlaSpec> {
        SamprW::new(self, 13)
    }
    #[doc = "Bits 16:17 - Transmit Data Pinout"]
    #[inline(always)]
    #[must_use]
    pub fn txpo(&mut self) -> TxpoW<CtrlaSpec> {
        TxpoW::new(self, 16)
    }
    #[doc = "Bits 20:21 - Receive Data Pinout"]
    #[inline(always)]
    #[must_use]
    pub fn rxpo(&mut self) -> RxpoW<CtrlaSpec> {
        RxpoW::new(self, 20)
    }
    #[doc = "Bits 22:23 - Sample Adjustment"]
    #[inline(always)]
    #[must_use]
    pub fn sampa(&mut self) -> SampaW<CtrlaSpec> {
        SampaW::new(self, 22)
    }
    #[doc = "Bits 24:27 - Frame Format"]
    #[inline(always)]
    #[must_use]
    pub fn form(&mut self) -> FormW<CtrlaSpec> {
        FormW::new(self, 24)
    }
    #[doc = "Bit 28 - Communication Mode"]
    #[inline(always)]
    #[must_use]
    pub fn cmode(&mut self) -> CmodeW<CtrlaSpec> {
        CmodeW::new(self, 28)
    }
    #[doc = "Bit 29 - Clock Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn cpol(&mut self) -> CpolW<CtrlaSpec> {
        CpolW::new(self, 29)
    }
    #[doc = "Bit 30 - Data Order"]
    #[inline(always)]
    #[must_use]
    pub fn dord(&mut self) -> DordW<CtrlaSpec> {
        DordW::new(self, 30)
    }
}
#[doc = "USART Control A\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrla::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrla::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
